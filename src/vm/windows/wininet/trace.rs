//! Trace helpers for WinINet traffic.

const BODY_PREVIEW_LIMIT: usize = 512;

pub(super) fn trace_net(message: &str) {
    if std::env::var("PE_VM_TRACE_NET").is_ok() || std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!("[pe_vm] {message}");
    }
}

pub(super) fn log_http_request(host: &str, port: u16, request: &str, body: &[u8]) {
    trace_net(&format!("WinInet request to {host}:{port}"));
    if !request.is_empty() {
        trace_net(&format!("WinInet request headers:\n{request}"));
    }
    if body.is_empty() {
        return;
    }
    let preview = body_preview(body);
    trace_net(&format!("WinInet request body {} bytes: {preview}", body.len()));
}

pub(super) fn log_http_response(status: u16, body: &[u8]) {
    trace_net(&format!(
        "WinInet response status={status} body_len={}",
        body.len()
    ));
    if body.is_empty() {
        return;
    }
    if std::env::var("PE_VM_TRACE_NET_BODY").is_ok() {
        let preview = body_preview(body);
        trace_net(&format!(
            "WinInet response body {} bytes: {preview}",
            body.len()
        ));
    }
}

fn body_preview(body: &[u8]) -> String {
    let end = body.len().min(BODY_PREVIEW_LIMIT);
    let slice = &body[..end];
    if is_printable_ascii(slice) {
        format!("{:?}", String::from_utf8_lossy(slice))
    } else {
        let mut out = String::new();
        for byte in slice {
            out.push_str(&format!("{:02X}", byte));
        }
        if body.len() > BODY_PREVIEW_LIMIT {
            out.push_str("...");
        }
        out
    }
}

fn is_printable_ascii(bytes: &[u8]) -> bool {
    bytes.iter().all(|value| matches!(value, b'\t' | b'\n' | b'\r' | 0x20..=0x7E))
}
