//! Server-side rendering for Linkdoku
//!
//! In order to render on the server side this must directly mirror
//! the shape of the client-side top level so that hydration
//! works properly

use std::collections::HashMap;

use frontend_core::BaseURIProvider;
use yew::prelude::*;
use yew_router::{
    history::{AnyHistory, History, MemoryHistory},
    Router,
};

use crate::Root;

#[derive(Clone, PartialEq, Properties)]
pub struct ServerAppProps {
    pub uri: String,
    pub query: HashMap<String, String>,
    pub base: AttrValue,
    pub login: Option<String>,
}

#[function_component(ServerApp)]
pub fn server_app(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    if history.push_with_query(&props.uri, &props.query).is_err() {
        html! {}
    } else {
        html! {
            <Router history={history}>
                <BaseURIProvider uri={props.base.clone()} login={props.login.clone()}>
                    <Root />
                </BaseURIProvider>
            </Router>
        }
    }
}
