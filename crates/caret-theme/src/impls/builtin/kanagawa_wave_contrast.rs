#[allow(unused_imports)]
use crate::prelude::*;

/// Kanagawa at night, ink darker and the pigments stronger.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "kanagawa-wave-contrast",
    description: "Kanagawa at night, ink darker and the pigments stronger.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x0d, 0x0d, 0x11),
    fg: Rgb::new(0xe6, 0xe2, 0xcb),
    cursor: Rgb::new(0xd6, 0xcc, 0x99),
    selection_bg: Rgb::new(0x22, 0x3b, 0x4d),
    accent: Rgb::new(0x82, 0xa4, 0xe9),
    normal: [
        Rgb::new(0x04, 0x03, 0x0a), //  0  black          #04030a
        Rgb::new(0xdb, 0x46, 0x49), //  1  red            #db4649
        Rgb::new(0x84, 0xa7, 0x76), //  2  green          #84a776
        Rgb::new(0xd2, 0xb1, 0x75), //  3  yellow         #d2b175
        Rgb::new(0x82, 0xa4, 0xe9), //  4  blue           #82a4e9
        Rgb::new(0xa0, 0x87, 0xc8), //  5  magenta        #a087c8
        Rgb::new(0x76, 0xa7, 0x9a), //  6  cyan           #76a79a
        Rgb::new(0xd6, 0xcc, 0x99), //  7  white          #d6cc99
    ],
    bright: [
        Rgb::new(0x3f, 0x3e, 0x3a), //  8  bright black   #3f3e3a
        Rgb::new(0xff, 0x2a, 0x2a), //  9  bright red     #ff2a2a
        Rgb::new(0xa6, 0xce, 0x73), // 10  bright green   #a6ce73
        Rgb::new(0xf6, 0xce, 0x86), // 11  bright yellow  #f6ce86
        Rgb::new(0x84, 0xc1, 0xda), // 12  bright blue    #84c1da
        Rgb::new(0x9e, 0x94, 0xb7), // 13  bright magenta #9e94b7
        Rgb::new(0x84, 0xb9, 0xae), // 14  bright cyan    #84b9ae
        Rgb::new(0xe5, 0xdf, 0xbe), // 15  bright white   #e5dfbe
    ],
};
