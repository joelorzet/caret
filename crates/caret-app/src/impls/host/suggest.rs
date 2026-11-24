#[allow(unused_imports)]
use crate::prelude::*;

/// Most suggestions shown under the cursor at once.
pub(crate) const SUGGESTION_ROWS: usize = 5;

impl Session {
    pub(crate) fn history_path() -> Option<std::path::PathBuf> {
        caret_config::paths::config_dir().map(|dir| dir.join("history"))
    }

    /// Notes where the command line starts, the first time something is typed
    /// at a fresh prompt.
    pub(crate) fn anchor_input(&mut self) {
        if self.input_anchor.is_some() {
            return;
        }
        let id = self.workspace.active_pane();
        let Some(pane) = self.pane(id) else { return };
        let screen = pane.terminal.screen();
        // An application using the alternate screen is not a shell prompt.
        if pane.terminal.is_alt() {
            return;
        }
        let cursor = screen.cursor();
        let row = screen.history_len() + cursor.y as u32;
        self.input_anchor = Some((id, row, cursor.x));
    }

    pub(crate) fn drop_anchor(&mut self) {
        self.input_anchor = None;
        self.autocomplete.clear();
    }

    /// Reads the command line off the grid and refreshes the suggestions.
    pub(crate) fn sync_suggestions(&mut self) {
        self.collect_pending_record();

        let Some(text) = self.current_input() else {
            if self.autocomplete.is_active() {
                self.needs_draw = true;
            }
            self.drop_anchor();
            return;
        };
        if self.autocomplete.set_input(&text) {
            self.needs_draw = true;
        }
    }

    /// What has been typed since the anchor, or `None` if the anchor no
    /// longer describes where the cursor is.
    pub(crate) fn current_input(&self) -> Option<String> {
        self.input_on_screen().map(|input| input.text)
    }

    /// The same line, with where it sits on the grid.
    ///
    /// Colouring it means writing to the very cells the shell echoed it
    /// onto, so the columns come from the same walk that reads the text:
    /// counting characters and counting cells part company the moment a
    /// wide glyph appears.
    pub(crate) fn input_on_screen(&self) -> Option<TypedLine> {
        let (id, row, col) = self.input_anchor?;
        if id != self.workspace.active_pane() {
            return None;
        }
        let pane = self.pane(id)?;
        if pane.terminal.is_alt() {
            return None;
        }

        let screen = pane.terminal.screen();
        let base = screen.history_len();
        // The line scrolled away, or the cursor moved off it.
        let y = row.checked_sub(base)?;
        let cursor = screen.cursor();
        if y != cursor.y as u32 || cursor.x < col {
            return None;
        }

        let line = screen.line(cursor.y);
        let cells = line.get(col as usize..cursor.x as usize)?;

        let mut text = String::with_capacity(cells.len());
        let mut columns = Vec::with_capacity(cells.len());
        for (offset, cell) in cells.iter().enumerate() {
            if cell.flags.contains(Flags::WIDE_TAIL) {
                continue;
            }
            text.push(cell.ch);
            columns.push(col + offset as u16);
        }

        Some(TypedLine {
            pane: id,
            row: cursor.y,
            text,
            columns,
        })
    }

    /// Accepts the suggestion by typing the rest of it into the shell.
    ///
    /// Sending the characters rather than rewriting the line keeps the shell's
    /// own editor in charge of its buffer.
    pub(crate) fn accept_suggestion(&mut self) -> bool {
        let Some(remainder) = self.autocomplete.remainder().map(str::to_string) else {
            return false;
        };
        let id = self.workspace.active_pane();
        let _ = self.ptys.write(PtyId(id.0), remainder.as_bytes());
        self.needs_draw = true;
        true
    }

    /// Marks the line for recording once the shell has finished echoing it.
    ///
    /// Reading it at the moment Enter is pressed loses anything still in
    /// flight: keys sent in a burst, or a pasted command, are echoed back
    /// after we have already sent the newline.
    pub(crate) fn record_command(&mut self) {
        self.pending_record = self.input_anchor;
        self.drop_anchor();
    }

    /// Takes the submitted line off the grid once the cursor has left it.
    fn collect_pending_record(&mut self) {
        let Some((id, row, col)) = self.pending_record else {
            return;
        };
        let Some(pane) = self.pane(id) else {
            self.pending_record = None;
            return;
        };

        let screen = pane.terminal.screen();
        let base = screen.history_len();
        let Some(y) = row.checked_sub(base) else {
            // Scrolled out of reach before it could be read.
            self.pending_record = None;
            return;
        };
        // Still on the line: the shell has not finished with it yet.
        if y == screen.cursor().y as u32 {
            return;
        }

        let command = screen
            .row_text(row)
            .chars()
            .skip(col as usize)
            .collect::<String>()
            .trim()
            .to_string();
        self.pending_record = None;

        if command.is_empty() {
            return;
        }
        self.autocomplete.record(&command);
        if let Some(path) = Session::history_path() {
            let _ = caret_features::history::append(&path, &command);
        }
    }

    /// True when the cursor sits at the end of what has been typed, which is
    /// the only place completing makes sense.
    pub(crate) fn at_input_end(&self) -> bool {
        self.current_input().is_some()
    }
}
