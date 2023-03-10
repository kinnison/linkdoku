//! Puzzle activities such as creation/updating of puzzles
//!

use common::objects;
use diesel_async::AsyncPgConnection;
use time::format_description::well_known::Iso8601;

use crate::{
    models::{self, Puzzle, PuzzleState, Role, Visibility},
    utils::random_uuid,
};

use super::{ActivityError, ActivityResult};

#[tracing::instrument(skip_all)]
pub async fn create(
    conn: &mut AsyncPgConnection,
    actor: &str,
    owner: &str,
    short_name: &str,
    display_name: &str,
    initial_state: &objects::PuzzleState,
) -> ActivityResult<models::Puzzle> {
    conn.build_transaction()
        .run(|txn| {
            Box::pin(async move {
                let owning_role = models::Role::by_uuid(txn, owner)
                    .await?
                    .ok_or(ActivityError::InvalidInput)?;
                if !owning_role.can_modify(txn, actor).await? {
                    return Err(ActivityError::PermissionDenied);
                }
                if models::Puzzle::by_short_name(txn, owner, short_name)
                    .await?
                    .is_some()
                {
                    return Err(ActivityError::ShortNameInUse);
                }
                // Okay, we can modify the role, thus create a puzzle.  Insertion could fail if the
                // short_name is not unique to the role despite our check
                let puzzle = models::Puzzle::create(
                    txn,
                    &random_uuid("puzzle"),
                    owner,
                    display_name,
                    short_name,
                    Visibility::Restricted,
                )
                .await?;
                // Now we can insert the initial state into this
                puzzle
                    .add_state(
                        txn,
                        &initial_state.description,
                        Visibility::Restricted,
                        &serde_json::to_string(&initial_state.data)?,
                    )
                    .await?;
                Ok(puzzle)
            })
        })
        .await
}

#[tracing::instrument(skip_all)]
pub async fn into_api_object(
    conn: &mut AsyncPgConnection,
    actor: Option<&str>,
    puzzle: models::Puzzle,
) -> ActivityResult<objects::Puzzle> {
    conn.build_transaction()
        .run(|txn| {
            Box::pin(async move {
                if !puzzle.can_be_seen(txn, actor).await? {
                    return Err(ActivityError::PermissionDenied);
                }

                let mut states = vec![];

                for state in puzzle.all_states(txn).await? {
                    if state.can_be_seen(txn, &puzzle, actor).await? {
                        states.push(objects::PuzzleState {
                            uuid: state.uuid,
                            description: state.description,
                            visibility: state.visibility.into(),
                            updated_at: state.updated_at.format(&Iso8601::DEFAULT)?,
                            data: serde_json::from_str(&state.data)?,
                        });
                    }
                }

                if states.is_empty() {
                    return Err(ActivityError::PermissionDenied);
                }

                let tags = puzzle.get_tags(txn).await?;

                Ok(objects::Puzzle {
                    uuid: puzzle.uuid,
                    owner: puzzle.owner,
                    display_name: puzzle.display_name,
                    short_name: puzzle.short_name,
                    visibility: puzzle.visibility.into(),
                    created_at: puzzle.created_at.format(&Iso8601::DEFAULT)?,
                    updated_at: puzzle.updated_at.format(&Iso8601::DEFAULT)?,
                    states,
                    tags,
                })
            })
        })
        .await
}

#[tracing::instrument(skip_all)]
pub async fn lookup(
    conn: &mut AsyncPgConnection,
    role: &str,
    puzzle: &str,
    user: Option<&str>,
) -> ActivityResult<String> {
    conn.build_transaction()
        .run(|txn| {
            Box::pin(async move {
                let role = match Role::by_short_name(txn, role).await? {
                    Some(role) => role,
                    None => return Err(ActivityError::NotFound),
                };
                let puzzle = match Puzzle::by_short_name(txn, &role.uuid, puzzle).await? {
                    Some(puzzle) => puzzle,
                    None => return Err(ActivityError::NotFound),
                };
                if !puzzle.can_be_seen(txn, user).await? {
                    return Err(ActivityError::NotFound);
                }
                Ok(puzzle.uuid)
            })
        })
        .await
}

#[tracing::instrument(skip_all)]
pub async fn update_metadata(
    conn: &mut AsyncPgConnection,
    user: &str,
    puzzle: &str,
    short_name: &str,
    display_name: &str,
) -> ActivityResult<models::Puzzle> {
    conn.build_transaction()
        .run(|txn| {
            Box::pin(async move {
                let puzzle = match Puzzle::by_uuid(txn, puzzle).await? {
                    Some(puzzle) => puzzle,
                    None => return Err(ActivityError::NotFound),
                };

                if !puzzle.can_edit(txn, user).await? {
                    return Err(ActivityError::PermissionDenied);
                }

                Ok(puzzle
                    .update_metadata(txn, short_name, display_name)
                    .await?)
            })
        })
        .await
}

