//! WinHTTP handle types.

use std::collections::HashMap;

#[derive(Default)]
pub(super) struct WinHttpStore {
    pub(super) next_handle: u32,
    pub(super) handles: HashMap<u32, WinHttpHandle>,
}

#[derive(Clone)]
pub(super) struct Session {
    pub(super) user_agent: String,
}

#[derive(Clone)]
pub(super) struct Connection {
    pub(super) host: String,
    pub(super) port: u16,
    pub(super) user_agent: String,
    pub(super) secure_hint: bool,
}

#[derive(Clone)]
pub(super) struct Request {
    pub(super) connection: u32,
    pub(super) method: String,
    pub(super) path: String,
    pub(super) secure: bool,
    pub(super) headers: String,
    pub(super) response: Option<Response>,
    pub(super) cursor: usize,
}

#[derive(Clone)]
pub(super) struct Response {
    #[allow(dead_code)]
    pub(super) status: u16,
    pub(super) body: Vec<u8>,
    #[allow(dead_code)]
    pub(super) raw_headers: String,
}

#[derive(Clone)]
pub(super) enum WinHttpHandle {
    Session(Session),
    Connection(Connection),
    Request(Request),
}
