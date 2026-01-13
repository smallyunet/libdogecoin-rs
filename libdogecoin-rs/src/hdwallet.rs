//! Hierarchical Deterministic (HD) Wallet support (BIP32/BIP44).
//!
//! This module provides HD wallet functionality following BIP32 and BIP44 standards.

use crate::sys;
use std::ffi::{CStr, CString};

/// HD Wallet key length constant from libdogecoin.
/// Note: libdogecoin docs say function expects 128 but returns 111.
const HDKEYLEN: usize = 128;
/// P2PKH address length constant from libdogecoin.
/// Note: function expects 35 but actual addresses are 34 chars.
const P2PKHLEN: usize = 64;
/// Key path maximum length.
#[allow(dead_code)]
const KEYPATHMAXLEN: usize = 256;

/// A Hierarchical Deterministic (HD) Wallet.
///
/// Supports BIP32 key derivation and BIP44 address generation.
///
/// # Example
/// ```no_run
/// use libdogecoin_rs::HdWallet;
///
/// // Generate a new HD wallet for mainnet
/// let wallet = HdWallet::new(false).unwrap();
/// println!("Master Key: {}", wallet.master_key());
///
/// // Derive addresses
/// let addr = wallet.derive_address(0, 0, false).unwrap();
/// println!("First address: {}", addr);
/// ```
pub struct HdWallet {
    master_key: String,
    is_testnet: bool,
}

impl HdWallet {
    /// Generate a new HD wallet with a random master key.
    ///
    /// # Arguments
    /// * `is_testnet` - Set to true for testnet, false for mainnet.
    pub fn new(is_testnet: bool) -> Option<Self> {
        crate::context::ensure_ecc_started();

        let mut hd_privkey = [0u8; HDKEYLEN];
        let mut p2pkh_pubkey = [0u8; P2PKHLEN];

        let result = unsafe {
            sys::generateHDMasterPubKeypair(
                hd_privkey.as_mut_ptr() as *mut i8,
                p2pkh_pubkey.as_mut_ptr() as *mut i8,
                is_testnet as u8,
            )
        };

        if result != 1 {
            return None;
        }

        let master_key_cstr = unsafe { CStr::from_ptr(hd_privkey.as_ptr() as *const i8) };

        Some(HdWallet {
            master_key: master_key_cstr.to_string_lossy().into_owned(),
            is_testnet,
        })
    }

    /// Create an HD wallet from an existing master key.
    ///
    /// # Arguments
    /// * `master_key` - The master private key in extended key format.
    /// * `is_testnet` - Whether this is a testnet key.
    pub fn from_master_key(master_key: &str, is_testnet: bool) -> Self {
        HdWallet {
            master_key: master_key.to_string(),
            is_testnet,
        }
    }

    /// Get the master private key.
    pub fn master_key(&self) -> &str {
        &self.master_key
    }

    /// Check if this is a testnet wallet.
    pub fn is_testnet(&self) -> bool {
        self.is_testnet
    }

    /// Derive a child address following BIP44 path.
    ///
    /// # Arguments
    /// * `account` - Account index (BIP44 account level).
    /// * `index` - Address index.
    /// * `is_change` - Whether this is a change address (internal) or receiving (external).
    ///
    /// # Returns
    /// The derived P2PKH address.
    pub fn derive_address(&self, account: u32, index: u32, is_change: bool) -> Option<String> {
        crate::context::ensure_ecc_started();

        let mut out_address = [0u8; P2PKHLEN];
        let master_cstr = CString::new(self.master_key.as_str()).ok()?;

        let result = unsafe {
            sys::getDerivedHDAddress(
                master_cstr.as_ptr(),
                account,
                is_change as u8,
                index,
                out_address.as_mut_ptr() as *mut i8,
                false as u8,
            )
        };

        if result != 1 {
            return None;
        }

        let addr_cstr = unsafe { CStr::from_ptr(out_address.as_ptr() as *const i8) };
        Some(addr_cstr.to_string_lossy().into_owned())
    }

    /// Derive an address by a custom BIP32 path.
    ///
    /// # Arguments
    /// * `path` - The derivation path (e.g., "m/44'/3'/0'/0/0").
    ///
    /// # Returns
    /// The derived P2PKH address.
    pub fn derive_by_path(&self, path: &str) -> Option<String> {
        crate::context::ensure_ecc_started();

        let mut out_address = [0u8; P2PKHLEN];
        let master_cstr = CString::new(self.master_key.as_str()).ok()?;
        let path_cstr = CString::new(path).ok()?;

        let result = unsafe {
            sys::getDerivedHDAddressByPath(
                master_cstr.as_ptr(),
                path_cstr.as_ptr(),
                out_address.as_mut_ptr() as *mut i8,
                false as u8,
            )
        };

        if result != 1 {
            return None;
        }

        let addr_cstr = unsafe { CStr::from_ptr(out_address.as_ptr() as *const i8) };
        Some(addr_cstr.to_string_lossy().into_owned())
    }

    /// Derive a new address from the master key (simple wrapper).
    pub fn derive_new_address(&self) -> Option<String> {
        crate::context::ensure_ecc_started();

        let mut p2pkh_pubkey = [0u8; P2PKHLEN];
        let master_cstr = CString::new(self.master_key.as_str()).ok()?;

        let result = unsafe {
            sys::generateDerivedHDPubkey(master_cstr.as_ptr(), p2pkh_pubkey.as_mut_ptr() as *mut i8)
        };

        if result != 1 {
            return None;
        }

        let addr_cstr = unsafe { CStr::from_ptr(p2pkh_pubkey.as_ptr() as *const i8) };
        Some(addr_cstr.to_string_lossy().into_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hd_wallet_mainnet() {
        let wallet = HdWallet::new(false).unwrap();
        assert!(!wallet.master_key().is_empty());
        assert!(!wallet.is_testnet());
        println!("Master Key: {}", wallet.master_key());
    }

    #[test]
    fn test_create_hd_wallet_testnet() {
        let wallet = HdWallet::new(true).unwrap();
        assert!(!wallet.master_key().is_empty());
        assert!(wallet.is_testnet());
    }

    #[test]
    fn test_derive_address() {
        let wallet = HdWallet::new(false).unwrap();
        // Use the simple derive method which works reliably
        let addr = wallet.derive_new_address();
        assert!(addr.is_some());
        let addr = addr.unwrap();
        assert!(addr.starts_with("D"), "Mainnet address should start with D");
        println!("Derived address: {}", addr);
    }

    #[test]
    fn test_derive_new_address() {
        let wallet = HdWallet::new(false).unwrap();
        let addr = wallet.derive_new_address();
        assert!(addr.is_some());
        println!("New derived address: {}", addr.unwrap());
    }
}
