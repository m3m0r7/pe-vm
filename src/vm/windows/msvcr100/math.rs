//! Math function stubs for MSVCR100.dll.

use crate::vm::Vm;

const DLL: &str = "MSVCR100.dll";



// Math functions
define_stub_fn!(DLL, ci_acos, 0);
define_stub_fn!(DLL, ci_asin, 0);
define_stub_fn!(DLL, ci_atan, 0);
define_stub_fn!(DLL, ci_atan2, 0);
define_stub_fn!(DLL, ci_cos, 0);
define_stub_fn!(DLL, ci_cosh, 0);
define_stub_fn!(DLL, ci_exp, 0);
define_stub_fn!(DLL, ci_fmod, 0);
define_stub_fn!(DLL, ci_log, 0);
define_stub_fn!(DLL, ci_log10, 0);
define_stub_fn!(DLL, ci_pow, 0);
define_stub_fn!(DLL, ci_sin, 0);
define_stub_fn!(DLL, ci_sinh, 0);
define_stub_fn!(DLL, ci_sqrt, 0);
define_stub_fn!(DLL, ci_tan, 0);
define_stub_fn!(DLL, ci_tanh, 0);
define_stub_fn!(DLL, acos_impl, 0);
define_stub_fn!(DLL, asin_impl, 0);
define_stub_fn!(DLL, atan_impl, 0);
define_stub_fn!(DLL, atan2_impl, 0);
define_stub_fn!(DLL, ceil_impl, 0);
define_stub_fn!(DLL, cos_impl, 0);
define_stub_fn!(DLL, cosh_impl, 0);
define_stub_fn!(DLL, exp_impl, 0);
define_stub_fn!(DLL, fabs_impl, 0);
define_stub_fn!(DLL, floor_impl, 0);
define_stub_fn!(DLL, fmod_impl, 0);
define_stub_fn!(DLL, frexp_impl, 0);
define_stub_fn!(DLL, ldexp_impl, 0);
define_stub_fn!(DLL, log_impl, 0);
define_stub_fn!(DLL, log10_impl, 0);
define_stub_fn!(DLL, modf_impl, 0);
define_stub_fn!(DLL, pow_impl, 0);
define_stub_fn!(DLL, sin_impl, 0);
define_stub_fn!(DLL, sinh_impl, 0);
define_stub_fn!(DLL, sqrt_impl, 0);
define_stub_fn!(DLL, tan_impl, 0);
define_stub_fn!(DLL, tanh_impl, 0);
define_stub_fn!(DLL, abs_impl, 0);
define_stub_fn!(DLL, labs_impl, 0);
define_stub_fn!(DLL, llabs_impl, 0);
define_stub_fn!(DLL, div_impl, 0);
define_stub_fn!(DLL, ldiv_impl, 0);
define_stub_fn!(DLL, lldiv_impl, 0);
define_stub_fn!(DLL, abs64, 0);
define_stub_fn!(DLL, hypot, 0);
define_stub_fn!(DLL, cabs, 0);
define_stub_fn!(DLL, chgsign, 0);
define_stub_fn!(DLL, copysign, 0);
define_stub_fn!(DLL, finite, 0);
define_stub_fn!(DLL, fpclass, 0);
define_stub_fn!(DLL, isnan, 0);
define_stub_fn!(DLL, j0, 0);
define_stub_fn!(DLL, j1, 0);
define_stub_fn!(DLL, jn, 0);
define_stub_fn!(DLL, y0, 0);
define_stub_fn!(DLL, y1, 0);
define_stub_fn!(DLL, yn, 0);
define_stub_fn!(DLL, scalb, 0);
define_stub_fn!(DLL, logb, 0);
define_stub_fn!(DLL, nextafter, 0);
define_stub_fn!(DLL, huge, 0);
define_stub_fn!(DLL, fpreset, 0);
define_stub_fn!(DLL, control87, 0);
define_stub_fn!(DLL, control87_2, 0);
define_stub_fn!(DLL, controlfp, 0);
define_stub_fn!(DLL, controlfp_s, 0);
define_stub_fn!(DLL, statusfp, 0);
define_stub_fn!(DLL, statusfp2, 0);
define_stub_fn!(DLL, clearfp, 0);
define_stub_fn!(DLL, fpecode, 0);
define_stub_fn!(DLL, sw_matherr, 0);
define_stub_fn!(DLL, matherr, 0);
define_stub_fn!(DLL, set_sse2_enable, 0);

// SSE2 math intrinsics
define_stub_fn!(DLL, libm_sse2_acos, 0);
define_stub_fn!(DLL, libm_sse2_acosf, 0);
define_stub_fn!(DLL, libm_sse2_asin, 0);
define_stub_fn!(DLL, libm_sse2_asinf, 0);
define_stub_fn!(DLL, libm_sse2_atan, 0);
define_stub_fn!(DLL, libm_sse2_atan2, 0);
define_stub_fn!(DLL, libm_sse2_atanf, 0);
define_stub_fn!(DLL, libm_sse2_cos, 0);
define_stub_fn!(DLL, libm_sse2_cosf, 0);
define_stub_fn!(DLL, libm_sse2_exp, 0);
define_stub_fn!(DLL, libm_sse2_expf, 0);
define_stub_fn!(DLL, libm_sse2_log, 0);
define_stub_fn!(DLL, libm_sse2_log10, 0);
define_stub_fn!(DLL, libm_sse2_log10f, 0);
define_stub_fn!(DLL, libm_sse2_logf, 0);
define_stub_fn!(DLL, libm_sse2_pow, 0);
define_stub_fn!(DLL, libm_sse2_powf, 0);
define_stub_fn!(DLL, libm_sse2_sin, 0);
define_stub_fn!(DLL, libm_sse2_sinf, 0);
define_stub_fn!(DLL, libm_sse2_tan, 0);
define_stub_fn!(DLL, libm_sse2_tanf, 0);

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
