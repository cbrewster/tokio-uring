use std::{
    fmt,
    time::{Duration, SystemTime},
};

/// Metadata information about a file.
#[derive(Clone)]
pub struct Metadata(pub(crate) libc::statx);

impl Metadata {
    /// Returns the last access time of this metadata.
    pub fn accessed(&self) -> SystemTime {
        system_time_from_statx_timestamp(self.0.stx_atime).unwrap()
    }

    /// Returns the creation time listed in this metadata.
    pub fn created(&self) -> SystemTime {
        system_time_from_statx_timestamp(self.0.stx_ctime).unwrap()
    }

    /// Returns the last modification time listed in this metadata.
    pub fn modified(&self) -> SystemTime {
        system_time_from_statx_timestamp(self.0.stx_mtime).unwrap()
    }

    /// Returns the file type for this metadata.
    pub fn file_type(&self) -> FileType {
        FileType(self.0.stx_mode as libc::mode_t)
    }

    /// Returns `true` if this metadata is for a directory.
    pub fn is_dir(&self) -> bool {
        self.file_type().is_dir()
    }

    /// Returns `true` if this metadata is for a file.
    pub fn is_file(&self) -> bool {
        self.file_type().is_file()
    }

    /// Returns `true` if this metadata is for a symlink.
    pub fn is_symlink(&self) -> bool {
        self.file_type().is_symlink()
    }

    /// Returns the size of the file, in bytes, this metadata is for.
    pub fn len(&self) -> u64 {
        self.0.stx_size
    }

    /// Returns the permissions of the file this metadata is for.
    pub fn permissions(&self) -> Permissions {
        Permissions(self.0.stx_mode as libc::mode_t)
    }
}

impl fmt::Debug for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Metadata")
            .field("file_type", &self.file_type())
            .field("is_dir", &self.is_dir())
            .field("is_file", &self.is_file())
            .field("permissions", &self.permissions())
            .field("modified", &self.modified())
            .field("accessed", &self.accessed())
            .field("created", &self.created())
            .finish_non_exhaustive()
    }
}

/// A structure representing a type of file with accessors for each file type.
#[derive(Debug)]
pub struct FileType(libc::mode_t);

impl FileType {
    /// Tests whether this file type respresents a directory.
    pub fn is_dir(&self) -> bool {
        self.is(libc::S_IFDIR)
    }

    /// Tests whether this file type respresents a file.
    pub fn is_file(&self) -> bool {
        self.is(libc::S_IFREG)
    }

    /// Tests whether this file type respresents a symlink.
    pub fn is_symlink(&self) -> bool {
        self.is(libc::S_IFLNK)
    }

    fn is(&self, mode: libc::mode_t) -> bool {
        self.0 & libc::S_IFMT == mode
    }
}

/// Respresentation of the various permissions on a file.
#[derive(Debug)]
pub struct Permissions(libc::mode_t);

impl Permissions {
    /// Returns `true` if these permissions descript a readonly (unwritable) file.
    pub fn readonly(&self) -> bool {
        // check if any class (owner, group, others) has write permission
        self.0 & 0o222 == 0
    }

    /// Returns the underlying raw `st_mode` bits that contain the standard Unix permissions for
    /// this file.
    pub fn mode(&self) -> u32 {
        self.0 as u32
    }
}

fn system_time_from_statx_timestamp(ts: libc::statx_timestamp) -> Option<SystemTime> {
    use std::convert::TryInto;
    SystemTime::UNIX_EPOCH.checked_add(Duration::new(ts.tv_sec.try_into().unwrap(), ts.tv_nsec))
}
