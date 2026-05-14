use crate::tools::container::{ToolContainer, ToolObjectList};

pub struct ToolQuery<'a> {
    pub container: &'a ToolContainer,
    pub excluded: Vec<String>,
}

impl<'a> ToolQuery<'a> {
    pub fn exclude_if<T: AsRef<str>>(mut self, condition: bool, name: T) -> Self {
        if condition {
            self.excluded.push(name.as_ref().to_string());
        }

        self
    }

    pub fn exclude_list(mut self, names: Vec<String>) -> Self {
        self.excluded.extend(names);

        self
    }

    pub fn run(self) -> ToolObjectList {
        self.container
            .tool_infos
            .iter()
            .filter(|t| !self.excluded.contains(&t.name))
            .cloned()
            .collect::<Vec<_>>()
            .into()
    }
}
