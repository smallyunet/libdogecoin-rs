//! Dogecoin ECC context management.

use crate::sys;

/// Ensure ECC context is initialized (thread-safe).
pub(crate) fn ensure_ecc_started() {
    static INIT: std::sync::Once = std::sync::Once::new();
    INIT.call_once(|| unsafe {
        sys::dogecoin_ecc_start();
    });
}

/// Dogecoin ECC context.
///
/// This struct manages the ECC context lifecycle.
/// The context is started on creation and stopped when dropped.
pub struct DogecoinContext {
    // Placeholder for context management
}

impl DogecoinContext {
    /// Create a new ECC context.
    pub fn new() -> Self {
        unsafe {
            sys::dogecoin_ecc_start();
        }
        DogecoinContext {}
    }
}

impl Default for DogecoinContext {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for DogecoinContext {
    fn drop(&mut self) {
        unsafe {
            sys::dogecoin_ecc_stop();
        }
    }
}
