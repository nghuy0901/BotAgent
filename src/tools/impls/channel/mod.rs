pub mod create_text;
pub mod delete;
pub mod get_information;
pub mod send_message;

use crate::tools::{
    container::ToolContainer,
    domain::ToolDomain,
    impls::channel::{
        create_text::CreateTextChannelTool, delete::DeleteChannelTool,
        get_information::GetChannelInformationTool, send_message::SendMessageTool,
    },
};

pub struct ChannelTools;

impl ToolDomain for ChannelTools {
    fn register_in(container: ToolContainer) -> ToolContainer {
        container
            .with_tool(SendMessageTool)
            .with_tool(GetChannelInformationTool)
            .with_tool(CreateTextChannelTool)
            .with_tool(DeleteChannelTool)
    }
}
