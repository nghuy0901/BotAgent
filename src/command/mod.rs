pub mod dispatcher;
pub mod impls;

use std::{pin::Pin, sync::Arc};

use serenity::all::{CommandInteraction, CreateCommand};
use thiserror::Error;

use crate::agent::context::AgentContext;

/// A Discord slash command.
pub trait Command: Send + Sync {
    // The name of the slash command
    const NAME: &str;

    fn build() -> CreateCommand {
        Self::register(CreateCommand::new(Self::NAME))
    }

    fn register(command: CreateCommand) -> CreateCommand;

    fn run(
        &self,
        command: CommandInteraction,
        ctx: Arc<AgentContext>,
    ) -> impl Future<Output = Result<(), CommandError>> + Send;
}

pub trait CommandHolder: Send + Sync {
    fn name(&self) -> &'static str;
    fn build(&self) -> CreateCommand;

    fn run(
        &self,
        command: CommandInteraction,
        ctx: Arc<AgentContext>,
    ) -> Pin<Box<dyn Future<Output = Result<(), CommandError>> + '_ + Send>>;
}

impl<T: Command> CommandHolder for T {
    fn name(&self) -> &'static str {
        T::NAME
    }

    fn build(&self) -> CreateCommand {
        T::build()
    }

    fn run(
        &self,
        command: CommandInteraction,
        ctx: Arc<AgentContext>,
    ) -> Pin<Box<dyn Future<Output = Result<(), CommandError>> + '_ + Send>> {
        Box::pin(T::run(self, command, ctx))
    }
}

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("discord error: {0}")]
    Serenity(#[from] serenity::Error),
}
