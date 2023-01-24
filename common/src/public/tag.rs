//! APIs for tags

pub mod list {
    use serde::{Deserialize, Serialize};

    pub const URI: &str = "/tag/list";

    #[derive(Serialize, Deserialize)]
    pub struct Request {
        pub prefix: Option<String>,
    }

    pub type Response = Vec<crate::objects::Tag>;
}
