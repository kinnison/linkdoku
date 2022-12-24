//! User related componentry, including state reducer and friends

use std::rc::Rc;

use apiprovider::use_apiprovider;
use frontend_core::{component::user::Avatar, Route};
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoginStatus {
    Unknown,
    LoggedOut,
    LoggedIn {
        display_name: String,
        gravatar_hash: String,
        roles: Vec<String>,
        role: String,
    },
}

impl LoginStatus {
    fn choose_role(&self, role: String) -> Self {
        match self {
            Self::Unknown => Self::Unknown,
            Self::LoggedOut => Self::LoggedOut,
            Self::LoggedIn {
                display_name,
                gravatar_hash,
                roles,
                ..
            } => Self::LoggedIn {
                display_name: display_name.clone(),
                gravatar_hash: gravatar_hash.clone(),
                roles: roles.clone(),
                role,
            },
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches! {self, Self::Unknown}
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
        log::debug!("Reducing login status by {action:?}");
        match action {
            LoginStatusAction::LoggedOut => LoginStatus::LoggedOut,
            LoginStatusAction::LoggedIn {
                display_name,
                gravatar_hash,
                roles,
                default_role,
            } => LoginStatus::LoggedIn {
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
    let api = use_apiprovider();
    let state = use_reducer_eq(|| LoginStatus::Unknown);

    // First time out of the gate, acquire the status
    use_effect({
        let dispatcher = state.dispatcher();
        let current_state = (*state).clone();
        move || {
            if current_state == LoginStatus::Unknown {
                spawn_local(async move {
                    match api.get_userinfo().await {
                        Ok(status) => {
                            if let Some(info) = status.info {
                                dispatcher.dispatch(LoginStatusAction::LoggedIn {
                                    display_name: info.display_name,
                                    gravatar_hash: info.gravatar_hash,
                                    roles: info.roles,
                                    default_role: info.default_role,
                                });
                            } else {
                                // Not logged in
                                dispatcher.dispatch(LoginStatusAction::LoggedOut);
                            }
                        }
                        Err(e) => {
                            log::error!("Woah, API error: {e:?}");
                            // Nothing for now, next time anything happens, we'll try again to talk to the server.
                        }
                    }
                });
            }
            // No destructor
            || ()
        }
    });

    html! {
        <ContextProvider<LoginStatus> context={(*state).clone()}>
            <ContextProvider<LoginStatusDispatcher> context={state.dispatcher()}>
                {props.children.clone()}
            </ContextProvider<LoginStatusDispatcher>>
        </ContextProvider<LoginStatus>>
    }
}

#[function_component(LoginButton)]
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
                    log::error!("Woah, API error: {e:?}");
                    // TODO: Nothing for now, maybe toast later?
                }
            }
        });
    });

    html! {
        <button class={"button is-primary"} onclick={login_click}>
            {"Login with Google"}
        </button>
    }
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
                    log::error!("Woah API error: {e:?}");
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
        LoginStatus::Unknown => html! {},
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
                    //let onclick = Callback::from(move |_| emitter.dispatch(LoginStatusAction::ChosenRole(role_uuid.clone())));
                    html! {
                        <div class={"navbar-item"}>
                            {"<Role active={role == this_role} uuid={this_role.clone()} onclick={onclick} />"}
                        </div>
                    }
                })
                .collect::<Html>();

            html! {
                <div class={"navbar-item has-dropdown is-hoverable"}>
                    <a class={"navbar-link"}>
                        <Avatar gravatar_hash={gravatar_hash} />
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
