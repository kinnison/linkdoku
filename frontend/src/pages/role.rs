//! Role pages for Linkdoku
//!
//! Currently there are two main pages here, the RolePage and the RoleEditPage

use apiprovider::{use_cached_value, CachedValue};
use common::objects;
use components::{layout::MainPageLayout, user::LoginStatus};
use frontend_core::Route;
use yew::prelude::*;
use yew_markdown::render::MarkdownRender;
use yew_router::prelude::*;
use yew_toastrack::{use_toaster, Toast, ToastLevel};

#[derive(Properties, PartialEq, Clone)]
pub struct RolePageProps {
    pub role: AttrValue,
}

#[function_component(RolePage)]
pub fn pages_role_render(props: &RolePageProps) -> Html {
    let fallback = html! {};

    html! {
        <MainPageLayout>
            <Suspense fallback={fallback}>
                <RolePageInner role={props.role.clone()} />
            </Suspense>
        </MainPageLayout>
    }
}

#[function_component(RolePageInner)]
fn pages_role_render_inner(props: &RolePageProps) -> HtmlResult {
    let raw_role = use_cached_value::<objects::Role>(props.role.clone())?;
    let toaster = use_toaster();

    let raw_role = match raw_role {
        CachedValue::Value(v) => v,
        CachedValue::Error(e) => {
            toaster.toast(
                Toast::new(format!("Failure viewing role: {e:?}")).with_level(ToastLevel::Danger),
            );
            return Ok(html! {
                <Redirect<Route> to={Route::Home} />
            });
        }
        CachedValue::Missing => {
            toaster.toast(
                Toast::new(format!("Role not found: {}", props.role))
                    .with_level(ToastLevel::Warning),
            );
            return Ok(html! {
                <Redirect<Route> to={Route::Home} />
            });
        }
    };

    Ok(html! {
        <>
            <h1 class={"title"}>{raw_role.display_name.clone()}</h1>
            <hr width={"40%"} />
            <MarkdownRender markdown={raw_role.description} />
        </>
    })
}

#[function_component(RoleEditPage)]
pub fn pages_role_edit(props: &RolePageProps) -> Html {
    todo!()
}
