use std::{fmt::Display, sync::Arc, time::Duration};

use rand::distr::{Alphanumeric, SampleString};
use serde_json::Value;

use crate::{
    Result,
    agent::context::DedicatedContext,
    approval::{Approval, NeededPermission, ParameterValue},
};

pub struct ApprovalBuilder(Approval);

impl ApprovalBuilder {
    pub fn new<T: AsRef<str>>(display_description: T, permissions: NeededPermission) -> Self {
        Self(Approval {
            id: Alphanumeric.sample_string(&mut rand::rng(), 12),
            display_description: display_description.as_ref().to_string(),
            parameters: Vec::new(),
            approval_callback: None,
            timeout: Duration::from_secs(60),
            needs_permissions: permissions,
        })
    }

    #[inline]
    pub fn param_inline<K: AsRef<str>, V: Display>(mut self, key: K, value: V) -> Self {
        self.0.parameters.push((
            key.as_ref().to_string(),
            ParameterValue::Inline(format!("{}", value)),
        ));

        self
    }

    #[inline]
    pub fn param_field<K: AsRef<str>, V: Display>(mut self, key: K, value: V) -> Self {
        self.0.parameters.push((
            key.as_ref().to_string(),
            ParameterValue::Field(format!("{}", value)),
        ));

        self
    }

    #[inline]
    pub fn on_approval<
        F: FnOnce(Arc<DedicatedContext>) -> Fut + Send + Sync + 'static,
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
    pub fn build(self) -> Approval {
        self.0
    }
}
