#[allow(unused_imports)]
use crate::prelude::*;

/// Desaturated amber.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "phosphor-amber-soft",
    description: "Desaturated amber.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x0d, 0x0a, 0x06),
    fg: Rgb::new(0xd4, 0xab, 0x6b),
    cursor: Rgb::new(0xc7, 0x9a, 0x56),
    selection_bg: Rgb::new(0x33, 0x24, 0x0f),
    accent: Rgb::new(0xbf, 0x8f, 0x45),
    normal: [
        Rgb::new(0x1b, 0x16, 0x10), //  0  black          #1b1610
        Rgb::new(0xd9, 0x73, 0x6a), //  1  red            #d9736a
        Rgb::new(0xb3, 0x9a, 0x52), //  2  green          #b39a52
        Rgb::new(0xd4, 0xab, 0x6b), //  3  yellow         #d4ab6b
        Rgb::new(0xbf, 0x94, 0x59), //  4  blue           #bf9459
        Rgb::new(0xd4, 0x9a, 0x7a), //  5  magenta        #d49a7a
        Rgb::new(0xc9, 0xa8, 0x6b), //  6  cyan           #c9a86b
        Rgb::new(0xd6, 0xc4, 0xa8), //  7  white          #d6c4a8
    ],
    bright: [
        Rgb::new(0x45, 0x3a, 0x28), //  8  bright black   #453a28
        Rgb::new(0xe0, 0x9a, 0x92), //  9  bright red     #e09a92
        Rgb::new(0xcc, 0xb8, 0x78), // 10  bright green   #ccb878
        Rgb::new(0xe3, 0xc4, 0x92), // 11  bright yellow  #e3c492
        Rgb::new(0xd4, 0xad, 0x84), // 12  bright blue    #d4ad84
        Rgb::new(0xe3, 0xb7, 0x9c), // 13  bright magenta #e3b79c
        Rgb::new(0xdc, 0xc2, 0x94), // 14  bright cyan    #dcc294
        Rgb::new(0xeb, 0xdf, 0xc9), // 15  bright white   #ebdfc9
    ],
};
