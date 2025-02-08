//! Unix Implementation



use std::{ffi::c_void, num::NonZeroUsize, os::fd::OwnedFd, path::Path, ptr::NonNull};

use nix::{fcntl::OFlag, sys::{mman::{mmap, munmap, shm_open, shm_unlink, MapFlags, ProtFlags}, stat::Mode}, unistd::ftruncate};

use crate::Error;



/// A raw memory mapping.
// NOTES:
// - See https://github.com/nix-rust/nix/pull/2000 for some more info on `nix` ops.
pub struct Mapping {
    owner: bool,
    fd: OwnedFd,
    uid: String,
    len: usize,
    addr: NonNull<c_void>,
}

impl Drop for Mapping {
    fn drop(&mut self) {
        // Unmap memory.
        if let Err(_e) = unsafe { munmap(self.addr, self.len) } {
            // println!("Failed to `munmap` shared memory: {}", _e);
        };

        // Unlink shm.
        if self.owner {
            if let Err(_e) = shm_unlink(self.uid.as_str()) {
                // println!("Failed to `shm_unlink` shared memory: {}", _e);
            };
        }
    }
}

impl Mapping {
    pub fn create(path: impl AsRef<Path>, size: usize) -> Result<Self, Error> {
        let nz_map_size = NonZeroUsize::new(size)
            .ok_or(Error::MapSizeZero)?;

        let uid = path.as_ref().to_string_lossy().to_string();
        let fd = match shm_open(
            uid.as_str(),
            OFlag::O_CREAT | OFlag::O_EXCL | OFlag::O_RDWR, // Create exclusively, read/write.
            Mode::S_IRUSR | Mode::S_IWUSR, // User read/write permissions.
        ) {
            Ok(value) => value,
            Err(nix::Error::EEXIST) => return Err(Error::MappingExists),
            Err(e) => return Err(Error::MapCreateFailed(e as u32)),
        };

        // Enlarge the memory descriptor file size to the requested map size.
        match ftruncate(&fd, size as _) {
            Ok(_) => {}
            Err(e) => return Err(Error::UnknownOsError(e as u32)),
        };

        let addr = match unsafe {
            mmap(
                None,
                nz_map_size,
                ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
                MapFlags::MAP_SHARED,
                &fd,
                0,
            )
        } {
            Ok(v) => v,
            Err(e) => return Err(Error::MapCreateFailed(e as u32)),
        };

        Ok(Self {
            owner: true,
            fd,
            uid: uid.to_string(),
            len: size,
            addr,
        })
    }
}
