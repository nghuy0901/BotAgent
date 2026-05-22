use std::{collections::HashSet, fmt::Display, hash::Hash};

use serde::Deserialize;

/// A reason for getting rejected when calling [AccessFilter::allows].
pub enum RejectionReason {
    NotWhitelisted,
    Blacklisted,
}

impl Display for RejectionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::NotWhitelisted => "not whitelisted",
                Self::Blacklisted => "blacklisted",
            }
        )
    }
}

/// A wrapper that flattens down into a `whitelist` and a `blacklist` field of type [`HashSet<T>`].
///
/// Used for simplifying whitelisting and blacklisting logic.
#[derive(Deserialize, Default)]
pub struct AccessFilter<T: Eq + Hash> {
    pub whitelist: HashSet<T>,
    pub blacklist: HashSet<T>,
}

impl<T: Eq + Hash> AccessFilter<T> {
    /// Checks whether a certain value is allowed.
    ///
    /// This method checks if the `whitelist` field contains values, and fully ignores the `blacklist` field if it does.
    /// If the `whitelist` field is empty, then this method checks if a function is blacklisted.
    pub fn check(&self, value: &T) -> Result<(), RejectionReason> {
        if !self.whitelist.is_empty() {
            if self.whitelist.contains(value) {
                Ok(())
            } else {
                Err(RejectionReason::NotWhitelisted)
            }
        } else if self.blacklist.contains(value) {
            Err(RejectionReason::Blacklisted)
        } else {
            Ok(())
        }
    }

    /// Checks if this [AccessFilter] has both a populated blacklist and a populated whitelist.
    /// If this method returns true, then it means that the blacklist is fully ignored.
    pub fn ignores_populated_blacklist(&self) -> bool {
        !self.whitelist.is_empty() && !self.blacklist.is_empty()
    }
}
