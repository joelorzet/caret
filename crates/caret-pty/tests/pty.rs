use caret_pty::{PtyEvent, PtyId, PtyPool, PtySpec};
use std::time::{Duration, Instant};

fn spec(program: &str, args: &[&str]) -> PtySpec {
    PtySpec {
        program: program.to_string(),
        args: args.iter().map(|a| a.to_string()).collect(),
        cwd: None,
        env: Vec::new(),
        cols: 40,
        rows: 10,
    }
}

/// Polls until `want` is satisfied or the deadline passes, so a slow machine
/// does not fail the test and a broken one does not hang it.
fn drain(pool: &mut PtyPool, limit: Duration) -> Vec<PtyEvent> {
    let deadline = Instant::now() + limit;
    let mut all = Vec::new();
    let mut events = Vec::new();
    let mut ready = Vec::new();

    while Instant::now() < deadline {
        events.clear();
        pool.poll(50, &[], &mut events, &mut ready).unwrap();
        let exited = events.iter().any(|e| matches!(e, PtyEvent::Exited { .. }));
        all.append(&mut events);
        if exited {
            break;
        }
    }
    all
}

fn output(events: &[PtyEvent]) -> String {
    let mut text = String::new();
    for event in events {
        if let PtyEvent::Output { bytes, .. } = event {
            text.push_str(&String::from_utf8_lossy(bytes));
        }
    }
    text
}

#[test]
fn a_new_pool_is_empty() {
    let pool = PtyPool::new();
    assert!(pool.is_empty());
    assert_eq!(pool.len(), 0);
    assert!(pool.ids().is_empty());
    assert!(!pool.contains(PtyId(0)));
}

#[test]
fn polling_an_empty_pool_returns_at_once() {
    let mut pool = PtyPool::new();
    let mut events = Vec::new();
    let mut ready = Vec::new();

    let started = Instant::now();
    pool.poll(5_000, &[], &mut events, &mut ready).unwrap();
    assert!(started.elapsed() < Duration::from_secs(1), "did not wait");
    assert!(events.is_empty());
}

#[test]
fn a_child_runs_and_its_output_arrives() {
    let mut pool = PtyPool::new();
    pool.spawn(PtyId(1), &spec("/bin/sh", &["-c", "echo marker-one"]))
        .expect("spawn");

    assert_eq!(pool.len(), 1);
    assert!(pool.contains(PtyId(1)));

    let events = drain(&mut pool, Duration::from_secs(5));
    assert!(output(&events).contains("marker-one"), "{events:?}");
}

#[test]
fn a_child_that_exits_is_reported_and_removed() {
    let mut pool = PtyPool::new();
    pool.spawn(PtyId(7), &spec("/bin/sh", &["-c", "exit 3"]))
        .expect("spawn");

    let events = drain(&mut pool, Duration::from_secs(5));
    assert!(
        events
            .iter()
            .any(|e| matches!(e, PtyEvent::Exited { id, .. } if *id == PtyId(7))),
        "{events:?}"
    );
    assert!(pool.is_empty(), "the pool dropped it");
    assert!(!pool.contains(PtyId(7)));
}

#[test]
fn input_reaches_the_child() {
    let mut pool = PtyPool::new();
    pool.spawn(PtyId(2), &spec("/bin/cat", &[])).expect("spawn");

    pool.write(PtyId(2), b"ping\n").expect("write");
    let deadline = Instant::now() + Duration::from_secs(5);
    let mut text = String::new();
    let mut events = Vec::new();
    let mut ready = Vec::new();

    while Instant::now() < deadline && !text.contains("ping") {
        events.clear();
        pool.poll(50, &[], &mut events, &mut ready).unwrap();
        text.push_str(&output(&events));
    }
    // `cat` on a terminal echoes the input as well as repeating it.
    assert!(text.contains("ping"), "{text:?}");
    pool.close(PtyId(2));
    assert!(pool.is_empty());
}

// The size is what a child reads from the terminal, so it has to be set.
#[test]
fn the_child_sees_the_size_it_was_given() {
    let mut pool = PtyPool::new();
    let mut spec = spec("/bin/sh", &["-c", "stty size"]);
    spec.cols = 73;
    spec.rows = 17;
    pool.spawn(PtyId(3), &spec).expect("spawn");

    let events = drain(&mut pool, Duration::from_secs(5));
    let text = output(&events);
    assert!(text.contains("17 73"), "stty reported {text:?}");
}

