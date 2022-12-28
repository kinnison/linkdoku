use yew::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum IconType {
    InternalLinkIcon,
    ExternalLinkIcon,
    RoleIcon,
    CurrentRoleIcon,
    RoleEditIcon,
}

pub use IconType::*;

impl IconType {
    fn icon_class(self) -> &'static str {
        match self {
            InternalLinkIcon => "mdi-arrow-right-bottom",
            ExternalLinkIcon => "mdi-arrow-u-left-top",
            RoleIcon => "mdi-account-circle-outline",
            CurrentRoleIcon => "mdi-account-circle",
            RoleEditIcon => "mdi-account-edit-outline",
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum IconSize {
    Small,
    #[default]
    Normal,
    Medium,
    Large,
}

impl IconSize {
    fn size_class(self) -> Option<&'static str> {
        match self {
            Self::Small => Some("is-small"),
            Self::Normal => None,
            Self::Medium => Some("is-medium"),
            Self::Large => Some("is-large"),
        }
    }

    fn icon_class(self) -> Option<&'static str> {
        match self {
            Self::Small => None,
            Self::Normal => Some("mdi-24px"),
            Self::Medium => Some("mdi-36px"),
            Self::Large => Some("mdi-48px"),
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct IconProps {
    pub class: Option<AttrValue>,
    pub icon: IconType,
    #[prop_or_default]
    pub size: IconSize,
}

#[function_component(Icon)]
pub fn icon_render(props: &IconProps) -> Html {
    let class = props
        .class
        .clone()
        .unwrap_or_else(|| "icon".into())
        .to_string();

    let icon_class = classes!["mdi", props.icon.icon_class(), props.size.icon_class()];

    let span_class = classes![class, props.size.size_class()];

    html! {
        <span class={span_class}>
            <i class={icon_class} aria-hidden={"true"}/>
        </span>
    }
}
