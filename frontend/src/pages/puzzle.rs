use apiprovider::{use_apiprovider, use_cached_value, use_puzzle_lookup};
use common::{
    clean_short_name,
    objects::{self, PuzzleData, PuzzleState, Visibility},
    public::puzzle,
};
use components::{layout::MainPageLayout, role::Role, user::LoginStatus};
use frontend_core::{
    component::{core::OpenGraphMeta, icon::*, utility::*},
    use_route_url, Route, ShortcutRoute,
};
use puzzleutils::{fpuzzles, xform::transform_markdown};
use serde_json::Value;
use stylist::yew::{styled_component, use_style};
use tracing::info;
use web_sys::HtmlInputElement;
use yew::{platform::spawn_local, prelude::*, virtual_dom::VChild};
use yew_bulma_tabs::*;
use yew_markdown::{editor::MarkdownEditor, render::MarkdownRender, xform::Transformer};
use yew_paginator::Paginator;
use yew_router::prelude::*;
use yew_toastrack::{use_toaster, Toast, ToastLevel};

use crate::{routes::core_frontend_route_switch, util_components::Title};

// Shortcut redirector
//
#[derive(Properties, PartialEq)]
pub struct FindPuzzleAndRedirectProps {
    pub role: AttrValue,
    pub puzzle: AttrValue,
}

#[function_component(FindPuzzleAndRedirect)]
pub fn find_role_and_redirect(props: &FindPuzzleAndRedirectProps) -> Html {
    let fallback = html! { {"Please wait…"} };
    html! {
        <Suspense fallback={fallback}>
            <FindPuzzleAndRedirectInner role={props.role.clone()} puzzle={props.puzzle.clone()} />
        </Suspense>
    }
}

#[function_component(FindPuzzleAndRedirectInner)]
fn find_role_and_redirect_inner(props: &FindPuzzleAndRedirectProps) -> HtmlResult {
    let uuid = use_puzzle_lookup(props.role.clone(), props.puzzle.clone())?;
    let toaster = use_toaster();

    let uuid = match uuid.as_ref() {
        Err(e) => {
            toaster.toast(
                Toast::new(format!("Failure looking up puzzle: {e:?}"))
                    .with_level(ToastLevel::Danger),
            );
            return Ok(html! {
                <Redirect<Route> to={Route::Home} />
            });
        }
        Ok(uuid) => uuid,
    };

    // We now "sub-render" as though our puzzle route was here
    Ok(core_frontend_route_switch(Route::ViewPuzzle {
        puzzle: uuid.to_string(),
    }))
}

// Viewers

#[derive(Properties, PartialEq)]
pub struct PuzzlePageProps {
    pub puzzle: AttrValue,
}

#[function_component(PuzzlePage)]
pub fn view_puzzle(props: &PuzzlePageProps) -> Html {
    let fallback = html! {};

    html! {
        <MainPageLayout>
            <Suspense fallback={fallback}>
                <PuzzlePageInner puzzle={props.puzzle.clone()} />
            </Suspense>
        </MainPageLayout>
    }
}

#[derive(PartialEq, Clone, Copy)]
enum ViewPuzzleState {
    Viewing,
    EditMetadata,
    AddingState,
    EditingState,
}

