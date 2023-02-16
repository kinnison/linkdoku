//! Redirectors for Linkdoku
//!

use axum::{
    extract::Path,
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use common::{
    objects::{PuzzleData, Visibility},
    APIError,
};
use database::{activity, models, Connection};
use serde::Deserialize;

use crate::{login::PrivateCookies, state::BackendState};

#[derive(Deserialize)]
enum Redirector {
    #[serde(rename = "fpuzzles")]
    #[serde(alias = "f-puzzles")]
    FPuzzles,
    #[serde(rename = "sudokupad")]
    Sudokupad,
    #[serde(rename = "sudokupad-beta")]
    #[serde(alias = "beta-sudokupad")]
    #[serde(alias = "betasudokupad")]
    #[serde(alias = "sudokupadbeta")]
    BetaSudokupad,
}

async fn shortcut_puzzle_redirector(
    Path((role, puzzle, redir)): Path<(String, String, Redirector)>,
    mut db: Connection,
    cookies: PrivateCookies,
) -> axum::response::Response {
    let logged_in = cookies.get_login_flow_status().await;
    let user = logged_in.user_uuid();
    match activity::puzzle::lookup(&mut db, &role, &puzzle, user).await {
        Ok(puzzle) => puzzle_redirector(Path((puzzle, redir)), db, cookies).await,
        Err(e) => APIError::from(e).into_response(),
    }
}

async fn puzzle_redirector(
    Path((puzzle, redir)): Path<(String, Redirector)>,
    mut db: Connection,
    cookies: PrivateCookies,
) -> axum::response::Response {
    let logged_in = cookies.get_login_flow_status().await;
    let user = logged_in.user_uuid();
    let puzzle = match models::Puzzle::by_uuid(&mut db, &puzzle)
        .await
        .map_err(|e| APIError::DatabaseError(e.to_string()))
        .transpose()
        .unwrap_or(Err(APIError::ObjectNotFound))
    {
        Ok(puzzle) => puzzle,
        Err(e) => return e.into_response(),
    };

    if !match puzzle
        .can_be_seen(&mut db, user)
        .await
        .map_err(|e| APIError::DatabaseError(e.to_string()))
    {
        Ok(b) => b,
        Err(e) => return e.into_response(),
    } {
        return APIError::ObjectNotFound.into_response();
    }

    let puzzle = match activity::puzzle::into_api_object(&mut db, user, puzzle).await {
        Ok(puzzle) => puzzle,
        Err(e) => return APIError::from(e).into_response(),
    };

    // Now we get to determine the best state...
    let display_index = puzzle
        .states
        .iter()
        .enumerate()
        .skip(1)
        .fold(
            (0, puzzle.states[0].visibility),
            |(best_index, best_vis), (idx, state)| {
                use Visibility::*;
                match (best_vis, state.visibility) {
                    (Public, Restricted) | (Published, Restricted) | (Published, Public) => {
                        (best_index, best_vis)
                    }
                    _ => (idx, state.visibility),
                }
            },
        )
        .0;

    match &puzzle.states[display_index].data {
        PuzzleData::FPuzzles(v) => {
            let fpuzzles_str = puzzleutils::fpuzzles::encode(v);
            let url = match redir {
                Redirector::FPuzzles => format!("https://f-puzzles.com/?load={fpuzzles_str}"),
                Redirector::Sudokupad => format!("https://sudokupad.app/fpuzzles{fpuzzles_str}"),
                Redirector::BetaSudokupad => {
                    format!("https://beta.sudokupad.app/fpuzzles{fpuzzles_str}")
                }
            };
            Redirect::to(&url).into_response()
        }
        _ => APIError::CannotCreatePuzzleShortcut.into_response(),
    }
}

pub fn router() -> Router<BackendState> {
    Router::new().route("/:role/:puzzle/:redir", get(shortcut_puzzle_redirector))
}
