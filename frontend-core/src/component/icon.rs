use yew::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum IconType {
    InternalLinkIcon,
    ExternalLinkIcon,
    SimpleLinkIcon,
    PermaLinkIcon,
    RoleIcon,
    CurrentRoleIcon,
    RoleEditIcon,
    RoleNiceLinkIcon,
    SubmitFormIcon,
    SpinnerIcon,
    PuzzleRestrictedIcon,
    PuzzlePublicIcon,
    PuzzlePublishedIcon,
    PuzzleAddIcon,
    PuzzleNiceLinkIcon,
    PuzzleEditMetadataIcon,
    PuzzleStateEditIcon,
    PuzzleStateAddIcon,
    OkayIcon,
    WarningIcon,
    BrokenIcon,
    CancelIcon,
}

pub use IconType::*;

impl IconType {
    fn icon_class(self) -> &'static str {
        match self {
            InternalLinkIcon => "mdi-arrow-right-bottom",
            ExternalLinkIcon => "mdi-arrow-u-left-top",
            SimpleLinkIcon => "mdi-link-variant",
            PermaLinkIcon => "mdi-pound",
            RoleIcon => "mdi-account-circle-outline",
            CurrentRoleIcon => "mdi-account-circle",
            RoleEditIcon => "mdi-account-edit-outline",
            RoleNiceLinkIcon => "mdi-account-arrow-left",
            SubmitFormIcon => "mdi-content-save",
            SpinnerIcon => "mdi-loading mdi-spin",
            PuzzleRestrictedIcon => "mdi-puzzle-outline",
            PuzzlePublicIcon => "mdi-puzzle-check-outline",
            PuzzlePublishedIcon => "mdi-puzzle-check",
            PuzzleAddIcon => "mdi-puzzle-plus-outline",
            PuzzleNiceLinkIcon => "mdi-puzzle-star",
            PuzzleEditMetadataIcon => "mdi-puzzle-edit-outline",
            PuzzleStateEditIcon => "mdi-database-edit-outline",
            PuzzleStateAddIcon => "mdi-database-plus-outline",
            OkayIcon => "mdi-check-circle",
            WarningIcon => "mdi-alert-circle",
            BrokenIcon => "mdi-heart-broken",
            CancelIcon => "mdi-cancel",
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
    pub onclick: Option<Callback<MouseEvent>>,
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
        <span class={span_class} onclick={props.onclick.clone()}>
            <i class={icon_class} aria-hidden={"true"}/>
        </span>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct GenericIconProps {
    pub class: Option<AttrValue>,
    pub icon: AttrValue,
    #[prop_or_default]
    pub size: IconSize,
}

#[function_component(GenericIcon)]
pub fn icon_render(props: &GenericIconProps) -> Html {
    let class = props
        .class
        .clone()
        .unwrap_or_else(|| "icon".into())
        .to_string();

    let icon_class = classes!["mdi", props.icon.to_string(), props.size.icon_class()];

    let span_class = classes![class, props.size.size_class()];

    html! {
        <span class={span_class}>
            <i class={icon_class} aria-hidden={"true"}/>
        </span>
    }
}
