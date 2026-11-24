use caret_app::impls::window::paint::Painter;
use caret_app::runs_in;
use caret_app::types::corners::Corners;
use caret_app::types::surface::Surface;
use caret_app::types::surface_cell::SurfaceCell;
use caret_core::{Flags, Rgb};
use caret_render::{FontLibrary, GlyphCache};

const BG: Rgb = Rgb::new(0, 0, 0);
const FG: Rgb = Rgb::new(200, 200, 200);
const OTHER_BG: Rgb = Rgb::new(60, 80, 120);

fn plain(ch: char) -> SurfaceCell {
    SurfaceCell {
        ch,
        fg: FG,
        bg: BG,
        flags: Flags::NONE,
        round: Corners::NONE,
        under: BG,
    }
}

/// Writes `text` across the first row, letting each cell be amended.
fn row_of(text: &str, amend: impl Fn(u16, SurfaceCell) -> SurfaceCell) -> Surface {
    let mut surface = Surface::new(text.chars().count() as u16, 1, BG);
    for (col, ch) in text.chars().enumerate() {
        let col = col as u16;
        surface.set(col, 0, amend(col, plain(ch)));
    }
    surface
}

fn runs(surface: &Surface) -> Vec<(u16, String)> {
    let (cols, _) = surface.size();
    runs_in(surface, 0, cols)
}

#[test]
fn a_line_of_code_offers_the_part_with_the_operator_in_it() {
    let surface = row_of("if a != b", |_, cell| cell);
    assert_eq!(runs(&surface), vec![(0, "if a != b".to_string())]);
}

// Words alone can never join, so the font is never asked about them.
#[test]
fn a_line_of_prose_offers_nothing() {
    let surface = row_of("the quick brown fox", |_, cell| cell);
    assert!(runs(&surface).is_empty());
}

// A joined shape cannot be half selected, so the run has to end where the
// selection does and both sides draw their own characters.
#[test]
fn a_selection_through_an_operator_ends_the_run_at_its_edge() {
    let surface = row_of("a != b", |col, mut cell| {
        if col >= 3 {
            cell.bg = OTHER_BG;
        }
        cell
    });

    let found = runs(&surface);
    assert!(
        found.iter().all(|(_, text)| !text.contains("!=")),
        "neither side holds both halves: {found:?}"
    );
}

#[test]
fn a_cell_coloured_differently_ends_the_run() {
    let surface = row_of("x => y", |col, mut cell| {
        if col == 3 {
            cell.fg = OTHER_BG;
        }
        cell
    });

    let found = runs(&surface);
    assert!(
        found.iter().all(|(_, text)| !text.contains("=>")),
        "{found:?}"
    );
}

#[test]
fn a_cell_in_a_different_style_ends_the_run() {
    let surface = row_of("a -> b", |col, mut cell| {
        if col == 2 {
            cell.flags = Flags::BOLD;
        }
        cell
    });

    let found = runs(&surface);
    assert!(
        found.iter().all(|(_, text)| !text.contains("->")),
        "{found:?}"
    );
}

#[test]
fn a_wide_character_ends_the_run() {
    let surface = row_of("a==b", |col, mut cell| {
        if col == 1 {
            cell.flags = Flags::WIDE;
        }
        cell
    });

    let found = runs(&surface);
    assert!(
        found.iter().all(|(_, text)| !text.contains("==")),
        "{found:?}"
    );
}

// A cell holding a letter and its accent has no single character to shape.
#[test]
fn a_cell_holding_more_than_one_character_ends_the_run() {
    let mut surface = Surface::new(4, 1, BG);
    let mut marked = plain('=');
    marked.ch = surface.hold("=\u{0330}");
    marked.flags = Flags::CLUSTER;
    surface.set(0, 0, plain('!'));
    surface.set(1, 0, marked);
    surface.set(2, 0, plain('='));
    surface.set(3, 0, plain('x'));

    let found = runs(&surface);
    let spans_marked = found.iter().any(|(start, text)| {
        let end = *start + text.chars().count() as u16;
        *start <= 1 && end > 1
    });
    assert!(!spans_marked, "no run spans the marked cell: {found:?}");
}

#[test]
fn a_run_stops_where_the_row_does() {
    let surface = row_of("!=", |_, cell| cell);
    assert_eq!(runs(&surface), vec![(0, "!=".to_string())]);
}

/// Paints one row and counts the lit pixels of each cell.
fn ink_per_cell(text: &str, ligatures: bool) -> Option<Vec<usize>> {
    let library = FontLibrary::load();
    let cache = GlyphCache::new(&library, "Cascadia Code", &[], 20.0, 1.2, 0.0)?;
    let mut painter = Painter::new(cache, 0, false);
    painter.ligatures = ligatures;
    painter.chrome = 0;

    let cols = text.chars().count() as u16;
    let mut surface = Surface::new(cols, 1, BG);
    for (col, ch) in text.chars().enumerate() {
        surface.set(
            col as u16,
            0,
            SurfaceCell {
                ch,
                fg: FG,
                bg: BG,
                flags: Flags::NONE,
                round: Corners::NONE,
                under: BG,
            },
        );
    }

    let (cell_w, cell_h) = painter.cell;
    let size = (cols as usize * cell_w, cell_h);
    let mut buffer = vec![0u32; size.0 * size.1];
    painter.paint(&mut buffer, size, &surface, BG);

    Some(
        (0..cols as usize)
            .map(|col| {
                let from = col * cell_w;
                (0..size.1)
                    .flat_map(|y| (from..from + cell_w).map(move |x| y * size.0 + x))
                    .filter(|at| buffer[*at] != 0)
                    .count()
            })
            .collect(),
    )
}

// A font is free to draw a joined shape anywhere across the run: Cascadia
// Code puts the whole of != in the first cell's glyph and leaves the second
// blank. Filling each background just before its own glyph wiped that out,
// so the shape came out half drawn with a gap beside it.
#[test]
fn a_joined_shape_is_drawn_across_the_cells_it_covers() {
    let Some(plain) = ink_per_cell("x!=y", false) else {
        eprintln!("Cascadia Code not installed; skipping");
        return;
    };
    let joined = ink_per_cell("x!=y", true).expect("the same font");

    assert!(
        joined[2] > 0,
        "the second cell of != is drawn on: plain {plain:?}, joined {joined:?}"
    );
    assert_eq!(
        (joined[0], joined[3]),
        (plain[0], plain[3]),
        "and the letters either side are untouched"
    );
}
