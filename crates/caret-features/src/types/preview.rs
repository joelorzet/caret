#[allow(unused_imports)]
use crate::prelude::*;

use std::path::PathBuf;

/// A read-only look at a file: numbered, coloured, and scrollable.
pub struct Preview {
    pub(crate) path: PathBuf,
    pub(crate) language: Language,
    pub(crate) lines: Vec<String>,
    /// One token list per line, worked out once when the file is opened.
    pub(crate) tokens: Vec<Vec<Token>>,
    pub(crate) cursor: usize,
    pub(crate) offset: usize,
    pub(crate) height: usize,
    /// Where a line selection started, if one is being made.
    pub(crate) anchor: Option<usize>,
    pub(crate) truncated: bool,
    pub(crate) error: Option<String>,
}
