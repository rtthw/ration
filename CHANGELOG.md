
# 0.2.1

### Fixed

- `Error` now implements `std::error::Error`.

# 0.2.0

*Array iteration, more documentation, a new example, and debuggable collections.*

### Added

- `Array` iteration with `ArrayIter` & `ArrayIterMut`.
- A [singleton example](./examples/singleton.rs) that shows a way to use `Block`s for something other than IPC.
- Implement `std::fmt::Debug` for the `Array` & `Block` types if the generic also implements it.
- Simple examples in the type documentation.

### Changed

- Various documentation improvements.

# 0.1.0

Initial release.
