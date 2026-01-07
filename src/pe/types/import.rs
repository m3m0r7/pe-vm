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
pub struct IatDirectory {
    pub rva: u32,
    pub size: u32,
    pub entries: Vec<u32>,
}
