//! Role pages for Linkdoku
//!
//! Currently there are two main pages here, the RolePage and the RoleEditPage

use apiprovider::{use_apiprovider, use_cached_value, use_cached_value_by_name};
use common::objects;
use components::{layout::MainPageLayout, puzzle::PuzzleList, user::LoginStatus};
use frontend_core::{
    component::{icon::*, utility::*},
    use_route_url, Route, ShortcutRoute,
};
use tutorials::{
    tutorial, use_tutorial_node, TutorialAnchor, TutorialAnchorPosition::TutorialTop,
    TutorialController, TutorialData,
};
use web_sys::HtmlInputElement;
use yew::{platform::spawn_local, prelude::*};
use yew_markdown::{editor::MarkdownEditor, render::MarkdownRender};
use yew_router::prelude::*;
use yew_toastrack::{use_toaster, Toast, ToastLevel};

use crate::{
    help_texts::ROLE_DESCRIPTION, routes::core_frontend_route_switch, util_components::Title,
};

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

tutorial! {
    ViewRoleTutorial,
    name: "This is the name of the role, for example it might be the name of the person who set these puzzles.",
    description: "Information about this role is shown here",
    puzzle_list: "The puzzles that this role has created can be found here",
    permalink: "You can click here to copy a link to this page which won't change",
    shortcutlink: "You can click here to copy a nicer shortcut link, but this might change",
    edit: "You can click here to edit this role"
}

