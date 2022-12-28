//! Common objects transacted by the various APIs

use serde::{Deserialize, Serialize};

/// Roles are owners of puzzles etc.
///
/// Roles have owners themselves (identities) and names, descriptions, etc.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Role {
    pub uuid: String,
    pub owner: String,
    pub display_name: String,
    pub description: String,
}

impl Role {
    pub fn can_edit(&self, identity: &str) -> bool {
        self.owner == identity
    }

    pub fn can_add_puzzles(&self, identity: &str) -> bool {
        self.owner == identity
    }
}
