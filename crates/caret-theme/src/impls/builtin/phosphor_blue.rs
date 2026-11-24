#[allow(unused_imports)]
use crate::prelude::*;

/// Blue CRT phosphor on black, like an old vector display.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "phosphor-blue",
    description: "Blue CRT phosphor on black, like an old vector display.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x05, 0x07, 0x08),
    fg: Rgb::new(0x33, 0x92, 0xff),
    cursor: Rgb::new(0x33, 0x92, 0xff),
    selection_bg: Rgb::new(0x10, 0x46, 0x44),
    accent: Rgb::new(0x33, 0x92, 0xff),
    normal: [
        Rgb::new(0x0e, 0x15, 0x1b), //  0  black          #0e151b
        Rgb::new(0xff, 0x69, 0x5c), //  1  red            #ff695c
        Rgb::new(0x33, 0x92, 0xff), //  2  green          #3392ff
        Rgb::new(0xff, 0xf0, 0x4d), //  3  yellow         #fff04d
        Rgb::new(0x36, 0xb9, 0xd3), //  4  blue           #36b9d3
        Rgb::new(0x80, 0xd4, 0xff), //  5  magenta        #80d4ff
        Rgb::new(0x52, 0xff, 0xf9), //  6  cyan           #52fff9
        Rgb::new(0xa8, 0xd9, 0xff), //  7  white          #a8d9ff
    ],
    bright: [
        Rgb::new(0x2b, 0x3d, 0x4a), //  8  bright black   #2b3d4a
        Rgb::new(0xff, 0x98, 0x8f), //  9  bright red     #ff988f
        Rgb::new(0x80, 0xbb, 0xff), // 10  bright green   #80bbff
        Rgb::new(0xff, 0xf5, 0x8a), // 11  bright yellow  #fff58a
        Rgb::new(0x72, 0xda, 0xee), // 12  bright blue    #72daee
        Rgb::new(0xa8, 0xe2, 0xff), // 13  bright magenta #a8e2ff
        Rgb::new(0x8f, 0xff, 0xfb), // 14  bright cyan    #8ffffb
        Rgb::new(0xe0, 0xf2, 0xff), // 15  bright white   #e0f2ff
    ],
};
