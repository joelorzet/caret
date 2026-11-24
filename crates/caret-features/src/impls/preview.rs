#[allow(unused_imports)]
use crate::prelude::*;

use std::path::{Path, PathBuf};

/// Largest file worth reading into memory to look at.
const MAX_BYTES: usize = 2 * 1024 * 1024;
/// Lines beyond this are dropped; a preview is not a pager.
const MAX_LINES: usize = 20_000;

impl Preview {
    pub fn open(path: &Path) -> Preview {
        let language = Language::of(path);
        let mut preview = Preview {
            path: path.to_path_buf(),
            language,
            lines: Vec::new(),
            tokens: Vec::new(),
            cursor: 0,
            offset: 0,
            height: 10,
            anchor: None,
            truncated: false,
            error: None,
        };

        match std::fs::read(path) {
            Ok(bytes) => preview.load(bytes),
            Err(e) => preview.error = Some(e.to_string()),
        }
        preview
    }

    fn load(&mut self, bytes: Vec<u8>) {
        let truncated = bytes.len() > MAX_BYTES;
        let bytes = if truncated {
            &bytes[..MAX_BYTES]
        } else {
            &bytes[..]
        };

        // A NUL byte early on means this is not text; showing it would spray
        // control characters through the panel.
        if bytes.iter().take(8192).any(|byte| *byte == 0) {
            self.error = Some("binary file".to_string());
            return;
        }

        let text = String::from_utf8_lossy(bytes);
        let mut state = HighlightState::default();
        for line in text.lines().take(MAX_LINES) {
            // Tabs would misalign every column after them.
            let line = line.replace('\t', "    ");
            self.tokens.push(self.language.highlight(&line, &mut state));
            self.lines.push(line);
        }
        self.truncated = truncated || text.lines().count() > MAX_LINES;
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn language(&self) -> Language {
        self.language
    }

    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    pub fn truncated(&self) -> bool {
        self.truncated
    }

    pub fn len(&self) -> usize {
        self.lines.len()
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn set_height(&mut self, height: usize) {
        self.height = height.max(1);
        self.scroll_into_view();
    }

    /// The lines on screen, with their numbers and tokens.
    pub fn visible(&self) -> impl Iterator<Item = (usize, &str, &[Token])> {
        self.lines
            .iter()
            .enumerate()
            .skip(self.offset)
            .take(self.height)
            .map(|(index, line)| (index, line.as_str(), self.tokens[index].as_slice()))
    }

    /// The selected line range, inclusive. Always at least the cursor line.
    pub fn selection(&self) -> (usize, usize) {
        match self.anchor {
            Some(anchor) if anchor <= self.cursor => (anchor, self.cursor),
            Some(anchor) => (self.cursor, anchor),
            None => (self.cursor, self.cursor),
        }
    }

    pub fn is_selecting(&self) -> bool {
        self.anchor.is_some()
    }

    /// The selected lines as text, ready for the clipboard.
    pub fn selected_text(&self) -> String {
        let (start, end) = self.selection();
        self.lines
            .get(start..=end.min(self.lines.len().saturating_sub(1)))
            .unwrap_or_default()
            .join("\n")
    }

    pub fn handle(&mut self, key: PanelKey) -> PanelOutcome {
        let page = self.height.saturating_sub(1).max(1) as isize;
        let half = (self.height / 2).max(1) as isize;

        match key {
            PanelKey::Escape | PanelKey::Left => PanelOutcome::Dismiss,

            // Arrows and the vim keys do the same thing, so neither hand has
            // to learn the other's.
            PanelKey::Up | PanelKey::Char('k') => self.move_by(-1),
            PanelKey::Down | PanelKey::Char('j') => self.move_by(1),
            PanelKey::PageUp => self.move_by(-page),
            PanelKey::PageDown | PanelKey::Char(' ') => self.move_by(page),
            PanelKey::HalfPageUp => self.move_by(-half),
            PanelKey::HalfPageDown => self.move_by(half),
            PanelKey::Home | PanelKey::Char('g') => self.move_to(0),
            PanelKey::End | PanelKey::Char('G') => self.move_to(self.last()),

            // Shift and an arrow starts a selection without a mode to enter.
            PanelKey::SelectUp => {
                self.begin_selection();
                self.move_by(-1)
            }
            PanelKey::SelectDown => {
                self.begin_selection();
                self.move_by(1)
            }

            PanelKey::Char('v') => {
                self.anchor = match self.anchor {
                    Some(_) => None,
                    None => Some(self.cursor),
                };
                PanelOutcome::Redraw
            }
            PanelKey::Char('a') => {
                self.anchor = Some(0);
                self.move_to(self.last())
            }
            PanelKey::Char('y') | PanelKey::Enter => {
                let text = self.selected_text();
                self.anchor = None;
                if text.is_empty() {
                    PanelOutcome::Redraw
                } else {
                    PanelOutcome::Copy(text)
                }
            }
            PanelKey::Char('e') => PanelOutcome::OpenEditor(self.path.clone()),
            PanelKey::Char('q') => PanelOutcome::Dismiss,
            _ => PanelOutcome::Ignored,
        }
    }

    fn last(&self) -> usize {
        self.lines.len().saturating_sub(1)
    }

    fn begin_selection(&mut self) {
        if self.anchor.is_none() {
            self.anchor = Some(self.cursor);
        }
    }

    /// Puts the cursor on a line, for a click.
    pub fn click(&mut self, line: usize) -> PanelOutcome {
        self.anchor = None;
        self.move_to(line + self.offset)
    }

    /// Extends the selection to a line, for a drag.
    pub fn drag(&mut self, line: usize) -> PanelOutcome {
        self.begin_selection();
        self.move_to(line + self.offset)
    }

    /// Scrolls without moving the cursor, for the wheel.
    pub fn scroll(&mut self, delta: isize) -> PanelOutcome {
        if self.lines.is_empty() {
            return PanelOutcome::Ignored;
        }
        let max = self.lines.len().saturating_sub(self.height) as isize;
        self.offset = (self.offset as isize + delta).clamp(0, max.max(0)) as usize;
        PanelOutcome::Redraw
    }

    /// How many lines are selected, for the panel to say so.
    pub fn selected_lines(&self) -> usize {
        let (start, end) = self.selection();
        end.saturating_sub(start) + 1
    }

    fn move_by(&mut self, delta: isize) -> PanelOutcome {
        if self.lines.is_empty() {
            return PanelOutcome::Ignored;
        }
        let last = self.lines.len() as isize - 1;
        let next = (self.cursor as isize + delta).clamp(0, last) as usize;
        self.move_to(next)
    }

    fn move_to(&mut self, line: usize) -> PanelOutcome {
        if self.lines.is_empty() {
            return PanelOutcome::Ignored;
        }
        self.cursor = line.min(self.lines.len() - 1);
        self.scroll_into_view();
        PanelOutcome::Redraw
    }

    fn scroll_into_view(&mut self) {
        if self.cursor < self.offset {
            self.offset = self.cursor;
        } else if self.cursor >= self.offset + self.height {
            self.offset = self.cursor + 1 - self.height;
        }
        self.offset = self
            .offset
            .min(self.lines.len().saturating_sub(self.height));
    }
}
