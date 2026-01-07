// Tests sandbox network allow/deny rules in the VM.
use pe_vm::{SandboxConfig, Vm, VmConfig};

#[test]
fn sandbox_network_policy() {
    let sandbox = SandboxConfig::new().enable_network("127.0.0.1");
    let vm = Vm::new(VmConfig::new().sandbox(sandbox)).expect("vm");

    assert!(vm.network_allowed("127.0.0.1"));
    assert!(vm.network_allowed("example.com"));

    let disabled = SandboxConfig::new().disable_network();
    let vm_disabled = Vm::new(VmConfig::new().sandbox(disabled)).expect("vm");
    assert!(!vm_disabled.network_allowed("example.com"));
}
