#[allow(unused_imports)]
use crate::prelude::*;

use caret_ui::PxRect;

impl Canvas<'_> {
    pub fn fill(&mut self, left: usize, top: usize, size: (usize, usize), colour: Rgb) {
        let packed = colour.to_u32();
        let (w, h) = size;
        for y in top..(top + h).min(self.height) {
            let row = y * self.width;
            let start = row + left.min(self.width);
            let end = (row + (left + w).min(self.width)).min(self.pixels.len());
            if start < end {
                self.pixels[start..end].fill(packed);
            }
        }
    }

    /// A hollow box, so the character under the cursor stays readable.
    pub fn outline(&mut self, left: usize, top: usize, size: (usize, usize), background: Rgb) {
        let (w, h) = size;
        let edge = Rgb::new(255 - background.r, 255 - background.g, 255 - background.b);
        self.fill(left, top, (w, 1), edge);
        self.fill(left, top + h.saturating_sub(1), (w, 1), edge);
        self.fill(left, top, (1, h), edge);
        self.fill(left + w.saturating_sub(1), top, (1, h), edge);
    }

    pub fn blend(&mut self, x: usize, y: usize, colour: Rgb, alpha: u8) {
        if x >= self.width || y >= self.height {
            return;
        }
        let index = y * self.width + x;
        self.pixels[index] = blend(self.pixels[index], colour, alpha);
    }

    pub fn rect(&mut self, rect: PxRect, colour: Rgb) {
        let (Some(left), Some(top)) = (clamp(rect.x), clamp(rect.y)) else {
            return;
        };
        self.fill(left, top, (rect.w as usize, rect.h as usize), colour);
    }

    /// A rounded rectangle with antialiased corners.
    ///
    /// Only the corner bands are measured; the rows between them are straight
    /// edges and fill at full speed.
    pub fn rounded(&mut self, rect: PxRect, radius: f32, colour: Rgb) {
        if rect.w == 0 || rect.h == 0 {
            return;
        }
        let radius = radius.min(rect.w as f32 / 2.0).min(rect.h as f32 / 2.0);
        if radius < 0.5 {
            self.rect(rect, colour);
            return;
        }

        let band = radius.ceil() as i32;
        let (Some(left), _) = (clamp(rect.x), ()) else {
            return;
        };
        for y in rect.y..rect.bottom() {
            if y < 0 || y as usize >= self.height {
                continue;
            }
            let from_edge = (y - rect.y).min(rect.bottom() - 1 - y);
            if from_edge >= band {
                self.fill(left, y as usize, (rect.w as usize, 1), colour);
                continue;
            }
            for x in rect.x..rect.right() {
                if x < 0 {
                    continue;
                }
                let coverage = coverage(rect, radius, x, y);
                if coverage > 0 {
                    self.blend(x as usize, y as usize, colour, coverage);
                }
            }
        }
    }
}

/// How much of this pixel the rounded rectangle covers.
///
/// The signed distance to the shape, taken at the pixel centre and softened
/// over one pixel; negative is inside.
fn coverage(rect: PxRect, radius: f32, x: i32, y: i32) -> u8 {
    let half_w = rect.w as f32 / 2.0;
    let half_h = rect.h as f32 / 2.0;
    let px = (x as f32 + 0.5 - rect.x as f32 - half_w).abs() - (half_w - radius);
    let py = (y as f32 + 0.5 - rect.y as f32 - half_h).abs() - (half_h - radius);

    let outside = px.max(0.0).hypot(py.max(0.0));
    let inside = px.max(py).min(0.0);
    let distance = outside + inside - radius;

    let alpha = (0.5 - distance).clamp(0.0, 1.0);
    (alpha * 255.0).round() as u8
}

fn clamp(value: i32) -> Option<usize> {
    (value >= 0).then_some(value as usize)
}

/// Blends `colour` over the pixel already there, by coverage.
pub fn blend(under: u32, colour: Rgb, alpha: u8) -> u32 {
    if alpha == 255 {
        return colour.to_u32();
    }
    let a = alpha as u32;
    let inv = 255 - a;
    let mix = |shift: u32, c: u8| {
        let base = (under >> shift) & 0xff;
        ((base * inv + c as u32 * a) / 255) << shift
    };
    mix(16, colour.r) | mix(8, colour.g) | mix(0, colour.b)
}
