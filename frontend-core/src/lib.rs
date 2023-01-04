//! Core linkdoku frontend components

mod base;
pub mod component;
mod route;

pub use base::*;
pub use route::*;

pub fn make_title<S: AsRef<str>>(partial: S) -> String {
    format!("{} - Linkdoku", partial.as_ref())
}
