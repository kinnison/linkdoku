//! API for basic object fetches etc.

// These are always rooted at PUBLIC_SEGMENT

use axum::{extract::Path, routing::get, Json, Router};
use common::{objects, APIError, APIResult};
use database::{models, Connection};

use crate::state::BackendState;

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

pub fn public_router() -> Router<BackendState> {
    Router::new().route("/role/:uuid", get(get_role))
}
