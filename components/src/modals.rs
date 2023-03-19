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

#[derive(Properties, PartialEq)]
pub struct TermsAndConditionsProps {
    pub redisplay_trigger: usize,
}

#[function_component(TermsAndConditions)]
pub fn ts_and_cs_render(props: &TermsAndConditionsProps) -> Html {
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
            move |(hash, trigger)| {
                let stored_hash = LocalStorage::get::<String>("terms-and-conditions").ok();
                if (*trigger > 0) || (stored_hash.as_ref() != Some(hash)) {
                    setter.set(true);
                }
            }
        },
        (tcs_hash, props.redisplay_trigger),
    );

    if *displayed {
        let buttons = if props.redisplay_trigger > 0 {
            &["Close"]
        } else {
            TS_CS_BUTTON_TITLES
        };
        html! {
            <ModalMarkdown
                title={"Linkdoku - Info about stored data, and terms and conditions of use"}
                markdown={TERMS_AND_CONDITIONS}
                buttons={buttons}
                action={dismissed}
            />
        }
    } else {
        html! {}
    }
}
