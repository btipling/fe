use std::path;
use std::fs;
use std::io;
#[cfg(target_os = "macos")]
use std::os::macos::fs::MetadataExt;
#[cfg(target_os = "unix")]
use std::os::unix::fs::MetadataExt;
#[cfg(target_os = "linux")]
use std::os::linux::fs::MetadataExt;

pub struct FileInfo {
    metadata: fs::Metadata,
}

impl FileInfo {

    pub fn new(path: &path::Path) -> Result<FileInfo, io::Error> {
        let metadata = try!(path.metadata());
        return Ok(FileInfo {
            metadata: metadata
        });
    }

    pub fn is_dir(&self) -> bool {
        return self.metadata.is_dir();
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    pub fn is_executable(&self) -> bool {
        self.metadata.st_mode() & 0o111 > 0
    }

    #[cfg(any(target_os = "unix"))]
    pub fn is_executable(&self) -> bool {
        self.metadata.mode() & 0o111 > 0
    }

    #[cfg(not(any(target_os = "macos", target_os = "unix", target_os = "linux")))]
    pub fn is_executable(&self) -> bool {
        false
    }

}