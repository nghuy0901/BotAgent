use crate::tools::container::ToolContainer;

pub trait ToolDomain {
    fn register_in(container: ToolContainer) -> ToolContainer;
}
