


pub mod array;
pub mod block;
mod map;

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
    MapCreateFailed(u32),
    MapOpenFailed(u32),
    /// Attempted to map shared memory that already exists.
    MappingExists,
    /// Attempted to create a map that was of length zero.
    MapSizeZero,
    UnknownOsError(u32),
}
