use std::collections::HashSet;

use openai_dive::v1::resources::chat::ChatCompletionTool;

use crate::tool::container::ToolContainer;

pub struct ToolQuery<'a> {
    pub container: &'a ToolContainer,
    pub excluded: HashSet<String>,
}

impl<'a> ToolQuery<'a> {
    pub fn exclude_list(mut self, names: Vec<String>) -> Self {
        self.excluded.extend(names);

        self
    }

    pub fn exclude_all(mut self) -> Self {
        self.excluded
            .extend(self.container.infos.iter().map(|t| t.function.name.clone()));

        self
    }

    pub fn exclude_if(mut self, condition: bool, name: &str) -> Self {
        if condition {
            self.excluded.insert(name.to_string());
        }

        self
    }

    pub fn include_list(mut self, names: &[String]) -> Self {
        for name in names {
            self.excluded.remove(name);
        }

        self
    }

    pub fn run(self) -> Vec<ChatCompletionTool> {
        self.container
            .infos
            .iter()
            .filter(|t| !self.excluded.contains(&t.function.name))
            .cloned()
            .collect::<Vec<_>>()
    }
}
