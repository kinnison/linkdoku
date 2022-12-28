//! Role APIs such as creating/updating roles.
//!
//! You can retrieve role information via the objects API though

pub mod update {
    use serde::{Deserialize, Serialize};

    pub const URI: &str = "/role/update";

    #[derive(Serialize, Deserialize)]
    pub struct Request {
        pub uuid: String,
        pub display_name: String,
        pub description: String,
    }

    pub type Response = ();
}
