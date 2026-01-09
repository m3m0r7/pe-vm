//! WinHTTP API entry points.

use crate::vm::windows::wininet::send_http_request;
use crate::vm::Vm;
use crate::vm_args;

use super::store::{alloc_handle, remove_handle, store};
use super::types::{Connection, Request, Session, WinHttpHandle};

const ERROR_ACCESS_DENIED: u32 = 5;
const ERROR_WINHTTP_NAME_NOT_RESOLVED: u32 = 12007;
const ERROR_WINHTTP_CANNOT_CONNECT: u32 = 12029;

const WINHTTP_FLAG_SECURE: u32 = 0x0080_0000;

pub(super) fn winhttp_open(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (agent_ptr, _, _, _, _) = vm_args!(vm, stack_ptr; u32, u32, u32, u32, u32);
    let agent = read_wide_or_utf16le_str!(vm, agent_ptr);
    let handle = WinHttpHandle::Session(Session { user_agent: agent });
    alloc_handle(handle)
}

pub(super) fn winhttp_connect(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (session_handle, server_ptr, port, _) = vm_args!(vm, stack_ptr; u32, u32, u32, u32);
    let (host, mut secure_hint) = parse_host(&read_wide_or_utf16le_str!(vm, server_ptr));
    let mut port = port as u16;
    if port == 0 {
        port = if secure_hint { 443 } else { 80 };
    } else if port == 443 {
        secure_hint = true;
    }
    if host.is_empty() {
        vm.set_last_error(ERROR_WINHTTP_NAME_NOT_RESOLVED);
        return 0;
    }
    if !vm.network_allowed(&host) {
        vm.set_last_error(ERROR_ACCESS_DENIED);
        return 0;
    }
    let user_agent = match store()
        .lock()
        .expect("winhttp store")
        .handles
        .get(&session_handle)
    {
        Some(WinHttpHandle::Session(session)) => session.user_agent.clone(),
        _ => String::new(),
    };
    let handle = WinHttpHandle::Connection(Connection {
        host,
        port,
        user_agent,
        secure_hint,
    });
    alloc_handle(handle)
}

pub(super) fn winhttp_open_request(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (connection_handle, verb_ptr, object_ptr, _, _, _, flags) =
        vm_args!(vm, stack_ptr; u32, u32, u32, u32, u32, u32, u32);
    let verb = read_wide_or_utf16le_str!(vm, verb_ptr);
    let object = read_wide_or_utf16le_str!(vm, object_ptr);
    let secure = (flags & WINHTTP_FLAG_SECURE) != 0;
    let method = if verb.is_empty() {
        "GET".to_string()
    } else {
        verb
    };
    let path = if object.is_empty() {
        "/".to_string()
    } else if object.starts_with('/') {
        object
    } else {
        format!("/{object}")
    };
    let handle = WinHttpHandle::Request(Request {
        connection: connection_handle,
        method,
        path,
        secure,
        headers: String::new(),
        response: None,
        cursor: 0,
    });
    alloc_handle(handle)
}

pub(super) fn winhttp_add_request_headers(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (request_handle, headers_ptr, headers_len, _) =
        vm_args!(vm, stack_ptr; u32, u32, u32, u32);
    let headers = read_optional_wide_string(vm, headers_ptr, headers_len);
    if headers.is_empty() {
        return 1;
    }
    let mut guard = store().lock().expect("winhttp store");
    let Some(WinHttpHandle::Request(request)) = guard.handles.get_mut(&request_handle) else {
        return 0;
    };
    let mut new_headers = headers.trim_end_matches(['\r', '\n']).to_string();
    if !new_headers.ends_with("\r\n") {
        new_headers.push_str("\r\n");
    }
    if !request.headers.is_empty() && !request.headers.ends_with("\r\n") {
        request.headers.push_str("\r\n");
    }
    request.headers.push_str(&new_headers);
    1
}

pub(super) fn winhttp_send_request(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (request_handle, headers_ptr, headers_len, optional_ptr, optional_len, _, _) =
        vm_args!(vm, stack_ptr; u32, u32, u32, u32, u32, u32, u32);
    let additional_headers = read_optional_wide_string(vm, headers_ptr, headers_len);
    let body = read_optional_bytes(vm, optional_ptr, optional_len as usize);

    let (method, path, host, port, user_agent, secure, headers) = {
        let guard = store().lock().expect("winhttp store");
        let Some(WinHttpHandle::Request(request)) = guard.handles.get(&request_handle) else {
            return 0;
        };
        let Some(WinHttpHandle::Connection(connection)) =
            guard.handles.get(&request.connection)
        else {
            return 0;
        };
        let mut secure = request.secure || connection.secure_hint;
        let mut port = connection.port;
        if port == 0 {
            port = if secure { 443 } else { 80 };
        } else if port == 443 {
            secure = true;
        }
        let merged_headers = merge_headers(&request.headers, &additional_headers);
        (
            request.method.clone(),
            request.path.clone(),
            connection.host.clone(),
            port,
            connection.user_agent.clone(),
            secure,
            merged_headers,
        )
    };

    if host.is_empty() {
        vm.set_last_error(ERROR_WINHTTP_NAME_NOT_RESOLVED);
        return 0;
    }
    if !vm.network_allowed(&host) {
        vm.set_last_error(ERROR_ACCESS_DENIED);
        return 0;
    }

    let headers = ensure_host_header(&headers, &host);
    let response = match send_http_request(
        &host,
        port,
        &method,
        &path,
        &user_agent,
        &headers,
        &body,
        secure,
    ) {
        Ok(response) => response,
        Err(_) => {
            vm.set_last_error(ERROR_WINHTTP_CANNOT_CONNECT);
            return 0;
        }
    };

    let mut guard = store().lock().expect("winhttp store");
    if let Some(WinHttpHandle::Request(request)) = guard.handles.get_mut(&request_handle) {
        request.response = Some(super::types::Response {
            status: response.status,
            body: response.body,
            raw_headers: response.raw_headers,
        });
        request.cursor = 0;
        vm.set_last_error(0);
        1
    } else {
        0
    }
}

