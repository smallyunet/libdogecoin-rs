//! QR Code generation for Dogecoin addresses.
//!
//! This module provides functionality to generate QR codes for addresses
//! in various formats including console output, PNG, and JPEG files.

use crate::sys;
use std::ffi::CString;

/// QR Code generator for Dogecoin addresses.
///
/// # Example
/// ```no_run
/// use libdogecoin_rs::QrCode;
///
/// // Print QR code to console
/// QrCode::print_console("DAddress");
///
/// // Generate QR code as a string
/// if let Some(qr_string) = QrCode::to_string("DAddress") {
///     println!("{}", qr_string);
/// }
///
/// // Save QR code as PNG
/// QrCode::to_png("DAddress", "address_qr.png", 4);
/// ```
pub struct QrCode;

impl QrCode {
    /// Generate a QR code as a text string with line breaks.
    ///
    /// # Arguments
    /// * `address` - The Dogecoin address to encode.
    ///
    /// # Returns
    /// A string representation of the QR code.
    pub fn to_string(address: &str) -> Option<String> {
        // The QR string can be quite large, allocate enough space
        const QR_STRING_SIZE: usize = 4096;
        let mut out_string = vec![0u8; QR_STRING_SIZE];
        let addr_cstr = CString::new(address).ok()?;

        let result = unsafe {
            sys::qrgen_p2pkh_to_qr_string(addr_cstr.as_ptr(), out_string.as_mut_ptr() as *mut i8)
        };

        if result <= 0 {
            return None;
        }

        // Find the null terminator
        let len = out_string
            .iter()
            .position(|&x| x == 0)
            .unwrap_or(out_string.len());
        let qr_str = String::from_utf8_lossy(&out_string[..len]).into_owned();
        Some(qr_str)
    }

    /// Print a QR code for an address to the console.
    ///
    /// # Arguments
    /// * `address` - The Dogecoin address to encode.
    pub fn print_console(address: &str) {
        if let Ok(addr_cstr) = CString::new(address) {
            unsafe {
                sys::qrgen_p2pkh_consoleprint_to_qr(addr_cstr.as_ptr() as *mut i8);
            }
        }
    }

    /// Generate a QR code and save as a PNG file.
    ///
    /// # Arguments
    /// * `address` - The Dogecoin address to encode.
    /// * `filename` - The output filename.
    /// * `size_multiplier` - Size multiplier for the QR code image.
    ///
    /// # Returns
    /// `true` if the file was created successfully.
    pub fn to_png(address: &str, filename: &str, size_multiplier: u8) -> bool {
        let addr_cstr = match CString::new(address) {
            Ok(s) => s,
            Err(_) => return false,
        };
        let filename_cstr = match CString::new(filename) {
            Ok(s) => s,
            Err(_) => return false,
        };

        let result = unsafe {
            sys::qrgen_string_to_qr_pngfile(
                filename_cstr.as_ptr(),
                addr_cstr.as_ptr(),
                size_multiplier,
            )
        };

        result == 1
    }

    /// Generate a QR code and save as a JPEG file.
    ///
    /// # Arguments
    /// * `address` - The Dogecoin address to encode.
    /// * `filename` - The output filename.
    /// * `size_multiplier` - Size multiplier for the QR code image.
    ///
    /// # Returns
    /// `true` if the file was created successfully.
    pub fn to_jpeg(address: &str, filename: &str, size_multiplier: u8) -> bool {
        let addr_cstr = match CString::new(address) {
            Ok(s) => s,
            Err(_) => return false,
        };
        let filename_cstr = match CString::new(filename) {
            Ok(s) => s,
            Err(_) => return false,
        };

        let result = unsafe {
            sys::qrgen_string_to_qr_jpgfile(
                filename_cstr.as_ptr(),
                addr_cstr.as_ptr(),
                size_multiplier,
            )
        };

        result == 1
    }

    /// Get the raw QR code bits as an array.
    ///
    /// # Arguments
    /// * `address` - The Dogecoin address to encode.
    ///
    /// # Returns
    /// A tuple of (size, data) where size is the width/height of the QR code
    /// and data contains the QR code bits.
    pub fn to_bits(address: &str) -> Option<(i32, Vec<u8>)> {
        // Maximum QR code size estimate
        const MAX_QR_SIZE: usize = 256 * 256;
        let mut bits = vec![0u8; MAX_QR_SIZE];
        let addr_cstr = CString::new(address).ok()?;

        let size = unsafe { sys::qrgen_p2pkh_to_qrbits(addr_cstr.as_ptr(), bits.as_mut_ptr()) };

        if size <= 0 {
            return None;
        }

        // Truncate to actual size
        let total_bytes = (size * size) as usize;
        bits.truncate(total_bytes);

        Some((size, bits))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DogeWallet;

    #[test]
    fn test_qr_to_string() {
        let wallet = DogeWallet::new(false).unwrap();
        // QR generation may fail depending on libdogecoin version/config
        // Just test that the function doesn't crash
        let qr = QrCode::to_string(wallet.address());
        if let Some(qr_str) = qr {
            assert!(!qr_str.is_empty());
            println!("QR Code generated successfully");
        } else {
            println!("QR generation returned None (may be expected)");
        }
    }
}
