//! Modals used in Linkdoku
//!

use gloo::storage::{LocalStorage, Storage};
use yew::prelude::*;

use crate::layout::ModalMarkdown;

const TERMS_AND_CONDITIONS: &str = concat!(
    include_str!("../ts-cs-header.md"),
    include_str!("../../CODE_OF_CONDUCT.md"),
    include_str!("../ts-cs-footer.md"),
);
const TS_CS_BUTTON_TITLES: &[&str] = &["Accept these terms", "Reject these terms"];

#[function_component(TermsAndConditions)]
pub fn ts_and_cs_render() -> Html {
    let displayed = use_state(|| false);

    let tcs_hash = format!("{:x}", md5::compute(TERMS_AND_CONDITIONS));

    let dismissed = Callback::from({
        let tcs_hash = tcs_hash.clone();
        let setter = displayed.setter();
        move |n| {
            if n == 0 {
                // Accepted
                LocalStorage::set("terms-and-conditions", &tcs_hash)
                    .expect("Unable to store ts-and-cs");
                setter.set(false);
            } else {
                // Rejected
                gloo::utils::window()
                    .location()
                    .set_href("about:blank")
                    .expect("Unable to navigate");
            }
        }
    });

    use_effect_with_deps(
        {
            let setter = displayed.setter();
            move |hash| {
                let stored_hash = LocalStorage::get::<String>("terms-and-conditions").ok();
                if stored_hash.as_ref() != Some(hash) {
                    setter.set(true);
                }
            }
        },
        tcs_hash,
    );

    if *displayed {
        html! {
            <ModalMarkdown
                title={"Linkdoku terms and conditions of use"}
                markdown={TERMS_AND_CONDITIONS}
                buttons={TS_CS_BUTTON_TITLES}
                action={dismissed}
            />
        }
    } else {
        html! {}
    }
}
