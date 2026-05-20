use crate::tools::{
    container::ToolContainer,
    domain::ToolDomain,
    impls::guild::{
        get_information::GetGuildInformationTool, list_channels::ListGuildChannelsTool,
    },
};

pub mod get_information;
pub mod list_channels;

pub struct GuildTools;

impl ToolDomain for GuildTools {
    fn register_in(container: ToolContainer) -> ToolContainer {
        container
            .with_tool(GetGuildInformationTool)
            .with_tool(ListGuildChannelsTool)
    }
}
