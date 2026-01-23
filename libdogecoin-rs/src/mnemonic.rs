//! BIP39 Mnemonic phrase support.
//!
//! This module provides mnemonic generation, seed derivation, and address generation
//! from mnemonic phrases following the BIP39 standard.

use crate::sys;
use std::ffi::{CStr, CString};
use zeroize::Zeroizing;

/// Maximum mnemonic size from libdogecoin.
const MAX_MNEMONIC_SIZE: usize = 1024;
/// Maximum passphrase size.
#[allow(dead_code)]
const MAX_PASS_SIZE: usize = 256;
/// Maximum seed size.
const MAX_SEED_SIZE: usize = 64;
/// P2PKH address length - using larger buffer for safety.
const P2PKHLEN: usize = 64;

/// A BIP39 mnemonic phrase.
///
/// Provides functionality to generate random mnemonics, derive seeds,
/// and create HD wallets from mnemonic phrases.
///
/// # Example
/// ```no_run
/// use libdogecoin_rs::Mnemonic;
///
/// // Generate a new 256-bit mnemonic (24 words)
/// let mnemonic = Mnemonic::generate("256").unwrap();
/// println!("Mnemonic: {}", mnemonic.phrase());
///
/// // Derive an address from the mnemonic
/// let addr = mnemonic.derive_address(0, 0, "", false).unwrap();
/// println!("Address: {}", addr);
/// ```
pub struct Mnemonic {
    phrase: Zeroizing<String>,
}

impl Mnemonic {
    /// Generate a new random mnemonic phrase.
    ///
    /// # Arguments
    /// * `entropy_size` - Entropy size: "128" for 12 words, "256" for 24 words.
    ///
    /// # Returns
    /// A new Mnemonic with a random phrase.
    pub fn generate(entropy_size: &str) -> Option<Self> {
        crate::context::ensure_ecc_started();

        let mut mnemonic = [0u8; MAX_MNEMONIC_SIZE];
        let size_cstr = CString::new(entropy_size).ok()?;

        let result = unsafe {
            sys::generateRandomEnglishMnemonic(
                size_cstr.as_ptr() as *mut i8,
                mnemonic.as_mut_ptr() as *mut i8,
            )
        };

        if result != 0 {
            return None;
        }

        let phrase_cstr = unsafe { CStr::from_ptr(mnemonic.as_ptr() as *const i8) };
        Some(Mnemonic {
            phrase: Zeroizing::new(phrase_cstr.to_string_lossy().into_owned()),
        })
    }

    /// Create a Mnemonic from an existing phrase.
    ///
    /// # Arguments
    /// * `phrase` - The mnemonic phrase (space-separated words).
    pub fn from_phrase(phrase: &str) -> Self {
        Mnemonic {
            phrase: Zeroizing::new(phrase.to_string()),
        }
    }

    /// Get the mnemonic phrase.
    pub fn phrase(&self) -> &str {
        self.phrase.as_str()
    }

    /// Derive a seed from the mnemonic phrase.
    ///
    /// # Arguments
    /// * `passphrase` - Optional passphrase (use empty string for no passphrase).
    ///
    /// # Returns
    /// A 64-byte seed.
    pub fn to_seed(&self, passphrase: &str) -> Option<[u8; MAX_SEED_SIZE]> {
        crate::context::ensure_ecc_started();

        let mut seed = [0u8; MAX_SEED_SIZE];
        let mnemonic_cstr = CString::new(self.phrase.as_str()).ok()?;
        let pass_cstr = CString::new(passphrase).ok()?;

        let result = unsafe {
            sys::dogecoin_seed_from_mnemonic(
                mnemonic_cstr.as_ptr() as *mut i8,
                pass_cstr.as_ptr() as *mut i8,
                seed.as_mut_ptr(),
            )
        };

        if result != 0 {
            return None;
        }

        Some(seed)
    }

