//! Utility components
//!

use std::collections::HashMap;

use gloo_utils::format::JsValueSerdeExt;
use js_sys::{Array, Intl, Object};
use serde::Deserialize;
use wasm_bindgen::JsValue;
use web_sys::HtmlDivElement;
use yew::prelude::*;

#[derive(Deserialize)]
struct DTFEntry {
    value: String,
}

#[derive(Properties, PartialEq)]
pub struct NiceDateProps {
    pub date: AttrValue,
}

#[function_component(NiceDate)]
pub fn nice_date_render(props: &NiceDateProps) -> Html {
    let date_time_format = use_memo(
        |_| {
            let mut ret = HashMap::new();
            ret.insert("dateStyle", "medium");
            ret.insert("timeStyle", "long");
            ret
        },
        (),
    );

    let date_ref = use_node_ref();
    use_effect_with_deps(
        move |(dateref, date)| {
            let nav_lang = gloo::utils::window()
                .navigator()
                .language()
                .unwrap_or_else(|| "en-GB".into());
            let date_div: HtmlDivElement = dateref.cast().unwrap();
            let date_obj = js_sys::Date::new(&date.as_str().into());
            let options = JsValue::from_serde(date_time_format.as_ref())
                .expect("Can't unpack datetime formatter options");
            let langs = Array::new();
            langs.push(&nav_lang.into());
            let formatter = Intl::DateTimeFormat::new(&langs, &options.into());
            let date_parts = formatter.format_to_parts(&date_obj);
            let date_parts: &Object = date_parts.as_ref();
            let date_parts: Vec<DTFEntry> =
                date_parts.into_serde().expect("Unable to unpack parts");
            let date_parts = date_parts.into_iter().map(|e| e.value).collect::<Vec<_>>();
            let date_str = date_parts.join("");
            date_div.set_inner_text(&date_str);
        },
        (date_ref.clone(), props.date.clone()),
    );

    html! {
        <span ref={date_ref}>{props.date.clone()}</span>
    }
}
