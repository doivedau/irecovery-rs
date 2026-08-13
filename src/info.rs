// SPDX-License-Identifier: LGPL-2.1-only
//! `DeviceInfo` — a typed parse of the iBoot USB serial string an Apple
//! device reports while in DFU/Recovery mode, e.g.:
//!
//! ```text
//! CPID:8030 CPRV:11 CPFM:03 SCEP:01 BDID:04 ECID:000012...
//! SRTG:[iBoot-...] SRNM:[...] PWND:[...]
//! ```
//!
//! Ports libirecovery's `irecv_load_device_info_from_iboot_string`. Pure
//! string parsing — no I/O, so it's trivially unit-testable and works on
//! any target.

/// Device info parsed from the iBoot serial string in DFU/Recovery mode.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct DeviceInfo {
    pub cpid: u32,
    pub cprv: u32,
    pub cpfm: u32,
    pub scep: u32,
    pub bdid: u32,
    pub ecid: u64,
    pub ibfl: u32,
    pub srnm: Option<String>,
    pub imei: Option<String>,
    /// The `SRTG:[...]` tag, typically `"iBoot-x.y.z.a.b"`.
    pub srtg: Option<String>,
    pub serial_string: String,
    pub ap_nonce: Vec<u8>,
    pub sep_nonce: Vec<u8>,
    /// Whether the serial string contains a `PWND:` tag, meaning the
    /// device has an exploited (checkm8) bootrom and is in Pwned DFU —
    /// not part of libirecovery's original `irecv_device_info`, but
    /// commonly needed alongside it.
    pub pwnd: bool,
}

fn find_after<'a>(s: &'a str, tag: &str) -> Option<&'a str> {
    s.find(tag).map(|i| &s[i + tag.len()..])
}

/// Read a hex number right after `tag`, stopping at the first non-hex
/// character — matches `sscanf("%x")`.
fn parse_hex_u64(s: &str, tag: &str) -> Option<u64> {
    let rest = find_after(s, tag)?.trim_start();
    let hex: String = rest.chars().take_while(|c| c.is_ascii_hexdigit()).collect();
    if hex.is_empty() {
        return None;
    }
    u64::from_str_radix(&hex, 16).ok()
}

/// Read the string inside `"TAG:[value]"` — matches
/// `sscanf("TAG:[%s]")` + `strrchr(']')`.
fn parse_bracketed(s: &str, tag_open: &str) -> Option<String> {
    let rest = find_after(s, tag_open)?; // tag_open includes the '[', e.g. "SRNM:["
    let end = rest.find(']')?;
    Some(rest[..end].to_string())
}

/// Ports `irecv_copy_nonce_with_tag_from_buffer`: find `"TAG:"`, read hex
/// digits up to the next whitespace, decode to bytes. `tag` is passed
/// WITHOUT the trailing ':'.
fn parse_nonce(s: &str, tag: &str) -> Vec<u8> {
    let needle = format!("{tag}:");
    let rest = match find_after(s, &needle) {
        Some(r) => r,
        None => return Vec::new(),
    };
    let hex: String = rest.chars().take_while(|c| c.is_ascii_hexdigit()).collect();
    if hex.len() < 2 {
        return Vec::new();
    }
    // Every 2 hex chars = 1 byte (a trailing odd char, if any, is dropped).
    let mut out = Vec::with_capacity(hex.len() / 2);
    let bytes = hex.as_bytes();
    let mut i = 0;
    while i + 1 < bytes.len() {
        let hi = (bytes[i] as char).to_digit(16).unwrap() as u8;
        let lo = (bytes[i + 1] as char).to_digit(16).unwrap() as u8;
        out.push((hi << 4) | lo);
        i += 2;
    }
    out
}

