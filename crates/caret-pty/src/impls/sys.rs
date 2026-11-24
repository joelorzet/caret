#[allow(unused_imports)]
use crate::prelude::*;

use std::io;
use std::os::fd::RawFd;

pub(crate) fn check(result: libc::c_int) -> io::Result<libc::c_int> {
    if result < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(result)
    }
}

pub(crate) fn set_winsize(fd: RawFd, cols: u16, rows: u16) -> io::Result<()> {
    let size = libc::winsize {
        ws_row: rows.max(1),
        ws_col: cols.max(1),
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    check(unsafe { libc::ioctl(fd, libc::TIOCSWINSZ as _, &size) })?;
    Ok(())
}

/// Reads must never block the event loop, so the master is non-blocking and a
/// drained fd reports `WouldBlock` instead of parking the thread.
pub(crate) fn set_nonblocking(fd: RawFd) -> io::Result<()> {
    let flags = check(unsafe { libc::fcntl(fd, libc::F_GETFL) })?;
    check(unsafe { libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK) })?;
    Ok(())
}

/// The terminal size of this process's own controlling terminal.
pub fn terminal_size() -> Option<(u16, u16)> {
    let mut size: libc::winsize = unsafe { core::mem::zeroed() };
    let ok = unsafe { libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ as _, &mut size) };
    if ok < 0 || size.ws_col == 0 {
        return None;
    }
    Some((size.ws_col, size.ws_row))
}
