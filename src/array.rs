//! Shared Memory Array



use std::{path::Path, sync::atomic::{AtomicIsize, AtomicU8, Ordering}};

use crate::{Error, Result};



/// A shared array that can store `capacity` elements of type `T`.
///
/// # Example
/// *In your "parent" process:*
/// ```no_run
/// use ration::Array;
///
/// let mut array: Array<char> = Array::alloc("/tmp/MY_ARRAY", 6).unwrap();
/// array.push_many("ration".chars());
/// ```
/// *In your "child" process:*
/// ```no_run
/// use ration::Array;
///
/// let mut array: Array<char> = Array::open("/tmp/MY_ARRAY").unwrap();
///
/// let mut s = String::new();
/// while let Some(c) = array.pop() {
///     s.push(c);
/// }
/// println!("MY_ARRAY: {}", s); // "ration"
/// ```
// TODO: Some sort of mutable access check.
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
    /// Allocate an array to shared memory identified by the given path, with the given capacity.
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

    /// Open an array in shared memory identified by the given path.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let shm = shared_memory::ShmemConf::new()
            .flink(path)
            .open()
            .map_err(|e| Error::Shm(e))?;

        let metadata_size
            = std::mem::size_of::<AtomicU8>()               // empty_flag
            + (std::mem::size_of::<AtomicIsize>() * 2);     // last & len

        let array_size = shm.len() - metadata_size;
        let slot_size = std::mem::size_of::<Option<T>>();
        let capacity = array_size / slot_size;

        unsafe {
            let empty_flag = shm.as_ptr() as *mut AtomicU8;
            let len = empty_flag.offset(1) as *mut AtomicIsize;
            let first = 1;
            let last = len.offset(1);
            let base = len.offset(2) as *mut Option<T>;
            let capacity = capacity as isize;

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

    /// Returns `true` if the array contains no elements.
    pub fn is_empty(&self) -> bool {
        unsafe { &*self.empty_flag }.load(Ordering::Relaxed) == 0
    }

    /// Returns the number of array slots that are empty.
    pub fn slots_remaining(&self) -> usize {
        (self.capacity - unsafe { &*self.len }.load(Ordering::SeqCst)).unsigned_abs()
    }

    /// Push an element to the back of the array.
    pub fn push(&mut self, element: T) -> bool {
        // Ensure the internal ring buffer isn't full.
        let count = unsafe { &*self.len }.fetch_add(1, Ordering::SeqCst);
        if count >= self.capacity {
            // The buffer is full; give up.
            unsafe { &*self.len }.fetch_sub(1, Ordering::SeqCst);
            return false;
        }

        self.push_unchecked(element);

        // Signal.
        unsafe { &mut *self.empty_flag }.store(1, Ordering::Relaxed);

        true
    }

    /// Push an iterator of elements to the back of the array.
    pub fn push_many(&mut self, elements: impl IntoIterator<Item = T>) {
        let slots_remaining = self.slots_remaining();
        for element in elements.into_iter().take(slots_remaining) {
            let _ = unsafe { &*self.len }.fetch_add(1, Ordering::SeqCst);
            self.push_unchecked(element);
        }

        // Signal.
        unsafe { &mut *self.empty_flag }.store(1, Ordering::Relaxed);
    }

    /// Push an element to the back of the array without checking for overflows, raising the empty
    /// flag, or checking access.
    pub fn push_unchecked(&mut self, element: T) {
        // Get the next available index, wrapping if need be.
        let index = unsafe { &*self.last }.fetch_add(1, Ordering::SeqCst) % self.capacity;
        if index == 0 {
            // Just mod on overflow; the buffer is circular.
            unsafe { &*self.last }.fetch_sub(self.capacity, Ordering::SeqCst);
        }

        // Write the element into the shared memory.
        unsafe {
            self.base.offset(index).write(Some(element));
        }
    }

    /// Push an iterator of elements to the back of the array without checking for overflows,
    /// raising the empty flag, or checking access.
    pub fn push_many_unchecked(&mut self, elements: impl Iterator<Item = T>) {
        for elem in elements {
            self.push_unchecked(elem)
        }
    }

    /// Pop an element from the front of the array.
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

    /// Pop an element from the front of the array without checking for overflows, raising the
    /// empty flag, or checking access.
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
    /// Returns `true` if the underlying shared memory mapping is owned by this array instance.
    pub fn is_owner(&self) -> bool {
        self.shm.is_owner()
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Array<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Array")
            .field("capacity", &self.capacity)
            .field("len", &self.len)
            // .field("elements", self.iter().collect())
            .finish_non_exhaustive()
    }
}



/// # Warning
///
/// This can be wildly unsafe if the array is being mutated while you are iterating over its
/// elements. **Use at your own risk.**
pub struct ArrayIter<'a, T> {
    array: &'a Array<T>,
    index: isize,
    capacity: isize,
    // FIXME: There is probably a better way to stop the iteration than counting up to length.
    count: isize,
    len: isize,
}

impl<'a, T> Iterator for ArrayIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let elem = unsafe { &*self.array.base.offset(self.index) }.as_ref();
        if elem.is_some() {
            self.index = (self.index + 1) % self.capacity;
            self.count += 1;
        }
        if self.count > self.len {
            // NOTE: This is a ring buffer, so the iterator will continue indefinitely if the
            //       array is full without this check.
            None
        } else {
            elem
        }
    }
}