impl DeviceInfo {
    /// Parse an iBoot serial string. Matches
    /// `irecv_load_device_info_from_iboot_string`. Never fails — any tag
    /// that's missing or malformed is simply left at its default value.
    pub fn parse(serial: &str) -> Self {
        let mut info = DeviceInfo {
            serial_string: serial.to_string(),
            // Early iOS 1-era devices don't report a CPID; libirecovery
            // defaults to 0x8900 in that case.
            cpid: 0x8900,
            ..Default::default()
        };

        if let Some(v) = parse_hex_u64(serial, "CPID:") {
            info.cpid = v as u32;
        }
        if let Some(v) = parse_hex_u64(serial, "CPRV:") {
            info.cprv = v as u32;
        }
        if let Some(v) = parse_hex_u64(serial, "CPFM:") {
            info.cpfm = v as u32;
        }
        if let Some(v) = parse_hex_u64(serial, "SCEP:") {
            info.scep = v as u32;
        }
        if let Some(v) = parse_hex_u64(serial, "BDID:") {
            info.bdid = v as u32;
        }
        if let Some(v) = parse_hex_u64(serial, "ECID:") {
            info.ecid = v;
        }
        if let Some(v) = parse_hex_u64(serial, "IBFL:") {
            info.ibfl = v as u32;
        }

        info.srnm = parse_bracketed(serial, "SRNM:[");
        info.imei = parse_bracketed(serial, "IMEI:[");
        info.srtg = parse_bracketed(serial, "SRTG:[");

        info.ap_nonce = parse_nonce(serial, "NONC");
        info.sep_nonce = parse_nonce(serial, "SNON");

        // PWND: (checkm8), case-insensitive.
        info.pwnd = serial.to_ascii_uppercase().contains("PWND:")
            || serial.split_whitespace().any(|tok| {
                tok.split(':')
                    .next()
                    .map(|k| k.eq_ignore_ascii_case("PWND"))
                    .unwrap_or(false)
            });

        info
    }

    /// Whether this device has an exploited (checkm8) bootrom, i.e. is in
    /// Pwned DFU rather than plain DFU.
    pub fn is_pwned(&self) -> bool {
        self.pwnd
    }

    /// The `"iBoot-x.y.z.a.b"` string from `SRTG:[...]`, if present.
    pub fn iboot_version(&self) -> Option<&str> {
        self.srtg.as_deref()
    }

    /// ECID as a 16-digit uppercase hex string (for display/dedup keys) —
    /// `None` if the ECID is 0 (i.e. wasn't present in the serial string).
    pub fn ecid_hex(&self) -> Option<String> {
        if self.ecid == 0 {
            None
        } else {
            Some(format!("{:016X}", self.ecid))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // A representative serial string for an A11 device in Pwned DFU.
    const SAMPLE: &str = "CPID:8015 CPRV:11 CPFM:03 SCEP:01 BDID:0C ECID:000C7C1A2B3C4D5E IBFL:3C \
SRNM:[F2LW...ABC] IMEI:[35...] SRTG:[iBoot-3865.0.0.0.7] NONC:00112233445566778899AABBCCDDEEFF PWND:[checkm8]";

    #[test]
    fn parses_all_fields() {
        let i = DeviceInfo::parse(SAMPLE);
        assert_eq!(i.cpid, 0x8015);
        assert_eq!(i.cprv, 0x11);
        assert_eq!(i.cpfm, 0x03);
        assert_eq!(i.scep, 0x01);
        assert_eq!(i.bdid, 0x0C);
        assert_eq!(i.ecid, 0x000C7C1A2B3C4D5E);
        assert_eq!(i.ibfl, 0x3C);
        assert_eq!(i.srtg.as_deref(), Some("iBoot-3865.0.0.0.7"));
        assert!(i.srnm.as_deref().unwrap().starts_with("F2LW"));
        assert_eq!(i.iboot_version(), Some("iBoot-3865.0.0.0.7"));
        assert!(i.is_pwned());
        assert_eq!(i.ecid_hex().as_deref(), Some("000C7C1A2B3C4D5E"));
    }

    #[test]
    fn parses_ap_nonce_hex() {
        let i = DeviceInfo::parse(SAMPLE);
        assert_eq!(i.ap_nonce.len(), 16);
        assert_eq!(i.ap_nonce[0], 0x00);
        assert_eq!(i.ap_nonce[1], 0x11);
        assert_eq!(i.ap_nonce[15], 0xFF);
    }

    #[test]
    fn default_cpid_when_absent() {
        let i = DeviceInfo::parse("ECID:0000000000000001");
        assert_eq!(i.cpid, 0x8900); // iOS 1-era fallback, matches libirecovery
        assert!(!i.is_pwned());
    }
}
