//! PE header and directory data structures.

#[derive(Debug, Clone)]
pub struct DosHeader {
    pub e_magic: u16,
    pub e_cblp: u16,
    pub e_cp: u16,
    pub e_crlc: u16,
    pub e_cparhdr: u16,
    pub e_minalloc: u16,
    pub e_maxalloc: u16,
    pub e_ss: u16,
    pub e_sp: u16,
    pub e_csum: u16,
    pub e_ip: u16,
    pub e_cs: u16,
    pub e_lfarlc: u16,
    pub e_ovno: u16,
    pub e_res: [u16; 4],
    pub e_oemid: u16,
    pub e_oeminfo: u16,
    pub e_res2: [u16; 10],
    pub e_lfanew: u32,
}

#[derive(Debug, Clone)]
pub struct FileHeader {
    pub machine: u16,
    pub number_of_sections: u16,
    pub time_date_stamp: u32,
    pub pointer_to_symbol_table: u32,
    pub number_of_symbols: u32,
    pub size_of_optional_header: u16,
    pub characteristics: u16,
}

#[derive(Debug, Clone)]
pub struct OptionalHeader32 {
    pub magic: u16,
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub size_of_code: u32,
    pub size_of_initialized_data: u32,
    pub size_of_uninitialized_data: u32,
    pub address_of_entry_point: u32,
    pub base_of_code: u32,
    pub base_of_data: u32,
    pub image_base: u32,
    pub section_alignment: u32,
    pub file_alignment: u32,
    pub major_operating_system_version: u16,
    pub minor_operating_system_version: u16,
    pub major_image_version: u16,
    pub minor_image_version: u16,
    pub major_subsystem_version: u16,
    pub minor_subsystem_version: u16,
    pub win32_version_value: u32,
    pub size_of_image: u32,
    pub size_of_headers: u32,
    pub checksum: u32,
    pub subsystem: u16,
    pub dll_characteristics: u16,
    pub size_of_stack_reserve: u32,
    pub size_of_stack_commit: u32,
    pub size_of_heap_reserve: u32,
    pub size_of_heap_commit: u32,
    pub loader_flags: u32,
    pub number_of_rva_and_sizes: u32,
}

