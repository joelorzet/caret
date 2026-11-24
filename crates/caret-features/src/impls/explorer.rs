#[allow(unused_imports)]
use crate::prelude::*;

use std::path::{Path, PathBuf};

impl Explorer {
    pub fn new(root: impl Into<PathBuf>, show_hidden: bool) -> Explorer {
        let mut explorer = Explorer {
            root: root.into(),
            entries: Vec::new(),
            selected: 0,
            offset: 0,
            height: 1,
            show_hidden,
            error: None,
        };
        explorer.refresh();
        explorer
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }

    pub fn selected(&self) -> Option<&Entry> {
        self.entries.get(self.selected)
    }

    pub fn show_hidden(&self) -> bool {
        self.show_hidden
    }

    /// Rows available for the listing, so paging and scrolling match what is
    /// on screen.
    pub fn set_height(&mut self, height: usize) {
        self.height = height.max(1);
        self.scroll_into_view();
    }

    /// Moves to `path`, keeping the selection on the directory just left when
    /// going upwards so walking back out lands where you were.
    pub fn set_root(&mut self, path: impl Into<PathBuf>) {
        let path = path.into();
        if path == self.root {
            return;
        }
        let came_from = self
            .root
            .starts_with(&path)
            .then(|| {
                self.root
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
            })
            .flatten();

        self.root = path;
        self.refresh();

        if let Some(name) = came_from {
            if let Some(at) = self.entries.iter().position(|e| e.name == name) {
                self.selected = at;
                self.scroll_into_view();
            }
        }
    }

    pub fn toggle_hidden(&mut self) {
        self.show_hidden = !self.show_hidden;
        let keep = self.selected().map(|e| e.name.clone());
        self.refresh();
        if let Some(name) = keep {
            if let Some(at) = self.entries.iter().position(|e| e.name == name) {
                self.selected = at;
            }
        }
        self.scroll_into_view();
    }

    pub fn refresh(&mut self) {
        self.error = None;
        self.entries.clear();

        let read = match std::fs::read_dir(&self.root) {
            Ok(read) => read,
            Err(e) => {
                self.error = Some(e.to_string());
                self.selected = 0;
                self.offset = 0;
                return;
            }
        };

        for entry in read.flatten() {
            let path = entry.path();
            let hidden = path
                .file_name()
                .map(|n| n.to_string_lossy().starts_with('.'))
                .unwrap_or(false);
            if hidden && !self.show_hidden {
                continue;
            }
            if let Some(entry) = Entry::read(&path) {
                self.entries.push(entry);
            }
        }

        self.entries.sort_by_key(|e| e.order());
        self.selected = self.selected.min(self.entries.len().saturating_sub(1));
        self.scroll_into_view();
    }

    /// First visible row, so a click can be mapped back to an entry.
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Selects by index, for a click rather than a keypress.
    pub fn select(&mut self, index: usize) -> bool {
        if index >= self.entries.len() {
            return false;
        }
        self.selected = index;
        self.scroll_into_view();
        true
    }

    pub fn handle(&mut self, key: PanelKey) -> PanelOutcome {
        match key {
            PanelKey::Up | PanelKey::SelectUp | PanelKey::Char('k') => self.move_by(-1),
            PanelKey::Down | PanelKey::SelectDown | PanelKey::Char('j') => self.move_by(1),
            PanelKey::HalfPageUp => self.move_by(-(self.height as isize / 2).max(1)),
            PanelKey::HalfPageDown => self.move_by((self.height as isize / 2).max(1)),
            PanelKey::PageUp => self.move_by(-(self.height as isize)),
            PanelKey::PageDown => self.move_by(self.height as isize),
            PanelKey::Home => self.move_to(0),
            PanelKey::End => self.move_to(self.entries.len().saturating_sub(1)),
            PanelKey::Right | PanelKey::Enter => self.enter(),
            PanelKey::Left => self.leave(),
            PanelKey::Escape => PanelOutcome::Dismiss,
            PanelKey::Char('.') => {
                self.toggle_hidden();
                PanelOutcome::Redraw
            }
            PanelKey::Char('r') => {
                self.refresh();
                PanelOutcome::Redraw
            }
            // Open a shell where you are looking, rather than cd-ing there by
            // hand afterwards.
            // Look at a file without leaving the explorer.
            PanelKey::Char('p') => match self.selected() {
                Some(entry) if !entry.is_directory() => PanelOutcome::Preview(entry.path.clone()),
                _ => PanelOutcome::Ignored,
            },
            PanelKey::Char('n') => PanelOutcome::OpenPane(self.target_directory()),
            PanelKey::Char('N') => PanelOutcome::OpenTab(self.target_directory()),
            PanelKey::Char(_) => PanelOutcome::Ignored,
        }
    }

