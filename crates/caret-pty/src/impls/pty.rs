#[allow(unused_imports)]
use crate::prelude::*;

use std::ffi::CStr;
use std::io::{self, Read, Write};
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd, RawFd};
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};

impl Pty {
    /// Allocates a pseudoterminal and starts `spec.program` on the far end.
    ///
    /// The child gets its own session and makes the slave its controlling
    /// terminal, which is what lets job control, `^C` and `$LINES` work.
    pub fn spawn(spec: &PtySpec) -> io::Result<Pty> {
        let master = unsafe { libc::posix_openpt(libc::O_RDWR | libc::O_NOCTTY) };
        let master = unsafe { OwnedFd::from_raw_fd(check(master)?) };
        check(unsafe { libc::grantpt(master.as_raw_fd()) })?;
        check(unsafe { libc::unlockpt(master.as_raw_fd()) })?;

        let name = unsafe { libc::ptsname(master.as_raw_fd()) };
        if name.is_null() {
            return Err(io::Error::last_os_error());
        }
        let slave_path = unsafe { CStr::from_ptr(name) }.to_owned();

        let slave = unsafe { libc::open(slave_path.as_ptr(), libc::O_RDWR | libc::O_NOCTTY) };
        let slave = unsafe { OwnedFd::from_raw_fd(check(slave)?) };

        set_winsize(master.as_raw_fd(), spec.cols, spec.rows)?;

        let mut command = Command::new(&spec.program);
        if let Some(name) = &spec.arg0 {
            command.arg0(name);
        }
        command
            .args(&spec.args)
            .stdin(Stdio::from(slave.try_clone()?))
            .stdout(Stdio::from(slave.try_clone()?))
            .stderr(Stdio::from(slave.try_clone()?))
            .env("TERM", "xterm-256color")
            .env("COLORTERM", "truecolor");
        if let Some(dir) = &spec.cwd {
            command.current_dir(dir);
        }
        for (key, value) in &spec.env {
            command.env(key, value);
        }

        let slave_fd = slave.as_raw_fd();
        unsafe {
            command.pre_exec(move || {
                // A new session detaches from our controlling terminal so the
                // slave can become the child's.
                check(libc::setsid())?;
                check(libc::ioctl(slave_fd, libc::TIOCSCTTY as _, 0))?;
                Ok(())
            });
        }

        let child = command.spawn()?;
        // Only the child needs the slave; holding it would stop us ever
        // seeing EOF when the child exits.
        drop(slave);
        set_nonblocking(master.as_raw_fd())?;

        Ok(Pty {
            master,
            child,
            exited: None,
        })
    }

    pub fn master(&self) -> RawFd {
        self.master.as_raw_fd()
    }

    pub fn resize(&self, cols: u16, rows: u16) -> io::Result<()> {
        set_winsize(self.master.as_raw_fd(), cols, rows)
    }

    pub fn write(&mut self, bytes: &[u8]) -> io::Result<()> {
        let mut file = unsafe { borrowed_file(self.master.as_raw_fd()) };
        let result = file.write_all(bytes).and_then(|()| file.flush());
        core::mem::forget(file);
        result
    }

