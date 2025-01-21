


pub mod block;



pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    Shm(shared_memory::ShmemError),
}
