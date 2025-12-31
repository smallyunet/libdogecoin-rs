use crate::sys;

pub struct DogecoinContext {
    // Placeholder for context management
}

impl DogecoinContext {
    pub fn new() -> Self {
        unsafe {
            sys::dogecoin_ecc_start();
        }
        DogecoinContext {}
    }
}

impl Drop for DogecoinContext {
    fn drop(&mut self) {
        unsafe {
            sys::dogecoin_ecc_stop();
        }
    }
}
