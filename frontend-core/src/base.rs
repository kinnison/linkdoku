use std::rc::Rc;

use common::public::userinfo::UserInfo;
use url::Url;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq)]
pub struct LinkdokuBase {
    pub uri: Rc<AttrValue>,
    pub login: Option<Rc<AttrValue>>,
    pub userinfo: Option<UserInfo>,
    pub asset_str: AttrValue,
}

#[derive(Clone, Properties, PartialEq)]
pub struct BaseURIProviderProps {
    pub uri: Option<AttrValue>,
    pub login: Option<AttrValue>,
    pub userinfo: Option<UserInfo>,
    pub children: Children,
    pub linkdoku_svg_asset: Option<AttrValue>,
}

#[function_component(BaseProvider)]
pub fn core_base_provider(props: &BaseURIProviderProps) -> Html {
    let fallback = html! {};
    html! {
        <Suspense fallback={fallback}>
            <BaseProviderInner uri={props.uri.clone()} login={props.login.clone()} userinfo={props.userinfo.clone()} linkdoku_svg_asset={props.linkdoku_svg_asset.clone()}>
                {for props.children.iter()}
            </BaseProviderInner>
        </Suspense>
    }
}

#[function_component(BaseProviderInner)]
pub fn core_base_provider_inner(props: &BaseURIProviderProps) -> HtmlResult {
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

    let _raw_userinfo = props.userinfo.clone();

    let prepared_userinfo =
        use_prepared_state!(move |_| -> Option<UserInfo> { _raw_userinfo }, ())?
            .and_then(|v| (*v).clone());

    let _raw_asset = props.linkdoku_svg_asset.as_ref().map(|s| s.to_string());

    let prepared_svg_asset = use_prepared_state!(move |_| -> Option<String> { _raw_asset }, ())?
        .and_then(|v| (*v).clone());

    let asset_str = match prepared_svg_asset {
        Some(s) => s,
        None => {
            // Find the link tag in the head since we're in the browser here
            todo!()
        }
    };

    let context = LinkdokuBase {
        uri,
        login: props.login.clone().map(Rc::new),
        userinfo: prepared_userinfo,
        asset_str: asset_str.into(),
    };

    Ok(html! {
        <ContextProvider<LinkdokuBase> context={context}>
            { for props.children.iter() }
        </ContextProvider<LinkdokuBase>>
    })
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

#[hook]
pub fn use_route_url<R: Routable>(route: &R) -> String {
    let base = use_context::<LinkdokuBase>().unwrap();
    let path = route.to_path();
    format!(
        "{}{}",
        base.uri,
        path.strip_prefix('/').unwrap_or(path.as_str())
    )
}
