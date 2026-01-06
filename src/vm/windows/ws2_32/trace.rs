//! Trace helpers for Winsock traffic.

const DATA_PREVIEW_LIMIT: usize = 512;

pub(super) fn trace_net(message: &str) {
    if std::env::var("PE_VM_TRACE_NET").is_ok() || std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!("[pe_vm] {message}");
    }
}

pub(super) fn log_connect(host: &str, port: u16) {
    trace_net(&format!("WSA connect {host}:{port}"));
}

pub(super) fn log_send(bytes: &[u8]) {
    if bytes.is_empty() {
        return;
    }
    let preview = bytes_preview(bytes);
    trace_net(&format!("WSA send {} bytes: {preview}", bytes.len()));
}

fn bytes_preview(bytes: &[u8]) -> String {
    let end = bytes.len().min(DATA_PREVIEW_LIMIT);
    let slice = &bytes[..end];
    if is_printable_ascii(slice) {
        format!("{:?}", String::from_utf8_lossy(slice))
    } else {
        let mut out = String::new();
        for byte in slice {
            out.push_str(&format!("{:02X}", byte));
        }
        if bytes.len() > DATA_PREVIEW_LIMIT {
            out.push_str("...");
        }
        out
    }
}

fn is_printable_ascii(bytes: &[u8]) -> bool {
    bytes.iter().all(|value| matches!(value, b'\t' | b'\n' | b'\r' | 0x20..=0x7E))
}
