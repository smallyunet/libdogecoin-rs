# Roadmap

## Phase 1: Foundation ✅
- [x] Set up workspace with `libdogecoin-sys` and `libdogecoin-rs`.
- [x] Configure `bindgen` and `cc` to build `libdogecoin`.
- [x] Implement basic `DogeWallet` creation.
- [x] Add basic transaction creation support.

## Phase 2: Transaction Support ✅
- [x] Implement `DogeTransaction` struct.
- [x] Add support for adding UTXOs and Outputs.
- [x] Implement transaction signing.

## Phase 3: Advanced Features ✅
- [x] HD Wallet support (BIP32/BIP44).
- [x] Mnemonic generation (BIP39).
- [x] QR Code generation for addresses.

## Phase 4: Polish & Publish ✅
- [x] Comprehensive documentation.
- [x] CI/CD pipeline.
- [x] Publish to crates.io.

## Phase 5: Network & RPC
- [ ] Simple RPC client for Dogecoin nodes.
- [ ] UTXO balance queries.
- [ ] Transaction broadcasting.

## Phase 6: Security Enhancements
- [ ] Address validation utilities.
- [ ] Message signing and verification.
- [ ] Zeroize sensitive data in memory.

## Phase 7: Ecosystem Integration
- [ ] WASM compilation support (browser).
- [ ] C FFI exports for other languages.
- [ ] More examples and tutorials.
