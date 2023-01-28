//! Login provision for Linkdoku, this is not the APIs, just the
//! core OpenID Connect behaviour

use std::{error::Error, sync::Arc};

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, State},
    http::{request::Parts, StatusCode},
    routing::{get, post},
    Json, RequestPartsExt, Router,
};
use common::{
    internal::{
        login::{begin, complete, providers},
        logout,
    },
    public::userinfo,
    APIError, APIResult,
};
use cookie::{Cookie, Key, SameSite};
use database::{activity, models};
use linked_hash_map::LinkedHashMap;
use openidconnect::{
    core::{CoreAuthenticationFlow, CoreClient, CoreGenderClaim, CoreProviderMetadata},
    reqwest::async_http_client,
    AuthorizationCode, ClientId, ClientSecret, CsrfToken, EmptyAdditionalClaims, IssuerUrl, Nonce,
    OAuth2TokenResponse, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, TokenResponse,
    UserInfoClaims,
};
use serde::{Deserialize, Serialize};
use tower_cookies::Cookies;
use tracing::{info, warn};
use url::Url;

use crate::{
    config::{ConfigState, Configuration},
    state::BackendState,
};

pub struct ProviderSetup {
    icon: String,
    client_id: String,
    client_secret: String,
    provider_metadata: CoreProviderMetadata,
    scopes: Vec<Scope>,
}

#[derive(Clone)]
pub struct Providers {
    inner: Arc<LinkedHashMap<String, ProviderSetup>>,
}

impl std::ops::Deref for Providers {
    type Target = LinkedHashMap<String, ProviderSetup>;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

#[tracing::instrument(skip(config))]
pub async fn load_providers(
    config: &Configuration,
) -> Result<Providers, Box<dyn Error + Send + Sync + 'static>> {
    let mut map = LinkedHashMap::new();
    for (name, oidp) in config.openid.iter() {
        info!("Loading OIDC metadata for {name} from config");
        let provider_metadata = CoreProviderMetadata::discover_async(
            IssuerUrl::new(oidp.discovery_doc.clone())?,
            async_http_client,
        )
        .await
        .map_err(|e| {
            warn!("Unable to load config for {name}: {e:?}");
            e
        })?;
        info!("Successfully loaded provider metadata for {name}");
        let client_id = oidp.client_id.clone();
        let client_secret = oidp.client_secret.clone();
        let scopes = oidp
            .scopes
            .iter()
            .map(String::clone)
            .map(Scope::new)
            .collect();
        map.insert(
            name.to_lowercase(),
            ProviderSetup {
                client_id,
                client_secret,
                provider_metadata,
                scopes,
                icon: oidp.icon.clone(),
            },
        );
    }

    Ok(Providers {
        inner: Arc::new(map),
    })
}

// ------------- login flow -------------

#[derive(Debug, Serialize, Deserialize)]
struct LoginFlowSetup {
    provider: String,
    pkce_verifier: PkceCodeVerifier,
    url: Url,
    csrf_token: CsrfToken,
    nonce: Nonce,
}

#[derive(Serialize, Deserialize)]
pub struct LoginFlowUserData {
    identity: models::Identity,
}

impl LoginFlowUserData {
    pub fn identity(&self) -> &models::Identity {
        &self.identity
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct LoginFlowStatus {
    flow: Option<LoginFlowSetup>,
    user: Option<LoginFlowUserData>,
}

impl LoginFlowStatus {
    pub fn user(&self) -> Option<&LoginFlowUserData> {
        self.user.as_ref()
    }

    pub fn user_uuid(&self) -> Option<&str> {
        self.user.as_ref().map(|u| u.identity().uuid.as_str())
    }
}

// ---- Private cookies stuff ----

pub struct PrivateCookies {
    key: Key,
    cookies: Cookies,
}

#[async_trait]
impl<S> FromRequestParts<S> for PrivateCookies
where
    S: Send + Sync,
    ConfigState: FromRef<S>,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cookies: Cookies = parts.extract().await?;
        let config = ConfigState::from_ref(state);

        Ok(Self {
            key: Key::derive_from(config.cookie_secret.as_bytes()),
            cookies,
        })
    }
}

