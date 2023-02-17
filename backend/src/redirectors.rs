//! Redirectors for Linkdoku
//!

use axum::{extract::Path, response::Redirect, routing::get, Router};
use common::{
    objects::{PuzzleData, Visibility},
    APIError, APIResult,
};
use database::{activity, models, Connection};
use serde::Deserialize;

use crate::{login::PrivateCookies, state::BackendState};

#[derive(Debug, Deserialize)]
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

#[tracing::instrument(skip(db, cookies))]
async fn shortcut_puzzle_redirector(
    Path((role, puzzle, redir)): Path<(String, String, Redirector)>,
    mut db: Connection,
    cookies: PrivateCookies,
) -> APIResult<Redirect> {
    let logged_in = cookies.get_login_flow_status().await;
    let user = logged_in.user_uuid();
    let puzzle = if role == "puzzle" {
        puzzle
    } else {
        activity::puzzle::lookup(&mut db, &role, &puzzle, user).await?
    };
    puzzle_redirector(Path((puzzle, redir)), db, cookies).await
}

#[tracing::instrument(skip(db, cookies))]
async fn puzzle_redirector(
    Path((puzzle, redir)): Path<(String, Redirector)>,
    mut db: Connection,
    cookies: PrivateCookies,
) -> APIResult<Redirect> {
    let logged_in = cookies.get_login_flow_status().await;
    let user = logged_in.user_uuid();
    let puzzle = models::Puzzle::by_uuid(&mut db, &puzzle)
        .await
        .map_err(|e| APIError::DatabaseError(e.to_string()))
        .transpose()
        .unwrap_or(Err(APIError::ObjectNotFound))?;
    if !puzzle
        .can_be_seen(&mut db, user)
        .await
        .map_err(|e| APIError::DatabaseError(e.to_string()))?
    {
        return Err(APIError::ObjectNotFound);
    }

    let puzzle = activity::puzzle::into_api_object(&mut db, user, puzzle).await?;

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
            Ok(Redirect::to(&url))
        }
        _ => Err(APIError::CannotCreatePuzzleShortcut),
    }
}

pub fn router() -> Router<BackendState> {
    Router::new().route("/:role/:puzzle/:redir", get(shortcut_puzzle_redirector))
}
