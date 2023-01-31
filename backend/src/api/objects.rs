//! API for basic object fetches etc.

// These are always rooted at PUBLIC_SEGMENT

use axum::{extract::Path, routing::get, Router};
use common::{objects, APIError, APIResult};
use database::{activity, models, Connection};
use time::format_description::well_known::Iso8601;

use crate::{login::PrivateCookies, state::BackendState};

async fn get_role_by_uuid(Path(uuid): Path<String>, db: Connection) -> APIResult<objects::Role> {
    get_role_(&uuid, db, false).await
}

async fn get_role_by_name(Path(uuid): Path<String>, db: Connection) -> APIResult<objects::Role> {
    get_role_(&uuid, db, true).await
}

async fn get_role_(item: &str, mut db: Connection, is_name: bool) -> APIResult<objects::Role> {
    // Role data is always public, so there's no access control to be done here
    let res = if is_name {
        models::Role::by_short_name(&mut db, item).await
    } else {
        models::Role::by_uuid(&mut db, item).await
    };

    let role = res
        .map_err(|e| APIError::DatabaseError(e.to_string()))
        .transpose()
        .unwrap_or(Err(APIError::ObjectNotFound))?;

    Ok(objects::Role {
        uuid: role.uuid,
        owner: role.owner,
        short_name: role.short_name,
        display_name: role.display_name,
        description: role.description,
    })
}

async fn get_puzzle(
    Path(uuid): Path<String>,
    mut db: Connection,
    cookies: PrivateCookies,
) -> APIResult<objects::Puzzle> {
    let puzzle = models::Puzzle::by_uuid(&mut db, &uuid)
        .await
        .map_err(|e| APIError::DatabaseError(e.to_string()))
        .transpose()
        .unwrap_or(Err(APIError::ObjectNotFound))?;
    let flow = cookies.get_login_flow_status().await;
    let user = flow.user_uuid();

    if !puzzle
        .can_be_seen(&mut db, user)
        .await
        .map_err(|e| APIError::DatabaseError(e.to_string()))?
    {
        return Err(APIError::ObjectNotFound);
    }

    activity::puzzle::into_api_object(&mut db, user, puzzle)
        .await
        .map_err(|e| e.into())
}

async fn get_tag(Path(uuid): Path<String>, mut db: Connection) -> APIResult<objects::Tag> {
    let tag = models::Tag::by_uuid(&mut db, &uuid)
        .await
        .map_err(|e| APIError::DatabaseError(e.to_string()))
        .transpose()
        .unwrap_or(Err(APIError::ObjectNotFound))?;

    Ok(objects::Tag {
        uuid: tag.uuid,
        name: tag.name,
        colour: tag.colour,
        black_text: tag.black_text,
        description: tag.description,
    })
}

async fn get_puzzle_metadata(
    Path(uuid): Path<String>,
    mut db: Connection,
    cookies: PrivateCookies,
) -> APIResult<objects::PuzzleMetadata> {
    let flow = cookies.get_login_flow_status().await;
    let user = flow.user_uuid();

    let puzzle = models::Puzzle::by_uuid(&mut db, &uuid)
        .await
        .map_err(|e| APIError::DatabaseError(e.to_string()))
        .transpose()
        .unwrap_or(Err(APIError::ObjectNotFound))?;
    if !puzzle
        .can_be_seen(&mut db, user)
        .await
        .map_err(|e| APIError::DatabaseError(e.to_string()))?
    {
        return Err(APIError::ObjectNotFound);
    }

    Ok(objects::PuzzleMetadata {
        uuid: puzzle.uuid,
        display_name: puzzle.display_name,
        short_name: puzzle.short_name,
        visibility: puzzle.visibility.into(),
        updated_at: puzzle
            .updated_at
            .format(&Iso8601::DEFAULT)
            .map_err(|e| APIError::Generic(e.to_string()))?,
    })
}

pub fn public_router() -> Router<BackendState> {
    Router::new()
        .route("/role/by-uuid/:uuid", get(get_role_by_uuid))
        .route("/role/by-name/:uuid", get(get_role_by_name))
        .route("/puzzle/by-uuid/:uuid", get(get_puzzle))
        .route("/tag/by-uuid/:uuid", get(get_tag))
        .route("/puzzle-metadata/by-uuid/:uuid)", get(get_puzzle_metadata))
}
