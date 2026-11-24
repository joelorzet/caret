use caret_features::shell_names;

// What zsh prints: bare assignments, quoted or not, and a line of its own
// about restoring the session that is nothing to do with us.
#[test]
fn zsh_aliases_are_read_and_its_chatter_is_not() {
    let listing = "\
Restored session: Tue Nov 24 15:11:32 -03 2026
-='cd -'
...=../..
gst='git status'
ll='ls -lah'
";
    // `...` is a real alias for `cd ../..`, so it belongs; the session
    // line and the one for `cd -` do not.
    assert_eq!(shell_names(listing), ["...", "gst", "ll"]);
}

// What bash prints: every line prefixed, and functions through declare.
#[test]
fn bash_aliases_and_functions_are_read() {
    let listing = "\
alias ll='ls -alF'
alias gs='git status'
declare -f mkcd
declare -f work_on
";
    assert_eq!(shell_names(listing), ["gs", "ll", "mkcd", "work_on"]);
}

#[test]
fn a_list_of_bare_function_names_is_read() {
    let listing = "_git_log_prettily\nmkcd\nwork_on\nfunction older_style\n";
    assert_eq!(
        shell_names(listing),
        ["_git_log_prettily", "mkcd", "older_style", "work_on"]
    );
}

// A shell that answers with nothing useful must leave the index as it was,
// rather than colouring half a sentence as commands.
#[test]
fn nothing_that_is_not_a_name_gets_through() {
    let listing = "\
\n\
zsh: command not found: nonsense
some sentence with spaces
=novalue
'quoted thing'=x
";
    assert!(
        shell_names(listing).is_empty(),
        "{:?}",
        shell_names(listing)
    );
    assert!(shell_names("").is_empty());
}

#[test]
fn the_same_name_twice_is_one_name() {
    assert_eq!(shell_names("alias ll='ls'\nll\nll=ls\n"), ["ll"]);
}