#[function_component(PuzzlePageInner)]
fn view_puzzle_inner(props: &PuzzlePageProps) -> HtmlResult {
    let user_info = use_context::<LoginStatus>().unwrap();
    let puzzle_data = use_cached_value::<objects::Puzzle>(props.puzzle.clone())?;
    let toaster = use_toaster();

    let puzzle = match puzzle_data.as_ref() {
        Err(e) => {
            toaster.toast(
                Toast::new(format!("Failure viewing puzzle: {e:?}")).with_level(ToastLevel::Danger),
            );
            return Ok(html! {
                <Redirect<Route> to={Route::Home} />
            });
        }
        Ok(puzzle) => {
            if let Some(puzzle) = puzzle.get() {
                puzzle
            } else {
                toaster.toast(
                    Toast::new(format!("Puzzle not found: {}", props.puzzle))
                        .with_level(ToastLevel::Warning),
                );
                return Ok(html! {
                    <Redirect<Route> to={Route::Home} />
                });
            }
        }
    };

    let role_data = use_cached_value::<objects::Role>(puzzle.owner.clone().into())?;
    let role = match role_data.as_ref() {
        Err(e) => {
            toaster.toast(
                Toast::new(format!("Failure finding owning role: {e:?}"))
                    .with_level(ToastLevel::Danger),
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
                    Toast::new(format!("Owing role not found: {}", puzzle.owner))
                        .with_level(ToastLevel::Warning),
                );
                return Ok(html! {
                    <Redirect<Route> to={Route::Home} />
                });
            }
        }
    };

    let can_edit = match user_info {
        LoginStatus::LoggedIn { roles, .. } => roles.contains(&puzzle.owner),
        _ => false,
    };

    let perma_link = {
        let permalink = Route::ViewPuzzle {
            puzzle: puzzle.uuid.clone(),
        };
        let permalink = use_route_url(&permalink);
        html! {
            <Tooltip content={"Copy permalink to puzzle"} alignment={TooltipAlignment::Bottom}>
                <CopyButton content={permalink} size={IconSize::Medium}/>
            </Tooltip>
        }
    };

    let short_url = {
        let shortlink = ShortcutRoute::PuzzleShortcut {
            role: role.short_name.clone(),
            puzzle: puzzle.short_name.clone(),
        };
        use_route_url(&shortlink)
    };

    let shortcut_link = {
        html! {
            <Tooltip content={"Copy shortcut to puzzle"} alignment={TooltipAlignment::Bottom}>
                <CopyButton content={short_url.clone()} icon={PuzzleNiceLinkIcon} size={IconSize::Medium}/>
            </Tooltip>
        }
    };

    let display_index = use_state_eq(|| {
        puzzle
            .states
            .iter()
            .enumerate()
            .skip(1)
            .fold(
                (0, puzzle.states[0].visibility),
                |(best_index, best_vis), (idx, state)| {
                    use Visibility::*;
                    match (best_vis, state.visibility) {
                        (Public, Restricted) | (Published, Restricted) | (Published, Public) => {
                            (best_index, best_vis)
                        }
                        _ => (idx, state.visibility),
                    }
                },
            )
            .0
    });

    let set_index = Callback::from({
        let setter = display_index.setter();
        move |n| setter.set(n)
    });

    let display_state = &puzzle.states[*display_index];

    let transformer = Transformer::from({
        let state = display_state.clone();
        move |req| transform_markdown(&state, req)
    });

    let image = match &display_state.data {
        PuzzleData::FPuzzles(data) => Some(fpuzzles::grid_url(data)),
        _ => None,
    };

    let ogtags = html! {
        <OpenGraphMeta
            title={puzzle.display_name.clone()}
            image={image}
            url={short_url}
            description={format!("{} by {}", puzzle.display_name, role.display_name)}
        />
    };

    let state = use_state_eq(|| ViewPuzzleState::Viewing);

    let edit_metadata_form = {
        let short_name_ref = use_node_ref();
        let display_name_ref = use_node_ref();
        let editor_short_name = use_state_eq(|| puzzle.short_name.clone());
        let editor_display_name = use_state_eq(|| puzzle.display_name.clone());
        let button_enabled = use_state_eq(|| true);
        let data_good = use_state_eq(|| true);

        let short_name_changed = Callback::from({
            let setter = editor_short_name.setter();
            let noderef = short_name_ref.clone();
            let button = data_good.setter();
            move |_| {
                let input: HtmlInputElement = noderef.cast().unwrap();
                let value = input.value();
                button.set(!value.is_empty());
                setter.set(value)
            }
        });

        let short_name_input = Callback::from({
            let setter = editor_short_name.setter();
            let noderef = short_name_ref.clone();
            let button = data_good.setter();
            move |_| {
                let input: HtmlInputElement = noderef.cast().unwrap();
                let value = input.value();
                button.set(!value.is_empty());
                setter.set(value)
            }
        });

        let display_name_changed = Callback::from({
            let setter = editor_display_name.setter();
            let noderef = display_name_ref.clone();
            let button = data_good.setter();
            move |_| {
                let input: HtmlInputElement = noderef.cast().unwrap();
                let value = input.value();
                button.set(!value.is_empty());
                setter.set(value)
            }
        });

        let display_name_input = Callback::from({
            let setter = editor_display_name.setter();
            let noderef = display_name_ref.clone();
            let button = data_good.setter();
            move |_| {
                let input: HtmlInputElement = noderef.cast().unwrap();
                let value = input.value();
                button.set(!value.is_empty());
                setter.set(value)
            }
        });

        let on_save_changes = Callback::from({
            let view_state = state.setter();
            let short_name = editor_short_name.clone();
            let display_name = editor_display_name.clone();
            let puzzle = puzzle.uuid.clone();
            let button = button_enabled.setter();
            let api = use_apiprovider();
            let toaster = toaster.clone();
            let puzzle_data = puzzle_data.clone();
            move |_| {
                let sn = short_name.as_str().to_string();
                let dn = display_name.as_str().to_string();
                let uuid = puzzle.clone();
                let api = api.clone();
                let button = button.clone();
                let toaster = toaster.clone();
                let puzzle_data = puzzle_data.clone();
                let view_state = view_state.clone();
                button.set(false);
                spawn_local(async move {
                    match api.update_puzzle_metadata(uuid, sn, dn).await {
                        Ok(_) => {
                            // Puzzle successfully updated, so refresh the local cache
                            if let Err(e) = puzzle_data.refresh().await {
                                toaster.toast(
                                    Toast::new(format!(
                                        "Updated puzzle, but failed to refresh cache: {e}"
                                    ))
                                    .with_level(ToastLevel::Warning)
                                    .with_lifetime(2500),
                                );
                            }
                            view_state.set(ViewPuzzleState::Viewing);
                        }
                        Err(e) => {
                            toaster.toast(
                                Toast::new(format!("Unable to update puzzle metadata: {e}"))
                                    .with_level(ToastLevel::Warning)
                                    .with_lifetime(2500),
                            );
                        }
                    }

                    button.set(true);
                });
            }
        });

        let cancel_onclick = Callback::from({
            let state = state.clone();
            move |_| state.set(ViewPuzzleState::Viewing)
        });

        html! {
            <>
                <div class="field">
                    <label class="label">{"Short name"}</label>
                    <div class="control">
                        <input ref={short_name_ref} class="input" placeholder="Short name for puzzle" value={editor_short_name.as_str().to_string()} onchange={short_name_changed} oninput={short_name_input} />
                    </div>
                </div>
                <div class="field">
                    <label class="label">{"Display name"}</label>
                    <div class="control">
                        <input ref={display_name_ref} class="input" placeholder="Display name for puzzle" value={editor_display_name.as_str().to_string()} onchange={display_name_changed} oninput={display_name_input} />
                    </div>
                </div>
                <div class="field is-grouped">
                    <div class="control">
                        <button class="button is-primary" disabled={!(*button_enabled && *data_good)} onclick={if *button_enabled && *data_good { Some(on_save_changes) } else { None }}>
                            <span class={"icon-text"}>
                                <Icon icon={if *button_enabled { SubmitFormIcon } else { SpinnerIcon }}/>
                                <span>{if *button_enabled { if *data_good { "Save changes" } else { "Please fill out both fields"} } else { "Saving changes" }}</span>
                            </span>
                        </button>
                    </div>
                    <div class="control">
                        <button class="button is-danger" onclick={cancel_onclick}>
                            <span class="icon-text">
                                <Icon icon={CancelIcon} />
                                <span>{"Cancel edit"}</span>
                            </span>
                        </button>
                    </div>
                </div>
            </>
        }
    };

    let metadata_edit = if can_edit {
        let edit_puzzle_click = Callback::from({
            let viewstate_setter = state.setter();
            move |_| {
                viewstate_setter.set(ViewPuzzleState::EditMetadata);
            }
        });
        html! {
            <Tooltip content={"Edit puzzle metadata"} alignment={TooltipAlignment::Bottom}>
                <span class="has-text-link">
                    <Icon icon={PuzzleEditMetadataIcon} onclick={edit_puzzle_click} size={IconSize::Medium} />
                </span>
            </Tooltip>
        }
    } else {
        html! {}
    };

    let page_body = match *state {
        ViewPuzzleState::Viewing => {
            html! {
                <>
                    {ogtags}
                    <Title value={puzzle.display_name.clone()} />
                    <h1 class="title">{format!("{} ({})", puzzle.display_name, puzzle.short_name)}{perma_link}{shortcut_link}{metadata_edit}</h1>
                    <hr width={"40%"} />
                    <MarkdownRender markdown={display_state.description.clone()} transformer={transformer}/>
                    <hr width={"40%"} />
                    <Paginator count={puzzle.states.len()} current={*display_index} aria_label={"Puzzle State"} element={"puzzle state"} onchange={set_index} />
                </>
            }
        }
        ViewPuzzleState::EditMetadata => {
            html! {
                <>
                    <Title value={format!("Editing - {}", puzzle.display_name)} />
                    <h1 class="title">{format!("Editing puzzle…")}</h1>
                    <h1 width={"40%"} />
                    {edit_metadata_form}
                </>
            }
        }
        ViewPuzzleState::EditingState => todo!(),
        ViewPuzzleState::AddingState => todo!(),
    };

    Ok(page_body)
}

