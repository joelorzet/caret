#[allow(unused_imports)]
use crate::prelude::*;

use std::path::PathBuf;

/// What a pane is to start.
pub struct Launch {
    pub program: String,
    pub args: Vec<String>,
    /// What the program is told its own name is. A shell started as `-zsh`
    /// reads the login profile, which is where most of a PATH comes from.
    pub arg0: Option<String>,
    /// `None` inherits the focused pane's directory.
    pub cwd: Option<PathBuf>,
}
