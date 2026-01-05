//! WinINet stubs with a minimal HTTP client implementation.

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Mutex, OnceLock};

use crate::vm::Vm;

const HANDLE_BASE: u32 = 0x7100_0000;
const ERROR_ACCESS_DENIED: u32 = 5;
const ERROR_INTERNET_NAME_NOT_RESOLVED: u32 = 12007;
const ERROR_INTERNET_CONNECTION_ABORTED: u32 = 12030;

const HTTP_QUERY_CONTENT_LENGTH: u32 = 5;
const HTTP_QUERY_STATUS_CODE: u32 = 19;
const HTTP_QUERY_RAW_HEADERS_CRLF: u32 = 22;
const HTTP_QUERY_FLAG_NUMBER: u32 = 0x2000_0000;

#[derive(Default)]
struct WinInetStore {
    next_handle: u32,
    handles: HashMap<u32, InternetHandle>,
}

#[derive(Clone)]
struct Session {
    user_agent: String,
}

#[derive(Clone)]
struct Connection {
    host: String,
    port: u16,
    user_agent: String,
}

#[derive(Clone)]
struct Request {
    connection: u32,
    method: String,
    path: String,
    response: Option<Response>,
    cursor: usize,
}

#[derive(Clone)]
struct Response {
    status: u16,
    body: Vec<u8>,
    raw_headers: String,
}

#[derive(Clone)]
enum InternetHandle {
    Session(Session),
    Connection(Connection),
    Request(Request),
}

fn store() -> &'static Mutex<WinInetStore> {
    static STORE: OnceLock<Mutex<WinInetStore>> = OnceLock::new();
    STORE.get_or_init(|| {
        Mutex::new(WinInetStore {
            next_handle: HANDLE_BASE,
            handles: HashMap::new(),
        })
    })
}

fn alloc_handle(handle: InternetHandle) -> u32 {
    let mut guard = store().lock().expect("wininet store");
    if guard.next_handle == 0 {
        guard.next_handle = HANDLE_BASE;
    }
    let handle_id = guard.next_handle;
    guard.next_handle = guard.next_handle.wrapping_add(1);
    guard.handles.insert(handle_id, handle);
    handle_id
}

fn remove_handle(handle_id: u32) -> bool {
    let mut guard = store().lock().expect("wininet store");
    guard.handles.remove(&handle_id).is_some()
}

// Register the minimal WinINet entry points used by JVLink.
pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall("WININET.dll", "InternetOpenA", crate::vm::stdcall_args(5), internet_open_a);
    vm.register_import_stdcall("WININET.dll", "InternetConnectA", crate::vm::stdcall_args(8), internet_connect_a);
    vm.register_import_stdcall("WININET.dll", "HttpOpenRequestA", crate::vm::stdcall_args(8), http_open_request_a);
    vm.register_import_stdcall("WININET.dll", "HttpSendRequestA", crate::vm::stdcall_args(5), http_send_request_a);
    vm.register_import_stdcall("WININET.dll", "HttpQueryInfoA", crate::vm::stdcall_args(5), http_query_info_a);
    vm.register_import_stdcall("WININET.dll", "InternetReadFile", crate::vm::stdcall_args(4), internet_read_file);
    vm.register_import_stdcall("WININET.dll", "InternetSetOptionA", crate::vm::stdcall_args(4), internet_set_option_a);
    vm.register_import_stdcall("WININET.dll", "InternetCloseHandle", crate::vm::stdcall_args(1), internet_close_handle);
}

fn internet_open_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    // InternetOpenA(hAgent, accessType, proxy, bypass, flags).
    let agent_ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let agent = read_c_string(vm, agent_ptr);
    let handle = InternetHandle::Session(Session { user_agent: agent });
    alloc_handle(handle)
}

