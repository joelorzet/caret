use caret_pty::{PtyEvent, PtyId, PtyPool, PtySpec};
use std::time::{Duration, Instant};

fn read_for(pool: &mut PtyPool, limit: Duration) -> String {
    let deadline = Instant::now() + limit;
    let mut text = String::new();
    let mut events = Vec::new();
    let mut ready = Vec::new();
    while Instant::now() < deadline {
        events.clear();
        pool.poll(50, &[], &mut events, &mut ready).unwrap();
        for event in &events {
            if let PtyEvent::Output { bytes, .. } = event {
                text.push_str(&String::from_utf8_lossy(bytes));
            }
        }
        if text.contains('\n') {
            break;
        }
    }
    text
}

fn spec(arg0: Option<&str>) -> PtySpec {
    PtySpec {
        program: "/bin/sh".into(),
        args: vec!["-c".into(), "echo $0".into()],
        arg0: arg0.map(str::to_string),
        cwd: None,
        env: Vec::new(),
        cols: 40,
        rows: 10,
    }
}

// A shell reads its login profile only when its own name begins with a
// dash, and that profile is where most of a PATH comes from. Without this
// a pane cannot find anything Homebrew installed.
#[test]
fn a_shell_can_be_told_it_is_a_login_shell() {
    let mut pool = PtyPool::new();
    pool.spawn(PtyId(1), &spec(Some("-sh"))).expect("spawn");
    assert!(read_for(&mut pool, Duration::from_secs(5)).contains("-sh"));
}

#[test]
fn without_a_name_the_program_keeps_its_own() {
    let mut pool = PtyPool::new();
    pool.spawn(PtyId(2), &spec(None)).expect("spawn");
    let text = read_for(&mut pool, Duration::from_secs(5));
    assert!(text.contains("/bin/sh"), "{text:?}");
    assert!(!text.contains("-sh"), "{text:?}");
}
