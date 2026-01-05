//! Windows .reg file parsing helpers.

use std::path::Path;

use super::{Registry, RegistryError, RegistryHive, RegistryMergeMode, RegistryValue};

pub(super) fn load_from_registry(path: impl AsRef<Path>) -> Result<Registry, RegistryError> {
    let bytes = std::fs::read(path)?;
    let contents = decode_registry_text(&bytes);
    let mut registry = Registry::with_defaults();
    merge_reg_str(&mut registry, &contents, RegistryMergeMode::Overwrite)?;
    Ok(registry)
}

fn merge_reg_str(
    registry: &mut Registry,
    contents: &str,
    mode: RegistryMergeMode,
) -> Result<(), RegistryError> {
    let mut current_hive: Option<RegistryHive> = None;
    let mut current_path: Vec<String> = Vec::new();
    let mut pending = String::new();

    for raw_line in contents.lines() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with("Windows Registry Editor") {
            continue;
        }
        if line.starts_with(';') || line.starts_with('#') {
            continue;
        }
        if line.ends_with('\\') {
            pending.push_str(line.trim_end_matches('\\'));
            continue;
        }
        let mut line = line.to_string();
        if !pending.is_empty() {
            pending.push_str(&line);
            line = pending.clone();
            pending.clear();
        }
        if line.starts_with('[') && line.ends_with(']') {
            let key = &line[1..line.len() - 1];
            let (hive, path) = parse_key_path(key)?;
            current_hive = Some(hive);
            current_path = path;
            continue;
        }
        let Some(hive) = current_hive else {
            continue;
        };
        if line.starts_with('[') {
            continue;
        }
        let (name, value) = parse_value_line(&line)?;
        registry.apply_value(hive, &current_path, name.as_deref(), value, mode);
    }
    Ok(())
}

fn parse_key_path(line: &str) -> Result<(RegistryHive, Vec<String>), RegistryError> {
    let normalized = line.trim().replace('/', "\\");
    let mut parts = normalized.split('\\');
    let hive_name = parts
        .next()
        .ok_or_else(|| RegistryError::InvalidKey("missing hive".to_string()))?;
    let hive = RegistryHive::parse(hive_name)
        .ok_or_else(|| RegistryError::InvalidHive(hive_name.to_string()))?;
    let path = parts.filter(|part| !part.is_empty()).map(|part| part.to_string()).collect();
    Ok((hive, path))
}

fn parse_value_line(line: &str) -> Result<(Option<String>, RegistryValue), RegistryError> {
    let mut parts = line.splitn(2, '=');
    let name_raw = parts
        .next()
        .ok_or_else(|| RegistryError::InvalidValue("missing value name".to_string()))?
        .trim();
    let value_raw = parts
        .next()
        .ok_or_else(|| RegistryError::InvalidValue("missing value data".to_string()))?
        .trim();

    let name = if name_raw == "@" || name_raw.eq_ignore_ascii_case("(default)") {
        None
    } else {
        Some(parse_string_literal(name_raw)?)
    };

    let value = parse_registry_value(value_raw)?;
    Ok((name, value))
}

fn parse_registry_value(value_raw: &str) -> Result<RegistryValue, RegistryError> {
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

fn decode_registry_text(bytes: &[u8]) -> String {
    if bytes.starts_with(&[0xFF, 0xFE]) {
        return decode_utf16_le(&bytes[2..]);
    }
    if bytes.starts_with(&[0xFE, 0xFF]) {
        return decode_utf16_be(&bytes[2..]);
    }
    if looks_like_utf16_le(bytes) {
        return decode_utf16_le(bytes);
    }
    if looks_like_utf16_be(bytes) {
        return decode_utf16_be(bytes);
    }
    String::from_utf8_lossy(bytes).to_string()
}

fn looks_like_utf16_le(bytes: &[u8]) -> bool {
    let sample_len = bytes.len().min(64);
    let mut zeros = 0usize;
    let mut total = 0usize;
    for idx in (1..sample_len).step_by(2) {
        total += 1;
        if bytes[idx] == 0 {
            zeros += 1;
        }
    }
    total > 0 && zeros * 3 >= total * 2
}

fn looks_like_utf16_be(bytes: &[u8]) -> bool {
    let sample_len = bytes.len().min(64);
    let mut zeros = 0usize;
    let mut total = 0usize;
    for idx in (0..sample_len).step_by(2) {
        total += 1;
        if bytes[idx] == 0 {
            zeros += 1;
        }
    }
    total > 0 && zeros * 3 >= total * 2
}

fn decode_utf16_le(bytes: &[u8]) -> String {
    let mut units = Vec::with_capacity(bytes.len() / 2);
    for chunk in bytes.chunks(2) {
        let lo = *chunk.first().unwrap_or(&0);
        let hi = *chunk.get(1).unwrap_or(&0);
        units.push(u16::from_le_bytes([lo, hi]));
    }
    String::from_utf16_lossy(&units)
}

fn decode_utf16_be(bytes: &[u8]) -> String {
    let mut units = Vec::with_capacity(bytes.len() / 2);
    for chunk in bytes.chunks(2) {
        let hi = *chunk.first().unwrap_or(&0);
        let lo = *chunk.get(1).unwrap_or(&0);
        units.push(u16::from_be_bytes([hi, lo]));
    }
    String::from_utf16_lossy(&units)
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

fn parse_string_literal(value_raw: &str) -> Result<String, RegistryError> {
    let trimmed = value_raw.trim();
    if !trimmed.starts_with('"') || !trimmed.ends_with('"') {
        return Err(RegistryError::InvalidValue(
            "expected quoted string".to_string(),
        ));
    }
    let inner = &trimmed[1..trimmed.len() - 1];
    Ok(unescape_reg_string(inner))
}

fn unescape_reg_string(input: &str) -> String {
    let mut out = String::new();
    let mut chars = input.chars();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            if let Some(next) = chars.next() {
                match next {
                    '\\' => out.push('\\'),
                    '"' => out.push('"'),
                    'n' => out.push('\n'),
                    'r' => out.push('\r'),
                    _ => {
                        out.push('\\');
                        out.push(next);
                    }
                }
            } else {
                out.push('\\');
            }
        } else {
            out.push(ch);
        }
    }
    out
}
