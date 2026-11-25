// A session reads its configuration from the environment, so this points
// it at a directory of its own.

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
// arrives in dozens of reads. Taking one per wake-up meant one per drawn
// frame, which is how an application's redraw came to be drawn in
// instalments and to take dozens of frames instead of one.
//
// A frame is what paces the real loop, so the wake-ups here are spaced
// like frames; without that the test reads faster than a window ever
// would and measures nothing.
#[test]
fn a_screenful_of_output_arrives_in_one_wake_up() {
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
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(20);
    loop {
        std::thread::sleep(FRAME);
        let _ = session.pump(0);
        wake_ups += 1;
        if session.focused_text().contains("FIN23") {
            break;
        }
        assert!(
            std::time::Instant::now() < deadline,
            "the output never finished"
        );
    }

    println!("40,000 bytes took {wake_ups} wake-ups");
    assert!(
        wake_ups <= 25,
        "a screenful took {wake_ups} wake-ups, which is a drawn frame each"
    );
}

/// What a drawn frame costs, which is how often the loop gets to read.
const FRAME: std::time::Duration = std::time::Duration::from_millis(16);
