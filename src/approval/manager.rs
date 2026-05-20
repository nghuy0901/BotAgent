use std::sync::Arc;

use dashmap::DashMap;
use parking_lot::Mutex;
use serenity::all::{CreateEmbed, CreateEmbedFooter, EditMessage};

use crate::{
    Result,
    agent::{
        context::DedicatedContext,
        event::{AgentEvent, EventContent},
    },
    approval::{APPROVAL_TIMEOUT, Approval, NeededPermission},
};

type ApprovalArc = Arc<Mutex<Option<Approval>>>;

#[derive(Default)]
pub struct ApprovalManager {
    pending_approvals: DashMap<String, ApprovalArc>,
}

impl ApprovalManager {
    pub async fn register(&self, ctx: Arc<DedicatedContext>, approval: Approval) -> Result<String> {
        let mut message = approval.send_embed(&ctx).await?;

        let approval_id = approval.id.clone();
        let approval = Arc::new(Mutex::new(Some(approval)));

        self.pending_approvals
            .insert(approval_id.clone(), approval.clone());

        let id = approval_id.clone();

        tokio::task::spawn(async move {
            tokio::time::sleep(APPROVAL_TIMEOUT).await;

            if let Some(approval) = ctx.approval_manager.take(&id) {
                let _ = message
                    .edit(
                        ctx.http(),
                        EditMessage::new()
                            .components(vec![])
                            .remove_all_attachments()
                            .embed(
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
                            approval_id: approval.id,
                            metadata: approval.metadata,
                        }))
                        .await
                {
                    tracing::error!(
                        "unable to send timed out event to agent of channel {}: {:?}",
                        ctx.channel_id,
                        err
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
