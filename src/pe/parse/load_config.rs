use super::super::error::PeParseError;
use super::super::io::{read_u16_opt, read_u32_opt};
use super::super::types::{DataDirectory, LoadConfigDirectory32};
use super::PeFile;

pub(super) fn parse_load_config_directory(
    image: &[u8],
    pe: &PeFile,
    dir: DataDirectory,
) -> Result<Option<LoadConfigDirectory32>, PeParseError> {
    if dir.rva == 0 || dir.size == 0 {
        return Ok(None);
    }
    let offset = pe
        .rva_to_offset(dir.rva)
        .ok_or(PeParseError::Invalid("load config rva"))? as usize;
    if offset + 4 > image.len() {
        return Err(PeParseError::UnexpectedEof("load config"));
    }
    let max = (dir.size as usize).min(image.len().saturating_sub(offset));
    let limit = offset + max;
    let cfg = LoadConfigDirectory32 {
        size: read_u32_opt(image, offset, limit),
        time_date_stamp: read_u32_opt(image, offset + 4, limit),
        major_version: read_u16_opt(image, offset + 8, limit),
        minor_version: read_u16_opt(image, offset + 10, limit),
        global_flags_clear: read_u32_opt(image, offset + 12, limit),
        global_flags_set: read_u32_opt(image, offset + 16, limit),
        critical_section_default_timeout: read_u32_opt(image, offset + 20, limit),
        decommit_free_block_threshold: read_u32_opt(image, offset + 24, limit),
        decommit_total_free_threshold: read_u32_opt(image, offset + 28, limit),
        lock_prefix_table: read_u32_opt(image, offset + 32, limit),
        maximum_allocation_size: read_u32_opt(image, offset + 36, limit),
        virtual_memory_threshold: read_u32_opt(image, offset + 40, limit),
        process_affinity_mask: read_u32_opt(image, offset + 44, limit),
        process_heap_flags: read_u32_opt(image, offset + 48, limit),
        csd_version: read_u16_opt(image, offset + 52, limit),
        dependent_load_flags: read_u16_opt(image, offset + 54, limit),
        edit_list: read_u32_opt(image, offset + 56, limit),
        security_cookie: read_u32_opt(image, offset + 60, limit),
        se_handler_table: read_u32_opt(image, offset + 64, limit),
        se_handler_count: read_u32_opt(image, offset + 68, limit),
        guard_cf_check_function_pointer: read_u32_opt(image, offset + 72, limit),
        guard_cf_dispatch_function_pointer: read_u32_opt(image, offset + 76, limit),
        guard_cf_function_table: read_u32_opt(image, offset + 80, limit),
        guard_cf_function_count: read_u32_opt(image, offset + 84, limit),
        guard_flags: read_u32_opt(image, offset + 88, limit),
    };

    Ok(Some(cfg))
}
