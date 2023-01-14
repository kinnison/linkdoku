use common::public::puzzle;
use components::{role::Role, user::LoginStatus};
use frontend_core::Route;
use tracing::info;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use yew_toastrack::{use_toaster, Toast, ToastLevel};

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
            <>
                <h1 class="title">{"Creating a puzzle"}</h1>
                {for fields.into_iter()}
            </>
        },
    }
}
