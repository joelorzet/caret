#[allow(unused_imports)]
use crate::prelude::*;

/// Gruvbox dark, medium contrast.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "gruvbox-dark",
    description: "Gruvbox dark, medium contrast.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x28, 0x28, 0x28),
    fg: Rgb::new(0xeb, 0xdb, 0xb2),
    cursor: Rgb::new(0xeb, 0xdb, 0xb2),
    selection_bg: Rgb::new(0x50, 0x49, 0x45),
    accent: Rgb::new(0xfa, 0xbd, 0x2f),
    normal: [
        Rgb::new(0x28, 0x28, 0x28), //  0  black          #282828
        Rgb::new(0xcc, 0x24, 0x1d), //  1  red            #cc241d
        Rgb::new(0x98, 0x97, 0x1a), //  2  green          #98971a
        Rgb::new(0xd7, 0x99, 0x21), //  3  yellow         #d79921
        Rgb::new(0x45, 0x85, 0x88), //  4  blue           #458588
        Rgb::new(0xb1, 0x62, 0x86), //  5  magenta        #b16286
        Rgb::new(0x68, 0x9d, 0x6a), //  6  cyan           #689d6a
        Rgb::new(0xa8, 0x99, 0x84), //  7  white          #a89984
    ],
    bright: [
        Rgb::new(0x92, 0x83, 0x74), //  8  bright black   #928374
        Rgb::new(0xfb, 0x49, 0x34), //  9  bright red     #fb4934
        Rgb::new(0xb8, 0xbb, 0x26), // 10  bright green   #b8bb26
        Rgb::new(0xfa, 0xbd, 0x2f), // 11  bright yellow  #fabd2f
        Rgb::new(0x83, 0xa5, 0x98), // 12  bright blue    #83a598
        Rgb::new(0xd3, 0x86, 0x9b), // 13  bright magenta #d3869b
        Rgb::new(0x8e, 0xc0, 0x7c), // 14  bright cyan    #8ec07c
        Rgb::new(0xeb, 0xdb, 0xb2), // 15  bright white   #ebdbb2
    ],
};
