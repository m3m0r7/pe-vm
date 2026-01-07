use crate::vm::{Os, Vm};
use crate::vm_args;

/// Windows code page identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum CodePage {
    /// Default ANSI code page (use ACP).
    Default = 0,
    /// OEM code page (use OEMCP).
    Oem = 1,
    /// MAC code page.
    Mac = 2,
    /// Thread's ANSI code page.
    ThreadAnsi = 3,
    /// Symbol code page.
    Symbol = 42,
    /// UTF-7 code page.
    Utf7 = 65000,
    /// UTF-8 code page.
    Utf8 = 65001,
    /// Japanese Shift-JIS.
    ShiftJis = 932,
    /// Simplified Chinese GBK.
    Gbk = 936,
    /// Korean.
    Korean = 949,
    /// Traditional Chinese Big5.
    Big5 = 950,
    /// Western European Latin-1.
    Latin1 = 1252,
}

impl CodePage {
    /// Convert from u32 code page ID to CodePage enum.
    pub fn from_id(id: u32) -> Option<Self> {
        match id {
            0 => Some(Self::Default),
            1 => Some(Self::Oem),
            2 => Some(Self::Mac),
            3 => Some(Self::ThreadAnsi),
            42 => Some(Self::Symbol),
            65000 => Some(Self::Utf7),
            65001 => Some(Self::Utf8),
            932 => Some(Self::ShiftJis),
            936 => Some(Self::Gbk),
            949 => Some(Self::Korean),
            950 => Some(Self::Big5),
            1252 => Some(Self::Latin1),
            _ => None,
        }
    }

    /// Get the u32 code page ID.
    pub fn id(self) -> u32 {
        self as u32
    }

    /// Check if this code page uses multi-byte characters.
    pub fn is_multibyte(self) -> bool {
        matches!(
            self,
            Self::ShiftJis | Self::Gbk | Self::Korean | Self::Big5 | Self::Utf8
        )
    }

    /// Get the maximum bytes per character for this code page.
    pub fn max_char_size(self) -> u32 {
        match self {
            Self::ShiftJis | Self::Gbk | Self::Korean | Self::Big5 => 2,
            Self::Utf8 => 4,
            _ => 1,
        }
    }
}

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "GetACP",
        crate::vm::stdcall_args(0),
        get_acp,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "GetOEMCP",
        crate::vm::stdcall_args(0),
        get_oemcp,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "AreFileApisANSI",
        crate::vm::stdcall_args(0),
        are_file_apis_ansi,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "IsValidCodePage",
        crate::vm::stdcall_args(1),
        is_valid_code_page,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "GetCPInfo",
        crate::vm::stdcall_args(2),
        get_cp_info,
    );
}

fn get_acp(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    default_ansi_codepage(_vm)
}

fn get_oemcp(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    default_ansi_codepage(_vm)
}

