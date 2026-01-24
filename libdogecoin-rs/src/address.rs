//! Address utilities (validation and network detection).

use crate::sys;
use std::ffi::CString;

/// Address network classification based on base58 prefix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressNetwork {
    Mainnet,
    Testnet,
    Unknown,
}

/// Address-related helper functions.
pub struct AddressUtils;

impl AddressUtils {
    /// Validate a P2PKH address (Base58Check + basic length/format).
    pub fn is_valid_p2pkh(address: &str) -> bool {
        if address.is_empty() {
            return false;
        }

        let c_address = match CString::new(address) {
            Ok(s) => s,
            Err(_) => return false,
        };

        // `verifyP2pkhAddress` expects `len` to be a buffer size. Using a value >= 25 is fine.
        let len = address.len().saturating_add(1);
        let result = unsafe { sys::verifyP2pkhAddress(c_address.as_ptr() as *mut i8, len) };
        result == 1
    }

    /// Determine whether a P2PKH address appears to be mainnet/testnet.
    ///
    /// This does not fully validate the address; call [`is_valid_p2pkh`] for full Base58Check.
    pub fn network(address: &str) -> AddressNetwork {
        if !Self::is_valid_p2pkh(address) {
            return AddressNetwork::Unknown;
        }

        let c_address = match CString::new(address) {
            Ok(s) => s,
            Err(_) => return AddressNetwork::Unknown,
        };

        // These libdogecoin functions expect a fixed-size char buffer, but the bindings accept a pointer.
        let is_test = unsafe { sys::isTestnetFromB58Prefix(c_address.as_ptr() as *const i8) };
        if is_test != 0 {
            return AddressNetwork::Testnet;
        }

        let is_main = unsafe { sys::isMainnetFromB58Prefix(c_address.as_ptr() as *const i8) };
        if is_main != 0 {
            return AddressNetwork::Mainnet;
        }

        AddressNetwork::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DogeWallet;

    #[test]
    fn test_address_validation_mainnet_wallet() {
        let wallet = DogeWallet::new(false).unwrap();
        assert!(AddressUtils::is_valid_p2pkh(wallet.address()));
        assert_eq!(
            AddressUtils::network(wallet.address()),
            AddressNetwork::Mainnet
        );
    }

    #[test]
    fn test_address_validation_rejects_garbage() {
        assert!(!AddressUtils::is_valid_p2pkh("not-an-address"));
        assert_eq!(
            AddressUtils::network("not-an-address"),
            AddressNetwork::Unknown
        );
    }
}
