use std::{
    pin::Pin,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use serde_json::Value;
use serenity::all::{
    ChannelId, Color, CreateActionRow, CreateAttachment, CreateButton, CreateEmbed, CreateMessage,
    Message, Permissions,
};

use crate::{Result, agent::context::DedicatedContext, approval::metadata::ApprovalMetadata};

pub mod builder;
pub mod manager;
pub mod metadata;

const APPROVAL_TIMEOUT: Duration = Duration::from_secs(30);
const SIDEBAR_COLOR: Color = Color::new(0x5665F2);

pub type AsyncCallback<T> =
    Box<dyn FnOnce(Arc<DedicatedContext>) -> Pin<Box<dyn Future<Output = T> + Send>> + Send + Sync>;

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

pub enum ParameterValue {
    Inline(String),
    Field(String),
}

pub struct Approval {
    pub id: String,
    pub parameters: Vec<(String, ParameterValue)>,
    pub approval_callback: Option<AsyncCallback<Result<Option<Value>>>>,
    pub needs_permissions: NeededPermission,
    pub metadata: ApprovalMetadata,
}

impl Approval {
    pub async fn send_embed(&self, ctx: &Arc<DedicatedContext>) -> Result<Message> {
        let mut file_attachments = vec![];

        let permission_names = self.needs_permissions.get_permission_names();
        let plural = if permission_names.len() == 1 { "" } else { "s" };
        let channel_suffix = match &self.needs_permissions {
            NeededPermission::InChannel(channel_id, _) => {
                format!(" in the <#{}> channel", channel_id)
            }
            _ => String::new(),
        };

        let description = format!(
            "I want to **{}**, but I need permission from someone who has the **`{}`** permission{}{}.",
            self.metadata.action,
            permission_names.join(", "),
            plural,
            channel_suffix
        );

        let mut fields = Vec::new();
        let mut inline_parameters = Vec::new();

        for (key, value) in &self.parameters {
            match value {
                ParameterValue::Inline(value) => {
                    inline_parameters.push((key, value.clone()));
                }
                ParameterValue::Field(value) if value.len() > 512 => {
                    let file_name = format!("{}.md", key.to_lowercase().replace(" ", "_"));

                    inline_parameters.push((key, format!("Attached as {}", file_name)));
                    file_attachments.push(CreateAttachment::bytes(value.as_bytes(), file_name));
                }
                ParameterValue::Field(value) => {
                    fields.push((
                        format!("📝 {key}"),
                        format!("```md\n{}```", value.replace('`', "\\`")),
                        false,
                    ));
                }
            }
        }

        if !inline_parameters.is_empty() {
            let lines = inline_parameters
                .iter()
                .enumerate()
                .map(|(i, (key, value))| {
                    let branch = if i + 1 == inline_parameters.len() {
                        "└"
                    } else {
                        "├"
                    };

                    format!("-# {branch} **{key} → {value}**")
                })
                .collect::<Vec<_>>()
                .join("\n");

            fields.push(("📋 Parameters".to_string(), lines, false));
        }

        let footer = match (SystemTime::now() + APPROVAL_TIMEOUT).duration_since(UNIX_EPOCH) {
            Ok(timestamp) => format!("Expires → <t:{}:R>", timestamp.as_secs()),
            Err(_) => format!("Timeout → {} seconds", APPROVAL_TIMEOUT.as_secs()),
        };

        if let Some(last) = fields.last_mut() {
            last.1
                .push_str(&format!("\n\n-# Request ID → `{}`\n-# {}", self.id, footer));
        }

        let embed = CreateEmbed::new()
            .color(SIDEBAR_COLOR)
            .title("🔐 Approval Required")
            .description(description)
            .fields(fields);

        let message = CreateMessage::new()
            .embed(embed)
            .components(vec![CreateActionRow::Buttons(vec![
                CreateButton::new(format!("approve-{}", self.id))
                    .emoji('✅')
                    .label("Approve")
                    .style(serenity::all::ButtonStyle::Secondary),
                CreateButton::new(format!("deny-{}", self.id))
                    .emoji('❌')
                    .label("Deny")
                    .style(serenity::all::ButtonStyle::Secondary),
            ])]);

        Ok(if file_attachments.is_empty() {
            ctx.channel_id.send_message(ctx.http(), message).await?
        } else {
            ctx.channel_id
                .send_files(ctx.http(), file_attachments, message)
                .await?
        })
    }
}
