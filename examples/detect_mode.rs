//! Classify a device's DFU/Recovery mode from its USB `(vendor_id,
//! product_id)`, as reported by any USB stack's device descriptor.
//!
//! Run with: `cargo run --example detect_mode`

use irecovery::mode::APPLE_VID;
use irecovery::IrecvMode;

fn main() {
    for (label, vid, pid) in [
        ("Recovery mode", APPLE_VID, 0x1281),
        ("DFU mode", APPLE_VID, 0x1227),
        ("Port DFU (A12+ checkm8/pongoOS path)", APPLE_VID, 0x1338),
        ("Some other USB device", 0x1234, 0x5678),
    ] {
        let mode = IrecvMode::from_pid(vid, pid);
        println!(
            "{label}: label={:?} is_recovery={} is_dfu={}",
            mode.label(),
            mode.is_recovery(),
            mode.is_dfu(),
        );
    }

    // Pwned DFU can't be told apart from plain DFU by PID alone — it's
    // inferred from the serial string's `PWND:` tag instead. See
    // `DeviceInfo::is_pwned` in `examples/parse_serial.rs`.
}
