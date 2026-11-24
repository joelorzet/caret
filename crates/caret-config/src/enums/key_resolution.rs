#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum KeyResolution {
    Run(Action),
    /// The prefix key was pressed; the next keypress is looked up in the
    /// prefixed table.
    PrefixPending,
    Unbound,
}
