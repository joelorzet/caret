#[allow(unused_imports)]
use crate::prelude::*;

use std::path::PathBuf;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Entry {
    pub name: String,
    pub path: PathBuf,
    pub kind: EntryKind,
}
