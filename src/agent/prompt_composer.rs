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

pub fn build_tool_rules(ctx: &Arc<DedicatedContext>) -> String {
    let mut rules = vec![
        "Always batch tool calls when possible.",
        "Only call `end_turn` if you have nothing left to do.",
    ];

    if ctx.configuration.bot.typing_indicator {
        rules.push("Always call `start_typing` before sending a response.");
    }

    format!("<tool_rules>\n{}\n</tool_rules>", rules.join("\n"))
}
