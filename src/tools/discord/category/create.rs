use std::sync::Arc;

use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};
use serenity::all::{CreateChannel, Permissions};

use crate::{
    Result,
    agent::context::DedicatedContext,
    approval::{NeededPermission, builder::ApprovalBuilder},
    tools::Tool,
};

#[derive(Deserialize, JsonSchema)]
pub struct Params {
    #[schemars(description = "The name of the category.")]
    pub name: String,
}

pub struct CreateCategoryTool;

impl Tool for CreateCategoryTool {
    type Params = Params;
    type Returns = Value;

    fn tool_name(&self) -> &'static str {
        "category.create"
    }

    fn description(&self) -> &'static str {
        "Create a category with the given name in the guild."
    }

    async fn execute(
        &self,
        params: Self::Params,
        ctx: Arc<DedicatedContext>,
    ) -> Result<Self::Returns> {
        let Some(guild_id) = ctx.guild_id else {
            return Ok(json!({
                "error": "you are not operating in a guild"
            }));
        };

        let approval = ApprovalBuilder::new(
            "create a category",
            NeededPermission::Basic(Permissions::MANAGE_CHANNELS),
        )
        .param_inline("Category Name", &params.name)
        .on_approval(async move |ctx| {
            let category = match guild_id
                .create_channel(
                    ctx.http(),
                    CreateChannel::new(params.name).kind(serenity::all::ChannelType::Category),
                )
                .await
            {
                Ok(c) => c,
                Err(err) => {
                    return Ok(Some(json!({
                        "error": format!("unable to create category: {err}")
                    })));
                }
            };

            Ok(Some(json!({
                "created_category_id": category.id,
            })))
        })
        .build();

        let approval_id = ctx.approval_manager.register(ctx.clone(), approval).await?;

        Ok(json!({
            "awaiting_approval": true,
            "approval_id": approval_id,
        }))
    }
}
