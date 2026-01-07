//! String function stubs for MSVCR100.dll.

use crate::vm::windows::check_stub;
use crate::vm::Vm;

const DLL: &str = "MSVCR100.dll";

macro_rules! stub {
    ($name:ident) => {
        fn $name(vm: &mut Vm, _sp: u32) -> u32 {
            check_stub(vm, DLL, stringify!($name));
            0
        }
    };
}

// String functions
stub!(strcpy_impl);
stub!(strcpy_s_impl);
stub!(strncpy_impl);
stub!(strncpy_s_impl);
stub!(strcat_impl);
stub!(strcat_s_impl);
stub!(strncat_impl);
stub!(strncat_s_impl);
stub!(strcmp_impl);
stub!(strncmp_impl);
stub!(stricmp_impl);
stub!(strnicmp_impl);
stub!(strcoll_impl);
stub!(strcoll_l_impl);
stub!(stricoll_impl);
stub!(stricoll_l_impl);
stub!(strncoll_impl);
stub!(strncoll_l_impl);
stub!(strnicoll_impl);
stub!(strnicoll_l_impl);
stub!(strlen_impl);
stub!(strnlen_impl);
stub!(strchr_impl);
stub!(strrchr_impl);
stub!(strstr_impl);
stub!(strpbrk_impl);
stub!(strspn_impl);
stub!(strcspn_impl);
stub!(strtok_impl);
stub!(strtok_s_impl);
stub!(strdup_impl);
stub!(strlwr_impl);
stub!(strlwr_s_impl);
stub!(strlwr_s_l_impl);
stub!(strupr_impl);
stub!(strupr_s_impl);
stub!(strupr_s_l_impl);
stub!(strrev_impl);
stub!(strset_impl);
stub!(strset_s_impl);
stub!(strnset_impl);
stub!(strnset_s_impl);
stub!(strxfrm_impl);
stub!(strxfrm_l_impl);
stub!(strerror_impl);
stub!(strerror_s_impl);
stub!(strerror_l_impl);

// Wide string functions
stub!(wcscpy_impl);
stub!(wcscpy_s_impl);
stub!(wcsncpy_impl);
stub!(wcsncpy_s_impl);
stub!(wcscat_impl);
stub!(wcscat_s_impl);
stub!(wcsncat_impl);
stub!(wcsncat_s_impl);
stub!(wcscmp_impl);
stub!(wcsncmp_impl);
stub!(wcsicmp_impl);
stub!(wcsicmp_l_impl);
stub!(wcsnicmp_impl);
stub!(wcsnicmp_l_impl);
stub!(wcscoll_impl);
stub!(wcscoll_l_impl);
stub!(wcsicoll_impl);
stub!(wcsicoll_l_impl);
stub!(wcsncoll_impl);
stub!(wcsncoll_l_impl);
stub!(wcsnicoll_impl);
stub!(wcsnicoll_l_impl);
stub!(wcslen_impl);
stub!(wcsnlen_impl);
stub!(wcschr_impl);
stub!(wcsrchr_impl);
stub!(wcsstr_impl);
stub!(wcspbrk_impl);
stub!(wcsspn_impl);
stub!(wcscspn_impl);
stub!(wcstok_impl);
stub!(wcstok_s_impl);
stub!(wcsdup_impl);
stub!(wcslwr_impl);
stub!(wcslwr_s_impl);
stub!(wcslwr_s_l_impl);
stub!(wcsupr_impl);
stub!(wcsupr_s_impl);
stub!(wcsupr_s_l_impl);
stub!(wcsrev_impl);
stub!(wcsset_impl);
stub!(wcsset_s_impl);
stub!(wcsnset_impl);
stub!(wcsnset_s_impl);
stub!(wcsxfrm_impl);
stub!(wcsxfrm_l_impl);
stub!(wcserror_impl);
stub!(wcserror_s_impl);

// Memory functions
stub!(memcpy_impl);
stub!(memcpy_s_impl);
stub!(memmove_impl);
stub!(memmove_s_impl);
stub!(memset_impl);
stub!(memcmp_impl);
stub!(memchr_impl);
stub!(memccpy_impl);
stub!(memicmp_impl);
stub!(memicmp_l_impl);

// Wide memory functions
stub!(wmemcpy_s_impl);
stub!(wmemmove_s_impl);

