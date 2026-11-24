#[allow(unused_imports)]
use crate::prelude::*;

/// Everforest after dusk, the greens holding their own.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "everforest-dark-contrast",
    description: "Everforest after dusk, the greens holding their own.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x13, 0x16, 0x19),
    fg: Rgb::new(0xe0, 0xd5, 0xbf),
    cursor: Rgb::new(0xde, 0xcf, 0xaf),
    selection_bg: Rgb::new(0x3f, 0x2c, 0x36),
    accent: Rgb::new(0xb3, 0xd0, 0x87),
    normal: [
        Rgb::new(0x1e, 0x22, 0x25), //  0  black          #1e2225
        Rgb::new(0xf7, 0x80, 0x82), //  1  red            #f78082
        Rgb::new(0xb3, 0xd0, 0x87), //  2  green          #b3d087
        Rgb::new(0xeb, 0xc8, 0x82), //  3  yellow         #ebc882
        Rgb::new(0x86, 0xcb, 0xc2), //  4  blue           #86cbc2
        Rgb::new(0xe3, 0x9d, 0xbe), //  5  magenta        #e39dbe
        Rgb::new(0x8a, 0xd0, 0x9b), //  6  cyan           #8ad09b
        Rgb::new(0xde, 0xcf, 0xaf), //  7  white          #decfaf
    ],
    bright: [
        Rgb::new(0x33, 0x3a, 0x3f), //  8  bright black   #333a3f
        Rgb::new(0xf7, 0x80, 0x82), //  9  bright red     #f78082
        Rgb::new(0xb3, 0xd0, 0x87), // 10  bright green   #b3d087
        Rgb::new(0xeb, 0xc8, 0x82), // 11  bright yellow  #ebc882
        Rgb::new(0x86, 0xcb, 0xc2), // 12  bright blue    #86cbc2
        Rgb::new(0xe3, 0x9d, 0xbe), // 13  bright magenta #e39dbe
        Rgb::new(0x8a, 0xd0, 0x9b), // 14  bright cyan    #8ad09b
        Rgb::new(0xec, 0xe8, 0xcf), // 15  bright white   #ece8cf
    ],
};
