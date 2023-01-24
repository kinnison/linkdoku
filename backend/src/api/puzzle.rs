//! Puzzle APIs

use axum::{routing::post, Json, Router};
use common::{public::puzzle, APIError, APIResult};
use database::{activity, Connection};
use tracing::info;

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

async fn lookup_puzzle(
    mut db: Connection,
    cookies: PrivateCookies,
    Json(req): Json<puzzle::lookup::Request>,
) -> Json<APIResult<puzzle::lookup::Response>> {
    let logged_in = cookies.get_login_flow_status().await;
    let user = logged_in.user_uuid();

    Json::from(
        activity::puzzle::lookup(&mut db, &req.role, &req.puzzle, user)
            .await
            .map(|s| puzzle::lookup::Response { uuid: s })
            .map_err(|e| e.into()),
    )
}

async fn update_puzzle_metadata(
    mut db: Connection,
    cookies: PrivateCookies,
    Json(req): Json<puzzle::update_metadata::Request>,
) -> Json<APIResult<puzzle::update_metadata::Response>> {
    let logged_in = cookies.get_login_flow_status().await;
    let logged_in = match logged_in.user() {
        Some(data) => data,
        None => {
            return Json::from(Err(APIError::PermissionDenied));
        }
    };

    let puzzle = match activity::puzzle::update_metadata(
        &mut db,
        &logged_in.identity().uuid,
        &req.puzzle,
        &req.short_name,
        &req.display_name,
    )
    .await
    {
        Ok(puzzle) => puzzle,
        Err(e) => return Json::from(Err(e.into())),
    };

    Json::from(
        activity::puzzle::into_api_object(&mut db, Some(&logged_in.identity().uuid), puzzle)
            .await
            .map_err(|e| e.into()),
    )
}

async fn update_puzzle_state(
    mut db: Connection,
    cookies: PrivateCookies,
    Json(req): Json<puzzle::update_state::Request>,
) -> Json<APIResult<puzzle::update_state::Response>> {
    let logged_in = cookies.get_login_flow_status().await;
    let logged_in = match logged_in.user() {
        Some(data) => data,
        None => {
            return Json::from(Err(APIError::PermissionDenied));
        }
    };

    let puzzle = match activity::puzzle::update_state(
        &mut db,
        &logged_in.identity().uuid,
        &req.puzzle,
        &req.state,
    )
    .await
    {
        Ok(puzzle) => puzzle,
        Err(e) => return Json::from(Err(e.into())),
    };

    Json::from(
        activity::puzzle::into_api_object(&mut db, Some(&logged_in.identity().uuid), puzzle)
            .await
            .map_err(|e| e.into()),
    )
}

async fn add_puzzle_state(
    mut db: Connection,
    cookies: PrivateCookies,
    Json(req): Json<puzzle::add_state::Request>,
) -> Json<APIResult<puzzle::add_state::Response>> {
    let logged_in = cookies.get_login_flow_status().await;
    let logged_in = match logged_in.user() {
        Some(data) => data,
        None => {
            return Json::from(Err(APIError::PermissionDenied));
        }
    };

    let puzzle = match activity::puzzle::add_state(
        &mut db,
        &logged_in.identity().uuid,
        &req.puzzle,
        &req.state,
    )
    .await
    {
        Ok(puzzle) => puzzle,
        Err(e) => return Json::from(Err(e.into())),
    };

    Json::from(
        activity::puzzle::into_api_object(&mut db, Some(&logged_in.identity().uuid), puzzle)
            .await
            .map_err(|e| e.into()),
    )
}

async fn set_puzzle_visibility(
    mut db: Connection,
    cookies: PrivateCookies,
    Json(req): Json<puzzle::set_visibility::Request>,
) -> Json<APIResult<puzzle::set_visibility::Response>> {
    let logged_in = cookies.get_login_flow_status().await;
    let logged_in = match logged_in.user() {
        Some(data) => data,
        None => {
            return Json::from(Err(APIError::PermissionDenied));
        }
    };

    let puzzle = match activity::puzzle::set_visibility(
        &mut db,
        &logged_in.identity().uuid,
        &req.puzzle,
        req.visibility,
    )
    .await
    {
        Ok(puzzle) => puzzle,
        Err(e) => return Json::from(Err(e.into())),
    };

    Json::from(
        activity::puzzle::into_api_object(&mut db, Some(&logged_in.identity().uuid), puzzle)
            .await
            .map_err(|e| e.into()),
    )
}

async fn set_puzzle_state_visibility(
    mut db: Connection,
    cookies: PrivateCookies,
    Json(req): Json<puzzle::set_state_visibility::Request>,
) -> Json<APIResult<puzzle::set_state_visibility::Response>> {
    let logged_in = cookies.get_login_flow_status().await;
    let logged_in = match logged_in.user() {
        Some(data) => data,
        None => {
            return Json::from(Err(APIError::PermissionDenied));
        }
    };

    let puzzle = match activity::puzzle::set_state_visibility(
        &mut db,
        &logged_in.identity().uuid,
        &req.puzzle,
        &req.state,
        req.visibility,
    )
    .await
    {
        Ok(puzzle) => puzzle,
        Err(e) => return Json::from(Err(e.into())),
    };

    Json::from(
        activity::puzzle::into_api_object(&mut db, Some(&logged_in.identity().uuid), puzzle)
            .await
            .map_err(|e| e.into()),
    )
}

async fn edit_puzzle_tags(
    mut db: Connection,
    cookies: PrivateCookies,
    Json(req): Json<puzzle::edit_tags::Request>,
) -> Json<APIResult<puzzle::edit_tags::Response>> {
    let logged_in = cookies.get_login_flow_status().await;
    let logged_in = match logged_in.user() {
        Some(data) => data,
        None => {
            return Json::from(Err(APIError::PermissionDenied));
        }
    };

    info!(
        "Edit puzzle {}, add tags {:?}, remove tags {:?}",
        req.puzzle, req.to_add, req.to_remove
    );

    let puzzle = match activity::puzzle::edit_puzzle_tags(
        &mut db,
        &logged_in.identity().uuid,
        &req.puzzle,
        &req.to_add,
        &req.to_remove,
    )
    .await
    {
        Ok(puzzle) => puzzle,
        Err(e) => return Json::from(Err(e.into())),
    };

    Json::from(
        activity::puzzle::into_api_object(&mut db, Some(&logged_in.identity().uuid), puzzle)
            .await
            .map_err(|e| e.into()),
    )
}

pub fn public_router() -> Router<BackendState> {
    Router::new()
        .route(puzzle::create::URI, post(create_puzzle))
        .route(puzzle::lookup::URI, post(lookup_puzzle))
        .route(puzzle::update_metadata::URI, post(update_puzzle_metadata))
        .route(puzzle::update_state::URI, post(update_puzzle_state))
        .route(puzzle::add_state::URI, post(add_puzzle_state))
        .route(puzzle::set_visibility::URI, post(set_puzzle_visibility))
        .route(
            puzzle::set_state_visibility::URI,
            post(set_puzzle_state_visibility),
        )
        .route(puzzle::edit_tags::URI, post(edit_puzzle_tags))
}