#[derive(Debug, Clone)]
pub struct SectionHeader {
    pub name: String,
    pub virtual_size: u32,
    pub virtual_address: u32,
    pub raw_size: u32,
    pub raw_ptr: u32,
    pub pointer_to_relocations: u32,
    pub pointer_to_linenumbers: u32,
    pub number_of_relocations: u16,
    pub number_of_linenumbers: u16,
    pub characteristics: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct DataDirectory {
    pub rva: u32,
    pub size: u32,
}

#[derive(Debug, Clone)]
pub struct ImportSymbol {
    pub module: String,
    pub name: Option<String>,
    pub ordinal: Option<u16>,
    pub hint: Option<u16>,
    pub iat_rva: u32,
}

#[derive(Debug, Clone)]
pub struct ImportDescriptor {
    pub module: String,
    pub original_first_thunk: u32,
    pub time_date_stamp: u32,
    pub forwarder_chain: u32,
    pub first_thunk: u32,
    pub symbols: Vec<ImportSymbol>,
}

#[derive(Debug, Clone)]
pub struct ImportDirectory {
    pub descriptors: Vec<ImportDescriptor>,
}

#[derive(Debug, Clone)]
pub struct DelayImportSymbol {
    pub module: String,
    pub name: Option<String>,
    pub ordinal: Option<u16>,
    pub hint: Option<u16>,
    pub iat_rva: u32,
}

#[derive(Debug, Clone)]
pub struct DelayImportDescriptor {
    pub module: String,
    pub attributes: u32,
    pub name_rva: u32,
    pub module_handle_rva: u32,
    pub delay_import_address_table: u32,
    pub delay_import_name_table: u32,
    pub bound_delay_import_table: u32,
    pub unload_delay_import_table: u32,
    pub time_date_stamp: u32,
    pub symbols: Vec<DelayImportSymbol>,
}

#[derive(Debug, Clone)]
pub struct DelayImportDirectory {
    pub descriptors: Vec<DelayImportDescriptor>,
}

#[derive(Debug, Clone)]
pub struct BoundForwarderRef {
    pub module: String,
    pub time_date_stamp: u32,
}

#[derive(Debug, Clone)]
pub struct BoundImportDescriptor {
    pub module: String,
    pub time_date_stamp: u32,
    pub forwarder_refs: Vec<BoundForwarderRef>,
}

#[derive(Debug, Clone)]
pub struct BoundImportDirectory {
    pub descriptors: Vec<BoundImportDescriptor>,
}

#[derive(Debug, Clone)]
pub struct ExportSymbol {
    pub name: Option<String>,
    pub ordinal: u16,
    pub rva: u32,
    pub forwarder: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ExportDirectory {
    pub name: Option<String>,
    pub ordinal_base: u32,
    pub symbols: Vec<ExportSymbol>,
}

#[derive(Debug, Clone)]
pub struct RelocationEntry {
    pub rva: u32,
    pub reloc_type: u8,
}

#[derive(Debug, Clone)]
pub struct RelocationBlock {
    pub page_rva: u32,
    pub entries: Vec<RelocationEntry>,
}

#[derive(Debug, Clone)]
pub struct RelocationDirectory {
    pub blocks: Vec<RelocationBlock>,
}

#[derive(Debug, Clone)]
pub enum ResourceId {
    Id(u32),
    Name(String),
}

#[derive(Debug, Clone)]
pub struct ResourceData {
    pub rva: u32,
    pub size: u32,
    pub codepage: u32,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct ResourceNode {
    pub id: ResourceId,
    pub children: Vec<ResourceNode>,
    pub data: Option<ResourceData>,
}

#[derive(Debug, Clone)]
pub struct ResourceDirectory {
    pub roots: Vec<ResourceNode>,
}

#[derive(Debug, Clone)]
pub struct DebugDirectoryEntry {
    pub characteristics: u32,
    pub time_date_stamp: u32,
    pub major_version: u16,
    pub minor_version: u16,
    pub debug_type: u32,
    pub size_of_data: u32,
    pub address_of_raw_data: u32,
    pub pointer_to_raw_data: u32,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct DebugDirectory {
    pub entries: Vec<DebugDirectoryEntry>,
}

#[derive(Debug, Clone)]
pub struct TlsDirectory {
    pub start_raw_data: u32,
    pub end_raw_data: u32,
    pub address_of_index: u32,
    pub address_of_callbacks: u32,
    pub size_of_zero_fill: u32,
    pub characteristics: u32,
    pub callbacks: Vec<u32>,
}

#[derive(Debug, Clone, Default)]
pub struct LoadConfigDirectory32 {
    pub size: Option<u32>,
    pub time_date_stamp: Option<u32>,
    pub major_version: Option<u16>,
    pub minor_version: Option<u16>,
    pub global_flags_clear: Option<u32>,
    pub global_flags_set: Option<u32>,
    pub critical_section_default_timeout: Option<u32>,
    pub decommit_free_block_threshold: Option<u32>,
    pub decommit_total_free_threshold: Option<u32>,
    pub lock_prefix_table: Option<u32>,
    pub maximum_allocation_size: Option<u32>,
    pub virtual_memory_threshold: Option<u32>,
    pub process_affinity_mask: Option<u32>,
    pub process_heap_flags: Option<u32>,
    pub csd_version: Option<u16>,
    pub dependent_load_flags: Option<u16>,
    pub edit_list: Option<u32>,
    pub security_cookie: Option<u32>,
    pub se_handler_table: Option<u32>,
    pub se_handler_count: Option<u32>,
    pub guard_cf_check_function_pointer: Option<u32>,
    pub guard_cf_dispatch_function_pointer: Option<u32>,
    pub guard_cf_function_table: Option<u32>,
    pub guard_cf_function_count: Option<u32>,
    pub guard_flags: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct IatDirectory {
    pub rva: u32,
    pub size: u32,
    pub entries: Vec<u32>,
}

#[derive(Debug, Clone)]
pub struct SecurityDirectory {
    pub file_offset: u32,
    pub size: u32,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct ClrDirectory {
    pub cb: u32,
    pub major_runtime_version: u16,
    pub minor_runtime_version: u16,
    pub metadata: DataDirectory,
    pub flags: u32,
    pub entry_point_token: u32,
    pub resources: DataDirectory,
    pub strong_name_signature: DataDirectory,
    pub code_manager_table: DataDirectory,
    pub vtable_fixups: DataDirectory,
    pub export_address_table_jumps: DataDirectory,
    pub managed_native_header: DataDirectory,
}

#[derive(Debug, Clone, Default)]
pub struct PeDirectories {
    pub export: Option<ExportDirectory>,
    pub import: Option<ImportDirectory>,
    pub resource: Option<ResourceDirectory>,
    pub exception: Option<Vec<u8>>,
    pub security: Option<SecurityDirectory>,
    pub reloc: Option<RelocationDirectory>,
    pub debug: Option<DebugDirectory>,
    pub architecture: Option<Vec<u8>>,
    pub global_ptr: Option<u32>,
    pub tls: Option<TlsDirectory>,
    pub load_config: Option<LoadConfigDirectory32>,
    pub bound_import: Option<BoundImportDirectory>,
    pub iat: Option<IatDirectory>,
    pub delay_import: Option<DelayImportDirectory>,
    pub clr: Option<ClrDirectory>,
}