    /// Derive a P2PKH address from the mnemonic using BIP44 derivation.
    ///
    /// # Arguments
    /// * `account` - Account index.
    /// * `index` - Address index.
    /// * `passphrase` - Mnemonic passphrase (can be empty).
    /// * `is_testnet` - Whether to generate testnet address.
    ///
    /// # Returns
    /// The derived P2PKH address.
    pub fn derive_address(
        &self,
        account: u32,
        index: u32,
        passphrase: &str,
        is_testnet: bool,
    ) -> Option<String> {
        crate::context::ensure_ecc_started();

        let mut p2pkh_pubkey = [0u8; P2PKHLEN];
        let mnemonic_cstr = CString::new(self.phrase.as_str()).ok()?;
        let pass_cstr = CString::new(passphrase).ok()?;

        // Change level: "0" for external (receiving), "1" for internal (change)
        let change_level_cstr = CString::new("0").ok()?;

        let result = unsafe {
            sys::getDerivedHDAddressFromMnemonic(
                account,
                index,
                change_level_cstr.as_ptr() as *mut i8,
                mnemonic_cstr.as_ptr() as *mut i8,
                pass_cstr.as_ptr() as *mut i8,
                p2pkh_pubkey.as_mut_ptr() as *mut i8,
                is_testnet,
            )
        };

        if result != 0 {
            return None;
        }

        let addr_cstr = unsafe { CStr::from_ptr(p2pkh_pubkey.as_ptr() as *const i8) };
        Some(addr_cstr.to_string_lossy().into_owned())
    }

    /// Derive a change address from the mnemonic.
    ///
    /// # Arguments
    /// * `account` - Account index.
    /// * `index` - Address index.
    /// * `passphrase` - Mnemonic passphrase.
    /// * `is_testnet` - Whether to generate testnet address.
    pub fn derive_change_address(
        &self,
        account: u32,
        index: u32,
        passphrase: &str,
        is_testnet: bool,
    ) -> Option<String> {
        crate::context::ensure_ecc_started();

        let mut p2pkh_pubkey = [0u8; P2PKHLEN];
        let mnemonic_cstr = CString::new(self.phrase.as_str()).ok()?;
        let pass_cstr = CString::new(passphrase).ok()?;
        let change_level_cstr = CString::new("1").ok()?;

        let result = unsafe {
            sys::getDerivedHDAddressFromMnemonic(
                account,
                index,
                change_level_cstr.as_ptr() as *mut i8,
                mnemonic_cstr.as_ptr() as *mut i8,
                pass_cstr.as_ptr() as *mut i8,
                p2pkh_pubkey.as_mut_ptr() as *mut i8,
                is_testnet,
            )
        };

        if result != 0 {
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
    fn test_generate_mnemonic_128() {
        let mnemonic = Mnemonic::generate("128").unwrap();
        let words: Vec<&str> = mnemonic.phrase().split_whitespace().collect();
        assert_eq!(words.len(), 12, "128-bit entropy should produce 12 words");
        println!("12-word mnemonic: {}", mnemonic.phrase());
    }

    #[test]
    fn test_generate_mnemonic_256() {
        let mnemonic = Mnemonic::generate("256").unwrap();
        let words: Vec<&str> = mnemonic.phrase().split_whitespace().collect();
        assert_eq!(words.len(), 24, "256-bit entropy should produce 24 words");
        println!("24-word mnemonic: {}", mnemonic.phrase());
    }

    #[test]
    fn test_mnemonic_to_seed() {
        let mnemonic = Mnemonic::generate("128").unwrap();
        let seed = mnemonic.to_seed("");
        assert!(seed.is_some());
        let seed = seed.unwrap();
        assert_eq!(seed.len(), 64);
    }

    #[test]
    fn test_derive_address_from_mnemonic() {
        let mnemonic = Mnemonic::generate("128").unwrap();
        let addr = mnemonic.derive_address(0, 0, "", false);
        assert!(addr.is_some());
        let addr = addr.unwrap();
        assert!(addr.starts_with("D"), "Mainnet address should start with D");
        println!("Address from mnemonic: {}", addr);
    }

    #[test]
    fn test_from_phrase() {
        let phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let mnemonic = Mnemonic::from_phrase(phrase);
        assert_eq!(mnemonic.phrase(), phrase);
    }
}
