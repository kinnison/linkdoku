//! Properly core components such as the footer, or navbar

use bounce::helmet::Helmet;
use git_testament::{git_testament, GitModification};
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{use_asset_url, use_page_url, LinkdokuBase, Route};

git_testament!(VERSION);

#[function_component(Footer)]
pub fn core_page_footer() -> Html {
    html! {
        <footer class={"footer"}>
            <div class="content has-text-centered">
                <p>
                    <strong>{"Linkdoku"}</strong> {" by "} <a href="https://github.com/kinnison">{"Daniel Silverstone"}</a>{". "}
                    <a href="https://github.com/kinnison/linkdoku">{"Linkdoku"}</a>
                    <Link<Route> to={Route::VersionInformation}>{format!(" {VERSION}")}</Link<Route>>
                    {" is licensed "}
                    <a href="https://www.gnu.org/licenses/#AGPL">{" GNU Affero GPL Version 3"}</a>{"."}
                </p>
            </div>
        </footer>
    }
}

#[function_component(VersionInfo)]
pub fn version_info_render() -> Html {
    fn field(title: &str, text: impl Into<String>) -> Html {
        html! {
            <div class="field">
                <label class="label">{title}</label>
                <div class="control">
                    <input class="input" type="text" value={text.into()} readonly=true/>
                </div>
            </div>
        }
    }
    fn render_modification(gmod: &GitModification<'_>) -> Html {
        match gmod {
            GitModification::Added(pathb) => html! {
                <>
                    <strong>{"Added: "}</strong>
                    {String::from_utf8_lossy(pathb)}
                    <br />
                </>
            },
            GitModification::Removed(pathb) => html! {
                <>
                    <strong>{"Removed: "}</strong>
                    {String::from_utf8_lossy(pathb)}
                    <br />
                </>
            },
            GitModification::Modified(pathb) => html! {
                <>
                    <strong>{"Modified: "}</strong>
                    {String::from_utf8_lossy(pathb)}
                    <br />
                </>
            },
            GitModification::Untracked(pathb) => html! {
                <>
                    <strong>{"Untracked: "}</strong>
                    {String::from_utf8_lossy(pathb)}
                    <br />
                </>
            },
        }
    }
    html! {
        <>
            {field("Version", format!("{}", VERSION.commit))}
            {if let Some(branch) = VERSION.branch_name { field("Built from branch", branch)}else{html!{}}}
            if !VERSION.modifications.is_empty() {
                <div class="field">
                    <label class="label">{"Modifications"}</label>
                    <div class="control">
                        <pre class="textarea">
                            {for VERSION.modifications.iter().map(render_modification)}
                        </pre>
                    </div>
                </div>
            }
        </>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct NavbarProps {
    pub children: Children,
}

#[function_component(Navbar)]
pub fn core_page_navbar(props: &NavbarProps) -> Html {
    let shortcut_icon = use_state_eq(|| None);

    use_effect({
        let icon = shortcut_icon.setter();
        move || {
            use web_sys::HtmlLinkElement;
            let mut node = gloo::utils::head().first_child();
            while let Some(maybe_link) = node {
                node = maybe_link.next_sibling();
                if let Ok(link) = maybe_link.dyn_into::<HtmlLinkElement>() {
                    if &link.rel() == "icon" {
                        icon.set(Some(link.href()))
                    }
                }
            }
        }
    });

    html! {
        <nav class={"navbar is-dark"} role={"navigation"} aria-label={"main navigation"}>
            <div class={"navbar-brand"}>
                <Link<Route> to={Route::Home} classes={"navbar-item"}>
                    {
                        if let Some(icon) = shortcut_icon.as_ref() {
                           html! {<img src={icon.to_string()} width={"32"} height={"32"} />}
                        } else {
                            html!{}
                        }
                    }
                    {"Linkdoku"}
                </Link<Route>>

                <a role={"button"} class={"navbar-burger"} aria-label={"menu"} aria-expanded={"false"} data-target={"navbarMenu"}>
                    <span aria-hidden={"true"}></span>
                    <span aria-hidden={"true"}></span>
                    <span aria-hidden={"true"}></span>
                </a>
            </div>

            <div id={"navbarMenu"} class={"navbar-menu"}>
                <div class={"navbar-start"}>
                    <Link<Route> to={Route::Home} classes={"navbar-item"}>
                        {"Home"}
                    </Link<Route>>
                </div>

                <div class={"navbar-end"}>
                    { for props.children.iter() }
                    <div class={"navbar-item"}>
                    </div>
                </div>
            </div>
        </nav>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct OpenGraphMetaProps {
    pub title: Option<AttrValue>,
    pub ogtype: Option<AttrValue>,
    pub image: Option<AttrValue>,
    pub width: Option<usize>,
    pub height: Option<usize>,
    pub mimetype: Option<AttrValue>,
    pub url: Option<AttrValue>,
    pub description: AttrValue,
}

#[function_component(OpenGraphMeta)]
pub fn opengraph_meta_render(props: &OpenGraphMetaProps) -> Html {
    let base = use_context::<LinkdokuBase>().unwrap();
    let title = props
        .title
        .clone()
        .unwrap_or_else(|| AttrValue::from("Linkdoku"));
    let ogtype = props
        .ogtype
        .clone()
        .unwrap_or_else(|| AttrValue::from("website"));
    let favicon = use_asset_url(&base.asset_str);
    let image = props
        .image
        .clone()
        .unwrap_or_else(|| AttrValue::from(favicon));
    let width = format!("{}", props.width.unwrap_or(100));
    let height = format!("{}", props.height.unwrap_or(100));
    let mimetype = props
        .mimetype
        .clone()
        .unwrap_or(AttrValue::Static("image/svg+xml"));
    let this_uri = use_page_url();
    let url = props
        .url
        .clone()
        .unwrap_or_else(|| AttrValue::from(this_uri));
    let description = props.description.clone();
    html! {
        <Helmet>
            <meta property="og:title" content={title} />
            <meta property="og:type" content={ogtype} />
            <meta property="og:image" content={image} />
            <meta property="og:image:width" content={width} />
            <meta property="og:image:height" content={height} />
            <meta property="og:image:type" content={mimetype} />
            <meta property="og:url" content={url} />
            <meta property="og:description" content={description} />
        </Helmet>
    }
}
