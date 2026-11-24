#[allow(unused_imports)]
use crate::prelude::*;

/// Ayu Mirage, the midtone one.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "ayu-mirage",
    description: "Ayu Mirage, the midtone one.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x1f, 0x24, 0x30),
    fg: Rgb::new(0xcb, 0xcc, 0xc6),
    cursor: Rgb::new(0xff, 0xcc, 0x66),
    selection_bg: Rgb::new(0x34, 0x45, 0x5a),
    accent: Rgb::new(0x73, 0xd0, 0xff),
    normal: [
        Rgb::new(0x21, 0x27, 0x33), //  0  black          #212733
        Rgb::new(0xf0, 0x87, 0x78), //  1  red            #f08778
        Rgb::new(0x53, 0xbf, 0x97), //  2  green          #53bf97
        Rgb::new(0xfd, 0xcc, 0x60), //  3  yellow         #fdcc60
        Rgb::new(0x60, 0xb8, 0xd6), //  4  blue           #60b8d6
        Rgb::new(0xec, 0x71, 0x71), //  5  magenta        #ec7171
        Rgb::new(0x98, 0xe6, 0xca), //  6  cyan           #98e6ca
        Rgb::new(0xfa, 0xfa, 0xfa), //  7  white          #fafafa
    ],
    bright: [
        Rgb::new(0x68, 0x68, 0x68), //  8  bright black   #686868
        Rgb::new(0xf5, 0x8c, 0x7d), //  9  bright red     #f58c7d
        Rgb::new(0x58, 0xc4, 0x9c), // 10  bright green   #58c49c
        Rgb::new(0xff, 0xd1, 0x65), // 11  bright yellow  #ffd165
        Rgb::new(0x65, 0xbd, 0xdb), // 12  bright blue    #65bddb
        Rgb::new(0xf1, 0x76, 0x76), // 13  bright magenta #f17676
        Rgb::new(0x9d, 0xeb, 0xcf), // 14  bright cyan    #9debcf
        Rgb::new(0xff, 0xff, 0xff), // 15  bright white   #ffffff
    ],
};
