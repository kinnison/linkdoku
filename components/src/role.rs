use apiprovider::use_cached_value;
use common::objects;
use stylist::yew::{styled_component, use_style};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RoleProps {
    pub uuid: AttrValue,
    pub active: Option<bool>,
    pub onclick: Option<Callback<()>>,
}

#[function_component(Role)]
pub fn role_widget(props: &RoleProps) -> Html {
    let fallback = html! {};

    html! {
        <Suspense fallback={fallback}>
            <RoleInner uuid={props.uuid.clone()} active={props.active} onclick={props.onclick.clone()} />
        </Suspense>
    }
}

#[styled_component(RoleInner)]
fn role_widget_inner(props: &RoleProps) -> HtmlResult {
    let role_style = use_style!(
        r#"
    display: inline-block;
    overflow-x: hidden;
    "#
    );
    let nowrap = use_style!("flex-wrap: nowrap !important;");
    let role = use_cached_value::<objects::Role>(props.uuid.clone())?;

    let role_body = match role.as_ref() {
        Err(e) => {
            html! {
                <span class="is-danger">{e.to_string()}</span>
            }
        }
        Ok(role) => {
            html! {
                <span>{role.display_name.clone()}</span>
            }
        }
    };

    let active = props.active.unwrap_or(false);

    let icon_class = if active {
        classes! { "mdi", "mdi-account-circle" }
    } else {
        classes! { "mdi", "mdi-account-circle-outline" }
    };

    let role_body = html! {
        <span class={classes!("icon-text", nowrap)}>
            <span class={"icon"}><i class={icon_class} /></span>
            {role_body}
        </span>
    };

    let role_classes = {
        let mut ret = if active {
            classes!(
                "box",
                "has-background-primary-light",
                "is-shadowless",
                "p-1"
            )
        } else {
            classes!("block")
        };
        ret.push(role_style);
        if props.onclick.is_some() {
            ret.push(classes! {"is-clickable"});
        }
        ret
    };

    let ret = if let Some(cb) = &props.onclick {
        // We have an onclick, so use it
        let cb = cb.clone();
        let onclick = Callback::from(move |_| cb.emit(()));
        html! {
            <div class={role_classes} onclick={onclick}>
                {role_body}
            </div>
        }
    } else {
        // No onclick, so just show
        html! {
            <div class={role_classes}>
                {role_body}
            </div>
        }
    };

    Ok(ret)
}
