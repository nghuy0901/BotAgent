use std::{fmt::Display, time::Duration};

use rand::distr::{Alphanumeric, SampleString};
use serde_json::Value;
use serenity::all::Permissions;

use crate::agent::{Result, approval::Approval, tools::discord::DiscordContext};

pub struct ApprovalBuilder(Approval);

impl ApprovalBuilder {
    pub fn new<T: AsRef<str>>(display_description: T, permissions: Permissions) -> Self {
        Self(Approval {
            id: Alphanumeric.sample_string(&mut rand::rng(), 12),
            display_description: display_description.as_ref().to_string(),
            parameters: Vec::new(),
            approval_callback: Box::new(None),
            timeout: Duration::from_secs(60),
            needs_permissions: permissions,
        })
    }

    #[inline]
    pub fn param<K: AsRef<str>, V: Display>(mut self, key: K, value: V) -> Self {
        self.0
            .parameters
            .push((key.as_ref().to_string(), format!("{}", value)));

        self
    }

    #[inline]
    pub fn on_approval<
        F: FnOnce(DiscordContext) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Option<Value>>> + Send,
    >(
        mut self,
        func: F,
    ) -> Self {
        self.0.approval_callback.replace(Box::new(move |ctx| {
            Box::pin(async move { func(ctx).await })
        }));

        self
    }

    #[inline]
    pub fn timeout(mut self, duration: Duration) -> Self {
        self.0.timeout = duration;

        self
    }

    #[inline]
    pub fn build(self) -> Approval {
        self.0
    }
}
