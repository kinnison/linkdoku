//! This is the structural API object which is
//! acquired when you use_apiprovider()

use std::{hash::Hash, sync::Arc};

use common::{
    internal::{login, logout, INTERNAL_SEGMENT},
    public::{self, userinfo, PUBLIC_SEGMENT},
    APIError, APIResult,
};
use reqwest::{header::COOKIE, Client, StatusCode, Url};
use serde::{de::DeserializeOwned, Serialize};
use yew::prelude::*;

use crate::backend::APIContents;

use frontend_core::LinkdokuBase;

#[derive(Clone)]
pub struct LinkdokuAPI {
    client: Arc<Client>,
    base: AttrValue,
    login: Option<AttrValue>,
}

impl PartialEq for LinkdokuAPI {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.client, &other.client)
            && self.base == other.base
            && self.login == other.login
    }
}

impl Eq for LinkdokuAPI {}

impl Hash for LinkdokuAPI {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // self.client.hash(state); // The actual client doesn't matter
        self.base.hash(state);
        self.login.hash(state);
    }
}

#[hook]
pub fn use_apiprovider() -> LinkdokuAPI {
    let base = use_context::<LinkdokuBase>()
        .expect("Invoked use_apiprovider() when not within a BaseURIProvider");
    let client = use_context::<APIContents>()
        .expect("Invoked use_apiprovider() when not within a ClientProvider");

    LinkdokuAPI {
        client: client.client,
        base: (*base.uri).clone(),
        login: base.login.as_ref().map(|v| (**v).clone()),
    }
}

const NO_BODY: Option<()> = None;
const EMPTY_BODY: Option<Vec<String>> = Some(Vec::new());

impl LinkdokuAPI {
    fn compute_uri(&self, base: &str, func: &str) -> Url {
        let combined = format!("{}api{}{}", self.base.as_str(), base, func);
        Url::parse(&combined).expect("Unable to construct API URL?")
    }

    fn compute_basic_uri(&self, kind: &str, uuid: &str) -> Url {
        let combined = format!(
            "{}api{}/{}/{}",
            self.base.as_str(),
            PUBLIC_SEGMENT,
            kind,
            uuid
        );
        Url::parse(&combined).expect("Unable to construct API url?")
    }

    async fn make_api_call<IN, OUT>(
        &self,
        mut api: Url,
        query_params: impl IntoIterator<Item = (&str, &str)>,
        body: Option<IN>,
    ) -> APIResult<OUT>
    where
        IN: Serialize,
        OUT: DeserializeOwned,
    {
        api.query_pairs_mut()
            .clear()
            .extend_pairs(query_params)
            .finish();
        let requestbuilder = if let Some(body) = body {
            self.client.post(api).json(&body)
        } else {
            self.client.get(api)
        };

        let request = if let Some(login) = &self.login {
            requestbuilder.header(COOKIE, format!("login={}", login))
        } else {
            requestbuilder
        }
        .build()
        .map_err(|e| APIError::ClientIssue(e.to_string()))?;

        let response = self
            .client
            .execute(request)
            .await
            .map_err(|e| APIError::ClientIssue(e.to_string()))?;
        if response.status() == StatusCode::NOT_FOUND {
            return Err(APIError::ObjectNotFound);
        }
        response
            .error_for_status()
            .map_err(|e| APIError::ClientIssue(e.to_string()))?
            .json()
            .await
            .map_err(|e| APIError::ClientIssue(e.to_string()))?
    }

    pub async fn login_providers(&self) -> APIResult<login::providers::Response> {
        let uri = self.compute_uri(INTERNAL_SEGMENT, login::providers::URI);

        self.make_api_call(uri, None, NO_BODY).await
    }

    pub async fn start_login(&self, provider: &str) -> APIResult<login::begin::Response> {
        let body = login::begin::Request {
            provider: provider.into(),
        };
        let uri = self.compute_uri(INTERNAL_SEGMENT, login::begin::URI);
        self.make_api_call(uri, None, Some(body)).await
    }

    pub async fn complete_login(
        &self,
        state: &str,
        code: Option<&str>,
        error: Option<&str>,
    ) -> APIResult<login::complete::Response> {
        let body = login::complete::Request {
            state: state.to_string(),
            code: code.map(String::from),
            error: error.map(String::from),
        };
        let uri = self.compute_uri(INTERNAL_SEGMENT, login::complete::URI);
        self.make_api_call(uri, None, Some(body)).await
    }

    pub async fn get_userinfo(&self) -> APIResult<userinfo::Response> {
        let uri = self.compute_uri(PUBLIC_SEGMENT, userinfo::URI);
        self.make_api_call(uri, None, NO_BODY).await
    }

    pub async fn logout(&self) -> APIResult<logout::Response> {
        let uri = self.compute_uri(INTERNAL_SEGMENT, logout::URI);
        self.make_api_call(uri, None, EMPTY_BODY).await
    }

    pub(crate) async fn get_generic_obj<T: DeserializeOwned>(
        &self,
        kind: &str,
        uuid: &str,
    ) -> APIResult<T> {
        let uri = self.compute_basic_uri(kind, uuid);
        self.make_api_call(uri, None, NO_BODY).await
    }

    pub async fn update_role(
        &self,
        uuid: impl Into<String>,
        short_name: impl Into<String>,
        display_name: impl Into<String>,
        description: impl Into<String>,
    ) -> APIResult<public::role::update::Response> {
        let uri = self.compute_uri(PUBLIC_SEGMENT, public::role::update::URI);
        self.make_api_call(
            uri,
            None,
            Some(public::role::update::Request {
                uuid: uuid.into(),
                short_name: short_name.into(),
                display_name: display_name.into(),
                description: description.into(),
            }),
        )
        .await
    }

    pub async fn published_puzzle_list(
        &self,
        role_uuid: impl Into<String>,
    ) -> APIResult<public::role::puzzles::Response> {
        let uri = self.compute_uri(PUBLIC_SEGMENT, public::role::puzzles::URI);
        let req = public::role::puzzles::Request {
            uuid: role_uuid.into(),
        };
        self.make_api_call(uri, None, Some(req)).await
    }
}
