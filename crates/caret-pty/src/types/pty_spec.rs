#[allow(unused_imports)]
use crate::prelude::*;

use std::path::PathBuf;

pub struct PtySpec {
    pub program: String,
    pub args: Vec<String>,
    /// What the program sees as its own name.
    ///
    /// A shell started as `-zsh` is a login shell, and reads the profile
    /// where most people's PATH is set. Terminals have launched their
    /// first shell that way since the beginning, which is why a login
    /// profile is where tools expect to put themselves.
    pub arg0: Option<String>,
    pub cwd: Option<PathBuf>,
    /// Added on top of the inherited environment.
    pub env: Vec<(String, String)>,
    pub cols: u16,
    pub rows: u16,
}
