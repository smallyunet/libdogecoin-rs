use crate::sys;

pub struct DogeWallet {
    private_key: String,
    address: String,
}

impl DogeWallet {
    /// Create a new wallet.
    ///
    /// # Arguments
    /// * `is_testnet` - Set to true for testnet, false for mainnet.
    pub fn new(is_testnet: bool) -> Option<Self> {
        static INIT: std::sync::Once = std::sync::Once::new();
        INIT.call_once(|| unsafe {
            sys::dogecoin_ecc_start();
        });

        // Defined in libdogecoin.h
        const PRIVKEYWIFLEN: usize = 53;
        const P2PKHLEN: usize = 35;

        let mut wif_privkey = [0u8; PRIVKEYWIFLEN];
        let mut p2pkh_pubkey = [0u8; P2PKHLEN];

        unsafe {
            let result = sys::generatePrivPubKeypair(
                wif_privkey.as_mut_ptr() as *mut i8,
                p2pkh_pubkey.as_mut_ptr() as *mut i8,
                is_testnet as u8,
            );

            if result != 1 {
                return None;
            }

            // Convert null-terminated C strings to Rust Strings
            let priv_key_cstr = std::ffi::CStr::from_ptr(wif_privkey.as_ptr() as *const i8);
            let address_cstr = std::ffi::CStr::from_ptr(p2pkh_pubkey.as_ptr() as *const i8);

            Some(DogeWallet {
                private_key: priv_key_cstr.to_string_lossy().into_owned(),
                address: address_cstr.to_string_lossy().into_owned(),
            })
        }
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn private_key(&self) -> &str {
        &self.private_key
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_wallet_mainnet() {
        let wallet = DogeWallet::new(false).unwrap();
        println!("Address: {}", wallet.address());
        println!("PrivKey: {}", wallet.private_key());

        // Mainnet addresses start with 'D'
        assert!(wallet.address().starts_with("D"));
    }

    #[test]
    fn test_create_wallet_testnet() {
        let wallet = DogeWallet::new(true).unwrap();
        println!("Address: {}", wallet.address());
        println!("PrivKey: {}", wallet.private_key());

        // Testnet addresses start with 'n'
        assert!(wallet.address().starts_with("n"));
    }
}
