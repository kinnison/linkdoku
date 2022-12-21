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

#[derive(Serialize, Deserialize)]
pub enum APIError {
    Generic(String),
}

pub type APIResult<T> = std::result::Result<T, APIError>;
