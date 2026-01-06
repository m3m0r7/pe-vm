use super::string::parse_string_literal;
use super::value::parse_registry_value;
use super::super::{Registry, RegistryError, RegistryHive, RegistryMergeMode};

pub(super) fn merge_reg_str(
    registry: &mut Registry,
    contents: &str,
    mode: RegistryMergeMode,
) -> Result<(), RegistryError> {
    let mut current_hive: Option<RegistryHive> = None;
    let mut current_path: Vec<String> = Vec::new();
    let mut pending = String::new();
    let mut pending_value: Option<PendingValue> = None;
    let max_pending_len = 256 * 1024;

    for raw_line in contents.lines() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(pending) = pending_value.as_mut() {
            if line.starts_with('[') && line.ends_with(']') {
                pending_value = None;
            } else if let Ok((name_raw, _)) = split_value_line(line) {
                if is_registry_name(name_raw) {
                    pending_value = None;
                } else {
                    pending.value.push('\n');
                    pending.value.push_str(line);
                    if pending.value.len() > max_pending_len {
                        pending_value = None;
                        continue;
                    }
                    if is_complete_quoted_value(&pending.value) {
                        let value = parse_registry_value(&pending.value)?;
                        registry.apply_value(
                            pending.hive,
                            &pending.path,
                            pending.name.as_deref(),
                            value,
                            mode,
                        );
                        pending_value = None;
                    }
                    continue;
                }
            } else {
                pending.value.push('\n');
                pending.value.push_str(line);
                if pending.value.len() > max_pending_len {
                    pending_value = None;
                    continue;
                }
                if is_complete_quoted_value(&pending.value) {
                    let value = parse_registry_value(&pending.value)?;
                    registry.apply_value(
                        pending.hive,
                        &pending.path,
                        pending.name.as_deref(),
                        value,
                        mode,
                    );
                    pending_value = None;
                }
                continue;
            }
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
        let Ok((name_raw, value_raw)) = split_value_line(&line) else {
            continue;
        };
        let Ok(name) = parse_value_name(name_raw) else {
            continue;
        };
        if value_raw.starts_with('"') && !is_complete_quoted_value(value_raw) {
            pending_value = Some(PendingValue {
                hive,
                path: current_path.clone(),
                name,
                value: value_raw.to_string(),
            });
            continue;
        }
        let value = parse_registry_value(value_raw)?;
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

fn parse_value_name(name_raw: &str) -> Result<Option<String>, RegistryError> {
    let name_raw = name_raw.trim();
    if name_raw == "@" || name_raw.eq_ignore_ascii_case("(default)") {
        Ok(None)
    } else {
        Ok(Some(parse_string_literal(name_raw)?))
    }
}

fn is_registry_name(name_raw: &str) -> bool {
    let name_raw = name_raw.trim();
    name_raw == "@"
        || name_raw.eq_ignore_ascii_case("(default)")
        || (name_raw.starts_with('"') && name_raw.ends_with('"'))
}

fn split_value_line(line: &str) -> Result<(&str, &str), RegistryError> {
    let mut in_quotes = false;
    let mut escaped = false;
    for (idx, ch) in line.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' {
            escaped = true;
            continue;
        }
        if ch == '"' {
            in_quotes = !in_quotes;
            continue;
        }
        if ch == '=' && !in_quotes {
            let name = &line[..idx];
            let value = &line[idx + 1..];
            return Ok((name, value));
        }
    }
    Err(RegistryError::InvalidValue(
        "missing value separator".to_string(),
    ))
}

fn is_complete_quoted_value(value_raw: &str) -> bool {
    let trimmed = value_raw.trim_end();
    if !trimmed.starts_with('"') || !trimmed.ends_with('"') {
        return false;
    }
    let mut backslashes = 0usize;
    for ch in trimmed[..trimmed.len().saturating_sub(1)].chars().rev() {
        if ch == '\\' {
            backslashes += 1;
        } else {
            break;
        }
    }
    backslashes % 2 == 0
}

struct PendingValue {
    hive: RegistryHive,
    path: Vec<String>,
    name: Option<String>,
    value: String,
}