fn internet_connect_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    // InternetConnectA(hInternet, server, port, user, pass, service, flags, context).
    let session_handle = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let server_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let mut port = vm.read_u32(stack_ptr + 12).unwrap_or(0) as u16;
    let mut server = normalize_host(&read_c_string(vm, server_ptr));
    if port == 0 {
        if let Some((host, port_str)) = server.rsplit_once(':') {
            if let Ok(parsed) = port_str.parse::<u16>() {
                server = host.to_string();
                port = parsed;
            }
        }
    }
    if server.is_empty() {
        if let Some(fallback) = network_fallback_host(vm) {
            server = fallback.to_string();
        }
    }
    if server.is_empty() {
        vm.set_last_error(ERROR_INTERNET_NAME_NOT_RESOLVED);
        return 0;
    }
    if !vm.network_allowed(&server) {
        vm.set_last_error(ERROR_ACCESS_DENIED);
        return 0;
    }
    let user_agent = match store()
        .lock()
        .expect("wininet store")
        .handles
        .get(&session_handle)
    {
        Some(InternetHandle::Session(session)) => session.user_agent.clone(),
        _ => String::new(),
    };
    let handle = InternetHandle::Connection(Connection {
        host: server,
        port: if port == 0 { 80 } else { port },
        user_agent,
    });
    alloc_handle(handle)
}

fn http_open_request_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    // HttpOpenRequestA(hConnect, verb, object, version, referrer, accept, flags, context).
    let connection_handle = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let verb_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let object_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    let verb = read_c_string(vm, verb_ptr);
    let object = read_c_string(vm, object_ptr);
    let method = if verb.is_empty() { "GET".to_string() } else { verb };
    let path = if object.is_empty() {
        "/".to_string()
    } else if object.starts_with('/') {
        object
    } else {
        format!("/{object}")
    };
    let handle = InternetHandle::Request(Request {
        connection: connection_handle,
        method,
        path,
        response: None,
        cursor: 0,
    });
    alloc_handle(handle)
}

fn http_send_request_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    // HttpSendRequestA(hRequest, headers, headersLen, optional, optionalLen).
    let request_handle = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let headers_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let headers_len = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    let optional_ptr = vm.read_u32(stack_ptr + 16).unwrap_or(0);
    let optional_len = vm.read_u32(stack_ptr + 20).unwrap_or(0);

    let headers = read_optional_string(vm, headers_ptr, headers_len);
    let body = read_optional_bytes(vm, optional_ptr, optional_len as usize);

    let (method, path, host, port, user_agent) = {
        let guard = store().lock().expect("wininet store");
        let Some(InternetHandle::Request(request)) = guard.handles.get(&request_handle) else {
            return 0;
        };
        let Some(InternetHandle::Connection(connection)) = guard.handles.get(&request.connection) else {
            return 0;
        };
        (
            request.method.clone(),
            request.path.clone(),
            connection.host.clone(),
            connection.port,
            connection.user_agent.clone(),
        )
    };

    if !vm.network_allowed(&host) {
        vm.set_last_error(ERROR_ACCESS_DENIED);
        return 0;
    }

    let response = match send_http_request(&host, port, &method, &path, &user_agent, &headers, &body) {
        Ok(response) => response,
        Err(_) => {
            vm.set_last_error(ERROR_INTERNET_CONNECTION_ABORTED);
            return 0;
        }
    };

    let mut guard = store().lock().expect("wininet store");
    if let Some(InternetHandle::Request(request)) = guard.handles.get_mut(&request_handle) {
        request.response = Some(response);
        request.cursor = 0;
        vm.set_last_error(0);
        1
    } else {
        0
    }
}

fn http_query_info_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    // HttpQueryInfoA(hRequest, infoLevel, buffer, bufferLen, index).
    let request_handle = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let info_level = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let buffer = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    let buffer_len_ptr = vm.read_u32(stack_ptr + 16).unwrap_or(0);
    let _index_ptr = vm.read_u32(stack_ptr + 20).unwrap_or(0);

    let response = {
        let guard = store().lock().expect("wininet store");
        match guard.handles.get(&request_handle) {
            Some(InternetHandle::Request(request)) => request.response.clone(),
            _ => None,
        }
    };
    let Some(response) = response else {
        return 0;
    };

    if buffer_len_ptr == 0 {
        return 0;
    }

    let info = info_level & 0xFFFF;
    let wants_number = (info_level & HTTP_QUERY_FLAG_NUMBER) != 0;

    let (text, number) = match info {
        HTTP_QUERY_STATUS_CODE => (
            response.status.to_string(),
            response.status as u32,
        ),
        HTTP_QUERY_CONTENT_LENGTH => (
            response.body.len().to_string(),
            response.body.len() as u32,
        ),
        HTTP_QUERY_RAW_HEADERS_CRLF => (
            response.raw_headers.clone(),
            0,
        ),
        _ => return 0,
    };

    if wants_number {
        let required = 4u32;
        let provided = vm.read_u32(buffer_len_ptr).unwrap_or(0);
        if provided < required {
            let _ = vm.write_u32(buffer_len_ptr, required);
            return 0;
        }
        if buffer == 0 {
            return 0;
        }
        let _ = vm.write_u32(buffer, number);
        let _ = vm.write_u32(buffer_len_ptr, required);
        return 1;
    }

    let mut bytes = text.into_bytes();
    if !bytes.ends_with(b"\r\n") && info == HTTP_QUERY_RAW_HEADERS_CRLF {
        bytes.extend_from_slice(b"\r\n");
    }
    bytes.push(0);
    let required = bytes.len() as u32;
    let provided = vm.read_u32(buffer_len_ptr).unwrap_or(0);
    if provided < required {
        let _ = vm.write_u32(buffer_len_ptr, required);
        return 0;
    }
    if buffer == 0 {
        return 0;
    }
    let _ = vm.write_bytes(buffer, &bytes);
    let _ = vm.write_u32(buffer_len_ptr, required);
    1
}

