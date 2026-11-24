#[allow(unused_imports)]
use crate::prelude::*;

impl Rgb {
    pub const BLACK: Rgb = Rgb { r: 0, g: 0, b: 0 };
    pub const WHITE: Rgb = Rgb {
        r: 255,
        g: 255,
        b: 255,
    };

    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Rgb { r, g, b }
    }

    pub const fn to_u32(self) -> u32 {
        (self.r as u32) << 16 | (self.g as u32) << 8 | self.b as u32
    }

    pub const fn from_u32(v: u32) -> Self {
        Rgb {
            r: (v >> 16) as u8,
            g: (v >> 8) as u8,
            b: v as u8,
        }
    }

    pub fn blend(self, other: Rgb, t: f32) -> Rgb {
        let t = t.clamp(0.0, 1.0);
        let lerp = |a: u8, b: u8| (a as f32 * t + b as f32 * (1.0 - t)).round() as u8;
        Rgb {
            r: lerp(self.r, other.r),
            g: lerp(self.g, other.g),
            b: lerp(self.b, other.b),
        }
    }

    /// How different two colours look, roughly.
    ///
    /// Contrast ratio only measures lightness, so it calls yellow and magenta
    /// identical when they happen to share a luminance. This is the "redmean"
    /// approximation: cheap, and close enough to perception to tell whether
    /// two colours read as the same one. Zero means identical; around 40 is
    /// the point where two colours stop being confusable.
    pub fn perceptual_distance(self, other: Rgb) -> f32 {
        let mean_red = (self.r as f32 + other.r as f32) / 2.0;
        let (dr, dg, db) = (
            self.r as f32 - other.r as f32,
            self.g as f32 - other.g as f32,
            self.b as f32 - other.b as f32,
        );

        let weight_red = 2.0 + mean_red / 256.0;
        let weight_blue = 2.0 + (255.0 - mean_red) / 256.0;
        (weight_red * dr * dr + 4.0 * dg * dg + weight_blue * db * db).sqrt()
    }

    /// WCAG relative luminance, with sRGB linearisation.
    pub fn relative_luminance(self) -> f32 {
        fn linear(c: u8) -> f32 {
            let c = c as f32 / 255.0;
            if c <= 0.04045 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        }
        0.2126 * linear(self.r) + 0.7152 * linear(self.g) + 0.0722 * linear(self.b)
    }

    /// WCAG contrast ratio, from 1.0 (identical) to 21.0 (black on white).
    /// Body text wants 4.5 or better.
    pub fn contrast_ratio(self, other: Rgb) -> f32 {
        let (a, b) = (self.relative_luminance(), other.relative_luminance());
        let (hi, lo) = if a > b { (a, b) } else { (b, a) };
        (hi + 0.05) / (lo + 0.05)
    }

    pub fn luminance(self) -> f32 {
        (0.2126 * self.r as f32 + 0.7152 * self.g as f32 + 0.0722 * self.b as f32) / 255.0
    }
}
