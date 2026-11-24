#[allow(unused_imports)]
use crate::prelude::*;

use caret_features::{link_at, LinkTarget};

/// The modifier that turns a click into an opening.
///
/// Cmd on a Mac and Ctrl elsewhere, which is where people's hands already
/// go: a plain click still places the cursor and starts a selection.
pub fn opens_links(mods: Modifiers) -> bool {
    if cfg!(target_os = "macos") {
        mods.contains(Modifiers::SUPER)
    } else {
        mods.contains(Modifiers::CTRL)
    }
}

impl Session {
    /// The link under a point in a pane, if there is one worth opening.
    ///
    /// A path is a link only if it is there: output is full of words with
    /// slashes in them, and offering to open something that does not exist
    /// is worse than offering nothing.
    pub(crate) fn link_under(
        &self,
        id: PaneId,
        point: Point,
    ) -> Option<(usize, usize, LinkTarget)> {
        let pane = self.pane(id)?;
        let text = pane.terminal.screen().row_text(point.row);
        let link = link_at(&text, point.col as usize)?;

        if let LinkTarget::File { path, .. } = &link.target {
            self.resolve(id, path)?;
        }
        Some((link.start, link.end, link.target))
    }

    /// Where a path in a pane's output actually is: beside the pane's
    /// working directory unless it says otherwise.
    fn resolve(&self, id: PaneId, path: &str) -> Option<std::path::PathBuf> {
        let candidate = std::path::PathBuf::from(path);
        if candidate.is_absolute() {
            return candidate.exists().then_some(candidate);
        }
        let base = self
            .pane(id)
            .and_then(|pane| pane.terminal.cwd())
            .map(std::path::PathBuf::from)
            .or_else(|| self.ptys.working_dir(PtyId(id.0)))?;
        let joined = base.join(path);
        joined.exists().then_some(joined)
    }

    /// Whether this cell is part of the link the pointer is over.
    pub(crate) fn hovers_link(&self, id: PaneId, row: u32, col: u16) -> bool {
        self.hover_link.is_some_and(|(owner, hovered, start, end)| {
            owner == id && hovered == row && (col as usize) >= start && (col as usize) < end
        })
    }

    /// Follows the pointer, so a link shows itself before it is clicked.
    pub(crate) fn track_link(&mut self, id: PaneId, point: Point, mods: Modifiers) {
        let found = opens_links(mods)
            .then(|| self.link_under(id, point))
            .flatten()
            .map(|(start, end, _)| (id, point.row, start, end));
        if found != self.hover_link {
            self.hover_link = found;
            self.needs_draw = true;
        }
    }

    /// Whether a click should open rather than select.
    pub(crate) fn click_opens_link(&mut self, id: PaneId, point: Point, mods: Modifiers) -> bool {
        if !opens_links(mods) {
            return false;
        }
        let Some((_, _, target)) = self.link_under(id, point) else {
            return false;
        };
        self.open_link(id, target);
        true
    }

    fn open_link(&mut self, id: PaneId, target: LinkTarget) {
        match target {
            LinkTarget::Url(url) => {
                self.message = format!(" opening {url}");
                open_externally(&url);
            }
            LinkTarget::File { path, line, .. } => {
                let Some(full) = self.resolve(id, &path) else {
                    return;
                };
                let _ = self.open_editor_at(full, line.map(|n| n as usize));
            }
        }
        self.needs_draw = true;
    }
}

/// Hands a URL to whatever the desktop opens such things with.
fn open_externally(url: &str) {
    let opener = if cfg!(target_os = "macos") {
        "open"
    } else {
        "xdg-open"
    };
    let _ = std::process::Command::new(opener)
        .arg(url)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn();
}
