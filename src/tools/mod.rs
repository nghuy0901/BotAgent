use std::{pin::Pin, sync::Arc};

use serde::Serialize;
use serde_json::{Value, json};
use thiserror::Error;
use tracing::instrument;

use crate::{Result, agent::context::DedicatedContext, tools::arguments::Arguments};

pub mod arguments;
pub mod container;
pub mod domain;
pub mod impls;
pub mod query;

#[derive(Error, Debug)]
pub enum ToolError {
    #[error("{0}")]
    Custom(String),
    #[error("validation error with argument \"{argument}\": {reason}")]
    Validation { argument: String, reason: String },
    #[error("discord error: {0}")]
    Serenity(#[from] serenity::Error),
    #[error("{0}")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

impl ToolError {
    pub fn custom<T: AsRef<str>>(reason: T) -> Self {
        Self::Custom(reason.as_ref().to_string())
    }

    pub fn validation<T: AsRef<str>, R: AsRef<str>>(argument: T, reason: R) -> Self {
        Self::Validation {
            argument: argument.as_ref().to_string(),
            reason: reason.as_ref().to_string(),
        }
    }
}

#[derive(Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Status<T: Serialize> {
    Success {
        data: T,
    },
    PendingApproval {
        approval_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        data: Option<T>,
    },
}

impl<T: Serialize> Status<T> {
    pub fn success(data: T) -> Self {
        Self::Success { data }
    }

    pub fn pending_approval<I: AsRef<str>>(approval_id: I, data: Option<T>) -> Self {
        Self::PendingApproval {
            approval_id: approval_id.as_ref().to_string(),
            data,
        }
    }
}

pub type ToolResult<T> = std::result::Result<T, ToolError>;

pub trait Tool: Send + Sync {
    type Args: Arguments;
    type Returns: Serialize;

    fn tool_name(&self) -> &'static str;
    fn description(&self) -> &'static str;

    fn execute(
        &self,
        args: Self::Args,
        ctx: Arc<DedicatedContext>,
    ) -> impl Future<Output = ToolResult<Status<Self::Returns>>> + Send;
}

pub trait ToolHolder: Send + Sync {
    fn execute(
        &self,
        args: Value,
        ctx: Arc<DedicatedContext>,
    ) -> Pin<Box<dyn Future<Output = Result<String>> + '_ + Send>>;
}

impl<T: Tool> ToolHolder for T {
    #[instrument(name = "execute_tool", skip_all, fields(name = self.tool_name(), args = %args.to_string()))]
    fn execute(
        &self,
        args: Value,
        ctx: Arc<DedicatedContext>,
    ) -> Pin<Box<dyn Future<Output = Result<String>> + '_ + Send>> {
        Box::pin(async move {
            tracing::debug!("parsing arguments");

            let args = match serde_json::from_value(args) {
                Ok(p) => p,
                Err(err) => {
                    return Ok(ToolError::custom(format!(
                        "unable to parse input arguments: {err}"
                    ))
                    .to_string());
                }
            };

            tracing::debug!("calling tool");

            let value = match T::execute(self, args, ctx).await {
                Ok(s) => serde_json::to_value(s)?,
                Err(ToolError::Other(err)) => return Err(err),
                Err(e) => json!({
                    "status": "error",
                    "reason": e.to_string()
                }),
            };

            tracing::debug!(response = %value.to_string(), "completed");

            Ok(value.to_string())
        })
    }
}
