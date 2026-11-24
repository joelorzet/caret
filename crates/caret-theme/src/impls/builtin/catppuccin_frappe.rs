#[allow(unused_imports)]
use crate::prelude::*;

/// Catppuccin, Frappe flavour, warmer than Mocha.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "catppuccin-frappe",
    description: "Catppuccin, Frappe flavour, warmer than Mocha.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x30, 0x34, 0x46),
    fg: Rgb::new(0xc6, 0xd0, 0xf5),
    cursor: Rgb::new(0xf2, 0xd5, 0xcf),
    selection_bg: Rgb::new(0x62, 0x68, 0x80),
    accent: Rgb::new(0x8c, 0xaa, 0xee),
    normal: [
        Rgb::new(0x51, 0x57, 0x6d), //  0  black          #51576d
        Rgb::new(0xe7, 0x82, 0x84), //  1  red            #e78284
        Rgb::new(0xa6, 0xd1, 0x89), //  2  green          #a6d189
        Rgb::new(0xe5, 0xc8, 0x90), //  3  yellow         #e5c890
        Rgb::new(0x8c, 0xaa, 0xee), //  4  blue           #8caaee
        Rgb::new(0xf4, 0xb8, 0xe4), //  5  magenta        #f4b8e4
        Rgb::new(0x81, 0xc8, 0xbe), //  6  cyan           #81c8be
        Rgb::new(0xb5, 0xbf, 0xe2), //  7  white          #b5bfe2
    ],
    bright: [
        Rgb::new(0x62, 0x68, 0x80), //  8  bright black   #626880
        Rgb::new(0xe7, 0x82, 0x84), //  9  bright red     #e78284
        Rgb::new(0xa6, 0xd1, 0x89), // 10  bright green   #a6d189
        Rgb::new(0xe5, 0xc8, 0x90), // 11  bright yellow  #e5c890
        Rgb::new(0x8c, 0xaa, 0xee), // 12  bright blue    #8caaee
        Rgb::new(0xf4, 0xb8, 0xe4), // 13  bright magenta #f4b8e4
        Rgb::new(0x81, 0xc8, 0xbe), // 14  bright cyan    #81c8be
        Rgb::new(0xa5, 0xad, 0xce), // 15  bright white   #a5adce
    ],
};
