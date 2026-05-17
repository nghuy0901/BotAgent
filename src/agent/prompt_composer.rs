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
    if ctx.configuration.bot.typing_indicator {
        r#"# Tool Rules

        - Always calls `start_typing` in the same batch while sending a message.
        - You MUST send a message to the user after calling `start_typing`.
        - Never call `start_typing` if you know that you will not send a message.

        Examples:

        <example>
            user: hey sweep, how are you doing?
            assistant: start_typing()
            assistant: channel.send_message(channel_id, content: "Hey there! I am doing great.")
        </example>

        <example>
            user: Hey Sweep! What's the time?
            assistant: time.get_local() // returns timestamp 123456
            assistant: start_typing()
            assistant: channel.send_message(channel_id, content: "Hey! It is currently <t:123456>.")
            assistant: end_turn()
            user: Okay thanks! Goodbye!
            assistant: channel.send_message(channel_id, content: "No problem! Bye, bye!")
            assistant: end_turn()
        </example>"#
    } else {
        ""
    }
    .to_string()
}

pub fn build_capabilities(ctx: &Arc<DedicatedContext>) -> String {
    let mut lines = vec![
        "Sweep's capabilities are influenced by the available set of tools.".to_string(),
        "Always let a user know if a tool is unavailable or disabled. Never include such tools in a response.".to_string(),
        "Sweep will never talk about tools that don't exist.".to_string(),
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
