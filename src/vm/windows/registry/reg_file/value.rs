use super::string::parse_string_literal;
use super::super::{RegistryError, RegistryValue};

pub(super) fn parse_registry_value(value_raw: &str) -> Result<RegistryValue, RegistryError> {
    let lowered = value_raw.to_ascii_lowercase();
    if lowered.starts_with('"') {
        return Ok(RegistryValue::String(parse_string_literal(value_raw)?));
    }
    if lowered.starts_with("dword:") {
        let hex = value_raw[6..].trim();
        let value = u32::from_str_radix(hex, 16).map_err(|_| {
            RegistryError::InvalidValue(format!("invalid dword value: {hex}"))
        })?;
        return Ok(RegistryValue::Dword(value));
    }
    if lowered.starts_with("hex") {
        let (kind, data) = split_hex_value(value_raw)?;
        let bytes = parse_hex_bytes(data)?;
        return Ok(match kind.as_deref() {
            Some("2") => RegistryValue::String(decode_utf16(bytes.as_slice())),
            Some("7") => RegistryValue::MultiString(decode_multi_sz(bytes.as_slice())),
            _ => RegistryValue::Binary(bytes),
        });
    }
    Err(RegistryError::InvalidValue(format!(
        "unsupported value: {value_raw}"
    )))
}

fn split_hex_value(value_raw: &str) -> Result<(Option<String>, &str), RegistryError> {
    let mut parts = value_raw.splitn(2, ':');
    let prefix = parts
        .next()
        .ok_or_else(|| RegistryError::InvalidValue("invalid hex prefix".to_string()))?;
    let data = parts
        .next()
        .ok_or_else(|| RegistryError::InvalidValue("missing hex data".to_string()))?;
    let kind = if prefix.starts_with("hex(") {
        let end = prefix.find(')').ok_or_else(|| {
            RegistryError::InvalidValue("invalid hex type".to_string())
        })?;
        Some(prefix[4..end].to_string())
    } else {
        None
    };
    Ok((kind, data))
}

fn parse_hex_bytes(data: &str) -> Result<Vec<u8>, RegistryError> {
    let mut bytes = Vec::new();
    for token in data.split(',') {
        let token = token.trim();
        if token.is_empty() {
            continue;
        }
        let token = token.trim_start_matches('\u{feff}');
        let value = u8::from_str_radix(token, 16).map_err(|_| {
            RegistryError::InvalidValue(format!("invalid hex byte: {token}"))
        })?;
        bytes.push(value);
    }
    Ok(bytes)
}

fn decode_utf16(bytes: &[u8]) -> String {
    let mut units = Vec::with_capacity(bytes.len() / 2);
    for chunk in bytes.chunks(2) {
        let lo = *chunk.first().unwrap_or(&0);
        let hi = *chunk.get(1).unwrap_or(&0);
        units.push(u16::from_le_bytes([lo, hi]));
    }
    let text = String::from_utf16_lossy(&units);
    text.trim_end_matches('\u{0}').to_string()
}

fn decode_multi_sz(bytes: &[u8]) -> Vec<String> {
    let text = decode_utf16(bytes);
    text.split('\u{0}')
        .filter(|part| !part.is_empty())
        .map(|part| part.to_string())
        .collect()
}
