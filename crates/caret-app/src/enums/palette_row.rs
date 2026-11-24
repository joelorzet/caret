#[allow(unused_imports)]
use crate::prelude::*;

/// A line of the palette's list.
///
/// The topic is said once, above the entries under it, rather than repeated
/// against every one of them.
pub enum PaletteRow {
    /// A topic, and how many entries are under it.
    Heading(String, usize),
    /// An index into the palette's items.
    Entry(usize),
}

impl PaletteRow {
    pub fn entry(&self) -> Option<usize> {
        match self {
            PaletteRow::Entry(index) => Some(*index),
            PaletteRow::Heading(..) => None,
        }
    }
}
