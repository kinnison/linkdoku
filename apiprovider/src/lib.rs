//! API Provider for Linkdoku
//!
//! The purpose of this crate is to encapsulate access to Linkdoku APIs,
//! in order to ensure that API access is as controlled as it can be.
//!
//! If it makes sense to add caching, it'll be in here, so that anything
//! accessing the API doesn't have to worry about it.

mod api;
mod backend;
mod cache;

pub use api::{use_apiprovider, LinkdokuAPI};
pub use backend::{ClientProvider, ClientProviderProps};
pub use cache::*;
