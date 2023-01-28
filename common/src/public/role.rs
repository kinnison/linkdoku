//! Role APIs such as creating/updating roles.
//!
//! You can retrieve role information via the objects API though

pub mod update {
    use crate::objects;
    use serde::{Deserialize, Serialize};

    pub const URI: &str = "/role/update";

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Request {
        pub uuid: String,
        pub short_name: String,
        pub display_name: String,
        pub description: String,
    }

    pub type Response = objects::Role;
}

pub mod puzzles {
    use serde::{Deserialize, Serialize};

    pub const URI: &str = "/role/puzzles";

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Request {
        pub uuid: String,
    }

    pub type Response = Vec<String>;
}
