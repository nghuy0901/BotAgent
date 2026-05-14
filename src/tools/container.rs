use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use async_openai::types::chat::{ChatCompletionTool, ChatCompletionTools, FunctionObject};

use crate::tools::{
    Tool, ToolHolder, domain::ToolDomain, parameters::Parameters, query::ToolQuery,
};

#[derive(Default, Clone)]
pub struct ToolObjectList(Vec<FunctionObject>);

impl From<ToolObjectList> for Vec<ChatCompletionTools> {
    fn from(value: ToolObjectList) -> Self {
        value
            .0
            .into_iter()
            .map(|function| ChatCompletionTools::Function(ChatCompletionTool { function }))
            .collect()
    }
}

impl From<Vec<FunctionObject>> for ToolObjectList {
    fn from(value: Vec<FunctionObject>) -> Self {
        Self(value)
    }
}

impl Deref for ToolObjectList {
    type Target = Vec<FunctionObject>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ToolObjectList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default)]
pub struct ToolContainer {
    pub tools: HashMap<String, Box<dyn ToolHolder>>,
    pub tool_infos: ToolObjectList,
}

impl ToolContainer {
    pub fn query(&self) -> ToolQuery<'_> {
        ToolQuery {
            container: self,
            excluded: Vec::new(),
        }
    }

    pub fn with_tool<T: Tool + 'static>(mut self, tool: T) -> Self {
        let name = tool.tool_name().to_string();
        let description = tool.description().to_string();

        self.tool_infos.push(FunctionObject {
            name: name.clone(),
            description: Some(description.clone()),
            parameters: Some(T::Params::into_schema()),
            strict: None,
        });

        self.tools.insert(name, Box::new(tool));

        self
    }

    pub fn with_domain<D: ToolDomain>(self) -> Self {
        D::register_in(self)
    }
}
