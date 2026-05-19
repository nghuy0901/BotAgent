use crate::tools::{
    container::ToolContainer,
    domain::ToolDomain,
    impls::{
        category::CategoryTools, channel::ChannelTools, guild::GuildTools, message::MessageTools,
        time::TimeTools,
    },
};

pub mod category;
pub mod channel;
pub mod guild;
pub mod message;
pub mod time;

pub struct AllTools;

impl ToolDomain for AllTools {
    fn register_in(container: ToolContainer) -> ToolContainer {
        container
            .with_domain::<CategoryTools>()
            .with_domain::<ChannelTools>()
            .with_domain::<GuildTools>()
            .with_domain::<MessageTools>()
            .with_domain::<TimeTools>()
    }
}
