//! # libdogecoin-rs
//!
//! Safe Rust bindings for libdogecoin, providing:
//! - Wallet creation and key generation
//! - Transaction creation and signing
//! - HD Wallet support (BIP32/BIP44)
//! - Mnemonic phrase generation (BIP39)
//! - QR Code generation for addresses

pub mod address;
pub mod context;
pub mod hdwallet;
pub mod message;
pub mod mnemonic;
pub mod qrcode;
#[cfg(feature = "rpc")]
pub mod rpc;
pub mod transaction;
pub mod wallet;

pub use address::{AddressNetwork, AddressUtils};
pub use hdwallet::HdWallet;
pub use libdogecoin_sys as sys;
pub use message::Message;
pub use mnemonic::Mnemonic;
pub use qrcode::QrCode;
#[cfg(feature = "rpc")]
pub use rpc::DogeRpcClient;
pub use transaction::DogeTransaction;
pub use wallet::DogeWallet;
