#[allow(unused_imports)]
use crate::prelude::*;

/// Yellow CRT phosphor on black, brighter and greener than amber.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "phosphor-yellow",
    description: "Yellow CRT phosphor on black, brighter and greener than amber.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x08, 0x08, 0x05),
    fg: Rgb::new(0xff, 0xee, 0x33),
    cursor: Rgb::new(0xff, 0xee, 0x33),
    selection_bg: Rgb::new(0x2c, 0x46, 0x10),
    accent: Rgb::new(0xff, 0xee, 0x33),
    normal: [
        Rgb::new(0x1b, 0x1b, 0x0e), //  0  black          #1b1b0e
        Rgb::new(0xff, 0x69, 0x5c), //  1  red            #ff695c
        Rgb::new(0xff, 0xee, 0x33), //  2  green          #ffee33
        Rgb::new(0xff, 0xf0, 0x4d), //  3  yellow         #fff04d
        Rgb::new(0xa7, 0xd3, 0x36), //  4  blue           #a7d336
        Rgb::new(0xf0, 0xff, 0x80), //  5  magenta        #f0ff80
        Rgb::new(0xab, 0xff, 0x52), //  6  cyan           #abff52
        Rgb::new(0xfe, 0xff, 0xa8), //  7  white          #feffa8
    ],
    bright: [
        Rgb::new(0x4a, 0x4a, 0x2b), //  8  bright black   #4a4a2b
        Rgb::new(0xff, 0x98, 0x8f), //  9  bright red     #ff988f
        Rgb::new(0xff, 0xf4, 0x80), // 10  bright green   #fff480
        Rgb::new(0xff, 0xf5, 0x8a), // 11  bright yellow  #fff58a
        Rgb::new(0xcb, 0xee, 0x72), // 12  bright blue    #cbee72
        Rgb::new(0xf5, 0xff, 0xa8), // 13  bright magenta #f5ffa8
        Rgb::new(0xc9, 0xff, 0x8f), // 14  bright cyan    #c9ff8f
        Rgb::new(0xfe, 0xff, 0xe0), // 15  bright white   #feffe0
    ],
};
