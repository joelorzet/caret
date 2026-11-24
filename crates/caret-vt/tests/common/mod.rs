// Each integration test binary compiles this separately.
#![allow(dead_code)]

use caret_core::Terminal;
use caret_vt::Parser;

pub struct Vt {
    pub term: Terminal,
    pub parser: Parser,
}

impl Vt {
    pub fn new(cols: u16, rows: u16) -> Vt {
        Vt::with_scrollback(cols, rows, 0)
    }

    pub fn with_scrollback(cols: u16, rows: u16, scrollback: u32) -> Vt {
        Vt {
            term: Terminal::new(cols, rows, scrollback),
            parser: Parser::new(),
        }
    }

    pub fn feed(&mut self, bytes: &str) -> &mut Vt {
        self.parser.advance(&mut self.term, bytes.as_bytes());
        self
    }

    /// Viewport row `y` as text, trailing blanks trimmed.
    pub fn row(&self, y: u16) -> String {
        let base = self.term.screen().history_len();
        self.term.screen().row_text(base + y as u32)
    }

    pub fn rows(&self) -> Vec<String> {
        (0..self.term.rows()).map(|y| self.row(y)).collect()
    }

    pub fn cursor(&self) -> (u16, u16) {
        let c = self.term.screen().cursor();
        (c.x, c.y)
    }

    pub fn cell(&self, x: u16, y: u16) -> caret_core::Cell {
        self.term.screen().line(y)[x as usize]
    }

    pub fn reply(&mut self) -> String {
        String::from_utf8_lossy(&self.term.take_reply()).into_owned()
    }
}
