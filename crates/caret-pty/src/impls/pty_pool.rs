#[allow(unused_imports)]
use crate::prelude::*;

use std::io;
use std::os::fd::RawFd;

/// Bytes asked for in one read.
const READ_CHUNK: usize = 65_536;

/// How much of one pane's output a single wake-up will take.
///
/// A pty hands back about a kilobyte at a time however large a buffer it
/// is given, so a full-screen redraw arrives in dozens of reads. Stopping
/// after the first would hand the caller a fragment of the application's
/// new frame, and drawing that paints the new screen half applied over the
/// old one: the tearing you see when a table redraws. Everything ready is
/// taken instead, so a frame shows one state of the screen or the next.
///
/// The cap is what keeps a pane that never stops printing from holding up
/// the keyboard: at that point the reading stops and the window gets its
/// turn, and the rest is waiting at the next wake-up.
const READ_BUDGET: usize = 256 * 1024;

/// How many chunks a pty may hand over on its way out.
///
/// A bound rather than a guess: whatever is left in the buffer is finite
/// because the child is gone, and this stops a pathological case from
/// holding the loop while the window waits to redraw.
const DRAIN_LIMIT: usize = 64;

impl PtyPool {
    pub fn new() -> PtyPool {
        PtyPool::default()
    }

    pub fn len(&self) -> usize {
        self.ptys.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ptys.is_empty()
    }

    pub fn ids(&self) -> Vec<PtyId> {
        self.ptys.iter().map(|(id, _)| *id).collect()
    }

    pub fn contains(&self, id: PtyId) -> bool {
        self.ptys.iter().any(|(other, _)| *other == id)
    }

    pub fn spawn(&mut self, id: PtyId, spec: &PtySpec) -> io::Result<()> {
        let pty = Pty::spawn(spec)?;
        self.ptys.retain(|(other, _)| *other != id);
        self.ptys.push((id, pty));
        Ok(())
    }

    fn get_mut(&mut self, id: PtyId) -> Option<&mut Pty> {
        self.ptys
            .iter_mut()
            .find(|(other, _)| *other == id)
            .map(|(_, pty)| pty)
    }

    pub fn write(&mut self, id: PtyId, bytes: &[u8]) -> io::Result<()> {
        match self.get_mut(id) {
            Some(pty) => pty.write(bytes),
            None => Ok(()),
        }
    }

    pub fn resize(&mut self, id: PtyId, cols: u16, rows: u16) -> io::Result<()> {
        match self.get_mut(id) {
            Some(pty) => pty.resize(cols, rows),
            None => Ok(()),
        }
    }

    /// Where a pane's child is working.
    pub fn working_dir(&self, id: PtyId) -> Option<std::path::PathBuf> {
        self.ptys
            .iter()
            .find(|(other, _)| *other == id)
            .and_then(|(_, pty)| pty.working_dir())
    }

    /// Closes a pty, hanging up its child.
    pub fn close(&mut self, id: PtyId) {
        self.ptys.retain(|(other, _)| *other != id);
    }

