#[allow(unused_imports)]
use crate::prelude::*;

/// Orange CRT phosphor on black, the glow of a plasma display.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "phosphor-orange",
    description: "Orange CRT phosphor on black, the glow of a plasma display.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x08, 0x06, 0x05),
    fg: Rgb::new(0xff, 0x7e, 0x33),
    cursor: Rgb::new(0xff, 0x7e, 0x33),
    selection_bg: Rgb::new(0x46, 0x43, 0x10),
    accent: Rgb::new(0xff, 0x7e, 0x33),
    normal: [
        Rgb::new(0x1b, 0x14, 0x0e), //  0  black          #1b140e
        Rgb::new(0xff, 0x69, 0x5c), //  1  red            #ff695c
        Rgb::new(0xff, 0x7e, 0x33), //  2  green          #ff7e33
        Rgb::new(0xff, 0xc4, 0x4d), //  3  yellow         #ffc44d
        Rgb::new(0xd3, 0xa9, 0x36), //  4  blue           #d3a936
        Rgb::new(0xff, 0xc8, 0x80), //  5  magenta        #ffc880
        Rgb::new(0xff, 0xf3, 0x52), //  6  cyan           #fff352
        Rgb::new(0xff, 0xd1, 0xa8), //  7  white          #ffd1a8
    ],
    bright: [
        Rgb::new(0x4a, 0x3a, 0x2b), //  8  bright black   #4a3a2b
        Rgb::new(0xff, 0x98, 0x8f), //  9  bright red     #ff988f
        Rgb::new(0xff, 0xae, 0x80), // 10  bright green   #ffae80
        Rgb::new(0xff, 0xd8, 0x8a), // 11  bright yellow  #ffd88a
        Rgb::new(0xee, 0xcd, 0x72), // 12  bright blue    #eecd72
        Rgb::new(0xff, 0xd9, 0xa8), // 13  bright magenta #ffd9a8
        Rgb::new(0xff, 0xf8, 0x8f), // 14  bright cyan    #fff88f
        Rgb::new(0xff, 0xef, 0xe0), // 15  bright white   #ffefe0
    ],
};
