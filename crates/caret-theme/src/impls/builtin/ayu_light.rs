#[allow(unused_imports)]
use crate::prelude::*;

/// Ayu light.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "ayu-light",
    description: "Ayu light.",
    appearance: Appearance::Light,
    bg: Rgb::new(0xfc, 0xfc, 0xfc),
    fg: Rgb::new(0x5c, 0x61, 0x66),
    cursor: Rgb::new(0xff, 0x99, 0x40),
    selection_bg: Rgb::new(0xd1, 0xe4, 0xf4),
    accent: Rgb::new(0x41, 0x96, 0xdf),
    normal: [
        Rgb::new(0x01, 0x01, 0x01), //  0  black          #010101
        Rgb::new(0xe7, 0x66, 0x6a), //  1  red            #e7666a
        Rgb::new(0x80, 0xab, 0x24), //  2  green          #80ab24
        Rgb::new(0xeb, 0xa5, 0x4d), //  3  yellow         #eba54d
        Rgb::new(0x41, 0x96, 0xdf), //  4  blue           #4196df
        Rgb::new(0x98, 0x70, 0xc3), //  5  magenta        #9870c3
        Rgb::new(0x51, 0xb8, 0x91), //  6  cyan           #51b891
        Rgb::new(0xc1, 0xc1, 0xc1), //  7  white          #c1c1c1
    ],
    bright: [
        Rgb::new(0x34, 0x34, 0x34), //  8  bright black   #343434
        Rgb::new(0xee, 0x92, 0x95), //  9  bright red     #ee9295
        Rgb::new(0x9f, 0xd3, 0x2f), // 10  bright green   #9fd32f
        Rgb::new(0xf0, 0xbc, 0x7b), // 11  bright yellow  #f0bc7b
        Rgb::new(0x6d, 0xae, 0xe6), // 12  bright blue    #6daee6
        Rgb::new(0xb2, 0x94, 0xd2), // 13  bright magenta #b294d2
        Rgb::new(0x75, 0xc7, 0xa8), // 14  bright cyan    #75c7a8
        Rgb::new(0xdb, 0xdb, 0xdb), // 15  bright white   #dbdbdb
    ],
};
