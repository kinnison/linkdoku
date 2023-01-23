//! Yew Paginator
//!
//! This is a Bulma pagination control, it's not particularly complex

use std::ops::RangeInclusive;

use tracing::{trace, warn};
use yew::prelude::*;

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub enum PaginatorSize {
    Small,
    #[default]
    Normal,
    Medium,
    Large,
}

impl PaginatorSize {
    fn class(self) -> Option<&'static str> {
        match self {
            Self::Small => Some("small"),
            Self::Normal => None,
            Self::Medium => Some("medium"),
            Self::Large => Some("large"),
        }
    }
}

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub enum PaginatorNumberPosition {
    #[default]
    Left,
    Middle,
    Right,
}

impl PaginatorNumberPosition {
    fn class(self) -> Option<&'static str> {
        match self {
            Self::Left => None,
            Self::Middle => Some("is-centered"),
            Self::Right => Some("is-right"),
        }
    }
}

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct PaginatorProps {
    pub aria_label: Option<AttrValue>,
    pub element: Option<AttrValue>,
    pub count: usize,
    pub current: usize,
    pub siblings: Option<usize>,
    pub onchange: Callback<usize>,
    #[prop_or_default]
    pub size: PaginatorSize,
    #[prop_or_default]
    pub rounded: bool,
    #[prop_or_default]
    pub number_position: PaginatorNumberPosition,
}

#[function_component(Paginator)]
pub fn paginator_render(props: &PaginatorProps) -> Html {
    let details = use_pagination(props.count, props.siblings.unwrap_or(1), props.current);

    let (count, current) = (props.count, props.current);

    if count < 2 {
        // There's no point in showing a nav
        return html! {};
    }

    if current > count || current == 0 {
        warn!("Bad arguments to paginator: {props:?}");
        return html! {};
    }

    let label = props
        .aria_label
        .clone()
        .unwrap_or(AttrValue::Static("pagination"));
    let element = props.element.clone().unwrap_or(AttrValue::Static("page"));

    let prev_classes = classes!(
        "pagination-previous",
        if current == 1 {
            Some("is-disabled")
        } else {
            None
        }
    );
    let next_classes = classes!(
        "pagination-next",
        if current == count {
            Some("is-disabled")
        } else {
            None
        }
    );

    let prev_cb = if current == 1 {
        None
    } else {
        Some(props.onchange.reform(move |_| current - 1))
    };

    let next_cb = if current == count {
        None
    } else {
        Some(props.onchange.reform(move |_| current + 1))
    };

    let item_for = {
        let element = element.clone();
        move |n| {
            let (cls, cb) = if n == current {
                ("pagination-link is-current", None)
            } else {
                ("pagination-link", Some(props.onchange.reform(move |_| n)))
            };
            html! {
                <li>
                    <a class={cls} onclick={cb} title={format!("Display {element} {n}")}>{n}</a>
                </li>
            }
        }
    };

    let do_ellipsis = |b| {
        if b {
            html! {
                <li>
                    <span class="pagination-ellipsis">{"…"}</span>
                </li>
            }
        } else {
            html! {}
        }
    };

    let first = if details.show_left {
        Some(item_for(1))
    } else {
        None
    };
    let last = if details.show_right {
        Some(item_for(count))
    } else {
        None
    };

    let inner = details.inner_range.clone().map(&item_for);

    let paginator_classes = classes! {
        "pagination",
        props.size.class(),
        if props.rounded { Some("is-rounded") } else { None },
        props.number_position.class(),
    };

    html! {
        <nav class={paginator_classes} role="navigation" aria-label={label}>
            <a class={prev_classes} onclick={prev_cb}>{format!("Previous {element}")}</a>
            <a class={next_classes} onclick={next_cb}>{format!("Next {element}")}</a>
            <ul class="pagination-list">
                {first}
                {do_ellipsis(details.show_left)}
                {for inner}
                {do_ellipsis(details.show_right)}
                {last}
            </ul>
        </nav>
    }
}

struct PaginationDetails {
    show_left: bool,
    show_right: bool,
    inner_range: RangeInclusive<usize>,
}

#[hook]
fn use_pagination(count: usize, siblings: usize, current: usize) -> std::rc::Rc<PaginationDetails> {
    use_memo(
        |(count, siblings, current)| {
            let (count, siblings, current) = (*count, *siblings, *current);
            trace!("Computing pagination: count={count} siblings={siblings} current={current}");
            // maximal display is
            // first DOTS sibs current sibs DOTS last
            let total_nums = siblings + siblings + 3;
            trace!("total_nums = {total_nums}");
            if total_nums >= count {
                return PaginationDetails {
                    show_left: false,
                    show_right: false,
                    inner_range: 1..=count,
                };
            }
            let left_sibling_index = current.saturating_sub(siblings).max(1);
            let right_sibling_index = (current + siblings).min(count);
            let show_left = left_sibling_index > 2;
            let show_right = right_sibling_index < (count - 2);
            let sib_size = 3 + siblings + siblings;
            trace!("lsi={left_sibling_index} rsi={right_sibling_index} show_left={show_left} show_right={show_right} sib_size={sib_size}");
            let inner_range = match (show_left, show_right) {
                (false, true) => 1..=sib_size,
                (true, false) => (count - sib_size + 1)..=count,
                (true, true) => left_sibling_index..=right_sibling_index,
                _ => unreachable!(),
            };
            PaginationDetails {
                show_left,
                show_right,
                inner_range,
            }
        },
        (count, siblings, current),
    )
}
