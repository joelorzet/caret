use crate::support::{assigns_key, parse};
use caret_config::Config;

// The settings list is how you find out what there is to change, so a key
// missing from it is a key nobody knows about.
#[test]
fn every_section_of_the_config_turns_up_in_the_settings_list() {
    let settings = Config::default().settings();
    let sections: Vec<&str> = settings.iter().map(|s| s.section).collect();
    for section in [
        "theme", "font", "window", "terminal", "chrome", "explorer", "agent",
    ] {
        assert!(sections.contains(&section), "{section} is missing");
    }
    for key in [
        "ligatures",
        "scrollback",
        "highlight_input",
        "cursor_fill",
        "opacity",
    ] {
        assert!(
            settings.iter().any(|s| s.key == key),
            "{key} is missing from the settings list"
        );
    }
}

#[test]
fn every_setting_says_what_it_is_and_what_it_does() {
    for setting in Config::default().settings() {
        let at = format!("{}.{}", setting.section, setting.key);
        assert!(!setting.value.is_empty(), "{at} has no value");
        assert!(!setting.about.is_empty(), "{at} has no description");
    }
}

#[test]
fn the_settings_list_shows_what_the_config_actually_holds() {
    let (config, _) = parse(
        r#"
[font]
ligatures = false

[terminal]
scrollback = 4242
"#,
    );
    let settings = config.settings();
    let value = |key: &str| {
        settings
            .iter()
            .find(|s| s.key == key)
            .map(|s| s.value.clone())
            .unwrap_or_default()
    };
    assert_eq!(value("ligatures"), "false");
    assert_eq!(value("scrollback"), "4242");
}

// The file is where the options are discovered. A setting the settings list
// knows about but the file never mentions is one nobody will find.
#[test]
fn every_setting_appears_in_the_example_file() {
    let missing: Vec<String> = Config::default()
        .settings()
        .iter()
        .filter(|setting| {
            !caret_config::EXAMPLE
                .lines()
                .any(|line| assigns_key(line, setting.key))
        })
        .map(|setting| format!("{}.{}", setting.section, setting.key))
        .collect();
    assert!(missing.is_empty(), "not in the example file: {missing:?}");
}

#[test]
fn every_table_the_settings_use_appears_in_the_example_file() {
    for setting in Config::default().settings() {
        if setting.table.is_empty() {
            continue;
        }
        let header = format!("[{}]", setting.table);
        assert!(
            caret_config::EXAMPLE.contains(&header),
            "{header} is not in the example file"
        );
    }
}

// A file the theme picker wrote is one line long. Opening it to change
// something should give you every option to read, not a screen of tildes.
#[test]
fn a_stub_config_is_filled_out_with_the_example() {
    let filled = Config::filled_out("theme = \"nord\"\n");
    assert!(filled.contains("[font]"));
    assert!(filled.contains("[terminal]"));
    assert!(filled.contains("[window]"));
    assert!(
        filled.contains("theme = \"nord\""),
        "the theme that was set is kept"
    );
    assert_eq!(
        filled.matches("theme = ").count(),
        1,
        "and not written twice"
    );

    let (config, problems) = parse(&filled);
    assert!(problems.is_empty(), "{problems:?}");
    assert_eq!(config.theme, "nord");
}

// A file with real sections in it is the user's own work.
#[test]
fn a_config_with_sections_is_left_exactly_as_it_is() {
    let mine = "# mine\ntheme = \"nord\"\n\n[font]\nsize = 15.0\n";
    assert_eq!(Config::filled_out(mine), mine);
}

#[test]
fn filling_out_an_empty_file_gives_the_whole_example() {
    assert_eq!(Config::filled_out(""), caret_config::EXAMPLE);
}
