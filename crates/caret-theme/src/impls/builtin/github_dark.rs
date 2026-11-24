#[allow(unused_imports)]
use crate::prelude::*;

/// GitHub dark, the default variant.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "github-dark",
    description: "GitHub dark, the default variant.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x0d, 0x11, 0x17),
    fg: Rgb::new(0xb3, 0xb1, 0xad),
    cursor: Rgb::new(0x58, 0xa6, 0xff),
    selection_bg: Rgb::new(0x26, 0x4f, 0x78),
    accent: Rgb::new(0x58, 0xa6, 0xff),
    normal: [
        Rgb::new(0x48, 0x4f, 0x58), //  0  black          #484f58
        Rgb::new(0xff, 0x7b, 0x72), //  1  red            #ff7b72
        Rgb::new(0x3f, 0xb9, 0x50), //  2  green          #3fb950
        Rgb::new(0xd2, 0x99, 0x22), //  3  yellow         #d29922
        Rgb::new(0x58, 0xa6, 0xff), //  4  blue           #58a6ff
        Rgb::new(0xbc, 0x8c, 0xff), //  5  magenta        #bc8cff
        Rgb::new(0x39, 0xc5, 0xcf), //  6  cyan           #39c5cf
        Rgb::new(0xb1, 0xba, 0xc4), //  7  white          #b1bac4
    ],
    bright: [
        Rgb::new(0x6e, 0x76, 0x81), //  8  bright black   #6e7681
        Rgb::new(0xff, 0xa1, 0x98), //  9  bright red     #ffa198
        Rgb::new(0x56, 0xd3, 0x64), // 10  bright green   #56d364
        Rgb::new(0xe3, 0xb3, 0x41), // 11  bright yellow  #e3b341
        Rgb::new(0x79, 0xc0, 0xff), // 12  bright blue    #79c0ff
        Rgb::new(0xd2, 0xa8, 0xff), // 13  bright magenta #d2a8ff
        Rgb::new(0x56, 0xd4, 0xdd), // 14  bright cyan    #56d4dd
        Rgb::new(0xf0, 0xf6, 0xfc), // 15  bright white   #f0f6fc
    ],
};
