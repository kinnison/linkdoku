//! Common objects transacted by the various APIs

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Roles are owners of puzzles etc.
///
/// Roles have owners themselves (identities) and names, descriptions, etc.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Role {
    pub uuid: String,
    pub owner: String,
    pub short_name: String,
    pub display_name: String,
    pub description: String,
}

impl Role {
    pub fn can_edit(&self, identity: &str) -> bool {
        self.owner == identity
    }

    pub fn can_add_puzzles(&self, identity: &str) -> bool {
        self.owner == identity
    }
}

/// Puzzle data, this is only ever returned as part of [PuzzleState]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum PuzzleData {
    #[default]
    Nothing,
    URLs(Vec<UrlEntry>),
    Pack(Vec<String>),
    FPuzzles(Value),
}

/// URL Entries for URL list puzzle states, only ever part of [PuzzleData]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UrlEntry {
    pub title: String,
    pub url: String,
}

/// What visibility does a [PuzzleState] or [Puzzle] have?
///
/// For someone to see a puzzle at all, the puzzle's visibility must be
/// at least [Visibility::Public].  The default visibility means that only
/// the owner (TODO: and those granted access) can see things.
///
/// Even though a puzzle may be visible to someone, they may be unable to see
/// some or all [PuzzleState]s depending on their [Visibility].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Visibility {
    #[default]
    /// Only owner (TODO: and those granted access) can see this
    Restricted,
    /// Anyone who knows where to find it can see this
    Public,
    /// This is both [Visibility::Public] and also listed on role pages etc.
    Published,
}
/// A state for a [Puzzle], this is only ever part of a Puzzle in the basic
/// objects API.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PuzzleState {
    pub uuid: String,
    pub description: String,
    pub data: PuzzleData,
    pub visibility: Visibility,
    pub updated_at: String,
}

/// Puzzles are what Linkdoku is all about.
///
/// Every puzzle has a short name, a display name, some other metadata,
/// and then a list of [PuzzleState]s the most recent published
/// one being the one which is considered the current state of the
/// puzzle.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Puzzle {
    pub uuid: String,
    pub owner: String,
    pub display_name: String,
    pub short_name: String,
    pub visibility: Visibility,
    pub created_at: String,
    pub updated_at: String,
    pub states: Vec<PuzzleState>,
}

/// Tags are present on puzzles and indicate some kind of basic
/// metadata which a user might care about such as if a puzzle is good
/// for streaming.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tag {
    pub uuid: String,
    pub name: String,
    pub colour: String,
    pub black_text: bool,
}
