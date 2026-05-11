use std::{pin::Pin, time::Duration};

use serde_json::Value;
use serenity::all::{
    ChannelId, CreateActionRow, CreateButton, CreateEmbed, CreateEmbedFooter, CreateMessage,
    Permissions,
};

use crate::agent::{Result, tools::discord::DiscordContext};

pub mod builder;
pub mod manager;

pub type AsyncCallback<T> =
    Box<dyn FnOnce(DiscordContext) -> Pin<Box<dyn Future<Output = T> + Send>> + Send + Sync>;

#[derive(Clone)]
pub enum NeededPermission {
    Basic(Permissions),
    InChannel(ChannelId, Permissions),
}

impl NeededPermission {
    pub fn get_permission_names(&self) -> Vec<&str> {
        match self {
            Self::Basic(p) => p.get_permission_names(),
            Self::InChannel(_, p) => p.get_permission_names(),
        }
    }
}

pub struct BasicApproval {
    pub needs_permissions: NeededPermission,
}

pub enum ParameterValue {
    Inline(String),
    Field(String),
}

pub struct Approval {
    pub id: String,
    pub display_description: String,
    pub parameters: Vec<(String, ParameterValue)>,
    pub approval_callback: Box<Option<AsyncCallback<Result<Option<Value>>>>>,
    pub timeout: Duration,
    pub needs_permissions: NeededPermission,
}

impl Approval {
    pub fn to_embed(&self) -> CreateEmbed {
        let permission_names = self.needs_permissions.get_permission_names();

        CreateEmbed::new()
            .title("⚠️ I need approval!")
            .footer(CreateEmbedFooter::new(format!(
                "Approval ID: {} | Timeout: {}s",
                self.id,
                self.timeout.as_secs_f32()
            )))
            .description(format!(
                "I would like to **{}**, but I need approval from someone that has the **`{}`** permission{}{}.\n\n{}",
                self.display_description,
                permission_names.join(", "),
                if permission_names.len() != 1 { "s" } else { "" },
                if let NeededPermission::InChannel(channel_id, _) = &self.needs_permissions { format!(" in the <#{}> channel", channel_id) } else { String::new() },
                self.parameters
                    .iter()
                    .filter_map(|(k, v)| {
                        if let ParameterValue::Inline(text) = v {
                            Some(format!("**{k}** → `{text}`"))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            )).fields(self.parameters
                .iter()
                .filter_map(|(k, v)| {
                    if let ParameterValue::Field(text) = v {
                        Some((k, format!("```\n{}\n```", text), true))
                    } else {
                        None
                    }
                }))
    }

    pub fn to_message(&self) -> CreateMessage {
        CreateMessage::new()
            .embed(self.to_embed())
            .components(vec![CreateActionRow::Buttons(vec![
                CreateButton::new(format!("approve-{}", self.id))
                    .label("Approve")
                    .style(serenity::all::ButtonStyle::Primary),
                CreateButton::new(format!("deny-{}", self.id))
                    .label("Deny")
                    .style(serenity::all::ButtonStyle::Secondary),
            ])])
    }
}
