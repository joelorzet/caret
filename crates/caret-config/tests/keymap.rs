use caret_config::{Action, ActionGroup, Key, KeyBinding, KeyResolution, Keymap, Modifiers};
use caret_layout::{Axis, Direction, DockKind};

fn press(spec: &str) -> KeyBinding {
    KeyBinding::parse(spec).unwrap()
}

#[test]
fn the_defaults_all_parse() {
    let map = Keymap::default();
    assert_eq!(map.prefix(), Some(press("ctrl+b")));
    let total: usize = map.shortcuts().iter().map(|(_, r)| r.len()).sum();
    assert!(total > 20, "only {total} actions bound");
}

#[test]
fn direct_bindings_resolve_without_the_prefix() {
    let map = Keymap::default();
    assert_eq!(
        map.resolve(press("cmd+d"), false),
        KeyResolution::Run(Action::SplitPane(Axis::Horizontal))
    );
    assert_eq!(
        map.resolve(press("cmd+D"), false),
        KeyResolution::Run(Action::SplitPane(Axis::Vertical))
    );
}

#[test]
fn the_prefix_arms_the_prefixed_table() {
    let map = Keymap::default();
    assert_eq!(
        map.resolve(press("ctrl+b"), false),
        KeyResolution::PrefixPending
    );
    assert_eq!(
        map.resolve(press("%"), true),
        KeyResolution::Run(Action::SplitPane(Axis::Horizontal))
    );
    // Without the prefix armed, `%` is just a character for the shell.
    assert_eq!(map.resolve(press("%"), false), KeyResolution::Unbound);
}

#[test]
fn an_armed_prefix_does_not_rearm_itself() {
    let map = Keymap::default();
    assert_eq!(map.resolve(press("ctrl+b"), true), KeyResolution::Unbound);
}

#[test]
fn sidebar_toggles_are_bound_both_ways() {
    let map = Keymap::default();
    assert_eq!(
        map.resolve(press("cmd+b"), false),
        KeyResolution::Run(Action::ToggleSidebar(DockKind::Explorer))
    );
    assert_eq!(
        map.resolve(press("e"), true),
        KeyResolution::Run(Action::ToggleSidebar(DockKind::Explorer))
    );
    assert_eq!(
        map.resolve(press("cmd+j"), false),
        KeyResolution::Run(Action::ToggleSidebar(DockKind::Agent))
    );
    assert_eq!(
        map.resolve(press("a"), true),
        KeyResolution::Run(Action::ToggleSidebar(DockKind::Agent))
    );
}

#[test]
fn vim_keys_and_arrows_both_move_focus() {
    let map = Keymap::default();
    for (spec, dir) in [
        ("h", Direction::Left),
        ("j", Direction::Down),
        ("k", Direction::Up),
        ("l", Direction::Right),
    ] {
        assert_eq!(
            map.resolve(press(spec), true),
            KeyResolution::Run(Action::FocusPane(dir)),
            "{spec}"
        );
    }
    assert_eq!(
        map.resolve(press("left"), true),
        KeyResolution::Run(Action::FocusPane(Direction::Left))
    );
}

#[test]
fn binding_replaces_rather_than_duplicates() {
    let mut map = Keymap::empty();
    let key = press("cmd+d");
    map.bind(key, Action::Quit, false);
    map.bind(key, Action::NewTab, false);
    assert_eq!(map.resolve(key, false), KeyResolution::Run(Action::NewTab));
}

#[test]
fn unbinding_removes_a_key() {
    let mut map = Keymap::default();
    let key = press("cmd+q");
    assert_eq!(map.resolve(key, false), KeyResolution::Run(Action::Quit));
    map.unbind(key, false);
    assert_eq!(map.resolve(key, false), KeyResolution::Unbound);
}

#[test]
fn the_prefix_can_be_turned_off() {
    let mut map = Keymap::default();
    map.set_prefix(None);
    assert_eq!(map.resolve(press("ctrl+b"), false), KeyResolution::Unbound);
}

#[test]
fn shortcuts_are_grouped_and_merge_their_keys() {
    let map = Keymap::default();
    let groups = map.shortcuts();

    let names: Vec<ActionGroup> = groups.iter().map(|(g, _)| *g).collect();
    assert_eq!(names.first(), Some(&ActionGroup::Panes));
    assert!(names.contains(&ActionGroup::Sidebars));

    let split = groups
        .iter()
        .flat_map(|(_, rows)| rows)
        .find(|s| s.action == Action::SplitPane(Axis::Horizontal))
        .expect("split should be listed");
    assert!(split.keys.len() >= 2, "both bindings should be listed");
    assert!(split.keys_display().contains(" / "));
    assert!(split.keys.iter().any(|k| k.starts_with("ctrl+b ")));
}

#[test]
fn bindings_for_finds_every_key_of_an_action() {
    let map = Keymap::default();
    let keys = map.bindings_for(&Action::ToggleZoom);
    // Printed the way this platform prints it: a Mac says `cmd`, everything
    // else says `super`, and what is being tested here is the lookup.
    let zoom = KeyBinding::parse("cmd+enter").expect("parses").to_string();
    assert!(keys.contains(&zoom), "{keys:?}");
    assert!(keys.iter().any(|k| k == "ctrl+b z"), "{keys:?}");
    assert!(map.bindings_for(&Action::SelectTab(9)).is_empty());
}

#[test]
fn an_empty_keymap_resolves_nothing() {
    let map = Keymap::empty();
    assert_eq!(map.resolve(press("cmd+d"), false), KeyResolution::Unbound);
    assert!(map.shortcuts().is_empty());
}

#[test]
fn unknown_keys_are_left_for_the_shell() {
    let map = Keymap::default();
    assert_eq!(
        map.resolve(KeyBinding::new(Modifiers::NONE, Key::Char('x')), false),
        KeyResolution::Unbound
    );
}

// A window is asked for the way a tab is, and the name is what a
// configuration file writes to rebind it.
#[test]
fn another_window_is_bound_and_named() {
    let map = Keymap::default();
    assert_eq!(
        map.resolve(press("cmd+n"), false),
        KeyResolution::Run(Action::NewWindow)
    );
    assert_eq!(Action::from_name("new_window"), Some(Action::NewWindow));
    assert_eq!(Action::NewWindow.name(), "new_window".to_string());
    assert!(!Action::NewWindow.description().is_empty());
    assert_eq!(Action::NewWindow.group(), ActionGroup::Tabs);
}
