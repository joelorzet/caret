#[allow(unused_imports)]
use crate::prelude::*;

/// A filterable list of things to do.
///
/// Replaces the static help: reading a wall of bindings tells you what exists,
/// but typing two letters and pressing enter is how you actually use it.
pub struct Palette {
    pub(crate) mode: PaletteMode,
    pub(crate) query: String,
    pub(crate) items: Vec<PaletteItem>,
    /// The list as it is shown: a heading for each topic that still has
    /// entries, and the entries under it, best match first.
    pub(crate) rows: Vec<PaletteRow>,
    /// An index into `rows`, always pointing at an entry.
    pub(crate) selected: usize,
    pub(crate) offset: usize,
    pub(crate) height: usize,
    /// The command whose shortcut is being replaced, and what to call it.
    /// While this is set the next key pressed becomes its binding.
    pub(crate) capturing: Option<(Action, String)>,
    /// Theme to put back if the picker is cancelled.
    pub(crate) restore: Option<String>,
}
