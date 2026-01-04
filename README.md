# pe_vm

pe_vm is a Rust crate that runs Portable Executable (PE) files inside a VM-like
environment. It is derived from
[PHP Machine Emulator](https://github.com/m3m0r7/php-machine-emulator).

**Note: This project is a Rust learning project by
[@m3m0r7](https://github.com/m3m0r7).**

Some parts of the DLL specification are still unclear, so behavior may be
approximate and subject to change as we learn more.

## Get started

```sh
cargo install pe_vm
```

```rust
use pe_vm::{
    Architecture, Os, Pe, SymbolExecutor, Value, Vm, VmConfig,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = VmConfig::new()
        .os(Os::Windows) // Windows, Unix, or Mac
        .architecture(Architecture::X86) // X86 or X86_64
        .registry("/path/to/registry.yml"); // registry is stored as YAML

    // Map host paths into the VM.
    config = config.paths(
        pe_vm::default_path_mapping()
            // Override mappings when needed.
            .with_override("C:\\", "/"),
    );

    let vm = Vm::create(config)?;
    let pe = Pe::load(&vm, "/path/to/file.dll")?;

    // List registered symbols.
    println!("{:?}", pe.symbols());

    // Fetch resources.
    println!("{:?}", pe.resources());

    // Execute a symbol.
    let mut executor = SymbolExecutor::new(&pe, "symbol_name")?;
    executor.execute(&[
        Value::Env([("xxx", "yyy")].into()),
        Value::U32(param1),
        Value::U64(param2),
        Value::String(param3.into()),
    ])?;

    Ok(())
}
```

## License

MIT
