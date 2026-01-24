//! Message signing and verification.

use crate::sys;
use std::ffi::{CStr, CString};
use std::os::raw::c_void;

/// Message signing helpers.
pub struct Message;

impl Message {
    /// Sign a message with a WIF private key.
    ///
    /// Returns a Base64 encoded signature.
    pub fn sign(privkey_wif: &str, message: &str) -> Option<String> {
        crate::context::ensure_ecc_started();

        let c_priv = CString::new(privkey_wif).ok()?;
        let c_msg = CString::new(message).ok()?;

        let sig_ptr =
            unsafe { sys::sign_message(c_priv.as_ptr() as *mut i8, c_msg.as_ptr() as *mut i8) };
        if sig_ptr.is_null() {
            return None;
        }

        let sig = unsafe { CStr::from_ptr(sig_ptr) }
            .to_string_lossy()
            .into_owned();
        unsafe {
            sys::dogecoin_free(sig_ptr as *mut c_void);
        }

        Some(sig)
    }

    /// Verify a Base64 signature against a message and address.
    pub fn verify(signature_base64: &str, message: &str, address: &str) -> bool {
        crate::context::ensure_ecc_started();

        let c_sig = match CString::new(signature_base64) {
            Ok(s) => s,
            Err(_) => return false,
        };
        let c_msg = match CString::new(message) {
            Ok(s) => s,
            Err(_) => return false,
        };
        let c_addr = match CString::new(address) {
            Ok(s) => s,
            Err(_) => return false,
        };

        let result = unsafe {
            sys::verify_message(
                c_sig.as_ptr() as *mut i8,
                c_msg.as_ptr() as *mut i8,
                c_addr.as_ptr() as *mut i8,
            )
        };

        result == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DogeWallet;

    #[test]
    fn test_sign_and_verify_message_mainnet() {
        let wallet = DogeWallet::new(false).unwrap();
        let msg = "hello from libdogecoin-rs";

        let sig = Message::sign(wallet.private_key(), msg).expect("sign_message failed");
        assert!(Message::verify(&sig, msg, wallet.address()));
        assert!(!Message::verify(
            &sig,
            "different message",
            wallet.address()
        ));
    }
}
