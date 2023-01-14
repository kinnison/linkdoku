use yew_router::prelude::*;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/role/:role")]
    ViewRole { role: String },
    #[at("/role/:role/edit")]
    EditRole { role: String },
    #[at("/puzzle/_new")]
    CreatePuzzle,

    // These routes are informational and not for general use
    #[at("/-/version-info")]
    VersionInformation,
    // The remaining routes are "internal"
    #[at("/-/complete-login")]
    CompleteLogin,
    #[not_found]
    #[at("/404")]
    NotFound,
}
