#[allow(unused_imports)]
use crate::prelude::*;

use std::sync::atomic::{AtomicBool, Ordering};

static RESIZED: AtomicBool = AtomicBool::new(false);
static ENDING: AtomicBool = AtomicBool::new(false);

extern "C" fn on_winch(_signal: libc::c_int) {
    RESIZED.store(true, Ordering::Relaxed);
}

extern "C" fn on_end(_signal: libc::c_int) {
    ENDING.store(true, Ordering::Relaxed);
}

/// Watches for terminal resizes.
///
/// The handler only sets a flag; the work happens in the event loop, because
/// almost nothing is safe to do inside a signal handler.
pub fn watch_resize() {
    unsafe {
        libc::signal(libc::SIGWINCH, on_winch as *const () as libc::sighandler_t);
        // Writing to a pty whose child has gone would otherwise kill us.
        libc::signal(libc::SIGPIPE, libc::SIG_IGN);
    }
}

pub fn take_resize() -> bool {
    RESIZED.swap(false, Ordering::Relaxed)
}

/// Watches for the signals that mean "stop", so the panes can be hung up.
///
/// Without this a `kill`, a `pkill` or a logout ends the process where it
/// stands: nothing is dropped, so no child is ever sent a hangup and every
/// shell, editor and agent is left running headless on a pty nothing is
/// reading. SIGKILL cannot be caught, and nothing can be done about that.
pub fn watch_termination() {
    unsafe {
        for signal in [libc::SIGTERM, libc::SIGINT, libc::SIGHUP, libc::SIGQUIT] {
            libc::signal(signal, on_end as *const () as libc::sighandler_t);
        }
    }
}

/// Whether a stop has been asked for.
pub fn ending() -> bool {
    ENDING.load(Ordering::Relaxed)
}
