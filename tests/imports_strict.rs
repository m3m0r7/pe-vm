// Validates that unresolved imports fail fast during resolution.
mod common;

use common::build_test_dll;
use pe_vm::{PeFile, Vm, VmError};

#[test]
fn resolve_imports_reports_missing() {
    let image = build_test_dll();
    let pe = PeFile::parse(&image).expect("parse");
    let mut vm = Vm::load(&pe, &image).expect("load");

    let err = vm.resolve_imports(&pe).expect_err("missing imports");
    match err {
        VmError::MissingImports(missing) => {
            assert!(missing.iter().any(|name| name == "msvcrt.dll!printf"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}
