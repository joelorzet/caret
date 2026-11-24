#[allow(unused_imports)]
use crate::prelude::*;

/// The command line as it stands on the grid.
pub struct TypedLine {
    pub pane: PaneId,
    /// The row within the pane's screen, not the window.
    pub row: u16,
    pub text: String,
    /// The column each character of `text` was echoed into.
    pub columns: Vec<u16>,
}
