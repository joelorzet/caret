#[allow(unused_imports)]
use crate::prelude::*;

/// How far a finished command's news travels.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum FinishNotice {
    /// Nothing leaves the pane it happened in.
    Off,
    /// The status line says so, which reaches you if you are looking at
    /// the window.
    #[default]
    Status,
    /// And a system notification when the window is not focused, which
    /// reaches you in another application.
    System,
}
