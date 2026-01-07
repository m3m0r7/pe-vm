use pe_vm::{
    Architecture, ExecuteOptions, Os, Pe, Renderer, SymbolExecutor, Value, Vm, VmConfig,
};
use std::collections::BTreeMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use Renderer::Stdout for CLI testing (no SDL window)
    let config = VmConfig::new()
        .os(Os::Windows)
        .architecture(Architecture::X86)
        .renderer(Renderer::Stdout);
    let mut vm = Vm::new(config)?;
    let args: Vec<String> = std::env::args().collect();
    let dll_path = args
        .get(1)
        .map(|value| value.as_str())
        .unwrap_or("debug/hello-world-x86.dll");
    let symbol_arg = args.get(2).map(|value| value.as_str());

    let pe = Pe::load(&mut vm, dll_path)?;

    let selected_symbol = select_symbol(&pe, symbol_arg)?;
    print_dll_info(&pe, &selected_symbol);
    print_export_opcodes(&pe, 256);

    let image_base = pe.file().optional_header.image_base;
    let mut executor = SymbolExecutor::new(&mut vm, &pe).load(&selected_symbol);
    let mut env = BTreeMap::new();
    env.insert("xxx".to_string(), "yyy".to_string());
    let options = ExecuteOptions::new().env(env);
    executor.execute(
        &[Value::U32(image_base), Value::U32(1), Value::U32(0)],
        options,
    )?;

    let output = vm.stdout_buffer();
    let output = String::from_utf8(output.lock().unwrap().clone())?;
    print!("{output}");

    Ok(())
}

fn print_dll_info(pe: &Pe, selected_symbol: &str) {
    let file = pe.file();
    println!("== PE Info ==");
    println!(
        "entry_point: 0x{:08X}",
        file.optional_header.address_of_entry_point
    );
    println!("image_base:  0x{:08X}", file.optional_header.image_base);
    println!("sections:");
    for section in &file.sections {
        println!(
            "  {:<8} rva=0x{:08X} vsize=0x{:08X} raw=0x{:08X}",
            section.name, section.virtual_address, section.virtual_size, section.raw_ptr
        );
    }

    println!("exports:");
    for symbol in pe.symbols() {
        let name = symbol.name.as_deref().unwrap_or("<ordinal>");
        if let Some(forwarder) = &symbol.forwarder {
            println!(
                "  {:>4} 0x{:08X} {} -> {}",
                symbol.ordinal, symbol.rva, name, forwarder
            );
        } else {
            println!("  {:>4} 0x{:08X} {}", symbol.ordinal, symbol.rva, name);
        }
    }

    println!("imports:");
    for import in &file.imports {
        if let Some(name) = &import.name {
            println!("  {}!{}", import.module, name);
        } else if let Some(ordinal) = import.ordinal {
            println!("  {}!#{}", import.module, ordinal);
        } else {
            println!("  {}!<unknown>", import.module);
        }
    }

    println!("resources:");
    print_resource_summary(pe.resources());

    println!("selected_symbol (export): {selected_symbol}");
}

fn select_symbol(pe: &Pe, symbol_arg: Option<&str>) -> Result<String, Box<dyn std::error::Error>> {
    if let Some(name) = symbol_arg {
        if !pe
            .symbols()
            .iter()
            .any(|symbol| symbol.name.as_deref() == Some(name))
        {
            return Err(format!("symbol not found: {name}").into());
        }
        return Ok(name.to_string());
    }
    if pe
        .symbols()
        .iter()
        .any(|symbol| symbol.name.as_deref() == Some("_DllMain@12"))
    {
        return Ok("_DllMain@12".to_string());
    }
    let fallback = pe
        .symbols()
        .iter()
        .find_map(|symbol| symbol.name.as_deref())
        .ok_or("no named exports found")?;
    Ok(fallback.to_string())
}

