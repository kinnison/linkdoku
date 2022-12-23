use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::RouteSwitcher;

pub mod routes;

#[cfg(feature = "ssr")]
pub mod ssr;

#[function_component]
pub fn App() -> Html {
    // Note, this is the top-level application here, we *MUST* ensure that structurally if
    // we change the app layering (eg providers) then we must have the same layering
    // in [ssr::ServerApp].
    html! {
        <BrowserRouter>
            <RouteSwitcher />
        </BrowserRouter>
    }
}
