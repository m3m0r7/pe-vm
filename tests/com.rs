// Tests COM registry lookups and path mapping integration.
mod common;

use common::build_test_dll;
use pe_vm::windows::{self, com::{Com, ComArg, DispatchTable}, registry::RegistryValue};
use pe_vm::{Architecture, Os, Vm, VmConfig};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

// Create a unique temp directory for the COM fixture.
fn make_temp_dir(prefix: &str) -> PathBuf {
    let mut dir = std::env::temp_dir();
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    dir.push(format!("pe_vm_{prefix}_{stamp}"));
    fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

// Ensure COM uses registry entries plus VM path mappings.
#[test]
fn com_create_instance_uses_registry_and_paths() {
    let temp_dir = make_temp_dir("com");
    let host_path = temp_dir.join("sample.dll");
    fs::write(&host_path, build_test_dll()).expect("write dll");

    let mut paths = BTreeMap::new();
    paths.insert(
        r"C:\Vendor".to_string(),
        temp_dir.to_string_lossy().to_string(),
    );

    let mut vm = Vm::new(
        VmConfig::new()
            .os(Os::Windows)
            .architecture(Architecture::X86)
            .paths(paths),
    )
    .expect("vm");

    let registry = windows::get_registry_mut(&mut vm).expect("registry");
    registry
        .set(
            r"HKCR\CLSID\{TEST}\InprocServer32",
            RegistryValue::String(r"C:\Vendor\sample.dll".to_string()),
        )
        .expect("set registry");

    let com = Com::new();
    let dispatch = com.lookup_dispatch("{TEST}", DispatchTable::new());
    let obj = com
        .create_instance(&mut vm, &dispatch)
        .expect("create instance");

    assert_eq!(obj.clsid(), "{TEST}");
    assert_eq!(obj.dll_path(), r"C:\Vendor\sample.dll");
    assert_eq!(obj.host_path(), host_path.to_string_lossy());
}

// Ensure dispatch handlers are invoked for matching DISPIDs.
#[test]
fn com_dispatch_invokes_handler() {
    let temp_dir = make_temp_dir("dispatch");
    let host_path = temp_dir.join("sample.dll");
    fs::write(&host_path, build_test_dll()).expect("write dll");

    let mut paths = BTreeMap::new();
    paths.insert(
        r"C:\Vendor".to_string(),
        temp_dir.to_string_lossy().to_string(),
    );

    let mut vm = Vm::new(
        VmConfig::new()
            .os(Os::Windows)
            .architecture(Architecture::X86)
            .paths(paths),
    )
    .expect("vm");

    let registry = windows::get_registry_mut(&mut vm).expect("registry");
    registry
        .set(
            r"HKCR\CLSID\{TEST}\InprocServer32",
            RegistryValue::String(r"C:\Vendor\sample.dll".to_string()),
        )
        .expect("set registry");

    let mut dispatch = DispatchTable::new();
    dispatch.register_i4(0x4, |_vm, args| {
        let Some(ComArg::BStr(value)) = args.first() else {
            return Ok(-1);
        };
        if value == "SID" {
            Ok(7)
        } else {
            Ok(-2)
        }
    });

    let com = Com::new();
    let dispatch = com.lookup_dispatch("{TEST}", dispatch);
    let obj = com
        .create_instance(&mut vm, &dispatch)
        .expect("create instance");

    let result = obj
        .invoke_i4(&mut vm, 0x4, &[ComArg::BStr("SID".to_string())])
        .expect("invoke");
    assert_eq!(result, 7);
}
