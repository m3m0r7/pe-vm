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
