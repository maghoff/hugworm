Targetting host/desktop platform
================================
    cargo run

Developed with Linux and macOS.

By default, this invocation will include all log messages. To limit logging to
a given log level, specify this in the environmental variable `RUST_LOG`, eg:

    RUST_LOG=info cargo run

Targetting WASM
===============
Inspect and/or run `prepare-wasm.sh` to get build dependencies.

Run `build-wasm.sh` to build.

To run in a browser, you can use `basic-http-server`:

    # Install:
    cargo install basic-http-server

    # Start:
    basic-http-server site

The program is now available at <http://127.0.0.1:4000/>.
