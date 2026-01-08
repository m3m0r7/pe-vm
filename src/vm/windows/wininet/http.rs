//! WinINet API entry points.

use crate::vm::Vm;
use crate::vm_args;

use super::client::send_http_request;
use super::store::{alloc_handle, remove_handle, store};
use super::trace::trace_net;
use super::types::{Connection, InternetHandle, Request, Session};
use super::utils::{
    apply_form_overrides, default_host_override, default_path_override, ensure_content_length,
    ensure_host_header, form_overrides, network_fallback_host, parse_host, read_c_string,
    read_optional_bytes, read_optional_string,
};

const ERROR_ACCESS_DENIED: u32 = 5;
const ERROR_INTERNET_NAME_NOT_RESOLVED: u32 = 12007;
const ERROR_INTERNET_CONNECTION_ABORTED: u32 = 12030;

const HTTP_QUERY_CONTENT_LENGTH: u32 = 5;
const HTTP_QUERY_STATUS_CODE: u32 = 19;
const HTTP_QUERY_RAW_HEADERS_CRLF: u32 = 22;
const HTTP_QUERY_FLAG_NUMBER: u32 = 0x2000_0000;
const INTERNET_FLAG_SECURE: u32 = 0x0080_0000;

pub(super) fn internet_open_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    // InternetOpenA(hAgent, accessType, proxy, bypass, flags).
    let (agent_ptr,) = vm_args!(vm, stack_ptr; u32);
    let agent = read_c_string(vm, agent_ptr);
    let handle = InternetHandle::Session(Session { user_agent: agent });
    alloc_handle(handle)
}

pub(super) fn internet_connect_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    // InternetConnectA(hInternet, server, port, user, pass, service, flags, context).
    let (session_handle, server_ptr, port_arg) = vm_args!(vm, stack_ptr; u32, u32, u32);
    let mut port = port_arg as u16;
    if std::env::var("PE_VM_TRACE").is_ok() && server_ptr != 0 {
        let mut raw = String::new();
        let mut ascii = String::new();
        for idx in 0..64u32 {
            let byte = vm.read_u8(server_ptr.wrapping_add(idx)).unwrap_or(0);
            if idx > 0 {
                raw.push(' ');
            }
            raw.push_str(&format!("{byte:02X}"));
            if (0x20..=0x7E).contains(&byte) {
                ascii.push(byte as char);
            } else {
                ascii.push('.');
            }
        }
        trace_net(&format!(
            "InternetConnectA server_ptr=0x{server_ptr:08X} raw={raw} ascii={ascii}"
        ));
    }
    let (mut server, mut secure_hint) = parse_host(&read_c_string(vm, server_ptr));
    if port == 0 {
        if let Some((host, port_str)) = server.rsplit_once(':') {
            if let Ok(parsed) = port_str.parse::<u16>() {
                server = host.to_string();
                port = parsed;
            }
        }
    }
    if server.is_empty() {
        if let Some(override_host) = default_host_override() {
            let (mut fallback_host, fallback_secure) = parse_host(&override_host);
            let mut fallback_port = None;
            if let Some((host, port_str)) = fallback_host.rsplit_once(':') {
                if let Ok(parsed) = port_str.parse::<u16>() {
                    fallback_host = host.to_string();
                    if parsed > 0 {
                        fallback_port = Some(parsed);
                    }
                }
            }
            if !fallback_host.is_empty() {
                server = fallback_host;
                secure_hint = secure_hint || fallback_secure;
                if port == 0 {
                    port = fallback_port.unwrap_or(0);
                }
            }
        }
    }
    if port == 443 {
        secure_hint = true;
    }
    if let Some(fallback) = network_fallback_host(vm) {
        let (mut fallback_host, fallback_secure) = parse_host(fallback);
        let mut fallback_port = None;
        if let Some((host, port_str)) = fallback_host.rsplit_once(':') {
            if let Ok(parsed) = port_str.parse::<u16>() {
                fallback_host = host.to_string();
                if parsed > 0 {
                    fallback_port = Some(parsed);
                }
            }
        }
        if !fallback_host.is_empty() {
            server = fallback_host;
            secure_hint = secure_hint || fallback_secure;
            port = fallback_port.unwrap_or(0);
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
        host: server.clone(),
        port,
        user_agent,
        secure_hint,
    });
    trace_net(&format!("InternetConnectA host={server} port={port}"));
    alloc_handle(handle)
}

pub(super) fn http_open_request_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    // HttpOpenRequestA(hConnect, verb, object, version, referrer, accept, flags, context).
    let (connection_handle, verb_ptr, object_ptr, _, _, _, flags) =
        vm_args!(vm, stack_ptr; u32, u32, u32, u32, u32, u32, u32);
    if std::env::var("PE_VM_TRACE").is_ok() && object_ptr != 0 {
        let mut raw = String::new();
        let mut ascii = String::new();
        for idx in 0..64u32 {
            let byte = vm.read_u8(object_ptr.wrapping_add(idx)).unwrap_or(0);
            if idx > 0 {
                raw.push(' ');
            }
            raw.push_str(&format!("{byte:02X}"));
            if (0x20..=0x7E).contains(&byte) {
                ascii.push(byte as char);
            } else {
                ascii.push('.');
            }
        }
        trace_net(&format!(
            "HttpOpenRequestA object_ptr=0x{object_ptr:08X} raw={raw} ascii={ascii}"
        ));
    }
    let verb = read_c_string(vm, verb_ptr);
    let object = read_c_string(vm, object_ptr);
    let secure = (flags & INTERNET_FLAG_SECURE) != 0;
    let method = if verb.is_empty() {
        "GET".to_string()
    } else {
        verb
    };
    let mut path = if object.is_empty() {
        "/".to_string()
    } else if object.starts_with('/') {
        object
    } else {
        format!("/{object}")
    };
    if path == "/" {
        if let Some(override_path) = default_path_override() {
            if override_path.starts_with('/') {
                path = override_path;
            } else {
                path = format!("/{override_path}");
            }
        }
    }
    trace_net(&format!("HttpOpenRequestA method={method} path={path}"));
    let handle = InternetHandle::Request(Request {
        connection: connection_handle,
        method,
        path,
        secure,
        response: None,
        cursor: 0,
    });
    alloc_handle(handle)
}

