use crate::tools::{
    container::ToolContainer, discord::message::react::ReactMessageTool, domain::ToolDomain,
};

pub mod react;

pub struct MessageTools;

impl ToolDomain for MessageTools {
    fn register_in(container: ToolContainer) -> ToolContainer {
        container.with_tool(ReactMessageTool)
    }
}