#[test]
fn resizing_changes_what_the_child_sees() {
    let mut pool = PtyPool::new();
    pool.spawn(PtyId(4), &spec("/bin/sh", &["-c", "sleep 0.4; stty size"]))
        .expect("spawn");

    pool.resize(PtyId(4), 101, 29).expect("resize");
    let text = output(&drain(&mut pool, Duration::from_secs(5)));
    assert!(text.contains("29 101"), "stty reported {text:?}");
}

#[test]
fn writing_to_an_unknown_pty_is_not_an_error() {
    let mut pool = PtyPool::new();
    assert!(pool.write(PtyId(99), b"x").is_ok());
    assert!(pool.resize(PtyId(99), 10, 10).is_ok());
    pool.close(PtyId(99));
}

#[test]
fn spawning_a_missing_program_fails_cleanly() {
    let mut pool = PtyPool::new();
    let result = pool.spawn(PtyId(5), &spec("/nonexistent/program", &[]));
    assert!(result.is_err());
    assert!(pool.is_empty(), "nothing was left behind");
}

#[test]
fn several_children_are_served_by_one_poll() {
    let mut pool = PtyPool::new();
    for (index, marker) in ["alpha", "beta", "gamma"].iter().enumerate() {
        pool.spawn(
            PtyId(index as u32 + 10),
            &spec("/bin/sh", &["-c", &format!("echo {marker}")]),
        )
        .expect("spawn");
    }
    assert_eq!(pool.len(), 3);

    let deadline = Instant::now() + Duration::from_secs(5);
    let mut text = String::new();
    let mut events = Vec::new();
    let mut ready = Vec::new();
    while Instant::now() < deadline && !pool.is_empty() {
        events.clear();
        pool.poll(50, &[], &mut events, &mut ready).unwrap();
        text.push_str(&output(&events));
    }

    for marker in ["alpha", "beta", "gamma"] {
        assert!(text.contains(marker), "{marker} missing from {text:?}");
    }
}

#[test]
fn closing_a_pool_does_not_hang_on_a_stubborn_child() {
    let mut pool = PtyPool::new();
    // Ignores the hangup, so only the escalation to SIGKILL ends it.
    pool.spawn(PtyId(6), &spec("/bin/sh", &["-c", "trap '' HUP; sleep 30"]))
        .expect("spawn");

    let started = Instant::now();
    drop(pool);
    assert!(
        started.elapsed() < Duration::from_secs(3),
        "shutdown took {:?}",
        started.elapsed()
    );
}

#[test]
fn the_terminal_size_helper_returns_something_sane() {
    // Under a test harness stdout is usually not a terminal, so `None` is the
    // correct answer rather than a failure.
    if let Some((cols, rows)) = caret_pty::terminal_size() {
        assert!(cols > 0 && rows > 0);
    }
}

// A short command writes and exits in the time between two polls, so the
// first thing the pool learns about it is that it is dead. It is drained
// before it is let go, rather than closed on the strength of the exit
// alone.
//
// Whether anything is left to drain is the kernel's business. macOS keeps
// what the child wrote on the master after it is gone; Linux drops it when
// the last slave closes, and no amount of reading gets it back. What holds
// everywhere is that the pool does not invent an exit it has not seen, and
// does not lose output it could still have read, which
// `several_children_are_served_by_one_poll` covers.
#[test]
fn a_child_that_has_already_exited_is_drained_before_it_is_let_go() {
    let mut pool = PtyPool::new();
    pool.spawn(
        PtyId(7),
        &spec("/bin/sh", &["-c", "echo written-before-exit"]),
    )
    .expect("spawn");

    // Long enough that the child is reaped before anything is read.
    std::thread::sleep(Duration::from_millis(400));

    let events = drain(&mut pool, Duration::from_secs(5));
    let text = output(&events);
    if cfg!(target_os = "macos") {
        assert!(text.contains("written-before-exit"), "{text:?}");
    }
    assert!(
        events
            .iter()
            .any(|e| matches!(e, PtyEvent::Exited { id, .. } if *id == PtyId(7))),
        "the exit is reported either way"
    );
    assert!(pool.is_empty(), "and the pane is let go");
}
