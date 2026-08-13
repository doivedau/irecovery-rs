// SPDX-License-Identifier: LGPL-2.1-only
//! Rust port of [libirecovery]'s Apple DFU/Recovery-mode device
//! identification: parse the iBoot USB serial string, classify the USB
//! mode from `(vendor_id, product_id)`, and look up the device in a
//! 318-entry database by CPID/BDID, `ProductType`, or `HardwareModel`.
//!
//! Pure Rust, **zero dependencies**, no I/O — this crate only understands
//! the *identification* layer (what device is this, what mode is it in).
//! It does not talk to USB hardware; pair it with a USB stack of your
//! choice (e.g. [`nusb`] or [`rusb`]) to actually open the device and read
//! its serial string.
//!
//! # Quick start
//!
//! ```
//! use irecovery::{DeviceInfo, IrecvMode, device_db};
//!
//! // Serial string as reported by a device in DFU/Recovery mode (read it
//! // via your USB stack's device descriptor `iSerialNumber`).
//! let serial = "CPID:8030 CPRV:11 CPFM:03 SCEP:01 BDID:04 \
//!               ECID:000C7C1A2B3C4D5E IBFL:3C SRTG:[iBoot-6723.0.0.6.11]";
//!
//! let info = DeviceInfo::parse(serial);
//! assert_eq!(info.cpid, 0x8030);
//! assert_eq!(info.iboot_version(), Some("iBoot-6723.0.0.6.11"));
//!
//! let device = device_db::by_cpid_bdid(info.cpid, info.bdid).unwrap();
//! assert_eq!(device.product_type, "iPhone12,1");
//! assert_eq!(device.display_name, "iPhone 11");
//!
//! let mode = IrecvMode::from_pid(0x05ac, 0x1281);
//! assert!(mode.is_recovery());
//! ```
//!
//! # License
//!
//! `device_db`'s device table is ported near-verbatim from libirecovery's
//! `irecv_devices[]`, so this crate is distributed under the same license
//! as upstream: **LGPL-2.1-only**. See `LICENSE` and `NOTICE.md`.
//!
//! [libirecovery]: https://github.com/libimobiledevice/libirecovery
//! [`nusb`]: https://docs.rs/nusb
//! [`rusb`]: https://docs.rs/rusb

pub mod device_db;
pub mod error;
pub mod info;
pub mod mode;

pub use device_db::{by_cpid_bdid, by_hardware_model, by_product_type, IrecvDevice};
pub use error::IrecvError;
pub use info::DeviceInfo;
pub use mode::IrecvMode;
