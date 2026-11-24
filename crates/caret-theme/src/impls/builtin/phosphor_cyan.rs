#[allow(unused_imports)]
use crate::prelude::*;

/// Cyan CRT phosphor on black, cold and clinical.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "phosphor-cyan",
    description: "Cyan CRT phosphor on black, cold and clinical.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x05, 0x07, 0x08),
    fg: Rgb::new(0x33, 0xee, 0xff),
    cursor: Rgb::new(0x33, 0xee, 0xff),
    selection_bg: Rgb::new(0x10, 0x23, 0x46),
    accent: Rgb::new(0x33, 0xee, 0xff),
    normal: [
        Rgb::new(0x0e, 0x19, 0x1b), //  0  black          #0e191b
        Rgb::new(0xff, 0x69, 0x5c), //  1  red            #ff695c
        Rgb::new(0x33, 0xee, 0xff), //  2  green          #33eeff
        Rgb::new(0xff, 0xf0, 0x4d), //  3  yellow         #fff04d
        Rgb::new(0x36, 0x8c, 0xd3), //  4  blue           #368cd3
        Rgb::new(0x80, 0xdb, 0xff), //  5  magenta        #80dbff
        Rgb::new(0x52, 0x8e, 0xff), //  6  cyan           #528eff
        Rgb::new(0xa8, 0xef, 0xff), //  7  white          #a8efff
    ],
    bright: [
        Rgb::new(0x2b, 0x45, 0x4a), //  8  bright black   #2b454a
        Rgb::new(0xff, 0x98, 0x8f), //  9  bright red     #ff988f
        Rgb::new(0x80, 0xf4, 0xff), // 10  bright green   #80f4ff
        Rgb::new(0xff, 0xf5, 0x8a), // 11  bright yellow  #fff58a
        Rgb::new(0x72, 0xb6, 0xee), // 12  bright blue    #72b6ee
        Rgb::new(0xa8, 0xe6, 0xff), // 13  bright magenta #a8e6ff
        Rgb::new(0x8f, 0xb6, 0xff), // 14  bright cyan    #8fb6ff
        Rgb::new(0xe0, 0xf9, 0xff), // 15  bright white   #e0f9ff
    ],
};
