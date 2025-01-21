//! Shared Memory Block



use crate::Result;



/// A typed, shared block of memory.
pub struct Block<T> {
    shm: shared_memory::Shmem,
    ptr: *mut T,
}

impl<T> Block<T> {
    pub fn alloc() -> Result<Self> {
        todo!()
    }

    pub fn open() -> Result<Self> {
        todo!()
    }
}
