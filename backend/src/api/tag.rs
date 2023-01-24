//! Database APIs for tags
//!

use axum::{routing::post, Json, Router};
use common::{public, APIResult};
use database::{activity, Connection};

use crate::state::BackendState;

async fn list_tags(
    mut db: Connection,
    Json(req): Json<public::tag::list::Request>,
) -> Json<APIResult<public::tag::list::Response>> {
    let tags = match activity::tag::list(&mut db, &req.pattern).await {
        Ok(tags) => tags,
        Err(e) => return Json::from(Err(e.into())),
    };

    Json::from(Ok(tags))
}

pub fn public_router() -> Router<BackendState> {
    Router::new().route(public::tag::list::URI, post(list_tags))
}
