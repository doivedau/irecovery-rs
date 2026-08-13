// SPDX-License-Identifier: LGPL-2.1-only
//! `IrecvMode` — the USB mode an Apple device is in while in the
//! DFU/Recovery/WTF family, mirroring the `IRECV_K_*_MODE` constants from
//! libirecovery, plus the checkm8-era "Port DFU" variant (A12+ SoCs).
//!
//! This only classifies the DFU/Recovery/WTF family by USB `(vendor_id,
//! product_id)`. Normal mode (regular usbmux/lockdownd) isn't part of this
//! family and isn't detected here — a caller that also watches for Normal
//! mode should treat any `Unknown` result as "not DFU/Recovery, check
//! elsewhere." Pwned DFU can't be told apart from a fresh DFU device by PID
//! alone; it's inferred from the serial string instead (see
//! [`crate::info::DeviceInfo::is_pwned`]).

pub const APPLE_VID: u16 = 0x05ac;

// Recovery/DFU family product IDs (matches libirecovery).
pub const PID_WTF: u16 = 0x1222;
pub const PID_DFU: u16 = 0x1227;
pub const PID_RECOVERY_1: u16 = 0x1280;
pub const PID_RECOVERY_2: u16 = 0x1281; // most common
pub const PID_RECOVERY_3: u16 = 0x1282;
pub const PID_RECOVERY_4: u16 = 0x1283;
// "Port DFU" — the DFU variant used by A12+ SoCs (checkm8/pongoOS path),
// outside the classic PID range above.
pub const PID_PORT_DFU: u16 = 0x1338;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrecvMode {
    Recovery1,
    Recovery2,
    Recovery3,
    Recovery4,
    Dfu,
    Wtf,
    PortDfu,
    /// Inferred from the serial string's `PWND:` tag (checkm8 exploited),
    /// not from the USB PID alone — set by higher-level code after reading
    /// the serial (see [`crate::info::DeviceInfo::is_pwned`]).
    PwnedDfu,
    /// Regular usbmux/lockdownd mode. Not detected by [`IrecvMode::from_pid`]
    /// (that only recognizes the DFU/Recovery/WTF family) — provided so
    /// callers have a place to put this once they've confirmed it another
    /// way (e.g. lockdownd responding).
    Normal,
    Unknown,
}

impl IrecvMode {
    /// Classify a device's DFU/Recovery/WTF mode from its USB `(vendor_id,
    /// product_id)`. Returns `Unknown` for anything outside that family
    /// (including Normal mode — see the `Normal` variant's docs).
    pub fn from_pid(vendor_id: u16, product_id: u16) -> IrecvMode {
        if vendor_id != APPLE_VID {
            return IrecvMode::Unknown;
        }
        match product_id {
            PID_RECOVERY_1 => IrecvMode::Recovery1,
            PID_RECOVERY_2 => IrecvMode::Recovery2,
            PID_RECOVERY_3 => IrecvMode::Recovery3,
            PID_RECOVERY_4 => IrecvMode::Recovery4,
            PID_DFU => IrecvMode::Dfu,
            PID_WTF => IrecvMode::Wtf,
            PID_PORT_DFU => IrecvMode::PortDfu,
            _ => IrecvMode::Unknown,
        }
    }

    /// Whether this is any Recovery mode variant (1..4).
    pub fn is_recovery(self) -> bool {
        matches!(
            self,
            IrecvMode::Recovery1
                | IrecvMode::Recovery2
                | IrecvMode::Recovery3
                | IrecvMode::Recovery4
        )
    }

    /// Whether this mode speaks the DFU protocol (DFU/WTF/PortDfu/PwnedDfu).
    pub fn is_dfu(self) -> bool {
        matches!(
            self,
            IrecvMode::Dfu | IrecvMode::Wtf | IrecvMode::PortDfu | IrecvMode::PwnedDfu
        )
    }

    /// Short lowercase label, handy for logs/UI/telemetry.
    pub fn label(self) -> &'static str {
        match self {
            IrecvMode::Recovery1
            | IrecvMode::Recovery2
            | IrecvMode::Recovery3
            | IrecvMode::Recovery4 => "recovery",
            IrecvMode::Dfu | IrecvMode::Wtf | IrecvMode::PortDfu => "dfu",
            IrecvMode::PwnedDfu => "pwned_dfu",
            IrecvMode::Normal => "normal",
            IrecvMode::Unknown => "unknown",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pid_mapping() {
        assert_eq!(IrecvMode::from_pid(APPLE_VID, 0x1281), IrecvMode::Recovery2);
        assert_eq!(IrecvMode::from_pid(APPLE_VID, 0x1227), IrecvMode::Dfu);
        assert_eq!(IrecvMode::from_pid(APPLE_VID, 0x1338), IrecvMode::PortDfu);
        assert_eq!(IrecvMode::from_pid(0x1234, 0x1281), IrecvMode::Unknown);
    }
    #[test]
    fn labels() {
        assert_eq!(IrecvMode::Recovery2.label(), "recovery");
        assert_eq!(IrecvMode::PortDfu.label(), "dfu");
        assert_eq!(IrecvMode::PwnedDfu.label(), "pwned_dfu");
    }
    #[test]
    fn family_predicates() {
        assert!(IrecvMode::Recovery3.is_recovery());
        assert!(IrecvMode::PwnedDfu.is_dfu());
        assert!(!IrecvMode::Recovery1.is_dfu());
    }
}
