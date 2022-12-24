use yew_router::prelude::*;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/-/complete-login")]
    CompleteLogin,
    #[not_found]
    #[at("/404")]
    NotFound,
}