// Editors

const DEFAULT_FPUZZLES_DESCRIPTION: &str = r"
## Rules

[rules]

## Grid preview

![grid]

## Play this puzzle

* [fpuzzles]
* [sudokupad]
* [beta-sudokupad]
";

#[function_component(CreatePuzzlePage)]
pub fn create_puzzle_page_render() -> Html {
    let nav = use_navigator().unwrap();
    let loc = use_location().unwrap();
    let user_info = use_context::<LoginStatus>().unwrap();
    let toaster = use_toaster();
    let api = use_apiprovider();

    let state = loc.state();

    let state = state.unwrap_or_else(|| {
        puzzle::create::Request {
            owner: user_info.current_role().unwrap_or("").to_string(),
            display_name: "".to_string(),
            short_name: "".to_string(),
            initial_state: PuzzleState {
                description: "".to_string(),
                visibility: Visibility::Restricted,
                updated_at: "".to_string(),
                data: PuzzleData::FPuzzles(Value::Null),
            },
        }
        .into()
    });

    let mut fields = vec![];

    // Owner (dropdown of available roles)
    {
        let set_owner = Callback::from({
            let nav = nav.clone();
            let state = state.clone();
            move |uuid| {
                let mut state = (*state).clone();
                state.owner = uuid;
                nav.replace_with_state(&Route::CreatePuzzle, state);
            }
        });

        let roles = user_info.roles().iter().map(|uuid| {
            let onclick = Callback::from({
                let uuid = uuid.clone();
                let set_owner = set_owner.clone();
                move |_| set_owner.emit(uuid.clone())
            });
            html! {
                <Role uuid={uuid.clone()} active={uuid == &state.owner} onclick={onclick}/>
            }
        });

        fields.push(html! {
            <div class="field">
                <label class="label">{"Owning role"}</label>
                {for roles}
            </div>
        });
    }

    // Short name
    {
        let input_ref = use_node_ref();
        let set_shortname = Callback::from({
            let nav = nav.clone();
            let state = state.clone();
            move |short_name| {
                let mut state = (*state).clone();
                state.short_name = short_name;
                nav.replace_with_state(&Route::CreatePuzzle, state);
            }
        });
        let onchange = Callback::from({
            let input_ref = input_ref.clone();
            let set_shortname = set_shortname.clone();
            move |_| {
                let input: HtmlInputElement = input_ref.cast().unwrap();
                set_shortname.emit(input.value());
            }
        });

        let oninput = Callback::from({
            let input_ref = input_ref.clone();
            move |_| {
                let input: HtmlInputElement = input_ref.cast().unwrap();
                set_shortname.emit(input.value());
            }
        });

        fields.push(html! {
            <div class="field">
                <label class="label">{"Puzzle short-name"}</label>
                <div class="control">
                    <input ref={input_ref} class="input" type="text" placeholder="Puzzle short name" onchange={onchange} oninput={oninput} value={state.short_name.clone()}/>
                </div>
            </div>
        });
    }

    // Display name
    {
        let input_ref = use_node_ref();
        let set_displayname = Callback::from({
            let nav = nav.clone();
            let state = state.clone();
            move |display_name| {
                let mut state = (*state).clone();
                state.display_name = display_name;
                nav.replace_with_state(&Route::CreatePuzzle, state);
            }
        });
        let onchange = Callback::from({
            let input_ref = input_ref.clone();
            let set_displayname = set_displayname.clone();
            move |_| {
                let input: HtmlInputElement = input_ref.cast().unwrap();
                set_displayname.emit(input.value());
            }
        });

        let oninput = Callback::from({
            let input_ref = input_ref.clone();
            move |_| {
                let input: HtmlInputElement = input_ref.cast().unwrap();
                set_displayname.emit(input.value());
            }
        });

        fields.push(html! {
            <div class="field">
                <label class="label">{"Puzzle display-name"}</label>
                <div class="control">
                    <input ref={input_ref} class="input" type="text" placeholder="Puzzle display name" onchange={onchange} oninput={oninput} value={state.display_name.clone()}/>
                </div>
            </div>
        });
    }

    // Puzzle state
    {
        let onchange = Callback::from({
            let nav = nav.clone();
            let state = state.clone();
            move |new_data: PuzzleState| {
                let mut state = (*state).clone();
                state.initial_state = new_data;
                // If we receive f-puzzles data and we've not set a display name, extract the title from the puzzle
                if let PuzzleData::FPuzzles(value) = &state.initial_state.data {
                    let metadata = fpuzzles::metadata(value);
                    if state.display_name.is_empty() {
                        if let Some(title) = &metadata.title {
                            state.display_name = title.clone();
                        }
                    }
                    if state.short_name.is_empty() {
                        if let Some(title) = metadata.title {
                            let space_dash = title.replace(' ', "-");
                            state.short_name =
                                clean_short_name(&space_dash, true).unwrap_or_default();
                        }
                    }
                }
                nav.replace_with_state(&Route::CreatePuzzle, state);
            }
        });
        fields.push(html! {
            <PuzzleStateEditor state_change={onchange} state={state.initial_state.clone()} />
        })
    }

    // Create button
    {
        let could_create = {
            !state.short_name.is_empty()
                && !state.display_name.is_empty()
                && !state.initial_state.description.is_empty()
        };

        let submitting = use_state_eq(|| false);

        let try_submit = Callback::from({
            //let state = state.clone();
            let submitting_setter = submitting.setter();
            //let api = api.clone();
            let nav = nav.clone();
            let toaster = toaster.clone();
            move |_| {
                submitting_setter.set(true);
                let submitting_setter = submitting_setter.clone();
                let api = api.clone();
                let nav = nav.clone();
                let toaster = toaster.clone();
                let state = state.clone();
                spawn_local(async move {
                    match api
                        .create_puzzle(
                            &state.owner,
                            &state.short_name,
                            &state.display_name,
                            &state.initial_state.description,
                            &state.initial_state.data,
                        )
                        .await
                    {
                        Ok(puzzle) => {
                            nav.push(&Route::ViewPuzzle {
                                puzzle: puzzle.uuid,
                            });
                        }
                        Err(e) => {
                            toaster.toast(
                                Toast::new(format!("Unable to create puzzle: {e}"))
                                    .with_level(ToastLevel::Warning)
                                    .with_lifetime(2500),
                            );
                            submitting_setter.set(false);
                        }
                    }
                });
            }
        });

        fields.push(html! {
            <div class={"field is-grouped"}>
                <div class="control">
                    <button class="button is-primary" disabled={!could_create || *submitting} onclick={try_submit}>
                        <span class={"icon-text"}>
                            <Icon icon={if *submitting { SpinnerIcon } else { SubmitFormIcon } }/>
                            <span>{if could_create { "Create puzzle" } else { "Missing inputs" } }</span>
                        </span>
                    </button>
                </div>
            </div>
        });
    }

    match user_info {
        LoginStatus::Unknown => html! {},
        LoginStatus::LoggedOut => {
            toaster.toast(
                Toast::new("You must be logged in in order to create a puzzle")
                    .with_level(ToastLevel::Warning)
                    .with_lifetime(2000),
            );
            nav.replace(&Route::Home);
            html! {}
        }
        LoginStatus::LoggedIn { .. } => html! {
            <MainPageLayout>
                <Title value="Create puzzle" />
                <h1 class="title">{"Creating a puzzle"}</h1>
                {for fields.into_iter()}
            </MainPageLayout>
        },
    }
}

