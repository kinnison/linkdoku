//! Main layout components
//!

use apiprovider::use_apiprovider;
use common::public::scaffold::{self, hash_version_info};
use frontend_core::{component::icon::*, Route};
use futures_util::stream::StreamExt;
use git_testament::git_testament;
use gloo::timers::future::IntervalStream;
use yew::{platform::spawn_local, prelude::*};
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
        LoginStatus::Unknown => {}
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
        }
    };

    blocks.push(html! {
        <Link<Route> to={Route::CreatePuzzle} classes={"panel-block"}>
            <Icon class={"panel-icon"} icon={PuzzleAddIcon}/>
            {"Create Puzzle"}
        </Link<Route>>
    });

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
            let toaster = toaster.clone();
            move |_| {
                spawn_local(async move {
                    IntervalStream::new(30_000)
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
            toaster.toast(Toast::new("Backend has updated").with_level(ToastLevel::Danger));
        }
    }

    html! {}
}
