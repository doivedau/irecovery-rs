//! Look up a device in the 318-entry database three different ways.
//!
//! Run with: `cargo run --example lookup_device`

use irecovery::device_db;

fn main() {
    // 1. By (CPID, BDID) — the pair you get from a DFU/Recovery-mode serial
    //    string via `DeviceInfo::parse` (see `examples/parse_serial.rs`).
    //    This is the lookup you'll use most often.
    let by_ids = device_db::by_cpid_bdid(0x8030, 0x04).expect("iPhone 11 should be in the table");
    println!(
        "by_cpid_bdid(0x8030, 0x04) -> {} ({})",
        by_ids.display_name, by_ids.product_type
    );

    // 2. By ProductType — e.g. from lockdownd's `ProductType` value when
    //    the device is in Normal mode.
    let by_type = device_db::by_product_type("iPhone12,1").unwrap();
    println!(
        "by_product_type(\"iPhone12,1\") -> {}",
        by_type.display_name
    );

    // 3. By HardwareModel — e.g. from a restore/build manifest.
    let by_model = device_db::by_hardware_model("D22AP").unwrap();
    println!("by_hardware_model(\"D22AP\") -> {}", by_model.display_name);

    // Unknown identifiers simply return `None` — no panics, no guessing.
    assert!(device_db::by_cpid_bdid(0xFFFF, 0xFF).is_none());
}
