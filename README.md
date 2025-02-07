


# iproc

And inter-process communication (IPC) library for Rust.

## Features

- **Performance.**
  > Shared memory is as fast as you can get with IPC.
- **Flexibility.**
  > `iproc` provides simple data types that can be used in a variety of ways.

## Examples

### A simple, string-like array.

```rust
// main.rs (server process)
use iproc::Array;

fn main() {
    // Allocate a 32-character array to shared memory.
    let mut my_array: Array<char> = Array::alloc("/tmp/MY_ARRAY", 32).unwrap();
    my_array.push_many("It's working!".chars());

    // Start the client process...
}
```

```rust
// main.rs (client process)
use iproc::Array;

fn main() {
    // Open a 32 character array in shared memory.
    let mut my_array: Array<char> = Array::open("/tmp/MY_ARRAY").unwrap();
    let mut my_string = String::new();

    while let Some(c) = my_array.pop() {
        my_string.push(c);
    }

    assert_eq!(my_string, "It's working!".to_string());
}
```
