//! Constants for ADVAPI32 registry stubs.

pub(super) const ERROR_SUCCESS: u32 = 0;
pub(super) const ERROR_FILE_NOT_FOUND: u32 = 2;
pub(super) const ERROR_MORE_DATA: u32 = 234;
pub(super) const ERROR_NO_MORE_ITEMS: u32 = 259;

pub(super) const REG_SZ: u32 = 1;
pub(super) const REG_BINARY: u32 = 3;
pub(super) const REG_DWORD: u32 = 4;
pub(super) const REG_MULTI_SZ: u32 = 7;

pub(super) const HKEY_CLASSES_ROOT: u32 = 0x8000_0000;
pub(super) const HKEY_CURRENT_USER: u32 = 0x8000_0001;
pub(super) const HKEY_LOCAL_MACHINE: u32 = 0x8000_0002;
pub(super) const HKEY_USERS: u32 = 0x8000_0003;
pub(super) const HKEY_CURRENT_CONFIG: u32 = 0x8000_0005;
