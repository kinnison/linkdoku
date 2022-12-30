use std::rc::Rc;

use common::public::userinfo::UserInfo;
use url::Url;
use yew::prelude::*;
use yew_router::prelude::use_location;

#[derive(Clone, PartialEq)]
pub struct LinkdokuBase {
    pub uri: Rc<AttrValue>,
    pub login: Option<Rc<AttrValue>>,
    pub userinfo: Option<UserInfo>,
}

#[derive(Clone, Properties, PartialEq)]
pub struct BaseURIProviderProps {
    pub uri: Option<AttrValue>,
    pub login: Option<AttrValue>,
    pub userinfo: Option<UserInfo>,
    pub children: Children,
}

#[function_component(BaseProvider)]
pub fn core_base_provider(props: &BaseURIProviderProps) -> Html {
    let uri = use_memo(
        |uri| match uri {
            Some(uri) => uri.clone(),
            None => {
                let uri = gloo::utils::document()
                    .base_uri()
                    .expect("Could not read document")
                    .expect("Document lacked .baseURI");
                let mut uri = Url::parse(&uri).expect("BaseURI was bad");
                uri.set_path("/");
                uri.set_fragment(None);
                uri.set_query(None);
                uri.to_string().into()
            }
        },
        props.uri.clone(),
    );

    let context = LinkdokuBase {
        uri,
        login: props.login.clone().map(Rc::new),
        userinfo: props.userinfo.clone(),
    };

    html! {
        <ContextProvider<LinkdokuBase> context={context}>
            { for props.children.iter() }
        </ContextProvider<LinkdokuBase>>
    }
}

#[hook]
pub fn use_asset_url<S: AsRef<str>>(asset: S) -> String {
    let base = use_context::<LinkdokuBase>().unwrap();
    format!("{}assets/{}", base.uri, asset.as_ref())
}

#[hook]
pub fn use_page_url() -> String {
    let base = use_context::<LinkdokuBase>().unwrap();
    let loc = use_location().unwrap();
    if let Some(rest) = loc.path().strip_prefix('/') {
        format!("{}{}", base.uri, rest)
    } else {
        format!("{}{}", base.uri, loc.path())
    }
}
