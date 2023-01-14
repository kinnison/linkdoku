//! Routes which are supported by the linkdoku frontend

use apiprovider::use_apiprovider;
use components::user::{LoginStatusAction, LoginStatusDispatcher};
use serde::Deserialize;
#[cfg(feature = "ssr")]
use yew::html::PhantomComponent;
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;

use frontend_core::{component::core::VersionInfo, Route};
use yew_toastrack::{use_toaster, Toast, ToastLevel};

use crate::{
    pages::{
        home::HomePage,
        puzzle::CreatePuzzlePage,
        role::{RoleEditPage, RolePage},
        shortcuts::ShortcutHandler,
    },
    util_components::Title,
};

#[function_component(RouteSwitcher)]
pub fn route_switcher() -> Html {
    html! {
        <Switch<Route> render={core_frontend_route_switch} />
    }
}

pub fn core_frontend_route_switch(route: Route) -> Html {
    let page_html = match route {
        Route::Home => {
            html! {
                <HomePage />
            }
        }

        Route::ViewRole { role } => {
            html! {
                <RolePage role={role} />
            }
        }
        Route::EditRole { role } => {
            html! {
                <RoleEditPage role={role} />
            }
        }

        Route::CreatePuzzle => {
            html! {
                <CreatePuzzlePage />
            }
        }

        // Uncommon routes
        Route::VersionInformation => {
            html! {
                <>
                    <Title value={"Version information"} />
                    <VersionInfo />
                </>
            }
        }
        // Internal routes
        Route::Shortcut => {
            html! {
                <ShortcutHandler />
            }
        }
        #[cfg(not(feature = "ssr"))]
        Route::CompleteLogin => {
            html! {
                <HandleLoginFlow />
            }
        }
        #[cfg(feature = "ssr")]
        Route::CompleteLogin => {
            html! {
                <PhantomComponent<HandleLoginFlow> />
            }
        }
    };

    html! {
        <div class={"block mt-4 mx-4"}>
            { page_html }
        </div>
    }
}

#[derive(Deserialize, PartialEq, Clone)]
struct FlowContinuation {
    state: String,
    error: Option<String>,
    code: Option<String>,
}

#[function_component(HandleLoginFlow)]
fn login_flow() -> Html {
    let nav = use_navigator().unwrap();
    let location = use_location().unwrap();
    let login_status_dispatch = use_context::<LoginStatusDispatcher>().unwrap();
    let api = use_apiprovider();
    let toaster = use_toaster();

    let query: Result<FlowContinuation, _> = location.query();

    use_effect_with_deps(
        {
            // Stuff
            move |status: &Result<FlowContinuation, String>| {
                let api = api.clone();
                let status = status.clone();
                spawn_local(async move {
                    match status {
                        Ok(status) => match api
                            .complete_login(
                                status.state.as_str(),
                                status.code.as_deref(),
                                status.error.as_deref(),
                            )
                            .await
                        {
                            Ok(response) => {
                                // Hurrah, logged in OK, so let's report that
                                login_status_dispatch.dispatch(LoginStatusAction::LoggedIn {
                                    uuid: response.uuid,
                                    display_name: response.display_name,
                                    gravatar_hash: response.gravatar_hash,
                                    roles: response.roles,
                                    default_role: response.default_role,
                                });
                                toaster
                                    .toast(Toast::new("You are now logged in").with_lifetime(2000));
                                nav.replace(&Route::Home);
                            }
                            Err(e) => {
                                // TODO: Toast this?
                                toaster.toast(
                                    Toast::new(format!("Error logging in: {e:?}"))
                                        .with_level(ToastLevel::Danger)
                                        .with_lifetime(5000),
                                );
                                nav.replace(&Route::Home);
                            }
                        },
                        Err(e) => {
                            toaster.toast(
                                Toast::new(format!("Error with login params: {e}"))
                                    .with_level(ToastLevel::Danger)
                                    .with_lifetime(5000),
                            );
                            nav.replace(&Route::Home);
                        }
                    }
                });
                || ()
            }
        },
        query.map_err(|e| format!("{e:?}")),
    );

    html! {
        "Handling login, please hold…"
    }
}
