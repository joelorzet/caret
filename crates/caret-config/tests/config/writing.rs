use crate::support::parse;
use caret_config::{Action, Config, KeyBinding, KeyResolution};

#[test]
fn the_theme_line_is_replaced_in_place() {
    let before = "# my config\ntheme = \"nord\"\n\n[font]\nsize = 15.0\n";
    let after = Config::with_theme(before, "dracula");
    assert_eq!(
        after,
        "# my config\ntheme = \"dracula\"\n\n[font]\nsize = 15.0\n"
    );
}

// The file is the user's; re-serialising it would discard their comments.
#[test]
fn saving_the_theme_keeps_comments_and_order() {
    let before = "# keep me\n[font]\n# and me\nsize = 16.0\n";
    let after = Config::with_theme(before, "nord");
    assert!(after.contains("# keep me"));
    assert!(after.contains("# and me"));
    assert!(after.contains("size = 16.0"));
    assert!(after.starts_with("theme = \"nord\""), "added at the top");
}

#[test]
fn an_empty_config_gains_the_setting() {
    assert_eq!(Config::with_theme("", "nord"), "theme = \"nord\"\n");
}

// `theme` inside a table is a different setting entirely.
#[test]
fn only_the_top_level_theme_is_touched() {
    let before = "[sidebar.agent]\ntheme = \"something else\"\n";
    let after = Config::with_theme(before, "nord");
    assert!(after.contains("theme = \"something else\""), "{after}");
    assert!(after.starts_with("theme = \"nord\""));
}

#[test]
fn the_rewritten_config_still_parses() {
    let (config, warnings) = parse(&Config::with_theme(caret_config::EXAMPLE, "gruvbox-dark"));
    assert!(warnings.is_empty(), "{warnings:?}");
    assert_eq!(config.theme, "gruvbox-dark");
}

#[test]
fn saving_twice_does_not_stack_up_lines() {
    let once = Config::with_theme("", "nord");
    let twice = Config::with_theme(&once, "dracula");
    assert_eq!(twice.matches("theme =").count(), 1);
    assert!(twice.contains("dracula"));
}