impl PrivateCookies {
    fn get(&self) -> tower_cookies::PrivateCookies<'_> {
        self.cookies.private(&self.key)
    }

    pub async fn get_login_flow_status(&self) -> LoginFlowStatus {
        login_flow_status(self).await
    }
}

// ----- Login flow stuff -----
pub async fn login_flow_status(cookies: &PrivateCookies) -> LoginFlowStatus {
    serde_json::from_str(
        &cookies
            .get()
            .get("login")
            .map(|c| c.value().to_owned())
            .unwrap_or_default(),
    )
    .unwrap_or_default()
}

async fn set_login_flow_status(cookies: &PrivateCookies, login: &LoginFlowStatus) {
    cookies.get().add(
        Cookie::build(
            "login",
            serde_json::to_string(login).expect("Unable to serialise login"),
        )
        .path("/")
        .same_site(SameSite::Lax)
        .finish(),
    );
}

// ------------ routes ------------------

async fn handle_userinfo(
    cookies: PrivateCookies,
    mut db: database::Connection,
) -> Json<APIResult<userinfo::Response>> {
    let flow = login_flow_status(&cookies).await;

    match flow.user() {
        Some(user) => match user.identity.roles(&mut db).await {
            Ok(roles) => Json::from(Ok(userinfo::Response {
                info: Some(userinfo::UserInfo {
                    uuid: user.identity.uuid.clone(),
                    display_name: user.identity.display_name.clone(),
                    gravatar_hash: user.identity.gravatar_hash.clone(),
                    roles: roles.into_iter().map(|role| role.uuid).collect(),
                    default_role: user.identity.default_role_uuid(),
                }),
            })),
            Err(e) => Json::from(Err(APIError::DatabaseError(e.to_string()))),
        },
        None => Json::from(Ok(userinfo::Response { info: None })),
    }
}

async fn handle_logout(cookies: PrivateCookies) -> Json<APIResult<logout::Response>> {
    let mut flow = login_flow_status(&cookies).await;
    flow.flow = None;
    flow.user = None;
    set_login_flow_status(&cookies, &flow).await;
    Json::from(Ok(logout::Response {
        redirect_to: "/".into(),
    }))
}

async fn handle_providers(
    State(providers): State<Providers>,
) -> Json<APIResult<providers::Response>> {
    Json::from(Ok(providers
        .iter()
        .map(|(s, prov)| providers::Provider {
            name: s.clone(),
            icon: prov.icon.clone(),
        })
        .collect::<Vec<_>>()))
}

async fn handle_start_auth(
    cookies: PrivateCookies,
    State(providers): State<Providers>,
    State(config): State<ConfigState>,
    Json(request): Json<begin::Request>,
) -> Json<APIResult<begin::Response>> {
    let mut flow = login_flow_status(&cookies).await;
    // First up, if we're already logged in, just redirect the user to the root of the app
    if flow.user.is_some() {
        return Json::from(Ok(begin::Response::LoggedIn));
    }
    if let Some(setup) = flow.flow.as_ref() {
        if setup.provider == request.provider {
            // We already have a login flow in progress, so redirect the user again
            return Json::from(Ok(begin::Response::Continue(setup.url.to_string())));
        }
    }
    if let Some(provider_data) = providers.get(&request.provider) {
        // Either no flow in progress, or user is trying a different flow for whatever reason
        let client = CoreClient::from_provider_metadata(
            provider_data.provider_metadata.clone(),
            ClientId::new(provider_data.client_id.clone()),
            Some(ClientSecret::new(provider_data.client_secret.clone())),
        )
        .set_redirect_uri(RedirectUrl::new(config.redirect_url.clone()).unwrap());

        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
        let (url, csrf_token, nonce) = {
            let mut actor = client.authorize_url(
                CoreAuthenticationFlow::AuthorizationCode,
                CsrfToken::new_random,
                Nonce::new_random,
            );
            for scope in provider_data.scopes.iter() {
                actor = actor.add_scope(scope.clone());
            }
            actor.set_pkce_challenge(pkce_challenge).url()
        };

        flow.flow = Some(LoginFlowSetup {
            provider: request.provider.clone(),
            pkce_verifier,
            url: url.clone(),
            csrf_token,
            nonce,
        });

        tracing::info!("Set up flow: {:?}", flow.flow);

        set_login_flow_status(&cookies, &flow).await;

        Json::from(Ok(begin::Response::Continue(url.to_string())))
    } else {
        // Selected provider was not available, let's go again
        Json::from(Err(APIError::UnknownLoginProvider(request.provider)))
    }
}

