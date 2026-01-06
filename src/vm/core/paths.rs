use super::super::*;

impl Vm {
    pub fn map_path(&self, path: &str) -> String {
        if self.config.paths_ref().is_empty() {
            return path.to_string();
        }
        let is_windows = matches!(self.config.os_value(), Os::Windows);
        let input = normalize_path(path, is_windows);
        let mut best_match: Option<(&String, &String)> = None;
        for (guest_prefix, host_prefix) in self.config.paths_ref() {
            let guest = normalize_path(guest_prefix, is_windows);
            if path_starts_with(&input, &guest, is_windows) {
                let is_better = best_match
                    .as_ref()
                    .map(|(best, _)| guest.len() > normalize_path(best, is_windows).len())
                    .unwrap_or(true);
                if is_better {
                    best_match = Some((guest_prefix, host_prefix));
                }
            }
        }
        let Some((guest_prefix, host_prefix)) = best_match else {
            return input;
        };
        let guest = normalize_path(guest_prefix, is_windows);
        let remainder = input.get(guest.len()..).unwrap_or("");
        let mut remainder = remainder.trim_start_matches(['\\', '/']).to_string();
        if remainder.is_empty() {
            return host_prefix.clone();
        }
        let sep = if host_prefix.contains('\\') { '\\' } else { '/' };
        if sep == '/' {
            remainder = remainder.replace('\\', "/");
        } else {
            remainder = remainder.replace('/', "\\");
        }
        let needs_sep = !host_prefix.ends_with(['\\', '/']);
        if needs_sep {
            format!("{host_prefix}{sep}{remainder}")
        } else {
            format!("{host_prefix}{remainder}")
        }
    }
}

fn normalize_path(path: &str, windows: bool) -> String {
    if !windows {
        return path.to_string();
    }
    let mut normalized = path.replace('/', "\\");
    if normalized.starts_with("\\\\") {
        return normalized;
    }
    let has_drive = normalized
        .as_bytes()
        .get(1)
        .copied()
        .map(|value| value == b':')
        .unwrap_or(false);
    if !has_drive && normalized.starts_with('\\') {
        normalized = format!("C:{normalized}");
    }
    normalized
}

fn path_starts_with(path: &str, prefix: &str, windows: bool) -> bool {
    if windows {
        path.to_ascii_lowercase()
            .starts_with(&prefix.to_ascii_lowercase())
    } else {
        path.starts_with(prefix)
    }
}
