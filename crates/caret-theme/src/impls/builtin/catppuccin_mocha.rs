#[allow(unused_imports)]
use crate::prelude::*;

/// Catppuccin, Mocha flavour.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "catppuccin-mocha",
    description: "Catppuccin, Mocha flavour.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x1e, 0x1e, 0x2e),
    fg: Rgb::new(0xcd, 0xd6, 0xf4),
    cursor: Rgb::new(0xf5, 0xe0, 0xdc),
    selection_bg: Rgb::new(0x58, 0x5b, 0x70),
    accent: Rgb::new(0xcb, 0xa6, 0xf7),
    normal: [
        Rgb::new(0x45, 0x47, 0x5a), //  0  black          #45475a
        Rgb::new(0xf3, 0x8b, 0xa8), //  1  red            #f38ba8
        Rgb::new(0xa6, 0xe3, 0xa1), //  2  green          #a6e3a1
        Rgb::new(0xf9, 0xe2, 0xaf), //  3  yellow         #f9e2af
        Rgb::new(0x89, 0xb4, 0xfa), //  4  blue           #89b4fa
        Rgb::new(0xf5, 0xc2, 0xe7), //  5  magenta        #f5c2e7
        Rgb::new(0x94, 0xe2, 0xd5), //  6  cyan           #94e2d5
        Rgb::new(0xba, 0xc2, 0xde), //  7  white          #bac2de
    ],
    bright: [
        Rgb::new(0x58, 0x5b, 0x70), //  8  bright black   #585b70
        Rgb::new(0xf3, 0x8b, 0xa8), //  9  bright red     #f38ba8
        Rgb::new(0xa6, 0xe3, 0xa1), // 10  bright green   #a6e3a1
        Rgb::new(0xf9, 0xe2, 0xaf), // 11  bright yellow  #f9e2af
        Rgb::new(0x89, 0xb4, 0xfa), // 12  bright blue    #89b4fa
        Rgb::new(0xf5, 0xc2, 0xe7), // 13  bright magenta #f5c2e7
        Rgb::new(0x94, 0xe2, 0xd5), // 14  bright cyan    #94e2d5
        Rgb::new(0xa6, 0xad, 0xc8), // 15  bright white   #a6adc8
    ],
};
