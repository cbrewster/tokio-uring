use crate::driver::Op;

use std::io;

use super::SharedFd;

/// Stat a path relative to the current working directory of the caller's process.
pub(crate) struct Stat {
    _fd: SharedFd,

    pub(crate) statx: Box<libc::statx>,
}

impl Op<Stat> {
    pub(crate) fn stat_fd_all(fd: &SharedFd) -> io::Result<Op<Stat>> {
        Self::stat_fd(fd, libc::AT_EMPTY_PATH, libc::STATX_ALL)
    }

    /// Submit a request to stat a fd with provided flags and mask.
    pub(crate) fn stat_fd(fd: &SharedFd, flags: i32, mask: u32) -> io::Result<Op<Stat>> {
        use io_uring::{opcode, types};

        let mut statx = Box::new(unsafe { std::mem::zeroed() });

        let statx_ptr = &mut *statx as *mut _ as *mut types::statx;

        Op::submit_with(
            Stat {
                _fd: fd.clone(),
                statx,
            },
            || {
                opcode::Statx::new(types::Fd(fd.raw_fd()), b"\0".as_ptr().cast(), statx_ptr)
                    .flags(flags)
                    .mask(mask)
                    .build()
            },
        )
    }
}
