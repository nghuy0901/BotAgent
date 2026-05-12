use std::sync::Arc;

use dashmap::DashMap;
use parking_lot::Mutex;

use crate::approval::{Approval, NeededPermission};

type ApprovalArc = Arc<Mutex<Option<Approval>>>;

#[derive(Default)]
pub struct ApprovalManager {
    pending_approvals: DashMap<String, ApprovalArc>,
}

impl ApprovalManager {
    pub fn register(&self, approval: Approval) -> ApprovalArc {
        let id = approval.id.clone();
        let approval = Arc::new(Mutex::new(Some(approval)));

        self.pending_approvals.insert(id, approval.clone());

        approval
    }

    pub fn get_needed_permission<T: AsRef<str>>(&self, id: T) -> Option<NeededPermission> {
        self.pending_approvals
            .get(id.as_ref())?
            .lock()
            .as_ref()
            .map(|a| a.needs_permissions.clone())
    }

    pub fn take<T: AsRef<str>>(&self, id: T) -> Option<Approval> {
        let taken = self
            .pending_approvals
            .get(id.as_ref())
            .and_then(|a| a.lock().take());

        self.pending_approvals.remove(id.as_ref());

        taken
    }
}
