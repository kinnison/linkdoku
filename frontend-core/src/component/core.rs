//! Properly core components such as the footer, or navbar

use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(Footer)]
pub fn core_page_footer() -> Html {
    html! {
        <footer class={"footer"}>
            <div class="content has-text-centered">
                <p>
                    <strong>{"Linkdoku"}</strong> {" by "} <a href="https://github.com/kinnison">{"Daniel Silverstone"}</a>{". "}
                    <a href="https://github.com/kinnison/linkdoku">{"Linkdoku"}</a> {" is licensed "}
                    <a href="https://www.gnu.org/licenses/#AGPL">{" GNU Affero GPL Version 3"}</a>{"."}
                </p>
            </div>
        </footer>
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
