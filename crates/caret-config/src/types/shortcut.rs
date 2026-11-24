#[allow(unused_imports)]
use crate::prelude::*;

/// One row of the shortcut list: every key that runs an action.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Shortcut {
    pub keys: Vec<String>,
    pub action: Action,
    pub description: String,
}
