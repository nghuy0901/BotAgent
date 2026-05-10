use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EventContent {
    Message {
        #[serde(skip_serializing_if = "Option::is_none")]
        guild_id: Option<String>,
        channel_id: String,
        message_id: String,
        author: Value,
        content: String,
    },
    RequestApproved {
        approval_id: String,

        #[serde(skip_serializing_if = "Option::is_none", flatten)]
        data: Option<Value>,
    },
    RequestDenied {
        approval_id: String,
    },
}

#[derive(Serialize)]
pub struct AgentEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
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
