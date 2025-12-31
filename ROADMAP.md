# Development Roadmap: dogecoin-rs

This roadmap outlines the path from project initialization to a production-ready Safe Rust wrapper for `libdogecoin`.

## Phase 1: Infrastructure Setup (Completed)
**Goal**: Establish a robust build system that compiles `libdogecoin` statically and generates raw Rust bindings.
- [x] Initialize Rust Workspace (`dogecoin-rs` root, `libdogecoin-sys` crate).
- [x] Configure `libdogecoin` as a git submodule (vendored source).
- [x] Implement `build.rs` in `libdogecoin-sys` to compile C sources using `cc`.
- [x] Configure `bindgen` to generate FFI bindings automatically.
- [x] Verify compilation on host architecture.

## Phase 2: MVP Verification (Current)
**Goal**: Validate the FFI boundary with a minimal "Hello World" example.
- [ ] Expose basic functions (e.g., `dogecoin_ecc_start`, `dogecoin_ecc_stop`) in `sys`.
- [ ] Create a high-level `DogecoinContext` struct in `dogecoin-rs`.
- [ ] Write an integration test that starts/stops the context and perhaps generates a random keypair (raw).
- [ ] Ensure CI/CD can build the C dependencies.

## Phase 3: Core Feature Encapsulation
**Goal**: Wrap unsafe C pointers in safe, ergonomic Rust types.
- [ ] **Memory Management**: Implement `Drop` traits for C-allocated opaque pointers.
- [ ] **Keys & Addresses**: Wrap private/public keys and address generation.
    - `DogePrivKey` / `DogePubKey`
    - Address derivation helper
- [ ] **Transactions**: implement transaction builder pattern.
    - `DogeTransaction` builder
    - Input/Output handling
    - Signing mechanisms

## Phase 4: Engineering & Release
**Goal**: Polish for public consumption.
- [ ] **Documentation**: Complete rustdoc comments with examples.
- [ ] **Safety Audit**: Review all `unsafe` blocks and verify invariants.
- [ ] **CI/CD**: Automate testing on Linux/macOS/Windows.
- [ ] **Publishing**: Release to crates.io (handling the C source distribution strategy).
