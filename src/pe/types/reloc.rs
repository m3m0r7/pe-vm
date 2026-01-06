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
