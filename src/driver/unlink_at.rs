use crate::driver::{self, Op};

use std::ffi::CString;
use std::io;
use std::path::Path;

/// Unlink a path relative to the current working directory of the caller's process.
pub(crate) struct Unlink {
    pub(crate) _path: CString,
}

impl Op<Unlink> {
    /// Submit a request to unlink a directory with provided flags.
    pub(crate) fn unlink_dir(path: &Path) -> io::Result<Op<Unlink>> {
        Self::unlink(path, libc::AT_REMOVEDIR)
    }

    /// Submit a request to unlink a file with provided flags.
    pub(crate) fn unlink_file(path: &Path) -> io::Result<Op<Unlink>> {
        Self::unlink(path, 0)
    }

    /// Submit a request to unlink a specifed path with provided flags.
    pub(crate) fn unlink(path: &Path, flags: i32) -> io::Result<Op<Unlink>> {
        use io_uring::{opcode, types};

        let _path = driver::util::cstr(path)?;

        // Get a reference to the memory. The string will be held by the
        // operation state and will not be accessed again until the operation
        // completes.
        let p_ref = _path.as_c_str().as_ptr();

        Op::submit_with(Unlink { _path }, || {
            opcode::UnlinkAt::new(types::Fd(libc::AT_FDCWD), p_ref)
                .flags(flags)
                .build()
        })
    }
}
