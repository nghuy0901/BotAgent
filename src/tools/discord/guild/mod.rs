use crate::tools::{
    container::ToolContainer, discord::guild::get_information::GetGuildInformationTool,
    domain::ToolDomain,
};

pub mod get_information;

pub struct GuildTools;

impl ToolDomain for GuildTools {
    fn register_in(container: ToolContainer) -> ToolContainer {
        container.with_tool(GetGuildInformationTool)
    }
}
