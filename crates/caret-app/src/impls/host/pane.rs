#[allow(unused_imports)]
use crate::prelude::*;

impl Pane {
    pub fn new(cols: u16, rows: u16, scrollback: u32, cursor: CursorShape) -> Pane {
        let mut terminal = Terminal::new(cols, rows, scrollback);
        // The config sets what the cursor looks like until an application
        // says otherwise with DECSCUSR.
        terminal.set_cursor_style(CursorStyle {
            shape: cursor,
            ..terminal.cursor_style()
        });
        Pane {
            terminal,
            parser: Parser::new(),
            running_since: None,
            last_status: None,
            last_duration: None,
        }
    }

    /// Feeds PTY output through the parser, returning anything the terminal
    /// wants written back (cursor reports, device attributes).
    pub fn consume(&mut self, bytes: &[u8]) -> Vec<u8> {
        self.parser.advance(&mut self.terminal, bytes);
        self.terminal.take_reply()
    }

    /// A short label for the tab bar and status line.
    pub fn label(&self, fallback: &str) -> String {
        let title = self.terminal.title();
        if title.is_empty() {
            fallback.to_string()
        } else {
            title.to_string()
        }
    }
}
