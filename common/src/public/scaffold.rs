//! Scaffold is core information about the backend, for example
//! the version of it.
//!
//! This will always work

use git_testament::{GitModification, GitTestament};
use serde::{Deserialize, Serialize};

pub const URI: &str = "/scaffold";

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Response {
    pub version: String,
    pub version_hash: String,
}

/// Construct a version hash from a Git testament

pub fn hash_version_info(version: &GitTestament) -> String {
    let mut hasher = md5::Context::new();

    hasher.consume(version.branch_name.unwrap_or("\0NONE"));
    hasher.consume("\0SEP");
    hasher.consume(version.commit.to_string());
    hasher.consume("\0SEP");
    for m in version.modifications {
        match *m {
            GitModification::Added(f) => {
                hasher.consume("\0ADDED");
                hasher.consume(f);
            }
            GitModification::Removed(f) => {
                hasher.consume("\0REMOVED");
                hasher.consume(f);
            }
            GitModification::Modified(f) => {
                hasher.consume("\0MODIFIED");
                hasher.consume(f);
            }
            GitModification::Untracked(f) => {
                hasher.consume("\0UNTRACKED");
                hasher.consume(f);
            }
        }
    }

    format!("{:x}", hasher.compute())
}
