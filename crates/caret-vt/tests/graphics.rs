mod common;

use caret_core::{Color, Flags, Rgb};
use common::Vt;

fn flags(vt: &Vt) -> Flags {
    vt.term.screen().pen().flags
}

#[test]
fn attributes_are_set_and_cleared() {
    let mut vt = Vt::new(10, 1);
    vt.feed("\x1b[1;3;4;7;9m");
    let f = flags(&vt);
    assert!(f.contains(Flags::BOLD));
    assert!(f.contains(Flags::ITALIC));
    assert!(f.contains(Flags::UNDERLINE));
    assert!(f.contains(Flags::REVERSE));
    assert!(f.contains(Flags::STRIKE));

    vt.feed("\x1b[22;23;24;27;29m");
    assert!(flags(&vt).is_empty());
}

#[test]
fn reset_clears_everything() {
    let mut vt = Vt::new(10, 1);
    vt.feed("\x1b[1;31;44m\x1b[0m");
    assert!(flags(&vt).is_empty());
    assert_eq!(vt.term.screen().pen().fg, Color::DEFAULT);
    assert_eq!(vt.term.screen().pen().bg, Color::DEFAULT);
}

#[test]
fn an_empty_sgr_is_a_reset() {
    let mut vt = Vt::new(10, 1);
    vt.feed("\x1b[1;31m\x1b[m");
    assert!(flags(&vt).is_empty());
    assert_eq!(vt.term.screen().pen().fg, Color::DEFAULT);
}

#[test]
fn the_sixteen_ansi_colours() {
    let mut vt = Vt::new(10, 1);
    vt.feed("\x1b[32m");
    assert_eq!(vt.term.screen().pen().fg, Color::indexed(2));
    vt.feed("\x1b[45m");
    assert_eq!(vt.term.screen().pen().bg, Color::indexed(5));

    vt.feed("\x1b[92m");
    assert_eq!(
        vt.term.screen().pen().fg,
        Color::indexed(10),
        "bright foreground"
    );
    vt.feed("\x1b[104m");
    assert_eq!(
        vt.term.screen().pen().bg,
        Color::indexed(12),
        "bright background"
    );

    vt.feed("\x1b[39;49m");
    assert_eq!(vt.term.screen().pen().fg, Color::DEFAULT);
    assert_eq!(vt.term.screen().pen().bg, Color::DEFAULT);
}

#[test]
fn indexed_256_colour() {
    let mut vt = Vt::new(10, 1);
    vt.feed("\x1b[38;5;200m");
    assert_eq!(vt.term.screen().pen().fg, Color::indexed(200));
    vt.feed("\x1b[48;5;17m");
    assert_eq!(vt.term.screen().pen().bg, Color::indexed(17));
}

#[test]
fn truecolour_in_the_semicolon_form() {
    let mut vt = Vt::new(10, 1);
    vt.feed("\x1b[38;2;10;20;30m");
    assert_eq!(vt.term.screen().pen().fg, Color::rgb(10, 20, 30));
    vt.feed("\x1b[48;2;40;50;60m");
    assert_eq!(vt.term.screen().pen().bg, Color::rgb(40, 50, 60));
}

// The colon form packs everything into one parameter, and some emitters add a
// colour-space slot: 38:2::r:g:b
#[test]
fn truecolour_in_the_colon_form() {
    let mut vt = Vt::new(10, 1);
    vt.feed("\x1b[38:2:1:2:3m");
    assert_eq!(vt.term.screen().pen().fg, Color::rgb(1, 2, 3));

    vt.feed("\x1b[38:2::4:5:6m");
    assert_eq!(
        vt.term.screen().pen().fg,
        Color::rgb(4, 5, 6),
        "colour space slot skipped"
    );

    vt.feed("\x1b[38:5:99m");
    assert_eq!(vt.term.screen().pen().fg, Color::indexed(99));
}

#[test]
fn attributes_continue_past_an_extended_colour() {
    let mut vt = Vt::new(10, 1);
    vt.feed("\x1b[1;38;2;7;8;9;4m");
    assert!(flags(&vt).contains(Flags::BOLD));
    assert!(
        flags(&vt).contains(Flags::UNDERLINE),
        "the trailing 4 was not swallowed"
    );
    assert_eq!(vt.term.screen().pen().fg, Color::rgb(7, 8, 9));
}

#[test]
fn underline_styles_are_distinct() {
    let mut vt = Vt::new(10, 1);
    vt.feed("\x1b[4:3m");
    assert!(flags(&vt).contains(Flags::UNDERLINE_CURLY));
    assert!(!flags(&vt).contains(Flags::UNDERLINE));

    vt.feed("\x1b[21m");
    assert!(flags(&vt).contains(Flags::UNDERLINE_DOUBLE));
    assert!(
        !flags(&vt).contains(Flags::UNDERLINE_CURLY),
        "styles are exclusive"
    );

    vt.feed("\x1b[4:0m");
    assert!(!flags(&vt).any_underline());
}

#[test]
fn printed_cells_carry_the_attributes() {
    let mut vt = Vt::new(4, 1);
    vt.feed("\x1b[1;31;44mX");
    let cell = vt.cell(0, 0);
    assert_eq!(cell.ch, 'X');
    assert_eq!(cell.fg, Color::indexed(1));
    assert_eq!(cell.bg, Color::indexed(4));
    assert!(cell.flags.contains(Flags::BOLD));
}

#[test]
fn reverse_video_is_a_screen_mode() {
    use caret_core::Modes;
    let mut vt = Vt::new(4, 1);
    vt.feed("\x1b[?5h");
    assert!(vt.term.modes().contains(Modes::REVERSE_VIDEO));
    vt.feed("\x1b[?5l");
    assert!(!vt.term.modes().contains(Modes::REVERSE_VIDEO));
}

#[test]
fn unknown_parameters_are_ignored_not_fatal() {
    let mut vt = Vt::new(10, 1);
    vt.feed("\x1b[1;999;31mok");
    assert_eq!(vt.row(0), "ok");
    assert_eq!(vt.term.screen().pen().fg, Color::indexed(1));
}

#[test]
fn rgb_components_above_255_are_clamped() {
    let mut vt = Vt::new(10, 1);
    vt.feed("\x1b[38;2;300;400;500m");
    assert_eq!(
        vt.term.screen().pen().fg,
        Color::from_rgb(Rgb::new(255, 255, 255))
    );
}
