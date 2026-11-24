#[allow(unused_imports)]
use crate::prelude::*;

/// One terminal: its screen state and the parser feeding it.
pub struct Pane {
    pub terminal: Terminal,
    pub parser: Parser,
    /// When the command now running started, if the shell said so.
    pub running_since: Option<std::time::Instant>,
    /// What the last command exited with, and how long it took.
    pub last_status: Option<i32>,
    pub last_duration: Option<std::time::Duration>,
}
