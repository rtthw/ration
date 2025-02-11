


# Ration

A shared memory library for Rust. Useful for interprocess communication (IPC) through message-passing, sharing data structures, etc.

## Features

- **Performance.**
  > Shared memory is as fast as you can get with IPC.
- **Flexibility.**
  > `ration` provides simple data types that can be used in a variety of ways.

## Examples

- **The obligatory "Hello, world!" program that passes a single character string from server to client.**
  > [Server](./examples/helloworld_server.rs) and [client](./examples/helloworld_client.rs).
- **A simple channel type that passes messages between server and client.**
  > [Server](./examples/channel_server.rs) and [client](./examples/channel_client.rs).
- **A FizzBuzz clone that uses the `Block` type as a singleton. Also shows how to share strings.**
  > [Link](./examples/singleton.rs).

## License

[MIT](./LICENSE)
