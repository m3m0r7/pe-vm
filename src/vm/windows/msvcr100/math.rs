//! Math function stubs for MSVCR100.dll.

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

// Math functions
stub!(ci_acos);
stub!(ci_asin);
stub!(ci_atan);
stub!(ci_atan2);
stub!(ci_cos);
stub!(ci_cosh);
stub!(ci_exp);
stub!(ci_fmod);
stub!(ci_log);
stub!(ci_log10);
stub!(ci_pow);
stub!(ci_sin);
stub!(ci_sinh);
stub!(ci_sqrt);
stub!(ci_tan);
stub!(ci_tanh);
stub!(acos_impl);
stub!(asin_impl);
stub!(atan_impl);
stub!(atan2_impl);
stub!(ceil_impl);
stub!(cos_impl);
stub!(cosh_impl);
stub!(exp_impl);
stub!(fabs_impl);
stub!(floor_impl);
stub!(fmod_impl);
stub!(frexp_impl);
stub!(ldexp_impl);
stub!(log_impl);
stub!(log10_impl);
stub!(modf_impl);
stub!(pow_impl);
stub!(sin_impl);
stub!(sinh_impl);
stub!(sqrt_impl);
stub!(tan_impl);
stub!(tanh_impl);
stub!(abs_impl);
stub!(labs_impl);
stub!(llabs_impl);
stub!(div_impl);
stub!(ldiv_impl);
stub!(lldiv_impl);
stub!(abs64);
stub!(hypot);
stub!(cabs);
stub!(chgsign);
stub!(copysign);
stub!(finite);
stub!(fpclass);
stub!(isnan);
stub!(j0);
stub!(j1);
stub!(jn);
stub!(y0);
stub!(y1);
stub!(yn);
stub!(scalb);
stub!(logb);
stub!(nextafter);
stub!(huge);
stub!(fpreset);
stub!(control87);
stub!(control87_2);
stub!(controlfp);
stub!(controlfp_s);
stub!(statusfp);
stub!(statusfp2);
stub!(clearfp);
stub!(fpecode);
stub!(sw_matherr);
stub!(matherr);
stub!(set_sse2_enable);

// SSE2 math intrinsics
stub!(libm_sse2_acos);
stub!(libm_sse2_acosf);
stub!(libm_sse2_asin);
stub!(libm_sse2_asinf);
stub!(libm_sse2_atan);
stub!(libm_sse2_atan2);
stub!(libm_sse2_atanf);
stub!(libm_sse2_cos);
stub!(libm_sse2_cosf);
stub!(libm_sse2_exp);
stub!(libm_sse2_expf);
stub!(libm_sse2_log);
stub!(libm_sse2_log10);
stub!(libm_sse2_log10f);
stub!(libm_sse2_logf);
stub!(libm_sse2_pow);
stub!(libm_sse2_powf);
stub!(libm_sse2_sin);
stub!(libm_sse2_sinf);
stub!(libm_sse2_tan);
stub!(libm_sse2_tanf);

