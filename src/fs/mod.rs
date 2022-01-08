//! Filesystem manipulation operations.

mod directory;
pub use directory::remove_dir;

mod file;
pub use file::remove_file;
pub use file::File;

mod metadata;
pub use metadata::FileType;
pub use metadata::Metadata;
pub use metadata::Permissions;

mod open_options;
pub use open_options::OpenOptions;
