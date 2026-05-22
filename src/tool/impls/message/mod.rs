use crate::tool::{
    container::ToolContainer, domain::ToolDomain, impls::message::react::ReactMessageTool,
};

pub mod react;

pub struct MessageTools;

impl ToolDomain for MessageTools {
    fn register_in(container: ToolContainer) -> ToolContainer {
        container.with_tool(ReactMessageTool)
    }
}
