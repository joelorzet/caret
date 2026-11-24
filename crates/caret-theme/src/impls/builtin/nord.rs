#[allow(unused_imports)]
use crate::prelude::*;

/// Nord's arctic palette.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "nord",
    description: "Nord's arctic palette.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x2e, 0x34, 0x40),
    fg: Rgb::new(0xd8, 0xde, 0xe9),
    cursor: Rgb::new(0xd8, 0xde, 0xe9),
    selection_bg: Rgb::new(0x43, 0x4c, 0x5e),
    accent: Rgb::new(0x88, 0xc0, 0xd0),
    normal: [
        Rgb::new(0x3b, 0x42, 0x52), //  0  black          #3b4252
        Rgb::new(0xbf, 0x61, 0x6a), //  1  red            #bf616a
        Rgb::new(0xa3, 0xbe, 0x8c), //  2  green          #a3be8c
        Rgb::new(0xeb, 0xcb, 0x8b), //  3  yellow         #ebcb8b
        Rgb::new(0x81, 0xa1, 0xc1), //  4  blue           #81a1c1
        Rgb::new(0xb4, 0x8e, 0xad), //  5  magenta        #b48ead
        Rgb::new(0x88, 0xc0, 0xd0), //  6  cyan           #88c0d0
        Rgb::new(0xe5, 0xe9, 0xf0), //  7  white          #e5e9f0
    ],
    bright: [
        Rgb::new(0x4c, 0x56, 0x6a), //  8  bright black   #4c566a
        Rgb::new(0xbf, 0x61, 0x6a), //  9  bright red     #bf616a
        Rgb::new(0xa3, 0xbe, 0x8c), // 10  bright green   #a3be8c
        Rgb::new(0xeb, 0xcb, 0x8b), // 11  bright yellow  #ebcb8b
        Rgb::new(0x81, 0xa1, 0xc1), // 12  bright blue    #81a1c1
        Rgb::new(0xb4, 0x8e, 0xad), // 13  bright magenta #b48ead
        Rgb::new(0x8f, 0xbc, 0xbb), // 14  bright cyan    #8fbcbb
        Rgb::new(0xec, 0xef, 0xf4), // 15  bright white   #eceff4
    ],
};
