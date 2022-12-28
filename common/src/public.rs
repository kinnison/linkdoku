//! These are public APIs which can be used to interrogate Linkdoku directly
//! about various things.  A public API is considered stable and has some
//! amount of version numbering behind it.

pub mod role;
pub mod userinfo;

pub const PUBLIC_SEGMENT: &str = "/public";
