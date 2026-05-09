use async_openai::config::Config;
use serde_json::json;
use tokio::{
    sync::mpsc::{self, Receiver},
    task::JoinHandle,
};
use tracing::error;

use crate::{
    agent::{Agent, event::AgentEvent},
    constant::COLLECT_TIMESPAN,
};

pub struct AgentChannel {
    pub tx: mpsc::Sender<AgentEvent>,

    task_handle: JoinHandle<()>,
}

impl AgentChannel {
    pub fn new<C: Config + 'static>(agent: Agent<C>) -> Self {
        // maximum 32 elements in the channel. maybe change to unbound later?
        let (tx, rx) = mpsc::channel(32);
        let task_handle = tokio::task::spawn(async move { channel_thread(agent, rx).await });

        Self { tx, task_handle }
    }
}

impl Drop for AgentChannel {
    fn drop(&mut self) {
        if !self.task_handle.is_finished() {
            self.task_handle.abort();
        }
    }
}

async fn channel_thread<C: Config>(mut agent: Agent<C>, mut rx: Receiver<AgentEvent>) {
    while let Some(event) = rx.recv().await {
        let mut events = vec![event];

        tokio::time::sleep(COLLECT_TIMESPAN).await;

        while let Ok(event_2) = rx.try_recv() {
            events.push(event_2);
        }

        match agent.chat(Some(json!(events).to_string())).await {
            Ok(_resp) => {}
            Err(err) => error!("agent errored: {:?}", err),
        }
    }
}
