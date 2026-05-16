use serde_json::{Value, json};
use serenity::all::ChannelType;

pub fn channel_kind_to_value(kind: ChannelType) -> Value {
    json!(match kind {
        ChannelType::Text => "text",
        ChannelType::Category => "category",
        ChannelType::Voice => "voice_chat",
        ChannelType::Stage => "stage",
        _ => "unknown",
    })
}
