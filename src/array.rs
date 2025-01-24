//! Shared Memory Array



use std::{path::Path, sync::atomic::{AtomicIsize, AtomicU8, Ordering}};

use crate::{Error, Result};



pub struct Array<T: Sized> {
    shm: shared_memory::Shmem,

    empty_flag: *mut AtomicU8,
    base: *mut Option<T>,
    capacity: isize,
    first: isize,
    last: *mut AtomicIsize,
    len: *mut AtomicIsize,
}

impl<T: Sized> Array<T> {
    pub fn alloc(path: impl AsRef<Path>, capacity: usize) -> Result<Self> {
        let block_size
            = (std::mem::size_of::<Option<T>>() * capacity) // elements
            + std::mem::size_of::<AtomicU8>()               // empty_flag
            + (std::mem::size_of::<AtomicIsize>() * 2);     // last & len

        let shm = match shared_memory::ShmemConf::new().flink(&path).size(block_size).create() {
            Ok(shmem) => shmem,
            Err(shared_memory::ShmemError::LinkExists) => {
                return Err(Error::BlockAlreadyAllocated);
            }
            Err(e) => { return Err(Error::Shm(e)); }
        };

        unsafe {
            let empty_flag = shm.as_ptr() as *mut AtomicU8;
            let len = empty_flag.offset(1) as *mut AtomicIsize;
            let first = 1;
            let last = len.offset(1);
            let base = len.offset(2) as *mut Option<T>;
            let capacity = capacity as isize;

            (&*len).store(0, Ordering::SeqCst);
            (&*last).store(first, Ordering::SeqCst);
            for i in 0..capacity {
                base.offset(i).write(None);
            }

            Ok(Self {
                shm,
                empty_flag,
                base,
                capacity,
                first,
                last,
                len,
            })
        }
    }

    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        todo!()
    }

    /// Returns `true` if the channel contains no elements.
    pub fn is_empty(&self) -> bool {
        unsafe { &*self.empty_flag }.load(Ordering::Relaxed) == 0
    }

    // pub fn exchange(&self) -> bool {
    //     let previous_value = match unsafe { &*self.empty_flag }
    //         .compare_exchange(1, 0, Ordering::Relaxed, Ordering::Relaxed)
    //     {
    //         Ok(val) => val,
    //         Err(val) => val,
    //     };

    //     previous_value == 1
    // }

    // TODO: `push_many` method that accepts an iterator of items, more efficient.
    pub fn push(&mut self, element: T) -> bool {
        // Ensure the internal ring buffer isn't full.
        let count = unsafe { &*self.len }.fetch_add(1, Ordering::SeqCst);
        if count >= self.capacity {
            // The buffer is full; give up.
            unsafe { &*self.len }.fetch_sub(1, Ordering::SeqCst);
            return false;
        }

        self.push_unchecked(element);

        true
    }

    // NOTE: This method does NOT check for overflows.
    //       It is up to you to ensure there is enough space in the channel.
    pub fn push_unchecked(&mut self, item: T) {
        // Get the next available index, wrapping if need be.
        let index = unsafe { &*self.last }.fetch_add(1, Ordering::SeqCst) % self.capacity;
        if index == 0 {
            // Just mod on overflow; the buffer is circular.
            unsafe { &*self.last }.fetch_sub(self.capacity, Ordering::SeqCst);
        }

        // Write the element into the shared memory.
        unsafe {
            self.base.offset(index).write(Some(item));
        }

        // Signal.
        unsafe { &mut *self.empty_flag }.store(1, Ordering::Relaxed);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let result = unsafe { &mut *self.base.offset(self.first) }.take();
        if !result.is_none() {
            self.first = (self.first + 1) % self.capacity;
            unsafe { &*self.len }.fetch_sub(1, Ordering::SeqCst);
        } else {
            // Signal.
            unsafe { &mut *self.empty_flag }.store(0, Ordering::Relaxed);
        }

        result
    }

    // NOTE: This method does NOT check the empty flag, nor does it clear it.
    pub fn pop_unchecked(&mut self) -> Option<T> {
        let result = unsafe { &mut *self.base.offset(self.first) }.take();
        if !result.is_none() {
            self.first = (self.first + 1) % self.capacity;
            unsafe { &*self.len }.fetch_sub(1, Ordering::SeqCst);
        }

        result
    }
}

impl<T> Array<T> {
    /// Returns `true` if the underlying shared memory mapping is owned by this channel instance.
    pub fn is_owner(&self) -> bool {
        self.shm.is_owner()
    }
}
