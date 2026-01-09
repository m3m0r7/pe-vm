//! Minimal MFC40D.dll ordinal stubs.

use crate::vm::Vm;

const DLL_NAME: &str = "MFC40D.DLL";
const ORDINALS: &[u16] = &[
    281, 283, 327, 481, 521, 608, 639, 650, 662, 666, 667, 955, 969, 1002, 1015, 1022, 1025,
    1043, 1044, 1064, 1071, 1146, 1150, 1152, 1154, 1155, 1234, 1681, 1742, 1761, 1884, 1960,
    2158, 2159, 2248, 2249, 2284, 2288, 2358, 2359, 2415, 2692, 2827, 2828, 2829, 2905, 3010,
    3011, 3179, 3281, 3313, 3324, 3404, 3443, 3725, 3797, 3809, 3811, 3821, 3964, 3971, 4044,
    4507, 4531, 4578,
];

pub fn register(vm: &mut Vm) {
    for &ordinal in ORDINALS {
        vm.register_import_ordinal(DLL_NAME, ordinal, mfc_stub);
    }
}

fn mfc_stub(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}
