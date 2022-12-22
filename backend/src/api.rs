//! API for Linkdoku
//!

use axum::Router;

use crate::state::BackendState;

pub fn router() -> Router<BackendState> {
    let internal = Router::new().merge(crate::login::internal_router());
    let public = Router::new().merge(crate::login::public_router());

    Router::new()
        .nest("/internal", internal)
        .nest("/public", public)
}
