//! Userinfo is basically information about the currently logged in user.
//!
//! This will always work, but may not contain data

use serde::{Deserialize, Serialize};

pub const URI: &str = "/userinfo";

#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    pub display_name: String,
    pub gravatar_hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub info: Option<UserInfo>,
}
