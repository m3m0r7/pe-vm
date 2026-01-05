// Tests VM path mapping behavior.
use pe_vm::{Architecture, Os, Vm, VmConfig};
use std::collections::BTreeMap;

// Verify longest-prefix mapping and case-insensitive Windows matching.
#[test]
fn map_path_prefers_longest_prefix() {
    let mut paths = BTreeMap::new();
    paths.insert(r"C:\Vendor".to_string(), "/host/vendor".to_string());
    paths.insert(r"C:\".to_string(), "/host/root".to_string());

    let vm = Vm::new(
        VmConfig::new()
            .os(Os::Windows)
            .architecture(Architecture::X86)
            .paths(paths),
    )
    .expect("vm");

    let mapped = vm.map_path(r"c:\VENDOR\sample.dll");
    assert_eq!(mapped, "/host/vendor/sample.dll");

    let fallback = vm.map_path(r"D:\other.dll");
    assert_eq!(fallback, r"D:\other.dll");
}
