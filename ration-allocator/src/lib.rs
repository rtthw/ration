//! Ration shared memory allocator

#![feature(allocator_api)]



use core::{
    alloc::{Layout, AllocError},
    ptr::NonNull,
};



#[derive(Clone)]
pub struct SharedAllocator;

unsafe impl core::alloc::Allocator for SharedAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        todo!()
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        todo!()
    }
}
