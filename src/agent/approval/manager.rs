use std::collections::HashMap;

use tokio::sync::RwLock;

use crate::agent::approval::Approval;

#[derive(Default)]
pub struct ApprovalManager {
    pub pending_approvals: RwLock<HashMap<String, Approval>>,
}

impl ApprovalManager {
    pub async fn register(&self, approval: Approval) {
        let mut pending = self.pending_approvals.write().await;

        pending.insert(approval.id.clone(), approval);
    }

    pub async fn take<T: AsRef<str>>(&self, id: T) -> Option<Approval> {
        let mut pending = self.pending_approvals.write().await;

        pending.remove(id.as_ref())
    }
}
