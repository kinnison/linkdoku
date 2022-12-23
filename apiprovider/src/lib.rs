//! API Provider for Linkdoku
//!
//! The purpose of this crate is to encapsulate access to Linkdoku APIs,
//! in order to ensure that API access is as controlled as it can be.
//!
//! If it makes sense to add caching, it'll be in here, so that anything
//! accessing the API doesn't have to worry about it.

mod api;
mod backend;

pub use api::{use_apiprovider, APIProvider};
pub use backend::{ClientProvider, ClientProviderProps};
