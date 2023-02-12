//! Puzzle utilities for Linkdoku

pub mod ctc;
pub mod fpuzzles;
pub mod xform;

#[derive(Debug)]
pub struct GridMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub rules: Option<String>,
    pub rows_cols: Option<(usize, usize)>,
    pub has_solution: bool,
}
