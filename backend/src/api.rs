//! API for Linkdoku
//!

use axum::Router;
use common::{internal::INTERNAL_SEGMENT, public::PUBLIC_SEGMENT};

use crate::state::BackendState;

mod objects;
mod role;

pub fn router() -> Router<BackendState> {
    let internal = Router::new().merge(crate::login::internal_router());
    let public = Router::new()
        .merge(crate::login::public_router())
        .merge(objects::public_router())
        .merge(role::public_router());

    Router::new()
        .nest(INTERNAL_SEGMENT, internal)
        .nest(PUBLIC_SEGMENT, public)
}
