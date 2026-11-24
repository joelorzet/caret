#[allow(unused_imports)]
use crate::prelude::*;

/// Duskfox, Nightfox's purple twilight variant.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "duskfox",
    description: "Duskfox, Nightfox's purple twilight variant.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x23, 0x21, 0x36),
    fg: Rgb::new(0xe0, 0xde, 0xf4),
    cursor: Rgb::new(0xcd, 0xcb, 0xe0),
    selection_bg: Rgb::new(0x43, 0x3c, 0x59),
    accent: Rgb::new(0xc4, 0xa7, 0xe7),
    normal: [
        Rgb::new(0x39, 0x35, 0x52), //  0  black          #393552
        Rgb::new(0xeb, 0x6f, 0x92), //  1  red            #eb6f92
        Rgb::new(0xa3, 0xbe, 0x8c), //  2  green          #a3be8c
        Rgb::new(0xf6, 0xc1, 0x77), //  3  yellow         #f6c177
        Rgb::new(0x56, 0x9f, 0xba), //  4  blue           #569fba
        Rgb::new(0xc4, 0xa7, 0xe7), //  5  magenta        #c4a7e7
        Rgb::new(0x9c, 0xcf, 0xd8), //  6  cyan           #9ccfd8
        Rgb::new(0xe0, 0xde, 0xf4), //  7  white          #e0def4
    ],
    bright: [
        Rgb::new(0x47, 0x40, 0x7d), //  8  bright black   #47407d
        Rgb::new(0xf0, 0x83, 0xa2), //  9  bright red     #f083a2
        Rgb::new(0xb1, 0xd1, 0x96), // 10  bright green   #b1d196
        Rgb::new(0xf9, 0xcb, 0x8c), // 11  bright yellow  #f9cb8c
        Rgb::new(0x65, 0xb1, 0xcd), // 12  bright blue    #65b1cd
        Rgb::new(0xcc, 0xb1, 0xed), // 13  bright magenta #ccb1ed
        Rgb::new(0xa6, 0xda, 0xe3), // 14  bright cyan    #a6dae3
        Rgb::new(0xe2, 0xe0, 0xf7), // 15  bright white   #e2e0f7
    ],
};
