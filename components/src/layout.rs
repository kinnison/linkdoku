//! Main layout components
//!

use std::time::Duration;

use apiprovider::use_apiprovider;
use bounce::helmet::Helmet;
use common::public::scaffold::{self, hash_version_info};
use frontend_core::{component::icon::*, Route};
use futures_util::stream::StreamExt;
use git_testament::git_testament;
use gloo::timers::future::IntervalStream;
use yew::{platform::spawn_local, prelude::*};
use yew_markdown::render::MarkdownRender;
use yew_router::prelude::*;
use yew_toastrack::{use_toaster, Toast, ToastLevel};

use crate::{
    role::Role,
    user::{LoginButton, LoginStatus},
};

#[derive(Properties, PartialEq, Clone)]
pub struct MainLayoutProps {
    pub children: Children,
}

#[function_component(MainPageLayout)]
pub fn main_page_layout_render(props: &MainLayoutProps) -> Html {
    html! {
        <div class={"columns"}>
            <div class={"column is-narrow"}>
                <MainMenu />
            </div>
            <div class={"column"}>
                { for props.children.iter() }
            </div>
        </div>
    }
}

#[function_component(MainMenu)]
fn main_menu_render() -> Html {
    let user_info = use_context::<LoginStatus>().unwrap();
    let nav = use_navigator().unwrap();

    let mut blocks = vec![html! {
        <Link<Route> to={Route::Home} classes={"panel-block"}>
            <Icon class={"panel-icon"} icon={InternalLinkIcon}/>
            {"Home"}
        </Link<Route>>
    }];

    match user_info {
        LoginStatus::LoggedOut => {
            blocks.push(html! {
                <div class={"panel-block"}>
                    <LoginButton />
                </div>
            });
        }
        LoginStatus::LoggedIn { roles, role, .. } => {
            for (i, r) in Some(role.clone())
                .into_iter()
                .chain(roles.into_iter().filter(|r| r != &role))
                .enumerate()
            {
                let view_role = Callback::from({
                    let target = Route::ViewRole { role: r.clone() };
                    let nav = nav.clone();
                    move |_| nav.push(&target)
                });
                blocks.push(html! {
                    <Role uuid={r.clone()} active={i == 0} onclick={view_role}/>
                });
            }

            blocks.push(html! {
                <Link<Route> to={Route::CreatePuzzle} classes={"panel-block"}>
                    <Icon class={"panel-icon"} icon={PuzzleAddIcon}/>
                    {"Create Puzzle"}
                </Link<Route>>
            });
        }
    };

    html! {
        <div class={"panel"}>
            <p class="panel-heading">
                {"Activities"}
            </p>
            {blocks}
        </div>
    }
}

git_testament!(VERSION);

#[function_component(VersionChecker)]
pub fn version_checker_render() -> Html {
    let my_hash = hash_version_info(&VERSION);
    let api = use_apiprovider();
    let toaster = use_toaster();
    let found_data = use_state_eq(|| None::<scaffold::Response>);

    use_effect_with_deps(
        {
            let setter = found_data.setter();
            move |_| {
                spawn_local(async move {
                    IntervalStream::new(Duration::from_secs(60 * 30).as_millis() as u32)
                        .for_each(|_| async {
                            match api.get_scaffold().await {
                                Ok(scaf) => {
                                    setter.set(Some(scaf));
                                }
                                Err(e) => {
                                    toaster.toast(
                                        Toast::new(format!(
                                            "Failure retrieving backend status: {e}"
                                        ))
                                        .with_level(ToastLevel::Warning)
                                        .with_lifetime(2500),
                                    );
                                }
                            }
                        })
                        .await;
                });
                move || ()
            }
        },
        (),
    );

    if let Some(scaf) = found_data.as_ref() {
        if scaf.version_hash != my_hash {
            let reloader = Callback::from(|_| {
                gloo::utils::window().location().reload().unwrap();
            });
            html! {
                <>
                    <Helmet>
                        <html class="is-clipped" />
                    </Helmet>
                    <div class="modal is-active">
                        <div class="modal-background"></div>
                        <div class="modal-card">
                            <header class="modal-card-head">
                                <p class="modal-card-title">{"Backend version change detected"}</p>
                            </header>
                            <section class="modal-card-body">
                                <div class="content">
                                    <div class="field">
                                        <label class="label">{"Frontend version"}</label>
                                        <div class="control">
                                            <input class="input" type="text" value={format!("{VERSION}")} readonly=true />
                                        </div>
                                    </div>
                                    <div class="field">
                                        <label class="label">{"Backend version"}</label>
                                        <div class="control">
                                            <input class="input" type="text" value={scaf.version.clone()} readonly=true />
                                        </div>
                                    </div>
                                </div>
                            </section>
                            <footer class="modal-card-foot">
                                <button class="button is-success" onclick={reloader}>{"Reload page"}</button>
                            </footer>
                        </div>
                    </div>
                </>
            }
        } else {
            html! {}
        }
    } else {
        html! {}
    }
}

#[derive(Properties, PartialEq)]
pub struct ModalMarkdownProps {
    pub title: AttrValue,
    pub markdown: AttrValue,
    pub buttons: Option<&'static [&'static str]>,
    pub default_button: Option<usize>,
    pub action: Callback<usize>,
}

#[function_component(ModalMarkdown)]
pub fn modal_markdown_render(props: &ModalMarkdownProps) -> Html {
    let default_idx = props
        .default_button
        .unwrap_or(0)
        .clamp(0, props.buttons.map(|b| b.len() - 1).unwrap_or(0));
    let buttons = props
        .buttons
        .unwrap_or(&["Dismiss"])
        .iter()
        .enumerate()
        .map(|(i, button)| {
            let button_class = if i == default_idx {
                "button is-success"
            } else {
                "button"
            };
            let button_cb = props.action.clone();
            let this_cb = Callback::from(move |_| button_cb.emit(i));
            html! {
                <button class={button_class} onclick={this_cb}>{button}</button>
            }
        });
    html! {
            <>
                <Helmet>
                    <html class="is-clipped" />
                </Helmet>
                <div class="modal is-active">
                <div class="modal-background"></div>
                <div class="modal-card">
                    <header class="modal-card-head">
                        <p class="modal-card-title">{&props.title}</p>
                    </header>
                    <section class="modal-card-body">
                        <MarkdownRender markdown={&props.markdown} />
                    </section>
                    <footer class="modal-card-foot">
                        {for buttons}
                    </footer>
                </div>
            </div>
    </>
        }
}
