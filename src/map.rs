//! Mapping



#[cfg(any(target_os="linux", target_os="freebsd", target_os="macos"))]
mod unix;

#[cfg(any(target_os="linux", target_os="freebsd", target_os="macos"))]
use unix as os;

use crate::Error;


pub struct MapCfg {}

impl MapCfg {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create(self) -> Result<MemoryMap, Error> {
        todo!()
    }
}

pub struct MemoryMap {
    raw: os::Mapping,
}
