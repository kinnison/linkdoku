//! Utility functions for use in this database
//!

const KIND_MARKER: &str = "KIND\0";
const ID_MARKER: &str = "\0ID\0";
const SALT_MARKER: &str = "\0SALT\0";
const RANDOM_MARKER: &str = "\0RANDOM\0";

pub fn uuid(kind: &str, identifier: &str, salt: &str) -> String {
    let mut state = md5::Context::new();
    state.consume(KIND_MARKER);
    state.consume(kind);
    state.consume(ID_MARKER);
    state.consume(identifier);
    state.consume(SALT_MARKER);
    state.consume(salt);
    format!("{:x}", state.compute())
}

pub fn random_uuid(kind: &str) -> String {
    let mut state = md5::Context::new();
    state.consume(KIND_MARKER);
    state.consume(kind);
    state.consume(RANDOM_MARKER);
    state.consume(rand::random::<[u8; 16]>());
    format!("{:x}", state.compute())
}
