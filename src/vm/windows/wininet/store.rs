//! WinINet handle store.

use std::sync::{Mutex, OnceLock};

use super::types::{InternetHandle, WinInetStore};

const HANDLE_BASE: u32 = 0x7100_0000;

pub(super) fn store() -> &'static Mutex<WinInetStore> {
    static STORE: OnceLock<Mutex<WinInetStore>> = OnceLock::new();
    STORE.get_or_init(|| {
        Mutex::new(WinInetStore {
            next_handle: HANDLE_BASE,
            ..WinInetStore::default()
        })
    })
}

pub(super) fn alloc_handle(handle: InternetHandle) -> u32 {
    let mut guard = store().lock().expect("wininet store");
    if guard.next_handle == 0 {
        guard.next_handle = HANDLE_BASE;
    }
    let handle_id = guard.next_handle;
    guard.next_handle = guard.next_handle.wrapping_add(1);
    guard.handles.insert(handle_id, handle);
    handle_id
}

pub(super) fn remove_handle(handle_id: u32) -> bool {
    let mut guard = store().lock().expect("wininet store");
    guard.handles.remove(&handle_id).is_some()
}
