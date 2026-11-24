#[allow(unused_imports)]
use crate::prelude::*;

/// Kanagawa Wave, after Hokusai's Great Wave.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "kanagawa-wave",
    description: "Kanagawa Wave, after Hokusai's Great Wave.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x1f, 0x1f, 0x28),
    fg: Rgb::new(0xdc, 0xd7, 0xba),
    cursor: Rgb::new(0xc8, 0xc0, 0x93),
    selection_bg: Rgb::new(0x2d, 0x4f, 0x67),
    accent: Rgb::new(0x7e, 0x9c, 0xd8),
    normal: [
        Rgb::new(0x09, 0x06, 0x18), //  0  black          #090618
        Rgb::new(0xc3, 0x40, 0x43), //  1  red            #c34043
        Rgb::new(0x76, 0x94, 0x6a), //  2  green          #76946a
        Rgb::new(0xc0, 0xa3, 0x6e), //  3  yellow         #c0a36e
        Rgb::new(0x7e, 0x9c, 0xd8), //  4  blue           #7e9cd8
        Rgb::new(0x95, 0x7f, 0xb8), //  5  magenta        #957fb8
        Rgb::new(0x6a, 0x95, 0x89), //  6  cyan           #6a9589
        Rgb::new(0xc8, 0xc0, 0x93), //  7  white          #c8c093
    ],
    bright: [
        Rgb::new(0x72, 0x71, 0x69), //  8  bright black   #727169
        Rgb::new(0xe8, 0x24, 0x24), //  9  bright red     #e82424
        Rgb::new(0x98, 0xbb, 0x6c), // 10  bright green   #98bb6c
        Rgb::new(0xe6, 0xc3, 0x84), // 11  bright yellow  #e6c384
        Rgb::new(0x7f, 0xb4, 0xca), // 12  bright blue    #7fb4ca
        Rgb::new(0x93, 0x8a, 0xa9), // 13  bright magenta #938aa9
        Rgb::new(0x7a, 0xa8, 0x9f), // 14  bright cyan    #7aa89f
        Rgb::new(0xdc, 0xd7, 0xba), // 15  bright white   #dcd7ba
    ],
};