/// # Warning
///
/// This can be wildly unsafe if the array is being mutated while you are iterating over its
/// elements. **Use at your own risk.**
pub struct ArrayIterMut<'a, T> {
    array: &'a mut Array<T>,
    index: isize,
    capacity: isize,
    // FIXME: There is probably a better way to stop the iteration than counting up to length.
    count: isize,
    len: isize,
}

impl<'a, T> Iterator for ArrayIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let elem = unsafe { &mut *self.array.base.offset(self.index) }.as_mut();
        if elem.is_some() {
            self.index = (self.index + 1) % self.capacity;
            self.count += 1;
        }
        if self.count > self.len {
            // NOTE: This is a ring buffer, so the iterator will continue indefinitely if the
            //       array is full without this check.
            None
        } else {
            elem
        }
    }
}

// Iteration methods.
impl<T: Sized> Array<T> {
    /// Iterate over this array's elements.
    ///
    /// # Warning
    ///
    /// This can be wildly unsafe if the array is being mutated while you are iterating over its
    /// elements. **Use at your own risk.**
    pub fn iter(&self) -> ArrayIter<'_, T> {
        ArrayIter {
            array: self,
            index: self.first,
            capacity: self.capacity,
            count: 0,
            len: unsafe { &*self.len }.load(Ordering::Relaxed),
        }
    }

    /// Mutably iterate over this array's elements.
    ///
    /// # Warning
    ///
    /// This can be wildly unsafe if the array is being mutated while you are iterating over its
    /// elements. **Use at your own risk.**
    pub fn iter_mut(&mut self) -> ArrayIterMut<'_, T> {
        let index = self.first.clone();
        let capacity = self.capacity.clone();
        let len = unsafe { &*self.len }.load(Ordering::Relaxed).clone();

        ArrayIterMut {
            array: self,
            index,
            capacity,
            count: 0,
            len,
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_test_1() {
        let mut array_1: Array<char> = Array::alloc("/tmp/TEST_ARRAY_1", 16).unwrap();
        assert!(array_1.is_owner());
        assert!(array_1.is_empty());

        let s = "Something...";

        array_1.push_many(s.chars());

        assert!(!array_1.is_empty());
        assert_eq!(array_1.slots_remaining(), 4);

        {
            let mut ref_array_1: Array<char> = Array::open("/tmp/TEST_ARRAY_1").unwrap();
            assert!(!ref_array_1.is_owner());
            assert!(!ref_array_1.is_empty());
            assert_eq!(array_1.capacity, ref_array_1.capacity);

            let mut ref_s = String::new();
            while let Some(c) = ref_array_1.pop() {
                ref_s.push(c);
            }

            assert_eq!(ref_array_1.slots_remaining(), 16);
            assert_eq!(s.to_string(), ref_s);
        }

        assert!(array_1.is_empty());
    }

    #[test]
    fn array_push_overflow() {
        let mut array: Array<u8> = Array::alloc("/tmp/TEST_ARRAY_OVERFLOW", 8).unwrap();

        let mut stopped_at = 0;
        for i in 0..16 {
            if !array.push(i) {
                stopped_at = i;
                break;
            }
        }

        assert_eq!(stopped_at, 8);
        assert_eq!(array.slots_remaining(), 0);
    }

    #[test]
    fn array_slots_update_correctly() {
        let mut array: Array<u8> = Array::alloc("/tmp/TEST_ARRAY_SLOTSUPDATE", 8).unwrap();

        for i in 0..9 {
            if !array.push(i) {
                assert_eq!(array.slots_remaining(), 0);

                for j in 0..4_u8 {
                    let Some(last_i) = array.pop() else {
                        panic!("array should have filled slots")
                    };
                    assert_eq!(last_i, j);
                    assert_eq!(array.slots_remaining(), (j + 1) as usize);
                }
                for k in (0..4_u8).rev() {
                    assert!(array.push(k));
                    assert_eq!(array.slots_remaining(), k as usize);
                }
            }
        }
    }

    #[test]
    fn array_traverse_full() {
        let mut array = Array::alloc("/tmp/TEST_ARRAY_ITERFULL", 16).unwrap();
        // 16th item is 'j'.
        array.push_many("This is a test just to see if the array iterates correctly.".chars());

        let iter = array.iter();

        assert_eq!(iter.len, 16);

        let mut s = String::new();
        for ch in iter {
            s.push(*ch);
        }

        assert_eq!(&s, "This is a test j");
    }

    #[test]
    fn array_traverse_partial() {
        let mut array = Array::alloc("/tmp/TEST_ARRAY_ITERPARTIAL", 16).unwrap();
        array.push_many("Testing...".chars());

        let iter = array.iter();

        assert_eq!(iter.len, 10);
        assert_eq!(array.slots_remaining(), 6);

        let mut s = String::new();
        for ch in iter {
            s.push(*ch);
        }

        assert_eq!(&s, "Testing...");
    }

    #[test]
    fn array_traverse_mutable() {
        let mut array = Array::alloc("/tmp/TEST_ARRAY_ITERMUT", 16).unwrap();
        array.push_many([1, 2, 3, 4, 5].into_iter());

        for n in array.iter_mut() {
            *n += 1;
        }

        let result = array.iter().copied().collect::<Vec<_>>();
        assert_eq!(result, vec![2, 3, 4, 5, 6]);
    }
}
