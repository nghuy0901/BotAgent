use std::{fmt::Display, sync::Arc};

use rand::distr::{Alphanumeric, SampleString};
use serde_json::Value;

use crate::{
    Result,
    agent::context::DedicatedContext,
    approval::{Approval, NeededPermission, ParameterValue, metadata::ApprovalMetadata},
};

pub struct ApprovalBuilder(Approval);

impl ApprovalBuilder {
    pub fn new<T: AsRef<str>>(display_description: T, permissions: NeededPermission) -> Self {
        Self(Approval {
            id: Alphanumeric.sample_string(&mut rand::rng(), 12),
            parameters: Vec::new(),
            approval_callback: None,
            needs_permissions: permissions,
            metadata: ApprovalMetadata {
                action: display_description.as_ref().to_string(),
                extra_data: None,
            },
        })
    }

    pub fn extra_data(mut self, data: Value) -> Self {
        self.0.metadata.extra_data = Some(data);

        self
    }

    pub fn param_inline<K: AsRef<str>, V: Display>(mut self, key: K, value: V) -> Self {
        self.0.parameters.push((
            key.as_ref().to_string(),
            ParameterValue::Inline(format!("{}", value)),
        ));

        self
    }

    pub fn param_field<K: AsRef<str>, V: Display>(mut self, key: K, value: V) -> Self {
        self.0.parameters.push((
            key.as_ref().to_string(),
            ParameterValue::Field(format!("{}", value)),
        ));

        self
    }

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

    pub fn build(self) -> Approval {
        self.0
    }
}
