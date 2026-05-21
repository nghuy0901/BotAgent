pub mod channel;
pub mod context;
pub mod event;
pub mod prompt_composer;

use std::sync::Arc;

use openai_dive::v1::resources::chat::{
    ChatCompletionParametersBuilder, ChatCompletionToolChoice, ChatMessage, ChatMessageContent,
    Function, ToolCall,
};
use serde_json::{Value, json};
use serenity::all::CreateMessage;

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
    pub history: Vec<ChatMessage>,
}

impl Agent {
    pub fn new(dedicated_context: Arc<DedicatedContext>) -> Self {
        let system_prompt = PromptComposer::new()
            .inject("CAPABILITIES", build_capabilities(&dedicated_context))
            .build();

        Self {
            dedicated_context,
            history: vec![ChatMessage::System {
                content: ChatMessageContent::Text(system_prompt),
                name: None,
            }],
        }
    }

    pub fn add_user_message(&mut self, message: String) {
        self.history.push(ChatMessage::User {
            content: ChatMessageContent::Text(message),
            name: None,
        });
    }

    pub async fn chat(&mut self, message: String) -> Result<()> {
        let max_turns = self.dedicated_context.configuration.bot.max_turns;

        self.add_user_message(message);

        if max_turns == 0 {
            self.run_completion(false).await?;

            return Ok(());
        }

        for _ in 0..max_turns {
            match self.run_completion(true).await? {
                true => continue,
                false => return Ok(()),
            }
        }

        tracing::warn!(
            max_turns,
            "agent exceeded maximum turns, running one last completion with tools disabled"
        );

        self.history.push(ChatMessage::System { content: ChatMessageContent::Text("The tool calling loop was stopped because you reached the maximum amount of turns. If you're done, complete your response. If not, communicate the problem to the user.".to_string()), name: None });
        self.run_completion(false).await?;

        Ok(())
    }

    async fn run_completion(&mut self, tools_enabled: bool) -> Result<bool> {
        let ctx = &self.dedicated_context;

        let mut parameters = ChatCompletionParametersBuilder::default();

        parameters.model(&self.dedicated_context.configuration.llm.model);
        parameters.parallel_tool_calls(true);
        parameters.messages(self.history.clone());

        if tools_enabled {
            parameters.tools(self.dedicated_context.tools.clone());
        } else {
            parameters.tool_choice(ChatCompletionToolChoice::None);
        }

        let parameters = parameters.build()?;

        let request = self
            .dedicated_context
            .endpoint
            .chat()
            .create(parameters)
            .await?;

        let message = request.choices[0].message.clone();

        self.history.push(message.clone());

        if let ChatMessage::Assistant {
            content: Some(ChatMessageContent::Text(content)),
            ..
        } = &message
            && !content.is_empty()
        // This will be deprecated at some point in favor of WakeOnMention.
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

        if let ChatMessage::Assistant {
            tool_calls: Some(tool_calls),
            ..
        } = message
        {
            let tools_called = !tool_calls.is_empty();

            for ToolCall {
                id: tool_call_id,
                function: Function { name, arguments },
                ..
            } in tool_calls
            {
                let Some(tool) = ctx.tool_container.tools.get(&name) else {
                    self.history.push(ChatMessage::Tool {
                        content: ChatMessageContent::Text(
                            json!({
                                "status": "error",
                                "reason": format!("a tool named {name} does not exist")
                            })
                            .to_string(),
                        ),
                        tool_call_id,
                    });

                    continue;
                };

                let args = match serde_json::from_str::<Value>(&arguments) {
                    Ok(v) => v,
                    Err(err) => {
                        self.history.push(ChatMessage::Tool {
                            content: ChatMessageContent::Text(
                                json!({
                                    "status": "error",
                                    "reason": format!("error while parsing arguments: {err}")
                                })
                                .to_string(),
                            ),
                            tool_call_id,
                        });

                        continue;
                    }
                };

                self.history.push(ChatMessage::Tool {
                    content: ChatMessageContent::Text(tool.execute(args, ctx.clone()).await?),
                    tool_call_id,
                });
            }

            Ok(tools_called)
        } else {
            Ok(false)
        }
    }
}