pub fn register(vm: &mut Vm) {
    // Standard C string functions
    vm.register_import(DLL, "strcpy", strcpy_impl);
    vm.register_import(DLL, "strcpy_s", strcpy_s_impl);
    vm.register_import(DLL, "strncpy", strncpy_impl);
    vm.register_import(DLL, "strncpy_s", strncpy_s_impl);
    vm.register_import(DLL, "strcat", strcat_impl);
    vm.register_import(DLL, "strcat_s", strcat_s_impl);
    vm.register_import(DLL, "strncat", strncat_impl);
    vm.register_import(DLL, "strncat_s", strncat_s_impl);
    vm.register_import(DLL, "strcmp", strcmp_impl);
    vm.register_import(DLL, "strncmp", strncmp_impl);
    vm.register_import(DLL, "_stricmp", stricmp_impl);
    vm.register_import(DLL, "_stricmp_l", stricmp_impl);
    vm.register_import(DLL, "_strnicmp", strnicmp_impl);
    vm.register_import(DLL, "_strnicmp_l", strnicmp_impl);
    vm.register_import(DLL, "strcoll", strcoll_impl);
    vm.register_import(DLL, "_strcoll_l", strcoll_l_impl);
    vm.register_import(DLL, "_stricoll", stricoll_impl);
    vm.register_import(DLL, "_stricoll_l", stricoll_l_impl);
    vm.register_import(DLL, "_strncoll", strncoll_impl);
    vm.register_import(DLL, "_strncoll_l", strncoll_l_impl);
    vm.register_import(DLL, "_strnicoll", strnicoll_impl);
    vm.register_import(DLL, "_strnicoll_l", strnicoll_l_impl);
    vm.register_import(DLL, "strlen", strlen_impl);
    vm.register_import(DLL, "strnlen", strnlen_impl);
    vm.register_import(DLL, "strchr", strchr_impl);
    vm.register_import(DLL, "strrchr", strrchr_impl);
    vm.register_import(DLL, "strstr", strstr_impl);
    vm.register_import(DLL, "strpbrk", strpbrk_impl);
    vm.register_import(DLL, "strspn", strspn_impl);
    vm.register_import(DLL, "strcspn", strcspn_impl);
    vm.register_import(DLL, "strtok", strtok_impl);
    vm.register_import(DLL, "strtok_s", strtok_s_impl);
    vm.register_import(DLL, "_strdup", strdup_impl);
    vm.register_import(DLL, "_strlwr", strlwr_impl);
    vm.register_import(DLL, "_strlwr_s", strlwr_s_impl);
    vm.register_import(DLL, "_strlwr_s_l", strlwr_s_l_impl);
    vm.register_import(DLL, "_strupr", strupr_impl);
    vm.register_import(DLL, "_strupr_s", strupr_s_impl);
    vm.register_import(DLL, "_strupr_s_l", strupr_s_l_impl);
    vm.register_import(DLL, "_strrev", strrev_impl);
    vm.register_import(DLL, "_strset", strset_impl);
    vm.register_import(DLL, "_strset_s", strset_s_impl);
    vm.register_import(DLL, "_strnset", strnset_impl);
    vm.register_import(DLL, "_strnset_s", strnset_s_impl);
    vm.register_import(DLL, "strxfrm", strxfrm_impl);
    vm.register_import(DLL, "_strxfrm_l", strxfrm_l_impl);
    vm.register_import(DLL, "strerror", strerror_impl);
    vm.register_import(DLL, "strerror_s", strerror_s_impl);
    vm.register_import(DLL, "__strncnt", strlen_impl);

    // Wide string functions
    vm.register_import(DLL, "wcscpy", wcscpy_impl);
    vm.register_import(DLL, "wcscpy_s", wcscpy_s_impl);
    vm.register_import(DLL, "wcsncpy", wcsncpy_impl);
    vm.register_import(DLL, "wcsncpy_s", wcsncpy_s_impl);
    vm.register_import(DLL, "wcscat", wcscat_impl);
    vm.register_import(DLL, "wcscat_s", wcscat_s_impl);
    vm.register_import(DLL, "wcsncat", wcsncat_impl);
    vm.register_import(DLL, "wcsncat_s", wcsncat_s_impl);
    vm.register_import(DLL, "wcscmp", wcscmp_impl);
    vm.register_import(DLL, "wcsncmp", wcsncmp_impl);
    vm.register_import(DLL, "_wcsicmp", wcsicmp_impl);
    vm.register_import(DLL, "_wcsicmp_l", wcsicmp_l_impl);
    vm.register_import(DLL, "_wcsnicmp", wcsnicmp_impl);
    vm.register_import(DLL, "_wcsnicmp_l", wcsnicmp_l_impl);
    vm.register_import(DLL, "wcscoll", wcscoll_impl);
    vm.register_import(DLL, "_wcscoll_l", wcscoll_l_impl);
    vm.register_import(DLL, "_wcsicoll", wcsicoll_impl);
    vm.register_import(DLL, "_wcsicoll_l", wcsicoll_l_impl);
    vm.register_import(DLL, "_wcsncoll", wcsncoll_impl);
    vm.register_import(DLL, "_wcsncoll_l", wcsncoll_l_impl);
    vm.register_import(DLL, "_wcsnicoll", wcsnicoll_impl);
    vm.register_import(DLL, "_wcsnicoll_l", wcsnicoll_l_impl);
    vm.register_import(DLL, "wcslen", wcslen_impl);
    vm.register_import(DLL, "wcsnlen", wcsnlen_impl);
    vm.register_import(DLL, "wcschr", wcschr_impl);
    vm.register_import(DLL, "wcsrchr", wcsrchr_impl);
    vm.register_import(DLL, "wcsstr", wcsstr_impl);
    vm.register_import(DLL, "wcspbrk", wcspbrk_impl);
    vm.register_import(DLL, "wcsspn", wcsspn_impl);
    vm.register_import(DLL, "wcscspn", wcscspn_impl);
    vm.register_import(DLL, "wcstok", wcstok_impl);
    vm.register_import(DLL, "wcstok_s", wcstok_s_impl);
    vm.register_import(DLL, "_wcsdup", wcsdup_impl);
    vm.register_import(DLL, "_wcslwr", wcslwr_impl);
    vm.register_import(DLL, "_wcslwr_s", wcslwr_s_impl);
    vm.register_import(DLL, "_wcslwr_s_l", wcslwr_s_l_impl);
    vm.register_import(DLL, "_wcslwr_l", wcslwr_impl);
    vm.register_import(DLL, "_wcsupr", wcsupr_impl);
    vm.register_import(DLL, "_wcsupr_s", wcsupr_s_impl);
    vm.register_import(DLL, "_wcsupr_s_l", wcsupr_s_l_impl);
    vm.register_import(DLL, "_wcsupr_l", wcsupr_impl);
    vm.register_import(DLL, "_wcsrev", wcsrev_impl);
    vm.register_import(DLL, "_wcsset", wcsset_impl);
    vm.register_import(DLL, "_wcsset_s", wcsset_s_impl);
    vm.register_import(DLL, "_wcsnset", wcsnset_impl);
    vm.register_import(DLL, "_wcsnset_s", wcsnset_s_impl);
    vm.register_import(DLL, "wcsxfrm", wcsxfrm_impl);
    vm.register_import(DLL, "_wcsxfrm_l", wcsxfrm_l_impl);
    vm.register_import(DLL, "__wcserror", wcserror_impl);
    vm.register_import(DLL, "__wcserror_s", wcserror_s_impl);
    vm.register_import(DLL, "_wcserror", wcserror_impl);
    vm.register_import(DLL, "_wcserror_s", wcserror_s_impl);
    vm.register_import(DLL, "__wcsncnt", wcslen_impl);

    // Memory functions
    vm.register_import(DLL, "memcpy", memcpy_impl);
    vm.register_import(DLL, "memcpy_s", memcpy_s_impl);
    vm.register_import(DLL, "memmove", memmove_impl);
    vm.register_import(DLL, "memmove_s", memmove_s_impl);
    vm.register_import(DLL, "memset", memset_impl);
    vm.register_import(DLL, "memcmp", memcmp_impl);
    vm.register_import(DLL, "memchr", memchr_impl);
    vm.register_import(DLL, "_memccpy", memccpy_impl);
    vm.register_import(DLL, "_memicmp", memicmp_impl);
    vm.register_import(DLL, "_memicmp_l", memicmp_l_impl);
    vm.register_import(DLL, "wmemcpy_s", wmemcpy_s_impl);
    vm.register_import(DLL, "wmemmove_s", wmemmove_s_impl);
}
