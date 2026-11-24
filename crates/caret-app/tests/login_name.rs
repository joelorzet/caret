use caret_app::login_name;

// A shell is a login shell when its own name starts with a dash, which is
// how every terminal has started one since the beginning.
#[test]
fn a_login_shell_is_its_own_name_with_a_dash() {
    assert_eq!(login_name("/bin/zsh"), "-zsh");
    assert_eq!(login_name("/opt/homebrew/bin/fish"), "-fish");
    assert_eq!(login_name("bash"), "-bash");
}

#[test]
fn a_path_that_ends_in_nothing_is_passed_through() {
    assert_eq!(login_name("/"), "-/");
    assert_eq!(login_name(""), "-");
}