#[test]
fn saving_writes_the_file() {
    let dir = std::env::temp_dir().join(format!("caret-save-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let path = dir.join("config.toml");

    Config::save_theme_to(&path, "nord").expect("write");
    assert!(std::fs::read_to_string(&path).unwrap().contains("nord"));

    Config::save_theme_to(&path, "dracula").expect("overwrite");
    let text = std::fs::read_to_string(&path).unwrap();
    assert!(text.contains("dracula"));
    assert_eq!(text.matches("theme =").count(), 1);
}

// The config file is the user's. Saving a binding must leave their comments
// and their ordering exactly where they were.
#[test]
fn saving_a_binding_keeps_the_rest_of_the_file() {
    let before =
        "# mine\ntheme = \"nord\"\n\n[keys.direct]\n# a note\n\"cmd+1\" = \"select_tab:1\"\n";
    let after = Config::with_binding(before, "cmd+9", "quit");
    assert!(after.contains("# mine"));
    assert!(after.contains("# a note"));
    assert!(after.contains("\"cmd+1\" = \"select_tab:1\""));
    assert!(after.contains("\"cmd+9\" = \"quit\""));
}

#[test]
fn a_binding_lands_in_the_right_table() {
    let before = "[keys.direct]\n\"cmd+1\" = \"quit\"\n\n[keys.prefixed]\n\"x\" = \"close_pane\"\n";
    let after = Config::with_binding(before, "cmd+2", "new_tab");
    let direct = after.find("[keys.direct]").expect("the direct table");
    let prefixed = after.find("[keys.prefixed]").expect("the prefixed table");
    let added = after.find("\"cmd+2\"").expect("the new binding");
    assert!(added > direct && added < prefixed, "{after}");
}

#[test]
fn what_is_written_reads_back_as_the_binding_it_was() {
    let after = Config::with_binding("", "cmd+9", "quit");
    let (config, problems) = parse(&after);
    assert!(problems.is_empty(), "{problems:?}");
    let binding = KeyBinding::parse("cmd+9").expect("parses");
    assert_eq!(
        config.keymap.resolve(binding, false),
        KeyResolution::Run(Action::Quit)
    );
}

// A config file is usually a stub: the theme picker writes one line. Opening
// it to change the window size and finding only `theme` there is no help, so
// the setting being looked at is written in first.
#[test]
fn a_setting_that_is_not_in_the_file_is_added_with_its_table() {
    let (text, line) = Config::with_setting("theme = \"nord\"\n", "window", "cols", "100");
    assert!(text.contains("[window]"));
    assert!(text.contains("cols = 100"));
    assert_eq!(text.lines().nth(line - 1), Some("cols = 100"));
    let (config, problems) = parse(&text);
    assert!(problems.is_empty(), "{problems:?}");
    assert_eq!(config.window.cols, 100);
}

#[test]
fn a_setting_already_there_is_pointed_at_rather_than_written_again() {
    let before = "[window]\ncols = 120\nrows = 40\n";
    let (text, line) = Config::with_setting(before, "window", "rows", "30");
    assert_eq!(text, before, "the file is untouched");
    assert_eq!(text.lines().nth(line - 1), Some("rows = 40"));
}

#[test]
fn a_setting_joins_the_table_it_belongs_to() {
    let before = "[window]\ncols = 120\n\n[terminal]\nscrollback = 500\n";
    let (text, line) = Config::with_setting(before, "window", "rows", "30");
    let window = text.find("[window]").expect("the window table");
    let terminal = text.find("[terminal]").expect("the terminal table");
    let added = text.find("rows = 30").expect("the new line");
    assert!(added > window && added < terminal, "{text}");
    assert_eq!(text.lines().nth(line - 1), Some("rows = 30"));
}

// `theme` has no table of its own, so it has to land at the top rather than
// inside whichever table happens to come first.
#[test]
fn a_top_level_setting_stays_at_the_top_level() {
    let before = "[window]\ncols = 120\n";
    let (text, line) = Config::with_setting(before, "", "theme", "\"nord\"");
    assert_eq!(text.lines().nth(line - 1), Some("theme = \"nord\""));
    let (config, problems) = parse(&text);
    assert!(problems.is_empty(), "{problems:?}");
    assert_eq!(config.theme, "nord");
    assert_eq!(config.window.cols, 120);
}

#[test]
fn the_line_reported_is_the_line_the_setting_is_on() {
    for (table, key, value) in [
        ("", "theme", "\"nord\""),
        ("font", "size", "13.0"),
        ("terminal", "scrollback", "10000"),
    ] {
        let (text, line) = Config::with_setting("", table, key, value);
        let found = text.lines().nth(line - 1).unwrap_or_default();
        assert!(found.starts_with(key), "{table}.{key} gave {found:?}");
    }
}

#[test]
fn every_setting_can_be_written_back_and_read_again() {
    let config = Config::default();
    let mut text = String::new();
    for setting in config.settings() {
        let value = match setting.value.as_str() {
            v if v == "true" || v == "false" || v.starts_with('[') => v.to_string(),
            v if v.parse::<f64>().is_ok() && setting.key != "family" => v.to_string(),
            v => format!("{v:?}"),
        };
        text = Config::with_setting(&text, setting.table, setting.key, &value).0;
    }
    let (_, problems) = parse(&text);
    let real: Vec<&String> = problems
        .iter()
        .filter(|p| !p.contains("$SHELL") && !p.contains("from theme"))
        .collect();
    assert!(real.is_empty(), "{real:?}\n{text}");
}

#[test]
fn rebinding_a_key_replaces_it_rather_than_adding_a_second() {
    let before = "[keys.direct]\n\"cmd+1\" = \"select_tab:1\"\n";
    let after = Config::with_binding(before, "cmd+1", "quit");
    assert_eq!(after.matches("\"cmd+1\"").count(), 1);
    assert!(after.contains("\"cmd+1\" = \"quit\""));
    assert!(!after.contains("select_tab:1"));
}

#[test]
fn a_file_without_the_table_gains_one() {
    let after = Config::with_binding("theme = \"nord\"\n", "cmd+9", "quit");
    assert!(after.contains("[keys.direct]"));
    assert!(after.contains("\"cmd+9\" = \"quit\""));
    let (config, problems) = parse(&after);
    assert!(problems.is_empty(), "{problems:?}");
    assert_eq!(config.theme, "nord");
}
