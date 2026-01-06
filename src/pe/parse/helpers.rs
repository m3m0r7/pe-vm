use super::super::types::DataDirectory;
use super::PeFile;

pub(super) fn directory(dirs: &[DataDirectory], index: usize) -> DataDirectory {
    dirs.get(index)
        .copied()
        .unwrap_or(DataDirectory { rva: 0, size: 0 })
}

pub(super) fn va_to_rva(pe: &PeFile, va: u32) -> Option<u32> {
    if va == 0 {
        return None;
    }
    if va < pe.optional_header.image_base {
        return None;
    }
    Some(va - pe.optional_header.image_base)
}
