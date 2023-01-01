//! State for the Backend of linkdoku
//!

use axum::extract::FromRef;
use database::Pool;

use crate::{cli::Cli, config::ConfigState, login::Providers};

#[derive(Clone, FromRef)]
pub struct BackendState {
    config: ConfigState,
    pool: Pool,
    providers: Providers,
    cli: Cli,
}

impl BackendState {
    pub fn new(cli: Cli, config: ConfigState, pool: Pool, providers: Providers) -> Self {
        Self {
            config,
            pool,
            providers,
            cli,
        }
    }
}
