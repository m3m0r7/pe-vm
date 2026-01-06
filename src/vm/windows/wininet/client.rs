//! Minimal HTTP client used by WinINet stubs.

use std::io::{Read, Write};
use std::net::TcpStream;

use super::trace::{log_http_request, log_http_response};
use super::types::Response;

pub(super) fn send_http_request(
    host: &str,
    port: u16,
    method: &str,
    path: &str,
    user_agent: &str,
    headers: &str,
    body: &[u8],
) -> Result<Response, std::io::Error> {
    let mut stream = TcpStream::connect((host, port))?;
    let mut request = String::new();
    request.push_str(&format!("{method} {path} HTTP/1.1\r\n"));
    request.push_str(&format!("Host: {host}\r\n"));
    request.push_str("Connection: close\r\n");
    if !user_agent.is_empty() {
        request.push_str(&format!("User-Agent: {user_agent}\r\n"));
    }
    if !headers.is_empty() {
        request.push_str(headers);
        if !headers.ends_with("\r\n") {
            request.push_str("\r\n");
        }
    }
    if !body.is_empty() && !headers.to_ascii_lowercase().contains("content-length") {
        request.push_str(&format!("Content-Length: {}\r\n", body.len()));
    }
    request.push_str("\r\n");

    log_http_request(host, port, &request, body);

    stream.write_all(request.as_bytes())?;
    if !body.is_empty() {
        stream.write_all(body)?;
    }

    let mut response_bytes = Vec::new();
    stream.read_to_end(&mut response_bytes)?;
    let response = parse_http_response(&response_bytes);
    log_http_response(response.status, response.body.len());
    Ok(response)
}

fn parse_http_response(bytes: &[u8]) -> Response {
    let (raw_headers, body) = split_headers(bytes);
    let header_text = String::from_utf8_lossy(raw_headers).to_string();
    let mut lines = header_text.lines();
    let status_line = lines.next().unwrap_or("");
    let status = status_line
        .split_whitespace()
        .nth(1)
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(0);
    let mut headers = Vec::new();
    for line in lines {
        if let Some((key, value)) = line.split_once(':') {
            headers.push((key.trim().to_string(), value.trim().to_string()));
        }
    }
    let mut body_bytes = body.to_vec();
    if headers
        .iter()
        .any(|(key, value)| key.eq_ignore_ascii_case("transfer-encoding") && value.eq_ignore_ascii_case("chunked"))
    {
        body_bytes = decode_chunked(&body_bytes);
    }
    Response {
        status,
        body: body_bytes,
        raw_headers: header_text,
    }
}

fn split_headers(bytes: &[u8]) -> (&[u8], &[u8]) {
    for idx in 0..bytes.len().saturating_sub(3) {
        if bytes[idx..].starts_with(b"\r\n\r\n") {
            return (&bytes[..idx], &bytes[idx + 4..]);
        }
    }
    (bytes, &[])
}

fn decode_chunked(bytes: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut cursor = 0usize;
    while cursor < bytes.len() {
        let Some(line_end) = find_crlf(bytes, cursor) else {
            break;
        };
        let line = &bytes[cursor..line_end];
        let size_str = String::from_utf8_lossy(line).trim().to_string();
        let Ok(size) = usize::from_str_radix(size_str.split(';').next().unwrap_or("0"), 16) else {
            break;
        };
        cursor = line_end + 2;
        if size == 0 {
            break;
        }
        if cursor + size > bytes.len() {
            break;
        }
        out.extend_from_slice(&bytes[cursor..cursor + size]);
        cursor += size + 2;
    }
    out
}

fn find_crlf(bytes: &[u8], start: usize) -> Option<usize> {
    let mut idx = start;
    while idx + 1 < bytes.len() {
        if bytes[idx] == b'\r' && bytes[idx + 1] == b'\n' {
            return Some(idx);
        }
        idx += 1;
    }
    None
}
