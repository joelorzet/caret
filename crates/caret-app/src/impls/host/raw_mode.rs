#[allow(unused_imports)]
use crate::prelude::*;

use std::io;

impl RawMode {
    /// Switches stdin to raw mode: no echo, no line buffering, no signal
    /// generation, so every keystroke reaches us untouched.
    pub fn enable() -> io::Result<RawMode> {
        let mut saved: libc::termios = unsafe { core::mem::zeroed() };
        if unsafe { libc::tcgetattr(libc::STDIN_FILENO, &mut saved) } < 0 {
            return Err(io::Error::last_os_error());
        }

        let mut raw = saved;
        unsafe { libc::cfmakeraw(&mut raw) };
        // Return from read as soon as a single byte is available.
        raw.c_cc[libc::VMIN] = 1;
        raw.c_cc[libc::VTIME] = 0;

        if unsafe { libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &raw) } < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(RawMode { saved })
    }
}

impl Drop for RawMode {
    fn drop(&mut self) {
        unsafe { libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &self.saved) };
    }
}
