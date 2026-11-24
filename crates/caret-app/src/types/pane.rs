#[allow(unused_imports)]
use crate::prelude::*;

/// One terminal: its screen state and the parser feeding it.
pub struct Pane {
    pub terminal: Terminal,
    pub parser: Parser,
}
