use std::collections::HashMap;

use async_openai::types::chat::{ChatCompletionTool, ChatCompletionTools, FunctionObject};

use crate::tools::{Tool, ToolHolder, domain::ToolDomain, parameters::Parameters};

#[derive(Default)]
pub struct ToolContainer {
    pub tools: HashMap<String, Box<dyn ToolHolder>>,
    pub tool_infos: Vec<ChatCompletionTools>,
}

impl ToolContainer {
    pub fn with_tool<T: Tool + 'static>(mut self, tool: T) -> Self {
        let name = tool.tool_name().to_string();
        let description = tool.description().to_string();

        self.tool_infos
            .push(ChatCompletionTools::Function(ChatCompletionTool {
                function: FunctionObject {
                    name: name.clone(),
                    description: Some(description.clone()),
                    parameters: Some(T::Params::into_schema()),
                    strict: None,
                },
            }));

        self.tools.insert(name, Box::new(tool));

        self
    }

    pub fn with_domain<D: ToolDomain>(self) -> Self {
        D::register_in(self)
    }
}
