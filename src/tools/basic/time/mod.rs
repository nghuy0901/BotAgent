use crate::tools::{
    basic::time::{get_local_time::GetLocalTime, timestamp_to_local::TimestampToLocal},
    container::ToolContainer,
    domain::ToolDomain,
};

pub mod get_local_time;
pub mod timestamp_to_local;

pub struct TimeTools;

impl ToolDomain for TimeTools {
    fn register_in(container: ToolContainer) -> ToolContainer {
        container
            .with_tool(GetLocalTime)
            .with_tool(TimestampToLocal)
    }
}
