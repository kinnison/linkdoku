//! Help texts, for the various Markdown editors

pub const ROLE_DESCRIPTION: &str = concat!(
    include_str!("../help/general.md"),
    include_str!("../help/role.md"),
    "\n\n---\n"
);

pub const PUZZLE_DESCRIPTION: &str = concat!(
    include_str!("../help/general.md"),
    include_str!("../help/puzzle.md"),
    "\n\n---\n"
);
