#[allow(unused_imports)]
use crate::prelude::*;

/// GitHub light on paper, with the text and syntax driven darker.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "github-light-contrast",
    description: "GitHub light on paper, with the text and syntax driven darker.",
    appearance: Appearance::Light,
    bg: Rgb::new(0xff, 0xff, 0xff),
    fg: Rgb::new(0x00, 0x00, 0x00),
    cursor: Rgb::new(0x00, 0x44, 0x94),
    selection_bg: Rgb::new(0xd0, 0xe2, 0xf8),
    accent: Rgb::new(0x00, 0x44, 0x94),
    normal: [
        Rgb::new(0x00, 0x00, 0x00), //  0  black          #000000
        Rgb::new(0x9d, 0x09, 0x13), //  1  red            #9d0913
        Rgb::new(0x00, 0x05, 0x02), //  2  green          #000502
        Rgb::new(0x00, 0x00, 0x00), //  3  yellow         #000000
        Rgb::new(0x00, 0x44, 0x94), //  4  blue           #004494
        Rgb::new(0x5b, 0x11, 0xe4), //  5  magenta        #5b11e4
        Rgb::new(0x05, 0x33, 0x36), //  6  cyan           #053336
        Rgb::new(0x49, 0x51, 0x5a), //  7  white          #49515a
    ],
    bright: [
        Rgb::new(0x2e, 0x34, 0x3b), //  8  bright black   #2e343b
        Rgb::new(0x55, 0x00, 0x0e), //  9  bright red     #55000e
        Rgb::new(0x05, 0x30, 0x11), // 10  bright green   #053011
        Rgb::new(0x00, 0x00, 0x00), // 11  bright yellow  #000000
        Rgb::new(0x00, 0x6c, 0xe2), // 12  bright blue    #006ce2
        Rgb::new(0x88, 0x47, 0xff), // 13  bright magenta #8847ff
        Rgb::new(0x15, 0x61, 0x74), // 14  bright cyan    #156174
        Rgb::new(0x6a, 0x77, 0x85), // 15  bright white   #6a7785
    ],
};
