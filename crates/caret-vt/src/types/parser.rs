#[allow(unused_imports)]
use crate::prelude::*;

/// Feeds PTY bytes into a [`Terminal`].
pub struct Parser {
    pub(crate) machine: vte::Parser,
}
