use caret_pty::{PtyId, PtyPool, PtySpec};

/// A shell is what panes actually run, and it is the thing that changes
/// directory without telling anyone.
fn spawn_in(pool: &mut PtyPool, id: PtyId, dir: &str) {
    pool.spawn(
        id,
        &PtySpec {
            program: "/bin/sh".into(),
            args: vec!["-c".into(), "sleep 5".into()],
            arg0: None,
            cwd: Some(std::path::PathBuf::from(dir)),
            env: Vec::new(),
            cols: 80,
            rows: 24,
        },
    )
    .expect("the shell starts");
}

// Without this a new pane opens wherever the window was started, because a
// shell only reports its directory if it has been set up to.
#[test]
fn a_panes_working_directory_is_known_without_the_shell_saying_so() {
    let mut pool = PtyPool::new();
    let id = PtyId(1);
    spawn_in(&mut pool, id, "/usr");

    let found = pool.working_dir(id).expect("the kernel knows where it is");
    assert_eq!(found, std::path::PathBuf::from("/usr"));
}

#[test]
fn each_pane_is_asked_about_separately() {
    let mut pool = PtyPool::new();
    spawn_in(&mut pool, PtyId(1), "/usr");
    spawn_in(&mut pool, PtyId(2), "/tmp");

    assert_eq!(
        pool.working_dir(PtyId(1)),
        Some(std::path::PathBuf::from("/usr"))
    );
    // /tmp is a link to /private/tmp on macOS, so the kernel answers with
    // the real path rather than the one we asked for.
    let second = pool.working_dir(PtyId(2)).expect("a directory");
    assert!(second.ends_with("tmp"), "{second:?}");
}

#[test]
fn a_pane_that_is_not_there_has_no_directory() {
    let pool = PtyPool::new();
    assert_eq!(pool.working_dir(PtyId(9)), None);
}
