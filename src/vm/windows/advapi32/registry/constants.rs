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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_codes() {
        assert_eq!(ERROR_SUCCESS, 0);
        assert_eq!(ERROR_FILE_NOT_FOUND, 2);
        assert_eq!(ERROR_MORE_DATA, 234);
        assert_eq!(ERROR_NO_MORE_ITEMS, 259);
    }

    #[test]
    fn test_registry_types() {
        assert_eq!(REG_SZ, 1);
        assert_eq!(REG_BINARY, 3);
        assert_eq!(REG_DWORD, 4);
        assert_eq!(REG_MULTI_SZ, 7);
    }

    #[test]
    fn test_hkey_values() {
        assert_eq!(HKEY_CLASSES_ROOT, 0x8000_0000);
        assert_eq!(HKEY_CURRENT_USER, 0x8000_0001);
        assert_eq!(HKEY_LOCAL_MACHINE, 0x8000_0002);
        assert_eq!(HKEY_USERS, 0x8000_0003);
        assert_eq!(HKEY_CURRENT_CONFIG, 0x8000_0005);
    }

    #[test]
    fn test_hkey_values_are_distinct() {
        let hkeys = [
            HKEY_CLASSES_ROOT,
            HKEY_CURRENT_USER,
            HKEY_LOCAL_MACHINE,
            HKEY_USERS,
            HKEY_CURRENT_CONFIG,
        ];
        for (i, &a) in hkeys.iter().enumerate() {
            for (j, &b) in hkeys.iter().enumerate() {
                if i != j {
                    assert_ne!(a, b);
                }
            }
        }
    }
}
