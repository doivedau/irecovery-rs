//! Parse the USB serial string an Apple device reports while in
//! DFU/Recovery mode into a typed `DeviceInfo`.
//!
//! Run with: `cargo run --example parse_serial`
//!
//! In a real program, read this string from your USB stack's device
//! descriptor (`iSerialNumber` / `serial_number()`) instead of a literal —
//! see `examples/with_a_usb_stack.rs` for the shape of that integration.

use irecovery::DeviceInfo;

fn main() {
    // A representative serial string for an iPhone 11 (A13) in Pwned DFU.
    let serial = "CPID:8030 CPRV:11 CPFM:03 SCEP:01 BDID:04 \
                   ECID:000C7C1A2B3C4D5E IBFL:3C \
                   SRTG:[iBoot-6723.0.0.6.11] PWND:[checkm8]";

    let info = DeviceInfo::parse(serial);

    println!("CPID: {:#06x}", info.cpid);
    println!("BDID: {:#04x}", info.bdid);
    println!(
        "ECID: {}",
        info.ecid_hex().unwrap_or_else(|| "unknown".into())
    );
    println!(
        "iBoot version: {}",
        info.iboot_version().unwrap_or("unknown")
    );
    println!("Pwned DFU (checkm8): {}", info.is_pwned());
}
