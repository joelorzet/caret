use caret_config::{Key, KeyBinding, Modifiers};

#[test]
fn parses_a_bare_character() {
    let b = KeyBinding::parse("%").unwrap();
    assert_eq!(b.key, Key::Char('%'));
    assert!(b.mods.is_empty());
}

#[test]
fn parses_modifiers_in_any_order() {
    let a = KeyBinding::parse("cmd+alt+left").unwrap();
    let b = KeyBinding::parse("alt+cmd+left").unwrap();
    assert_eq!(a, b);
    assert_eq!(a.key, Key::Left);
    assert!(a.mods.contains(Modifiers::SUPER | Modifiers::ALT));
}

#[test]
fn accepts_modifier_aliases() {
    for spec in ["cmd+q", "command+q", "super+q", "win+q"] {
        assert_eq!(
            KeyBinding::parse(spec).unwrap(),
            KeyBinding::new(Modifiers::SUPER, Key::Char('q')),
            "{spec}"
        );
    }
    for spec in ["alt+q", "opt+q", "option+q", "meta+q"] {
        assert_eq!(
            KeyBinding::parse(spec).unwrap().mods,
            Modifiers::ALT,
            "{spec}"
        );
    }
}

#[test]
fn parses_named_keys_and_function_keys() {
    assert_eq!(KeyBinding::parse("enter").unwrap().key, Key::Enter);
    assert_eq!(KeyBinding::parse("pgup").unwrap().key, Key::PageUp);
    assert_eq!(KeyBinding::parse("f12").unwrap().key, Key::F(12));
    assert!(KeyBinding::parse("f99").is_err());
}

#[test]
fn parses_punctuation_by_name_and_literally() {
    assert_eq!(KeyBinding::parse("slash").unwrap().key, Key::Char('/'));
    assert_eq!(KeyBinding::parse("cmd+/").unwrap().key, Key::Char('/'));
    assert_eq!(KeyBinding::parse("cmd++").unwrap().key, Key::Char('+'));
    assert_eq!(KeyBinding::parse("plus").unwrap().key, Key::Char('+'));
}

// Writing the shifted character is unambiguous across layouts; shift+d is not.
#[test]
fn rejects_shift_with_a_character_key() {
    let err = KeyBinding::parse("cmd+shift+d").unwrap_err();
    assert!(err.reason.contains("shifted character"), "{err}");
    assert!(KeyBinding::parse("cmd+D").is_ok());
}

#[test]
fn allows_shift_with_a_named_key() {
    let b = KeyBinding::parse("ctrl+shift+tab").unwrap();
    assert_eq!(b.key, Key::Tab);
    assert!(b.mods.contains(Modifiers::SHIFT));
}

#[test]
fn reports_what_was_wrong() {
    assert!(KeyBinding::parse("").is_err());
    assert!(KeyBinding::parse("hyper+a")
        .unwrap_err()
        .reason
        .contains("modifier"));
    assert!(KeyBinding::parse("cmd+nope")
        .unwrap_err()
        .reason
        .contains("key"));
}

// Shift is already baked into the character, so it is not compared for
// character keys; for named keys it matters.
#[test]
fn matching_ignores_shift_only_for_characters() {
    let pct = KeyBinding::parse("%").unwrap();
    let shifted_pct = KeyBinding::new(Modifiers::SHIFT, Key::Char('%'));
    assert!(pct.matches(&shifted_pct));

    let tab = KeyBinding::parse("ctrl+tab").unwrap();
    let shift_tab = KeyBinding::parse("ctrl+shift+tab").unwrap();
    assert!(!tab.matches(&shift_tab));
}

#[test]
fn matching_requires_the_other_modifiers() {
    let a = KeyBinding::parse("cmd+d").unwrap();
    assert!(!a.matches(&KeyBinding::parse("d").unwrap()));
    assert!(!a.matches(&KeyBinding::parse("cmd+alt+d").unwrap()));
    assert!(a.matches(&KeyBinding::parse("cmd+d").unwrap()));
}

// A binding is shown with the names on the keyboard in front of the person
// reading it: `option` on a Mac, which has no Alt key, `alt` elsewhere.
#[test]
fn displays_the_way_people_write_it() {
    let (command, option) = if cfg!(target_os = "macos") {
        ("cmd", "option")
    } else {
        ("super", "alt")
    };
    assert_eq!(
        KeyBinding::parse("cmd+alt+left").unwrap().to_string(),
        format!("{command}+{option}+left")
    );
    assert_eq!(KeyBinding::parse("ctrl+b").unwrap().to_string(), "ctrl+b");
    assert_eq!(KeyBinding::parse("%").unwrap().to_string(), "%");
}

// A binding written as `space` has to match a typed space; they were once
// different variants and the binding could never fire.
#[test]
fn space_is_a_character_however_it_is_written() {
    let named = KeyBinding::parse("space").unwrap();
    assert_eq!(named.key, Key::Char(' '));
    assert_eq!(named.to_string(), "space");

    let typed = KeyBinding::new(Modifiers::NONE, Key::Char(' '));
    assert!(named.matches(&typed));

    let prefixed = KeyBinding::parse("ctrl+space").unwrap();
    assert!(prefixed.matches(&KeyBinding::new(Modifiers::CTRL, Key::Char(' '))));
}
