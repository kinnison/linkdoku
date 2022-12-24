use apiprovider::ClientProvider;
use components::user::{UserMenuNavbarItem, UserProvider};
use frontend_core::{
    component::core::{Footer, Navbar},
    BaseURIProvider,
};
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
    // in [ssr::ServerApp].  Where providers don't have to be different, they'll be dealt
    // with in the Root element instead
    html! {
        <BrowserRouter>
            <BaseURIProvider>
                <Root />
            </BaseURIProvider>
        </BrowserRouter>
    }
}

#[function_component(Root)]
pub(crate) fn root_element() -> Html {
    html! {
        <ClientProvider>
            <UserProvider>
                <Navbar>
                    <UserMenuNavbarItem />
                </Navbar>
                <RouteSwitcher />
                <Footer />
            </UserProvider>
        </ClientProvider>
    }
}
