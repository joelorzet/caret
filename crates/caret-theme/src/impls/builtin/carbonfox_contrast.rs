#[allow(unused_imports)]
use crate::prelude::*;

/// Carbonfox with the carbon deepened and the colours lifted.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "carbonfox-contrast",
    description: "Carbonfox with the carbon deepened and the colours lifted.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x09, 0x09, 0x09),
    fg: Rgb::new(0xf5, 0xf7, 0xfa),
    cursor: Rgb::new(0xbe, 0xc0, 0xc4),
    selection_bg: Rgb::new(0x20, 0x20, 0x20),
    accent: Rgb::new(0x88, 0xb3, 0xff),
    normal: [
        Rgb::new(0x11, 0x11, 0x11), //  0  black          #111111
        Rgb::new(0xff, 0x59, 0xa1), //  1  red            #ff59a1
        Rgb::new(0x15, 0xf0, 0x78), //  2  green          #15f078
        Rgb::new(0x00, 0xeb, 0xe7), //  3  yellow         #00ebe7
        Rgb::new(0x88, 0xb3, 0xff), //  4  blue           #88b3ff
        Rgb::new(0xc6, 0xa2, 0xff), //  5  magenta        #c6a2ff
        Rgb::new(0x4b, 0xba, 0xff), //  6  cyan           #4bbaff
        Rgb::new(0xe3, 0xe3, 0xe4), //  7  white          #e3e3e4
    ],
    bright: [
        Rgb::new(0x28, 0x28, 0x28), //  8  bright black   #282828
        Rgb::new(0xff, 0x72, 0xaf), //  9  bright red     #ff72af
        Rgb::new(0x4b, 0xe0, 0x8d), // 10  bright green   #4be08d
        Rgb::new(0x2a, 0xea, 0xe6), // 11  bright yellow  #2aeae6
        Rgb::new(0x9a, 0xbf, 0xff), // 12  bright blue    #9abfff
        Rgb::new(0xcf, 0xb0, 0xff), // 13  bright magenta #cfb0ff
        Rgb::new(0x67, 0xc5, 0xff), // 14  bright cyan    #67c5ff
        Rgb::new(0xe7, 0xe7, 0xe8), // 15  bright white   #e7e7e8
    ],
};