fn are_file_apis_ansi(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn is_valid_code_page(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn get_cp_info(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (code_page, info_ptr) = vm_args!(vm, stack_ptr; u32, u32);
    if info_ptr == 0 {
        return 0;
    }
    let resolved = resolve_code_page(vm, code_page);
    if resolved == 932 {
        let _ = vm.write_u32(info_ptr, 2);
        let _ = vm.write_u8(info_ptr + 4, b'?');
        let _ = vm.write_u8(info_ptr + 5, 0);
        let lead_bytes = [(0x81, 0x9F), (0xE0, 0xFC)];
        let mut cursor = info_ptr + 6;
        for (start, end) in lead_bytes {
            let _ = vm.write_u8(cursor, start);
            let _ = vm.write_u8(cursor + 1, end);
            cursor += 2;
        }
        for idx in 0..(12 - lead_bytes.len() * 2) {
            let _ = vm.write_u8(cursor + idx as u32, 0);
        }
    } else {
        let _ = vm.write_u32(info_ptr, 1);
        let _ = vm.write_u8(info_ptr + 4, b'?');
        let _ = vm.write_u8(info_ptr + 5, 0);
        for idx in 0..12 {
            let _ = vm.write_u8(info_ptr + 6 + idx, 0);
        }
    }
    1
}

pub(crate) fn resolve_code_page(vm: &Vm, code_page: u32) -> u32 {
    if code_page == 0 {
        default_ansi_codepage(vm)
    } else {
        code_page
    }
}

fn default_ansi_codepage(vm: &Vm) -> u32 {
    match vm.config().os_value() {
        Os::Windows => 932,
        _ => 65001,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::{Architecture, VmConfig};
    use crate::vm_set_args;

    fn create_test_vm() -> Vm {
        let mut vm = Vm::new(VmConfig::new().architecture(Architecture::X86)).expect("vm");
        vm.memory = vec![0u8; 0x10000];
        vm.base = 0x1000;
        vm.stack_top = 0x1000 + 0x10000 - 4;
        vm.regs.esp = vm.stack_top;
        vm.heap_start = 0x2000;
        vm.heap_end = 0x8000;
        vm.heap_cursor = vm.heap_start;
        vm
    }

    #[test]
    fn test_codepage_from_id_valid() {
        assert_eq!(CodePage::from_id(0), Some(CodePage::Default));
        assert_eq!(CodePage::from_id(65001), Some(CodePage::Utf8));
        assert_eq!(CodePage::from_id(932), Some(CodePage::ShiftJis));
        assert_eq!(CodePage::from_id(1252), Some(CodePage::Latin1));
    }

    #[test]
    fn test_codepage_from_id_invalid() {
        assert_eq!(CodePage::from_id(99999), None);
        assert_eq!(CodePage::from_id(12345), None);
    }

    #[test]
    fn test_codepage_id_roundtrip() {
        let pages = [
            CodePage::Default,
            CodePage::Utf8,
            CodePage::ShiftJis,
            CodePage::Gbk,
            CodePage::Korean,
            CodePage::Big5,
            CodePage::Latin1,
        ];
        for cp in pages {
            assert_eq!(CodePage::from_id(cp.id()), Some(cp));
        }
    }

    #[test]
    fn test_codepage_is_multibyte() {
        assert!(CodePage::ShiftJis.is_multibyte());
        assert!(CodePage::Gbk.is_multibyte());
        assert!(CodePage::Korean.is_multibyte());
        assert!(CodePage::Big5.is_multibyte());
        assert!(CodePage::Utf8.is_multibyte());
        assert!(!CodePage::Latin1.is_multibyte());
        assert!(!CodePage::Default.is_multibyte());
    }

    #[test]
    fn test_codepage_max_char_size() {
        assert_eq!(CodePage::ShiftJis.max_char_size(), 2);
        assert_eq!(CodePage::Gbk.max_char_size(), 2);
        assert_eq!(CodePage::Utf8.max_char_size(), 4);
        assert_eq!(CodePage::Latin1.max_char_size(), 1);
    }

    #[test]
    fn test_get_acp_returns_codepage() {
        let mut vm = create_test_vm();
        let result = get_acp(&mut vm, 0);
        // Windows default is 932 (Shift-JIS)
        assert_eq!(result, 932);
    }

    #[test]
    fn test_get_oemcp_returns_codepage() {
        let mut vm = create_test_vm();
        let result = get_oemcp(&mut vm, 0);
        assert_eq!(result, 932);
    }

    #[test]
    fn test_are_file_apis_ansi_returns_one() {
        let mut vm = create_test_vm();
        let result = are_file_apis_ansi(&mut vm, 0);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_is_valid_code_page_returns_one() {
        let mut vm = create_test_vm();
        let result = is_valid_code_page(&mut vm, 0);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_get_cp_info_null_ptr_returns_zero() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 12;
        vm_set_args!(vm, stack; 932u32, 0u32);
        let result = get_cp_info(&mut vm, stack);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_get_cp_info_shift_jis() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 12;
        let info_ptr = vm.heap_start as u32;
        vm_set_args!(vm, stack; 932u32, info_ptr);
        let result = get_cp_info(&mut vm, stack);
        assert_eq!(result, 1);
        // MaxCharSize should be 2 for Shift-JIS
        assert_eq!(vm.read_u32(info_ptr).unwrap(), 2);
    }

    #[test]
    fn test_get_cp_info_single_byte() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 12;
        let info_ptr = vm.heap_start as u32;
        vm_set_args!(vm, stack; 1252u32, info_ptr);
        let result = get_cp_info(&mut vm, stack);
        assert_eq!(result, 1);
        // MaxCharSize should be 1 for Latin-1
        assert_eq!(vm.read_u32(info_ptr).unwrap(), 1);
    }

    #[test]
    fn test_resolve_code_page_zero() {
        let vm = create_test_vm();
        let resolved = resolve_code_page(&vm, 0);
        assert_eq!(resolved, 932); // default ACP for Windows
    }

    #[test]
    fn test_resolve_code_page_nonzero() {
        let vm = create_test_vm();
        let resolved = resolve_code_page(&vm, 65001);
        assert_eq!(resolved, 65001);
    }
}
