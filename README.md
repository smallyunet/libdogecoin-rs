# libdogecoin-rs

Safe Rust bindings for [libdogecoin](https://github.com/dogecoinfoundation/libdogecoin).

## Features

- **Wallet Creation** - Generate Dogecoin keypairs (mainnet/testnet)
- **Transaction Building** - Create, sign, and serialize transactions
- **HD Wallets** - BIP32/BIP44 hierarchical deterministic wallets
- **Mnemonic Phrases** - BIP39 seed phrase generation and derivation
- **QR Codes** - Generate QR codes for addresses (PNG/JPEG)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
libdogecoin-rs = { git = "https://github.com/your-repo/libdogecoin-rs" }
```

## Quick Start

```rust
use libdogecoin_rs::{DogeWallet, HdWallet, Mnemonic, DogeTransaction};

// Create a simple wallet
let wallet = DogeWallet::new(false).unwrap();  // mainnet
println!("Address: {}", wallet.address());
println!("Private Key: {}", wallet.private_key());

// Create an HD wallet
let hd_wallet = HdWallet::new(false).unwrap();
let addr = hd_wallet.derive_new_address().unwrap();
println!("HD Address: {}", addr);

// Generate a mnemonic
let mnemonic = Mnemonic::generate("256").unwrap();  // 24 words
println!("Mnemonic: {}", mnemonic.phrase());

// Derive address from mnemonic
let addr = mnemonic.derive_address(0, 0, "", false).unwrap();
println!("Mnemonic Address: {}", addr);

// Create a transaction
let mut tx = DogeTransaction::new();
tx.add_utxo("previous_txid_hex", 0);
tx.add_output("DDestinationAddress", "10.0");
```

## Building

```bash
# Clone with submodules
git clone --recursive https://github.com/your-repo/libdogecoin-rs
cd libdogecoin-rs

# Or initialize submodules after cloning
git submodule update --init --recursive

# Build
cargo build

# Run tests
cargo test -- --test-threads=1
```

## License

MIT License - see [LICENSE](LICENSE) for details.
