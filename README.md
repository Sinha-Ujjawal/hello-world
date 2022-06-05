## Hello World Websocket in [Actix Web](https://actix.rs/)
A hello world websocket server in [Actix Web](https://actix.rs/). The websocket will stream random integers between 1..10.

## Quick Start
1. Install rust from [rustlang](https://www.rust-lang.org/tools/install). Use `rustup` to install Rust, it also installs build tools like `cargo`

2. Run `cargo run` from the terminal
```console
cargo run
```

3. You can optionally build the server using `cargo build --release` for improved performance.
```console
cargo build --release
./target/release/hello-world.exe
```

4. Websocket will be hosted on
`ws://localhost:8080/ws/random_integer`

5. Connect that websocket to get streams of random number every `2 seconds`. Below is an example response-

```json
{integer: 5}
```

## Copyrights
Licensed under [@MIT](./LICENSE)
