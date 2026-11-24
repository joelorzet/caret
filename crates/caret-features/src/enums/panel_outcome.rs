#[allow(unused_imports)]
use crate::prelude::*;

use std::path::PathBuf;

/// What the host should do after a key reached a panel.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum PanelOutcome {
    /// Handled; the panel's own state changed.
    Redraw,
    /// Nothing to do with this key.
    Ignored,
    /// Give focus back to the terminal panes.
    Dismiss,
    /// Insert this path into the focused pane's input.
    Insert(PathBuf),
    /// Open a new pane with this directory as its working directory.
    OpenPane(PathBuf),
    /// Open a new tab there instead.
    OpenTab(PathBuf),
    /// Show this file in the preview panel.
    Preview(PathBuf),
    /// Put this text on the clipboard.
    Copy(String),
    /// Open this file in the configured editor.
    OpenEditor(PathBuf),
}
