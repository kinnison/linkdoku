//! Puzzle related components

use apiprovider::use_apiprovider;
use frontend_core::Route;
use yew::{prelude::*, suspense::*};
use yew_router::prelude::use_navigator;
use yew_toastrack::{use_toaster, Toast, ToastLevel};

#[derive(Properties, PartialEq, Clone)]
pub struct PuzzleListProps {
    pub role: Option<AttrValue>,
}

#[function_component(PuzzleList)]
pub fn puzzle_list_render(props: &PuzzleListProps) -> Html {
    let fallback = html! {};

    html! {
        <Suspense fallback={fallback}>
            <PuzzleListInner props={props.clone()} />
        </Suspense>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct PuzzleListInnerProps {
    pub props: PuzzleListProps,
}

#[function_component(PuzzleListInner)]
fn puzzle_list_inner_render(props: &PuzzleListInnerProps) -> HtmlResult {
    let props = &props.props;
    let api = use_apiprovider();
    let toaster = use_toaster();
    let nav = use_navigator().unwrap();

    let list = use_future({
        let props = props.clone();
        move || async move {
            if let Some(role) = &props.role {
                api.published_puzzle_list(role.as_str()).await
            } else {
                Ok(vec![])
            }
        }
    })?;

    let list = match list.as_ref() {
        Ok(list) => list,
        Err(e) => {
            toaster.toast(
                Toast::new(format!("Failure fetching puzzle list: {}", e))
                    .with_level(ToastLevel::Warning)
                    .with_lifetime(5000),
            );
            nav.push(&Route::Home);
            return Ok(html! {});
        }
    };

    let list = list.iter().map(|s| html! {<ul>{s.clone()}</ul>});

    Ok(html! {
        <ul>
            {for list}
        </ul>
    })
}