    /// Waits up to `timeout_ms` for output from any pane.
    ///
    /// One `poll` covers every master fd, so cost scales with a single syscall
    /// rather than with the number of panes. A negative timeout waits forever.
    /// `watch` holds extra descriptors to wait on alongside the ptys, such as
    /// the application's own stdin; those that became readable are reported in
    /// `ready`. Keeping them in the same `poll` is what allows one loop to
    /// serve input and every pane at once.
    pub fn poll(
        &mut self,
        timeout_ms: i32,
        watch: &[RawFd],
        out: &mut Vec<PtyEvent>,
        ready: &mut Vec<RawFd>,
    ) -> io::Result<()> {
        ready.clear();
        self.collect_exits(out);
        if self.ptys.is_empty() && watch.is_empty() {
            return Ok(());
        }

        let mut fds: Vec<libc::pollfd> = self
            .ptys
            .iter()
            .map(|(_, pty)| libc::pollfd {
                fd: pty.master(),
                events: libc::POLLIN,
                revents: 0,
            })
            .collect();
        let pty_count = fds.len();
        fds.extend(watch.iter().map(|fd| libc::pollfd {
            fd: *fd,
            events: libc::POLLIN,
            revents: 0,
        }));

        let ready_count = unsafe { libc::poll(fds.as_mut_ptr(), fds.len() as _, timeout_ms) };
        if ready_count < 0 {
            let err = io::Error::last_os_error();
            // A signal arriving mid-poll is normal; the caller will come back.
            if err.kind() == io::ErrorKind::Interrupted {
                return Ok(());
            }
            return Err(err);
        }
        if ready_count == 0 {
            return Ok(());
        }

        const READY: libc::c_short = libc::POLLIN | libc::POLLHUP | libc::POLLERR;

        for (fd, polled) in watch.iter().zip(&fds[pty_count..]) {
            if polled.revents & READY != 0 {
                ready.push(*fd);
            }
        }

        let flagged: Vec<usize> = fds[..pty_count]
            .iter()
            .enumerate()
            .filter(|(_, fd)| fd.revents & READY != 0)
            .map(|(i, _)| i)
            .collect();

        self.scratch.resize(READ_CHUNK, 0);
        let mut hung_up = Vec::new();

        for index in flagged {
            let (id, pty) = &mut self.ptys[index];
            let id = *id;
            let mut taken: Vec<u8> = Vec::new();
            loop {
                match pty.read(&mut self.scratch) {
                    Ok(0) => {
                        hung_up.push(id);
                        break;
                    }
                    Ok(n) => {
                        taken.extend_from_slice(&self.scratch[..n]);
                        if taken.len() >= READ_BUDGET {
                            break;
                        }
                    }
                    // Nothing more waiting, which is the usual way out.
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,
                    Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                    // A closed pty reports EIO rather than EOF on some
                    // platforms.
                    Err(_) => {
                        hung_up.push(id);
                        break;
                    }
                }
            }
            if !taken.is_empty() {
                out.push(PtyEvent::Output { id, bytes: taken });
            }
        }

        for id in hung_up {
            self.drain(id, out);
            let status = self
                .get_mut(id)
                .and_then(|pty| pty.exit_status())
                .unwrap_or(0);
            self.close(id);
            out.push(PtyEvent::Exited { id, status });
        }

        Ok(())
    }

    /// Notices children that exited without closing the terminal.
    fn collect_exits(&mut self, out: &mut Vec<PtyEvent>) {
        let mut done = Vec::new();
        for (id, pty) in self.ptys.iter_mut() {
            if let Some(status) = pty.exit_status() {
                done.push((*id, status));
            }
        }
        for (id, status) in done {
            self.drain(id, out);
            self.close(id);
            out.push(PtyEvent::Exited { id, status });
        }
    }

    /// Hands over whatever a pty still holds, before it is let go.
    ///
    /// A short command writes and exits between two wake-ups, so the first
    /// thing the pool learns about it is that it is dead. Closing on the
    /// strength of that alone throws away the last thing it said, which for
    /// anything that finishes in one go is everything it said.
    fn drain(&mut self, id: PtyId, out: &mut Vec<PtyEvent>) {
        let mut buf = std::mem::take(&mut self.scratch);
        buf.resize(READ_CHUNK, 0);
        for _ in 0..DRAIN_LIMIT {
            let Some(pty) = self.get_mut(id) else {
                break;
            };
            match pty.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => out.push(PtyEvent::Output {
                    id,
                    bytes: buf[..n].to_vec(),
                }),
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                // WouldBlock means nothing more is waiting, and a closed pty
                // reports EIO: either way there is no more to be had.
                Err(_) => break,
            }
        }
        self.scratch = buf;
    }
}
