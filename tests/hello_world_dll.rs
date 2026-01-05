// Integration test for running the sample hello-world DLL.
use pe_vm::{
    windows, Architecture, ExecuteOptions, MessageBoxMode, Os, Pe, SymbolExecutor, Value, Vm,
    VmConfig,
};

// Run DllMain and ensure no stdout is produced in silent mode.
#[test]
fn execute_hello_world_dll() {
    let dll_path = std::path::Path::new("debug/hello-world-x86.dll");
    if !dll_path.exists() {
        eprintln!("skipping: {dll_path:?} not found");
        return;
    }

    let mut vm = Vm::new(
        VmConfig::new()
            .os(Os::Windows)
            .architecture(Architecture::X86),
    )
    .expect("vm");
    vm.set_message_box_mode(MessageBoxMode::Silent);
    let pe = Pe::load(&mut vm, dll_path).expect("load");

    windows::register_default(&mut vm);
    vm.resolve_imports(pe.file());

    let image_base = pe.file().optional_header.image_base;
    let mut executor = SymbolExecutor::new(&mut vm, &pe).load("_DllMain@12");
    executor
        .execute(
            &[Value::U32(image_base), Value::U32(1), Value::U32(0)],
            ExecuteOptions::default(),
        )
        .expect("execute");

    let output = vm.stdout_buffer();
    let output = String::from_utf8(output.lock().unwrap().clone()).expect("utf8");
    assert!(output.is_empty());
}
