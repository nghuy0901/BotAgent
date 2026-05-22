use std::{collections::HashMap, sync::Arc};

use serenity::all::{CommandInteraction, CreateCommand};
use thiserror::Error;

use crate::{
    agent::context::AgentContext,
    command::{CommandError, CommandHolder},
};

/// Holds all registered slash commands and handles their execution.
#[derive(Default)]
pub struct CommandDispatcher {
    map: HashMap<String, Box<dyn CommandHolder>>,
}

#[derive(Debug, Error)]
pub enum DispatchError {
    #[error("unknown command \"{0}\"")]
    UnknownCommand(String),
    #[error("command errored: {0}")]
    CommandError(#[from] CommandError),
}

impl CommandDispatcher {
    /// Gets a [Vec] containing all command's [CreateCommand]s
    pub fn all(&self) -> Vec<(&'static str, CreateCommand)> {
        self.map.values().map(|v| (v.name(), v.build())).collect()
    }

    /// Registers a new command in the dispatcher.
    pub fn with_command<C: CommandHolder + 'static>(mut self, command: C) -> Self {
        self.map
            .insert(command.name().to_string(), Box::new(command));

        self
    }

    /// Tries to find the command handler for the interaction and executes it if found.
    pub async fn dispatch(
        &self,
        interaction: CommandInteraction,
        ctx: Arc<AgentContext>,
    ) -> Result<(), DispatchError> {
        match self.map.get(&interaction.data.name) {
            Some(command) => Ok(command.run(interaction, ctx).await?),
            None => Err(DispatchError::UnknownCommand(interaction.data.name)),
        }
    }
}
