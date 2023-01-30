//! User related componentry, including state reducer and friends

use std::rc::Rc;

use apiprovider::use_apiprovider;
use frontend_core::{
    component::{icon::GenericIcon, user::Avatar},
    LinkdokuBase, Route,
};
use tracing::{debug, error};
use yew::{platform::spawn_local, prelude::*, suspense::use_future};
use yew_router::prelude::*;
use yew_toastrack::{use_toaster, Toast};

use crate::role::Role;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoginStatus {
    LoggedOut,
    LoggedIn {
        uuid: String,
        display_name: String,
        gravatar_hash: String,
        roles: Vec<String>,
        role: String,
    },
}

impl LoginStatus {
    fn choose_role(&self, role: String) -> Self {
        match self {
            Self::LoggedOut => Self::LoggedOut,
            Self::LoggedIn {
                uuid,
                display_name,
                gravatar_hash,
                roles,
                ..
            } => Self::LoggedIn {
                uuid: uuid.clone(),
                display_name: display_name.clone(),
                gravatar_hash: gravatar_hash.clone(),
                roles: roles.clone(),
                role,
            },
        }
    }

    pub fn is_logged_in(&self) -> bool {
        matches! {self, Self::LoggedIn{..}}
    }

    pub fn roles(&self) -> &[String] {
        match self {
            Self::LoggedIn { roles, .. } => roles,
            _ => &[],
        }
    }

