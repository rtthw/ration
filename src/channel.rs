//! Shared Memory Channel



use std::{path::Path, sync::atomic::{AtomicIsize, AtomicU8, Ordering}};

use crate::Result;



pub struct Channel<T: Sized> {
    shm: shared_memory::Shmem,

    flag: *mut AtomicU8,
    base: *mut Option<T>,
    capacity: isize,
    start: isize,
    finish: *mut AtomicIsize,
    size: *mut AtomicIsize,
}

impl<T: Sized> Channel<T> {
    pub fn alloc(path: impl AsRef<Path>, capacity: usize) -> Result<Self> {
        todo!()
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
