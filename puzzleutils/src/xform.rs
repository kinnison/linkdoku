//! Transformer for puzzles, including fpuzzles data etc.

use yew::{prelude::*, virtual_dom::VNode};

use common::objects::{PuzzleData, PuzzleState};
use yew_markdown::xform::*;

fn trivially_text(node: &Html, target: &str) -> bool {
    if let VNode::VList(l) = node {
        if l.len() == 1 {
            if let VNode::VList(l) = &l[0] {
                if l.len() == 1 {
                    if let VNode::VText(text) = &l[0] {
                        if target == &*text.text {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

pub fn transform_markdown(grid: &PuzzleState, req: TransformRequest) -> TransformResponse {
    fn error(msg: String) -> TransformResponse {
        Some(html! {
            <strong><em>{msg}</em></strong>
        })
    }

    match req {
        TransformRequest::Link {
            url,
            title,
            content,
        } => {
            if let Some(maybe_idx) = url.strip_prefix("url-") {
                match maybe_idx.parse::<usize>() {
                    Ok(num) if num > 0 => {
                        if let PuzzleData::URLs(urls) = &grid.data {
                            if let Some(ue) = urls.get(num - 1) {
                                let content = if trivially_text(&content, &url) {
                                    html! {{ue.title.clone()}}
                                } else {
                                    content
                                };
                                let title = if title == url {
                                    ue.title.clone()
                                } else {
                                    title
                                };
                                Some(html! {
                                    <a href={ue.url.clone()} title={title.clone()}>{content}</a>
                                })
                            } else {
                                error(format!("URL index out of range: {}", num))
                            }
                        } else {
                            error(format!("Use of {} in non-URLs form puzzle state", url))
                        }
                    }
                    _ => error(format!("Bad number in `url-{}`", maybe_idx)),
                }
            } else if let Some(maybe_idx) = url.strip_prefix("puzzle-") {
                match maybe_idx.parse::<usize>() {
                    Ok(num) if num > 0 => {
                        if let PuzzleData::Pack(urls) = &grid.data {
                            if let Some(ue) = urls.get(num - 1) {
                                let content = if trivially_text(&content, &url) {
                                    html! {
                                        <em>{"TODO: Magic puzzle link content"}</em>
                                    }
                                } else {
                                    content
                                };
                                Some(html! {
                                    <span>{"This would be a puzzle link to "} {ue}{". "} {content}</span>
                                })
                            } else {
                                error(format!("Puzzle index out of range: {}", num))
                            }
                        } else {
                            error(format!("Use of {} in non-pack form puzzle state", url))
                        }
                    }
                    _ => error(format!("Bad number in `puzzle-{}`", maybe_idx)),
                }
            } else {
                match url.as_str() {
                    "grid" | "rules" | "fpuzzles" | "sudokupad" | "beta-sudokupad"
                    | "sudokupad-beta" => {
                        // Must have an fpuzzles dataset
                        if let PuzzleData::FPuzzles(grid) = &grid.data {
                            match url.as_str() {
                                "grid" => error(
                                    "Use of [grid] as a non-image link.  Did you mean `![grid]` instead?"
                                        .to_string(),
                                ),
                                "rules" => {
                                    let rules = grid
                                        .as_object()
                                        .and_then(|o| o.get("ruleset"))
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("No rules available in data");
                                    Some(html! {
                                        <yew_markdown::render::MarkdownRender markdown={rules.to_string()} />
                                    })
                                }
                                "fpuzzles" | "sudokupad" | "beta-sudokupad" | "sudokupad-beta" => {
                                    let data_str = crate::fpuzzles::encode(grid);
                                    let content = if trivially_text(&content, &url) {
                                        html! {
                                            {match url.as_str() {
                                                "fpuzzles" => "Play this on F-Puzzles",
                                                "sudokupad" => "Play this on Sudokupad",
                                                "beta-sudokupad" | "sudokupad-beta" => "Play this on Sudokupad (beta)",
                                                _ => unreachable!(),
                                            }}
                                        }
                                    } else {
                                        content
                                    };
                                    let link = match url.as_str() {
                                        "fpuzzles" => {
                                            format!("http://f-puzzles.com/?load={}", data_str)
                                        }
                                        "sudokupad" => {
                                            format!("https://sudokupad.app/fpuzzles{}", data_str)
                                        }
                                        "beta-sudokupad" | "sudokupad-beta" => format!(
                                            "https://beta.sudokupad.app/fpuzzles{}",
                                            data_str,
                                        ),
                                        _ => unreachable!(),
                                    };
                                    Some(html! {
                                        <a href={link}>{content}</a>
                                    })
                                }
                                _ => unreachable!(),
                            }
                        } else {
                            error(format!(
                                "Use of `{}` in a non-fpuzzles form puzzle state",
                                url
                            ))
                        }
                    }
                    _ => error(format!("Unknown special link: `{}`", url)),
                }
            }
        }
        TransformRequest::Image { url, .. } => {
            if url == "grid" {
                if let PuzzleData::FPuzzles(grid) = &grid.data {
                    Some(html! {
                        <img src={crate::fpuzzles::grid_url(grid)} style={"width: 50vh; height: 50vh;"} />
                    })
                } else {
                    error("Use of ![grid] in a non-fpuzzles puzzle state".to_string())
                }
            } else {
                None
            }
        }
    }
}
