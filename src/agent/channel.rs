use std::{sync::Arc, time::Duration};

use serde_json::json;
use tokio::{
    sync::mpsc::{self, Receiver},
    task::JoinHandle,
};
use tracing::instrument;

use crate::agent::{Agent, context::DedicatedContext, event::AgentEvent};

pub struct AgentChannel {
    pub dedicated_context: Arc<DedicatedContext>,
    pub tx: mpsc::Sender<AgentEvent>,

    task_handle: JoinHandle<()>,
}

impl AgentChannel {
    pub fn new(agent: Agent) -> Self {
        let dedicated_context = agent.dedicated_context.clone();

        // maximum 32 elements in the channel. maybe change to unbound later?
        let (tx, rx) = mpsc::channel(32);
        let task_handle = tokio::task::spawn(async move { channel_thread(agent, rx).await });

        Self {
            tx,
            task_handle,
            dedicated_context,
        }
    }
}

impl Drop for AgentChannel {
    fn drop(&mut self) {
        if !self.task_handle.is_finished() {
            self.task_handle.abort();
        }
    }
}

#[instrument(name = "agent", skip_all, fields(channel_id = %agent.dedicated_context.channel_id))]
async fn channel_thread(mut agent: Agent, mut rx: Receiver<AgentEvent>) {
    let config = &agent.dedicated_context.configuration;

    let duration = Duration::from_millis(config.bot.debounce_ms);
    let skip_completion_for = config
        .approval
        .skip_completion
        .iter()
        .map(|s| format!("request_{}", s))
        .collect::<Vec<_>>();

    while let Some(event) = rx.recv().await {
        let mut events = vec![event];

        tokio::time::sleep(duration).await;

        while let Ok(event_2) = rx.try_recv() {
            events.push(event_2);
        }

        if events
            .iter()
            .all(|e| skip_completion_for.contains(&e.name().to_string()))
        {
            tracing::debug!("skipping batch, all events are configured to skip completion");

            // We still want the timeout message to be in the chat.

            agent.add_user_message(json!(events).to_string());

            continue;
        }

        match agent.chat(json!(events).to_string()).await {
            Ok(_) => {}
            Err(err) => tracing::error!("agent errored: {:?}", err),
        }
    }
}
