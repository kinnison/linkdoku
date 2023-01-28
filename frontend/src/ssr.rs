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
use sentry_core::Span;
use yew::prelude::*;
use yew_router::{
    history::{AnyHistory, History, MemoryHistory},
    Router,
};

use crate::Root;

#[derive(Properties)]
pub struct ServerAppProps {
    pub uri: String,
    pub query: HashMap<String, String>,
    pub base: AttrValue,
    pub login: Option<String>,
    pub userinfo: Option<UserInfo>,
    pub header_writer: StaticWriter,
    pub linkdoku_svg_asset: AttrValue,
    pub span: Option<Span>,
}

impl PartialEq for ServerAppProps {
    fn eq(&self, other: &Self) -> bool {
        self.uri == other.uri
            && self.query == other.query
            && self.base == other.base
            && self.login == other.login
            && self.userinfo == other.userinfo
            && self.header_writer == other.header_writer
            && self.linkdoku_svg_asset == other.linkdoku_svg_asset
    }
}

impl Drop for ServerAppProps {
    fn drop(&mut self) {
        if let Some(span) = self.span.take() {
            span.finish();
        }
    }
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
                    <BaseProvider uri={props.base.clone()} login={props.login.clone()} userinfo={props.userinfo.clone()} linkdoku_svg_asset={props.linkdoku_svg_asset.clone()}>
                        <Root />
                    </BaseProvider>
                </Router>
            </BounceRoot>
        }
    }
}
