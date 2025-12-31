# dogecoin-rs

A safe, idiomatic Rust wrapper for the official [libdogecoin](https://github.com/dogecoinfoundation/libdogecoin) C library.

## Project Structure

This project is a Cargo Workspace containing:

-   `libdogecoin-sys`: Low-level unsafe bindings generated from `libdogecoin` (statically linked).
-   `dogecoin-rs`: High-level safe wrapper providing an ergonomic Rust API.

## Roadmap

See [ROADMAP.md](ROADMAP.md) for the development plan.

## Build

Ensure you have a C compiler installed (e.g., `clang`, `gcc`) and `bindgen` dependencies if necessary.

```bash
cargo build
```

## Usage

(Coming soon in Phase 2)
