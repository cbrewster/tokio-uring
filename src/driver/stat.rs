use libc::c_char;

use crate::driver::Op;

use std::{io, mem::MaybeUninit};

use super::SharedFd;

/// Stat a path relative to the current working directory of the caller's process.
pub(crate) struct Stat {
    pub(crate) _stats: Box<MaybeUninit<libc::statx>>,
}

impl Op<Stat> {
    pub(crate) fn stat_fd_all(fd: &SharedFd) -> io::Result<Op<Stat>> {
        Self::stat_fd(fd, libc::AT_EMPTY_PATH, libc::STATX_ALL)
    }

    /// Submit a request to stat a fd with provided flags and mask.
    pub(crate) fn stat_fd(fd: &SharedFd, flags: i32, mask: u32) -> io::Result<Op<Stat>> {
        use io_uring::{opcode, types};

        let mut stat = Stat {
            _stats: Box::new(MaybeUninit::zeroed()),
        };

        // Get a reference to the memory. The string will be held by the
        // operation state and will not be accessed again until the operation
        // completes.
        let s_ref = stat._stats.as_mut_ptr();

        Op::submit_with(stat, || {
            opcode::Statx::new(
                types::Fd(fd.raw_fd()),
                b"\0".as_ptr().cast(),
                s_ref as *mut types::statx,
            )
            .flags(flags)
            .mask(mask)
            .build()
        })
    }
}
