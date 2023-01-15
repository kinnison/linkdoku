use std::rc::Rc;

use common::{
    objects::{PuzzleData, PuzzleState, Visibility},
    public::puzzle,
};
use components::{layout::MainPageLayout, role::Role, user::LoginStatus};
use frontend_core::{component::icon::*, Route};
use puzzleutils::fpuzzles;
use serde_json::Value;
use stylist::yew::{styled_component, use_style};
use web_sys::HtmlInputElement;
use yew::{prelude::*, virtual_dom::VChild};
use yew_bulma_tabs::*;
use yew_markdown::editor::MarkdownEditor;
use yew_router::prelude::*;
use yew_toastrack::{use_toaster, Toast, ToastLevel};

use crate::util_components::Title;

#[function_component(CreatePuzzlePage)]
pub fn create_puzzle_page_render() -> Html {
    let nav = use_navigator().unwrap();
    let loc = use_location().unwrap();
    let user_info = use_context::<LoginStatus>().unwrap();
    let toaster = use_toaster();

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
                nav.replace_with_state(&Route::CreatePuzzle, state);
            }
        });
        fields.push(html! {
            <PuzzleStateEditor state_change={onchange} state={state.initial_state.clone()} />
        })
    }

    // Create button
    {
        fields.push(html! {
            <div class={"field is-grouped"}>
                <div class="control">
                    <button class="button is-primary" disabled={true}>
                        <span class={"icon-text"}>
                            <Icon icon={SubmitFormIcon}/>
                            <span>{"Creation unavailable"}</span>
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
                    <MarkdownEditor initial={props.state.description.clone()} onchange={onchange}/>
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

    // FPuzzles data
    {
        let input_ref = use_node_ref();
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

        let onchanged = Callback::from({
            let input_ref = input_ref.clone();
            let setter = props.state_change.clone();
            let state = props.state.clone();
            let memory_setter = fpuzzles_memory.setter();
            move |_| {
                let input: HtmlInputElement = input_ref.cast().unwrap();
                let value = input.value();
                memory_setter.set(value.clone());
                let acquired = fpuzzles::extract(value);
                if let Some(value) = acquired {
                    let mut new_state = state.clone();
                    new_state.data = PuzzleData::FPuzzles(value);
                    setter.emit(new_state);
                }
            }
        });

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
                        <input ref={input_ref} class="input" type="text" placeholder="http://f-puzzles.com/?load=......" onchange={onchanged} value={fpuzzles_memory.to_string()}/>
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
            move |title: AttrValue| {
                kind_setter.set(EditorKind::from_title(&title));
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
