# Notice

This crate is a Rust port of parts of [**libirecovery**](https://github.com/libimobiledevice/libirecovery),
a C library from the [libimobiledevice](https://github.com/libimobiledevice) project
for communicating with iBoot/iBSS on Apple devices via USB.

## What was ported

- `src/device_db.rs` — the `irecv_devices[]` static device table and the
  `irecv_devices_get_device_by_{product_type,hardware_model}` /
  `irecv_get_device` lookup functions.
- `src/info.rs` — `irecv_load_device_info_from_iboot_string` (parsing the
  DFU/Recovery-mode USB serial string) and
  `irecv_copy_nonce_with_tag_from_buffer`.
- `src/mode.rs` — the `IRECV_K_*_MODE` USB product ID constants.
- `src/error.rs` — the `irecv_error_t` error taxonomy and `irecv_strerror()`
  messages.

This is a from-scratch Rust re-implementation (no C code or bindings are
included), but the device table in particular is copied near-verbatim, so
this crate is licensed the same way as upstream — see [LICENSE](LICENSE)
(LGPL-2.1-only).

**Not ported:** libirecovery's actual USB transport (opening the device,
control/bulk transfers) is not included here — this crate only implements
the identification layer. libirecovery itself depends on
[libimobiledevice-glue](https://github.com/libimobiledevice/libimobiledevice-glue)
and libusb (or IOKit on macOS, WinUSB on Windows) for that; pair this crate
with a Rust USB stack such as [`nusb`](https://docs.rs/nusb) or
[`rusb`](https://docs.rs/rusb) to do the same.

## Copyright

libirecovery's copyright header (`src/libirecovery.c`) lists:

```
Copyright (c) 2011-2023 Nikias Bassen <nikias@gmx.li>
Copyright (c) 2012-2020 Martin Szulecki <martin.szulecki@libimobiledevice.org>
Copyright (c) 2010 Chronic-Dev Team
Copyright (c) 2010 Joshua Hill
Copyright (c) 2008-2011 Nicolas Haunold
```

libirecovery is itself a fork of an older version originally hosted at
openjailbreak.org.

## Trademarks

Apple, iPhone, iPad, iPod, iPod Touch, Apple TV, Apple Watch, Mac, iOS,
iPadOS, tvOS, watchOS, and macOS are trademarks of Apple Inc. This project
is an independent, unofficial library and has not been authorized,
sponsored, or otherwise approved by Apple Inc.
