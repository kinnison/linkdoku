//! Utility functions
//!
//! You must be logged in to use these

pub mod expand_tinyurl {
    use serde::{Deserialize, Serialize};
    pub const URI: &str = "/util/expand-tinyurl";

    #[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
    pub struct Request {
        pub slug: String,
    }

    #[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
    pub struct Response {
        pub replacement: String,
    }
}
