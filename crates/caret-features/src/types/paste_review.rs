#[allow(unused_imports)]
use crate::prelude::*;

/// What a paste would do, as far as the text itself can say.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct PasteReview {
    pub lines: usize,
    pub chars: usize,
    /// It ends with a newline, so the shell runs the last line without
    /// anybody pressing anything. This is how a copied snippet runs itself.
    pub runs_on_arrival: bool,
    /// Control characters other than tab and newline, by the names they go
    /// by, in the order they appear and without repeats.
    pub controls: Vec<&'static str>,
}