#[tracing::instrument(skip_all)]
pub async fn update_state(
    conn: &mut AsyncPgConnection,
    user: &str,
    puzzle: &str,
    state: &objects::PuzzleState,
) -> ActivityResult<models::Puzzle> {
    conn.build_transaction()
        .run(|txn| {
            Box::pin(async move {
                let puzzle = match Puzzle::by_uuid(txn, puzzle).await? {
                    Some(puzzle) => puzzle,
                    None => return Err(ActivityError::NotFound),
                };

                if !puzzle.can_edit(txn, user).await? {
                    return Err(ActivityError::PermissionDenied);
                }

                let puzzle_state = match PuzzleState::by_uuid(txn, &state.uuid).await? {
                    Some(ps) => ps,
                    None => return Err(ActivityError::NotFound),
                };

                if puzzle_state.puzzle != puzzle.uuid {
                    return Err(ActivityError::NotFound);
                }

                puzzle_state
                    .update(
                        txn,
                        &state.description,
                        &serde_json::to_string(&state.data)?,
                    )
                    .await?;

                Ok(puzzle)
            })
        })
        .await
}

#[tracing::instrument(skip_all)]
pub async fn add_state(
    conn: &mut AsyncPgConnection,
    user: &str,
    puzzle: &str,
    state: &objects::PuzzleState,
) -> ActivityResult<models::Puzzle> {
    conn.build_transaction()
        .run(|txn| {
            Box::pin(async move {
                let puzzle = match Puzzle::by_uuid(txn, puzzle).await? {
                    Some(puzzle) => puzzle,
                    None => return Err(ActivityError::NotFound),
                };

                if !puzzle.can_edit(txn, user).await? {
                    return Err(ActivityError::PermissionDenied);
                }

                puzzle
                    .add_state(
                        txn,
                        &state.description,
                        Visibility::Restricted,
                        &serde_json::to_string(&state.data)?,
                    )
                    .await?;

                Ok(puzzle)
            })
        })
        .await
}

#[tracing::instrument(skip_all)]
pub async fn set_visibility(
    conn: &mut crate::Connection,
    user: &str,
    puzzle: &str,
    visibility: objects::Visibility,
    in_view_state: Option<&str>,
) -> ActivityResult<models::Puzzle> {
    conn.build_transaction()
        .run(|txn| {
            Box::pin(async move {
                let puzzle = match Puzzle::by_uuid(txn, puzzle).await? {
                    Some(puzzle) => puzzle,
                    None => return Err(ActivityError::NotFound),
                };

                if !puzzle.can_edit(txn, user).await? {
                    return Err(ActivityError::PermissionDenied);
                }

                if matches!(puzzle.visibility, Visibility::Restricted)
                    && !matches!(visibility, common::objects::Visibility::Restricted)
                {
                    // We're derestricting the puzzle, so we should ensure there's at least one
                    // visible state
                    let states = puzzle.all_states(txn).await?;
                    if states
                        .iter()
                        .filter(|s| !matches!(s.visibility, Visibility::Restricted))
                        .count()
                        == 0
                    {
                        if let Some(state) =
                            in_view_state.and_then(|s| states.iter().find(|state| state.uuid == s))
                        {
                            state.set_visibility(txn, visibility.into()).await?;
                        } else {
                            return Err(ActivityError::InvalidInput);
                        }
                    }
                }

                let puzzle = puzzle.set_visibility(txn, visibility.into()).await?;

                Ok(puzzle)
            })
        })
        .await
}

#[tracing::instrument(skip_all)]
pub async fn set_state_visibility(
    conn: &mut crate::Connection,
    user: &str,
    puzzle: &str,
    state: &str,
    visibility: objects::Visibility,
) -> ActivityResult<models::Puzzle> {
    conn.build_transaction()
        .run(|txn| {
            Box::pin(async move {
                let puzzle = match Puzzle::by_uuid(txn, puzzle).await? {
                    Some(puzzle) => puzzle,
                    None => return Err(ActivityError::NotFound),
                };

                if !puzzle.can_edit(txn, user).await? {
                    return Err(ActivityError::PermissionDenied);
                }

                let puzzle_state = match PuzzleState::by_uuid(txn, state).await? {
                    Some(ps) => ps,
                    None => return Err(ActivityError::NotFound),
                };

                if puzzle_state.puzzle != puzzle.uuid {
                    return Err(ActivityError::NotFound);
                }

                puzzle_state.set_visibility(txn, visibility.into()).await?;

                Ok(puzzle)
            })
        })
        .await
}

#[tracing::instrument(skip_all)]
pub async fn edit_puzzle_tags(
    conn: &mut AsyncPgConnection,
    user: &str,
    puzzle: &str,
    to_add: &[String],
    to_remove: &[String],
) -> ActivityResult<models::Puzzle> {
    conn.build_transaction()
        .run(|txn| {
            Box::pin(async move {
                let puzzle = match Puzzle::by_uuid(txn, puzzle).await? {
                    Some(puzzle) => puzzle,
                    None => return Err(ActivityError::NotFound),
                };

                if !puzzle.can_edit(txn, user).await? {
                    return Err(ActivityError::PermissionDenied);
                }

                for tag in to_add {
                    puzzle.add_tag(txn, tag).await?;
                }

                for tag in to_remove {
                    puzzle.remove_tag(txn, tag).await?;
                }

                Ok(puzzle)
            })
        })
        .await
}
