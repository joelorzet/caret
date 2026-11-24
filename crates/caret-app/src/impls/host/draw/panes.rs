//! A pane's grid of cells, and the colours each one resolves to.
//!
//! Part of the drawing layer: one region of the window per file, so a
//! change to one of them is a change to one file.

#[allow(unused_imports)]
use super::*;

use caret_render::BoxGlyph;

impl Session {
    pub(super) fn draw_pane(&self, surface: &mut Surface, id: PaneId, rect: Rect) {
        let Some(pane) = self.pane(id) else {
            surface.fill(rect, self.theme.bg);
            let ui = &self.theme.ui;
            surface.text(
                rect.x,
                rect.y,
                rect.w,
                "[process exited]",
                Style::new(ui.tab_inactive_fg, self.theme.bg),
            );
            return;
        };

        let reverse_screen = pane.terminal.modes().contains(Modes::REVERSE_VIDEO);
        let screen = pane.terminal.screen();
        let rows = rect.h.min(screen.rows());
        let cols = rect.w.min(screen.cols());

        let selection = self
            .selection
            .as_ref()
            .and_then(|(owner, selection)| (*owner == id).then_some(selection));

        for y in 0..rows {
            let line = screen.display_line(y);
            let logical = screen.display_row(y);
            for x in 0..cols {
                let cell = line[x as usize];
                let (mut fg, mut bg) = self.cell_colours(&cell, reverse_screen);
                if selection.is_some_and(|s| s.contains(logical, x)) {
                    fg = self.theme.selection_fg;
                    bg = self.theme.selection_bg;
                }
                let mut flags = cell.flags;
                if self.hovers_link(id, logical, x) {
                    flags = flags.union(Flags::UNDERLINE);
                }
                surface.set(
                    rect.x + x,
                    rect.y + y,
                    SurfaceCell {
                        ch: cell.ch,
                        fg,
                        bg,
                        flags,
                        round: Corners::NONE,
                        under: bg,
                    },
                );
            }
        }
    }

    /// Resolves a cell's colours through the theme, applying the attributes
    /// that change colour rather than shape.
    fn cell_colours(&self, cell: &Cell, reverse_screen: bool) -> (Rgb, Rgb) {
        let mut source = cell.fg;
        // Bold text optionally also brightens, which is what most terminals
        // did historically and some prompts still rely on.
        if self.config.font.bold_is_bright && cell.flags.contains(Flags::BOLD) {
            if let ColorKind::Indexed(i) = source.kind() {
                if i < 8 {
                    source = Color::indexed(i + 8);
                }
            }
        }

        let mut fg = self.theme.resolve(source, ColorSlot::Foreground);
        let mut bg = self.theme.resolve(cell.bg, ColorSlot::Background);

        if cell.flags.contains(Flags::REVERSE) != reverse_screen {
            core::mem::swap(&mut fg, &mut bg);
        }
        if cell.flags.contains(Flags::DIM) {
            fg = fg.blend(bg, 0.55);
        }
        if cell.flags.contains(Flags::HIDDEN) {
            fg = bg;
        }
        (fg, bg)
    }
}

/// Turns the cells where dividers meet into the junction that joins them.
///
/// Each divider is drawn as a straight run, so where two cross, one of them
/// is overwritten and its line stops a cell short. The gap reads as the
/// panes not quite lining up, which is the one thing a divider exists to
/// deny. Arms are taken from the neighbours rather than from the dividers
/// themselves, so a junction of any shape comes out right.
pub fn join_dividers(surface: &mut Surface, dividers: &[Divider]) {
    use std::collections::HashSet;

    let cells: HashSet<(u16, u16)> = dividers
        .iter()
        .flat_map(|divider| {
            let rect = divider.rect;
            (rect.y..rect.bottom()).flat_map(move |y| (rect.x..rect.right()).map(move |x| (x, y)))
        })
        .collect();

    for &(x, y) in &cells {
        let arm = |dx: i32, dy: i32| {
            let at = (
                x.checked_add_signed(dx as i16)?,
                y.checked_add_signed(dy as i16)?,
            );
            cells.contains(&at).then_some(())
        };
        let (up, down, left, right) = (
            arm(0, -1).is_some(),
            arm(0, 1).is_some(),
            arm(-1, 0).is_some(),
            arm(1, 0).is_some(),
        );

        // Asking the box-drawing table for the character with these arms,
        // rather than keeping a second copy of it here. A straight run or a
        // stub with one arm is already drawn correctly.
        let Some(glyph) = BoxGlyph::joining(up, down, left, right) else {
            continue;
        };

        if let Some(mut cell) = surface.get(x, y) {
            cell.ch = glyph;
            surface.set(x, y, cell);
        }
    }
}

/// True when a divider runs along an edge of `rect`.
pub(super) fn adjacent(divider: &Divider, rect: &Rect) -> bool {
    match divider.axis {
        Axis::Horizontal => {
            (divider.rect.x == rect.right() || divider.rect.right() == rect.x)
                && divider.rect.overlap_y(rect) > 0
        }
        Axis::Vertical => {
            (divider.rect.y == rect.bottom() || divider.rect.bottom() == rect.y)
                && divider.rect.overlap_x(rect) > 0
        }
    }
}
