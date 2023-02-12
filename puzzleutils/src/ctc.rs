//! utilities for CTC encoded puzzles
//!

// CTC encoded puzzles are like fpuzzles encoding, but with a little more complexity
// we have the lzstr compression, but then there is an encoding applied to the JSON
// which makes the puzzles smaller.

mod encode;
mod parse;

pub use encode::*;
pub use parse::*;
use serde_json::Value;

use crate::GridMetadata;

fn find_entry<'a>(value: &'a Value, name: &'_ str) -> Option<&'a Value> {
    value.as_object().and_then(|m| m.get(name))
}

fn find_kvsub(value: &Value, what: &str) -> Option<String> {
    find_entry(value, "ca")
        .and_then(Value::as_array)
        .and_then(|arr| {
            for v in arr {
                if let Some(value) = v
                    .as_object()
                    .and_then(|m| m.get("v"))
                    .and_then(Value::as_str)
                    .and_then(|s| s.strip_prefix(what))
                {
                    return Some(value.to_string());
                }
            }
            None
        })
}

pub fn metadata(value: &Value) -> GridMetadata {
    let cells = find_entry(value, "ce")
        .and_then(Value::as_array)
        .and_then(|rows| rows[0].as_array().map(|cols| (rows.len(), cols.len())));

    GridMetadata {
        title: find_kvsub(value, "title: "),
        author: find_kvsub(value, "author: "),
        rules: find_kvsub(value, "rules: "),
        rows_cols: cells,
        has_solution: find_kvsub(value, "solution: ").is_some(),
    }
}
