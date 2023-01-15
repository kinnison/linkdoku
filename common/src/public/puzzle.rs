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
