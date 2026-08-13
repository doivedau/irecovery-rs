# irecovery

[![CI](https://github.com/doivedau/irecovery-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/doivedau/irecovery-rs/actions/workflows/ci.yml)
[![License: LGPL-2.1](https://img.shields.io/badge/license-LGPL--2.1--only-blue.svg)](LICENSE)

Identify Apple devices in **DFU** / **Recovery** mode from Rust: parse the
iBoot USB serial string, classify the USB mode from `(vendor_id,
product_id)`, and look up the device in a 318-entry database by
CPID/BDID, `ProductType`, or `HardwareModel`.

A native Rust port of the identification layer of
[libirecovery](https://github.com/libimobiledevice/libirecovery) (the C
library behind `idevicerestore`, `checkra1n`, and friends). **Pure Rust,
zero dependencies, no I/O** — this crate only answers "what device is
this, what mode is it in." Pair it with a USB stack of your choice (e.g.
[`nusb`](https://docs.rs/nusb) or [`rusb`](https://docs.rs/rusb)) to
actually talk to the device.

## Why

Every DFU/Recovery/restore tool ends up reimplementing the same bit of
libirecovery: read the serial string off a USB descriptor, pull out
`CPID:`/`BDID:`/`ECID:`/`SRTG:`, and map that to a human-readable device
name. This crate is that piece, extracted, tested, and documented, so you
don't have to port it yourself.

## Install

```toml
[dependencies]
irecovery = "0.1"
```

_(Not yet published to crates.io — until then, depend on it via git:)_

```toml
[dependencies]
irecovery = { git = "https://github.com/doivedau/irecovery-rs" }
```

## Usage

```rust
use irecovery::{DeviceInfo, IrecvMode, device_db};

// Read this off your USB stack's device descriptor (iSerialNumber /
// serial_number()) — shown here as a literal for the example.
let serial = "CPID:8030 CPRV:11 CPFM:03 SCEP:01 BDID:04 \
               ECID:000C7C1A2B3C4D5E IBFL:3C SRTG:[iBoot-6723.0.0.6.11]";

let info = DeviceInfo::parse(serial);
assert_eq!(info.cpid, 0x8030);
assert_eq!(info.iboot_version(), Some("iBoot-6723.0.0.6.11"));

let device = device_db::by_cpid_bdid(info.cpid, info.bdid).unwrap();
assert_eq!(device.product_type, "iPhone12,1");
assert_eq!(device.display_name, "iPhone 11");

let mode = IrecvMode::from_pid(0x05ac, 0x1281);
assert!(mode.is_recovery());
```

See [`examples/`](examples) for more:

| Example | Run it | What it shows |
|---|---|---|
| [`parse_serial.rs`](examples/parse_serial.rs) | `cargo run --example parse_serial` | Parsing a DFU/Recovery serial string into `DeviceInfo` |
| [`lookup_device.rs`](examples/lookup_device.rs) | `cargo run --example lookup_device` | The three ways to look a device up in the database |
| [`detect_mode.rs`](examples/detect_mode.rs) | `cargo run --example detect_mode` | Classifying `(vendor_id, product_id)` into an `IrecvMode` |

## What this crate does *not* do

It doesn't open a USB device, send control transfers, or talk to iBoot —
that's libirecovery's `transport`/`client` layer, which depends on a real
USB stack and platform driver access. This crate only covers the
identification logic that sits on top of that. A typical integration
looks like:

```text
your USB stack (nusb/rusb)  ─┬─▶ read serial_number() ─▶ irecovery::DeviceInfo::parse()
                              └─▶ read (vendor_id, product_id) ─▶ irecovery::IrecvMode::from_pid()
```

If a request comes in for a companion crate that does implement the
transport (control transfers, `getenv`/`setenv`, file upload, DFU boot),
open an issue — the [`error`](src/error.rs) module's `IrecvError` already
mirrors libirecovery's `irecv_error_t` so such a crate could report errors
consistently with this one.

## License

`LGPL-2.1-only`, matching [libirecovery](https://github.com/libimobiledevice/libirecovery)
— see [LICENSE](LICENSE). This crate's device database is copied
near-verbatim from libirecovery; see [NOTICE.md](NOTICE.md) for full
attribution and what was and wasn't ported.
