// Tests PE parsing and execution using a synthetic printf DLL.
mod common;

use common::build_test_dll;
use pe_vm::{host_printf, PeFile, Vm};

// Verify that imports and exports are parsed from the synthetic image.
#[test]
fn parse_imports_and_exports() {
    let image = build_test_dll();
    let pe = PeFile::parse(&image).expect("parse");

    assert!(pe
        .imports
        .iter()
        .any(|imp| imp.name.as_deref() == Some("printf")));
    assert!(pe
        .exports
        .iter()
        .any(|exp| exp.name.as_deref() == Some("hello")));
}

// Execute the exported hello routine and confirm stdout output.
#[test]
fn execute_hello_printf() {
    let image = build_test_dll();
    let pe = PeFile::parse(&image).expect("parse");

    let mut vm = Vm::load(&pe, &image).expect("load");
    vm.register_import("msvcrt.dll", "printf", host_printf);
    vm.resolve_imports(&pe).expect("imports");
    vm.call_export(&pe, "hello").expect("execute");

    let output = vm.stdout_buffer();
    let output = String::from_utf8(output.lock().unwrap().clone()).expect("utf8");
    assert_eq!(output, "Hello, world!\n");
}
