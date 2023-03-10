//! Logging out from Linkdoku is pretty easy
//!

use serde::{Deserialize, Serialize};

pub const URI: &str = "/logout";

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub redirect_to: String,
}
