#[allow(unused_imports)]
use crate::prelude::*;

use std::path::PathBuf;

/// One face within a family: a particular weight and slant.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FontFace {
    pub family: String,
    pub post_script_name: String,
    pub slant: FontSlant,
    /// CSS-style weight, where 400 is regular and 700 bold.
    pub weight: u16,
    pub monospaced: bool,
    /// Absent for fonts loaded from memory rather than from disk.
    pub path: Option<PathBuf>,
    /// Index within a collection file such as a `.ttc`.
    pub index: u32,
}
