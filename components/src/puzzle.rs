//! Puzzle related components

use apiprovider::{use_apiprovider, use_cached_value};
use common::{
    objects::{self, Visibility},
    public::role::puzzles,
};
use frontend_core::{component::icon::*, Route};
use yew::{prelude::*, suspense::*};
use yew_router::prelude::*;
use yew_toastrack::{use_toaster, Toast, ToastLevel};

use crate::utils::NiceDate;

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
                Ok(puzzles::Response { puzzles: vec![] })
            }
        }
    })?;

    let list = match list.as_ref() {
        Ok(list) => list,
        Err(e) => {
            toaster.toast(
                Toast::new(format!("Failure fetching puzzle list: {e}"))
                    .with_level(ToastLevel::Warning)
                    .with_lifetime(5000),
            );
            nav.push(&Route::Home);
            return Ok(html! {});
        }
    };

    let list = list.puzzles.iter().map(|meta| {
        html! {
            <Link<Route> to={Route::ViewPuzzle { puzzle: meta.uuid.clone() }} classes="panel-block">
                <PuzzleListEntry puzzle={meta.uuid.clone()} />
            </Link<Route>>
        }
    });

    Ok(html! {
        <div class="panel">
            <p class="panel-heading">{"Puzzles"}</p>
            {for list}
        </div>
    })
}

#[derive(Properties, PartialEq)]
pub struct PuzzleListEntryProps {
    pub puzzle: AttrValue,
}

#[function_component(PuzzleListEntry)]
pub fn puzzle_list_entry(props: &PuzzleListEntryProps) -> Html {
    let fallback = html! {};

    html! {
        <Suspense fallback={fallback}>
            <PuzzleListEntryInner puzzle={props.puzzle.clone()} />
        </Suspense>
    }
}

#[function_component(PuzzleListEntryInner)]
fn puzzle_list_entry_inner(props: &PuzzleListEntryProps) -> HtmlResult {
    let puzzle = use_cached_value::<objects::PuzzleMetadata>(props.puzzle.clone())?;

    let puzzle = match puzzle.as_ref() {
        Err(_) => {
            return Ok(html! {
                {format!("Unknown puzzle: {}", props.puzzle)}
            })
        }
        Ok(r) => r,
    };
    let icon = match puzzle.visibility {
        Visibility::Restricted => PuzzleRestrictedIcon,
        Visibility::Public => PuzzlePublicIcon,
        Visibility::Published => PuzzlePublishedIcon,
    };

    Ok(html! {
        <>
            <Icon icon={icon} class="panel-icon"/>
            <div class="columns">
                <div class="column is-clipped">{format!("{} ({})", puzzle.display_name, puzzle.short_name)}</div>
                <div class="column is-narrow"><NiceDate date={puzzle.updated_at.clone()} /></div>
            </div>
        </>
    })
}
