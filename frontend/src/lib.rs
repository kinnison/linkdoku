use apiprovider::LinkdokuAPIProvider;
use bounce::{
    helmet::{Helmet, HelmetBridge},
    BounceRoot,
};
use components::{
    layout::VersionChecker,
    user::{UserMenuNavbarItem, UserProvider},
};
use frontend_core::{
    component::core::{Footer, Navbar},
    make_title, BaseProvider,
};
use yew::prelude::*;
use yew_router::prelude::*;
use yew_toastrack::ToastContainer;

use crate::routes::RouteSwitcher;

pub(crate) mod pages;
pub mod routes;

pub(crate) mod util_components;

pub(crate) mod help_texts;

#[cfg(feature = "ssr")]
pub mod ssr;

#[function_component]
pub fn App() -> Html {
    // Note, this is the top-level application here, we *MUST* ensure that structurally if
    // we change the app layering (eg providers) then we must have the same layering
    // in [ssr::ServerApp].  Where providers don't have to be different, they'll be dealt
    // with in the Root element instead
    html! {
        <BounceRoot>
            <HelmetBridge default_title={make_title("A Sudoku puzzle site")} />
            <BrowserRouter>
                <BaseProvider>
                    <Root />
                </BaseProvider>
            </BrowserRouter>
        </BounceRoot>
    }
}

#[function_component(Root)]
pub(crate) fn root_element() -> Html {
    html! {
        <LinkdokuAPIProvider>
            <UserProvider>
                <ToastContainer>
                    <Helmet>
                        <meta charset="utf-8" />
                    </Helmet>
                    <VersionChecker />
                    <Navbar>
                        <UserMenuNavbarItem />
                    </Navbar>
                    <RouteSwitcher />
                    <Footer />
                </ToastContainer>
            </UserProvider>
        </LinkdokuAPIProvider>
    }
}
