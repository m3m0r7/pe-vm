# pe_vm

pe_vm is a Rust crate that executes Portable Executable (PE) files inside a VM-like
environment, enabling cross-platform execution on Windows (.dll), macOS (.dylib),
and Linux (.so). It is derived from 
[PHP Machine Emulator](https://github.com/m3m0r7/php-machine-emulator).

**Note: This project is a Rust learning project by
[@m3m0r7](https://github.com/m3m0r7).**

Some parts of the DLL specification are still unclear, so behavior may be
approximate and subject to change as we learn more. The current focus is PE32
(32-bit); PE32+ (64-bit) is planned later.

This project targets Windows PE (DLL/EXE) and runs them on a host VM. It does
not load Mach-O or ELF binaries; only PE is supported today.

## Get started

```sh
cargo install pe_vm # Install the crate from crates.io.
```

```rust
use pe_vm::{windows, Architecture, ExecuteOptions, Os, Pe, SymbolExecutor, Value, Vm, VmConfig}; // Import VM APIs.
use std::collections::BTreeMap; // Map for environment variables.

fn main() -> Result<(), Box<dyn std::error::Error>> { // Entry point with error propagation.
    let registry = windows::registry::load_from_yml("/path/to/registry.yml")?; // Load registry mappings.
    // For .reg exports: windows::registry::load_from_registry("path/to/registry.reg")?
    let mut config = VmConfig::new() // Start with default config.
        .os(Os::Windows) // Stored in config (not enforced yet).
        .architecture(Architecture::X86) // X86 only for now (X86_64 planned).
        .properties(registry) // Registry-backed properties for COM lookups.
        .font_path("/path/to/font.ttf"); // Optional; defaults to host fonts.

    let mut paths = Pe::default_path_mapping(); // Default path mappings for guest paths.
    paths.insert("C:\\".to_string(), "/".to_string()); // Override mapping if needed.
    config = config.paths(paths); // Apply mappings to config.

    let mut vm = Vm::new(config)?; // Create a VM.
    let pe = Pe::load(&mut vm, "/path/to/file.dll")?; // Load a PE into the VM.

    println!("{:?}", pe.symbols()); // List registered symbols.
    println!("{:?}", pe.resources()); // Fetch resources.

    let mut env = BTreeMap::new(); // Environment map for execution.
    env.insert("xxx".to_string(), "yyy".to_string()); // Example key/value.

    let options = ExecuteOptions::new().env(env); // Execute options with env.
    let mut executor = SymbolExecutor::new(&mut vm, &pe).load("symbol_name"); // Prepare executor.
    executor.execute( // Execute the symbol with arguments.
        &[
            Value::U32(param1), // First argument.
            Value::U64(param2), // Second argument.
            Value::String(param3.into()), // Third argument.
        ],
        options, // Options including env.
    )?; // Propagate errors.

    Ok(()) // Exit success.
}
```

Note: `VmConfig` stores OS/property/path configuration for future host
integration. Registry and path mappings are used for COM lookups and guest path
resolution; other OS settings are placeholders. `font_path` is used by the SDL
dialog renderer when `MessageBoxA` is called.

## Run hello world

To try `HelloWorld.dll`, download a release build from
https://github.com/carterjones/hello-world-dll (Release assets).

Run the VM (path and symbol are optional):
```
cargo run --example hello_world [path/to/hello-world-x86.dll] [_DllMain@12] # Run the example.
```

If no path is provided, `debug/hello-world-x86.dll` is used by default.
`MessageBoxA` dialogs use SDL2 by default. The dialog font defaults to common
system fonts on macOS/Windows/Linux; override it via `VmConfig::font_path` if
you need a specific typeface. The bundled SDL2 build requires a C toolchain and
CMake.

Sample output:

```
== PE Info ==
entry_point: 0x00001360
image_base:  0x10000000
sections:
  .text    rva=0x00001000 vsize=0x00000CB4 raw=0x00000400
  .rdata   rva=0x00002000 vsize=0x000008B0 raw=0x00001200
  .data    rva=0x00003000 vsize=0x00000388 raw=0x00001C00
  .gfids   rva=0x00004000 vsize=0x00000014 raw=0x00001E00
  .rsrc    rva=0x00005000 vsize=0x000001E0 raw=0x00002000
  .reloc   rva=0x00006000 vsize=0x00000134 raw=0x00002200
exports:
     1 0x00001020 _DllMain@12
     2 0x00001000 _MessageBoxThread@4
imports:
  KERNEL32.dll!CreateThread
  KERNEL32.dll!IsDebuggerPresent
  KERNEL32.dll!InitializeSListHead
  KERNEL32.dll!GetSystemTimeAsFileTime
  KERNEL32.dll!GetCurrentThreadId
  KERNEL32.dll!GetCurrentProcessId
  KERNEL32.dll!QueryPerformanceCounter
  KERNEL32.dll!IsProcessorFeaturePresent
  KERNEL32.dll!TerminateProcess
  KERNEL32.dll!GetCurrentProcess
  KERNEL32.dll!SetUnhandledExceptionFilter
  KERNEL32.dll!UnhandledExceptionFilter
  USER32.dll!MessageBoxA
  VCRUNTIME140.dll!__std_type_info_destroy_list
  VCRUNTIME140.dll!_except_handler4_common
  VCRUNTIME140.dll!memset
  api-ms-win-crt-runtime-l1-1-0.dll!_initialize_onexit_table
  api-ms-win-crt-runtime-l1-1-0.dll!_initialize_narrow_environment
  api-ms-win-crt-runtime-l1-1-0.dll!_seh_filter_dll
  api-ms-win-crt-runtime-l1-1-0.dll!_initterm_e
  api-ms-win-crt-runtime-l1-1-0.dll!_initterm
  api-ms-win-crt-runtime-l1-1-0.dll!_cexit
  api-ms-win-crt-runtime-l1-1-0.dll!_crt_atexit
  api-ms-win-crt-runtime-l1-1-0.dll!_register_onexit_function
  api-ms-win-crt-runtime-l1-1-0.dll!_configure_narrow_argv
  api-ms-win-crt-runtime-l1-1-0.dll!_execute_onexit_table
resources:
  nodes=3 leaves=1 total_size=381
  id=24
    id=2
      id=1033 size=381
selected_symbol (export): _DllMain@12
opcodes (by export):
  _DllMain@12: <unmapped>
  _MessageBoxThread@4:
    0x00001000  6A 00 68 AC 20 00 10 68 BC 20 00 10 6A 00 FF 15
    0x00001010  34 20 00 10 33 C0 C2 04 00 CC CC CC CC CC CC CC
```

Notes:
- `_DllMain@12` is the x86 stdcall-decorated export name. It appears only if the
  DLL actually exports it (as the example does).
- The opcode listing is a raw byte dump at each export RVA, not a full
  disassembly.
- Resource output is summarized to keep it readable.

## C ABI (experimental)

This crate exposes a minimal C ABI for PE inspection so other languages can
reuse the DLL parsing logic without embedding Rust. All exported symbols are
prefixed with `pevm_`.

Basic flow:

- `pevm_pe_open(path)` returns a handle or `NULL`.
- `pevm_last_error()` returns a UTF-8 error string (caller frees with
  `pevm_string_free`).
- Use `pevm_pe_export_count`/`pevm_pe_symbol_count`, `pevm_pe_export_name`/
  `pevm_pe_symbol_name`, `pevm_pe_import_count`, `pevm_pe_resource_count`, etc.
  to query metadata.
- Execute an export with `pevm_pe_execute_symbol_u32`, passing `u32` arguments.
- Call `pevm_pe_close(handle)` when done.

Strings returned by the API are heap-allocated UTF-8 and must be released with
`pevm_string_free`.

Resource paths are flattened as `/`-separated segments; named entries are
prefixed with `name:` (for example: `name:VERSION/1/1033`).
Import ordinals return `0` when no ordinal is present.
`pevm_pe_execute_symbol_u32` returns the EAX value; on failure it returns `0`
and sets `pevm_last_error`.

### Example (C):

```c
#include <stdint.h> // Fixed-width integer types.
#include <stdio.h> // fprintf/printf.

typedef struct PeHandle PeHandle; // Opaque handle type.

extern PeHandle* pevm_pe_open(const char* path); // Open a PE file.
extern void pevm_pe_close(PeHandle* handle); // Close the handle.
extern char* pevm_last_error(void); // Get last error string.
extern void pevm_string_free(char* str); // Free strings from the API.
extern size_t pevm_pe_export_count(const PeHandle* handle); // Export count.
extern char* pevm_pe_export_name(const PeHandle* handle, size_t index); // Export name by index.
extern uint32_t pevm_pe_image_base(const PeHandle* handle); // Image base address.
extern uint32_t pevm_pe_execute_symbol_u32(const PeHandle* handle, // Execute export (u32 args).
                                           const char* name, // Export name.
                                           const uint32_t* args, // Argument array.
                                           size_t args_len); // Argument length.

int main(int argc, char** argv) { // Program entry.
  PeHandle* handle = pevm_pe_open("hello-world-x86.dll"); // Open DLL.
  if (!handle) { // Check for errors.
    char* err = pevm_last_error(); // Fetch error string.
    if (err) { // If an error message exists.
      fprintf(stderr, "pevm error: %s\n", err); // Print error.
      pevm_string_free(err); // Free error string.
    }
    return 1; // Exit failure.
  }
  for (size_t i = 0; i < pevm_pe_export_count(handle); ++i) { // Iterate exports.
    char* name = pevm_pe_export_name(handle, i); // Get export name.
    if (name) { // If name exists.
      printf("export: %s\n", name); // Print export.
      pevm_string_free(name); // Free name string.
    }
  }
  { // Scope for DllMain call.
    uint32_t hinst = pevm_pe_image_base(handle); // Image base as HINSTANCE.
    uint32_t args[3] = {hinst, 1, 0}; // DllMain arguments.
    pevm_pe_execute_symbol_u32(handle, "_DllMain@12", args, 3); // Execute export.
  }
  pevm_pe_close(handle); // Close the handle.
  return 0; // Exit success.
}
```

### Example (PHP FFI):

```php
<?php // PHP FFI example.
$lib = "./libpe_vm.dylib"; // or libpe_vm.so / pe_vm.dll

$ffi = FFI::cdef(' // Define the C ABI signatures for FFI.
    typedef struct PeHandle PeHandle;
    PeHandle* pevm_pe_open(const char* path);
    void pevm_pe_close(PeHandle* handle);
    char* pevm_last_error(void);
    void pevm_string_free(char* str);
    size_t pevm_pe_export_count(const PeHandle* handle);
    char* pevm_pe_export_name(const PeHandle* handle, size_t index);
    uint32_t pevm_pe_image_base(const PeHandle* handle);
    uint32_t pevm_pe_execute_symbol_u32(const PeHandle* handle,
                                        const char* name,
                                        const uint32_t* args,
                                        size_t args_len);
', $lib); // Bind to the shared library.

$handle = $ffi->pevm_pe_open("hello-world-x86.dll"); // Open the DLL.
if (FFI::isNull($handle)) { // Check for errors.
    $err = $ffi->pevm_last_error(); // Fetch error string.
    if (!FFI::isNull($err)) { // If error string exists.
        fwrite(STDERR, "pevm error: " . FFI::string($err) . PHP_EOL); // Print error.
        $ffi->pevm_string_free($err); // Free error string.
    }
    exit(1); // Exit failure.
}

$count = $ffi->pevm_pe_export_count($handle); // Fetch export count.
for ($i = 0; $i < $count; $i++) { // Iterate exports.
    $name = $ffi->pevm_pe_export_name($handle, $i); // Get export name.
    if (!FFI::isNull($name)) { // If name exists.
        echo "export: " . FFI::string($name) . PHP_EOL; // Print export.
        $ffi->pevm_string_free($name); // Free name string.
    }
}

$hinst = $ffi->pevm_pe_image_base($handle); // Image base as HINSTANCE.
$args = FFI::new("uint32_t[3]"); // Allocate arg array.
$args[0] = $hinst; // First arg: HINSTANCE.
$args[1] = 1; // Second arg: reason code.
$args[2] = 0; // Third arg: reserved.
$ffi->pevm_pe_execute_symbol_u32($handle, "_DllMain@12", $args, 3); // Execute export.

$ffi->pevm_pe_close($handle); // Close the handle.
```

### Example (Ruby Fiddle):

```ruby
require "fiddle" # Load Fiddle.
require "fiddle/import" # Import FFI helpers.

module PeVm # Namespace for bindings.
  extend Fiddle::Importer # Enable FFI declarations.
  dlload "./libpe_vm.dylib" # or libpe_vm.so / pe_vm.dll
  typealias "size_t", "size_t" # Ensure size_t type.
  extern "void* pevm_pe_open(char*)" # Open a PE file.
  extern "void pevm_pe_close(void*)" # Close the handle.
  extern "char* pevm_last_error()" # Fetch last error string.
  extern "void pevm_string_free(char*)" # Free strings from the API.
  extern "size_t pevm_pe_export_count(void*)" # Export count.
  extern "char* pevm_pe_export_name(void*, size_t)" # Export name by index.
  extern "uint32_t pevm_pe_image_base(void*)" # Image base address.
  extern "uint32_t pevm_pe_execute_symbol_u32(void*, char*, uint32_t*, size_t)" # Execute export.
end

handle = PeVm.pevm_pe_open("hello-world-x86.dll") # Open DLL.
if handle.to_i == 0 # Check for errors.
  err = PeVm.pevm_last_error # Fetch error string.
  if err && err.to_i != 0 # If error string exists.
    puts "pevm error: #{err.to_s}" # Print error.
    PeVm.pevm_string_free(err) # Free error string.
  end
  exit 1 # Exit failure.
end

count = PeVm.pevm_pe_export_count(handle) # Export count.
count.times do |i| # Iterate exports.
  name = PeVm.pevm_pe_export_name(handle, i) # Get export name.
  if name && name.to_i != 0 # If name exists.
    puts "export: #{name.to_s}" # Print export.
    PeVm.pevm_string_free(name) # Free name string.
  end
end

hinst = PeVm.pevm_pe_image_base(handle) # Image base as HINSTANCE.
args = [hinst, 1, 0].pack("L<3") # Pack args into little-endian u32s.
arg_ptr = Fiddle::Pointer[args] # Pointer to args buffer.
PeVm.pevm_pe_execute_symbol_u32(handle, "_DllMain@12", arg_ptr, 3) # Execute export.

PeVm.pevm_pe_close(handle) # Close the handle.
```

### Example (Python ctypes):

```python
import ctypes # ctypes for FFI.

lib = ctypes.CDLL("./libpe_vm.dylib")  # or libpe_vm.so / pe_vm.dll

lib.pevm_pe_open.argtypes = [ctypes.c_char_p] # Open a PE file.
lib.pevm_pe_open.restype = ctypes.c_void_p # Returns an opaque handle.
lib.pevm_pe_close.argtypes = [ctypes.c_void_p] # Close the handle.
lib.pevm_pe_export_count.argtypes = [ctypes.c_void_p] # Export count.
lib.pevm_pe_export_count.restype = ctypes.c_size_t # size_t return.
lib.pevm_pe_export_name.argtypes = [ctypes.c_void_p, ctypes.c_size_t] # Export name by index.
lib.pevm_pe_export_name.restype = ctypes.c_void_p # Returns a char*.
lib.pevm_pe_image_base.argtypes = [ctypes.c_void_p] # Image base address.
lib.pevm_pe_image_base.restype = ctypes.c_uint32 # u32 return.
lib.pevm_pe_execute_symbol_u32.argtypes = [ # Execute export (u32 args).
    ctypes.c_void_p,
    ctypes.c_char_p,
    ctypes.POINTER(ctypes.c_uint32),
    ctypes.c_size_t,
]
lib.pevm_pe_execute_symbol_u32.restype = ctypes.c_uint32 # Returns EAX.
lib.pevm_last_error.restype = ctypes.c_void_p # Returns a char*.
lib.pevm_string_free.argtypes = [ctypes.c_void_p] # Free strings from the API.

handle = lib.pevm_pe_open(b"hello-world-x86.dll") # Open DLL.
if not handle: # Check for errors.
    err = lib.pevm_last_error() # Fetch error string.
    if err: # If error string exists.
        msg = ctypes.cast(err, ctypes.c_char_p).value.decode("utf-8") # Decode error.
        print("pevm error:", msg) # Print error.
        lib.pevm_string_free(err) # Free error string.
    raise SystemExit(1) # Exit failure.

count = lib.pevm_pe_export_count(handle) # Export count.
for i in range(count): # Iterate exports.
    ptr = lib.pevm_pe_export_name(handle, i) # Get export name.
    if ptr: # If name exists.
        name = ctypes.cast(ptr, ctypes.c_char_p).value.decode("utf-8") # Decode name.
        print("export:", name) # Print export.
        lib.pevm_string_free(ptr) # Free name string.

hinst = lib.pevm_pe_image_base(handle) # Image base as HINSTANCE.
args = (ctypes.c_uint32 * 3)(hinst, 1, 0) # Build arg array.
lib.pevm_pe_execute_symbol_u32(handle, b"_DllMain@12", args, 3) # Execute export.

lib.pevm_pe_close(handle) # Close the handle.
```

## License

MIT
