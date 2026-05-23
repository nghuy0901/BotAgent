use serde_json::{Value, json};
use serenity::all::ChannelType;

pub fn channel_kind_to_value(kind: ChannelType) -> Value {
    json!(match kind {
        ChannelType::Category => "category",
        ChannelType::Directory => "directory",
        ChannelType::Forum => "forum",
        ChannelType::GroupDm => "group_dm",
        ChannelType::News => "news",
        ChannelType::NewsThread => "news_thread",
        ChannelType::Private => "direct_message",
        ChannelType::PrivateThread => "private_thread",
        ChannelType::PublicThread => "public_thread",
        ChannelType::Stage => "stage",
        ChannelType::Text => "text",
        ChannelType::Voice => "voice_chat",
        _ => "unknown",
    })
}
