use std::{collections::HashMap, sync::Arc};

use crate::agent::context::DedicatedContext;

pub struct PromptComposer {
    template: &'static str,
    sections: HashMap<&'static str, String>,
}

impl PromptComposer {
    pub fn new() -> Self {
        Self {
            template: include_str!("../../prompts/system.md"),
            sections: HashMap::new(),
        }
    }

    pub fn inject(mut self, key: &'static str, content: String) -> Self {
        self.sections.insert(key, content);

        self
    }

    pub fn build(self) -> String {
        let mut result = self.template.to_string();

        for (key, value) in self.sections {
            result = result.replace(&format!("%{{{}}}", key), &value);
        }

        result
    }
}

pub fn build_capabilities(ctx: &Arc<DedicatedContext>) -> String {
    let mut lines = vec![
        "Your capabilities are influenced by the available set of tools.".to_string(),
        "Always let a user know if a tool is unavailable or disabled. Never include unavailable tools in a response.".to_string(),
        "Never talk about tools that don't exist.".to_string(),
    ];

    if !ctx.configuration.tools.disable.is_empty() {
        lines.extend_from_slice(&[
            "".to_string(),
            format!(
                "The following tools are currently disabled: `{}`",
                ctx.configuration.tools.disable.join("`, `")
            ),
        ]);
    }

    format!("# Capabilities\n\n{}", lines.join("\n"))
}
