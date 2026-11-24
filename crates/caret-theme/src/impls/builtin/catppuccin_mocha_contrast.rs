#[allow(unused_imports)]
use crate::prelude::*;

/// Mocha served black, the pastels standing clear of it.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "catppuccin-mocha-contrast",
    description: "Mocha served black, the pastels standing clear of it.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x0d, 0x0d, 0x13),
    fg: Rgb::new(0xd9, 0xe0, 0xf8),
    cursor: Rgb::new(0xfa, 0xe1, 0xdd),
    selection_bg: Rgb::new(0x42, 0x44, 0x54),
    accent: Rgb::new(0xd1, 0xaa, 0xff),
    normal: [
        Rgb::new(0x1d, 0x1e, 0x26), //  0  black          #1d1e26
        Rgb::new(0xff, 0x8e, 0xae), //  1  red            #ff8eae
        Rgb::new(0xa9, 0xef, 0xa4), //  2  green          #a9efa4
        Rgb::new(0xff, 0xe7, 0xb3), //  3  yellow         #ffe7b3
        Rgb::new(0x93, 0xbc, 0xff), //  4  blue           #93bcff
        Rgb::new(0xfd, 0xc3, 0xed), //  5  magenta        #fdc3ed
        Rgb::new(0x97, 0xf0, 0xe1), //  6  cyan           #97f0e1
        Rgb::new(0xbe, 0xc7, 0xe7), //  7  white          #bec7e7
    ],
    bright: [
        Rgb::new(0x30, 0x32, 0x3e), //  8  bright black   #30323e
        Rgb::new(0xff, 0x8e, 0xae), //  9  bright red     #ff8eae
        Rgb::new(0xa9, 0xef, 0xa4), // 10  bright green   #a9efa4
        Rgb::new(0xff, 0xe7, 0xb3), // 11  bright yellow  #ffe7b3
        Rgb::new(0x93, 0xbc, 0xff), // 12  bright blue    #93bcff
        Rgb::new(0xfd, 0xc3, 0xed), // 13  bright magenta #fdc3ed
        Rgb::new(0x97, 0xf0, 0xe1), // 14  bright cyan    #97f0e1
        Rgb::new(0xac, 0xb4, 0xd3), // 15  bright white   #acb4d3
    ],
};
