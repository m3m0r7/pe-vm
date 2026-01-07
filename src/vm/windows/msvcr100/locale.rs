//! Locale function stubs for MSVCR100.dll.

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

// Locale functions
stub!(setlocale_impl);
stub!(wsetlocale_impl);
stub!(localeconv_impl);
stub!(create_locale_impl);
stub!(free_locale_impl);
stub!(get_current_locale_impl);
stub!(configthreadlocale_impl);
stub!(lconv_init_impl);
stub!(lconv);

// Character classification
stub!(isalpha_impl);
stub!(isalpha_l_impl);
stub!(isalnum_impl);
stub!(isalnum_l_impl);
stub!(isblank_impl);
stub!(isblank_l_impl);
stub!(iscntrl_impl);
stub!(iscntrl_l_impl);
stub!(isdigit_impl);
stub!(isdigit_l_impl);
stub!(isgraph_impl);
stub!(isgraph_l_impl);
stub!(islower_impl);
stub!(islower_l_impl);
stub!(isprint_impl);
stub!(isprint_l_impl);
stub!(ispunct_impl);
stub!(ispunct_l_impl);
stub!(isspace_impl);
stub!(isspace_l_impl);
stub!(isupper_impl);
stub!(isupper_l_impl);
stub!(isxdigit_impl);
stub!(isxdigit_l_impl);
stub!(isascii_impl);
stub!(iscsym_impl);
stub!(iscsymf_impl);
stub!(isleadbyte_impl);
stub!(isleadbyte_l_impl);

// Wide character classification
stub!(iswalpha_impl);
stub!(iswalpha_l_impl);
stub!(iswalnum_impl);
stub!(iswalnum_l_impl);
stub!(iswblank_impl);
stub!(iswblank_l_impl);
stub!(iswcntrl_impl);
stub!(iswcntrl_l_impl);
stub!(iswdigit_impl);
stub!(iswdigit_l_impl);
stub!(iswgraph_impl);
stub!(iswgraph_l_impl);
stub!(iswlower_impl);
stub!(iswlower_l_impl);
stub!(iswprint_impl);
stub!(iswprint_l_impl);
stub!(iswpunct_impl);
stub!(iswpunct_l_impl);
stub!(iswspace_impl);
stub!(iswspace_l_impl);
stub!(iswupper_impl);
stub!(iswupper_l_impl);
stub!(iswxdigit_impl);
stub!(iswxdigit_l_impl);
stub!(iswascii_impl);
stub!(iswctype_impl);
stub!(iswcsym_impl);
stub!(iswcsymf_impl);
stub!(is_wctype_impl);
stub!(wctype_impl);

// Character conversion
stub!(tolower_impl);
stub!(tolower_l_impl);
stub!(toupper_impl);
stub!(toupper_l_impl);
stub!(towlower_impl);
stub!(towlower_l_impl);
stub!(towupper_impl);
stub!(towupper_l_impl);
stub!(toascii_impl);

// Multibyte/wide conversion
stub!(mblen_impl);
stub!(mbrlen_impl);
stub!(mbrtowc_impl);
stub!(mbsrtowcs_impl);
stub!(mbsrtowcs_s_impl);
stub!(mbstowcs_impl);
stub!(mbstowcs_s_impl);
stub!(mbstowcs_l_impl);
stub!(mbstowcs_s_l_impl);
stub!(mbtowc_impl);
stub!(mbtowc_l_impl);
stub!(wcrtomb_impl);
stub!(wcrtomb_s_impl);
stub!(wcsrtombs_impl);
stub!(wcsrtombs_s_impl);
stub!(wcstombs_impl);
stub!(wcstombs_s_impl);
stub!(wcstombs_l_impl);
stub!(wcstombs_s_l_impl);
stub!(wctomb_impl);
stub!(wctomb_s_impl);
stub!(wctomb_l_impl);
stub!(wctomb_s_l_impl);
stub!(wctob_impl);
stub!(btowc_impl);

