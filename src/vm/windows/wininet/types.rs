//! WinINet handle types.

use std::collections::HashMap;

#[derive(Default)]
pub(super) struct WinInetStore {
    pub(super) next_handle: u32,
    pub(super) handles: HashMap<u32, InternetHandle>,
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
    pub(super) response: Option<Response>,
    pub(super) cursor: usize,
}

#[derive(Clone)]
pub(crate) struct Response {
    pub(crate) status: u16,
    pub(crate) body: Vec<u8>,
    pub(crate) raw_headers: String,
}

#[derive(Clone)]
pub(super) enum InternetHandle {
    Session(Session),
    Connection(Connection),
    Request(Request),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wininet_store_default() {
        let store = WinInetStore::default();
        assert_eq!(store.next_handle, 0);
        assert!(store.handles.is_empty());
    }

    #[test]
    fn test_session_create() {
        let session = Session {
            user_agent: "TestAgent".to_string(),
        };
        assert_eq!(session.user_agent, "TestAgent");
    }

    #[test]
    fn test_connection_create() {
        let conn = Connection {
            host: "example.com".to_string(),
            port: 443,
            user_agent: "TestAgent".to_string(),
            secure_hint: true,
        };
        assert_eq!(conn.host, "example.com");
        assert_eq!(conn.port, 443);
        assert!(conn.secure_hint);
    }

    #[test]
    fn test_request_create() {
        let req = Request {
            connection: 1,
            method: "GET".to_string(),
            path: "/api".to_string(),
            secure: true,
            response: None,
            cursor: 0,
        };
        assert_eq!(req.method, "GET");
        assert_eq!(req.path, "/api");
        assert!(req.secure);
        assert!(req.response.is_none());
    }

    #[test]
    fn test_response_create() {
        let resp = Response {
            status: 200,
            body: vec![1, 2, 3],
            raw_headers: "Content-Type: text/html".to_string(),
        };
        assert_eq!(resp.status, 200);
        assert_eq!(resp.body, vec![1, 2, 3]);
    }

    #[test]
    fn test_internet_handle_variants() {
        let session = InternetHandle::Session(Session {
            user_agent: "Test".to_string(),
        });
        assert!(matches!(session, InternetHandle::Session(_)));

        let conn = InternetHandle::Connection(Connection {
            host: "test.com".to_string(),
            port: 80,
            user_agent: "Test".to_string(),
            secure_hint: false,
        });
        assert!(matches!(conn, InternetHandle::Connection(_)));
    }
}
