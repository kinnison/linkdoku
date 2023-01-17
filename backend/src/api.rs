//! API for Linkdoku
//!

use axum::{routing::get, Json, Router};
use common::{
    internal::INTERNAL_SEGMENT,
    public::{
        scaffold::{self, hash_version_info},
        PUBLIC_SEGMENT,
    },
    APIResult,
};
use git_testament::git_testament;

use crate::state::BackendState;

mod objects;
mod puzzle;
mod role;

git_testament!(VERSION);

async fn get_scaffold() -> Json<APIResult<scaffold::Response>> {
    Json::from(Ok(scaffold::Response {
        version: format!("{VERSION}"),
        version_hash: hash_version_info(&VERSION),
    }))
}

fn public_router() -> Router<BackendState> {
    Router::new().route(scaffold::URI, get(get_scaffold))
}

pub fn router() -> Router<BackendState> {
    let internal = Router::new().merge(crate::login::internal_router());
    let public = Router::new()
        .merge(public_router())
        .merge(crate::login::public_router())
        .merge(objects::public_router())
        .merge(role::public_router())
        .merge(puzzle::public_router());

    Router::new()
        .nest(INTERNAL_SEGMENT, internal)
        .nest(PUBLIC_SEGMENT, public)
}
