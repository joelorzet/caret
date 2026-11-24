#[allow(unused_imports)]
use crate::prelude::*;

/// GitHub dark, darker, with the syntax colours pulled forward.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "github-dark-contrast",
    description: "GitHub dark, darker, with the syntax colours pulled forward.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x05, 0x07, 0x0a),
    fg: Rgb::new(0xc7, 0xc5, 0xc2),
    cursor: Rgb::new(0x6c, 0xb1, 0xff),
    selection_bg: Rgb::new(0x1d, 0x3b, 0x5a),
    accent: Rgb::new(0x6c, 0xb1, 0xff),
    normal: [
        Rgb::new(0x1e, 0x21, 0x25), //  0  black          #1e2125
        Rgb::new(0xff, 0x8b, 0x83), //  1  red            #ff8b83
        Rgb::new(0x42, 0xd5, 0x57), //  2  green          #42d557
        Rgb::new(0xf8, 0xb1, 0x1c), //  3  yellow         #f8b11c
        Rgb::new(0x6c, 0xb1, 0xff), //  4  blue           #6cb1ff
        Rgb::new(0xc4, 0x9a, 0xff), //  5  magenta        #c49aff
        Rgb::new(0x3d, 0xdd, 0xe9), //  6  cyan           #3ddde9
        Rgb::new(0xb8, 0xc2, 0xce), //  7  white          #b8c2ce
    ],
    bright: [
        Rgb::new(0x3c, 0x41, 0x47), //  8  bright black   #3c4147
        Rgb::new(0xff, 0xac, 0xa4), //  9  bright red     #ffaca4
        Rgb::new(0x5a, 0xe9, 0x6a), // 10  bright green   #5ae96a
        Rgb::new(0xfc, 0xc5, 0x42), // 11  bright yellow  #fcc542
        Rgb::new(0x89, 0xc8, 0xff), // 12  bright blue    #89c8ff
        Rgb::new(0xd7, 0xb2, 0xff), // 13  bright magenta #d7b2ff
        Rgb::new(0x58, 0xe9, 0xf3), // 14  bright cyan    #58e9f3
        Rgb::new(0xf0, 0xf7, 0xfe), // 15  bright white   #f0f7fe
    ],
};
