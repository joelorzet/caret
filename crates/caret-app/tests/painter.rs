mod support;

use caret_app::impls::window::paint::Painter;
use caret_app::types::corners::Corners;
use caret_app::types::surface::Surface;
use caret_app::types::surface_cell::SurfaceCell;
use caret_core::{Flags, Rgb};
use caret_render::{FontLibrary, GlyphCache};

const INK: Rgb = Rgb::new(255, 255, 255);
const PAPER: Rgb = Rgb::new(0, 0, 0);

fn painter(builtin_boxes: bool) -> Option<Painter> {
    let library = FontLibrary::load();
    let family = library.monospaced().first().map(|f| f.name.clone())?;
    let cache = GlyphCache::new(&library, &family, &[], 16.0, 1.2, 0.0)?;
    Some(Painter::new(cache, 0, builtin_boxes))
}

/// A one-column surface of `ch`, painted into a fresh buffer.
fn paint_column(painter: &mut Painter, ch: char, rows: u16) -> (Vec<u32>, usize, usize) {
    let mut surface = Surface::new(1, rows, PAPER);
    for y in 0..rows {
        surface.set(
            0,
            y,
            SurfaceCell {
                ch,
                fg: INK,
                bg: PAPER,
                flags: Flags::NONE,
                round: Corners::NONE,
                under: PAPER,
            },
        );
    }
    let (w, h) = (painter.cell.0, painter.cell.1 * rows as usize);
    let mut buffer = vec![0u32; w * h];
    painter.paint(&mut buffer, (w, h), &surface, PAPER);
    (buffer, w, h)
}

// The whole point: a column of dividers must be one unbroken line, not a
// dotted one. Drawing them from the font leaves a gap at every cell boundary.
#[test]
fn a_column_of_dividers_has_no_gaps() {
    let Some(mut painter) = painter(true) else {
        support::skipping("no monospaced fonts installed");
        return;
    };
    let rows = 4;
    let (buffer, width, height) = paint_column(&mut painter, '│', rows);

    let inked: Vec<usize> = (0..height)
        .filter(|&y| (0..width).any(|x| buffer[y * width + x] != PAPER.to_u32()))
        .collect();

    assert_eq!(inked.len(), height, "every row of the line is drawn");
    assert_eq!(inked.first(), Some(&0), "it starts at the very top");
    assert_eq!(inked.last(), Some(&(height - 1)), "and reaches the bottom");
}

#[test]
fn a_row_of_horizontal_rules_has_no_gaps() {
    let Some(mut painter) = painter(true) else {
        return;
    };
    let cols = 4;
    let mut surface = Surface::new(cols, 1, PAPER);
    for x in 0..cols {
        surface.set(
            x,
            0,
            SurfaceCell {
                ch: '─',
                fg: INK,
                bg: PAPER,
                flags: Flags::NONE,
                round: Corners::NONE,
                under: PAPER,
            },
        );
    }
    let (w, h) = (painter.cell.0 * cols as usize, painter.cell.1);
    let mut buffer = vec![0u32; w * h];
    painter.paint(&mut buffer, (w, h), &surface, PAPER);

    let inked: Vec<usize> = (0..w)
        .filter(|&x| (0..h).any(|y| buffer[y * w + x] != PAPER.to_u32()))
        .collect();
    assert_eq!(inked.len(), w, "every column of the rule is drawn");
}

#[test]
fn a_full_block_covers_its_whole_cell() {
    let Some(mut painter) = painter(true) else {
        return;
    };
    let (buffer, width, height) = paint_column(&mut painter, '█', 1);
    assert!(
        buffer.iter().all(|&p| p == INK.to_u32()),
        "{} of {} pixels were left unpainted",
        buffer.iter().filter(|&&p| p != INK.to_u32()).count(),
        width * height
    );
}

#[test]
fn a_half_block_covers_half_its_cell() {
    let Some(mut painter) = painter(true) else {
        return;
    };
    let (buffer, width, height) = paint_column(&mut painter, '▀', 1);
    let inked = buffer.iter().filter(|&&p| p == INK.to_u32()).count();
    let expected = width * (height / 2);
    assert_eq!(inked, expected, "the upper half and no more");
}

#[test]
fn ordinary_text_still_comes_from_the_font() {
    let Some(mut painter) = painter(true) else {
        return;
    };
    let (buffer, _, _) = paint_column(&mut painter, 'M', 1);
    assert!(buffer.iter().any(|&p| p != PAPER.to_u32()), "M was drawn");
    assert!(
        buffer.iter().any(|&p| p == PAPER.to_u32()),
        "and is not a solid block"
    );
}

#[test]
fn the_setting_can_be_turned_off() {
    let Some(mut painter) = painter(false) else {
        return;
    };
    let (buffer, _, _) = paint_column(&mut painter, '│', 2);
    // Whether the font's own glyph leaves a gap depends on the font, so this
    // only checks the switch is honoured and something is still drawn.
    assert!(buffer.iter().any(|&p| p != PAPER.to_u32()));
}

#[test]
fn the_grid_size_follows_the_window() {
    let Some(painter) = painter(true) else { return };
    let (cell_w, cell_h) = painter.cell;
    let (cols, rows) = painter.grid_size(cell_w * 20, cell_h * 10);
    assert_eq!((cols, rows), (20, 10));

    let (cols, rows) = painter.grid_size(1, 1);
    assert_eq!((cols, rows), (1, 1), "a tiny window still has one cell");
}
