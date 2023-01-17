//! Role APIs such as updating/creating them

use axum::{routing::post, Json, Router};
use common::{clean_short_name, public, APIError, APIResult, BadShortNameReason};
use database::{
    activity::{self},
    models, Connection,
};
use tracing::info;

use crate::{login::PrivateCookies, state::BackendState};

const RESERVED_ROLE_NAMES: &[&str] = &["puzzle", "role", "settings", "linkdoku"];

async fn update_role(
    mut db: Connection,
    cookies: PrivateCookies,
    Json(req): Json<public::role::update::Request>,
) -> Json<APIResult<public::role::update::Response>> {
    let flow = cookies.get_login_flow_status().await;
    let user = if let Some(uuid) = flow.user_uuid() {
        uuid
    } else {
        return Json::from(Err(APIError::PermissionDenied));
    };

    let mut role = match models::Role::by_uuid(&mut db, &req.uuid).await {
        Ok(Some(r)) => r,
        Ok(None) => return Json::from(Err(APIError::ObjectNotFound)),
        Err(e) => return Json::from(Err(APIError::DatabaseError(e.to_string()))),
    };

    role.short_name = match clean_short_name(&req.short_name, false) {
        Ok(short_name) => short_name,
        Err(reason) => return Json::from(Err(APIError::BadShortName(reason))),
    };
    role.display_name = req.display_name;
    role.description = req.description;

    if RESERVED_ROLE_NAMES.iter().any(|&v| v == role.short_name) {
        return Json::from(Err(APIError::BadShortName(
            BadShortNameReason::ReservedWord,
        )));
    }

    activity::role::update(&mut db, user, &role)
        .await
        .map_err(|e| e.into())
        .into()
}

async fn role_puzzles(
    mut db: Connection,
    cookies: PrivateCookies,
    Json(req): Json<public::role::puzzles::Request>,
) -> Json<APIResult<public::role::puzzles::Response>> {
    let role = match models::Role::by_uuid(&mut db, &req.uuid).await {
        Ok(Some(r)) => r,
        Ok(None) => return Json::from(Err(APIError::ObjectNotFound)),
        Err(e) => return Json::from(Err(APIError::DatabaseError(e.to_string()))),
    };

    info!("Looking up puzzles belonging to {}", role.uuid);

    let logged_in = cookies.get_login_flow_status().await;
    let user = logged_in.user_uuid();

    let puzzles = match role
        .visible_puzzles(&mut db, user)
        .await
        .map_err(|e| APIError::DatabaseError(e.to_string()))
    {
        Err(e) => return Json::from(Err(e)),
        Ok(puzzles) => puzzles,
    };

    info!("Found {} puzzles", puzzles.len());

    let mut ret = vec![];

    for puzzle in puzzles {
        match puzzle
            .can_be_seen(&mut db, user)
            .await
            .map_err(|e| APIError::DatabaseError(e.to_string()))
        {
            Err(e) => return Json::from(Err(e)),
            Ok(true) => ret.push(puzzle.uuid),
            Ok(false) => {}
        }
    }

    info!("Calling user can see {} of them", ret.len());
    Ok(ret).into()
}

pub fn public_router() -> Router<BackendState> {
    Router::new()
        .route(public::role::update::URI, post(update_role))
        .route(public::role::puzzles::URI, post(role_puzzles))
}