#[derive(Properties, PartialEq)]
struct PuzzleStateEditorProps {
    state: PuzzleState,
    state_change: Callback<PuzzleState>,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum EditorKind {
    Nothing,
    FPuzzles,
    URLs,
    Pack,
}

const KIND_TITLE_NOTHING: &str = "No data";
const KIND_TITLE_FPUZZLES: &str = "F-Puzzles data";
const KIND_TITLE_URLS: &str = "List of URLs";
const KIND_TITLE_PACK: &str = "List of puzzles";

impl EditorKind {
    fn title(self) -> &'static str {
        use EditorKind::*;
        match self {
            Nothing => KIND_TITLE_NOTHING,
            FPuzzles => KIND_TITLE_FPUZZLES,
            URLs => KIND_TITLE_URLS,
            Pack => KIND_TITLE_PACK,
        }
    }

    fn from_title(title: &str) -> Self {
        use EditorKind::*;
        match title {
            KIND_TITLE_NOTHING => Nothing,
            KIND_TITLE_FPUZZLES => FPuzzles,
            KIND_TITLE_URLS => URLs,
            KIND_TITLE_PACK => Pack,
            _ => unreachable!(),
        }
    }
}

#[function_component(PuzzleStateEditor)]
fn puzzle_state_editor_render(props: &PuzzleStateEditorProps) -> Html {
    let mut fields = vec![];

    let transformer = Transformer::from({
        let state = props.state.clone();
        move |req| transform_markdown(&state, req)
    });

    // Description
    {
        let onchange = Callback::from({
            let state_change = props.state_change.clone();
            let state = props.state.clone();
            move |new_description: AttrValue| {
                let mut new_state = state.clone();
                new_state.description = new_description.to_string();
                state_change.emit(new_state);
            }
        });

        fields.push(html! {
            <div class="field">
                <label class="label">{"Description"}</label>
                <div class="control">
                    <MarkdownEditor initial={props.state.description.clone()} onchange={onchange} transformer={transformer}/>
                </div>
            </div>
        });
    }

    // Editors
    let mut editors: Vec<VChild<TabContent>> = vec![];

    // No extra content
    {
        editors.push(html_nested! {
            <TabContent title={EditorKind::Nothing.title()}>
                <h1 class="title">{"No extra data"}</h1>
            </TabContent>
        })
    }

    let fpuzzles_memory = use_state_eq(|| {
        if let PuzzleData::FPuzzles(value) = &props.state.data {
            if matches!(value, Value::Null) {
                "".into()
            } else {
                fpuzzles::encode(value)
            }
        } else {
            "".into()
        }
    });

    // FPuzzles data
    {
        let input_ref = use_node_ref();

        let handle_change = Callback::from({
            let input_ref = input_ref.clone();
            let setter = props.state_change.clone();
            let state = props.state.clone();
            let memory_setter = fpuzzles_memory.setter();
            move |()| {
                let input: HtmlInputElement = input_ref.cast().unwrap();
                let value = input.value();
                memory_setter.set(value.clone());
                let acquired = fpuzzles::extract(value);
                if let Some(value) = acquired {
                    let mut new_state = state.clone();
                    new_state.data = PuzzleData::FPuzzles(value);
                    if new_state.description.is_empty() {
                        new_state.description = DEFAULT_FPUZZLES_DESCRIPTION.to_string();
                    }
                    setter.emit(new_state);
                }
            }
        });

        let onchanged = Callback::from({
            let handle_change = handle_change.clone();
            move |_| handle_change.emit(())
        });

        let oninput = Callback::from(move |_| handle_change.emit(()));

        let content_to_render = if let PuzzleData::FPuzzles(value) = &props.state.data {
            if !matches!(value, Value::Null) {
                Some(value)
            } else {
                None
            }
        } else {
            None
        };
        let content_rendered = if let Some(value) = content_to_render {
            html! {
                <div class="tile is-child notification is-success">
                    <FPuzzlesRenderer data={value.clone()} />
                </div>
            }
        } else {
            html! {
                <div class="tile is-child notification is-danger">
                    <p class="subtitle">{"No valid FPuzzles found"}</p>
                </div>
            }
        };

        editors.push(html_nested! {
            <TabContent title={EditorKind::FPuzzles.title()}>
                <div class="field">
                    <label class="label">{"Link to puzzle, or puzzle string"}</label>
                    <div class="control has-icons-left">
                        <input ref={input_ref} class="input" type="text" placeholder="http://f-puzzles.com/?load=......" onchange={onchanged} oninput={oninput} value={fpuzzles_memory.to_string()}/>
                        <Icon size={IconSize::Small} icon={SimpleLinkIcon} class="icon is-left" />
                    </div>
                </div>
                <div class="field">
                    <label class="label">{"Decoded puzzle"}</label>
                    <div class="control">
                        {content_rendered}
                    </div>
                </div>
            </TabContent>
        })
    }

    let editor_kind = use_state_eq(|| EditorKind::FPuzzles);

    // Editors tabcontrol field
    {
        let tabchanged = Callback::from({
            let kind_setter = editor_kind.setter();
            let state = props.state.clone();
            let state_setter = props.state_change.clone();
            move |title: AttrValue| {
                let kind = EditorKind::from_title(&title);
                kind_setter.set(kind);
                let mut new_state = state.clone();
                match kind {
                    EditorKind::Nothing => {
                        new_state.data = PuzzleData::Nothing;
                    }
                    EditorKind::FPuzzles => {
                        new_state.data = PuzzleData::FPuzzles(
                            fpuzzles::extract(fpuzzles_memory.as_str()).unwrap_or(Value::Null),
                        );
                    }
                    _ => {
                        todo!();
                    }
                }
                state_setter.emit(new_state);
            }
        });
        fields.push(html! {
            <div class="field">
                <label class="label">{"Puzzle data"}</label>
                <div class="control">
                    <Tabbed default={editor_kind.title()} tabchanged={tabchanged}>
                        {for editors.into_iter()}
                    </Tabbed>
                </div>
            </div>
        })
    }

    html! {
        <>
            {for fields.into_iter()}
        </>
    }
}