// MBCS functions
stub!(mbbtombc_impl);
stub!(mbbtombc_l_impl);
stub!(mbbtype_impl);
stub!(mbbtype_l_impl);
stub!(mbcjistojms_impl);
stub!(mbcjistojms_l_impl);
stub!(mbcjmstojis_impl);
stub!(mbcjmstojis_l_impl);
stub!(mbclen_impl);
stub!(mbclen_l_impl);
stub!(mbctombb_impl);
stub!(mbctombb_l_impl);
stub!(mbsbtype_impl);
stub!(mbsbtype_l_impl);
stub!(mbsnbcat_impl);
stub!(mbsnbcat_s_impl);
stub!(mbsnbcat_s_l_impl);
stub!(mbsnbcat_l_impl);
stub!(mbsnbcmp_impl);
stub!(mbsnbcmp_l_impl);
stub!(mbsnbcoll_impl);
stub!(mbsnbcoll_l_impl);
stub!(mbsnbcnt_impl);
stub!(mbsnbcnt_l_impl);
stub!(mbsnbcpy_impl);
stub!(mbsnbcpy_s_impl);
stub!(mbsnbcpy_s_l_impl);
stub!(mbsnbcpy_l_impl);
stub!(mbsnbicmp_impl);
stub!(mbsnbicmp_l_impl);
stub!(mbsnbicoll_impl);
stub!(mbsnbicoll_l_impl);
stub!(mbsnbset_impl);
stub!(mbsnbset_s_impl);
stub!(mbsnbset_s_l_impl);
stub!(mbsnbset_l_impl);
stub!(mbsnccnt_impl);
stub!(mbsnccnt_l_impl);
stub!(mbsnextc_impl);
stub!(mbsnextc_l_impl);

// Locale internal functions
stub!(pctype_func);
stub!(pwctype_func);
stub!(mb_cur_max);
stub!(mb_cur_max_func);
stub!(mb_cur_max_l_func);
stub!(lc_codepage_func);
stub!(lc_collate_cp_func);
stub!(lc_handle_func);
stub!(setlc_active_func);
stub!(unguarded_readlc_active_add_func);
stub!(setlc_active);
stub!(unguarded_readlc_active);
stub!(pctype);
stub!(pwctype);
stub!(mbctype);
stub!(mbcasemap);

// CRT string comparison helpers
stub!(crt_compare_string_a);
stub!(crt_compare_string_w);
stub!(crt_lc_map_string_a);
stub!(crt_lc_map_string_w);

// Locale time/date names
stub!(getdays);
stub!(getmonths);
stub!(gettnames);
stub!(strftime_impl);
stub!(strftime_l_impl);
stub!(wcsftime_impl);
stub!(wcsftime_l_impl);

