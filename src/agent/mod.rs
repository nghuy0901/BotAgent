pub mod channel;
pub mod context;
pub mod event;
pub mod prompt_composer;

use std::sync::Arc;

use async_openai::types::chat::{
    ChatCompletionMessageToolCalls, ChatCompletionRequestAssistantMessageArgs,
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage,
    ChatCompletionRequestToolMessageArgs, ChatCompletionRequestUserMessage,
    ChatCompletionResponseMessage, CreateChatCompletionRequestArgs, CreateChatCompletionResponse,
};
use serde_json::{Value, json};
use serenity::all::CreateMessage;
use tracing::error;

use crate::{
    Result,
    agent::{
        context::DedicatedContext,
        prompt_composer::{PromptComposer, build_capabilities},
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
            .inject("CAPABILITIES", build_capabilities(&dedicated_context))
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
        }

        if let Some(content) = &message.content {
            assistant_message.content(content.clone());
        }

        self.history.push(assistant_message.build()?.into());

        if let Some(content) = message.content.as_ref().map(|v| v.trim())
            && !content.is_empty()
            && !content.contains("[IGNORE]")
        {
            let messages = content
                .split("[SPLIT]")
                .map(|s| s.trim())
                .collect::<Vec<_>>();

            for message in messages {
                ctx.channel_id
                    .send_message(ctx.http(), CreateMessage::new().content(message))
                    .await?;
            }
        }

        if let Some(tool_calls) = &message.tool_calls
            && !tool_calls.is_empty()
        {
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
                                .content(json!({ "status": "error", "reason": format!("a tool named \"{}\" does not exist", call.function.name) }).to_string())
                                .build()?
                                .into(),
                        );

                        continue;
                    };

                    match serde_json::from_str::<Value>(&call.function.arguments) {
                        Ok(args) => {
                            let response =
                                tool.execute(args, self.dedicated_context.clone()).await?;

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
                                "status": "error",
                                "reason": format!("parsing error: {e}"),
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

            if tool_calls.is_empty() {
                Ok(message.clone())
            } else {
                Box::pin(self.chat(None)).await
            }
        } else {
            Ok(message.clone())
        }
    }
}
