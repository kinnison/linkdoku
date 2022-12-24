//! Routes which are supported by the linkdoku frontend

use apiprovider::use_apiprovider;
use components::user::{LoginStatus, LoginStatusAction, LoginStatusDispatcher};
use serde::Deserialize;
use yew::{html::PhantomComponent, platform::spawn_local, prelude::*};
use yew_router::prelude::*;

use frontend_core::Route;
use yew_toastrack::{use_toaster, Toast, ToastLevel};

#[function_component(RouteSwitcher)]
pub fn route_switcher() -> Html {
    html! {
        <Switch<Route> render={route_switch} />
    }
}

fn route_switch(route: Route) -> Html {
    let page_html = match route {
        Route::Home => {
            html! {
                <>
                { "Welcome home" }
                <br />
                <Link<Route> to={Route::NotFound}>{"404 me"}</Link<Route>>
                </>
            }
        }
        Route::NotFound => {
            html! {
                <Redirect<Route> to={Route::Home} />
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
    let login_status = use_context::<LoginStatus>().unwrap();
    let login_status_dispatch = use_context::<LoginStatusDispatcher>().unwrap();
    let api = use_apiprovider();
    let toaster = use_toaster();

    let query: FlowContinuation = match location.query() {
        Ok(q) => q,
        Err(e) => {
            // Something bad happened, probably want to toast
            return html! {
                <Redirect<Route> to={Route::Home} />
            };
        }
    };

    use_effect_with_deps(
        {
            // Stuff
            move |status: &FlowContinuation| {
                let api = api.clone();
                let status = status.clone();
                spawn_local(async move {
                    match api
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
                                display_name: response.display_name,
                                gravatar_hash: response.gravatar_hash,
                                roles: response.roles,
                                default_role: response.default_role,
                            });
                            toaster.toast(Toast::new("You are now logged in").with_lifetime(2000));
                            nav.replace(&Route::Home);
                        }
                        Err(e) => {
                            // TODO: Toast this?
                            toaster.toast(
                                Toast::new(format!("Error logging in: {e:?}"))
                                    .with_level(ToastLevel::Danger)
                                    .with_lifetime(2000),
                            );
                            nav.replace(&Route::Home);
                        }
                    }
                });
                || ()
            }
        },
        query,
    );

    html! {
        "Handling login, please hold…"
    }
}