fn print_resource_summary(resources: Option<&pe_vm::ResourceDirectory>) {
    let Some(resources) = resources else {
        println!("  <none>");
        return;
    };
    let mut stats = ResourceStats::default();
    for node in &resources.roots {
        collect_resource_stats(node, &mut stats);
    }
    println!(
        "  nodes={} leaves={} total_size={}",
        stats.nodes, stats.leaves, stats.total_size
    );
    let mut shown = 0usize;
    for node in &resources.roots {
        print_resource_node(node, 1, &mut shown, 32);
    }
    if shown >= 32 {
        println!("  ...");
    }
}

#[derive(Default)]
struct ResourceStats {
    nodes: usize,
    leaves: usize,
    total_size: u32,
}

fn collect_resource_stats(node: &pe_vm::ResourceNode, stats: &mut ResourceStats) {
    stats.nodes += 1;
    if let Some(data) = &node.data {
        stats.leaves += 1;
        stats.total_size = stats.total_size.saturating_add(data.size);
    }
    for child in &node.children {
        collect_resource_stats(child, stats);
    }
}

fn print_resource_node(node: &pe_vm::ResourceNode, depth: usize, shown: &mut usize, limit: usize) {
    if *shown >= limit {
        return;
    }
    *shown += 1;
    let indent = "  ".repeat(depth);
    let id = match &node.id {
        pe_vm::ResourceId::Id(value) => format!("id={value}"),
        pe_vm::ResourceId::Name(name) => format!("name={name}"),
    };
    if let Some(data) = &node.data {
        println!("{indent}{id} size={}", data.size);
    } else {
        println!("{indent}{id}");
    }
    for child in &node.children {
        print_resource_node(child, depth + 1, shown, limit);
    }
}

fn print_export_opcodes(pe: &Pe, max_bytes: usize) {
    if pe.symbols().is_empty() {
        println!("opcodes: <no exports>");
        return;
    }
    println!("opcodes (by export):");
    for symbol in pe.symbols() {
        let name = symbol.name.as_deref().unwrap_or("<ordinal>");
        if symbol.forwarder.is_some() || symbol.rva == 0 {
            println!("  {name}: <forwarder or empty>");
            continue;
        }
        let Some((start_rva, bytes)) = symbol_bytes(pe, symbol) else {
            println!("  {name}: <unmapped>");
            continue;
        };
        println!("  {name}:");
        let limit = max_bytes.min(bytes.len());
        for (index, chunk) in bytes[..limit].chunks(16).enumerate() {
            let addr = start_rva + (index as u32) * 16;
            let line = chunk
                .iter()
                .map(|byte| format!("{byte:02X}"))
                .collect::<Vec<_>>()
                .join(" ");
            println!("    0x{addr:08X}  {line}");
        }
        if bytes.len() > limit {
            println!("    ... {} bytes omitted", bytes.len() - limit);
        }
    }
}

fn symbol_bytes<'a>(pe: &'a Pe, symbol: &pe_vm::ExportSymbol) -> Option<(u32, &'a [u8])> {
    let start = symbol.rva;
    let end = symbol_end_rva(pe, start)?;
    let start_off = pe.file().rva_to_offset(start)? as usize;
    let end_off = pe.file().rva_to_offset(end)? as usize;
    if start_off >= pe.image().len() || end_off > pe.image().len() || end_off <= start_off {
        return None;
    }
    Some((start, &pe.image()[start_off..end_off]))
}

fn symbol_end_rva(pe: &Pe, start_rva: u32) -> Option<u32> {
    let file = pe.file();
    let section_end = file
        .sections
        .iter()
        .find(|section| {
            start_rva >= section.virtual_address
                && start_rva < section.virtual_address + section.raw_size
        })
        .map(|section| section.virtual_address + section.raw_size)?;
    let mut next_rva: Option<u32> = None;
    for symbol in pe.symbols() {
        if symbol.rva > start_rva {
            next_rva = Some(match next_rva {
                Some(current) => current.min(symbol.rva),
                None => symbol.rva,
            });
        }
    }
    Some(match next_rva {
        Some(next) if next < section_end => next,
        _ => section_end,
    })
}
