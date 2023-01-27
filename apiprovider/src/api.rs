//! This is the structural API object which is
//! acquired when you use_apiprovider()

use std::{hash::Hash, sync::Arc};

use common::{
    internal::{self, login, logout, INTERNAL_SEGMENT},
    objects,
    public::{self, scaffold, userinfo, PUBLIC_SEGMENT},
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

    fn compute_basic_uri_by_uuid(&self, kind: &str, uuid: &str) -> Url {
        let combined = format!(
            "{}api{}/{}/by-uuid/{}",
            self.base.as_str(),
            PUBLIC_SEGMENT,
            kind,
            uuid
        );
        Url::parse(&combined).expect("Unable to construct API url?")
    }

    fn compute_basic_uri_by_name(&self, kind: &str, name: &str) -> Url {
        let combined = format!(
            "{}api{}/{}/by-name/{}",
            self.base.as_str(),
            PUBLIC_SEGMENT,
            kind,
            name
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

    pub async fn get_scaffold(&self) -> APIResult<scaffold::Response> {
        let uri = self.compute_uri(PUBLIC_SEGMENT, scaffold::URI);
        self.make_api_call(uri, None, NO_BODY).await
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
        let uri = self.compute_basic_uri_by_uuid(kind, uuid);
        self.make_api_call(uri, None, NO_BODY).await
    }

    pub(crate) async fn get_generic_obj_by_name<T: DeserializeOwned>(
        &self,
        kind: &str,
        name: &str,
    ) -> APIResult<T> {
        let uri = self.compute_basic_uri_by_name(kind, name);
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

    pub async fn create_puzzle(
        &self,
        owner: impl Into<String>,
        short_name: impl Into<String>,
        display_name: impl Into<String>,
        description: impl Into<String>,
        data: &objects::PuzzleData,
    ) -> APIResult<public::puzzle::create::Response> {
        let uri = self.compute_uri(PUBLIC_SEGMENT, public::puzzle::create::URI);
        let req = public::puzzle::create::Request {
            owner: owner.into(),
            display_name: display_name.into(),
            short_name: short_name.into(),
            initial_state: objects::PuzzleState {
                uuid: "".into(), // Ignored, but hey
                description: description.into(),
                visibility: objects::Visibility::Restricted, // Ignored, but hey
                updated_at: "".into(),                       // Ignored, but hey
                data: data.clone(),
            },
        };
        self.make_api_call(uri, None, Some(req)).await
    }

    pub async fn lookup_puzzle(
        &self,
        role: impl Into<String>,
        puzzle: impl Into<String>,
    ) -> APIResult<public::puzzle::lookup::Response> {
        let uri = self.compute_uri(PUBLIC_SEGMENT, public::puzzle::lookup::URI);
        let req = public::puzzle::lookup::Request {
            role: role.into(),
            puzzle: puzzle.into(),
        };
        self.make_api_call(uri, None, Some(req)).await
    }

    pub async fn update_puzzle_metadata(
        &self,
        puzzle: impl Into<String>,
        short_name: impl Into<String>,
        display_name: impl Into<String>,
    ) -> APIResult<public::puzzle::update_metadata::Response> {
        let uri = self.compute_uri(PUBLIC_SEGMENT, public::puzzle::update_metadata::URI);
        let req = public::puzzle::update_metadata::Request {
            puzzle: puzzle.into(),
            short_name: short_name.into(),
            display_name: display_name.into(),
        };
        self.make_api_call(uri, None, Some(req)).await
    }

    pub async fn update_puzzle_state(
        &self,
        puzzle: impl Into<String>,
        state: &objects::PuzzleState,
    ) -> APIResult<public::puzzle::update_state::Response> {
        let uri = self.compute_uri(PUBLIC_SEGMENT, public::puzzle::update_state::URI);
        let req = public::puzzle::update_state::Request {
            puzzle: puzzle.into(),
            state: state.clone(),
        };
        self.make_api_call(uri, None, Some(req)).await
    }

    pub async fn add_puzzle_state(
        &self,
        puzzle: impl Into<String>,
        state: &objects::PuzzleState,
    ) -> APIResult<public::puzzle::add_state::Response> {
        let uri = self.compute_uri(PUBLIC_SEGMENT, public::puzzle::add_state::URI);
        let req = public::puzzle::add_state::Request {
            puzzle: puzzle.into(),
            state: state.clone(),
        };
        self.make_api_call(uri, None, Some(req)).await
    }

    pub async fn set_puzzle_visibility(
        &self,
        puzzle: impl Into<String>,
        visibility: objects::Visibility,
    ) -> APIResult<public::puzzle::set_visibility::Response> {
        let uri = self.compute_uri(PUBLIC_SEGMENT, public::puzzle::set_visibility::URI);
        let req = public::puzzle::set_visibility::Request {
            puzzle: puzzle.into(),
            visibility,
        };
        self.make_api_call(uri, None, Some(req)).await
    }

    pub async fn set_puzzle_state_visibility(
        &self,
        puzzle: impl Into<String>,
        state: impl Into<String>,
        visibility: objects::Visibility,
    ) -> APIResult<public::puzzle::set_state_visibility::Response> {
        let uri = self.compute_uri(PUBLIC_SEGMENT, public::puzzle::set_state_visibility::URI);
        let req = public::puzzle::set_state_visibility::Request {
            puzzle: puzzle.into(),
            state: state.into(),
            visibility,
        };
        self.make_api_call(uri, None, Some(req)).await
    }

    pub async fn edit_puzzle_tags(
        &self,
        puzzle: impl Into<String>,
        to_add: &[String],
        to_remove: &[String],
    ) -> APIResult<public::puzzle::edit_tags::Response> {
        let uri = self.compute_uri(PUBLIC_SEGMENT, public::puzzle::edit_tags::URI);
        let req = public::puzzle::edit_tags::Request {
            puzzle: puzzle.into(),
            to_add: to_add.to_vec(),
            to_remove: to_remove.to_vec(),
        };
        self.make_api_call(uri, None, Some(req)).await
    }

    pub async fn find_tags(
        &self,
        pattern: impl Into<String>,
    ) -> APIResult<public::tag::list::Response> {
        let uri = self.compute_uri(PUBLIC_SEGMENT, public::tag::list::URI);
        let req = public::tag::list::Request {
            pattern: pattern.into(),
        };
        self.make_api_call(uri, None, Some(req)).await
    }

    pub async fn try_expand_tinyurl(
        &self,
        slug: impl Into<String>,
    ) -> APIResult<internal::util::expand_tinyurl::Response> {
        let uri = self.compute_uri(INTERNAL_SEGMENT, internal::util::expand_tinyurl::URI);
        let req = internal::util::expand_tinyurl::Request { slug: slug.into() };
        self.make_api_call(uri, None, Some(req)).await
    }
}
