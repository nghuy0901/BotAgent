use crate::tools::{
    container::ToolContainer,
    discord::{
        channel::ChannelTools, guild::GuildTools, message::MessageTools,
        start_typing::StartTypingTool,
    },
    domain::ToolDomain,
};

pub mod channel;
pub mod guild;
pub mod message;
pub mod start_typing;

pub struct DiscordTools;

impl ToolDomain for DiscordTools {
    fn register_in(container: ToolContainer) -> ToolContainer {
        container
            .with_domain::<ChannelTools>()
            .with_domain::<GuildTools>()
            .with_domain::<MessageTools>()
            .with_tool(StartTypingTool)
    }
}
