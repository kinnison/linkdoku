//! Puzzle APIs

use axum::{routing::post, Json, Router};
use common::{public::puzzle, APIError, APIResult};
use database::{activity, Connection};

use crate::{login::PrivateCookies, state::BackendState};

async fn create_puzzle(
    mut db: Connection,
    cookies: PrivateCookies,
    Json(req): Json<puzzle::create::Request>,
) -> Json<APIResult<puzzle::create::Response>> {
    let logged_in = cookies.get_login_flow_status().await;
    let logged_in = match logged_in.user() {
        Some(data) => data,
        None => {
            return Json::from(Err(APIError::PermissionDenied));
        }
    };

    // Let's do some basic sanity checking, e.g. short-name and display-name must not be empty
    if req.short_name.is_empty() || req.display_name.is_empty() {
        return Json::from(Err(APIError::BadInput));
    }

    // Let's run the create puzzle activity.  This will ignore the visibility and updated_at of
    // the initial data because we're creating it now, but the description and the data will be
    // honoured.

    let puzz = match activity::puzzle::create(
        &mut db,
        &logged_in.identity().uuid,
        &req.owner,
        &req.short_name,
        &req.display_name,
        &req.initial_state,
    )
    .await
    {
        Ok(puzz) => puzz,
        Err(e) => return Json::from(Err(e.into())),
    };

    Json::from(
        activity::puzzle::into_api_object(&mut db, Some(&logged_in.identity().uuid), puzz)
            .await
            .map_err(|e| e.into()),
    )
}

pub fn public_router() -> Router<BackendState> {
    Router::new().route(puzzle::create::URI, post(create_puzzle))
}