//! API for basic object fetches etc.

// These are always rooted at PUBLIC_SEGMENT

use axum::{extract::Path, routing::get, Json, Router};
use common::{objects, APIError, APIResult};
use database::{models, Connection};

use crate::{login::PrivateCookies, state::BackendState};

async fn get_role(Path(uuid): Path<String>, mut db: Connection) -> Json<APIResult<objects::Role>> {
    // Role data is always public, so there's no access control to be done here
    let role = match models::Role::by_uuid(&mut db, &uuid)
        .await
        .map_err(|e| APIError::DatabaseError(e.to_string()))
        .transpose()
        .unwrap_or(Err(APIError::ObjectNotFound))
    {
        Ok(role) => role,
        Err(e) => return Json::from(Err(e)),
    };

    Json::from(Ok(objects::Role {
        uuid: role.uuid,
        owner: role.owner,
        display_name: role.display_name,
        description: role.description,
    }))
}

async fn get_puzzle(
    Path(uuid): Path<String>,
    mut db: Connection,
    cookies: PrivateCookies,
) -> Json<APIResult<objects::Puzzle>> {
    let puzzle = match models::Puzzle::by_uuid(&mut db, &uuid)
        .await
        .map_err(|e| APIError::DatabaseError(e.to_string()))
        .transpose()
        .unwrap_or(Err(APIError::ObjectNotFound))
    {
        Ok(puzzle) => puzzle,
        Err(e) => return Json::from(Err(e)),
    };
    let flow = cookies.get_login_flow_status().await;
    let user = flow.user_uuid();

    if !match puzzle
        .can_be_seen(&mut db, user)
        .await
        .map_err(|e| APIError::DatabaseError(e.to_string()))
    {
        Ok(v) => v,
        Err(e) => return Json::from(Err(e)),
    } {
        return Json::from(Err(APIError::ObjectNotFound));
    }

    // Okay it's possible to see the puzzle, let's gather the puzzle states too
    let states = {
        let mut ret = Vec::new();
        for state in match puzzle
            .all_states(&mut db)
            .await
            .map_err(|e| APIError::DatabaseError(e.to_string()))
        {
            Ok(v) => v,
            Err(e) => return Json::from(Err(e)),
        } {
            if match state
                .can_be_seen(&mut db, &puzzle, user)
                .await
                .map_err(|e| APIError::DatabaseError(e.to_string()))
            {
                Ok(v) => v,
                Err(e) => return Json::from(Err(e)),
            } {
                let ostate = objects::PuzzleState {
                    description: state.description,
                    visibility: state.visibility.into(),
                    updated_at: state.updated_at.to_string(),
                    data: match serde_json::from_str(&state.data) {
                        Ok(v) => v,
                        Err(e) => return Json::from(Err(APIError::Generic(e.to_string()))),
                    },
                };
                ret.push(ostate);
            }
        }
        ret
    };

    // We now have a puzzle and its states, we need to turn that into a returnable object.

    let ret = objects::Puzzle {
        uuid: puzzle.uuid.clone(),
        display_name: puzzle.display_name.clone(),
        short_name: puzzle.short_name.clone(),
        owner: puzzle.owner.clone(),
        visibility: puzzle.visibility.into(),
        created_at: puzzle.created_at.to_string(),
        updated_at: puzzle.updated_at.to_string(),
        states,
    };

    Ok(ret).into()
}

pub fn public_router() -> Router<BackendState> {
    Router::new()
        .route("/role/:uuid", get(get_role))
        .route("/puzzle/:uuid", get(get_puzzle))
}
