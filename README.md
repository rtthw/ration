


# Ration

A shared memory library for Rust. Useful for interprocess communication (IPC) through message-passing, sharing data structures, etc.

## Features

- **Performance.**
  > Shared memory is as fast as you can get when it comes to interprocess communication.
- **Flexibility.**
  > Simple data structures that can be used in a variety of ways. Abstract and combine them however you like to form your own types.

## Quickstart

In your main process (the one that owns the shared allocation), create your shared type and give it some initial data (don't forget this part because blocks don't start off with initial data, and accessing unitialized data is undefined behavior):
```rust
use ration::Block;

fn main() {
    let mut block: Block<i32> = Block::alloc("/tmp/MY_BLOCK").unwrap();
    *block = 71;
}
```
...then in some other process, you can access (and even mutate it) like so:

```rust
use ration::Block;

fn main() {
    let mut block: Block<i32> = Block::open("/tmp/MY_BLOCK").unwrap();
    println!("MY_BLOCK VALUE: {:?}", *block); // 71
}
```
> [!NOTE]
> I'd recommend using some mutable access checker (like a `Mutex`) if you plan on mutating shared data.

## Examples

- **The obligatory "Hello, world!" program that passes a single character string from server to client.**
  > [Server](./examples/helloworld_server.rs) and [client](./examples/helloworld_client.rs).
- **A simple channel type that passes messages between server and client.**
  > [Server](./examples/channel_server.rs) and [client](./examples/channel_client.rs).
- **A FizzBuzz clone that uses the `Block` type as a singleton. Also shows how to share strings.**
  > [Link](./examples/singleton.rs).

## License

[MIT](./LICENSE)
