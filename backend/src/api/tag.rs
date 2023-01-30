//! Database APIs for tags
//!

use axum::{routing::post, Json, Router};
use common::{public, APIResult};
use database::{activity, Connection};

use crate::state::BackendState;

async fn list_tags(
    mut db: Connection,
    Json(req): Json<public::tag::list::Request>,
) -> APIResult<public::tag::list::Response> {
    let tags = activity::tag::list(&mut db, &req.pattern).await?;

    Ok(public::tag::list::Response { tags })
}

pub fn public_router() -> Router<BackendState> {
    Router::new().route(public::tag::list::URI, post(list_tags))
}