    pub fn current_role(&self) -> Option<&str> {
        match self {
            Self::LoggedIn { role, .. } => Some(role.as_str()),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum LoginStatusAction {
    LoggedOut,
    LoggedIn {
        uuid: String,
        display_name: String,
        gravatar_hash: String,
        roles: Vec<String>,
        default_role: String,
    },
    ChosenRole(String),
}

impl Reducible for LoginStatus {
    type Action = LoginStatusAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        debug!("Reducing login status by {action:?}");
        match action {
            LoginStatusAction::LoggedOut => LoginStatus::LoggedOut,
            LoginStatusAction::LoggedIn {
                uuid,
                display_name,
                gravatar_hash,
                roles,
                default_role,
            } => LoginStatus::LoggedIn {
                uuid,
                display_name,
                gravatar_hash,
                roles,
                role: default_role,
            },
            LoginStatusAction::ChosenRole(role) => self.choose_role(role),
        }
        .into()
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct UserProviderProps {
    pub children: Children,
}

pub type LoginStatusDispatcher = UseReducerDispatcher<LoginStatus>;

#[function_component(UserProvider)]
pub fn login_user_provider(props: &UserProviderProps) -> Html {
    let base = use_context::<LinkdokuBase>().unwrap();
    let state = use_reducer_eq(|| {
        base.userinfo
            .as_ref()
            .map(|u| {
                debug!("Initialising user provider with: {u:?}");
                LoginStatus::LoggedIn {
                    display_name: u.display_name.clone(),
                    gravatar_hash: u.gravatar_hash.clone(),
                    role: u.default_role.clone(),
                    uuid: u.uuid.clone(),
                    roles: u.roles.clone(),
                }
            })
            .unwrap_or_else(|| {
                debug!("Initialising user provider as logged out");
                LoginStatus::LoggedOut
            })
    });

    html! {
        <ContextProvider<LoginStatus> context={(*state).clone()}>
            <ContextProvider<LoginStatusDispatcher> context={state.dispatcher()}>
                {props.children.clone()}
            </ContextProvider<LoginStatusDispatcher>>
        </ContextProvider<LoginStatus>>
    }
}

#[function_component(LoginButtonx)]
pub fn login_button() -> Html {
    let nav = use_navigator().unwrap();
    let api = use_apiprovider();
    let login_click = Callback::from(move |_| {
        let nav = nav.clone();
        let api = api.clone();
        spawn_local(async move {
            match api.start_login("google").await {
                Ok(response) => {
                    use common::internal::login::begin;
                    match response {
                        begin::Response::LoggedIn => {
                            // Nothing to do, let's just force a jump to the root
                            nav.push(&Route::Home);
                        }
                        begin::Response::Continue(url) => {
                            gloo::utils::window().location().set_href(&url).unwrap();
                        }
                    }
                }
                Err(e) => {
                    error!("Woah, API error: {e:?}");
                    // TODO: Nothing for now, maybe toast later?
                }
            }
        });
    });

    html! {
        <button class={"button is-primary"} onclick={login_click}>
            <span class="icon-text">
                <GenericIcon icon="mdi-google" />
                <span>{"Login with Google"}</span>
            </span>
        </button>
    }
}

#[function_component(LoginButton)]
pub fn login_buttons() -> Html {
    let fallback = html! {};
    html! {
        <Suspense fallback={fallback}>
            <LoginButtonsInner />
        </Suspense>
    }
}

#[function_component(LoginButtonsInner)]
pub fn login_buttons_inner() -> HtmlResult {
    let nav = use_navigator().unwrap();
    let api = use_apiprovider();
    let toaster = use_toaster();
    let providers = use_future({
        let api = api.clone();
        move || async move { api.login_providers().await }
    })?;

    let p = match providers.as_ref() {
        Err(e) => {
            toaster.toast(
                Toast::new(format!("Unable to retrieve login providers: {e}"))
                    .with_level(yew_toastrack::ToastLevel::Danger)
                    .with_lifetime(5000),
            );
            return Ok(html! {});
        }
        Ok(p) => p,
    };

    let login_click = Callback::from(move |name: String| {
        let nav = nav.clone();
        let api = api.clone();
        spawn_local(async move {
            match api.start_login(&name).await {
                Ok(response) => {
                    use common::internal::login::begin;
                    match response {
                        begin::Response::LoggedIn => {
                            // Nothing to do, let's just force a jump to the root
                            nav.push(&Route::Home);
                        }
                        begin::Response::Continue(url) => {
                            gloo::utils::window().location().set_href(&url).unwrap();
                        }
                    }
                }
                Err(e) => {
                    error!("Woah, API error: {e:?}");
                    // TODO: Nothing for now, maybe toast later?
                }
            }
        });
    });

    let mut buttons = Vec::new();
    for (prov, bclass) in p
        .providers
        .iter()
        .zip(std::iter::once("is-primary").chain(std::iter::repeat("is-link")))
    {
        let cb = Callback::from({
            let login_click = login_click.clone();
            let name = prov.name.to_lowercase();
            move |_| {
                login_click.emit(name.clone());
            }
        });
        let title = format!("Log in using {}", prov.name.clone());
        let buttonclasses = classes!("button", bclass);
        buttons.push(html! {
            <button class={buttonclasses} onclick={cb} title={title}>
                <GenericIcon icon={prov.icon.clone()} />
            </button>
        });
    }

    Ok(html! {
        <div class="control buttons">
            {for buttons}
        </div>
    })
}

#[function_component(LogoutButton)]
pub fn logout_button() -> Html {
    let login_status_dispatch =
        use_context::<LoginStatusDispatcher>().expect("Cannot get login status dispatcher");
    let nav = use_navigator().unwrap();
    let api = use_apiprovider();
    let logout_click = Callback::from(move |_| {
        // To log out, we should run the logout api, and then reroute
        let api = api.clone();
        let login_status_dispatch = login_status_dispatch.clone();
        let nav = nav.clone();
        spawn_local(async move {
            match api.logout().await {
                Ok(response) => {
                    login_status_dispatch.dispatch(LoginStatusAction::LoggedOut);
                    if let Some(route) = Route::recognize(&response.redirect_to) {
                        nav.push(&route);
                    }
                }
                Err(e) => {
                    error!("Woah API error: {e:?}");
                    // TODO: Nothing for now, but maybe toast later?
                }
            }
        });
    });
    html! {
        <button class={"button is-danger"} onclick={logout_click}>
            {"Log out"}
        </button>
    }
}

#[function_component(UserMenuNavbarItem)]
pub fn user_menu_button() -> Html {
    let login_status_dispatch =
        use_context::<LoginStatusDispatcher>().expect("Cannot get login status dispatcher");
    match use_context::<LoginStatus>().expect("Unable to retrieve login status") {
        LoginStatus::LoggedOut => html! {
            <div class={"navbar-item"}>
                <div class={"buttons"}>
                    <LoginButton />
                </div>
            </div>
        },
        LoginStatus::LoggedIn {
            display_name,
            gravatar_hash,
            roles,
            role,
            ..
        } => {
            let roles = roles
                .into_iter()
                .map(|this_role| {
                    let emitter = login_status_dispatch.clone();
                    let role_uuid = this_role.clone();
                    let onclick = Callback::from(move |_| emitter.dispatch(LoginStatusAction::ChosenRole(role_uuid.clone())));
                    html! {
                        <div class={"navbar-item"}>
                            <Role active={role == this_role} uuid={this_role.clone()} onclick={onclick} />
                        </div>
                    }
                })
                .collect::<Html>();

            html! {
                <div class={"navbar-item has-dropdown is-hoverable"}>
                    <a class={"navbar-link"}>
                        <Avatar gravatar_hash={gravatar_hash} />
                        <span>{display_name}</span>
                    </a>

                    <div class={"navbar-dropdown is-right"}>
                        {roles}
                        <hr class={"navbar-divider"} />
                        <div class={"navbar-item"}>
                            <div class={"buttons"}>
                                <LogoutButton />
                            </div>
                        </div>
                    </div>
                </div>
            }
        }
    }
}
