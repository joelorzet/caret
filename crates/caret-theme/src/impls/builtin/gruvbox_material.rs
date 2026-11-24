#[allow(unused_imports)]
use crate::prelude::*;

/// Gruvbox Material, softer contrast than the original.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "gruvbox-material",
    description: "Gruvbox Material, softer contrast than the original.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x28, 0x28, 0x28),
    fg: Rgb::new(0xdf, 0xbf, 0x8e),
    cursor: Rgb::new(0xdf, 0xbf, 0x8e),
    selection_bg: Rgb::new(0x45, 0x40, 0x3d),
    accent: Rgb::new(0xe7, 0x8a, 0x4e),
    normal: [
        Rgb::new(0x66, 0x5c, 0x54), //  0  black          #665c54
        Rgb::new(0xea, 0x69, 0x62), //  1  red            #ea6962
        Rgb::new(0xa9, 0xb6, 0x65), //  2  green          #a9b665
        Rgb::new(0xe7, 0x8a, 0x4e), //  3  yellow         #e78a4e
        Rgb::new(0x7d, 0xae, 0xa3), //  4  blue           #7daea3
        Rgb::new(0xd3, 0x86, 0x9b), //  5  magenta        #d3869b
        Rgb::new(0x89, 0xb4, 0x82), //  6  cyan           #89b482
        Rgb::new(0xdf, 0xbf, 0x8e), //  7  white          #dfbf8e
    ],
    bright: [
        Rgb::new(0x92, 0x83, 0x74), //  8  bright black   #928374
        Rgb::new(0xea, 0x69, 0x62), //  9  bright red     #ea6962
        Rgb::new(0xa9, 0xb6, 0x65), // 10  bright green   #a9b665
        Rgb::new(0xe3, 0xa8, 0x4e), // 11  bright yellow  #e3a84e
        Rgb::new(0x7d, 0xae, 0xa3), // 12  bright blue    #7daea3
        Rgb::new(0xd3, 0x86, 0x9b), // 13  bright magenta #d3869b
        Rgb::new(0x89, 0xb4, 0x82), // 14  bright cyan    #89b482
        Rgb::new(0xdf, 0xbf, 0x8e), // 15  bright white   #dfbf8e
    ],
};
