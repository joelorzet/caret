#[allow(unused_imports)]
use crate::prelude::*;

/// A run of a command line, in character offsets so it lines up with the
/// grid the shell echoed it onto.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ShellSpan {
    pub start: usize,
    pub end: usize,
    pub role: ShellRole,
}
