#[allow(unused_imports)]
use crate::prelude::*;

use std::path::PathBuf;

/// A directory browser driven entirely by the arrow keys.
///
/// Up and down move the selection, right enters a directory, left goes back to
/// the parent. The whole thing is plain filesystem reads, so it costs no
/// process and no thread.
pub struct Explorer {
    pub(crate) root: PathBuf,
    pub(crate) entries: Vec<Entry>,
    pub(crate) selected: usize,
    /// First visible row, so the selection can scroll inside the panel.
    pub(crate) offset: usize,
    pub(crate) height: usize,
    pub(crate) show_hidden: bool,
    /// Set when the directory could not be read.
    pub(crate) error: Option<String>,
}
