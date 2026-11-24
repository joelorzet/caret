#[allow(unused_imports)]
use crate::prelude::*;

/// Monokai Pro on black, every colour at full strength.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "monokai-pro-contrast",
    description: "Monokai Pro on black, every colour at full strength.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x13, 0x12, 0x13),
    fg: Rgb::new(0xff, 0xf5, 0xf6),
    cursor: Rgb::new(0xff, 0xf3, 0xf4),
    selection_bg: Rgb::new(0x44, 0x43, 0x45),
    accent: Rgb::new(0xff, 0xdd, 0x78),
    normal: [
        Rgb::new(0x12, 0x10, 0x10), //  0  black          #121010
        Rgb::new(0xff, 0x78, 0x91), //  1  red            #ff7891
        Rgb::new(0xb8, 0xeb, 0x7b), //  2  green          #b8eb7b
        Rgb::new(0xff, 0xd4, 0x78), //  3  yellow         #ffd478
        Rgb::new(0xff, 0x95, 0x77), //  4  blue           #ff9577
        Rgb::new(0xaa, 0xab, 0xf6), //  5  magenta        #aaabf6
        Rgb::new(0x88, 0xea, 0xda), //  6  cyan           #88eada
        Rgb::new(0xff, 0xf3, 0xf4), //  7  white          #fff3f4
    ],
    bright: [
        Rgb::new(0x3f, 0x3a, 0x3a), //  8  bright black   #3f3a3a
        Rgb::new(0xff, 0x78, 0x91), //  9  bright red     #ff7891
        Rgb::new(0xb8, 0xeb, 0x7b), // 10  bright green   #b8eb7b
        Rgb::new(0xff, 0xd4, 0x78), // 11  bright yellow  #ffd478
        Rgb::new(0xff, 0x95, 0x77), // 12  bright blue    #ff9577
        Rgb::new(0xaa, 0xab, 0xf6), // 13  bright magenta #aaabf6
        Rgb::new(0x88, 0xea, 0xda), // 14  bright cyan    #88eada
        Rgb::new(0xff, 0xf3, 0xf4), // 15  bright white   #fff3f4
    ],
};
