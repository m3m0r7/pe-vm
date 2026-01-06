use crate::pe::{ResourceData, ResourceDirectory, ResourceId, ResourceNode};

const TYPELIB_RESOURCE_ID: u32 = 6;

pub(super) fn find_typelib_resource(dir: &ResourceDirectory) -> Option<&ResourceData> {
    let node = dir.roots.iter().find(|node| matches_typelib_id(&node.id))?;
    find_first_resource(node)
}

fn matches_typelib_id(id: &ResourceId) -> bool {
    match id {
        ResourceId::Id(value) => *value == TYPELIB_RESOURCE_ID,
        ResourceId::Name(name) => name.eq_ignore_ascii_case("TYPELIB"),
    }
}

fn find_first_resource(node: &ResourceNode) -> Option<&ResourceData> {
    if let Some(data) = node.data.as_ref() {
        return Some(data);
    }
    for child in &node.children {
        if let Some(data) = find_first_resource(child) {
            return Some(data);
        }
    }
    None
}
