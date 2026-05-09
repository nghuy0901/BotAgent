pub mod channel;
pub mod event;

use std::sync::Arc;

use async_openai::{
    Client,
    config::Config,
    types::chat::{
        ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage,
        ChatCompletionRequestUserMessage, ChatCompletionResponseMessage,
        CreateChatCompletionRequestArgs, CreateChatCompletionResponse,
    },
};
use serenity::all::{Cache, Http};

pub struct Agent<C: Config> {
    pub model: String,

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
    pub fn new<T: AsRef<str>>(
        model: T,
        client: Arc<Client<C>>,
        http: Arc<Http>,
        cache: Arc<Cache>,
    ) -> Self {
        Self {
            model: model.as_ref().to_string(),
            client,
            http,
            cache,
            history: Vec::new(),
        }
    }

    pub async fn chat(
        &mut self,
        message: Option<String>,
    ) -> Result<ChatCompletionResponseMessage, Box<dyn std::error::Error>> {
        if let Some(message) = message {
            self.history
                .push(ChatCompletionRequestUserMessage::from(message).into());
        }

        let request = CreateChatCompletionRequestArgs::default()
            .model(&self.model)
            .parallel_tool_calls(true)
            /*.tool_choice(ChatCompletionToolChoiceOption::Mode(
                ToolChoiceOptions::Required,
            ))*/
            .messages(self.history.clone())
            .build()?;

        let CreateChatCompletionResponse { choices, .. } =
            self.client.chat().create(request).await?;

        // we might have to add support for multiple choices at some point?
        let message = &choices
            .first()
            .ok_or("no choices returned".to_string())?
            .message;

        let mut assistant_message = ChatCompletionRequestAssistantMessageArgs::default();

        if let Some(content) = &message.content {
            assistant_message.content(content.clone());
        }

        if let Some(tool_calls) = &message.tool_calls {
            assistant_message.tool_calls(tool_calls.clone());
        }

        self.history.push(assistant_message.build().unwrap().into());

        if let Some(tool_calls) = &message.tool_calls
            && tool_calls.len() > 0
        {
            todo!()
        } else {
            Ok(message.clone())
        }
    }
}
