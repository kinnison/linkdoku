//! Server-side rendering for Linkdoku
//!
//! In order to render on the server side this must directly mirror
//! the shape of the client-side top level so that hydration
//! works properly

use std::collections::HashMap;

use bounce::{
    helmet::{HelmetBridge, StaticWriter},
    BounceRoot,
};
use common::public::userinfo::UserInfo;
use frontend_core::{make_title, BaseProvider};
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
    pub userinfo: Option<UserInfo>,
    pub header_writer: StaticWriter,
}

#[function_component(ServerApp)]
pub fn server_app(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    if history.push_with_query(&props.uri, &props.query).is_err() {
        html! {}
    } else {
        html! {
            <BounceRoot>
                <HelmetBridge default_title={make_title("A Sudoku puzzle site")} writer={props.header_writer.clone()} />
                <Router history={history}>
                    <BaseProvider uri={props.base.clone()} login={props.login.clone()} userinfo={props.userinfo.clone()}>
                        <Root />
                    </BaseProvider>
                </Router>
            </BounceRoot>
        }
    }
}
