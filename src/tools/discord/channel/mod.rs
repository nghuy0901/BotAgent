pub mod create_text;
pub mod get_information;
pub mod send_message;

use crate::tools::{
    container::ToolContainer,
    discord::channel::{
        create_text::CreateTextChannelTool, get_information::GetChannelInformationTool,
        send_message::SendMessageTool,
    },
    domain::ToolDomain,
};

pub struct ChannelTools;

impl ToolDomain for ChannelTools {
    fn register_in(container: ToolContainer) -> ToolContainer {
        container
            .with_tool(SendMessageTool)
            .with_tool(GetChannelInformationTool)
            .with_tool(CreateTextChannelTool)
    }
}
