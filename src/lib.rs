//! Ration
//!
//! A simple, easy-to-use, shared memory library.
//!
//! # Getting Started
//!
//! It's best to start by looking through the [examples directory](https://github.com/rtthw/ration/tree/master/examples).
//!
//! # Note on allocating/opening paths.
//!
//! If the path you provide to `alloc` or `open` for some data type, `ration` will attempt to
//! place your data structure into `/dev/shm/`.



pub mod array;
pub mod block;

pub use array::*;
pub use block::*;



pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// Generic shared memory error.
    Shm(shared_memory::ShmemError),
    /// Expected a different block size from the one that was given.
    InvalidBlockSize,
    /// Attempted to allocated a block that has already been allocated.
    BlockAlreadyAllocated,
}
