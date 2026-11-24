#[allow(unused_imports)]
use crate::prelude::*;

use std::os::fd::OwnedFd;
use std::process::Child;

/// One pseudoterminal and the process group on the far end of it.
pub struct Pty {
    pub(crate) master: OwnedFd,
    pub(crate) child: Child,
    pub(crate) exited: Option<i32>,
}