pub(super) fn http_send_request_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    // HttpSendRequestA(hRequest, headers, headersLen, optional, optionalLen).
    let (request_handle, headers_ptr, headers_len, optional_ptr, optional_len) =
        vm_args!(vm, stack_ptr; u32, u32, u32, u32, u32);

    let headers = read_optional_string(vm, headers_ptr, headers_len);
    let mut body = read_optional_bytes(vm, optional_ptr, optional_len as usize);

    let (method, path, host, port, user_agent, secure) = {
        let guard = store().lock().expect("wininet store");
        let Some(InternetHandle::Request(request)) = guard.handles.get(&request_handle) else {
            return 0;
        };
        let Some(InternetHandle::Connection(connection)) = guard.handles.get(&request.connection)
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
        (
            request.method.clone(),
            request.path.clone(),
            connection.host.clone(),
            port,
            connection.user_agent.clone(),
            secure,
        )
    };

    let mut headers = ensure_host_header(&headers, &host);
    let overrides = form_overrides();
    if !overrides.is_empty()
        && headers
            .to_ascii_lowercase()
            .contains("application/x-www-form-urlencoded")
    {
        if let Ok(body_text) = String::from_utf8(body.clone()) {
            let (updated, changed) = apply_form_overrides(&body_text, &overrides);
            if changed {
                body = updated.into_bytes();
                headers = ensure_content_length(&headers, body.len());
            }
        }
    }

    if !vm.network_allowed(&host) {
        vm.set_last_error(ERROR_ACCESS_DENIED);
        return 0;
    }

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
        Err(err) => {
            trace_net(&format!("HttpSendRequestA failed: {err}"));
            // Return a stub HTTP 200 response on connection failure to allow
            // applications that require network initialization to proceed.
            // This prevents JVLink from interpreting connection errors as
            // "server maintenance" and aborting initialization.
            use super::types::Response;
            trace_net("HttpSendRequestA: returning stub 200 response");
            Response {
                status: 200,
                body: Vec::new(),
                raw_headers: "HTTP/1.1 200 OK\r\nContent-Length: 0\r\n".to_string(),
            }
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

pub(super) fn http_query_info_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    // HttpQueryInfoA(hRequest, infoLevel, buffer, bufferLen, index).
    let (request_handle, info_level, buffer, buffer_len_ptr, _index_ptr) =
        vm_args!(vm, stack_ptr; u32, u32, u32, u32, u32);

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
    if std::env::var("PE_VM_TRACE").is_ok() {
        let provided = vm.read_u32(buffer_len_ptr).unwrap_or(0);
        trace_net(&format!(
            "HttpQueryInfoA handle=0x{request_handle:08X} info=0x{info_level:08X} buf=0x{buffer:08X} len={provided}"
        ));
    }

    let (text, number) = match info {
        HTTP_QUERY_STATUS_CODE => (response.status.to_string(), response.status as u32),
        HTTP_QUERY_CONTENT_LENGTH => (response.body.len().to_string(), response.body.len() as u32),
        HTTP_QUERY_RAW_HEADERS_CRLF => (response.raw_headers.clone(), 0),
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

pub(super) fn internet_read_file(vm: &mut Vm, stack_ptr: u32) -> u32 {
    // InternetReadFile(hRequest, buffer, bytesToRead, bytesRead).
    let (request_handle, buffer, bytes_to_read, bytes_read_ptr) =
        vm_args!(vm, stack_ptr; u32, u32, u32, u32);
    let bytes_to_read = bytes_to_read as usize;
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

pub(super) fn internet_set_option_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (handle, option, buffer, buffer_len) = vm_args!(vm, stack_ptr; u32, u32, u32, u32);
    let buffer_len = buffer_len as usize;
    if std::env::var("PE_VM_TRACE").is_ok() {
        let mut preview = String::new();
        if buffer != 0 && buffer_len > 0 {
            let len = buffer_len.min(64);
            let mut ascii = String::new();
            for idx in 0..len {
                let byte = vm.read_u8(buffer.wrapping_add(idx as u32)).unwrap_or(0);
                if (0x20..=0x7E).contains(&byte) {
                    ascii.push(byte as char);
                } else {
                    ascii.push('.');
                }
            }
            preview = ascii;
        }
        trace_net(&format!(
            "InternetSetOptionA handle=0x{handle:08X} option=0x{option:08X} len={buffer_len} preview={preview:?}"
        ));
    }
    1
}

pub(super) fn internet_close_handle(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (handle,) = vm_args!(vm, stack_ptr; u32);
    if handle == 0 {
        return 0;
    }
    remove_handle(handle) as u32
}
