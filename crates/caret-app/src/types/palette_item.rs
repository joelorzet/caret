#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct PaletteItem {
    /// What this is about: the group a command belongs to, or the table a
    /// setting lives under. Shown beside the row and matched on, so a long
    /// list can be narrowed to one topic by typing its name.
    pub section: String,
    pub label: String,
    /// Shown dim on the right: the keys, or a description.
    pub hint: String,
    pub kind: PaletteKind,
}
