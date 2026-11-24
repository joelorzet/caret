#[allow(unused_imports)]
use crate::prelude::*;

/// Paper-white CRT phosphor on black, colour kept for errors alone.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "phosphor-white",
    description: "Paper-white CRT phosphor on black, colour kept for errors alone.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x06, 0x06, 0x06),
    fg: Rgb::new(0xd6, 0xd6, 0xd6),
    cursor: Rgb::new(0xd6, 0xd6, 0xd6),
    selection_bg: Rgb::new(0x2b, 0x2b, 0x2b),
    accent: Rgb::new(0xd6, 0xd6, 0xd6),
    normal: [
        Rgb::new(0x14, 0x14, 0x14), //  0  black          #141414
        Rgb::new(0xff, 0x69, 0x5c), //  1  red            #ff695c
        Rgb::new(0x7e, 0x99, 0xb4), //  2  green          #7e99b4
        Rgb::new(0xff, 0xf0, 0x4d), //  3  yellow         #fff04d
        Rgb::new(0x65, 0x9c, 0xa4), //  4  blue           #659ca4
        Rgb::new(0xaf, 0xc6, 0xd0), //  5  magenta        #afc6d0
        Rgb::new(0x92, 0xbf, 0xbc), //  6  cyan           #92bfbc
        Rgb::new(0xd4, 0xd4, 0xd4), //  7  white          #d4d4d4
    ],
    bright: [
        Rgb::new(0x3b, 0x3b, 0x3b), //  8  bright black   #3b3b3b
        Rgb::new(0xff, 0x98, 0x8f), //  9  bright red     #ff988f
        Rgb::new(0xaf, 0xbf, 0xd0), // 10  bright green   #afbfd0
        Rgb::new(0xff, 0xf5, 0x8a), // 11  bright yellow  #fff58a
        Rgb::new(0x9b, 0xbf, 0xc5), // 12  bright blue    #9bbfc5
        Rgb::new(0xc8, 0xd8, 0xdf), // 13  bright magenta #c8d8df
        Rgb::new(0xb8, 0xd5, 0xd4), // 14  bright cyan    #b8d5d4
        Rgb::new(0xf0, 0xf0, 0xf0), // 15  bright white   #f0f0f0
    ],
};
