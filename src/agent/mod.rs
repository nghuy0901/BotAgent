pub mod channel;
pub mod config;
pub mod event;
pub mod tools;

use std::sync::Arc;

use async_openai::{
    Client,
    config::Config,
    types::chat::{
        ChatCompletionMessageToolCalls, ChatCompletionRequestAssistantMessageArgs,
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage,
        ChatCompletionRequestToolMessageArgs, ChatCompletionRequestUserMessage,
        ChatCompletionResponseMessage, ChatCompletionToolChoiceOption,
        CreateChatCompletionRequestArgs, CreateChatCompletionResponse, ToolChoiceOptions,
    },
};
use serde_json::{Value, json};
use serenity::all::{Cache, Http};
use tracing::error;

use crate::agent::{
    config::AgentConfig,
    tools::{container::ToolEntry, discord::DiscordContext},
};

pub struct Agent<C: Config> {
    pub config: Arc<AgentConfig>,

    /// The OpenAI client
    pub client: Arc<Client<C>>,

    /// The Http struct used for interacting with the Discord API.
    pub http: Arc<Http>,

    /// A cache to minimize API requests where possible.
    pub cache: Arc<Cache>,

    /// The chat history
    pub history: Vec<ChatCompletionRequestMessage>,
}

impl<C: Config> Agent<C> {
    /// Creates a new [Agent] with the given OpenAI client, http struct and cache.
    pub fn new(
        config: Arc<AgentConfig>,
        client: Arc<Client<C>>,
        http: Arc<Http>,
        cache: Arc<Cache>,
    ) -> Self {
        Self {
            config,
            client,
            http,
            cache,
            history: Vec::new(),
        }
    }

    pub fn with_system_prompt<T: AsRef<str>>(mut self, prompt: T) -> Self {
        let prompt = prompt.as_ref().to_string();

        self.history
            .push(ChatCompletionRequestSystemMessage::from(prompt).into());

        self
    }

    pub async fn chat(
        &mut self,
        message: Option<String>,
    ) -> Result<ChatCompletionResponseMessage, Box<dyn std::error::Error + Send + Sync>> {
        if let Some(message) = message {
            self.history
                .push(ChatCompletionRequestUserMessage::from(message).into());
        }

        let request = CreateChatCompletionRequestArgs::default()
            .model(&self.config.model)
            .parallel_tool_calls(true)
            .tool_choice(ChatCompletionToolChoiceOption::Mode(
                ToolChoiceOptions::Required,
            ))
            .messages(self.history.clone())
            .tools(self.config.tools.tool_infos.clone())
            .build()?;

        let CreateChatCompletionResponse { choices, .. } =
            self.client.chat().create(request).await?;

        // we might have to add support for multiple choices at some point?
        let message = &choices
            .first()
            .ok_or("no choices returned".to_string())?
            .message;

        let mut assistant_message = ChatCompletionRequestAssistantMessageArgs::default();

        if let Some(tool_calls) = &message.tool_calls {
            assistant_message.tool_calls(tool_calls.clone());
        } else {
            // the agent should never return any content because it is generally not used. however, if it does, we just ignore it and replace it
            // we dont replace it in the other if-block, because content can be None when tool_calls are specified.

            assistant_message.content("");
        }

        self.history.push(assistant_message.build().unwrap().into());

        if let Some(tool_calls) = &message.tool_calls
            && !tool_calls.is_empty()
        {
            let mut finish_call_included = false;

            for call in tool_calls {
                if let ChatCompletionMessageToolCalls::Function(call) = call {
                    let Some(tool) = self.config.tools.tools.get(call.function.name.as_str())
                    else {
                        error!(
                            "unable to find tool named \"{}\", returning error to agent",
                            call.function.name
                        );

                        self.history.push(
                            ChatCompletionRequestToolMessageArgs::default()
                                .tool_call_id(call.id.clone())
                                .content(json!({ "error": format!("a tool named \"{}\" does not exist", call.function.name) }).to_string())
                                .build()?
                                .into(),
                        );

                        continue;
                    };

                    match serde_json::from_str::<Value>(&call.function.arguments) {
                        Ok(args) => {
                            let response = match tool {
                                ToolEntry::BasicTool(t) if call.function.name == "finish" => {
                                    finish_call_included = true;

                                    t.execute(args).await
                                }
                                ToolEntry::BasicTool(t) => t.execute(args).await,
                                ToolEntry::DiscordTool(t) => {
                                    t.execute(
                                        args,
                                        DiscordContext {
                                            cache: self.cache.clone(),
                                            http: self.http.clone(),
                                        },
                                    )
                                    .await
                                }
                            }?;

                            self.history.push(
                                ChatCompletionRequestToolMessageArgs::default()
                                    .tool_call_id(call.id.clone())
                                    .content(response)
                                    .build()?
                                    .into(),
                            );
                        }
                        Err(e) => {
                            let override_response = json!({
                                "success": false,
                                "reason": "error while parsing arguments",
                                "error": format!("{:?}", e)
                            });

                            self.history.push(
                                ChatCompletionRequestToolMessageArgs::default()
                                    .tool_call_id(call.id.clone())
                                    .content(override_response.to_string())
                                    .build()
                                    .unwrap()
                                    .into(),
                            );
                        }
                    }
                } else {
                    todo!()
                }
            }

            if finish_call_included {
                Ok(message.clone())
            } else {
                Box::pin(self.chat(None)).await
            }
        } else {
            Ok(message.clone())
        }
    }
}
