//! Shortcut handling
//!

use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{puzzle::FindPuzzleAndRedirect, role::FindRoleAndRedirect};

#[function_component(ShortcutHandler)]
pub fn shortcut_handler() -> Html {
    let loc = use_location().unwrap();
    let path = loc.path().trim_start_matches('/').trim_end_matches('/');

    if let Some((role, puzzle)) = path.split_once('/') {
        html! {
            <FindPuzzleAndRedirect role={role.to_string()} puzzle={puzzle.to_string()} />
        }
    } else {
        // Just a role, try and find out which one
        html! {
            <FindRoleAndRedirect name={path.to_string()} />
        }
    }
}
