#[allow(unused_imports)]
use crate::prelude::*;

/// Where keystrokes go.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FocusTarget {
    Panes,
    Explorer,
    Agent,
}
