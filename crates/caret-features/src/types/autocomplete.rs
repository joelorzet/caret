#[allow(unused_imports)]
use crate::prelude::*;

/// Inline command suggestions drawn from what you have run before.
///
/// Terminal-level rather than shell-level: it watches what is typed and
/// completed, so it works the same under zsh, bash or fish and needs no shell
/// integration.
pub struct Autocomplete {
    /// Oldest first, so the newest match is found by searching backwards.
    pub(crate) history: Vec<String>,
    pub(crate) input: String,
    /// Indices into `history`, newest first.
    pub(crate) matches: Vec<usize>,
    pub(crate) selected: usize,
    pub(crate) limit: usize,
}
