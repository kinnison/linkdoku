//! Components for use with the tutorials

use std::rc::Rc;

use web_sys::{Element, ScrollIntoViewOptions, ScrollBehavior, ScrollLogicalPosition};
use yew::prelude::*;

use crate::{data::TutorialDataNode, TutorialData};

#[derive(Properties, PartialEq)]
pub struct TutorialControllerProps {
    pub tutorial: TutorialData,
}

#[function_component(TutorialController)]
pub fn tutorial_controller_render(_props: &TutorialControllerProps) -> Html {
    let client_side = use_state(|| false);

    use_effect_with_deps(
        {
            let setter = client_side.setter();
            move |_| {
                setter.set(true);
            }
        },
        (),
    );

    if *client_side {
        html! {
            <TutorialControllerInner tutorial={Rc::new(_props.tutorial.clone())} />
        }
    } else {
        html! {}
    }
}

#[derive(Properties, PartialEq)]
struct TutorialControllerInnerProps {
    tutorial: Rc<TutorialData>,
}

#[function_component(TutorialControllerInner)]
fn tutorial_controller_inner_render(props: &TutorialControllerInnerProps) -> Html {
    let change_displayed: Callback<(Option<usize>, Option<usize>, bool)> = use_callback(
        |(hide, show, abandon), tutorial| {
            //
            if let Some(n) = hide {
                let data: &TutorialDataNode = &tutorial.nodes()[n];
                let node: Element = data.node().cast().unwrap();
                let parent = node.parent_element().unwrap();
                parent.class_list().remove_1("is-popover-active").unwrap();
                tutorial.mark_node_used(n);
            }
            if let Some(n) = show {
                let data: &TutorialDataNode = &tutorial.nodes()[n];
                let node: Element = data.node().cast().unwrap();
                let parent = node.parent_element().unwrap();
                parent.class_list().add_1("is-popover-active").unwrap();
                let mut opts = ScrollIntoViewOptions::new();
                opts.behavior(ScrollBehavior::Smooth);
                opts.block(ScrollLogicalPosition::Nearest);
                opts.inline(ScrollLogicalPosition::Start);
                parent.scroll_into_view_with_scroll_into_view_options(&opts)
            }
            if abandon {
                tutorial.abandon();
            }
        },
        props.tutorial.clone(),
    );

    use_effect_with_deps(
        |(setter, tutorial)| {
            // We want to cause the first node to be shown
            if let Some(n) = tutorial.first_unseen() {
                setter.emit((None, Some(n), false));
            }
        },
        (change_displayed.clone(), props.tutorial.clone()),
    );

    let contents = props.tutorial.nodes().iter().enumerate().map(|(n, datanode)| {
        let node = datanode.node().cast().unwrap();
        
        let prev_onclick = props.tutorial.prev_unseen(n).map(|prev| change_displayed.reform(move|_| (Some(n), Some(prev), false)));
        let next_onclick = props.tutorial.next_unseen(n).map(|next| change_displayed.reform(move |_| (Some(n), Some(next), false)));
        let hide_onclick = change_displayed.reform(move |_| (Some(n), None, true));

        create_portal(
            html! {
                <>
                    <div class="field">
                        <label class="label">{"Help"}</label>
                        <div class="control">
                            {datanode.text()}
                        </div>
                    </div>
                    <div class="field is-grouped">
                        <div class="control">
                            <button class="button is-light is-link" disabled={prev_onclick.is_none()} onclick={prev_onclick}>{"Previous hint"}</button>
                        </div>
                        <div class="control">
                            <button class="button is-primary" disabled={next_onclick.is_none()} onclick={next_onclick}>{"Next hint"}</button>
                        </div>
                        <div class="control">
                            <button class="button is-light is-danger" onclick={hide_onclick}>{"Finish"}</button>
                        </div>
                    </div>
                </>
            },
            node,
        )
    });

    html! {
        {for contents}
    }
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum TutorialAnchorPosition {
    #[default]
    TutorialBottom,
    TutorialTop,
    TutorialRight,
    TutorialLeft,
}

impl TutorialAnchorPosition {
    fn class(self) -> &'static str {
        match self {
            TutorialAnchorPosition::TutorialBottom => "is-popover-bottom",
            TutorialAnchorPosition::TutorialTop => "is-popover-top",
            TutorialAnchorPosition::TutorialRight => "is-popover-right",
            TutorialAnchorPosition::TutorialLeft => "is-popover-left",
        }
    }
}

pub use TutorialAnchorPosition::*;

#[derive(Properties, PartialEq)]
pub struct TutorialAnchorProps {
    pub noderef: Option<NodeRef>,
    pub children: Children,
    pub class: Option<AttrValue>,
    #[prop_or_default]
    pub position: TutorialAnchorPosition,
}

#[function_component(TutorialAnchor)]
pub fn tutorial_anchor_render(props: &TutorialAnchorProps) -> Html {
    let popover_classes = classes!(
        props.class.as_ref().map(|v| v.to_string()),
        "popover",
        "is-not-popover-hover",
        props.position.class(),
    );
    let trigger_classes = classes!(
        props.class.as_ref().map(|v| v.to_string()),
        "popover-trigger"
    );
    if props.noderef.is_some() {
        html! {
            <div class={popover_classes}>
                <div class={trigger_classes}>
                    {for props.children.clone()}
                </div>
                <div ref={props.noderef.clone().unwrap()} class="popover-content" />
            </div>
        }
    } else {
        html! {
            {for props.children.clone()}
        }
    }
}
