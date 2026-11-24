#[allow(unused_imports)]
use crate::prelude::*;

/// One setting, as it stands and as the file spells it.
///
/// The config file is the place to change things; this is the place to find
/// out what there is to change, and what it is set to now.
pub struct Setting {
    /// The table it lives under, which is also what it is about.
    pub section: &'static str,
    /// The table's name in the file, empty for a top-level setting. Not
    /// always the section: the sidebars read as `explorer` and `agent` but
    /// live under `[sidebar.explorer]` and `[sidebar.agent]`.
    pub table: &'static str,
    pub key: &'static str,
    pub value: String,
    pub about: &'static str,
}
