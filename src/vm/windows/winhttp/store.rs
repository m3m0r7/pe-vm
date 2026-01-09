//! WinHTTP handle store.

use std::sync::{Mutex, OnceLock};

use super::types::{WinHttpHandle, WinHttpStore};

const HANDLE_BASE: u32 = 0x7200_0000;

pub(super) fn store() -> &'static Mutex<WinHttpStore> {
    static STORE: OnceLock<Mutex<WinHttpStore>> = OnceLock::new();
    STORE.get_or_init(|| {
        Mutex::new(WinHttpStore {
            next_handle: HANDLE_BASE,
            ..WinHttpStore::default()
        })
    })
}

pub(super) fn alloc_handle(handle: WinHttpHandle) -> u32 {
    let mut guard = store().lock().expect("winhttp store");
    if guard.next_handle == 0 {
        guard.next_handle = HANDLE_BASE;
    }
    let handle_id = guard.next_handle;
    guard.next_handle = guard.next_handle.wrapping_add(1);
    guard.handles.insert(handle_id, handle);
    handle_id
}

pub(super) fn remove_handle(handle_id: u32) -> bool {
    let mut guard = store().lock().expect("winhttp store");
    guard.handles.remove(&handle_id).is_some()
}
