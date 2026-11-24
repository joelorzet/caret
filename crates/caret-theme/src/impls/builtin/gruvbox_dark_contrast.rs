#[allow(unused_imports)]
use crate::prelude::*;

/// Gruvbox with the retro warmth kept and the background taken away.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "gruvbox-dark-contrast",
    description: "Gruvbox with the retro warmth kept and the background taken away.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x11, 0x11, 0x11),
    fg: Rgb::new(0xf2, 0xe5, 0xc4),
    cursor: Rgb::new(0xf5, 0xe3, 0xb4),
    selection_bg: Rgb::new(0x3c, 0x37, 0x34),
    accent: Rgb::new(0xff, 0xc7, 0x44),
    normal: [
        Rgb::new(0x11, 0x11, 0x11), //  0  black          #111111
        Rgb::new(0xfc, 0x18, 0x0e), //  1  red            #fc180e
        Rgb::new(0xd1, 0xd0, 0x09), //  2  green          #d1d009
        Rgb::new(0xfa, 0xaf, 0x1e), //  3  yellow         #faaf1e
        Rgb::new(0x45, 0xa8, 0xac), //  4  blue           #45a8ac
        Rgb::new(0xc5, 0x6a, 0x94), //  5  magenta        #c56a94
        Rgb::new(0x73, 0xb0, 0x75), //  6  cyan           #73b075
        Rgb::new(0xb7, 0xa6, 0x8e), //  7  white          #b7a68e
    ],
    bright: [
        Rgb::new(0x51, 0x48, 0x3f), //  8  bright black   #51483f
        Rgb::new(0xff, 0x5c, 0x49), //  9  bright red     #ff5c49
        Rgb::new(0xe9, 0xee, 0x16), // 10  bright green   #e9ee16
        Rgb::new(0xff, 0xc7, 0x44), // 11  bright yellow  #ffc744
        Rgb::new(0x8d, 0xb4, 0xa5), // 12  bright blue    #8db4a5
        Rgb::new(0xe2, 0x8a, 0xa2), // 13  bright magenta #e28aa2
        Rgb::new(0x97, 0xd1, 0x83), // 14  bright cyan    #97d183
        Rgb::new(0xf5, 0xe3, 0xb4), // 15  bright white   #f5e3b4
    ],
};
