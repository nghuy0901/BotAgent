use crate::tools::{
    container::ToolContainer,
    discord::message::{react::ReactMessageTool, reply::ReplyToMessageTool},
    domain::ToolDomain,
};

pub mod react;
pub mod reply;

pub struct MessageTools;

impl ToolDomain for MessageTools {
    fn register_in(container: ToolContainer) -> ToolContainer {
        container
            .with_tool(ReactMessageTool)
            .with_tool(ReplyToMessageTool)
    }
}
