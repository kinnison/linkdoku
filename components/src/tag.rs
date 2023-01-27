//! Tag components for Frontend

use apiprovider::use_cached_value;
use common::objects;
use frontend_core::component::utility::{Tooltip, TooltipAlignment};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TagProps {
    pub tag: AttrValue,
    pub ondelete: Option<Callback<AttrValue>>,
    pub onclick: Option<Callback<AttrValue>>,
}

#[function_component(Tag)]
pub fn tag_render(props: &TagProps) -> Html {
    let fallback = html! {};

    html! {
        <Suspense fallback={fallback}>
            <TagInner tag={props.tag.clone()} ondelete={props.ondelete.clone()} onclick={props.onclick.clone()}/>
        </Suspense>
    }
}

#[function_component(TagInner)]
fn tag_inner_render(props: &TagProps) -> HtmlResult {
    let cached_tag = use_cached_value::<objects::Tag>(props.tag.clone())?;

    let tag = match cached_tag.as_ref() {
        Err(e) => objects::Tag {
            uuid: "".to_string(),
            name: format!("error:{}", props.tag),
            colour: "#f14668".to_string(),
            black_text: true,
            description: format!("Unable to retrieve tag: {e}"),
        },

        Ok(v) => v.clone(),
    };

    // We have a tag, we need to split it into prefix and body, and prepare colours

    let (prefix, body) = tag.name.split_once(':').unwrap_or(("broken", &tag.name));

    let tag_style = format!(
        "background-color: {}; color: {}",
        tag.colour,
        if tag.black_text {
            "rgba(0,0,0,.7)"
        } else {
            "rgba(1,1,1,.7)"
        }
    );

    let delete_button = if let Some(cb) = props.ondelete.clone() {
        let uuid = props.tag.clone();
        let cb = cb.reform(move |_| uuid.clone());
        html! {
            <a class="tag is-delete" onclick={cb} />
        }
    } else {
        html! {}
    };

    let onclick = props.onclick.clone().map(|cb| {
        let uuid = props.tag.clone();
        cb.reform(move |_| uuid.clone())
    });

    let tagsclass = if onclick.is_some() {
        "tags has-addons is-clickable"
    } else {
        "tags has-addons"
    };

    Ok(html! {
        <div class={tagsclass} onclick={onclick}>
            <Tooltip content={tag.description} alignment={TooltipAlignment::Bottom} block={true}>
                <span class="tag is-dark">{prefix.to_string()}</span>
                <span class="tag" style={tag_style}>{body.to_string()}</span>
                {delete_button}
            </Tooltip>
        </div>
    })
}

#[derive(Properties, PartialEq)]
pub struct TagSetProps {
    pub label: Option<AttrValue>,
    pub tags: Vec<String>,
    pub ondelete: Option<Callback<AttrValue>>,
    pub onclick: Option<Callback<AttrValue>>,
}

#[function_component(TagSet)]
pub fn tag_set_render(props: &TagSetProps) -> Html {
    let tags = props.tags.iter().map(|tag| {
        html! {
            <div class="control">
                <Tag tag={tag.to_string()} ondelete={props.ondelete.clone()} onclick={props.onclick.clone()} />
            </div>
        }
    });

    let label = if let Some(label) = props.label.clone() {
        html! {
            <div class="field">
                <label class="label">{label}</label>
            </div>
        }
    } else {
        html! {}
    };

    html! {
        <>
            {label}
            <div class="field is-grouped is-grouped-multiline">
                {for tags}
            </div>
        </>
    }
}
