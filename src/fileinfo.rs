use std::path;
use std::fs;
use std::io;

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

    #[cfg(target_os = "macos")]
    pub fn is_executable(&self) -> bool {
        use std::os::macos::fs::MetadataExt;
        self.metadata.st_mode() & 0o111 > 0
    }

    #[cfg(target_os = "linux")]
    pub fn is_executable(&self) -> bool {
        use std::os::linux::fs::MetadataExt;
        self.metadata.st_mode() & 0o111 > 0
    }

    #[cfg(all(not(target_os = "macos"), unix))]
    pub fn is_executable(&self) -> bool {
        use std::os::unix::fs::MetadataExt;
        self.metadata.mode() & 0o111 > 0
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux", unix)))]
    pub fn is_executable(&self) -> bool {
        false
    }
}