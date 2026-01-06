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
