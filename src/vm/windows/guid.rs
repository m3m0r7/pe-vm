//! GUID parsing and formatting helpers.

// Parse a GUID string like {XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX}.
pub(crate) fn parse_guid(input: &str) -> Option<[u8; 16]> {
    let trimmed = input.trim().trim_start_matches('{').trim_end_matches('}');
    let parts: Vec<&str> = trimmed.split('-').collect();
    if parts.len() != 5 {
        return None;
    }

    let data1 = u32::from_str_radix(parts[0], 16).ok()?;
    let data2 = u16::from_str_radix(parts[1], 16).ok()?;
    let data3 = u16::from_str_radix(parts[2], 16).ok()?;

    if parts[3].len() != 4 || parts[4].len() != 12 {
        return None;
    }
    let mut data4 = [0u8; 8];
    for (i, slot) in data4.iter_mut().take(2).enumerate() {
        let start = i * 2;
        *slot = u8::from_str_radix(&parts[3][start..start + 2], 16).ok()?;
    }
    for (i, slot) in data4.iter_mut().skip(2).take(6).enumerate() {
        let start = i * 2;
        *slot = u8::from_str_radix(&parts[4][start..start + 2], 16).ok()?;
    }

    let mut out = [0u8; 16];
    out[0..4].copy_from_slice(&data1.to_le_bytes());
    out[4..6].copy_from_slice(&data2.to_le_bytes());
    out[6..8].copy_from_slice(&data3.to_le_bytes());
    out[8..16].copy_from_slice(&data4);
    Some(out)
}

// Format GUID bytes into {XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX}.
pub(crate) fn format_guid(bytes: &[u8; 16]) -> String {
    let data1 = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    let data2 = u16::from_le_bytes([bytes[4], bytes[5]]);
    let data3 = u16::from_le_bytes([bytes[6], bytes[7]]);
    let data4 = &bytes[8..16];
    format!(
        "{{{data1:08X}-{data2:04X}-{data3:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}}}",
        data4[0], data4[1], data4[2], data4[3], data4[4], data4[5], data4[6], data4[7]
    )
}
