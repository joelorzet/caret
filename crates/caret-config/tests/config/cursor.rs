use crate::support::parse;
use caret_config::CursorFill;
use caret_core::Rgb;

#[test]
fn the_cursor_fill_is_named() {
    for (name, expected) in [
        ("blink", CursorFill::Blink),
        ("solid", CursorFill::Solid),
        ("steady", CursorFill::Solid),
        ("translucent", CursorFill::Translucent),
    ] {
        let (config, warnings) = parse(&format!("[terminal]\ncursor_fill = \"{name}\"\n"));
        assert!(warnings.is_empty(), "{name}: {warnings:?}");
        assert_eq!(config.terminal.cursor_fill, expected, "{name}");
    }
}

#[test]
fn an_unknown_fill_is_reported_and_the_default_kept() {
    let (config, warnings) = parse("[terminal]\ncursor_fill = \"strobe\"\n");
    assert_eq!(config.terminal.cursor_fill, CursorFill::Blink);
    assert!(
        warnings.iter().any(|w| w.contains("cursor_fill")),
        "{warnings:?}"
    );
}

// The boolean shipped first and is still in people's config files.
#[test]
fn the_older_blink_boolean_still_works() {
    let (config, warnings) = parse("[terminal]\ncursor_blink = false\n");
    assert!(warnings.is_empty(), "{warnings:?}");
    assert_eq!(config.terminal.cursor_fill, CursorFill::Solid);
}

#[test]
fn the_named_fill_wins_over_the_boolean() {
    let (config, _) = parse("[terminal]\ncursor_blink = true\ncursor_fill = \"translucent\"\n");
    assert_eq!(config.terminal.cursor_fill, CursorFill::Translucent);
}

#[test]
fn the_cursor_colour_overrides_the_theme() {
    let (config, warnings) = parse("[terminal]\ncursor_color = \"#ff8800\"\n");
    assert!(warnings.is_empty(), "{warnings:?}");
    assert_eq!(
        config.terminal.cursor_color,
        Some(Rgb::new(0xff, 0x88, 0x00))
    );
}

#[test]
fn a_bad_cursor_colour_is_reported_and_the_theme_kept() {
    let (config, warnings) = parse("[terminal]\ncursor_color = \"octarine\"\n");
    assert_eq!(config.terminal.cursor_color, None);
    assert!(
        warnings.iter().any(|w| w.contains("cursor_color")),
        "{warnings:?}"
    );
}

#[test]
fn only_blinking_blinks_and_only_translucent_is_see_through() {
    assert!(CursorFill::Blink.blinks());
    assert!(!CursorFill::Solid.blinks());
    assert!(!CursorFill::Translucent.blinks());

    assert_eq!(CursorFill::Solid.alpha(), 255);
    assert_eq!(CursorFill::Blink.alpha(), 255);
    assert!(CursorFill::Translucent.alpha() < 255);
}