pub fn register(vm: &mut Vm) {
    // Locale functions
    vm.register_import(DLL, "setlocale", setlocale_impl);
    vm.register_import(DLL, "_wsetlocale", wsetlocale_impl);
    vm.register_import(DLL, "localeconv", localeconv_impl);
    vm.register_import(DLL, "__create_locale", create_locale_impl);
    vm.register_import(DLL, "_create_locale", create_locale_impl);
    vm.register_import(DLL, "__free_locale", free_locale_impl);
    vm.register_import(DLL, "_free_locale", free_locale_impl);
    vm.register_import(DLL, "__get_current_locale", get_current_locale_impl);
    vm.register_import(DLL, "_get_current_locale", get_current_locale_impl);
    vm.register_import(DLL, "_configthreadlocale", configthreadlocale_impl);
    vm.register_import(DLL, "__lconv_init", lconv_init_impl);
    vm.register_import(DLL, "__lconv", lconv);

    // Character classification
    vm.register_import(DLL, "isalpha", isalpha_impl);
    vm.register_import(DLL, "_isalpha_l", isalpha_l_impl);
    vm.register_import(DLL, "isalnum", isalnum_impl);
    vm.register_import(DLL, "_isalnum_l", isalnum_l_impl);
    vm.register_import(DLL, "isblank", isblank_impl);
    vm.register_import(DLL, "_isblank_l", isblank_l_impl);
    vm.register_import(DLL, "iscntrl", iscntrl_impl);
    vm.register_import(DLL, "_iscntrl_l", iscntrl_l_impl);
    vm.register_import(DLL, "isdigit", isdigit_impl);
    vm.register_import(DLL, "_isdigit_l", isdigit_l_impl);
    vm.register_import(DLL, "isgraph", isgraph_impl);
    vm.register_import(DLL, "_isgraph_l", isgraph_l_impl);
    vm.register_import(DLL, "islower", islower_impl);
    vm.register_import(DLL, "_islower_l", islower_l_impl);
    vm.register_import(DLL, "isprint", isprint_impl);
    vm.register_import(DLL, "_isprint_l", isprint_l_impl);
    vm.register_import(DLL, "ispunct", ispunct_impl);
    vm.register_import(DLL, "_ispunct_l", ispunct_l_impl);
    vm.register_import(DLL, "isspace", isspace_impl);
    vm.register_import(DLL, "_isspace_l", isspace_l_impl);
    vm.register_import(DLL, "isupper", isupper_impl);
    vm.register_import(DLL, "_isupper_l", isupper_l_impl);
    vm.register_import(DLL, "isxdigit", isxdigit_impl);
    vm.register_import(DLL, "_isxdigit_l", isxdigit_l_impl);
    vm.register_import(DLL, "__isascii", isascii_impl);
    vm.register_import(DLL, "__iscsym", iscsym_impl);
    vm.register_import(DLL, "__iscsymf", iscsymf_impl);
    vm.register_import(DLL, "isleadbyte", isleadbyte_impl);
    vm.register_import(DLL, "_isleadbyte_l", isleadbyte_l_impl);

    // Wide character classification
    vm.register_import(DLL, "iswalpha", iswalpha_impl);
    vm.register_import(DLL, "_iswalpha_l", iswalpha_l_impl);
    vm.register_import(DLL, "iswalnum", iswalnum_impl);
    vm.register_import(DLL, "_iswalnum_l", iswalnum_l_impl);
    vm.register_import(DLL, "iswblank", iswblank_impl);
    vm.register_import(DLL, "_iswblank_l", iswblank_l_impl);
    vm.register_import(DLL, "iswcntrl", iswcntrl_impl);
    vm.register_import(DLL, "_iswcntrl_l", iswcntrl_l_impl);
    vm.register_import(DLL, "iswdigit", iswdigit_impl);
    vm.register_import(DLL, "_iswdigit_l", iswdigit_l_impl);
    vm.register_import(DLL, "iswgraph", iswgraph_impl);
    vm.register_import(DLL, "_iswgraph_l", iswgraph_l_impl);
    vm.register_import(DLL, "iswlower", iswlower_impl);
    vm.register_import(DLL, "_iswlower_l", iswlower_l_impl);
    vm.register_import(DLL, "iswprint", iswprint_impl);
    vm.register_import(DLL, "_iswprint_l", iswprint_l_impl);
    vm.register_import(DLL, "iswpunct", iswpunct_impl);
    vm.register_import(DLL, "_iswpunct_l", iswpunct_l_impl);
    vm.register_import(DLL, "iswspace", iswspace_impl);
    vm.register_import(DLL, "_iswspace_l", iswspace_l_impl);
    vm.register_import(DLL, "iswupper", iswupper_impl);
    vm.register_import(DLL, "_iswupper_l", iswupper_l_impl);
    vm.register_import(DLL, "iswxdigit", iswxdigit_impl);
    vm.register_import(DLL, "_iswxdigit_l", iswxdigit_l_impl);
    vm.register_import(DLL, "iswascii", iswascii_impl);
    vm.register_import(DLL, "iswctype", iswctype_impl);
    vm.register_import(DLL, "__iswcsym", iswcsym_impl);
    vm.register_import(DLL, "__iswcsymf", iswcsymf_impl);
    vm.register_import(DLL, "is_wctype", is_wctype_impl);
    vm.register_import(DLL, "_wctype", wctype_impl);

    // Character conversion
    vm.register_import(DLL, "tolower", tolower_impl);
    vm.register_import(DLL, "_tolower", tolower_impl);
    vm.register_import(DLL, "_tolower_l", tolower_l_impl);
    vm.register_import(DLL, "toupper", toupper_impl);
    vm.register_import(DLL, "_toupper", toupper_impl);
    vm.register_import(DLL, "_toupper_l", toupper_l_impl);
    vm.register_import(DLL, "towlower", towlower_impl);
    vm.register_import(DLL, "_towlower_l", towlower_l_impl);
    vm.register_import(DLL, "towupper", towupper_impl);
    vm.register_import(DLL, "_towupper_l", towupper_l_impl);
    vm.register_import(DLL, "__toascii", toascii_impl);

    // Multibyte/wide conversion
    vm.register_import(DLL, "mblen", mblen_impl);
    vm.register_import(DLL, "mbrlen", mbrlen_impl);
    vm.register_import(DLL, "mbrtowc", mbrtowc_impl);
    vm.register_import(DLL, "mbsrtowcs", mbsrtowcs_impl);
    vm.register_import(DLL, "mbsrtowcs_s", mbsrtowcs_s_impl);
    vm.register_import(DLL, "mbstowcs", mbstowcs_impl);
    vm.register_import(DLL, "mbstowcs_s", mbstowcs_s_impl);
    vm.register_import(DLL, "_mbstowcs_l", mbstowcs_l_impl);
    vm.register_import(DLL, "_mbstowcs_s_l", mbstowcs_s_l_impl);
    vm.register_import(DLL, "mbtowc", mbtowc_impl);
    vm.register_import(DLL, "_mbtowc_l", mbtowc_l_impl);
    vm.register_import(DLL, "wcrtomb", wcrtomb_impl);
    vm.register_import(DLL, "wcrtomb_s", wcrtomb_s_impl);
    vm.register_import(DLL, "wcsrtombs", wcsrtombs_impl);
    vm.register_import(DLL, "wcsrtombs_s", wcsrtombs_s_impl);
    vm.register_import(DLL, "wcstombs", wcstombs_impl);
    vm.register_import(DLL, "wcstombs_s", wcstombs_s_impl);
    vm.register_import(DLL, "_wcstombs_l", wcstombs_l_impl);
    vm.register_import(DLL, "_wcstombs_s_l", wcstombs_s_l_impl);
    vm.register_import(DLL, "wctomb", wctomb_impl);
    vm.register_import(DLL, "wctomb_s", wctomb_s_impl);
    vm.register_import(DLL, "_wctomb_l", wctomb_l_impl);
    vm.register_import(DLL, "_wctomb_s_l", wctomb_s_l_impl);
    vm.register_import(DLL, "wctob", wctob_impl);
    vm.register_import(DLL, "btowc", btowc_impl);

    // MBCS functions
    vm.register_import(DLL, "_mbbtombc", mbbtombc_impl);
    vm.register_import(DLL, "_mbbtombc_l", mbbtombc_l_impl);
    vm.register_import(DLL, "_mbbtype", mbbtype_impl);
    vm.register_import(DLL, "_mbbtype_l", mbbtype_l_impl);
    vm.register_import(DLL, "_mbcjistojms", mbcjistojms_impl);
    vm.register_import(DLL, "_mbcjistojms_l", mbcjistojms_l_impl);
    vm.register_import(DLL, "_mbcjmstojis", mbcjmstojis_impl);
    vm.register_import(DLL, "_mbcjmstojis_l", mbcjmstojis_l_impl);
    vm.register_import(DLL, "_mbclen", mbclen_impl);
    vm.register_import(DLL, "_mbclen_l", mbclen_l_impl);
    vm.register_import(DLL, "_mbctombb", mbctombb_impl);
    vm.register_import(DLL, "_mbctombb_l", mbctombb_l_impl);
    vm.register_import(DLL, "_mbsbtype", mbsbtype_impl);
    vm.register_import(DLL, "_mbsbtype_l", mbsbtype_l_impl);
    vm.register_import(DLL, "_mbsnbcat", mbsnbcat_impl);
    vm.register_import(DLL, "_mbsnbcat_s", mbsnbcat_s_impl);
    vm.register_import(DLL, "_mbsnbcat_s_l", mbsnbcat_s_l_impl);
    vm.register_import(DLL, "_mbsnbcat_l", mbsnbcat_l_impl);
    vm.register_import(DLL, "_mbsnbcmp", mbsnbcmp_impl);
    vm.register_import(DLL, "_mbsnbcmp_l", mbsnbcmp_l_impl);
    vm.register_import(DLL, "_mbsnbcoll", mbsnbcoll_impl);
    vm.register_import(DLL, "_mbsnbcoll_l", mbsnbcoll_l_impl);
    vm.register_import(DLL, "_mbsnbcnt", mbsnbcnt_impl);
    vm.register_import(DLL, "_mbsnbcnt_l", mbsnbcnt_l_impl);
    vm.register_import(DLL, "_mbsnbcpy", mbsnbcpy_impl);
    vm.register_import(DLL, "_mbsnbcpy_s", mbsnbcpy_s_impl);
    vm.register_import(DLL, "_mbsnbcpy_s_l", mbsnbcpy_s_l_impl);
    vm.register_import(DLL, "_mbsnbcpy_l", mbsnbcpy_l_impl);
    vm.register_import(DLL, "_mbsnbicmp", mbsnbicmp_impl);
    vm.register_import(DLL, "_mbsnbicmp_l", mbsnbicmp_l_impl);
    vm.register_import(DLL, "_mbsnbicoll", mbsnbicoll_impl);
    vm.register_import(DLL, "_mbsnbicoll_l", mbsnbicoll_l_impl);
    vm.register_import(DLL, "_mbsnbset", mbsnbset_impl);
    vm.register_import(DLL, "_mbsnbset_s", mbsnbset_s_impl);
    vm.register_import(DLL, "_mbsnbset_s_l", mbsnbset_s_l_impl);
    vm.register_import(DLL, "_mbsnbset_l", mbsnbset_l_impl);
    vm.register_import(DLL, "_mbsnccnt", mbsnccnt_impl);
    vm.register_import(DLL, "_mbsnccnt_l", mbsnccnt_l_impl);
    vm.register_import(DLL, "_mbsnextc", mbsnextc_impl);
    vm.register_import(DLL, "_mbsnextc_l", mbsnextc_l_impl);

    // Locale internal functions
    vm.register_import(DLL, "__pctype_func", pctype_func);
    vm.register_import(DLL, "__pwctype_func", pwctype_func);
    vm.register_import(DLL, "__mb_cur_max", mb_cur_max);
    vm.register_import(DLL, "___mb_cur_max_func", mb_cur_max_func);
    vm.register_import(DLL, "___mb_cur_max_l_func", mb_cur_max_l_func);
    vm.register_import(DLL, "___lc_codepage_func", lc_codepage_func);
    vm.register_import(DLL, "___lc_collate_cp_func", lc_collate_cp_func);
    vm.register_import(DLL, "___lc_handle_func", lc_handle_func);
    vm.register_import(DLL, "___setlc_active_func", setlc_active_func);
    vm.register_import(DLL, "___unguarded_readlc_active_add_func", unguarded_readlc_active_add_func);
    vm.register_import(DLL, "__setlc_active", setlc_active);
    vm.register_import(DLL, "__unguarded_readlc_active", unguarded_readlc_active);
    vm.register_import(DLL, "__p__pctype", pctype);
    vm.register_import(DLL, "__p__pwctype", pwctype);
    vm.register_import(DLL, "__p__mbctype", mbctype);
    vm.register_import(DLL, "__p__mbcasemap", mbcasemap);

    // CRT string comparison helpers
    vm.register_import(DLL, "__crtCompareStringA", crt_compare_string_a);
    vm.register_import(DLL, "__crtCompareStringW", crt_compare_string_w);
    vm.register_import(DLL, "__crtLCMapStringA", crt_lc_map_string_a);
    vm.register_import(DLL, "__crtLCMapStringW", crt_lc_map_string_w);

    // Locale time/date names
    vm.register_import(DLL, "_Getdays", getdays);
    vm.register_import(DLL, "_Getmonths", getmonths);
    vm.register_import(DLL, "_Gettnames", gettnames);
    vm.register_import(DLL, "strftime", strftime_impl);
    vm.register_import(DLL, "_strftime_l", strftime_l_impl);
    vm.register_import(DLL, "_Strftime", strftime_impl);
    vm.register_import(DLL, "wcsftime", wcsftime_impl);
    vm.register_import(DLL, "_wcsftime_l", wcsftime_l_impl);
}
