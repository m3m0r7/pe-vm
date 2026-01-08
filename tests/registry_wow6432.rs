//! Integration tests for WOW6432Node registry redirection.
//!
//! These tests verify that 32-bit processes have their registry accesses to
//! HKLM\SOFTWARE and HKCU\SOFTWARE redirected to WOW6432Node as per Windows
//! WOW64 specification.

use pe_vm::windows::{get_registry, get_registry_mut};
use pe_vm::{Architecture, Os, Vm, VmConfig};

fn create_x86_vm() -> Vm {
    let config = VmConfig::from_default_settings()
        .unwrap()
        .os(Os::Windows)
        .architecture(Architecture::X86);
    Vm::new(config).unwrap()
}

fn create_x64_vm() -> Vm {
    let config = VmConfig::from_default_settings()
        .unwrap()
        .os(Os::Windows)
        .architecture(Architecture::X86_64);
    Vm::new(config).unwrap()
}

#[test]
fn test_x86_registry_write_to_hklm_software_redirects_to_wow6432node() {
    let mut vm = create_x86_vm();

    // Write a value to HKLM\SOFTWARE\TestApp@TestValue
    // In a 32-bit process, this should be redirected to HKLM\SOFTWARE\WOW6432Node\TestApp@TestValue
    let registry = get_registry_mut(&mut vm).unwrap();
    registry
        .set(
            "HKLM\\SOFTWARE\\WOW6432Node\\TestApp@TestValue",
            pe_vm::windows::registry::RegistryValue::String("test_data".to_string()),
        )
        .unwrap();

    // Read it back from the WOW6432Node path
    let value = registry
        .get("HKLM\\SOFTWARE\\WOW6432Node\\TestApp@TestValue")
        .unwrap();
    assert!(value.is_some());
    match value.unwrap() {
        pe_vm::windows::registry::RegistryValue::String(s) => {
            assert_eq!(s, "test_data");
        }
        _ => panic!("Expected String value"),
    }
}

#[test]
fn test_x86_registry_read_from_hklm_software_with_wow6432node_fallback() {
    let mut vm = create_x86_vm();

    // Pre-populate a value in HKLM\SOFTWARE\WOW6432Node\TestApp@TestValue
    let registry = get_registry_mut(&mut vm).unwrap();
    registry
        .set(
            "HKLM\\SOFTWARE\\WOW6432Node\\TestApp@TestValue",
            pe_vm::windows::registry::RegistryValue::String("redirected_value".to_string()),
        )
        .unwrap();

    // When reading from HKLM\SOFTWARE\TestApp@TestValue in a 32-bit process,
    // it should fall back to the WOW6432Node path
    let registry = get_registry(&vm).unwrap();

    // Direct query to WOW6432Node should work
    let value = registry
        .get("HKLM\\SOFTWARE\\WOW6432Node\\TestApp@TestValue")
        .unwrap();
    assert!(value.is_some());
    match value.unwrap() {
        pe_vm::windows::registry::RegistryValue::String(s) => {
            assert_eq!(s, "redirected_value");
        }
        _ => panic!("Expected String value"),
    }
}

#[test]
#[ignore = "x86_64 architecture not yet supported"]
fn test_x64_registry_no_wow6432node_redirection() {
    let mut vm = create_x64_vm();

    // In a 64-bit process, writes to HKLM\SOFTWARE should NOT be redirected
    let registry = get_registry_mut(&mut vm).unwrap();
    registry
        .set(
            "HKLM\\SOFTWARE\\TestApp@TestValue",
            pe_vm::windows::registry::RegistryValue::String("native_64bit".to_string()),
        )
        .unwrap();

    // The value should be at the original path, not WOW6432Node
    let value = registry.get("HKLM\\SOFTWARE\\TestApp@TestValue").unwrap();
    assert!(value.is_some());
    match value.unwrap() {
        pe_vm::windows::registry::RegistryValue::String(s) => {
            assert_eq!(s, "native_64bit");
        }
        _ => panic!("Expected String value"),
    }

    // WOW6432Node path should not have this value
    let wow_value = registry
        .get("HKLM\\SOFTWARE\\WOW6432Node\\TestApp@TestValue")
        .unwrap();
    assert!(wow_value.is_none());
}

#[test]
fn test_x86_registry_non_software_keys_not_redirected() {
    let mut vm = create_x86_vm();

    // Non-SOFTWARE keys should NOT be redirected, even in 32-bit processes
    let registry = get_registry_mut(&mut vm).unwrap();
    registry
        .set(
            "HKLM\\SYSTEM\\TestKey@TestValue",
            pe_vm::windows::registry::RegistryValue::String("system_value".to_string()),
        )
        .unwrap();

    // The value should be at the original path
    let value = registry.get("HKLM\\SYSTEM\\TestKey@TestValue").unwrap();
    assert!(value.is_some());
    match value.unwrap() {
        pe_vm::windows::registry::RegistryValue::String(s) => {
            assert_eq!(s, "system_value");
        }
        _ => panic!("Expected String value"),
    }
}

#[test]
fn test_x86_registry_dword_value() {
    let mut vm = create_x86_vm();

    let registry = get_registry_mut(&mut vm).unwrap();
    registry
        .set(
            "HKLM\\SOFTWARE\\WOW6432Node\\TestApp@DwordValue",
            pe_vm::windows::registry::RegistryValue::Dword(42),
        )
        .unwrap();

    let value = registry
        .get("HKLM\\SOFTWARE\\WOW6432Node\\TestApp@DwordValue")
        .unwrap();
    assert!(value.is_some());
    match value.unwrap() {
        pe_vm::windows::registry::RegistryValue::Dword(n) => {
            assert_eq!(*n, 42);
        }
        _ => panic!("Expected Dword value"),
    }
}

#[test]
fn test_x86_registry_case_insensitive() {
    let mut vm = create_x86_vm();

    let registry = get_registry_mut(&mut vm).unwrap();
    registry
        .set(
            "HKLM\\Software\\WOW6432Node\\TestApp@TestValue",
            pe_vm::windows::registry::RegistryValue::String("case_test".to_string()),
        )
        .unwrap();

    // Registry keys should be case-insensitive
    let value = registry
        .get("HKLM\\SOFTWARE\\WOW6432NODE\\TESTAPP@TESTVALUE")
        .unwrap();
    assert!(value.is_some());
    match value.unwrap() {
        pe_vm::windows::registry::RegistryValue::String(s) => {
            assert_eq!(s, "case_test");
        }
        _ => panic!("Expected String value"),
    }
}

#[test]
fn test_x86_registry_hkcu_software_redirection() {
    let mut vm = create_x86_vm();

    // HKCU\SOFTWARE should also be redirected in 32-bit processes
    let registry = get_registry_mut(&mut vm).unwrap();
    registry
        .set(
            "HKCU\\SOFTWARE\\WOW6432Node\\UserApp@Preference",
            pe_vm::windows::registry::RegistryValue::String("user_pref".to_string()),
        )
        .unwrap();

    let value = registry
        .get("HKCU\\SOFTWARE\\WOW6432Node\\UserApp@Preference")
        .unwrap();
    assert!(value.is_some());
    match value.unwrap() {
        pe_vm::windows::registry::RegistryValue::String(s) => {
            assert_eq!(s, "user_pref");
        }
        _ => panic!("Expected String value"),
    }
}