#[function_component(RolePageInner)]
fn pages_role_render_inner(props: &RolePageProps) -> HtmlResult {
    let user_info = use_context::<LoginStatus>().unwrap();
    let raw_role = use_cached_value::<objects::Role>(props.role.clone())?;
    let toaster = use_toaster();

    let raw_role = match raw_role.as_ref() {
        Err(e) => {
            toaster.toast(
                Toast::new(format!("Failure viewing role: {e:?}")).with_level(ToastLevel::Danger),
            );
            return Ok(html! {
                <Redirect<Route> to={Route::Home} />
            });
        }
        Ok(role) => role,
    };

    let can_edit = match &user_info {
        LoginStatus::LoggedIn { uuid, .. } => raw_role.can_edit(uuid),
        _ => false,
    };

    let mut tutorial = ViewRoleTutorial::default();

    let edit_tutorial_node = use_node_ref();
    let edit_link = if can_edit {
        tutorial.edit(edit_tutorial_node.clone());
        html! {
            <TutorialAnchor noderef={edit_tutorial_node}>
                <Link<Route> to={Route::EditRole{role: props.role.to_string()}}>
                    <Tooltip content={"Edit role"} alignment={TooltipAlignment::Bottom}>
                        <Icon icon={RoleEditIcon} size={IconSize::Medium} />
                    </Tooltip>
                </Link<Route>>
            </TutorialAnchor>
        }
    } else {
        html! {}
    };

    let perma_link_node = use_tutorial_node!(tutorial.permalink);

    let perma_link = {
        let permalink = Route::ViewRole {
            role: raw_role.uuid.clone(),
        };
        let permalink = use_route_url(&permalink);
        html! {
            <TutorialAnchor noderef={perma_link_node}>
                <Tooltip content={"Copy permalink to role"} alignment={TooltipAlignment::Bottom}>
                    <CopyButton content={permalink} size={IconSize::Medium}/>
                </Tooltip>
            </TutorialAnchor>
        }
    };

    let shortcut_link_node = use_tutorial_node!(tutorial.shortcutlink);

    let shortcut_link = {
        let shortlink = ShortcutRoute::RoleShortcut {
            role: raw_role.short_name.clone(),
        };
        let shortlink = use_route_url(&shortlink);
        html! {
            <TutorialAnchor noderef={shortcut_link_node}>
                <Tooltip content={"Copy shortcut to role"} alignment={TooltipAlignment::Bottom}>
                    <CopyButton content={shortlink} icon={RoleNiceLinkIcon} size={IconSize::Medium}/>
                </Tooltip>
            </TutorialAnchor>
        }
    };

    let name_node = use_tutorial_node!(tutorial.name);
    let description_node = use_tutorial_node!(tutorial.description);
    let puzzle_list_node = use_tutorial_node!(tutorial.puzzle_list);

    Ok(html! {
        <>
            <TutorialController tutorial={TutorialData::from(tutorial)} />
            <Title value={format!("{} - Role", raw_role.display_name)} />
            <h1 class={"title"}>
                <TutorialAnchor noderef={name_node}>
                    {format!("{} ({}) ", raw_role.display_name, raw_role.short_name)}
                </TutorialAnchor>
                {" "}
                {perma_link}{shortcut_link}{edit_link}
            </h1>
            <hr width={"40%"} />
            <TutorialAnchor noderef={description_node} position={TutorialTop} class="is-block">
                <MarkdownRender markdown={raw_role.description.clone()} />
            </TutorialAnchor>
            <hr width={"40%"} />
            <TutorialAnchor noderef={puzzle_list_node} position={TutorialTop} class="is-block">
                <PuzzleList role={raw_role.uuid.clone()} />
            </TutorialAnchor>
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

tutorial!(
    EditRoleTutorial,
    short_name: "The short name is used in shortcut URLs and is shown on the view-role page",
    display_name: "The display name is used on the view-role page and on puzzle pages",
    description: "You can write in a description of your role here.  This could be a short bio and may contain links to other websites"
);

#[function_component(RoleEditPageInner)]
fn role_page_edit_inner(props: &RolePageProps) -> HtmlResult {
    let user_info = use_context::<LoginStatus>().unwrap();
    let cached_role = use_cached_value::<objects::Role>(props.role.clone())?;
    let toaster = use_toaster();
    let short_name_ref = use_node_ref();
    let display_name_ref = use_node_ref();
    let api = use_apiprovider();
    let mut tutorial = EditRoleTutorial::default();

    let raw_role = match cached_role.as_ref() {
        Err(e) => {
            toaster.toast(
                Toast::new(format!("Failure editing role: {e:?}")).with_level(ToastLevel::Danger),
            );
            return Ok(html! {
                <Redirect<Route> to={Route::Home} />
            });
        }
        Ok(role) => role,
    };

    let can_edit = match &user_info {
        LoginStatus::LoggedIn { uuid, .. } => raw_role.can_edit(uuid),
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

    let short_name = use_state_eq(|| raw_role.short_name.clone());
    let display_name = use_state_eq(|| raw_role.display_name.clone());
    let description = use_state_eq(|| raw_role.description.clone());
    let button_enabled = use_state_eq(|| true);

    let short_name_changed = Callback::from({
        let setter = short_name.setter();
        let short_name_ref = short_name_ref.clone();
        move |_| {
            let field: HtmlInputElement = short_name_ref.cast().unwrap();
            let value = field.value();
            setter.set(value);
        }
    });

    let short_name_updated = Callback::from({
        let setter = short_name.setter();
        let short_name_ref = short_name_ref.clone();
        move |_| {
            let field: HtmlInputElement = short_name_ref.cast().unwrap();
            let value = field.value();
            setter.set(value);
        }
    });

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
        let short_name = short_name.clone();
        let display_name = display_name.clone();
        let button_setter = button_enabled.setter();
        let uuid = raw_role.uuid.clone();
        let cached_role = cached_role.clone();
        //let toaster = toaster.clone();
        move |_| {
            // Disable the button and begin the save operation
            button_setter.set(false);
            let description = (*description).clone();
            let short_name = (*short_name).clone();
            let display_name = (*display_name).clone();
            let api = api.clone();
            let uuid = uuid.clone();
            let toaster = toaster.clone();
            let button_setter = button_setter.clone();
            let cached_role = cached_role.clone();
            spawn_local(async move {
                match api
                    .update_role(&uuid, short_name, display_name, description)
                    .await
                {
                    Ok(role) => {
                        cached_role.refresh(&uuid, role);
                        // We successfully saved
                        toaster.toast(
                            Toast::new("Successfully saved")
                                .with_level(ToastLevel::Success)
                                .with_lifetime(2000),
                        );
                    }
                    Err(e) => {
                        toaster.toast(
                            Toast::new(format!("Unable to save: {e}"))
                                .with_level(ToastLevel::Danger)
                                .with_lifetime(5000),
                        );
                    }
                };
                button_setter.set(true);
            });
        }
    });

    let short_name_tutorial = use_tutorial_node!(tutorial.short_name);
    let display_name_tutorial = use_tutorial_node!(tutorial.display_name);
    let description_tutorial = use_tutorial_node!(tutorial.description);

    Ok(html! {
        <>
            <Title value={format!("Edit Role - {}", raw_role.display_name)} />
            <div class="field">
                <label class={"label"}>
                    {"Short name"}
                </label>
                <div class="control">
                    <TutorialAnchor noderef={short_name_tutorial} class="is-block">
                        <input ref={short_name_ref} class={"input"} type={"text"} placeholder={"Role's Short Name"}
                               value={(*short_name).clone()} onchange={short_name_changed} oninput={short_name_updated}/>
                    </TutorialAnchor>
                 </div>
            </div>
            <div class={"field"}>
                <label class={"label"}>
                    {"Display name"}
                </label>
                <div class={"control"}>
                    <TutorialAnchor noderef={display_name_tutorial} class="is-block">
                        <input ref={display_name_ref} class={"input"} type={"text"} placeholder={"Role's Display Name"}
                               value={(*display_name).clone()} onchange={display_name_changed} oninput={display_name_updated}/>
                    </TutorialAnchor>
                </div>
            </div>
            <div class={"field"}>
                <label class={"label"}>
                    {"Description (Markdown)"}
                </label>
                <div class={"control"}>
                    <TutorialAnchor noderef={description_tutorial} position={TutorialTop} class="is-block">
                        <MarkdownEditor initial={(*description).clone()} onchange={markdown_updated} help={ROLE_DESCRIPTION}/>
                    </TutorialAnchor>
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
            <TutorialController tutorial={TutorialData::from(tutorial)} />
        </>
    })
}

#[derive(Properties, PartialEq)]
pub struct FindRoleAndRedirectProps {
    pub name: AttrValue,
}

#[function_component(FindRoleAndRedirect)]
pub fn find_role_and_redirect(props: &FindRoleAndRedirectProps) -> Html {
    let fallback = html! { {"Please wait…"} };
    html! {
        <Suspense fallback={fallback}>
            <FindRoleAndRedirectInner name={props.name.clone()} />
        </Suspense>
    }
}

#[function_component(FindRoleAndRedirectInner)]
fn find_role_and_redirect_inner(props: &FindRoleAndRedirectProps) -> HtmlResult {
    let raw_role = use_cached_value_by_name::<objects::Role>(props.name.clone())?;
    let toaster = use_toaster();

    let raw_role = match raw_role.as_ref() {
        Err(e) => {
            toaster.toast(
                Toast::new(format!("Failure viewing role: {e:?}")).with_level(ToastLevel::Danger),
            );
            return Ok(html! {
                <Redirect<Route> to={Route::Home} />
            });
        }
        Ok(role) => role,
    };

    // We now "sub-render" as though our role route was here
    Ok(core_frontend_route_switch(Route::ViewRole {
        role: raw_role.uuid.clone(),
    }))
}
