use std::path;
use std::fs;
use std::io;

#[allow(non_camel_case_types)]
type mode_t = u32;

pub struct FileInfo {
    metadata: fs::Metadata,
    mode: mode_t,
    l_mode: mode_t,
}

// Info from http://pubs.opengroup.org/onlinepubs/009604499/basedefs/sys/stat.h.html
// and from https://github.com/rust-lang/libc/blob/cb7f66732175e6171587ed69656b7aae7dd2e6ec/src/unix/bsd/apple/mod.rs
// as well as from https://www.gnu.org/software/libc/manual/html_node/Permission-Bits.html

// TODO: support these?
#[allow(dead_code)]
const S_IFIFO: mode_t = 4096; // FIFO special.
#[allow(dead_code)]
const S_IFCHR: mode_t = 8192; // Character special.
#[allow(dead_code)]
const S_IFBLK: mode_t = 24576; // Block special
#[allow(dead_code)]
const S_IFSOCK: mode_t = 49152; // Socket


const S_IFLNK: mode_t = 40960; // Symbolic link.
const S_IFMT: mode_t = 61440; // Type of file
const S_IRWXU: mode_t = 448; // Read, write, execute/search by owner.
const S_IXUSR: mode_t = 64; // Execute/search permission, owner.
const S_IRWXG: mode_t = 56; // Read, write, execute/search by group.
const S_IXGRP: mode_t = 8; // Execute/search permission, group.
const S_IRWXO: mode_t = 7; // Read, write, execute/search by others.
const S_IXOTH: mode_t = 1; // Execute/search permission, others.

impl FileInfo {

    pub fn new(path: &path::Path) -> Result<FileInfo, io::Error> {
        let metadata = try!(path.metadata());
        let l_metadata = try!(path.symlink_metadata());
        let mode = FileInfo::mode(&metadata);
        let l_mode = FileInfo::mode(&l_metadata);
        return Ok(FileInfo {
            metadata: metadata,
            mode: mode,
            l_mode: l_mode,
        });
    }

    pub fn is_symbolic_link(&self) -> bool {
        (self.l_mode & S_IFMT) & S_IFLNK == S_IFLNK
    }

    pub fn everyone_can_do_everything (&self) -> bool {
        let everything = S_IRWXU | S_IRWXG | S_IRWXO;
        self.mode & everything == everything
    }

    pub fn is_dir(&self) -> bool {
        return self.metadata.is_dir();
    }

    pub fn is_executable(&self) -> bool {
        self.mode & (S_IXGRP | S_IXUSR | S_IXOTH) > 0
    }

    #[cfg(target_os = "macos")]
    fn mode(metadata: &fs::Metadata) -> mode_t {
        use std::os::macos::fs::MetadataExt;
        metadata.st_mode()
    }

    #[cfg(target_os = "linux")]
    fn mode(metadata: &fs::Metadata) -> mode_t {
        use std::os::linux::fs::MetadataExt;
        metadata.st_mode()
    }

    #[cfg(all(not(target_os = "macos"), not(target_os = "linux"), unix))]
    fn mode(metadata: &fs::Metadata) -> mode_t {
        use std::os::unix::fs::MetadataExt;
        metadata.mode()
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux", unix)))]
    pub fn mode(metadata: &fs::Metadata) -> mode_t {
        0
    }
}
