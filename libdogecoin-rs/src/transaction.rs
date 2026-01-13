//! Transaction creation and signing for Dogecoin.
//!
//! This module provides a safe Rust interface to libdogecoin's transaction API.

use crate::sys;
use std::ffi::{CStr, CString};

/// A Dogecoin transaction builder.
///
/// # Example
/// ```no_run
/// use libdogecoin_rs::DogeTransaction;
///
/// let mut tx = DogeTransaction::new();
/// tx.add_utxo("previous_txid_hex", 0);
/// tx.add_output("DDestinationAddress", "10.5");
/// let raw = tx.finalize("DDestinationAddress", "0.01", None);
/// tx.sign_with_privkey(0, "private_key_wif");
/// let signed_raw = tx.get_raw();
/// ```
pub struct DogeTransaction {
    tx_index: i32,
}

impl DogeTransaction {
    /// Create a new transaction.
    ///
    /// This allocates a new transaction in libdogecoin's internal memory.
    pub fn new() -> Self {
        let tx_index = unsafe { sys::start_transaction() };
        DogeTransaction { tx_index }
    }

    /// Add a UTXO (Unspent Transaction Output) to this transaction.
    ///
    /// # Arguments
    /// * `txid` - The transaction ID of the UTXO in hexadecimal format.
    /// * `vout` - The output index within that transaction.
    ///
    /// # Returns
    /// `true` if the UTXO was added successfully.
    pub fn add_utxo(&mut self, txid: &str, vout: i32) -> bool {
        let txid_cstr = CString::new(txid).expect("Invalid txid string");
        let result = unsafe { sys::add_utxo(self.tx_index, txid_cstr.as_ptr() as *mut i8, vout) };
        result == 1
    }

    /// Add an output to this transaction.
    ///
    /// # Arguments
    /// * `address` - The destination Dogecoin address.
    /// * `amount` - The amount in DOGE as a string (e.g., "10.5").
    ///
    /// # Returns
    /// `true` if the output was added successfully.
    pub fn add_output(&mut self, address: &str, amount: &str) -> bool {
        let addr_cstr = CString::new(address).expect("Invalid address");
        let amount_cstr = CString::new(amount).expect("Invalid amount");
        let result = unsafe {
            sys::add_output(
                self.tx_index,
                addr_cstr.as_ptr() as *mut i8,
                amount_cstr.as_ptr() as *mut i8,
            )
        };
        result == 1
    }

    /// Finalize the transaction.
    ///
    /// # Arguments
    /// * `destination` - The destination address (for verification).
    /// * `fee` - The transaction fee in DOGE (will be subtracted).
    /// * `change_address` - Optional change address. If None, change goes to first UTXO's address.
    ///
    /// # Returns
    /// The raw transaction hex string.
    pub fn finalize(
        &self,
        destination: &str,
        fee: &str,
        change_address: Option<&str>,
    ) -> Option<String> {
        let dest_cstr = CString::new(destination).expect("Invalid destination");
        let fee_cstr = CString::new(fee).expect("Invalid fee");

        // For verification amount, we use "0" as placeholder
        let amount_cstr = CString::new("0").unwrap();

        let change_cstr = change_address.map(|s| CString::new(s).expect("Invalid change address"));

        let change_ptr = match &change_cstr {
            Some(s) => s.as_ptr() as *mut i8,
            None => std::ptr::null_mut(),
        };

        let result = unsafe {
            sys::finalize_transaction(
                self.tx_index,
                dest_cstr.as_ptr() as *mut i8,
                fee_cstr.as_ptr() as *mut i8,
                amount_cstr.as_ptr() as *mut i8,
                change_ptr,
            )
        };

        if result.is_null() {
            None
        } else {
            let raw_str = unsafe { CStr::from_ptr(result) };
            Some(raw_str.to_string_lossy().into_owned())
        }
    }

    /// Sign an input of the transaction.
    ///
    /// # Arguments
    /// * `script_pubkey` - The scriptPubKey of the UTXO being spent.
    /// * `privkey` - The private key in WIF format.
    ///
    /// # Returns
    /// `true` if signing was successful.
    pub fn sign(&mut self, script_pubkey: &str, privkey: &str) -> bool {
        let script_cstr = CString::new(script_pubkey).expect("Invalid script");
        let privkey_cstr = CString::new(privkey).expect("Invalid privkey");
        let result = unsafe {
            sys::sign_transaction(
                self.tx_index,
                script_cstr.as_ptr() as *mut i8,
                privkey_cstr.as_ptr() as *mut i8,
            )
        };
        result == 1
    }

    /// Sign an input by vout index using a private key.
    ///
    /// # Arguments
    /// * `vout_index` - The index of the input to sign.
    /// * `privkey` - The private key in WIF format.
    ///
    /// # Returns
    /// `true` if signing was successful.
    pub fn sign_with_privkey(&mut self, vout_index: i32, privkey: &str) -> bool {
        let privkey_cstr = CString::new(privkey).expect("Invalid privkey");
        let result = unsafe {
            sys::sign_transaction_w_privkey(
                self.tx_index,
                vout_index,
                privkey_cstr.as_ptr() as *mut i8,
            )
        };
        result == 1
    }

    /// Get the raw transaction hex.
    ///
    /// # Returns
    /// The transaction as a hexadecimal string.
    pub fn get_raw(&self) -> Option<String> {
        let result = unsafe { sys::get_raw_transaction(self.tx_index) };
        if result.is_null() {
            None
        } else {
            let raw_str = unsafe { CStr::from_ptr(result) };
            Some(raw_str.to_string_lossy().into_owned())
        }
    }

    /// Get the internal transaction index.
    pub fn index(&self) -> i32 {
        self.tx_index
    }
}

impl Default for DogeTransaction {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for DogeTransaction {
    fn drop(&mut self) {
        unsafe {
            sys::clear_transaction(self.tx_index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_transaction() {
        let tx = DogeTransaction::new();
        assert!(tx.index() >= 0);
    }

    #[test]
    fn test_transaction_default() {
        let tx = DogeTransaction::default();
        assert!(tx.index() >= 0);
    }
}
