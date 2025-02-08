//! Mapping



#[cfg(any(target_os="linux", target_os="freebsd", target_os="macos"))]
mod unix;

#[cfg(any(target_os="linux", target_os="freebsd", target_os="macos"))]
pub use unix::Mapping as Mapping;


pub struct MapCfg {}
