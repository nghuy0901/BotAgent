use std::collections::HashMap;

use async_openai::types::chat::{ChatCompletionTool, ChatCompletionTools, FunctionObject};

use crate::agent::tools::{
    Parameters,
    basic::{BasicTool, BasicToolHolder},
    discord::{DiscordTool, DiscordToolHolder},
};

pub enum ToolEntry {
    DiscordTool(Box<dyn DiscordToolHolder>),
    BasicTool(Box<dyn BasicToolHolder>),
}
#[derive(Default)]
pub struct ToolContainer {
    pub tools: HashMap<String, ToolEntry>,
    //pub basic_tools: HashMap<String, Box<dyn BasicToolHolder>>,
    pub tool_infos: Vec<ChatCompletionTools>,
}

impl ToolContainer {
    pub fn with_discord_tool<T: DiscordTool + 'static>(mut self, tool: T) -> Self {
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

        self.tools
            .insert(name, ToolEntry::DiscordTool(Box::new(tool)));

        self
    }

    pub fn with_basic_tool<T: BasicTool + 'static>(mut self, tool: T) -> Self {
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

        self.tools
            .insert(name, ToolEntry::BasicTool(Box::new(tool)));

        self
    }
}
