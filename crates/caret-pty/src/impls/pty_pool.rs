#[allow(unused_imports)]
use crate::prelude::*;

use std::io;
use std::os::fd::RawFd;

/// Bytes read from one ready fd per wake-up.
const READ_CHUNK: usize = 65_536;

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
            match pty.read(&mut self.scratch) {
                Ok(0) => hung_up.push(id),
                Ok(n) => out.push(PtyEvent::Output {
                    id,
                    bytes: self.scratch[..n].to_vec(),
                }),
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {}
                Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
                // A closed pty reports EIO rather than EOF on some platforms.
                Err(_) => hung_up.push(id),
            }
        }

        for id in hung_up {
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
            self.close(id);
            out.push(PtyEvent::Exited { id, status });
        }
    }
}
