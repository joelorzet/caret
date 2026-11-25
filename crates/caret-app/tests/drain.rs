//! How much of a pane's output one wake-up takes.
//!
//! A measurement rather than a test: what it reports depends on how
//! promptly the machine schedules the pane again, which differs by
//! platform and with load, so there is no number here that means the same
//! thing everywhere. Run it with
//!
//! ```sh
//! cargo test --release -p caret-app --test drain -- --ignored --nocapture
//! ```
//!
//! What the fix it guards actually changed is counted in drawn frames, not
//! in wake-ups, and that needs a window: set `CARET_TRACE` and read the
//! `frame:` lines, as CONTRIBUTING describes.
//!
//! A session reads its configuration from the environment, so this points
//! it at a directory of its own.

use caret_app::types::key_press::KeyPress;
use caret_app::Session;

fn session() -> Session {
    let root = std::env::temp_dir().join(format!("caret-drain-{}", std::process::id()));
    std::fs::create_dir_all(&root).expect("a config directory");
    std::env::set_var("CARET_CONFIG_DIR", &root);
    Session::new().expect("a session")
}

/// Types a command into the focused pane and runs it.
fn run(session: &mut Session, line: &str) {
    for ch in line.chars() {
        let _ = session.handle_key(KeyPress {
            binding: caret_config::KeyBinding::ch(ch),
            bytes: ch.to_string().into_bytes(),
        });
    }
    let _ = session.handle_key(KeyPress {
        binding: caret_config::KeyBinding::parse("enter").expect("enter"),
        bytes: vec![b'\r'],
    });
}

// A pty hands over a kilobyte or two at a time, so a screenful of output
// arrives in dozens of reads. Taking one of those per wake-up meant one
// per drawn frame: an application's redraw was painted in instalments and
// cost a frame for each of them.
#[test]
#[ignore = "a measurement, not a test"]
fn how_much_of_a_screenful_one_wake_up_takes() {
    let mut session = session();
    for _ in 0..40 {
        let _ = session.pump(10);
    }

    // A file written out in one go, which is how an application draws a
    // screen: one write, not a character at a time. The sentinel is
    // assembled by the shell, so the command line echoed back cannot match
    // it and only the output can.
    let big = std::env::temp_dir().join(format!("caret-drain-{}.txt", std::process::id()));
    std::fs::write(&big, "y".repeat(40_000)).expect("a screenful");
    run(
        &mut session,
        &format!("cat {}; echo FIN$((20+3))", big.display()),
    );

    let mut wake_ups = 0;
    let started = std::time::Instant::now();
    loop {
        // Spaced like the frames that pace the real loop; without this it
        // reads faster than any window would and measures nothing.
        std::thread::sleep(std::time::Duration::from_millis(16));
        let _ = session.pump(0);
        wake_ups += 1;
        if session.focused_text().contains("FIN23") {
            break;
        }
        assert!(
            started.elapsed() < std::time::Duration::from_secs(20),
            "the output never finished"
        );
    }

    println!(
        "40,000 bytes took {wake_ups} wake-ups in {:.0}ms, {} bytes a wake-up",
        started.elapsed().as_secs_f64() * 1000.0,
        session.bytes_in() / wake_ups
    );
}
