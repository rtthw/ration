//! Shared Memory Block



use std::path::Path;

use crate::{Error, Result};



/// A typed, shared block of memory.
pub struct Block<T: Sized> {
    shm: shared_memory::Shmem,
    ptr: *mut T,
}

impl<T: Sized> Block<T> {
    /// Allocate a new shared block of memory at the given path, and of the given type.
    pub fn alloc(path: impl AsRef<Path>) -> Result<Self> {
        let size = std::mem::size_of::<T>();
        let shm = match shared_memory::ShmemConf::new().flink(&path).size(size).create() {
            Ok(shmem) => shmem,
            Err(shared_memory::ShmemError::LinkExists) => {
                return Err(Error::BlockAlreadyAllocated);
            }
            Err(e) => { return Err(Error::Shm(e)); }
        };

        let ptr = shm.as_ptr() as *mut T;

        Ok(Self {
            shm,
            ptr,
        })
    }

    /// Open a shared block of memory identified by the given path and type.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let shm = shared_memory::ShmemConf::new()
            .flink(path)
            .open()
            .map_err(|e| Error::Shm(e))?;

        // Check if the expected type's size matches the allocated block's size.
        let size = std::mem::size_of::<T>();
        if shm.len() != size {
            return Err(Error::InvalidBlockSize);
        }

        let ptr = shm.as_ptr() as *mut T;

        Ok(Self {
            shm,
            ptr,
        })
    }
}
