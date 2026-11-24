use caret_core::{xterm_256, Color, ColorKind, Rgb};

#[test]
fn stays_four_bytes() {
    assert_eq!(std::mem::size_of::<Color>(), 4);
}

#[test]
fn roundtrips_each_variant() {
    assert_eq!(Color::DEFAULT.kind(), ColorKind::Default);
    assert_eq!(Color::indexed(200).kind(), ColorKind::Indexed(200));
    assert_eq!(
        Color::rgb(0x12, 0x34, 0x56).kind(),
        ColorKind::Rgb(Rgb::new(0x12, 0x34, 0x56))
    );
}

#[test]
fn black_is_not_default() {
    assert!(Color::DEFAULT.is_default());
    assert!(!Color::rgb(0, 0, 0).is_default());
    assert!(!Color::indexed(0).is_default());
}

#[test]
fn expands_cube_and_grey_ramp() {
    assert_eq!(xterm_256(0), None);
    assert_eq!(xterm_256(15), None);
    assert_eq!(xterm_256(16), Some(Rgb::new(0, 0, 0)));
    assert_eq!(xterm_256(231), Some(Rgb::new(255, 255, 255)));
    assert_eq!(xterm_256(232), Some(Rgb::new(8, 8, 8)));
    assert_eq!(xterm_256(255), Some(Rgb::new(238, 238, 238)));
}

#[test]
fn blend_endpoints_are_exact() {
    let (a, b) = (Rgb::new(200, 100, 0), Rgb::new(0, 50, 100));
    assert_eq!(a.blend(b, 1.0), a);
    assert_eq!(a.blend(b, 0.0), b);
    assert_eq!(a.blend(b, 2.0), a, "t is clamped");
}

#[test]
fn rgb_packs_to_u32() {
    let c = Rgb::new(0xab, 0xcd, 0xef);
    assert_eq!(c.to_u32(), 0x00ab_cdef);
    assert_eq!(Rgb::from_u32(0x00ab_cdef), c);
}

// Contrast ratio only measures lightness, so two colours that differ in hue
// but share a luminance come out identical by that measure. Dracula's green
// and cyan are 1.01:1 apart and nobody would confuse them.
#[test]
fn perceptual_distance_sees_hue_that_contrast_misses() {
    let green = Rgb::new(0x50, 0xfa, 0x7b);
    let cyan = Rgb::new(0x8b, 0xe9, 0xfd);

    let contrast = green.contrast_ratio(cyan);
    assert!(
        contrast < 1.1,
        "contrast calls them the same: {contrast:.2}"
    );

    let distance = green.perceptual_distance(cyan);
    assert!(distance > 100.0, "but they look different: {distance:.0}");
}

#[test]
fn a_colour_is_no_distance_from_itself() {
    let colour = Rgb::new(12, 34, 56);
    assert_eq!(colour.perceptual_distance(colour), 0.0);
}

#[test]
fn near_identical_colours_stay_close() {
    let a = Rgb::new(0x33, 0xff, 0x66);
    let b = Rgb::new(0x34, 0xff, 0x67);
    assert!(a.perceptual_distance(b) < 10.0);
}

#[test]
fn distance_is_symmetric() {
    let (a, b) = (Rgb::new(10, 200, 30), Rgb::new(220, 40, 180));
    let difference = (a.perceptual_distance(b) - b.perceptual_distance(a)).abs();
    assert!(difference < 0.001);
}

#[test]
fn black_and_white_are_as_far_apart_as_it_gets() {
    let span = Rgb::BLACK.perceptual_distance(Rgb::WHITE);
    assert!(span > 700.0, "got {span}");
}
