pub(super) fn decode_registry_text(bytes: &[u8]) -> String {
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