    /// The directory a new pane should start in: the highlighted one, or the
    /// folder being browsed when a file is highlighted.
    fn target_directory(&self) -> PathBuf {
        match self.selected() {
            Some(entry) if entry.is_directory() => entry.path.clone(),
            _ => self.root.clone(),
        }
    }

    /// Right or Enter: descend into a directory, or hand a file's path back to
    /// the shell.
    fn enter(&mut self) -> PanelOutcome {
        match self.selected() {
            Some(entry) if entry.is_directory() => {
                let path = entry.path.clone();
                self.root = path;
                self.selected = 0;
                self.offset = 0;
                self.refresh();
                PanelOutcome::Redraw
            }
            Some(entry) => PanelOutcome::Insert(entry.path.clone()),
            None => PanelOutcome::Ignored,
        }
    }

    /// Left: back to the parent, or nothing at the filesystem root.
    fn leave(&mut self) -> PanelOutcome {
        match self.root.parent() {
            Some(parent) => {
                self.set_root(parent.to_path_buf());
                PanelOutcome::Redraw
            }
            None => PanelOutcome::Ignored,
        }
    }

    fn move_by(&mut self, delta: isize) -> PanelOutcome {
        if self.entries.is_empty() {
            return PanelOutcome::Ignored;
        }
        let last = self.entries.len() as isize - 1;
        let next = (self.selected as isize + delta).clamp(0, last) as usize;
        self.move_to(next)
    }

    fn move_to(&mut self, index: usize) -> PanelOutcome {
        if self.entries.is_empty() {
            return PanelOutcome::Ignored;
        }
        let next = index.min(self.entries.len() - 1);
        if next == self.selected {
            return PanelOutcome::Redraw;
        }
        self.selected = next;
        self.scroll_into_view();
        PanelOutcome::Redraw
    }

    fn scroll_into_view(&mut self) {
        if self.selected < self.offset {
            self.offset = self.selected;
        } else if self.selected >= self.offset + self.height {
            self.offset = self.selected + 1 - self.height;
        }
        let max_offset = self.entries.len().saturating_sub(self.height);
        self.offset = self.offset.min(max_offset);
    }

    /// The panel's contents, ready to draw: a header, then one line per entry
    /// within the visible window.
    pub fn lines(&self) -> Vec<PanelLine> {
        let mut out = Vec::with_capacity(self.height + 1);
        out.push(PanelLine {
            text: self.header(),
            kind: LineKind::Header,
            selected: false,
        });

        if let Some(error) = &self.error {
            out.push(PanelLine {
                text: error.clone(),
                kind: LineKind::Error,
                selected: false,
            });
            return out;
        }

        if self.entries.is_empty() {
            out.push(PanelLine {
                text: "(empty)".to_string(),
                kind: LineKind::Hint,
                selected: false,
            });
            return out;
        }

        for (index, entry) in self
            .entries
            .iter()
            .enumerate()
            .skip(self.offset)
            .take(self.height)
        {
            let kind = match entry.kind {
                EntryKind::Directory => LineKind::Directory,
                EntryKind::File => LineKind::File,
                EntryKind::Executable => LineKind::Executable,
                EntryKind::Symlink => LineKind::Symlink,
            };
            let mark = match entry.kind {
                EntryKind::Directory => "/",
                EntryKind::Executable => "*",
                EntryKind::Symlink => "@",
                EntryKind::File => "",
            };
            out.push(PanelLine {
                text: format!("{}{mark}", entry.name),
                kind,
                selected: index == self.selected,
            });
        }
        out
    }

    /// The deepest part of the path that fits is more useful than the start of
    /// an absolute path, so the head is elided.
    fn header(&self) -> String {
        let text = self.root.to_string_lossy();
        match text.rsplit('/').next() {
            Some("") | None => text.into_owned(),
            Some(name) => name.to_string(),
        }
    }
}
