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
