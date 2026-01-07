pub(super) const DUMMY_HWND: u32 = 1;
pub(super) const DUMMY_HDC: u32 = 1;
pub(super) const DUMMY_HMONITOR: u32 = 1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dummy_handles_are_nonzero() {
        assert_ne!(DUMMY_HWND, 0);
        assert_ne!(DUMMY_HDC, 0);
        assert_ne!(DUMMY_HMONITOR, 0);
    }

    #[test]
    fn test_dummy_handles_values() {
        assert_eq!(DUMMY_HWND, 1);
        assert_eq!(DUMMY_HDC, 1);
        assert_eq!(DUMMY_HMONITOR, 1);
    }
}
