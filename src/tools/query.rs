use openai_dive::v1::resources::chat::ChatCompletionTool;

use crate::tools::container::ToolContainer;

pub struct ToolQuery<'a> {
    pub container: &'a ToolContainer,
    pub excluded: Vec<String>,
}

impl<'a> ToolQuery<'a> {
    pub fn exclude_list(mut self, names: Vec<String>) -> Self {
        self.excluded.extend(names);

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
