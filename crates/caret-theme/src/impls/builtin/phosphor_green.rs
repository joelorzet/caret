#[allow(unused_imports)]
use crate::prelude::*;

/// Green CRT phosphor on black. Blue, magenta and cyan are pulled toward
/// teal so the theme stays green-led, while red and yellow keep their
/// hue, because errors and warnings have to stay readable.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "phosphor-green",
    description:
        "Green CRT phosphor on black; red and yellow keep their hue so errors stay readable.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x05, 0x08, 0x06),
    fg: Rgb::new(0x33, 0xff, 0x66),
    cursor: Rgb::new(0x33, 0xff, 0x66),
    selection_bg: Rgb::new(0x11, 0x47, 0x2a),
    accent: Rgb::new(0x33, 0xff, 0x66),
    normal: [
        Rgb::new(0x0d, 0x1a, 0x12), //  0  black          #0d1a12
        Rgb::new(0xff, 0x6b, 0x5e), //  1  red            #ff6b5e
        Rgb::new(0x33, 0xff, 0x66), //  2  green          #33ff66
        Rgb::new(0xb6, 0xff, 0x4d), //  3  yellow         #b6ff4d
        Rgb::new(0x2f, 0xd9, 0x8f), //  4  blue           #2fd98f
        Rgb::new(0x7d, 0xff, 0xb0), //  5  magenta        #7dffb0
        Rgb::new(0x52, 0xff, 0xc7), //  6  cyan           #52ffc7
        Rgb::new(0xa8, 0xff, 0xc4), //  7  white          #a8ffc4
    ],
    bright: [
        Rgb::new(0x2a, 0x4a, 0x35), //  8  bright black   #2a4a35
        Rgb::new(0xff, 0x9b, 0x90), //  9  bright red     #ff9b90
        Rgb::new(0x7d, 0xff, 0x9e), // 10  bright green   #7dff9e
        Rgb::new(0xd6, 0xff, 0x8a), // 11  bright yellow  #d6ff8a
        Rgb::new(0x6f, 0xf0, 0xb8), // 12  bright blue    #6ff0b8
        Rgb::new(0xa9, 0xff, 0xd0), // 13  bright magenta #a9ffd0
        Rgb::new(0x8f, 0xff, 0xf0), // 14  bright cyan    #8ffff0
        Rgb::new(0xe3, 0xff, 0xec), // 15  bright white   #e3ffec
    ],
};
