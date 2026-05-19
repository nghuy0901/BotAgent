use crate::tools::{basic::time::TimeTools, container::ToolContainer, domain::ToolDomain};

pub mod time;

pub struct BasicTools;

impl ToolDomain for BasicTools {
    fn register_in(container: ToolContainer) -> ToolContainer {
        container.with_domain::<TimeTools>()
    }
}
