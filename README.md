


# iproc

And inter-process communication (IPC) library for Rust.

## Features

- **Performance.**
  > Shared memory is as fast as you can get with IPC.
- **Flexibility.**
  > `iproc` provides simple data types that can be used in a variety of ways.

## Examples

- **The obligatory "Hello, world!" program that passes a single character string from server to client.**
  > [Server](./examples/helloworld_server.rs) and [client](./examples/helloworld_client.rs).
- **A simple channel type that passes messages between server and client.**
  > [Server](./examples/channel_server.rs) and [client](./examples/channel_client.rs).