pub fn register(vm: &mut Vm) {
    // Intrinsic math (CI = compiler intrinsic)
    vm.register_import(DLL, "_CIacos", ci_acos);
    vm.register_import(DLL, "_CIasin", ci_asin);
    vm.register_import(DLL, "_CIatan", ci_atan);
    vm.register_import(DLL, "_CIatan2", ci_atan2);
    vm.register_import(DLL, "_CIcos", ci_cos);
    vm.register_import(DLL, "_CIcosh", ci_cosh);
    vm.register_import(DLL, "_CIexp", ci_exp);
    vm.register_import(DLL, "_CIfmod", ci_fmod);
    vm.register_import(DLL, "_CIlog", ci_log);
    vm.register_import(DLL, "_CIlog10", ci_log10);
    vm.register_import(DLL, "_CIpow", ci_pow);
    vm.register_import(DLL, "_CIsin", ci_sin);
    vm.register_import(DLL, "_CIsinh", ci_sinh);
    vm.register_import(DLL, "_CIsqrt", ci_sqrt);
    vm.register_import(DLL, "_CItan", ci_tan);
    vm.register_import(DLL, "_CItanh", ci_tanh);

    // Standard C math functions
    vm.register_import(DLL, "acos", acos_impl);
    vm.register_import(DLL, "asin", asin_impl);
    vm.register_import(DLL, "atan", atan_impl);
    vm.register_import(DLL, "atan2", atan2_impl);
    vm.register_import(DLL, "ceil", ceil_impl);
    vm.register_import(DLL, "cos", cos_impl);
    vm.register_import(DLL, "cosh", cosh_impl);
    vm.register_import(DLL, "exp", exp_impl);
    vm.register_import(DLL, "fabs", fabs_impl);
    vm.register_import(DLL, "floor", floor_impl);
    vm.register_import(DLL, "fmod", fmod_impl);
    vm.register_import(DLL, "frexp", frexp_impl);
    vm.register_import(DLL, "ldexp", ldexp_impl);
    vm.register_import(DLL, "log", log_impl);
    vm.register_import(DLL, "log10", log10_impl);
    vm.register_import(DLL, "modf", modf_impl);
    vm.register_import(DLL, "pow", pow_impl);
    vm.register_import(DLL, "sin", sin_impl);
    vm.register_import(DLL, "sinh", sinh_impl);
    vm.register_import(DLL, "sqrt", sqrt_impl);
    vm.register_import(DLL, "tan", tan_impl);
    vm.register_import(DLL, "tanh", tanh_impl);

    // Integer math
    vm.register_import(DLL, "abs", abs_impl);
    vm.register_import(DLL, "labs", labs_impl);
    vm.register_import(DLL, "llabs", llabs_impl);
    vm.register_import(DLL, "div", div_impl);
    vm.register_import(DLL, "ldiv", ldiv_impl);
    vm.register_import(DLL, "lldiv", lldiv_impl);
    vm.register_import(DLL, "_abs64", abs64);

    // Extended math
    vm.register_import(DLL, "_hypot", hypot);
    vm.register_import(DLL, "_cabs", cabs);
    vm.register_import(DLL, "_chgsign", chgsign);
    vm.register_import(DLL, "_copysign", copysign);
    vm.register_import(DLL, "_finite", finite);
    vm.register_import(DLL, "_fpclass", fpclass);
    vm.register_import(DLL, "_isnan", isnan);
    vm.register_import(DLL, "_j0", j0);
    vm.register_import(DLL, "_j1", j1);
    vm.register_import(DLL, "_jn", jn);
    vm.register_import(DLL, "_y0", y0);
    vm.register_import(DLL, "_y1", y1);
    vm.register_import(DLL, "_yn", yn);
    vm.register_import(DLL, "_scalb", scalb);
    vm.register_import(DLL, "_logb", logb);
    vm.register_import(DLL, "_nextafter", nextafter);

    // Floating point constants and control
    vm.register_import(DLL, "_HUGE", huge);
    vm.register_import(DLL, "_fpreset", fpreset);
    vm.register_import(DLL, "_control87", control87);
    vm.register_import(DLL, "__control87_2", control87_2);
    vm.register_import(DLL, "_controlfp", controlfp);
    vm.register_import(DLL, "_controlfp_s", controlfp_s);
    vm.register_import(DLL, "_statusfp", statusfp);
    vm.register_import(DLL, "_statusfp2", statusfp2);
    vm.register_import(DLL, "_clearfp", clearfp);
    vm.register_import(DLL, "__fpecode", fpecode);
    vm.register_import(DLL, "__sw_matherr", sw_matherr);
    vm.register_import(DLL, "_matherr", matherr);
    vm.register_import(DLL, "__setusermatherr", sw_matherr);
    vm.register_import(DLL, "_set_SSE2_enable", set_sse2_enable);

    // SSE2 math intrinsics
    vm.register_import(DLL, "__libm_sse2_acos", libm_sse2_acos);
    vm.register_import(DLL, "__libm_sse2_acosf", libm_sse2_acosf);
    vm.register_import(DLL, "__libm_sse2_asin", libm_sse2_asin);
    vm.register_import(DLL, "__libm_sse2_asinf", libm_sse2_asinf);
    vm.register_import(DLL, "__libm_sse2_atan", libm_sse2_atan);
    vm.register_import(DLL, "__libm_sse2_atan2", libm_sse2_atan2);
    vm.register_import(DLL, "__libm_sse2_atanf", libm_sse2_atanf);
    vm.register_import(DLL, "__libm_sse2_cos", libm_sse2_cos);
    vm.register_import(DLL, "__libm_sse2_cosf", libm_sse2_cosf);
    vm.register_import(DLL, "__libm_sse2_exp", libm_sse2_exp);
    vm.register_import(DLL, "__libm_sse2_expf", libm_sse2_expf);
    vm.register_import(DLL, "__libm_sse2_log", libm_sse2_log);
    vm.register_import(DLL, "__libm_sse2_log10", libm_sse2_log10);
    vm.register_import(DLL, "__libm_sse2_log10f", libm_sse2_log10f);
    vm.register_import(DLL, "__libm_sse2_logf", libm_sse2_logf);
    vm.register_import(DLL, "__libm_sse2_pow", libm_sse2_pow);
    vm.register_import(DLL, "__libm_sse2_powf", libm_sse2_powf);
    vm.register_import(DLL, "__libm_sse2_sin", libm_sse2_sin);
    vm.register_import(DLL, "__libm_sse2_sinf", libm_sse2_sinf);
    vm.register_import(DLL, "__libm_sse2_tan", libm_sse2_tan);
    vm.register_import(DLL, "__libm_sse2_tanf", libm_sse2_tanf);
}
