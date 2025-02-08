//! Unix Implementation



use std::{ffi::c_void, os::fd::RawFd, ptr::NonNull};

use nix::sys::mman::{munmap, shm_unlink};



/// A raw memory mapping.
///
/// # Safety
/// - It will close itself when dropped.
// NOTES:
// - See https://github.com/nix-rust/nix/pull/2000 for some more info on `nix` ops.
pub struct Mapping {
    owner: bool,
    fd: RawFd,
    uid: String,
    map_size: usize,
    addr: NonNull<c_void>,
}

impl Drop for Mapping {
    fn drop(&mut self) {
        // Unmap memory.
        if let Err(_e) = unsafe { munmap(self.addr, self.map_size) } {
            // println!("Failed to `munmap` shared memory: {}", _e);
        };

        // Unlink shm.
        if self.fd != 0 {
            if self.owner {
                if let Err(_e) = shm_unlink(self.uid.as_str()) {
                    // println!("Failed to `shm_unlink` shared memory: {}", _e);
                };
            }

            if let Err(_e) = nix::unistd::close(self.fd) {
                // println!("Failed to `close` shared memory file: {}", _e);
            };
        }
    }
}
