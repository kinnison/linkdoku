//! Puzzle related APIs

pub mod create {
    use serde::{Deserialize, Serialize};

    use crate::objects;

    pub const URI: &str = "/puzzle/create";

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct Request {
        pub owner: String,
        pub display_name: String,
        pub short_name: String,
        pub initial_state: objects::PuzzleState,
    }

    pub type Response = objects::Puzzle;
}

pub mod lookup {
    use serde::{Deserialize, Serialize};

    pub const URI: &str = "/puzzle/lookup";

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Request {
        pub role: String,
        pub puzzle: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Response {
        pub uuid: String,
    }
}
