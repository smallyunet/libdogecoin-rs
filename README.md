# libdogecoin-rs

A safe, ergonomic Rust wrapper for [libdogecoin](https://github.com/dogecoin/libdogecoin).

## Project Goal
To provide a clean, safe Rust API for Dogecoin operations, hiding the complexity of unsafe C FFI calls and memory management.

## Features
- **Safe Wrapper**: Encapsulates `libdogecoin` C functions in safe Rust structs.
- **Easy Wallet Creation**: Create wallets without manual memory management.
- **Statically Linked**: `libdogecoin` is built and linked statically.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
libdogecoin-rs = { path = "libdogecoin-rs" }
```

### Example: Create a Wallet

```rust
use libdogecoin_rs::DogeWallet;

fn main() {
    // Create a new mainnet wallet
    let wallet = DogeWallet::new(false).expect("Failed to create wallet");
    
    println!("Address: {}", wallet.address());
    println!("Private Key: {}", wallet.private_key());
}
```

## Architecture
- **libdogecoin-sys**: Low-level FFI bindings and build script for `libdogecoin`.
- **dogecoin-rs**: High-level safe Rust API.
