//! API for Linkdoku
//!

use axum::{
    routing::{get, post},
    Json, Router,
};
use common::{
    internal::{self, INTERNAL_SEGMENT},
    public::{
        scaffold::{self, hash_version_info},
        PUBLIC_SEGMENT,
    },
    APIError, APIResult,
};
use git_testament::git_testament;
use puzzleutils::fpuzzles;
use reqwest::{header::LOCATION, redirect::Policy, Client};

use crate::{login::PrivateCookies, state::BackendState};

mod objects;
mod puzzle;
mod role;
mod tag;

git_testament!(VERSION);

async fn get_scaffold() -> APIResult<scaffold::Response> {
    Ok(scaffold::Response {
        version: format!("{VERSION}"),
        version_hash: hash_version_info(&VERSION),
    })
}

async fn try_expand_tinyurl(
    cookies: PrivateCookies,
    Json(req): Json<internal::util::expand_tinyurl::Request>,
) -> APIResult<internal::util::expand_tinyurl::Response> {
    let login_state = cookies.get_login_flow_status().await;
    if login_state.user().is_none() {
        return Err(APIError::PermissionDenied);
    }

    let client = Client::builder()
        .redirect(Policy::none())
        .build()
        .map_err(|e| APIError::Generic(format!("Unable to build HTTP client: {e}")))?;

    let url = format!("https://tinyurl.com/{}", req.slug);

    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| APIError::Generic(format!("Unable to run HTTP get: {e}")))?;

    let url = response
        .headers()
        .get(LOCATION)
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| APIError::Generic("Did not get a redirect?".into()))?;

    let fpuzz = fpuzzles::extract(url)
        .ok_or_else(|| APIError::Generic("Unable to extract fpuzzles data".into()))?;

    Ok(internal::util::expand_tinyurl::Response {
        replacement: fpuzzles::encode(&fpuzz),
    })
}

fn public_router() -> Router<BackendState> {
    Router::new().route(scaffold::URI, get(get_scaffold))
}

fn internal_router() -> Router<BackendState> {
    Router::new().route(
        internal::util::expand_tinyurl::URI,
        post(try_expand_tinyurl),
    )
}

pub fn router() -> Router<BackendState> {
    let internal = Router::new()
        .merge(internal_router())
        .merge(crate::login::internal_router());
    let public = Router::new()
        .merge(public_router())
        .merge(crate::login::public_router())
        .merge(objects::public_router())
        .merge(role::public_router())
        .merge(puzzle::public_router())
        .merge(tag::public_router());

    Router::new()
        .nest(INTERNAL_SEGMENT, internal)
        .nest(PUBLIC_SEGMENT, public)
}
