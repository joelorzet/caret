use caret_features::CommandIndex;

#[test]
fn the_shells_own_words_are_always_known() {
    let index = CommandIndex::from_path();
    for word in ["cd", "echo", "export", "source", "if", "while", "unset"] {
        assert!(index.contains(word), "{word} should be recognised");
    }
}

// Colour says "this would run". A typo must not get it.
#[test]
fn a_word_that_runs_nothing_is_not_recognised() {
    let index = CommandIndex::from_path();
    for word in ["", "gitt", "nosuchcommandanywhere", "ls -la", "--help"] {
        assert!(!index.contains(word), "{word:?} should not be recognised");
    }
}

// A path written out is taken at its word: stat-ing it on every keystroke
// would cost more than the mistake it prevents.
#[test]
fn a_written_out_path_counts_as_a_command() {
    let index = CommandIndex::from_path();
    assert!(index.contains("./build.sh"));
    assert!(index.contains("/usr/bin/env"));
    assert!(index.contains("~/bin/thing"));
}

#[test]
fn the_path_itself_turns_up_something() {
    let index = CommandIndex::from_path();
    assert!(index.len() > 20, "only found {}", index.len());
    assert!(!index.is_empty());
}
