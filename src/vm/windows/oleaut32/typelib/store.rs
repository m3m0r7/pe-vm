use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use super::{TypeInfoData, TypeLib};

#[derive(Debug, Clone)]
struct TypeInfoHandle {
    typelib_id: u32,
    index: usize,
}

#[derive(Default)]
struct TypeLibStore {
    next_id: u32,
    typelibs: HashMap<u32, TypeLib>,
    typeinfos: HashMap<u32, TypeInfoHandle>,
}

fn store() -> &'static Mutex<TypeLibStore> {
    static STORE: OnceLock<Mutex<TypeLibStore>> = OnceLock::new();
    STORE.get_or_init(|| {
        Mutex::new(TypeLibStore {
            next_id: 1,
            ..TypeLibStore::default()
        })
    })
}

pub(super) fn store_typelib(lib: TypeLib) -> u32 {
    let mut guard = store().lock().expect("typelib store");
    let id = guard.next_id;
    guard.next_id = guard.next_id.wrapping_add(1);
    guard.typelibs.insert(id, lib);
    id
}

pub(super) fn store_typeinfo(typelib_id: u32, index: usize) -> Option<u32> {
    let mut guard = store().lock().expect("typelib store");
    guard.typelibs.get(&typelib_id)?;
    let id = guard.next_id;
    guard.next_id = guard.next_id.wrapping_add(1);
    guard
        .typeinfos
        .insert(id, TypeInfoHandle { typelib_id, index });
    Some(id)
}

pub(super) fn get_typelib(id: u32) -> Option<TypeLib> {
    let guard = store().lock().expect("typelib store");
    guard.typelibs.get(&id).cloned()
}

pub(super) fn get_typeinfo(id: u32) -> Option<TypeInfoData> {
    let guard = store().lock().expect("typelib store");
    let handle = guard.typeinfos.get(&id)?;
    guard
        .typelibs
        .get(&handle.typelib_id)
        .and_then(|lib| lib.typeinfos.get(handle.index).cloned())
}
