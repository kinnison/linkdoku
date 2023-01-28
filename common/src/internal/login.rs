//! Logging into Linkdoku involves exchanging some JSON objects and then
//! acting on those.
//!

pub mod providers {
    use serde::{Deserialize, Serialize};

    pub const URI: &str = "/login/providers";

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Provider {
        pub name: String,
        pub icon: String,
    }

    pub type Response = Vec<Provider>;
}

pub mod begin {
    use serde::{Deserialize, Serialize};

    pub const URI: &str = "/login/begin";

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Request {
        pub provider: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum Response {
        LoggedIn,
        Continue(String),
    }
}

pub mod complete {
    use serde::{Deserialize, Serialize};

    use crate::public::userinfo::UserInfo;

    pub const URI: &str = "/login/complete";

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Request {
        pub state: String,
        pub code: Option<String>,
        pub error: Option<String>,
    }

    // Either we succeed, in which case the cookie is the side-effect, or we return an error
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Response {
        pub userinfo: UserInfo,
        pub is_first_login: bool,
    }
}
