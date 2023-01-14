//! Shortcut handling
//!

use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::role::FindRoleAndRedirect;

#[function_component(ShortcutHandler)]
pub fn shortcut_handler() -> Html {
    let loc = use_location().unwrap();
    let path = {
        let raw_path = loc.path();
        if let Some(rest) = raw_path.strip_prefix('/') {
            rest
        } else {
            raw_path
        }
    };

    if let Some((_role, _puzzle)) = path.split_once('/') {
        todo!()
    } else {
        // Just a role, try and find out which one
        html! {
            <FindRoleAndRedirect name={path.to_string()} />
        }
    }
}
