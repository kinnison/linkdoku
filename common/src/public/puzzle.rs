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

pub mod update_metadata {
    use crate::objects;
    use serde::{Deserialize, Serialize};

    pub const URI: &str = "/puzzle/update-metadata";

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Request {
        pub puzzle: String,
        pub short_name: String,
        pub display_name: String,
    }

    pub type Response = objects::Puzzle;
}

pub mod update_state {
    use crate::objects;
    use serde::{Deserialize, Serialize};

    pub const URI: &str = "/puzzle/update-state";

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Request {
        pub puzzle: String,
        pub state: objects::PuzzleState,
    }

    pub type Response = objects::Puzzle;
}

pub mod add_state {
    use crate::objects;
    use serde::{Deserialize, Serialize};

    pub const URI: &str = "/puzzle/add-state";

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Request {
        pub puzzle: String,
        pub state: objects::PuzzleState,
    }

    pub type Response = objects::Puzzle;
}

pub mod set_visibility {
    use crate::objects;

    use serde::{Deserialize, Serialize};

    pub const URI: &str = "/puzzle/set-visibility";

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Request {
        pub puzzle: String,
        pub visibility: objects::Visibility,
        pub in_view_state: Option<String>,
    }

    pub type Response = objects::Puzzle;
}

pub mod set_state_visibility {
    use crate::objects;

    use serde::{Deserialize, Serialize};

    pub const URI: &str = "/puzzle/set-state-visibility";

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Request {
        pub puzzle: String,
        pub state: String,
        pub visibility: objects::Visibility,
    }

    pub type Response = objects::Puzzle;
}

pub mod edit_tags {
    use crate::objects;

    use serde::{Deserialize, Serialize};

    pub const URI: &str = "/puzzle/edit-tags";

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Request {
        pub puzzle: String,
        pub to_add: Vec<String>,
        pub to_remove: Vec<String>,
    }

    pub type Response = objects::Puzzle;
}
