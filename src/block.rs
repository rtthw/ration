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

impl<T> Block<T> {
    /// Returns `true` if the underlying shared memory mapping is owned by this block instance.
    pub fn is_owner(&self) -> bool {
        self.shm.is_owner()
    }
}

impl<T> std::ops::Deref for Block<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl<T> std::ops::DerefMut for Block<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    struct TestDatatype {
        field_a: u32,
        field_b: [char; 16],
    }

    #[test]
    fn block_test_1() {
        let mut block: Block<TestDatatype> = Block::alloc("/tmp/TEST_BLOCK_1").unwrap();
        assert!(block.is_owner());

        *block = TestDatatype {
            field_a: 0xffffffff,
            field_b: [
                'T', 'h', 'i', 's', ' ', 'i', 's', ' ', 'w', 'o', 'r', 'k', 'i', 'n', 'g', '.',
            ],
        };

        {
            let mut ref_block: Block<TestDatatype> = Block::open("/tmp/TEST_BLOCK_1").unwrap();
            assert!(!ref_block.is_owner());

            assert_eq!(ref_block.field_a, 0xffffffff);
            assert_eq!(
                ref_block.field_b.iter().collect::<String>(),
                "This is working.".to_string(),
            );

            ref_block.field_a = 0x000000ff;
        }

        assert_eq!(block.field_a, 0x000000ff);
    }

    #[test]
    fn block_responsive_afterward() {
        let mut block: Block<u8> = Block::alloc("/tmp/TEST_BLOCK_RESPAFTER").unwrap();
        let ref_block: Block<u8> = Block::open("/tmp/TEST_BLOCK_RESPAFTER").unwrap();
        *block = 11;
        assert_eq!(*ref_block, 11);
    }

    #[test]
    fn block_multithreading() {
        let mut block: Block<u8> = Block::alloc("/tmp/TEST_BLOCK_MTHREADING").unwrap();
        let handle = std::thread::spawn(move || {
            let ref_block: Block<u8> = Block::open("/tmp/TEST_BLOCK_MTHREADING").unwrap();
            std::thread::sleep(std::time::Duration::from_millis(5));
            *ref_block
        });
        *block = 11;
        assert_eq!(handle.join().unwrap(), 11);
    }
}
