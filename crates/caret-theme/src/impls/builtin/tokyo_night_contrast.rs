#[allow(unused_imports)]
use crate::prelude::*;

/// Tokyo Night at midnight, with the neon turned up.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "tokyo-night-contrast",
    description: "Tokyo Night at midnight, with the neon turned up.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x0b, 0x0b, 0x10),
    fg: Rgb::new(0xce, 0xd7, 0xfa),
    cursor: Rgb::new(0xc1, 0xcc, 0xfd),
    selection_bg: Rgb::new(0x1e, 0x27, 0x41),
    accent: Rgb::new(0x83, 0xab, 0xff),
    normal: [
        Rgb::new(0x09, 0x09, 0x0d), //  0  black          #09090d
        Rgb::new(0xff, 0x7f, 0x97), //  1  red            #ff7f97
        Rgb::new(0xaa, 0xe1, 0x6f), //  2  green          #aae16f
        Rgb::new(0xf4, 0xbc, 0x6a), //  3  yellow         #f4bc6a
        Rgb::new(0x83, 0xab, 0xff), //  4  blue           #83abff
        Rgb::new(0xc1, 0x9f, 0xff), //  5  magenta        #c19fff
        Rgb::new(0x8d, 0xd5, 0xff), //  6  cyan           #8dd5ff
        Rgb::new(0xad, 0xb7, 0xe1), //  7  white          #adb7e1
    ],
    bright: [
        Rgb::new(0x24, 0x28, 0x39), //  8  bright black   #242839
        Rgb::new(0xff, 0x8a, 0xa0), //  9  bright red     #ff8aa0
        Rgb::new(0xc2, 0xff, 0x80), // 10  bright green   #c2ff80
        Rgb::new(0xff, 0xaa, 0x77), // 11  bright yellow  #ffaa77
        Rgb::new(0x8d, 0xb1, 0xff), // 12  bright blue    #8db1ff
        Rgb::new(0xc1, 0x9f, 0xff), // 13  bright magenta #c19fff
        Rgb::new(0x07, 0xda, 0xff), // 14  bright cyan    #07daff
        Rgb::new(0xc1, 0xcc, 0xfd), // 15  bright white   #c1ccfd
    ],
};
