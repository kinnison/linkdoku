//! Linkdoku common types (API types primarily)
//!
//! This crate exists primarily to model the API
//! which exists between Linkdoku backend and frontend.
//!
//! The API is split into two parts, a "public" API which
//! is intended to be stable and provided to other users of
//! linkdoku.  The "private" API is public in the sense that
//! linkdoku is free software, but it is only intended to be
//! used by the frontend and makes no stability guarantees.

use std::fmt;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[cfg(feature = "backend")]
mod impls;
pub mod internal;
pub mod objects;
pub mod public;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Error)]
pub enum APIError {
    /// Client problem, only generated client-side, never returned from the server
    #[error("{0}")]
    ClientIssue(String),
    /// Generic problem, rarely returned
    #[error("{0}")]
    Generic(String),
    /// A bad shortname was selected
    #[error("Bad short name, {0}")]
    BadShortName(BadShortNameReason),
    /// Whatever was asked for was not found (we locally transform this from a 404 in the client)
    #[error("Object not found")]
    ObjectNotFound,
    /// Whatever you asked to do, you're not permitted to
    #[error("Permission denied")]
    PermissionDenied,
    /// You tried to do something, but the input was malformed
    #[error("Malformed input")]
    BadInput,
    /// Generic database error, should not be returned
    #[error("Database error: {0}")]
    DatabaseError(String),
    /// Unknown login provider (could be raised at any point in the login process)
    #[error("Unknown Login provider: {0}")]
    UnknownLoginProvider(String),
    /// Bad login state token
    #[error("Bad login state token during exchange")]
    BadLoginStateToken,
    /// Code exchange failure
    #[error("Login code exchange failed")]
    LoginCodeExchangeFailed,
    /// Login flow got an error from the OIDP
    #[error("OIDP error: {0}")]
    LoginFlowError(String),
    /// Login flow failed to get identity token
    #[error("OIDP did not return identity token")]
    NoIdentityToken,
    /// Login flow produced a bad identity token
    #[error("OIDP returned a bad identity token")]
    BadIdentityToken,
    /// Unable to create puzzle shortcut for some reason
    #[error("Cannot create puzzle shortcut")]
    CannotCreatePuzzleShortcut,
}

// Every API call possible will return APIResult<Response>
// where the response is the response type
pub type APIResult<T> = std::result::Result<T, APIError>;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum APIOutcome<T> {
    Success(T),
    Error(APIError),
}

impl<T> From<APIResult<T>> for APIOutcome<T> {
    fn from(value: APIResult<T>) -> Self {
        match value {
            Ok(v) => APIOutcome::Success(v),
            Err(e) => APIOutcome::Error(e),
        }
    }
}

impl<T> From<APIOutcome<T>> for APIResult<T> {
    fn from(value: APIOutcome<T>) -> Self {
        match value {
            APIOutcome::Success(v) => Ok(v),
            APIOutcome::Error(e) => Err(e),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum BadShortNameReason {
    NotUnique,
    TooShort,
    TooLong,
    InvalidCharacter,
    ReservedWord,
}

impl fmt::Display for BadShortNameReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotUnique => write!(f, "name is not unique"),
            Self::TooShort => write!(f, "minimum length is 3 characters"),
            Self::TooLong => write!(f, "maximum length is 32 characters"),
            Self::InvalidCharacter => write!(
                f,
                "only basic (not accented) letters, numbers, underscore, and hyphen are permitted"
            ),
            Self::ReservedWord => write!(f, "you may not use a reserved word"),
        }
    }
}

pub fn clean_short_name(name: &str, skip_bad_chars: bool) -> Result<String, BadShortNameReason> {
    let mut ret = String::new();
    for ch in name.chars() {
        match ch {
            '-' | '_' | '0'..='9' | 'a'..='z' => ret.push(ch),
            'A'..='Z' => ret.push(ch.to_ascii_lowercase()),
            _ => {
                if !skip_bad_chars {
                    return Err(BadShortNameReason::InvalidCharacter);
                }
            }
        }
    }
    match ret {
        _ if ret.len() < 3 => Err(BadShortNameReason::TooShort),
        _ if ret.len() > 32 => Err(BadShortNameReason::TooLong),
        _ => Ok(ret),
    }
}