fn internet_read_file(vm: &mut Vm, stack_ptr: u32) -> u32 {
    // InternetReadFile(hRequest, buffer, bytesToRead, bytesRead).
    let request_handle = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let buffer = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let bytes_to_read = vm.read_u32(stack_ptr + 12).unwrap_or(0) as usize;
    let bytes_read_ptr = vm.read_u32(stack_ptr + 16).unwrap_or(0);
    if buffer == 0 || bytes_read_ptr == 0 {
        return 0;
    }

    let mut guard = store().lock().expect("wininet store");
    let Some(InternetHandle::Request(request)) = guard.handles.get_mut(&request_handle) else {
        return 0;
    };
    let Some(response) = request.response.as_ref() else {
        return 0;
    };

    let remaining = response.body.len().saturating_sub(request.cursor);
    let read_len = remaining.min(bytes_to_read);
    let start = request.cursor;
    let end = start + read_len;
    if read_len > 0 {
        let _ = vm.write_bytes(buffer, &response.body[start..end]);
    }
    request.cursor = end;
    let _ = vm.write_u32(bytes_read_ptr, read_len as u32);
    1
}

fn internet_set_option_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn internet_close_handle(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let handle = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    if handle == 0 {
        return 0;
    }
    remove_handle(handle) as u32
}

fn send_http_request(
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

    stream.write_all(request.as_bytes())?;
    if !body.is_empty() {
        stream.write_all(body)?;
    }

    let mut response_bytes = Vec::new();
    stream.read_to_end(&mut response_bytes)?;
    Ok(parse_http_response(&response_bytes))
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

fn read_c_string(vm: &Vm, ptr: u32) -> String {
    if ptr == 0 {
        return String::new();
    }
    vm.read_c_string(ptr).unwrap_or_default()
}

fn read_optional_string(vm: &Vm, ptr: u32, len: u32) -> String {
    if ptr == 0 {
        return String::new();
    }
    if len == 0 || len == 0xFFFF_FFFF {
        return read_c_string(vm, ptr);
    }
    let mut bytes = Vec::with_capacity(len as usize);
    for offset in 0..len {
        if let Ok(value) = vm.read_u8(ptr.wrapping_add(offset)) {
            if value == 0 {
                break;
            }
            bytes.push(value);
        }
    }
    String::from_utf8_lossy(&bytes).to_string()
}

fn read_optional_bytes(vm: &Vm, ptr: u32, len: usize) -> Vec<u8> {
    if ptr == 0 || len == 0 {
        return Vec::new();
    }
    let mut bytes = Vec::with_capacity(len);
    for offset in 0..len {
        if let Ok(value) = vm.read_u8(ptr.wrapping_add(offset as u32)) {
            bytes.push(value);
        }
    }
    bytes
}

fn normalize_host(host: &str) -> String {
    let trimmed = host.trim();
    let trimmed = trimmed.strip_prefix("http://").unwrap_or(trimmed);
    let trimmed = trimmed.strip_prefix("https://").unwrap_or(trimmed);
    trimmed.trim_end_matches('/').to_string()
}

fn network_fallback_host(vm: &Vm) -> Option<&str> {
    vm.config()
        .sandbox_config()
        .and_then(|sandbox| sandbox.network_fallback_host())
}
