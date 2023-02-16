//! Role APIs such as updating/creating them

use axum::{routing::post, Json, Router};
use common::{clean_short_name, objects, public, APIError, APIResult, BadShortNameReason};
use database::{
    activity::{self},
    models, Connection,
};
use time::format_description::well_known::Iso8601;
use tracing::info;

use crate::{login::PrivateCookies, state::BackendState};

const RESERVED_ROLE_NAMES: &[&str] = &["puzzle", "role", "settings", "linkdoku"];

async fn update_role(
    mut db: Connection,
    cookies: PrivateCookies,
    Json(req): Json<public::role::update::Request>,
) -> APIResult<public::role::update::Response> {
    let flow = cookies.get_login_flow_status().await;
    let user = if let Some(uuid) = flow.user_uuid() {
        uuid
    } else {
        return Err(APIError::PermissionDenied);
    };

    let mut role = models::Role::by_uuid(&mut db, &req.uuid)
        .await
        .map_err(|e| APIError::DatabaseError(e.to_string()))?
        .ok_or(APIError::ObjectNotFound)?;

    role.short_name = clean_short_name(&req.short_name, false).map_err(APIError::BadShortName)?;
    role.display_name = req.display_name;
    role.description = req.description;

    if RESERVED_ROLE_NAMES.iter().any(|&v| v == role.short_name) {
        return Err(APIError::BadShortName(BadShortNameReason::ReservedWord));
    }

    activity::role::update(&mut db, user, &role)
        .await
        .map_err(|e| e.into())
        .map(|_| objects::Role {
            uuid: role.uuid,
            owner: role.owner,
            short_name: role.short_name,
            display_name: role.display_name,
            description: role.description,
        })
}

async fn role_puzzles(
    mut db: Connection,
    cookies: PrivateCookies,
    Json(req): Json<public::role::puzzles::Request>,
) -> APIResult<public::role::puzzles::Response> {
    let role = models::Role::by_uuid(&mut db, &req.uuid)
        .await
        .map_err(|e| APIError::DatabaseError(e.to_string()))?
        .ok_or(APIError::ObjectNotFound)?;

    info!("Looking up puzzles belonging to {}", role.uuid);

    let logged_in = cookies.get_login_flow_status().await;
    let user = logged_in.user_uuid();

    let puzzles = role
        .visible_puzzles(&mut db, user)
        .await
        .map_err(|e| APIError::DatabaseError(e.to_string()))?;

    info!("Found {} puzzles", puzzles.len());

    let mut ret = vec![];

    for puzzle in puzzles {
        match puzzle
            .can_be_seen(&mut db, user)
            .await
            .map_err(|e| APIError::DatabaseError(e.to_string()))
        {
            Err(e) => return Err(e),
            Ok(true) => ret.push(objects::PuzzleMetadata {
                uuid: puzzle.uuid,
                owner: puzzle.owner,
                display_name: puzzle.display_name,
                short_name: puzzle.short_name,
                visibility: puzzle.visibility.into(),
                updated_at: puzzle
                    .updated_at
                    .format(&Iso8601::DEFAULT)
                    .map_err(|e| APIError::Generic(e.to_string()))?,
            }),
            Ok(false) => {}
        }
    }

    info!("Calling user can see {} of them", ret.len());

    Ok(public::role::puzzles::Response { puzzles: ret })
}

pub fn public_router() -> Router<BackendState> {
    Router::new()
        .route(public::role::update::URI, post(update_role))
        .route(public::role::puzzles::URI, post(role_puzzles))
}
