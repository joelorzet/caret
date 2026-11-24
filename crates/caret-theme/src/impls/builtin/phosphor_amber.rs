#[allow(unused_imports)]
use crate::prelude::*;

/// Amber CRT phosphor on black, the classic #ffb000.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "phosphor-amber",
    description: "Amber CRT phosphor on black, the classic #ffb000.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x0a, 0x07, 0x03),
    fg: Rgb::new(0xff, 0xb0, 0x00),
    cursor: Rgb::new(0xff, 0xb0, 0x00),
    selection_bg: Rgb::new(0x4a, 0x2f, 0x00),
    accent: Rgb::new(0xff, 0xb0, 0x00),
    normal: [
        Rgb::new(0x1a, 0x12, 0x05), //  0  black          #1a1205
        Rgb::new(0xff, 0x5f, 0x45), //  1  red            #ff5f45
        Rgb::new(0xc8, 0xa0, 0x20), //  2  green          #c8a020
        Rgb::new(0xff, 0xb0, 0x00), //  3  yellow         #ffb000
        Rgb::new(0xd9, 0x8f, 0x3f), //  4  blue           #d98f3f
        Rgb::new(0xff, 0x8f, 0x5e), //  5  magenta        #ff8f5e
        Rgb::new(0xe0, 0xa8, 0x56), //  6  cyan           #e0a856
        Rgb::new(0xf0, 0xd9, 0xa8), //  7  white          #f0d9a8
    ],
    bright: [
        Rgb::new(0x4a, 0x33, 0x10), //  8  bright black   #4a3310
        Rgb::new(0xff, 0x8f, 0x7a), //  9  bright red     #ff8f7a
        Rgb::new(0xe0, 0xbc, 0x4d), // 10  bright green   #e0bc4d
        Rgb::new(0xff, 0xc8, 0x4d), // 11  bright yellow  #ffc84d
        Rgb::new(0xed, 0xb0, 0x6b), // 12  bright blue    #edb06b
        Rgb::new(0xff, 0xb0, 0x8a), // 13  bright magenta #ffb08a
        Rgb::new(0xf0, 0xc5, 0x7d), // 14  bright cyan    #f0c57d
        Rgb::new(0xff, 0xf0, 0xd0), // 15  bright white   #fff0d0
    ],
};
