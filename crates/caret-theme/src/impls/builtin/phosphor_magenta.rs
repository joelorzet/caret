#[allow(unused_imports)]
use crate::prelude::*;

/// Magenta CRT phosphor on black, the one nobody built.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "phosphor-magenta",
    description: "Magenta CRT phosphor on black, the one nobody built.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x08, 0x05, 0x07),
    fg: Rgb::new(0xff, 0x33, 0xcc),
    cursor: Rgb::new(0xff, 0x33, 0xcc),
    selection_bg: Rgb::new(0x35, 0x10, 0x46),
    accent: Rgb::new(0xff, 0x33, 0xcc),
    normal: [
        Rgb::new(0x1b, 0x0e, 0x19), //  0  black          #1b0e19
        Rgb::new(0xff, 0x69, 0x5c), //  1  red            #ff695c
        Rgb::new(0xff, 0x33, 0xcc), //  2  green          #ff33cc
        Rgb::new(0xff, 0xf0, 0x4d), //  3  yellow         #fff04d
        Rgb::new(0xc1, 0x36, 0xd3), //  4  blue           #c136d3
        Rgb::new(0xff, 0x80, 0xf9), //  5  magenta        #ff80f9
        Rgb::new(0xc8, 0x52, 0xff), //  6  cyan           #c852ff
        Rgb::new(0xff, 0xa8, 0xf2), //  7  white          #ffa8f2
    ],
    bright: [
        Rgb::new(0x4a, 0x2b, 0x46), //  8  bright black   #4a2b46
        Rgb::new(0xff, 0x98, 0x8f), //  9  bright red     #ff988f
        Rgb::new(0xff, 0x80, 0xdf), // 10  bright green   #ff80df
        Rgb::new(0xff, 0xf5, 0x8a), // 11  bright yellow  #fff58a
        Rgb::new(0xe0, 0x72, 0xee), // 12  bright blue    #e072ee
        Rgb::new(0xff, 0xa8, 0xfb), // 13  bright magenta #ffa8fb
        Rgb::new(0xdb, 0x8f, 0xff), // 14  bright cyan    #db8fff
        Rgb::new(0xff, 0xe0, 0xfa), // 15  bright white   #ffe0fa
    ],
};
