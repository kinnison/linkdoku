//! Role pages for Linkdoku
//!
//! Currently there are two main pages here, the RolePage and the RoleEditPage

use apiprovider::{use_apiprovider, use_cached_value, CachedValue};
use common::objects;
use components::{layout::MainPageLayout, user::LoginStatus};
use frontend_core::{component::icon::*, Route};
use web_sys::HtmlInputElement;
use yew::{platform::spawn_local, prelude::*};
use yew_markdown::{editor::MarkdownEditor, render::MarkdownRender};
use yew_router::prelude::*;
use yew_toastrack::{use_toaster, Toast, ToastLevel};

use crate::util_components::Title;

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
    let user_info = use_context::<LoginStatus>().unwrap();
    let raw_role = use_cached_value::<objects::Role>(props.role.clone())?;
    let toaster = use_toaster();

    let raw_role = match &raw_role {
        Err(e) => {
            toaster.toast(
                Toast::new(format!("Failure viewing role: {e:?}")).with_level(ToastLevel::Danger),
            );
            return Ok(html! {
                <Redirect<Route> to={Route::Home} />
            });
        }
        Ok(role) => {
            if let Some(role) = role.get() {
                role
            } else {
                toaster.toast(
                    Toast::new(format!("Role not found: {}", props.role))
                        .with_level(ToastLevel::Warning),
                );
                return Ok(html! {
                    <Redirect<Route> to={Route::Home} />
                });
            }
        }
    };

    let can_edit = match &user_info {
        LoginStatus::LoggedIn { uuid, .. } => raw_role.can_edit(uuid),
        _ => false,
    };

    let edit_link = if can_edit {
        html! {
            <Link<Route> to={Route::EditRole{role: props.role.to_string()}}>
                <Icon icon={RoleEditIcon} size={IconSize::Medium} />
            </Link<Route>>
        }
    } else {
        html! {}
    };

    Ok(html! {
        <>
            <Title value={format!("Role - {}", raw_role.display_name)} />
            <h1 class={"title"}>{raw_role.display_name.clone()}{edit_link}</h1>
            <hr width={"40%"} />
            <MarkdownRender markdown={raw_role.description.clone()} />
        </>
    })
}

#[function_component(RoleEditPage)]
pub fn pages_role_edit(props: &RolePageProps) -> Html {
    let fallback = html! {};

    html! {
        <MainPageLayout>
            <Suspense fallback={fallback}>
                <RoleEditPageInner role={props.role.clone()} />
            </Suspense>
        </MainPageLayout>
    }
}

#[function_component(RoleEditPageInner)]
fn role_page_edit_inner(props: &RolePageProps) -> HtmlResult {
    let user_info = use_context::<LoginStatus>().unwrap();
    let raw_role = use_cached_value::<objects::Role>(props.role.clone())?;
    let toaster = use_toaster();
    let display_name_ref = use_node_ref();
    let api = use_apiprovider();

    let raw_role = match &raw_role {
        Err(e) => {
            toaster.toast(
                Toast::new(format!("Failure editing role: {e:?}")).with_level(ToastLevel::Danger),
            );
            return Ok(html! {
                <Redirect<Route> to={Route::Home} />
            });
        }
        Ok(role) => {
            if let Some(role) = role.get() {
                role
            } else {
                toaster.toast(
                    Toast::new(format!("Role not found: {}", props.role))
                        .with_level(ToastLevel::Warning),
                );
                return Ok(html! {
                    <Redirect<Route> to={Route::Home} />
                });
            }
        }
    };

    let can_edit = match &user_info {
        LoginStatus::LoggedIn { uuid, .. } => raw_role.can_edit(uuid),
        LoginStatus::Unknown => true, // Let's just assume we can edit if we don't know, that's nicer for everyone.
        LoginStatus::LoggedOut => false, // Definitely cannot edit if we're logged out.
    };

    if !can_edit {
        toaster.toast(
            Toast::new("Unable to edit role, insufficient permissions")
                .with_level(ToastLevel::Danger)
                .with_lifetime(3000),
        );
        return Ok(html! {
            <Redirect<Route> to={Route::Home} />
        });
    }

    let display_name = use_state_eq(|| raw_role.display_name.clone());
    let description = use_state_eq(|| raw_role.description.clone());
    let button_enabled = use_state_eq(|| true);

    let display_name_changed = Callback::from({
        let setter = display_name.setter();
        let display_name_ref = display_name_ref.clone();
        move |_| {
            let field: HtmlInputElement = display_name_ref.cast().unwrap();
            let value = field.value();
            setter.set(value);
        }
    });

    let display_name_updated = Callback::from({
        let setter = display_name.setter();
        let display_name_ref = display_name_ref.clone();
        move |_| {
            let field: HtmlInputElement = display_name_ref.cast().unwrap();
            let value = field.value();
            setter.set(value);
        }
    });

    let markdown_updated = Callback::from({
        let setter = description.setter();
        move |val: AttrValue| {
            setter.set(val.to_string());
        }
    });

    let on_save_changes = Callback::from({
        let description = description.clone();
        let display_name = display_name.clone();
        let button_setter = button_enabled.setter();
        let uuid = raw_role.uuid.clone();
        //let toaster = toaster.clone();
        move |_| {
            // Disable the button and begin the save operation
            button_setter.set(false);
            let description = (*description).clone();
            let display_name = (*display_name).clone();
            let api = api.clone();
            let uuid = uuid.clone();
            let toaster = toaster.clone();
            let button_setter = button_setter.clone();
            spawn_local(async move {
                match api.update_role(uuid, display_name, description).await {
                    Ok(_) => {
                        // We successfully saved
                        toaster.toast(
                            Toast::new("Successfully saved")
                                .with_level(ToastLevel::Success)
                                .with_lifetime(2000),
                        );
                    }
                    Err(e) => {
                        toaster.toast(
                            Toast::new(format!("Unable to save: {e:?}"))
                                .with_level(ToastLevel::Danger)
                                .with_lifetime(5000),
                        );
                    }
                };
                button_setter.set(true);
            });
        }
    });

    Ok(html! {
        <>
            <Title value={format!("Edit Role - {}", raw_role.display_name)} />
            <div class={"field"}>
                <label class={"label"}>
                    {"Display name"}
                </label>
                <div class={"control"}>
                    <input ref={display_name_ref} class={"input"} type={"text"} placeholder={"Role's Display Name"}
                           value={(*display_name).clone()} onchange={display_name_changed} oninput={display_name_updated}/>
                </div>
            </div>
            <div class={"field"}>
                <label class={"label"}>
                    {"Description (Markdown)"}
                </label>
                <div class={"control"}>
                    <MarkdownEditor initial={(*description).clone()} onchange={markdown_updated}/>
                </div>
            </div>
            <div class={"field is-grouped"}>
                <div class="control">
                    <button class="button is-primary" disabled={!*button_enabled} onclick={on_save_changes}>
                        <span class={"icon-text"}>
                            <Icon icon={if *button_enabled { SubmitFormIcon } else { SpinnerIcon }}/> <span>{if *button_enabled { "Save changes" } else { "Saving changes" }}</span>
                        </span>
                    </button>
                </div>
            </div>
        </>
    })
}
