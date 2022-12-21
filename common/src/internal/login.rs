//! Logging into Linkdoku involves exchanging some JSON objects and then
//! acting on those.
//!

pub mod providers {
    use serde::{Deserialize, Serialize};

    pub const URI: &str = "login/providers";

    #[derive(Serialize, Deserialize)]
    pub struct Provider {
        name: String,
        icon: String,
    }

    pub type Response = Vec<Provider>;
}

pub mod begin {
    use serde::{Deserialize, Serialize};

    pub const URI: &str = "login/begin";

    #[derive(Serialize, Deserialize)]
    pub struct Request {
        provider: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Response {
        redirect_to: String,
    }
}
