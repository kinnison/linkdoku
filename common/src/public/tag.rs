//! APIs for tags

pub mod list {
    use serde::{Deserialize, Serialize};

    use crate::objects::Tag;

    pub const URI: &str = "/tag/list";

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Request {
        pub pattern: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Response {
        pub tags: Vec<Tag>,
    }
}
