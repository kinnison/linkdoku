//! Puzzle activities such as creation/updating of puzzles
//!

use common::objects;
use diesel_async::AsyncPgConnection;
use time::format_description::well_known::Iso8601;

use crate::{
    models::{self, Puzzle, Role, Visibility},
    utils::random_uuid,
};

use super::{ActivityError, ActivityResult};

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
                        serde_json::to_string(&initial_state.data)?,
                    )
                    .await?;
                Ok(puzzle)
            })
        })
        .await
}

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

                Ok(objects::Puzzle {
                    uuid: puzzle.uuid,
                    owner: puzzle.owner,
                    display_name: puzzle.display_name,
                    short_name: puzzle.short_name,
                    visibility: puzzle.visibility.into(),
                    created_at: puzzle.created_at.format(&Iso8601::DEFAULT)?,
                    updated_at: puzzle.updated_at.format(&Iso8601::DEFAULT)?,
                    states,
                })
            })
        })
        .await
}

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
                    None => return Err(ActivityError::UnknownShortName),
                };
                let puzzle = match Puzzle::by_short_name(txn, &role.uuid, puzzle).await? {
                    Some(puzzle) => puzzle,
                    None => return Err(ActivityError::UnknownShortName),
                };
                if !puzzle.can_be_seen(txn, user).await? {
                    return Err(ActivityError::UnknownShortName);
                }
                Ok(puzzle.uuid)
            })
        })
        .await
}
