//! What shaping costs a frame, measured rather than assumed.
//!
//! Run it with
//!
//! ```sh
//! cargo test --release -p caret-app --test shaping_cost -- --ignored --nocapture
//! ```
//!
//! It is not part of the suite: it needs a font, it needs a release build to
//! mean anything, and a timing is not a pass or a fail.

use std::time::Instant;

use caret_app::impls::window::paint::Painter;
use caret_app::types::corners::Corners;
use caret_app::types::surface::Surface;
use caret_app::types::surface_cell::SurfaceCell;
use caret_core::{Flags, Rgb};
use caret_render::{FontLibrary, GlyphCache};

const BG: Rgb = Rgb::new(16, 18, 24);
const FG: Rgb = Rgb::new(210, 215, 225);

/// A screenful of the kind of text a ligature font is bought for.
const CODE: [&str; 8] = [
    "    if value != other && count >= limit {",
    "        let next = items.iter().map(|x| x * 2).collect::<Vec<_>>();",
    "        return Err(Error::Mismatch { want: a, got: b });",
    "    } else if ptr->field == NULL || flag <= 0 {",
    "        println!(\"{name} => {value:?}\");",
    "        /* a comment with -- and ... in it */",
    "        x <<= 2; y >>= 1; z |= mask; w &= ~bits;",
    "    }",
];

fn surface(cols: u16, rows: u16) -> Surface {
    let mut surface = Surface::new(cols, rows, BG);
    for row in 0..rows {
        let line = CODE[row as usize % CODE.len()];
        for (col, ch) in line.chars().take(cols as usize).enumerate() {
            surface.set(
                col as u16,
                row,
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
    }
    surface
}

fn painter(family: &str, ligatures: bool) -> Option<Painter> {
    let library = FontLibrary::load();
    let cache = GlyphCache::new(&library, family, &[], 14.0, 1.2, 0.0)?;
    let mut painter = Painter::new(cache, 8, false);
    painter.ligatures = ligatures;
    Some(painter)
}

fn family_with_ligatures() -> Option<String> {
    const KNOWN: [&str; 6] = [
        "Cascadia Code",
        "CaskaydiaCove Nerd Font Mono",
        "Fira Code",
        "JetBrains Mono",
        "Iosevka",
        "Monaspace Neon",
    ];
    let library = FontLibrary::load();
    KNOWN
        .into_iter()
        .find(|name| library.resolve(name, &[]).is_some())
        .map(|name| name.to_string())
}

/// Paints `frames` frames and reports the mean, warming the caches first so
/// what is measured is drawing rather than rasterising.
fn cost(painter: &mut Painter, frame: &Surface, size: (usize, usize), frames: u32) -> f64 {
    let mut buffer = vec![0u32; size.0 * size.1];
    painter.paint(&mut buffer, size, frame, BG);

    let start = Instant::now();
    for _ in 0..frames {
        painter.paint(&mut buffer, size, frame, BG);
    }
    start.elapsed().as_secs_f64() * 1_000.0 / frames as f64
}

#[test]
#[ignore = "a measurement, not a test"]
fn what_shaping_costs_a_frame() {
    let Some(family) = family_with_ligatures() else {
        println!("no ligature font installed; nothing to measure");
        return;
    };
    let (cols, rows) = (200u16, 50u16);
    let frame = surface(cols, rows);

    let mut off = painter(&family, false).expect("the family loads");
    let (cell_w, cell_h) = off.cell;
    let size = (
        cols as usize * cell_w + 16,
        rows as usize * cell_h + 16 + off.chrome,
    );

    let plain = cost(&mut off, &frame, size, 60);
    let mut on = painter(&family, true).expect("the family loads");
    let shaped = cost(&mut on, &frame, size, 60);
    // The first frame of a new window pays for shaping every run it sees.
    let mut cold = painter(&family, true).expect("the family loads");
    let first = {
        let mut buffer = vec![0u32; size.0 * size.1];
        let start = Instant::now();
        cold.paint(&mut buffer, size, &frame, BG);
        start.elapsed().as_secs_f64() * 1_000.0
    };

    println!(
        "font: {family}, grid {cols}x{rows}, {}x{} px",
        size.0, size.1
    );
    println!("  ligatures off : {plain:.2} ms a frame");
    println!("  ligatures on  : {shaped:.2} ms a frame");
    println!("  first frame on: {first:.2} ms (every run shaped once)");
}
