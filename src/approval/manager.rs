use std::sync::Arc;

use dashmap::DashMap;
use parking_lot::Mutex;
use serenity::all::{Channel, CreateEmbed, CreateEmbedFooter, EditMessage};
use tracing::error;

use crate::{
    Result,
    agent::{
        context::DedicatedContext,
        event::{AgentEvent, EventContent},
    },
    approval::{Approval, NeededPermission},
};

type ApprovalArc = Arc<Mutex<Option<Approval>>>;

#[derive(Default)]
pub struct ApprovalManager {
    pending_approvals: DashMap<String, ApprovalArc>,
}

impl ApprovalManager {
    pub async fn register(&self, ctx: Arc<DedicatedContext>, approval: Approval) -> Result<String> {
        let channel = ctx.channel_id.to_channel(ctx.http()).await?;

        let mut message = match channel {
            Channel::Guild(g) => g.send_message(ctx.http(), approval.to_message()).await?,
            Channel::Private(p) => p.send_message(ctx.http(), approval.to_message()).await?,
            _ => return Err("unknown channel kind".into()),
        };

        let approval_id = approval.id.clone();
        let timeout = approval.timeout;

        let approval = Arc::new(Mutex::new(Some(approval)));

        self.pending_approvals
            .insert(approval_id.clone(), approval.clone());

        let id = approval_id.clone();

        tokio::task::spawn(async move {
            tokio::time::sleep(timeout).await;

            if ctx.approval_manager().take(&id).is_some() {
                let _ = message
                    .edit(
                        ctx.http(),
                        EditMessage::new().components(vec![]).embed(
                            CreateEmbed::new()
                                .title("⌛ Timed Out")
                                .description("This approval has timed out.")
                                .footer(CreateEmbedFooter::new("")),
                        ),
                    )
                    .await;

                if let Some(agent) = ctx.agent_context.agents.get(&ctx.channel_id)
                    && let Err(err) = agent
                        .tx
                        .send(AgentEvent::new(EventContent::RequestTimedOut {
                            approval_id: id,
                        }))
                        .await
                {
                    error!(
                        "unable to send timed out event to agent of channel {}: {:?}",
                        ctx.channel_id, err
                    );
                }
            }
        });

        Ok(approval_id)
    }

    pub fn get_needed_permission<T: AsRef<str>>(&self, id: T) -> Option<NeededPermission> {
        self.pending_approvals
            .get(id.as_ref())?
            .lock()
            .as_ref()
            .map(|a| a.needs_permissions.clone())
    }

    pub fn take<T: AsRef<str>>(&self, id: T) -> Option<Approval> {
        let taken = self
            .pending_approvals
            .get(id.as_ref())
            .and_then(|a| a.lock().take());

        self.pending_approvals.remove(id.as_ref());

        taken
    }
}
