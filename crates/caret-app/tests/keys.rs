use caret_app::encode_for_child;
use caret_config::{Key, KeyBinding, Modifiers};

fn encode(key: Key, mods: Modifiers) -> String {
    let binding = KeyBinding { key, mods };
    String::from_utf8(encode_for_child(&binding, false).expect("encoded")).unwrap()
}

fn unmodified(key: Key) -> String {
    encode(key, Modifiers::NONE)
}

#[test]
fn arrows_follow_application_cursor_mode() {
    let up = KeyBinding {
        key: Key::Up,
        mods: Modifiers::NONE,
    };
    assert_eq!(encode_for_child(&up, false).unwrap(), b"\x1b[A");
    assert_eq!(encode_for_child(&up, true).unwrap(), b"\x1bOA");
}

#[test]
fn modified_arrows_carry_the_xterm_parameter() {
    assert_eq!(encode(Key::Up, Modifiers::SHIFT), "\x1b[1;2A");
    assert_eq!(encode(Key::Up, Modifiers::CTRL), "\x1b[1;5A");
}

#[test]
fn option_arrows_move_by_word() {
    assert_eq!(encode(Key::Left, Modifiers::ALT), "\x1bb");
    assert_eq!(encode(Key::Right, Modifiers::ALT), "\x1bf");
}

#[test]
fn command_arrows_jump_to_the_ends_of_the_line() {
    assert_eq!(encode(Key::Left, Modifiers::SUPER), unmodified(Key::Home));
    assert_eq!(encode(Key::Right, Modifiers::SUPER), unmodified(Key::End));
}

#[test]
fn option_delete_removes_a_word_and_command_delete_the_line() {
    assert_eq!(encode(Key::Backspace, Modifiers::ALT), "\x1b\x7f");
    assert_eq!(encode(Key::Backspace, Modifiers::SUPER), "\x15");
    assert_eq!(encode(Key::Delete, Modifiers::ALT), "\x1bd");
    assert_eq!(encode(Key::Delete, Modifiers::SUPER), "\x0b");
}

// The line-editing table must not swallow the plain keys it shares.
#[test]
fn unmodified_editing_keys_keep_their_usual_sequences() {
    assert_eq!(unmodified(Key::Backspace), "\x7f");
    assert_eq!(unmodified(Key::Delete), "\x1b[3~");
    assert_eq!(unmodified(Key::Left), "\x1b[D");
}

#[test]
fn a_combination_with_other_modifiers_is_not_line_editing() {
    assert_eq!(
        encode(Key::Left, Modifiers::ALT | Modifiers::SHIFT),
        "\x1b[1;4D"
    );
}
