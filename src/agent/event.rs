use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EventContent {
    Message { author: String, content: String },
}

#[derive(Serialize)]
pub struct AgentEvent {
    pub timestamp: Option<i64>,

    #[serde(flatten)]
    pub content: EventContent,
}

impl AgentEvent {
    #[inline]
    pub fn new(content: EventContent) -> Self {
        Self {
            timestamp: None,
            content,
        }
    }

    #[inline]
    pub fn with_timestamp(mut self, timestamp: i64) -> Self {
        self.timestamp = Some(timestamp);

        self
    }
}
