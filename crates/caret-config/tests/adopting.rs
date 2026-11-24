// `adopt_former_config` reads HOME, and the only honest way to test it is to
// point HOME somewhere safe. Setting an environment variable is process wide,
// so this test lives in a file of its own: nothing else runs beside it.

// A rename must not be able to lose someone's settings, and must not delete
// them either.
#[test]
fn a_configuration_written_under_the_old_name_is_adopted() {
    let root = std::env::temp_dir().join(format!("caret-adopt-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let before = root.join(".config").join("main-terminal");
    std::fs::create_dir_all(before.join("themes")).expect("the old directory");
    std::fs::write(before.join("config.toml"), "theme = \"nord\"\n").expect("a config");
    std::fs::write(before.join("themes").join("mine.toml"), "x = 1\n").expect("a theme");

    // An editor's working files name the path they were made for, so carrying
    // one across produces a warning about a file that is not there. They have
    // to be left behind, which means they have to be here to leave behind.
    for leftover in [".config.toml.swp", "config.toml~", ".DS_Store"] {
        std::fs::write(before.join(leftover), "junk\n").expect("a leftover");
    }

    // Scoped to this test's own HOME, so nothing outside it is touched.
    temp_env(&root, || {
        caret_config::paths::adopt_former_config();
    });

    let now = root.join(".config").join("caret");
    assert_eq!(
        std::fs::read_to_string(now.join("config.toml")).expect("carried across"),
        "theme = \"nord\"\n"
    );
    assert!(now.join("themes").join("mine.toml").exists(), "themes too");
    assert!(
        before.join("config.toml").exists(),
        "the old one is left alone"
    );
    for leftover in [".config.toml.swp", "config.toml~", ".DS_Store"] {
        assert!(
            !now.join(leftover).exists(),
            "{leftover} should not have come across"
        );
    }

    // Running again must not overwrite what is now the real configuration.
    std::fs::write(now.join("config.toml"), "theme = \"dracula\"\n").expect("edited");
    temp_env(&root, || {
        caret_config::paths::adopt_former_config();
    });
    assert_eq!(
        std::fs::read_to_string(now.join("config.toml")).expect("still there"),
        "theme = \"dracula\"\n"
    );

    let _ = std::fs::remove_dir_all(&root);
}

/// Runs `body` with `HOME` pointed at `root`, putting it back afterwards.
fn temp_env(root: &std::path::Path, body: impl FnOnce()) {
    let home = std::env::var_os("HOME");
    let xdg = std::env::var_os("XDG_CONFIG_HOME");
    unsafe {
        std::env::set_var("HOME", root);
        std::env::remove_var("XDG_CONFIG_HOME");
    }
    body();
    unsafe {
        match home {
            Some(value) => std::env::set_var("HOME", value),
            None => std::env::remove_var("HOME"),
        }
        if let Some(value) = xdg {
            std::env::set_var("XDG_CONFIG_HOME", value);
        }
    }
}