#[derive(Properties, PartialEq)]
struct FPuzzlesDataRender {
    data: Value,
}

#[styled_component(FPuzzlesRenderer)]
fn fpuzzles_renderer(props: &FPuzzlesDataRender) -> Html {
    let metadata = fpuzzles::metadata(&props.data);

    let obj_style = use_style!("width: 50vh; height: 50vh;");

    enum FieldState {
        Ok,
        Warn,
        Bad,
    }

    use FieldState::*;

    fn show_field(key: &'static str, state: FieldState, value: String) -> Html {
        let icon = match state {
            Ok => OkayIcon,
            Warn => WarningIcon,
            Bad => BrokenIcon,
        };
        html! {
            <div class={"field"}>
                <div class={"label"}>{key}</div>
                <div class={"control has-icons-right"}>
                    <input class={"input"} type={"text"} value={value} readonly={true} />
                    <Icon class="icon is-right" icon={icon} />
                </div>
            </div>
        }
    }

    let grid = if let Some((rows, cols)) = metadata.rows_cols {
        format!("{rows}x{cols}")
    } else {
        "No grid?".into()
    };

    html! {
        <div class={"tile is-ancestor"}>
            <div class={"tile"}>
                <div class={"tile is-parent is-vertical"}>
                    <div class={"tile is-child"}>
                        {show_field("Grid size", Ok, grid)}
                        {show_field("Title", metadata.title.as_ref().map(|_| Ok).unwrap_or(Bad), metadata.title.unwrap_or_else(||"No embedded title".to_string()))}
                        {show_field("Author", metadata.author.as_ref().map(|_| Ok).unwrap_or(Bad), metadata.author.unwrap_or_else(||"No embedded author".to_string()))}
                        {show_field("Ruleset", metadata.rules.as_ref().map(|_| Ok).unwrap_or(Bad), metadata.rules.map(|_| "Provided").unwrap_or("Not provided").to_string())}
                        {show_field("Solution", if metadata.has_solution { Ok } else { Warn }, (if metadata.has_solution { "Provided" } else { "Not provided" }).to_string())}
                    </div>
                </div>
            </div>
            <div class={"tile notification is-4"}>
                <object type={"image/svg+xml"} data={fpuzzles::grid_url(&props.data)} class={obj_style}/>
            </div>
        </div>
    }
}
