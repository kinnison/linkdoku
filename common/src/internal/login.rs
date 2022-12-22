//! Logging into Linkdoku involves exchanging some JSON objects and then
//! acting on those.
//!

pub mod providers {
    use serde::{Deserialize, Serialize};

    pub const URI: &str = "/login/providers";

    #[derive(Serialize, Deserialize)]
    pub struct Provider {
        pub name: String,
        pub icon: String,
    }

    pub type Response = Vec<Provider>;
}

pub mod begin {
    use serde::{Deserialize, Serialize};

    pub const URI: &str = "/login/begin";

    #[derive(Serialize, Deserialize)]
    pub struct Request {
        pub provider: String,
    }

    #[derive(Serialize, Deserialize)]
    pub enum Response {
        LoggedIn,
        Continue(String),
    }
}

pub mod complete {
    use serde::{Deserialize, Serialize};

    pub const URI: &str = "/login/complete";

    #[derive(Serialize, Deserialize)]
    pub struct Request {
        pub state: Option<String>,
        pub code: Option<String>,
        pub error: Option<String>,
    }

    // Either we succeed, in which case the cookie is the side-effect, or we return an error
    pub type Response = ();
}
