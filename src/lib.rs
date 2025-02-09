


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
