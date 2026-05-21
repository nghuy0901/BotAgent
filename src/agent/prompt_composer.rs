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
    if ctx.configuration.bot.max_turns == 0 || ctx.tools.is_empty() {
        return "# Capabilities\n\nTools are currently disabled. You can only perform tasks that involve talking.".to_string();
    }

    let mut lines = vec![
        "- Your capabilities are defined by the available tools.".to_string(),
        "- Always let a user know if a tool is unavailable or disabled. Never include unavailable tools in a response.".to_string(),
        "- Never talk about tools that don't exist.".to_string(),
    ];

    let mut disabled_tools = vec![];
    let mut enabled_tools = vec![];

    for tool in &ctx.tool_container.infos {
        if ctx.tools.contains(tool) {
            enabled_tools.push(tool.function.name.clone());
        } else {
            disabled_tools.push(tool.function.name.clone());
        }
    }

    if !disabled_tools.is_empty() || !enabled_tools.is_empty() {
        lines.push("".to_string());
    }

    if !enabled_tools.is_empty() {
        lines.push(format!(
            "The following tools are enabled: `{}`",
            enabled_tools.join("`, `")
        ));
    }

    if !disabled_tools.is_empty() {
        lines.push(format!(
            "The following tools are disabled: `{}`",
            disabled_tools.join("`, `")
        ));
    }

    format!("# Capabilities\n\n{}", lines.join("\n"))
}
