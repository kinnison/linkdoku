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

use serde::{Deserialize, Serialize};

pub mod internal;
pub mod public;

#[derive(Serialize, Deserialize, Debug)]
pub enum APIError {
    /// Client problem, only generated client-side, never returned from the server
    ClientIssue(String),
    /// Generic problem, rarely returned
    Generic(String),
    /// Generic database error, should not be returned
    DatabaseError(String),
    /// Unknown login provider (could be raised at any point in the login process)
    UnknownLoginProvider(String),
    /// Bad login state token
    BadLoginStateToken,
    /// Code exchange failure
    LoginCodeExchangeFailed,
    /// Login flow got an error from the OIDP
    LoginFlowError(String),
    /// Login flow failed to get identity token
    NoIdentityToken,
    /// Login flow produced a bad identity token
    BadIdentityToken,
}

// Every API call possible will return APIResult<Response>
// where the response is the response type
pub type APIResult<T> = std::result::Result<T, APIError>;
