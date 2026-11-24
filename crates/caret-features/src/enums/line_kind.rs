#[allow(unused_imports)]
use crate::prelude::*;

/// What a rendered line represents. The host maps these onto theme colours so
/// this crate never needs to know about themes.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LineKind {
    Header,
    Directory,
    File,
    Executable,
    Symlink,
    Hint,
    Error,
}
