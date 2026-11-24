#[allow(unused_imports)]
use crate::prelude::*;

impl Parser {
    pub fn new() -> Parser {
        Parser {
            machine: vte::Parser::new(),
        }
    }

    /// Applies `bytes` to `term`. Partial sequences are retained across calls,
    /// so a read that splits an escape sequence is handled correctly.
    pub fn advance(&mut self, term: &mut Terminal, bytes: &[u8]) {
        let mut performer = Performer { term };
        self.machine.advance(&mut performer, bytes);
    }
}

impl Default for Parser {
    fn default() -> Self {
        Parser::new()
    }
}
