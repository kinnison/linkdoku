//! Utility components for Linkdoku which rely on being close to the app
//!

use bounce::helmet::Helmet;
use frontend_core::make_title;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct TitleProps {
    pub value: AttrValue,
}

#[function_component(Title)]
pub fn title_renderer(props: &TitleProps) -> Html {
    html! {
        <Helmet>
            <title>
                {make_title(&props.value)}
            </title>
        </Helmet>
    }
}