pub(super) fn winhttp_receive_response(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (request_handle, _) = vm_args!(vm, stack_ptr; u32, u32);
    let guard = store().lock().expect("winhttp store");
    match guard.handles.get(&request_handle) {
        Some(WinHttpHandle::Request(request)) if request.response.is_some() => 1,
        _ => 0,
    }
}

pub(super) fn winhttp_query_data_available(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (request_handle, size_ptr) = vm_args!(vm, stack_ptr; u32, u32);
    if size_ptr == 0 {
        return 0;
    }
    let guard = store().lock().expect("winhttp store");
    let Some(WinHttpHandle::Request(request)) = guard.handles.get(&request_handle) else {
        return 0;
    };
    let Some(response) = request.response.as_ref() else {
        return 0;
    };
    let remaining = response.body.len().saturating_sub(request.cursor);
    let _ = vm.write_u32(size_ptr, remaining as u32);
    1
}

pub(super) fn winhttp_read_data(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (request_handle, buffer, bytes_to_read, bytes_read_ptr) =
        vm_args!(vm, stack_ptr; u32, u32, u32, u32);
    if buffer == 0 || bytes_read_ptr == 0 {
        return 0;
    }
    let mut guard = store().lock().expect("winhttp store");
    let Some(WinHttpHandle::Request(request)) = guard.handles.get_mut(&request_handle) else {
        return 0;
    };
    let Some(response) = request.response.as_ref() else {
        return 0;
    };
    let remaining = response.body.len().saturating_sub(request.cursor);
    let read_len = remaining.min(bytes_to_read as usize);
    let start = request.cursor;
    let end = start + read_len;
    if read_len > 0 {
        let _ = vm.write_bytes(buffer, &response.body[start..end]);
    }
    request.cursor = end;
    let _ = vm.write_u32(bytes_read_ptr, read_len as u32);
    1
}

pub(super) fn winhttp_close_handle(_vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (handle,) = vm_args!(_vm, stack_ptr; u32);
    if handle == 0 {
        return 0;
    }
    remove_handle(handle) as u32
}

fn read_optional_wide_string(vm: &Vm, ptr: u32, len: u32) -> String {
    if ptr == 0 {
        return String::new();
    }
    if len == 0 || len == 0xFFFF_FFFF {
        return read_wide_or_utf16le_str!(vm, ptr);
    }
    let mut units = Vec::with_capacity(len as usize);
    for i in 0..len {
        let unit = vm
            .read_u16(ptr.wrapping_add((i as u32) * 2))
            .unwrap_or(0);
        if unit == 0 {
            break;
        }
        units.push(unit);
    }
    String::from_utf16_lossy(&units)
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

fn parse_host(host: &str) -> (String, bool) {
    let trimmed = host.trim();
    let mut secure = false;
    let trimmed = if let Some(rest) = trimmed.strip_prefix("https://") {
        secure = true;
        rest
    } else {
        trimmed
    };
    let trimmed = trimmed.strip_prefix("http://").unwrap_or(trimmed);
    let trimmed = trimmed.trim_end_matches('/');
    (trimmed.to_string(), secure)
}

fn merge_headers(existing: &str, additional: &str) -> String {
    let mut out = String::new();
    if !existing.is_empty() {
        out.push_str(existing.trim_end_matches(['\r', '\n']));
        if !out.ends_with("\r\n") {
            out.push_str("\r\n");
        }
    }
    if !additional.is_empty() {
        out.push_str(additional.trim_end_matches(['\r', '\n']));
        if !out.ends_with("\r\n") {
            out.push_str("\r\n");
        }
    }
    out
}

fn ensure_host_header(headers: &str, host: &str) -> String {
    if host.is_empty() {
        return headers.to_string();
    }
    let mut out_lines = Vec::new();
    let mut host_set = false;
    for line in headers.split('\n') {
        let trimmed = line.trim_end_matches('\r');
        if trimmed.to_ascii_lowercase().starts_with("host:") {
            let value = trimmed["host:".len()..].trim();
            if value.is_empty() && !host_set {
                out_lines.push(format!("Host: {host}"));
                host_set = true;
            } else {
                out_lines.push(trimmed.to_string());
                if !value.is_empty() {
                    host_set = true;
                }
            }
            continue;
        }
        if !trimmed.is_empty() {
            out_lines.push(trimmed.to_string());
        }
    }
    if !host_set {
        out_lines.push(format!("Host: {host}"));
    }
    let mut joined = out_lines.join("\r\n");
    if !joined.is_empty() {
        joined.push_str("\r\n");
    }
    joined
}
