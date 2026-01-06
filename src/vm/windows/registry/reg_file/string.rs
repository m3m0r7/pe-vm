use super::RegistryError;

pub(super) fn parse_string_literal(value_raw: &str) -> Result<String, RegistryError> {
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
