#[allow(unused_imports)]
use crate::prelude::*;

pub mod arrange;
pub mod cursor;
pub mod docks;
pub mod palette;
pub mod panes;
pub mod preview;
pub mod status;
pub mod tabs;
pub mod typing;

// The submodules are where these live; this keeps the path other code uses
// the same as it was when they all sat in one file.
pub use self::arrange::{arrange_canvas, arrange_rect};
pub(crate) use self::docks::DOCK_HEADER;
pub use self::docks::{agent_rect, dock_body};
pub(crate) use self::palette::{MAX_PALETTE_WIDTH, MIN_PALETTE_WIDTH, PALETTE_CHROME};
use self::panes::adjacent;
pub use self::panes::join_dividers;

impl Session {
    /// Renders a frame as escape sequences for the surrounding terminal.
    pub(crate) fn draw(&self, frame: &Frame) -> String {
        let (surface, cursor) = self.compose(frame);
        let mut out = surface.render();
        out.push_str(&self.place_cursor(cursor));
        out
    }

    /// Builds the frame, and says where the cursor belongs.
    ///
    /// Shared by both backends: the terminal one turns this into escape
    /// sequences, the windowed one paints it with glyphs.
    pub(crate) fn compose(&self, frame: &Frame) -> (Surface, Option<(PaneId, Rect)>) {
        let ui = &self.theme.ui;
        let mut surface = Surface::new(self.size.0, self.size.1, self.theme.bg);

        if let Some(rect) = frame.tab_bar {
            self.draw_tab_bar(&mut surface, rect);
        }
        if let Some(rect) = frame.left {
            self.draw_explorer(&mut surface, rect);
        }
        if let Some(rect) = frame.right {
            self.draw_agent(&mut surface, rect);
        }

        let tab = self.workspace.active_tab();
        let resolved = tab.resolve(frame.center);
        let focused = resolved
            .panes
            .iter()
            .find(|(id, _)| *id == tab.focus())
            .map(|(_, r)| *r);

        for (id, rect) in &resolved.panes {
            self.draw_pane(&mut surface, *id, *rect);
        }
        for divider in &resolved.dividers {
            let touching = focused.is_some_and(|f| adjacent(divider, &f));
            let colour = if touching {
                ui.border_focused
            } else {
                ui.border
            };
            let ch = match divider.axis {
                Axis::Horizontal => '\u{2502}',
                Axis::Vertical => '\u{2500}',
            };
            surface.hline(divider.rect, ch, colour, self.theme.bg);
        }
        join_dividers(&mut surface, &resolved.dividers);

        if self.focus == FocusTarget::Panes && self.rearrange.is_none() {
            if let Some(rect) = focused {
                self.draw_input_highlight(&mut surface, rect);
                self.draw_suggestions(&mut surface, rect);
            }
        }

        if let Some(rect) = frame.status {
            self.draw_status(&mut surface, rect);
        }
        if self.preview.is_some() {
            self.draw_preview(&mut surface);
        }
        if self.palette.is_some() {
            self.draw_palette(&mut surface);
        }
        if self.rearrange.is_some() {
            self.draw_arrange(&mut surface);
        }

        let cursor = match self.focus {
            // Nothing is being typed into while the panes are being moved.
            _ if self.rearrange.is_some() => None,
            FocusTarget::Panes => focused.map(|rect| (tab.focus(), rect)),
            FocusTarget::Agent => frame
                .right
                .zip(self.agent)
                .map(|(rect, id)| (id, agent_rect(rect))),
            // The explorer draws its own selection; a blinking cursor in it
            // would read as a text caret.
            FocusTarget::Explorer => None,
        };
        (surface, cursor)
    }
}

fn draw_border(surface: &mut Surface, rect: Rect, fg: Rgb, bg: Rgb) {
    if rect.w < 2 || rect.h < 2 {
        return;
    }
    let style = Style {
        fg,
        bg,
        flags: Flags::NONE,
    };
    let (right, bottom) = (rect.right() - 1, rect.bottom() - 1);

    for x in rect.x..=right {
        surface.put(x, rect.y, '\u{2500}', style);
        surface.put(x, bottom, '\u{2500}', style);
    }
    for y in rect.y..=bottom {
        surface.put(rect.x, y, '\u{2502}', style);
        surface.put(right, y, '\u{2502}', style);
    }
    surface.put(rect.x, rect.y, '\u{256d}', style);
    surface.put(right, rect.y, '\u{256e}', style);
    surface.put(rect.x, bottom, '\u{2570}', style);
    surface.put(right, bottom, '\u{256f}', style);
}
/// A column of breathing room at the edges of the chrome.
const GUTTER: u16 = 1;
fn truncate(text: &str, limit: usize) -> String {
    if text.chars().count() <= limit {
        return text.to_string();
    }
    if limit <= 1 {
        return "\u{2026}".to_string();
    }
    text.chars().take(limit - 1).chain(['\u{2026}']).collect()
}
