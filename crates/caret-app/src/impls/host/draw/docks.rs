//! The explorer and the agent panel, and the cards they sit in.
//!
//! Part of the drawing layer: one region of the window per file, so a
//! change to one of them is a change to one file.

#[allow(unused_imports)]
use super::*;

impl Session {
    fn draw_dock_edge(&self, surface: &mut Surface, rect: Rect, on_left: bool, focused: bool) {
        let ui = &self.theme.ui;
        let colour = if focused {
            ui.border_focused
        } else {
            ui.border
        };
        let x = if on_left {
            rect.right().saturating_sub(1)
        } else {
            rect.x
        };
        for y in rect.y..rect.bottom() {
            surface.put(x, y, '\u{2502}', Style::new(colour, self.theme.bg));
        }
    }

    /// The dock's card: the area it paints, inset from the space it was given.
    ///
    /// A panel with rounded corners has to float, or the curves are cut off by
    /// the window edge and read as a mistake. The margin it leaves is also
    /// what separates it from the panes beside it, which is why the edge rule
    /// is only drawn to show focus.
    pub(crate) fn dock_content(rect: Rect, on_left: bool) -> Rect {
        if rect.w <= DOCK_GAP {
            // All gap and no card. Shifting by the gap anyway would put the
            // card past the dock's own right edge.
            return Rect::new(rect.x, rect.y, 0, rect.h);
        }
        let width = rect.w - DOCK_GAP;
        let x = if on_left { rect.x } else { rect.x + DOCK_GAP };
        Rect::new(x, rect.y, width, rect.h)
    }

    /// The area inside a card where its contents go.
    ///
    /// Two columns against one row, for the same reason the card's own margin
    /// is: a cell is about twice as tall as it is wide, so equal counts would
    /// read as a narrow left edge under a deep top one.
    pub(crate) fn panel_body(card: Rect) -> Rect {
        if card.w <= PANEL_PAD_X * 2 || card.h <= PANEL_PAD_Y * 2 {
            // Too small to pad. Offsetting anyway would leave the body
            // outside the card and draw it over whatever is next door.
            return Rect::new(card.x, card.y, 0, 0);
        }
        Rect::new(
            card.x + PANEL_PAD_X,
            card.y + PANEL_PAD_Y,
            card.w - PANEL_PAD_X * 2,
            card.h - PANEL_PAD_Y * 2,
        )
    }

    /// A selected row's pill, which sits inside the card but outside the text,
    /// so the highlight has a little of the card either side of it.
    fn row_highlight(card: Rect, y: u16) -> Rect {
        Rect::new(card.x + 1, y, card.w.saturating_sub(2), 1)
    }

    pub(super) fn draw_explorer(&self, surface: &mut Surface, rect: Rect) {
        let ui = &self.theme.ui;
        let focused = self.focus == FocusTarget::Explorer;
        let content = Self::dock_content(rect, true);

        surface.fill_rounded(content, ui.panel_bg, self.theme.bg);
        if focused {
            self.draw_dock_edge(surface, rect, true, focused);
        }

        let body = dock_body(rect, true);
        let lines = self.explorer.lines();
        let heading = Style::new(ui.accent, ui.panel_bg).bold();
        if let Some(header) = lines.first() {
            surface.text(body.x, body.y, body.w, &header.text, heading);
        }
        for x in body.x..body.right() {
            surface.put(
                x,
                body.y + 1,
                '\u{2500}',
                Style::new(ui.border, ui.panel_bg),
            );
        }

        for (offset, line) in lines.iter().skip(1).enumerate() {
            let y = body.y + DOCK_HEADER + offset as u16;
            if y >= body.bottom() {
                break;
            }

            let fg = match line.kind {
                LineKind::Header => ui.accent,
                LineKind::Directory => self.theme.indexed(4),
                LineKind::Executable => self.theme.indexed(2),
                LineKind::Symlink => self.theme.indexed(6),
                LineKind::File => ui.panel_fg,
                LineKind::Hint => ui.tab_inactive_fg,
                LineKind::Error => self.theme.indexed(1),
            };

            // An unfocused selection stays visible but stops competing with
            // the focused pane for attention.
            let bg = match (line.selected, focused) {
                (true, true) => ui.selection_ui,
                (true, false) => ui.panel_bg,
                (false, _) => ui.panel_bg,
            };
            if line.selected {
                surface.fill_rounded(Self::row_highlight(content, y), bg, ui.panel_bg);
            }

            let mut style = Style {
                fg,
                bg,
                flags: Flags::NONE,
            };
            if line.kind == LineKind::Directory {
                style = style.bold();
            }
            surface.text_over(body.x, y, body.w, &line.text, style);
        }
    }

    pub(super) fn draw_agent(&self, surface: &mut Surface, rect: Rect) {
        let ui = &self.theme.ui;
        let focused = self.focus == FocusTarget::Agent;
        let card = agent_rect(rect);

        match self.agent {
            Some(id) if self.pane(id).is_some() => {
                self.draw_pane(surface, id, card);
                // The terminal paints its own background over the whole card,
                // so the corners are taken off afterwards rather than being
                // laid down first and overwritten.
                surface.round_corners(card, self.theme.bg);
            }
            _ => {
                surface.fill_rounded(card, ui.panel_bg, self.theme.bg);
                let body = dock_body(rect, false);
                surface.text(
                    body.x,
                    body.y,
                    body.w,
                    "AGENT",
                    Style::new(ui.accent, ui.panel_bg).bold(),
                );
                for x in body.x..body.right() {
                    surface.put(
                        x,
                        body.y + 1,
                        '\u{2500}',
                        Style::new(ui.border, ui.panel_bg),
                    );
                }
                surface.text(
                    body.x,
                    body.y + DOCK_HEADER,
                    body.w,
                    &self.config.sidebars.agent.command,
                    Style::new(ui.tab_inactive_fg, ui.panel_bg),
                );
            }
        }
        if focused {
            self.draw_dock_edge(surface, rect, false, focused);
        }
    }
}

/// The gap between a dock's card and the panes beside it.
///
/// Only on that side: the window's own padding already holds the card off the
/// window edge, and insetting there as well put the card four times further
/// from the window than from the terminal next to it. One column is about the
/// same number of pixels as that padding, so the two sides match.
const DOCK_GAP: u16 = 1;

/// Padding between a card's edge and what it holds.
const PANEL_PAD_X: u16 = 2;

const PANEL_PAD_Y: u16 = 1;

/// Rows a dock spends on its heading and the rule beneath it.
pub(crate) const DOCK_HEADER: u16 = 2;

/// Where the agent's terminal is drawn: the dock's whole card.
///
/// The explorer is a list, so the card can pad it. The agent is a terminal,
/// and a terminal's background is the window's own, so padding leaves a ring
/// of panel colour that reads as a grey border drawn round the panel rather
/// than as breathing room. It gets the card itself, corners taken off.
pub fn agent_rect(dock: Rect) -> Rect {
    Session::dock_content(dock, false)
}

/// Where a dock's contents are drawn, for the space the layout gave it.
///
/// The focus rule lives in the gap outside the card, so a rectangle worked out
/// anywhere else is a column off: that is what put the agent's cursor on the
/// rule and sent its clicks to the wrong column. Drawing, sizing, the cursor
/// and the mouse all come through here.
pub fn dock_body(rect: Rect, on_left: bool) -> Rect {
    Session::panel_body(Session::dock_content(rect, on_left))
}
