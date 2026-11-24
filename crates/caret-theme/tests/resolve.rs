use caret_core::{xterm_256, Color, Rgb};
use caret_theme::{builtin, ColorSlot};

#[test]
fn default_resolves_per_slot() {
    let t = builtin::default_theme();
    assert_eq!(t.resolve(Color::DEFAULT, ColorSlot::Foreground), t.fg);
    assert_eq!(t.resolve(Color::DEFAULT, ColorSlot::Background), t.bg);
}

#[test]
fn the_first_sixteen_slots_come_from_the_theme() {
    let t = builtin::by_name("dracula").unwrap();
    assert_eq!(t.indexed(0), t.palette.normal[0]);
    assert_eq!(t.indexed(7), t.palette.normal[7]);
    assert_eq!(t.indexed(8), t.palette.bright[0]);
    assert_eq!(t.indexed(15), t.palette.bright[7]);
    assert_eq!(t.indexed(1), Rgb::new(0xff, 0x55, 0x55));
}

#[test]
fn higher_slots_come_from_the_xterm_cube() {
    let t = builtin::default_theme();
    assert_eq!(t.indexed(16), xterm_256(16).unwrap());
    assert_eq!(t.indexed(255), xterm_256(255).unwrap());
}

#[test]
fn direct_colors_pass_through() {
    let t = builtin::default_theme();
    let c = Rgb::new(1, 2, 3);
    assert_eq!(t.resolve(Color::from_rgb(c), ColorSlot::Foreground), c);
}
