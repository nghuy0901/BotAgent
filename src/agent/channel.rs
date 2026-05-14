use std::{sync::Arc, time::Duration};

use serde_json::json;
use tokio::{
    sync::mpsc::{self, Receiver},
    task::JoinHandle,
};
use tracing::error;

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

async fn channel_thread(mut agent: Agent, mut rx: Receiver<AgentEvent>) {
    let duration = Duration::from_millis(agent.dedicated_context.configuration.bot.debounce_ms);

    while let Some(event) = rx.recv().await {
        let mut events = vec![event];

        tokio::time::sleep(duration).await;

        while let Ok(event_2) = rx.try_recv() {
            events.push(event_2);
        }

        match agent.chat(Some(json!(events).to_string())).await {
            Ok(_resp) => {}
            Err(err) => error!("agent errored: {:?}", err),
        }
    }
}
