// SPDX-License-Identifier: LGPL-2.1-only
//! `IrecvError` — mirrors `irecv_error_t` from
//! `libirecovery/include/libirecovery.h`. This crate itself never returns
//! one (it does no I/O), but it's provided so code that implements its own
//! USB transport on top of [`crate::device_db`]/[`crate::info`] (e.g. via
//! [`nusb`](https://docs.rs/nusb) or [`rusb`](https://docs.rs/rusb)) can
//! report errors with the same taxonomy and numeric codes as libirecovery.
//! `code()` matches libirecovery's error numbers, `Display` matches
//! `irecv_strerror()`.

use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrecvError {
    /// Carries a free-form detail message (e.g. the underlying USB error) —
    /// not part of libirecovery's original enum, added so callers don't
    /// lose diagnostic context.
    NoDevice(String),
    OutOfMemory,
    UnableToConnect(String),
    InvalidInput(String),
    FileNotFound,
    UsbUpload(String),
    UsbStatus(String),
    UsbInterface(String),
    UsbConfiguration(String),
    Pipe(String),
    Timeout(String),
    Unsupported(String),
    /// Extension beyond libirecovery: the device isn't in Pwned DFU mode
    /// when an operation requires it.
    PwnedDevice(String),
    Unknown(String),
}

impl IrecvError {
    /// The negative numeric code libirecovery's `irecv_error_t` uses for
    /// this variant, for interop with tools/telemetry that expect it.
    pub fn code(&self) -> i32 {
        match self {
            IrecvError::NoDevice(_) => -1,
            IrecvError::OutOfMemory => -2,
            IrecvError::UnableToConnect(_) => -3,
            IrecvError::InvalidInput(_) => -4,
            IrecvError::FileNotFound => -5,
            IrecvError::UsbUpload(_) => -6,
            IrecvError::UsbStatus(_) => -7,
            IrecvError::UsbInterface(_) => -8,
            IrecvError::UsbConfiguration(_) => -9,
            IrecvError::Pipe(_) => -10,
            IrecvError::Timeout(_) => -11,
            IrecvError::Unsupported(_) => -254,
            IrecvError::PwnedDevice(_) => -100, // outside libirecovery's own range
            IrecvError::Unknown(_) => -255,
        }
    }

    fn detail(&self) -> Option<&str> {
        match self {
            IrecvError::NoDevice(s)
            | IrecvError::UnableToConnect(s)
            | IrecvError::InvalidInput(s)
            | IrecvError::UsbUpload(s)
            | IrecvError::UsbStatus(s)
            | IrecvError::UsbInterface(s)
            | IrecvError::UsbConfiguration(s)
            | IrecvError::Pipe(s)
            | IrecvError::Timeout(s)
            | IrecvError::Unsupported(s)
            | IrecvError::PwnedDevice(s)
            | IrecvError::Unknown(s) => Some(s.as_str()),
            IrecvError::OutOfMemory | IrecvError::FileNotFound => None,
        }
    }

    /// Matches libirecovery's `irecv_strerror()` — the fixed English
    /// description for each error.
    fn strerror(&self) -> &'static str {
        match self {
            IrecvError::NoDevice(_) => "No device found",
            IrecvError::OutOfMemory => "Out of memory",
            IrecvError::UnableToConnect(_) => "Unable to connect to device",
            IrecvError::InvalidInput(_) => "Invalid input",
            IrecvError::FileNotFound => "File not found",
            IrecvError::UsbUpload(_) => "USB upload error",
            IrecvError::UsbStatus(_) => "USB status error",
            IrecvError::UsbInterface(_) => "USB interface error",
            IrecvError::UsbConfiguration(_) => "USB configuration error",
            IrecvError::Pipe(_) => "USB pipe error",
            IrecvError::Timeout(_) => "Timeout error",
            IrecvError::Unsupported(_) => "Operation not supported",
            IrecvError::PwnedDevice(_) => "Device is not in Pwned DFU mode",
            IrecvError::Unknown(_) => "Unknown error",
        }
    }
}

impl fmt::Display for IrecvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.detail() {
            Some(d) if !d.is_empty() => {
                write!(f, "{} (code {}): {}", self.strerror(), self.code(), d)
            }
            _ => write!(f, "{} (code {})", self.strerror(), self.code()),
        }
    }
}

impl std::error::Error for IrecvError {}

/// This crate's standard result type.
pub type Result<T> = core::result::Result<T, IrecvError>;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn codes_match_libirecovery() {
        assert_eq!(IrecvError::NoDevice(String::new()).code(), -1);
        assert_eq!(IrecvError::Timeout(String::new()).code(), -11);
        assert_eq!(IrecvError::Unsupported(String::new()).code(), -254);
        assert_eq!(IrecvError::Unknown(String::new()).code(), -255);
    }
    #[test]
    fn display_has_detail() {
        let e = IrecvError::Timeout("device.open timed out after 8000ms".into());
        let s = e.to_string();
        assert!(s.contains("Timeout error"));
        assert!(s.contains("device.open"));
    }
}
