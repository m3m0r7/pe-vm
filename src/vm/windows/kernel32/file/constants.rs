pub(super) const FILE_ATTRIBUTE_NORMAL: u32 = 0x80;
pub(super) const FILE_TYPE_DISK: u32 = 1;
pub(super) const DRIVE_FIXED: u32 = 3;
pub(super) const INVALID_HANDLE_VALUE: u32 = 0xFFFF_FFFF;
pub(super) const INVALID_FILE_ATTRIBUTES: u32 = 0xFFFF_FFFF;

pub(super) const GENERIC_READ: u32 = 0x8000_0000;
pub(super) const GENERIC_WRITE: u32 = 0x4000_0000;

pub(super) const CREATE_NEW: u32 = 1;
pub(super) const CREATE_ALWAYS: u32 = 2;
pub(super) const OPEN_EXISTING: u32 = 3;
pub(super) const OPEN_ALWAYS: u32 = 4;
pub(super) const TRUNCATE_EXISTING: u32 = 5;

pub(super) const FILE_BEGIN: u32 = 0;

pub(super) const ERROR_FILE_NOT_FOUND: u32 = 2;
pub(super) const ERROR_INVALID_HANDLE: u32 = 6;
