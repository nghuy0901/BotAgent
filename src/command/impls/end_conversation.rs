use std::sync::Arc;

use serenity::all::{
    CommandInteraction, CreateCommand, CreateEmbed, CreateEmbedAuthor, CreateInteractionResponse,
    CreateInteractionResponseMessage, Http,
};

use crate::{
    agent::context::AgentContext,
    command::{Command, CommandError},
};

/// A command used for ending a conversation with Sweep while it's in the wake-on-mention mode.
pub struct EndConversationCommand;

impl Command for EndConversationCommand {
    const NAME: &str = "end_conversation";

    fn register(command: CreateCommand) -> CreateCommand {
        command
            .description("Ends the current conversation. Only available in wake-on-mention mode.")
    }

    async fn run(
        &self,
        command: CommandInteraction,
        ctx: Arc<AgentContext>,
    ) -> Result<(), CommandError> {
        let Some(agent_channel) = ctx.agents.get(&command.channel_id).map(|a| a.clone()) else {
            reply_ephemeral(
                &command,
                ctx.http(),
                "This channel doesn't have any ongoing conversation.",
            )
            .await?;

            return Ok(());
        };

        if !agent_channel
            .dedicated_context
            .participants
            .contains(&command.user.id)
        {
            reply_ephemeral(
                &command,
                ctx.http(),
                "Only participants may end the conversation.",
            )
            .await?;

            return Ok(());
        }

        if ctx.agents.remove(&command.channel_id).is_none() {
            reply_ephemeral(
                &command,
                ctx.http(),
                "The conversation has already been ended by another user.",
            )
            .await?;

            return Ok(());
        };

        tracing::debug!(channel_id = %command.channel_id, "agent removed");

        let avatar = command
            .user
            .avatar_url()
            .unwrap_or_else(|| command.user.default_avatar_url());

        let user_id = command.user.id;

        let embed = CreateEmbed::new()
            .color(0xa6d189)
            .author(CreateEmbedAuthor::new(&command.user.name).icon_url(avatar))
            .title("🗑️ The conversation has been ended!")
            .description(format!("-# Ended by <@{user_id}>"));

        command
            .create_response(
                ctx.http(),
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new().embed(embed),
                ),
            )
            .await?;

        Ok(())
    }
}

async fn reply_ephemeral(
    command: &CommandInteraction,
    http: &Http,
    message: &str,
) -> Result<(), CommandError> {
    command
        .create_response(
            http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content(message)
                    .ephemeral(true),
            ),
        )
        .await?;

    Ok(())
}
