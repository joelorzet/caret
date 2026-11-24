#[allow(unused_imports)]
use crate::prelude::*;

use std::path::PathBuf;

pub struct PtySpec {
    pub program: String,
    pub args: Vec<String>,
    pub cwd: Option<PathBuf>,
    /// Added on top of the inherited environment.
    pub env: Vec<(String, String)>,
    pub cols: u16,
    pub rows: u16,
}
