//! Byte-level helpers for PE parsing.

use super::error::PeParseError;

pub(super) fn read_name(data: &[u8], offset: usize) -> Result<String, PeParseError> {
    if offset + 8 > data.len() {
        return Err(PeParseError::UnexpectedEof("section name"));
    }
    let raw = &data[offset..offset + 8];
    let end = raw.iter().position(|&b| b == 0).unwrap_or(raw.len());
    Ok(String::from_utf8_lossy(&raw[..end]).to_string())
}

pub(super) fn read_c_string(data: &[u8], offset: usize) -> Result<String, PeParseError> {
    if offset >= data.len() {
        return Err(PeParseError::UnexpectedEof("string"));
    }
    let mut end = offset;
    while end < data.len() && data[end] != 0 {
        end += 1;
    }
    if end >= data.len() {
        return Err(PeParseError::UnexpectedEof("string terminator"));
    }
    Ok(String::from_utf8_lossy(&data[offset..end]).to_string())
}

pub(super) fn read_utf16_string(data: &[u8], offset: usize) -> Result<String, PeParseError> {
    if offset + 2 > data.len() {
        return Err(PeParseError::UnexpectedEof("utf16 string"));
    }
    let len = read_u16(data, offset)? as usize;
    let bytes_len = len * 2;
    let start = offset + 2;
    if start + bytes_len > data.len() {
        return Err(PeParseError::UnexpectedEof("utf16 string bytes"));
    }
    let mut buf = Vec::with_capacity(len);
    for i in 0..len {
        let pos = start + i * 2;
        buf.push(u16::from_le_bytes([data[pos], data[pos + 1]]));
    }
    Ok(String::from_utf16_lossy(&buf))
}

pub(super) fn read_u8(data: &[u8], offset: usize) -> Result<u8, PeParseError> {
    if offset + 1 > data.len() {
        return Err(PeParseError::UnexpectedEof("u8"));
    }
    Ok(data[offset])
}

pub(super) fn read_u16(data: &[u8], offset: usize) -> Result<u16, PeParseError> {
    if offset + 2 > data.len() {
        return Err(PeParseError::UnexpectedEof("u16"));
    }
    Ok(u16::from_le_bytes([data[offset], data[offset + 1]]))
}

pub(super) fn read_u32(data: &[u8], offset: usize) -> Result<u32, PeParseError> {
    if offset + 4 > data.len() {
        return Err(PeParseError::UnexpectedEof("u32"));
    }
    Ok(u32::from_le_bytes([
        data[offset],
        data[offset + 1],
        data[offset + 2],
        data[offset + 3],
    ]))
}

pub(super) fn read_u16_opt(data: &[u8], offset: usize, limit: usize) -> Option<u16> {
    if offset + 2 > limit {
        return None;
    }
    read_u16(data, offset).ok()
}

pub(super) fn read_u32_opt(data: &[u8], offset: usize, limit: usize) -> Option<u32> {
    if offset + 4 > limit {
        return None;
    }
    read_u32(data, offset).ok()
}

pub(super) fn write_u32(data: &mut [u8], offset: usize, value: u32) -> Result<(), PeParseError> {
    if offset + 4 > data.len() {
        return Err(PeParseError::UnexpectedEof("write u32"));
    }
    let bytes = value.to_le_bytes();
    data[offset..offset + 4].copy_from_slice(&bytes);
    Ok(())
}
