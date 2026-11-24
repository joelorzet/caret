use crate::support::parse;
use caret_config::{BellStyle, Config};
use caret_core::CursorShape;
use caret_layout::Dock;

// Guards the file `caret config init` writes against drift in the schema.
#[test]
fn the_shipped_example_is_valid() {
    let (_, warnings) = parse(caret_config::EXAMPLE);
    assert!(warnings.is_empty(), "example config warns: {warnings:#?}");
}

#[test]
fn the_example_reproduces_the_defaults() {
    let (config, _) = parse(caret_config::EXAMPLE);
    let defaults = Config::default();
    assert_eq!(config.theme, defaults.theme);
    assert_eq!(config.font.size, defaults.font.size);
    assert_eq!(config.terminal.scrollback, defaults.terminal.scrollback);
    assert_eq!(
        config.sidebars.agent.command,
        defaults.sidebars.agent.command
    );
    assert_eq!(config.window.padding, defaults.window.padding);
}

#[test]
fn an_empty_config_is_the_defaults() {
    let (config, warnings) = parse("");
    assert!(warnings.is_empty());
    assert_eq!(config, Config::default());
}

#[test]
fn a_partial_config_only_overrides_what_it_mentions() {
    let (config, warnings) = parse("theme = \"dracula\"\n[font]\nsize = 16.0\n");
    assert!(warnings.is_empty(), "{warnings:?}");
    assert_eq!(config.theme, "dracula");
    assert_eq!(config.font.size, 16.0);
    assert_eq!(config.font.family, Config::default().font.family);
}

#[test]
fn unknown_keys_are_reported_but_not_fatal() {
    let (config, warnings) = parse("theme = \"nord\"\nnope = 1\n[font]\nfamilly = \"x\"\n");
    assert_eq!(config.theme, "nord", "valid settings still apply");
    assert!(
        warnings.iter().any(|w| w.contains("\"nope\"")),
        "{warnings:?}"
    );
    assert!(
        warnings.iter().any(|w| w.contains("\"font.familly\"")),
        "{warnings:?}"
    );
}

#[test]
fn freeform_key_tables_are_not_flagged() {
    let (_, warnings) =
        parse("[keys.direct]\n\"cmd+y\" = \"copy\"\n[keys.prefixed]\n\"y\" = \"copy\"\n");
    assert!(warnings.is_empty(), "{warnings:?}");
}

#[test]
fn bad_values_warn_and_keep_the_default() {
    let (config, warnings) = parse("[terminal]\ncursor = \"sausage\"\nbell = \"whistle\"\n");
    assert_eq!(config.terminal.cursor, CursorShape::Block);
    assert_eq!(config.terminal.bell, BellStyle::Visual);
    assert_eq!(warnings.len(), 2, "{warnings:?}");
    // The message should say what is allowed.
    assert!(warnings[0].contains("block"), "{}", warnings[0]);
}

#[test]
fn enum_values_accept_their_aliases() {
    let (config, warnings) = parse("[terminal]\ncursor = \"beam\"\nbell = \"off\"\n");
    assert!(warnings.is_empty(), "{warnings:?}");
    assert_eq!(config.terminal.cursor, CursorShape::Bar);
    assert_eq!(config.terminal.bell, BellStyle::None);
}

#[test]
fn out_of_range_numbers_are_clamped() {
    let (config, _) =
        parse("[window]\nopacity = 5.0\ncols = 1\nrows = 1\n[sidebar.explorer]\nwidth = 9999\n");
    assert_eq!(config.window.opacity, 1.0);
    assert!(config.window.cols >= 20);
    assert!(config.window.rows >= 5);
    assert_eq!(config.sidebars.explorer.width, Dock::MAX_CELLS);
}

#[test]
fn a_negative_font_size_warns() {
    let (config, warnings) = parse("[font]\nsize = -3.0\n");
    assert_eq!(config.font.size, Config::default().font.size);
    assert!(
        warnings.iter().any(|w| w.contains("font.size")),
        "{warnings:?}"
    );
}

#[test]
fn malformed_toml_still_yields_a_usable_config() {
    let (config, warnings) = parse("this is not = = toml");
    assert_eq!(config, Config::default());
    assert!(
        warnings.iter().any(|w| w.contains("could not parse")),
        "{warnings:?}"
    );
}
