pub mod create;

use crate::tools::{
    container::ToolContainer, domain::ToolDomain, impls::category::create::CreateCategoryTool,
};

pub struct CategoryTools;

impl ToolDomain for CategoryTools {
    fn register_in(container: ToolContainer) -> ToolContainer {
        container.with_tool(CreateCategoryTool)
    }
}
