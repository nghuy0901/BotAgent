pub mod channel;
pub mod context;
pub mod event;
pub mod prompt_composer;

use std::sync::Arc;

use async_openai::types::chat::{
    ChatCompletionMessageToolCalls, ChatCompletionRequestAssistantMessageArgs,
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage,
    ChatCompletionRequestToolMessageArgs, ChatCompletionRequestUserMessage,
    ChatCompletionResponseMessage, ChatCompletionToolChoiceOption, CreateChatCompletionRequestArgs,
    CreateChatCompletionResponse, ToolChoiceOptions,
};
use serde_json::{Value, json};
use tracing::error;

use crate::{
    Result,
    agent::{
        context::DedicatedContext,
        prompt_composer::{PromptComposer, build_tool_rules},
    },
};

pub struct Agent {
    pub dedicated_context: Arc<DedicatedContext>,

    /// The chat history
    pub history: Vec<ChatCompletionRequestMessage>,
}

impl Agent {
    /// Creates a new [Agent] with the given OpenAI client, http struct and cache.
    pub fn new(dedicated_context: Arc<DedicatedContext>) -> Self {
        let system_prompt = PromptComposer::new()
            .inject("TOOL_RULES", build_tool_rules(&dedicated_context))
            .build();

        Self {
            dedicated_context,
            history: vec![ChatCompletionRequestSystemMessage::from(system_prompt).into()],
        }
    }

    pub async fn chat(&mut self, message: Option<String>) -> Result<ChatCompletionResponseMessage> {
        if let Some(message) = message {
            self.history
                .push(ChatCompletionRequestUserMessage::from(message).into());
        }

        let ctx = &self.dedicated_context;

        let request = CreateChatCompletionRequestArgs::default()
            .model(&ctx.configuration.llm.model)
            .parallel_tool_calls(true)
            .tool_choice(ChatCompletionToolChoiceOption::Mode(
                ToolChoiceOptions::Required,
            ))
            .messages(self.history.clone())
            .tools(ctx.tools.clone())
            .build()?;

        let CreateChatCompletionResponse { choices, .. } =
            ctx.agent_context.base_client.chat().create(request).await?;

        // we might have to add support for multiple choices at some point?
        let message = &choices
            .first()
            .ok_or("no choices returned".to_string())?
            .message;

        let mut assistant_message = ChatCompletionRequestAssistantMessageArgs::default();

        if let Some(tool_calls) = &message.tool_calls {
            assistant_message.tool_calls(tool_calls.clone());
        } else {
            // the agent should never return any content because it is generally not used. however, if it does, we just ignore it and replace it.
            // we dont replace it in the other if-block, because content can be None when tool_calls are specified. we do append an assistant message later

            assistant_message.content("[DONE]");
        }

        self.history.push(assistant_message.build()?.into());

        if let Some(tool_calls) = &message.tool_calls
            && !tool_calls.is_empty()
        {
            let mut finish_call_included = false;

            for call in tool_calls {
                if let ChatCompletionMessageToolCalls::Function(call) = call {
                    let Some(tool) = self
                        .dedicated_context
                        .tool_container
                        .tools
                        .get(call.function.name.as_str())
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
                            let response =
                                tool.execute(args, self.dedicated_context.clone()).await?;

                            if call.function.name == "end_turn" {
                                finish_call_included = true;
                            }

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
                // This is mandatory because some chat templates require alternating turns where the last assistant message is only a non-empty content string and is then followed by a user message.
                self.history.push(
                    ChatCompletionRequestAssistantMessageArgs::default()
                        .content("[DONE]")
                        .build()?
                        .into(),
                );

                Ok(message.clone())
            } else {
                Box::pin(self.chat(None)).await
            }
        } else {
            Ok(message.clone())
        }
    }
}
