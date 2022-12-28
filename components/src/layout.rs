//! Main layout components
//!

use frontend_core::{component::icon::*, Route};
use yew::prelude::*;
use yew_router::prelude::*;

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

    html! {
        <div class={"panel"}>
            <p class="panel-heading">
                {"Activities"}
            </p>
            {blocks}
        </div>
    }
}
