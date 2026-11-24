#[allow(unused_imports)]
use crate::prelude::*;

/// Parses `#rgb`, `#rrggbb` or the same without the hash.
pub fn parse_hex(text: &str) -> Option<Rgb> {
    let s = text.trim().trim_start_matches('#');
    let value = u32::from_str_radix(s, 16).ok()?;
    /// `#rgb` has one hex digit per channel, `#rrggbb` has two.
    const SHORT: usize = 3;
    const LONG: usize = 6;
    /// A nibble repeated into a byte: 0xf becomes 0xff, which is 0xf * 17.
    const WIDEN: u32 = 0x11;
    const NIBBLE: u32 = 0xf;

    match s.len() {
        SHORT => {
            let (r, g, b) = ((value >> 8) & NIBBLE, (value >> 4) & NIBBLE, value & NIBBLE);
            Some(Rgb::new(
                (r * WIDEN) as u8,
                (g * WIDEN) as u8,
                (b * WIDEN) as u8,
            ))
        }
        LONG => Some(Rgb::from_u32(value)),
        _ => None,
    }
}