    /// Reads whatever is buffered into `buffer`, returning the byte count.
    ///
    /// `Ok(0)` means the child closed the terminal. `WouldBlock` is reported
    /// as `Ok(0)` only after `poll` said the fd was ready, so the caller
    /// cannot mistake a drained fd for an exit.
    pub(crate) fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        let mut file = unsafe { borrowed_file(self.master.as_raw_fd()) };
        let result = file.read(buffer);
        core::mem::forget(file);
        result
    }

    /// Reaps the child without blocking, caching the status once it exits.
    pub fn exit_status(&mut self) -> Option<i32> {
        if let Some(status) = self.exited {
            return Some(status);
        }
        match self.child.try_wait() {
            Ok(Some(status)) => {
                let code = status.code().unwrap_or(-1);
                self.exited = Some(code);
                Some(code)
            }
            Ok(None) => None,
            Err(_) => {
                self.exited = Some(-1);
                Some(-1)
            }
        }
    }

    /// Where the child is working, asked of the operating system.
    ///
    /// A shell only tells its terminal where it is if it has been set up to,
    /// with OSC 7, and most are not. Asking the kernel instead means a new
    /// pane opens where you were working whatever shell you use and whatever
    /// is in your profile.
    pub fn working_dir(&self) -> Option<std::path::PathBuf> {
        working_dir_of(self.child.id() as i32)
    }

    /// Asks the child's process group to terminate.
    pub fn signal(&self, signal: i32) {
        let pid = self.child.id() as libc::pid_t;
        // Negating the pid targets the whole process group, so a shell's
        // children go down with it.
        unsafe { libc::kill(-pid, signal) };
    }
}

impl Drop for Pty {
    /// Hangs the child up without ever blocking.
    ///
    /// Closing a pane must not be able to wedge the window, so this escalates
    /// on a timer and then gives up: SIGHUP (which a shell is entitled to
    /// trap), then SIGKILL, then abandon it. An abandoned child is already
    /// dead or unkillable, and either way the OS reparents it on exit, which is far
    /// better than an unbounded `wait` in a destructor.
    fn drop(&mut self) {
        if self.exited.is_some() {
            return;
        }

        self.signal(libc::SIGHUP);
        if self.wait_briefly(HANGUP_GRACE) {
            return;
        }

        self.signal(libc::SIGKILL);
        self.wait_briefly(KILL_GRACE);
    }
}

impl Pty {
    /// Polls for the child's exit for up to `limit`. Returns whether it went.
    fn wait_briefly(&mut self, limit: std::time::Duration) -> bool {
        let deadline = std::time::Instant::now() + limit;
        loop {
            match self.child.try_wait() {
                Ok(Some(_)) | Err(_) => return true,
                Ok(None) => {}
            }
            if std::time::Instant::now() >= deadline {
                return false;
            }
            std::thread::sleep(POLL_STEP);
        }
    }
}

const HANGUP_GRACE: std::time::Duration = std::time::Duration::from_millis(150);
const KILL_GRACE: std::time::Duration = std::time::Duration::from_millis(150);
const POLL_STEP: std::time::Duration = std::time::Duration::from_millis(5);

/// Wraps a raw fd in a `File` for its `Read`/`Write` impls without taking
/// ownership. Every caller `forget`s the result so the fd is not closed.
unsafe fn borrowed_file(fd: RawFd) -> std::fs::File {
    use std::os::fd::FromRawFd;
    std::fs::File::from_raw_fd(fd)
}

#[cfg(target_os = "macos")]
fn working_dir_of(pid: i32) -> Option<std::path::PathBuf> {
    // `proc_pidinfo` fills the struct only when it returns its full size.
    let mut info: libc::proc_vnodepathinfo = unsafe { std::mem::zeroed() };
    let size = std::mem::size_of::<libc::proc_vnodepathinfo>() as libc::c_int;
    let filled = unsafe {
        libc::proc_pidinfo(
            pid,
            libc::PROC_PIDVNODEPATHINFO,
            0,
            &mut info as *mut _ as *mut libc::c_void,
            size,
        )
    };
    if filled != size {
        return None;
    }

    let path = unsafe { std::ffi::CStr::from_ptr(info.pvi_cdir.vip_path.as_ptr().cast()) };
    let path = std::path::PathBuf::from(path.to_string_lossy().into_owned());
    path.is_dir().then_some(path)
}

#[cfg(target_os = "linux")]
fn working_dir_of(pid: i32) -> Option<std::path::PathBuf> {
    let path = std::fs::read_link(format!("/proc/{pid}/cwd")).ok()?;
    path.is_dir().then_some(path)
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn working_dir_of(_pid: i32) -> Option<std::path::PathBuf> {
    None
}
