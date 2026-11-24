#[allow(unused_imports)]
use crate::prelude::*;

const TAG_SHIFT: u32 = 24;
const TAG_DEFAULT: u32 = 0x00;
const TAG_INDEXED: u32 = 0x01;
const TAG_RGB: u32 = 0x02;

impl Color {
    pub const DEFAULT: Color = Color(TAG_DEFAULT << TAG_SHIFT);

    pub const fn indexed(i: u8) -> Color {
        Color(TAG_INDEXED << TAG_SHIFT | i as u32)
    }

    pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color(TAG_RGB << TAG_SHIFT | (r as u32) << 16 | (g as u32) << 8 | b as u32)
    }

    pub const fn from_rgb(c: Rgb) -> Color {
        Color::rgb(c.r, c.g, c.b)
    }

    pub const fn kind(self) -> ColorKind {
        match self.0 >> TAG_SHIFT {
            TAG_INDEXED => ColorKind::Indexed(self.0 as u8),
            TAG_RGB => ColorKind::Rgb(Rgb::from_u32(self.0 & 0x00ff_ffff)),
            _ => ColorKind::Default,
        }
    }

    pub const fn is_default(self) -> bool {
        self.0 >> TAG_SHIFT == TAG_DEFAULT
    }

    pub const BLACK: Color = Color::indexed(0);
    pub const RED: Color = Color::indexed(1);
    pub const GREEN: Color = Color::indexed(2);
    pub const YELLOW: Color = Color::indexed(3);
    pub const BLUE: Color = Color::indexed(4);
    pub const MAGENTA: Color = Color::indexed(5);
    pub const CYAN: Color = Color::indexed(6);
    pub const WHITE: Color = Color::indexed(7);
}

impl core::fmt::Debug for Color {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.kind() {
            ColorKind::Default => f.write_str("Default"),
            ColorKind::Indexed(i) => write!(f, "Indexed({i})"),
            ColorKind::Rgb(c) => write!(f, "Rgb({:02x}{:02x}{:02x})", c.r, c.g, c.b),
        }
    }
}

pub fn xterm_256(i: u8) -> Option<Rgb> {
    match i {
        0..=15 => None,
        16..=231 => {
            const STEPS: [u8; 6] = [0, 95, 135, 175, 215, 255];
            let i = i as usize - 16;
            Some(Rgb::new(STEPS[i / 36], STEPS[(i / 6) % 6], STEPS[i % 6]))
        }
        232..=255 => {
            // The grey ramp starts just off black and climbs in even steps.
            const FIRST: u8 = 232;
            const BASE: u16 = 8;
            const STEP: u16 = 10;
            let v = (BASE + (i - FIRST) as u16 * STEP) as u8;
            Some(Rgb::new(v, v, v))
        }
    }
}
