//! Routes which are supported by the linkdoku frontend

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

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
    };

    html! {
        <div class={"block mt-4 mx-4"}>
            { page_html }
        </div>
    }
}
