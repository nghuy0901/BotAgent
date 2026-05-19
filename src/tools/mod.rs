use std::{pin::Pin, sync::Arc};

use serde::Serialize;
use serde_json::{Value, json};
use thiserror::Error;

use crate::{Result, agent::context::DedicatedContext, tools::parameters::Parameters};

pub mod basic;
pub mod container;
pub mod discord;
pub mod domain;
pub mod parameters;
pub mod query;

#[derive(Error, Debug)]
pub enum ToolError {
    #[error("{0}")]
    Custom(String),
    #[error("validation error with parameter \"{parameter}\": {reason}")]
    Validation { parameter: String, reason: String },
    #[error("discord error: {0}")]
    Serenity(#[from] serenity::Error),
    #[error("{0}")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

impl ToolError {
    pub fn custom<T: AsRef<str>>(reason: T) -> Self {
        Self::Custom(reason.as_ref().to_string())
    }

    pub fn validation<T: AsRef<str>, R: AsRef<str>>(parameter: T, reason: R) -> Self {
        Self::Validation {
            parameter: parameter.as_ref().to_string(),
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
    type Params: Parameters;
    type Returns: Serialize;

    fn tool_name(&self) -> &'static str;
    fn description(&self) -> &'static str;

    fn execute(
        &self,
        params: Self::Params,
        ctx: Arc<DedicatedContext>,
    ) -> impl Future<Output = ToolResult<Status<Self::Returns>>> + Send;
}

pub trait ToolHolder: Send + Sync {
    fn execute(
        &self,
        params: Value,
        ctx: Arc<DedicatedContext>,
    ) -> Pin<Box<dyn Future<Output = Result<String>> + '_ + Send>>;
}

impl<T: Tool> ToolHolder for T {
    fn execute(
        &self,
        params: Value,
        ctx: Arc<DedicatedContext>,
    ) -> Pin<Box<dyn Future<Output = Result<String>> + '_ + Send>> {
        Box::pin(async move {
            let param = serde_json::from_value(params)?;

            match T::execute(self, param, ctx).await {
                Ok(s) => Ok(serde_json::to_string(&s)?),
                Err(ToolError::Other(err)) => Err(err),
                Err(e) => Ok(json!({
                    "status": "error",
                    "reason": e.to_string()
                })
                .to_string()),
            }
        })
    }
}
