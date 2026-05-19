use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct ApprovalMetadata {
    pub action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_data: Option<Value>,
}
