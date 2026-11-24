use crate::support::parse;
use caret_config::{Action, KeyBinding, KeyResolution, Modifiers, Platform};
use caret_layout::Axis;

#[test]
fn keys_can_be_added_rebound_and_removed() {
    let (config, warnings) = parse(
        r#"
[keys]
prefix = "ctrl+a"
unbind = ["cmd+q"]

[keys.direct]
"cmd+y" = "copy"
"cmd+d" = "new_tab"
"cmd+v" = "none"

[keys.prefixed]
"|" = "split_horizontal"
"#,
    );
    assert!(warnings.is_empty(), "{warnings:?}");
    let map = &config.keymap;
    let press = |s: &str| KeyBinding::parse(s).unwrap();

    assert_eq!(map.prefix(), Some(press("ctrl+a")));
    assert_eq!(
        map.resolve(press("cmd+y"), false),
        KeyResolution::Run(Action::Copy)
    );
    assert_eq!(
        map.resolve(press("cmd+d"), false),
        KeyResolution::Run(Action::NewTab)
    );
    assert_eq!(
        map.resolve(press("cmd+v"), false),
        KeyResolution::Unbound,
        "none unbinds"
    );
    assert_eq!(
        map.resolve(press("cmd+q"), false),
        KeyResolution::Unbound,
        "unbind list"
    );
    assert_eq!(
        map.resolve(press("|"), true),
        KeyResolution::Run(Action::SplitPane(Axis::Horizontal))
    );
    assert_eq!(
        map.resolve(press("%"), true),
        KeyResolution::Run(Action::SplitPane(Axis::Horizontal)),
        "defaults are kept alongside additions"
    );
}

#[test]
fn actions_can_carry_an_amount() {
    let (config, warnings) =
        parse("[keys.direct]\n\"cmd+1\" = \"select_tab:7\"\n\"cmd+2\" = \"resize_left:12\"\n");
    assert!(warnings.is_empty(), "{warnings:?}");
    let press = |s: &str| KeyBinding::parse(s).unwrap();
    assert_eq!(
        config.keymap.resolve(press("cmd+1"), false),
        KeyResolution::Run(Action::SelectTab(7))
    );
    assert_eq!(
        config.keymap.resolve(press("cmd+2"), false),
        KeyResolution::Run(Action::ResizePane(caret_layout::Direction::Left, 12))
    );
}

#[test]
fn defaults_false_starts_from_an_empty_keymap() {
    let (config, _) = parse("[keys]\ndefaults = false\n\n[keys.direct]\n\"cmd+y\" = \"copy\"\n");
    let press = |s: &str| KeyBinding::parse(s).unwrap();
    assert_eq!(
        config.keymap.resolve(press("cmd+d"), false),
        KeyResolution::Unbound
    );
    assert_eq!(
        config.keymap.resolve(press("cmd+y"), false),
        KeyResolution::Run(Action::Copy)
    );
}

#[test]
fn the_prefix_can_be_disabled_from_config() {
    let (config, warnings) = parse("[keys]\nprefix = \"none\"\n");
    assert!(warnings.is_empty());
    assert_eq!(config.keymap.prefix(), None);
}

#[test]
fn bad_bindings_warn_without_losing_the_good_ones() {
    let (config, warnings) = parse(
        "[keys.direct]\n\"not a key\" = \"quit\"\n\"cmd+y\" = \"nonsense\"\n\"cmd+u\" = \"copy\"\n",
    );
    assert_eq!(warnings.len(), 2, "{warnings:?}");
    let press = |s: &str| KeyBinding::parse(s).unwrap();
    assert_eq!(
        config.keymap.resolve(press("cmd+u"), false),
        KeyResolution::Run(Action::Copy)
    );
}

// A Mac has no Alt key. Showing `alt+left` sends people hunting for a key
// that is not on the keyboard in front of them.
#[test]
fn modifiers_are_named_the_way_the_platform_names_them() {
    let names = |p: Platform| {
        p.modifier_names()
            .iter()
            .map(|(_, n)| *n)
            .collect::<Vec<_>>()
    };
    assert_eq!(names(Platform::MacOs), ["cmd", "ctrl", "option", "shift"]);
    assert_eq!(names(Platform::Windows), ["win", "ctrl", "alt", "shift"]);
    assert_eq!(names(Platform::Other), ["super", "ctrl", "alt", "shift"]);
}

// Only the names are local. A config file written on one machine has to keep
// working on another, so every alias parses everywhere.
#[test]
fn every_modifier_alias_parses_on_every_platform() {
    for name in ["alt", "opt", "option", "meta"] {
        let binding = KeyBinding::parse(&format!("{name}+left")).expect(name);
        assert_eq!(binding.mods, Modifiers::ALT, "{name}");
    }
    for name in ["cmd", "command", "super", "win"] {
        let binding = KeyBinding::parse(&format!("{name}+left")).expect(name);
        assert_eq!(binding.mods, Modifiers::SUPER, "{name}");
    }
    for name in ["ctrl", "control", "c"] {
        let binding = KeyBinding::parse(&format!("{name}+left")).expect(name);
        assert_eq!(binding.mods, Modifiers::CTRL, "{name}");
    }
}

#[test]
fn a_printed_binding_parses_back_to_itself() {
    for spec in ["cmd+alt+left", "ctrl+b", "cmd+alt+shift+right", "opt+up"] {
        let binding = KeyBinding::parse(spec).expect(spec);
        let shown = binding.to_string();
        assert_eq!(KeyBinding::parse(&shown).expect(&shown), binding, "{spec}");
    }
}
