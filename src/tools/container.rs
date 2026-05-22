use std::collections::{HashMap, HashSet};

use openai_dive::v1::resources::chat::{
    ChatCompletionFunction, ChatCompletionTool, ChatCompletionToolType,
};

use crate::tools::{Tool, ToolHolder, arguments::Arguments, domain::ToolDomain, query::ToolQuery};

#[derive(Default)]
pub struct ToolContainer {
    pub tools: HashMap<String, Box<dyn ToolHolder>>,
    pub infos: Vec<ChatCompletionTool>,
}

impl ToolContainer {
    pub fn query(&self) -> ToolQuery<'_> {
        ToolQuery {
            container: self,
            excluded: HashSet::new(),
        }
    }

    pub fn with_tool<T: Tool + 'static>(mut self, tool: T) -> Self {
        let name = tool.tool_name().to_string();
        let description = tool.description().to_string();

        self.infos.push(ChatCompletionTool {
            r#type: ChatCompletionToolType::Function,
            function: ChatCompletionFunction {
                name: name.clone(),
                description: Some(description),
                parameters: T::Args::into_schema(),
            },
        });

        self.tools.insert(name, Box::new(tool));

        self
    }

    pub fn with_domain<D: ToolDomain>(self) -> Self {
        D::register_in(self)
    }
}
