pub mod create;

use crate::tools::{
    container::ToolContainer, discord::category::create::CreateCategoryTool, domain::ToolDomain,
};

pub struct CategoryTools;

impl ToolDomain for CategoryTools {
    fn register_in(container: ToolContainer) -> ToolContainer {
        container.with_tool(CreateCategoryTool)
    }
}
