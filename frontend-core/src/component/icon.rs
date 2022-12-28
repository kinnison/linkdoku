use yew::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum IconType {
    InternalLinkIcon,
    ExternalLinkIcon,
    RoleIcon,
    CurrentRoleIcon,
}

pub use IconType::*;

impl IconType {
    fn icon_class(self) -> &'static str {
        match self {
            InternalLinkIcon => "mdi-arrow-right-bottom",
            ExternalLinkIcon => "mdi-arrow-u-left-top",
            RoleIcon => "mdi-account-circle-outline",
            CurrentRoleIcon => "mdi-account-circle",
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct IconProps {
    pub class: Option<AttrValue>,
    pub icon: IconType,
}

#[function_component(Icon)]
pub fn icon_render(props: &IconProps) -> Html {
    let class = props
        .class
        .clone()
        .unwrap_or_else(|| "icon".into())
        .to_string();

    let icon_class = classes!["mdi", props.icon.icon_class(),];

    html! {
        <span class={class}>
            <i class={icon_class} aria-hidden={"true"}/>
        </span>
    }
}
