#[allow(unused_imports)]
use crate::prelude::*;

/// Iceberg, a cold, bluish dark theme.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "iceberg",
    description: "Iceberg, a cold, bluish dark theme.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x16, 0x18, 0x21),
    fg: Rgb::new(0xc6, 0xc8, 0xd1),
    cursor: Rgb::new(0xc6, 0xc8, 0xd1),
    selection_bg: Rgb::new(0x27, 0x2c, 0x42),
    accent: Rgb::new(0x84, 0xa0, 0xc6),
    normal: [
        Rgb::new(0x1e, 0x21, 0x32), //  0  black          #1e2132
        Rgb::new(0xe2, 0x78, 0x78), //  1  red            #e27878
        Rgb::new(0xb4, 0xbe, 0x82), //  2  green          #b4be82
        Rgb::new(0xe2, 0xa4, 0x78), //  3  yellow         #e2a478
        Rgb::new(0x84, 0xa0, 0xc6), //  4  blue           #84a0c6
        Rgb::new(0xa0, 0x93, 0xc7), //  5  magenta        #a093c7
        Rgb::new(0x89, 0xb8, 0xc2), //  6  cyan           #89b8c2
        Rgb::new(0xc6, 0xc8, 0xd1), //  7  white          #c6c8d1
    ],
    bright: [
        Rgb::new(0x6b, 0x70, 0x89), //  8  bright black   #6b7089
        Rgb::new(0xe9, 0x89, 0x89), //  9  bright red     #e98989
        Rgb::new(0xc0, 0xca, 0x8e), // 10  bright green   #c0ca8e
        Rgb::new(0xe9, 0xb1, 0x89), // 11  bright yellow  #e9b189
        Rgb::new(0x91, 0xac, 0xd1), // 12  bright blue    #91acd1
        Rgb::new(0xad, 0xa0, 0xd3), // 13  bright magenta #ada0d3
        Rgb::new(0x95, 0xc4, 0xce), // 14  bright cyan    #95c4ce
        Rgb::new(0xd2, 0xd4, 0xde), // 15  bright white   #d2d4de
    ],
};