async fn handle_login_continue(
    cookies: PrivateCookies,
    mut db: database::Connection,
    State(providers): State<Providers>,
    State(config): State<ConfigState>,
    Json(params): Json<complete::Request>,
) -> Json<APIResult<complete::Response>> {
    let mut flow = login_flow_status(&cookies).await;
    // First up, if we're already logged in, just redirect the user to the root of the app
    if let Some(user) = &flow.user {
        let roles = match user.identity.roles(&mut db).await {
            Ok(roles) => roles,
            Err(e) => {
                return Json::from(Err(APIError::DatabaseError(e.to_string())));
            }
        };

        return Json::from(Ok(userinfo::UserInfo {
            uuid: user.identity.uuid.clone(),
            display_name: user.identity.display_name.clone(),
            gravatar_hash: user.identity.gravatar_hash.clone(),
            roles: roles.into_iter().map(|role| role.uuid).collect(),
            default_role: user.identity.default_role_uuid(),
        }));
    }
    if let Some(setup) = flow.flow.as_ref() {
        // Flow is in progress, so let's check the state first
        if params.state.as_str() != setup.csrf_token.secret() {
            // State value is bad, so clean up and BAD_REQUEST
            flow.flow = None;
            set_login_flow_status(&cookies, &flow).await;
            return Json::from(Err(APIError::BadLoginStateToken));
        }
        if let Some(error) = params.error {
            tracing::error!("Error in flow: {}", error);
            flow.flow = None;
            set_login_flow_status(&cookies, &flow).await;
            return Json::from(Err(APIError::LoginFlowError(error)));
        }
        let code = params.code.as_deref().unwrap();
        tracing::info!("Trying to transact code: {}", code);
        if let Some(provider_data) = providers.get(&setup.provider) {
            let client = CoreClient::from_provider_metadata(
                provider_data.provider_metadata.clone(),
                ClientId::new(provider_data.client_id.clone()),
                Some(ClientSecret::new(provider_data.client_secret.clone())),
            )
            .set_redirect_uri(RedirectUrl::new(config.redirect_url.clone()).unwrap());
            match client
                .exchange_code(AuthorizationCode::new(code.to_string()))
                .set_pkce_verifier(PkceCodeVerifier::new(setup.pkce_verifier.secret().clone()))
                .request_async(async_http_client)
                .await
            {
                Ok(token_response) => {
                    let id_token = match token_response.id_token() {
                        Some(token) => token,
                        None => {
                            tracing::error!("Failed to get id_token");
                            flow.flow = None;
                            set_login_flow_status(&cookies, &flow).await;
                            return Json::from(Err(APIError::NoIdentityToken));
                        }
                    };
                    let claims = match id_token.claims(&client.id_token_verifier(), &setup.nonce) {
                        Ok(claims) => claims,
                        Err(e) => {
                            tracing::error!("Failed to verify id_token: {:?}", e);
                            flow.flow = None;
                            set_login_flow_status(&cookies, &flow).await;
                            return Json::from(Err(APIError::BadIdentityToken));
                        }
                    };
                    let uinfo: UserInfoClaims<EmptyAdditionalClaims, CoreGenderClaim> = match {
                        match client.user_info(token_response.access_token().clone(), None) {
                            Ok(uinfo) => uinfo,
                            Err(e) => {
                                tracing::error!("Failed to acquire user info: {e:?}");
                                flow.flow = None;
                                set_login_flow_status(&cookies, &flow).await;
                                return Json::from(Err(APIError::LoginFlowError(
                                    "Unable to create user info request".into(),
                                )));
                            }
                        }
                    }
                    .request_async(async_http_client)
                    .await
                    {
                        Ok(uinfo) => uinfo,
                        Err(e) => {
                            tracing::error!("Failed to acquire user info: {e:?}");
                            flow.flow = None;
                            set_login_flow_status(&cookies, &flow).await;
                            return Json::from(Err(APIError::LoginFlowError(
                                "Unable to acquire user info".into(),
                            )));
                        }
                    };
                    // Okay, at this point we *are* logged in, so let's prepare our data
                    let subject = format!("{}:{}", setup.provider, claims.subject().as_str());
                    let name = claims
                        .name()
                        .and_then(|n| n.get(None).map(|n| n.to_string()));
                    let name = name.or_else(|| {
                        uinfo
                            .name()
                            .and_then(|n| n.get(None).map(|n| n.to_string()))
                    });
                    let email = claims
                        .email()
                        .or_else(|| uinfo.email())
                        .map(|e| e.to_string());

                    flow.flow = None;

                    // At this point we want to log in, which means creating an identity if there isn't one, and then
                    // acquiring it and putting it into our flow status
                    let gravatar_hash = format!(
                        "{:x}",
                        md5::compute(email.as_deref().unwrap_or(subject.as_str()))
                    );
                    let (identity, roles) = match activity::login::login_upsert(
                        &mut db,
                        &subject,
                        &gravatar_hash,
                        name.as_deref().unwrap_or(subject.as_str()),
                    )
                    .await
                    {
                        Ok(identity) => identity,
                        Err(e) => {
                            return Json::from(Err(APIError::DatabaseError(e.to_string())));
                        }
                    };
                    // Prepare the flow
                    let ret = userinfo::UserInfo {
                        uuid: identity.uuid.clone(),
                        display_name: identity.display_name.clone(),
                        gravatar_hash,
                        roles: roles.into_iter().map(|role| role.uuid).collect(),
                        default_role: identity.default_role_uuid(),
                    };
                    flow.user = Some(LoginFlowUserData { identity });
                    set_login_flow_status(&cookies, &flow).await;
                    Json::from(Ok(ret))
                }
                Err(e) => {
                    // Failed to exchange the token, return something
                    tracing::error!("Failed exchanging codes: {:?}", e);
                    flow.flow = None;
                    set_login_flow_status(&cookies, &flow).await;
                    Json::from(Err(APIError::LoginCodeExchangeFailed))
                }
            }
        } else {
            let mut flow = login_flow_status(&cookies).await;
            flow.flow = None;
            set_login_flow_status(&cookies, &flow).await;
            Json::from(Err(APIError::UnknownLoginProvider(setup.provider.clone())))
        }
    } else {
        // No login in progress, redirect user to root
        Json::from(Err(APIError::LoginFlowError("No login in progess".into())))
    }
}

// ------------ routers -----------------
pub fn internal_router() -> Router<BackendState> {
    Router::new()
        .route(logout::URI, post(handle_logout))
        .route(providers::URI, get(handle_providers))
        .route(begin::URI, post(handle_start_auth))
        .route(complete::URI, post(handle_login_continue))
}

pub fn public_router() -> Router<BackendState> {
    Router::new().route(userinfo::URI, get(handle_userinfo))
}
