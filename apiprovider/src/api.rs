//! This is the structural API object which is
//! acquired when you use_apiprovider()

use std::sync::Arc;

use common::{
    internal::{login, logout, INTERNAL_SEGMENT},
    public::{userinfo, PUBLIC_SEGMENT},
    APIError, APIResult,
};
use reqwest::{Client, Url};
use serde::{de::DeserializeOwned, Serialize};
use yew::prelude::*;

use crate::backend::ReqwestClient;

use frontend_core::BaseURI;

#[derive(Clone)]
pub struct APIProvider {
    client: Arc<Client>,
    base: AttrValue,
}

#[hook]
pub fn use_apiprovider() -> APIProvider {
    let base = use_context::<BaseURI>()
        .expect("Invoked use_apiprovider() when not within a BaseURIProvider");
    let client = use_context::<ReqwestClient>()
        .expect("Invoked use_apiprovider() when not within a ClientProvider");

    APIProvider {
        client: client.client,
        base: (*base.uri).clone(),
    }
}

const NO_BODY: Option<()> = None;
const EMPTY_BODY: Option<Vec<String>> = Some(Vec::new());

impl APIProvider {
    fn compute_uri(&self, base: &str, func: &str) -> Url {
        let combined = format!("{}api{}{}", self.base.as_str(), base, func);
        Url::parse(&combined).expect("Unable to construct API URL?")
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
        let request = if let Some(body) = body {
            self.client.post(api).json(&body).build()
        } else {
            self.client.get(api).build()
        }
        .map_err(|e| APIError::ClientIssue(e.to_string()))?;
        let response = self
            .client
            .execute(request)
            .await
            .map_err(|e| APIError::ClientIssue(e.to_string()))?
            .error_for_status()
            .map_err(|e| APIError::ClientIssue(e.to_string()))?;
        response
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
}
