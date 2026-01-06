#[derive(Debug, Clone)]
pub struct SecurityDirectory {
    pub file_offset: u32,
    pub size: u32,
    pub data: Vec<u8>,
}
