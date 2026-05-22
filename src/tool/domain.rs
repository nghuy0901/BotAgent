use crate::tool::container::ToolContainer;

pub trait ToolDomain {
    fn register_in(container: ToolContainer) -> ToolContainer;
}
