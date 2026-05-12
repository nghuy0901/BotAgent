use crate::tools::{
    basic::{end_turn::EndTurnTool, time::TimeTools},
    container::ToolContainer,
    domain::ToolDomain,
};

pub mod end_turn;
pub mod time;

pub struct BasicTools;

impl ToolDomain for BasicTools {
    fn register_in(container: ToolContainer) -> ToolContainer {
        container.with_domain::<TimeTools>().with_tool(EndTurnTool)
    }
}
