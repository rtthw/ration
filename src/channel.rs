//! Shared Memory Channel



use std::{path::Path, sync::atomic::{AtomicIsize, AtomicU8, Ordering}};

use crate::{Error, Result};



pub struct Channel<T: Sized> {
    shm: shared_memory::Shmem,

    flag: *mut AtomicU8,
    base: *mut Option<T>,
    capacity: isize,
    start: isize,
    finish: *mut AtomicIsize,
    count: *mut AtomicIsize,
}

impl<T: Sized> Channel<T> {
    pub fn alloc(path: impl AsRef<Path>, capacity: usize) -> Result<Self> {
        let block_size
            = (std::mem::size_of::<Option<T>>() * capacity) // items
            + std::mem::size_of::<AtomicU8>()               // flag
            + (std::mem::size_of::<AtomicIsize>() * 2);     // finish & count

        let shm = match shared_memory::ShmemConf::new().flink(&path).size(block_size).create() {
            Ok(shmem) => shmem,
            Err(shared_memory::ShmemError::LinkExists) => {
                return Err(Error::BlockAlreadyAllocated);
            }
            Err(e) => { return Err(Error::Shm(e)); }
        };

        unsafe {
            let flag = shm.as_ptr() as *mut AtomicU8;
            let count = flag.offset(1) as *mut AtomicIsize;
            let start = 1;
            let finish = count.offset(1);
            let base = count.offset(2) as *mut Option<T>;
            let capacity = capacity as isize;

            (&*count).store(0, Ordering::SeqCst);
            (&*finish).store(start, Ordering::SeqCst);
            for i in 0..capacity {
                base.offset(i).write(None);
            }

            Ok(Self {
                shm,
                flag,
                base,
                capacity,
                start,
                finish,
                count,
            })
        }
    }

    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        todo!()
    }

    pub fn check(&self) -> bool {
        unsafe { &*self.flag }.load(Ordering::Relaxed) == 1
    }

    pub fn exchange(&self) -> bool {
        let previous_value = match unsafe { &*self.flag }
            .compare_exchange(1, 0, Ordering::Relaxed, Ordering::Relaxed)
        {
            Ok(val) => val,
            Err(val) => val,
        };

        previous_value == 1
    }
}

impl<T> Channel<T> {
    /// Whether the unnderlying shared memory mapping is owned by this channel.
    pub fn owned(&self) -> bool {
        self.shm.is_owner()
    }
}
