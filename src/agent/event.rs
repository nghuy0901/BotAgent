use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EventContent {
    Message {
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
    RequestTimedOut {
        approval_id: String,
    },
}

#[derive(Serialize)]
pub struct AgentEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<u64>,

    #[serde(flatten)]
    pub content: EventContent,
}

impl AgentEvent {
    pub fn new(content: EventContent) -> Self {
        Self {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .ok(),
            content,
        }
    }

    pub fn with_timestamp(mut self, timestamp: u64) -> Self {
        self.timestamp = Some(timestamp);

        self
    }
}
