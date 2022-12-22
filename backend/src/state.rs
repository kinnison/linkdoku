//! State for the Backend of linkdoku
//!

use axum::extract::FromRef;
use database::Pool;

use crate::{config::ConfigState, login::Providers};

#[derive(Clone, FromRef)]
pub struct BackendState {
    config: ConfigState,
    pool: Pool,
    providers: Providers,
}

impl BackendState {
    pub fn new(config: ConfigState, pool: Pool, providers: Providers) -> Self {
        Self {
            config,
            pool,
            providers,
        }
    }
}
