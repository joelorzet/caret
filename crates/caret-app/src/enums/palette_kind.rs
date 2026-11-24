#[allow(unused_imports)]
use crate::prelude::*;

/// What choosing an entry does.
#[derive(Clone, PartialEq, Debug)]
pub enum PaletteKind {
    Run(Action),
    /// Applied as the selection moves, so the theme can be seen before it is
    /// chosen.
    Theme(String),
    /// A setting: where it lives in the file, its key and its value, so
    /// choosing it opens the file on that line rather than at the top.
    Setting {
        table: String,
        key: String,
        value: String,
    },
}
