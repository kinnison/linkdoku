//! Tutorials system for Linkdoku
//!
//!
//! Each tutorial consists of some number of points on a page which need to have the attention
//! of the user of the website drawn to.  We only want to show tutorials the first time a user
//! sees a particular page, or rather we give them a chance to tell us they don't want them
//! when they see the page.
//!
//! Tutorials are purely in-browser though, so they're never rendered for hydration.  Instead
//! tutorials are created as components which `use_effect()` in order to work.

mod components;
mod data;
mod macros;

pub use components::*;
pub use data::TutorialData;
