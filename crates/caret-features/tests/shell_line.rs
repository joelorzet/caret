use caret_features::{shell_spans, ShellRole};

/// The role covering the character at `at`, for asserting about a position
/// rather than about the span list's shape.
fn role_at(line: &str, at: usize) -> Option<ShellRole> {
    shell_spans(line)
        .into_iter()
        .find(|s| at >= s.start && at < s.end)
        .map(|s| s.role)
}

/// Every span of `role`, as the text it covers.
fn texts(line: &str, role: ShellRole) -> Vec<String> {
    let chars: Vec<char> = line.chars().collect();
    shell_spans(line)
        .into_iter()
        .filter(|s| s.role == role)
        .map(|s| chars[s.start..s.end].iter().collect())
        .collect()
}

#[test]
fn the_first_word_is_the_command() {
    assert_eq!(texts("ls -la", ShellRole::Command), ["ls"]);
    assert_eq!(texts("ls -la", ShellRole::Flag), ["-la"]);
}

#[test]
fn a_tools_own_verb_reads_differently_from_its_arguments() {
    let line = "git commit message";
    assert_eq!(texts(line, ShellRole::Command), ["git"]);
    assert_eq!(texts(line, ShellRole::Subcommand), ["commit"]);
    assert_eq!(texts(line, ShellRole::Plain), ["message"]);
}

// Everything after a pipe is a program again, which is the thing that makes
// a long pipeline readable.
#[test]
fn each_stage_of_a_pipeline_starts_a_new_command() {
    let line = "cat file | grep -i foo | wc -l";
    assert_eq!(texts(line, ShellRole::Command), ["cat", "grep", "wc"]);
    assert_eq!(texts(line, ShellRole::Operator), ["|", "|"]);
}

#[test]
fn separators_other_than_pipes_also_restart_the_line() {
    for line in ["a; b", "a && b", "a || b", "a & b"] {
        assert_eq!(texts(line, ShellRole::Command), ["a", "b"], "{line}");
    }
}

// A redirection is plumbing, but it does not start a new command.
#[test]
fn a_redirection_does_not_restart_the_line() {
    let line = "echo hi > out.txt";
    assert_eq!(texts(line, ShellRole::Command), ["echo"]);
    assert_eq!(texts(line, ShellRole::Operator), [">"]);
}

#[test]
fn quotes_cover_what_is_inside_them() {
    assert_eq!(
        texts("echo 'hello world'", ShellRole::Quoted),
        ["'hello world'"]
    );
    assert_eq!(texts("echo \"a b\"", ShellRole::Quoted), ["\"a b\""]);
}

// Half-typed is the normal state of a line being typed, so an unclosed quote
// has to colour as a string rather than leaving the rest uncoloured.
#[test]
fn an_unclosed_quote_runs_to_the_end_of_the_line() {
    assert_eq!(texts("echo 'unfinis", ShellRole::Quoted), ["'unfinis"]);
}

#[test]
fn a_pipe_inside_quotes_is_not_a_pipe() {
    let line = "echo 'a | b' end";
    assert!(texts(line, ShellRole::Operator).is_empty());
    assert_eq!(texts(line, ShellRole::Command), ["echo"]);
}

#[test]
fn variables_are_picked_out_in_both_forms() {
    assert_eq!(texts("echo $HOME", ShellRole::Variable), ["$HOME"]);
    assert_eq!(texts("echo ${HOME}", ShellRole::Variable), ["${HOME}"]);
    assert_eq!(texts("cd ${A}/${B}", ShellRole::Variable), ["${A}", "${B}"]);
}

#[test]
fn a_lone_dollar_is_just_a_dollar() {
    assert_eq!(role_at("echo $", 5), Some(ShellRole::Variable));
    assert_eq!(shell_spans("echo $").len(), 2);
}

#[test]
fn an_assignment_separates_its_name_from_its_value() {
    let line = "REGION=us-east-2";
    assert_eq!(texts(line, ShellRole::Variable), ["REGION"]);
    assert_eq!(texts(line, ShellRole::Operator), ["="]);
    assert!(texts(line, ShellRole::Command).is_empty());
}

// `FOO=bar cmd` still runs cmd, so the command slot must survive the prefix.
#[test]
fn an_assignment_prefix_leaves_the_command_slot_open() {
    let line = "FOO=bar cargo build";
    assert_eq!(texts(line, ShellRole::Command), ["cargo"]);
    assert_eq!(texts(line, ShellRole::Subcommand), ["build"]);
}

#[test]
fn a_long_flag_with_a_value_splits_at_the_equals() {
    let line = "aws --region=us-east-2";
    assert_eq!(texts(line, ShellRole::Flag), ["--region"]);
    assert_eq!(texts(line, ShellRole::Operator), ["="]);
}

#[test]
fn arguments_that_look_like_places_read_as_places() {
    assert_eq!(texts("cat src/main.rs", ShellRole::Path), ["src/main.rs"]);
    assert_eq!(texts("cd ~/work", ShellRole::Path), ["~/work"]);
    assert_eq!(texts("ls ./here", ShellRole::Path), ["./here"]);
}

#[test]
fn numbers_read_as_numbers() {
    assert_eq!(texts("sleep 30", ShellRole::Number), ["30"]);
    assert_eq!(texts("kill 1234", ShellRole::Number), ["1234"]);
}

#[test]
fn a_comment_runs_to_the_end() {
    let line = "ls # everything here";
    assert_eq!(texts(line, ShellRole::Comment), ["# everything here"]);
}

#[test]
fn a_hash_inside_a_word_is_not_a_comment() {
    let line = "git show abc#1";
    assert!(texts(line, ShellRole::Comment).is_empty());
}

// The spans are read against the grid, so they must stay inside the line and
// never overlap or the colouring walks over the wrong cells.
#[test]
fn spans_are_ordered_disjoint_and_inside_the_line() {
    for line in [
        "",
        "ls",
        "git commit -m 'a message' && echo $? > /tmp/x # done",
        "FOO=1 BAR=2 cmd --flag=val arg ~/p | wc -l",
        "echo \"unclosed",
        "|||",
        "$",
    ] {
        let len = line.chars().count();
        let spans = shell_spans(line);
        let mut last = 0;
        for span in &spans {
            assert!(span.start >= last, "{line:?} overlaps at {}", span.start);
            assert!(span.start < span.end, "{line:?} has an empty span");
            assert!(span.end <= len, "{line:?} runs past the end");
            last = span.end;
        }
    }
}

#[test]
fn an_empty_line_has_nothing_to_colour() {
    assert!(shell_spans("").is_empty());
    assert!(shell_spans("   ").is_empty());
}
