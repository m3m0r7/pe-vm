// Instruction-level execution tests for the x86 VM.

mod common;
use common::{TestVm, TEST_BASE};

fn step(code: &[u8], setup: impl FnOnce(&mut TestVm)) -> TestVm {
    let mut vm = TestVm::new(code);
    setup(&mut vm);
    vm.step().expect("step");
    vm
}

fn addr(offset: u32) -> u32 {
    TEST_BASE + offset
}

fn modrm_reg_rm(reg: u8, rm: u8) -> u8 {
    0xC0 | ((reg & 7) << 3) | (rm & 7)
}

fn modrm_disp32(reg: u8) -> u8 {
    (reg & 7) << 3 | 0x05
}

// Exercise add/adc opcode variants and flag updates.
#[test]
fn add_and_adc_variants() {
    let mem = addr(0x200);
    let mut code = vec![0x00, modrm_disp32(1)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u8(mem, 2).unwrap();
        vm.set_reg8(1, 5);
    });
    assert_eq!(vm.read_u8(mem).unwrap(), 7);

    let mut code = vec![0x01, modrm_disp32(2)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 3).unwrap();
        vm.set_reg32(2, 4);
    });
    assert_eq!(vm.read_u32(mem).unwrap(), 7);

    let mut code = vec![0x02, modrm_disp32(3)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u8(mem, 2).unwrap();
        vm.set_reg8(3, 1);
    });
    assert_eq!(vm.reg8(3), 3);

    let mut code = vec![0x03, modrm_disp32(0)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 5).unwrap();
        vm.set_reg32(0, 2);
    });
    assert_eq!(vm.reg32(0), 7);

    let code = [0x04, 0x02];
    let vm = step(&code, |vm| vm.set_reg8(0, 1));
    assert_eq!(vm.reg8(0), 3);

    let mut code = vec![0x05];
    code.extend_from_slice(&2u32.to_le_bytes());
    let vm = step(&code, |vm| vm.set_reg32(0, 1));
    assert_eq!(vm.reg32(0), 3);

    let mut code = vec![0x10, modrm_disp32(0)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.set_flags(false, false, false, true);
        vm.write_u8(mem, 1).unwrap();
        vm.set_reg8(0, 1);
    });
    assert_eq!(vm.read_u8(mem).unwrap(), 3);

    let mut code = vec![0x11, modrm_disp32(1)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.set_flags(false, false, false, true);
        vm.write_u32(mem, 1).unwrap();
        vm.set_reg32(1, 1);
    });
    assert_eq!(vm.read_u32(mem).unwrap(), 3);

    let mut code = vec![0x12, modrm_disp32(2)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.set_flags(false, false, false, true);
        vm.write_u8(mem, 2).unwrap();
        vm.set_reg8(2, 1);
    });
    assert_eq!(vm.reg8(2), 4);

    let mut code = vec![0x13, modrm_disp32(3)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.set_flags(false, false, false, true);
        vm.write_u32(mem, 2).unwrap();
        vm.set_reg32(3, 1);
    });
    assert_eq!(vm.reg32(3), 4);

    let code = [0x14, 0x02];
    let vm = step(&code, |vm| {
        vm.set_flags(false, false, false, true);
        vm.set_reg8(0, 1);
    });
    assert_eq!(vm.reg8(0), 4);

    let mut code = vec![0x15];
    code.extend_from_slice(&2u32.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.set_flags(false, false, false, true);
        vm.set_reg32(0, 1);
    });
    assert_eq!(vm.reg32(0), 4);

    let code = [0x40];
    let vm = step(&code, |vm| vm.set_reg32(0, 1));
    assert_eq!(vm.reg32(0), 2);
}

// Exercise sub/sbb/cmp/neg/dec opcode variants and flags.
#[test]
fn sub_sbb_cmp_neg_and_dec() {
    let mem = addr(0x220);
    let mut code = vec![0x28, modrm_disp32(1)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u8(mem, 5).unwrap();
        vm.set_reg8(1, 2);
    });
    assert_eq!(vm.read_u8(mem).unwrap(), 3);

    let mut code = vec![0x29, modrm_disp32(2)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 9).unwrap();
        vm.set_reg32(2, 4);
    });
    assert_eq!(vm.read_u32(mem).unwrap(), 5);

    let mut code = vec![0x2A, modrm_disp32(3)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u8(mem, 2).unwrap();
        vm.set_reg8(3, 5);
    });
    assert_eq!(vm.reg8(3), 3);

    let mut code = vec![0x2B, modrm_disp32(0)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 2).unwrap();
        vm.set_reg32(0, 5);
    });
    assert_eq!(vm.reg32(0), 3);

    let code = [0x2C, 0x02];
    let vm = step(&code, |vm| vm.set_reg8(0, 5));
    assert_eq!(vm.reg8(0), 3);

    let mut code = vec![0x2D];
    code.extend_from_slice(&2u32.to_le_bytes());
    let vm = step(&code, |vm| vm.set_reg32(0, 5));
    assert_eq!(vm.reg32(0), 3);

    let code = [0x48];
    let vm = step(&code, |vm| vm.set_reg32(0, 2));
    assert_eq!(vm.reg32(0), 1);

    let mut code = vec![0x38, modrm_disp32(1)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u8(mem, 1).unwrap();
        vm.set_reg8(1, 1);
    });
    assert!(vm.zf());

    let mut code = vec![0x3B, modrm_disp32(2)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 4).unwrap();
        vm.set_reg32(2, 4);
    });
    assert!(vm.zf());

    let code = [0x3C, 0x02];
    let vm = step(&code, |vm| vm.set_reg8(0, 2));
    assert!(vm.zf());

    let mut code = vec![0x3D];
    code.extend_from_slice(&2u32.to_le_bytes());
    let vm = step(&code, |vm| vm.set_reg32(0, 2));
    assert!(vm.zf());

    let mut code = vec![0x18, modrm_disp32(0)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.set_flags(false, false, false, true);
        vm.write_u8(mem, 5).unwrap();
        vm.set_reg8(0, 1);
    });
    assert_eq!(vm.read_u8(mem).unwrap(), 3);

    let mut code = vec![0x1B, modrm_disp32(1)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.set_flags(false, false, false, true);
        vm.write_u32(mem, 3).unwrap();
        vm.set_reg32(1, 1);
    });
    assert_eq!(vm.reg32(1), 0xFFFF_FFFD);

    let mut code = vec![0xF6, modrm_disp32(3)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u8(mem, 2).unwrap();
    });
    assert_eq!(vm.read_u8(mem).unwrap(), 0xFE);

    let mut code = vec![0xF7, modrm_disp32(3)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 2).unwrap();
    });
    assert_eq!(vm.read_u32(mem).unwrap(), 0xFFFF_FFFE);

    let mut code = vec![0xFE, modrm_disp32(0)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u8(mem, 1).unwrap();
    });
    assert_eq!(vm.read_u8(mem).unwrap(), 2);

    let mut code = vec![0xFE, modrm_disp32(1)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u8(mem, 2).unwrap();
    });
    assert_eq!(vm.read_u8(mem).unwrap(), 1);

    let mut code = vec![0xFF, modrm_disp32(1)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 2).unwrap();
    });
    assert_eq!(vm.read_u32(mem).unwrap(), 1);
}

// Cover logic/test opcodes across reg/mem forms.
#[test]
fn logic_and_test_variants() {
    let mem = addr(0x240);
    let mut code = vec![0x08, modrm_disp32(0)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u8(mem, 0x0F).unwrap();
        vm.set_reg8(0, 0xF0);
    });
    assert_eq!(vm.read_u8(mem).unwrap(), 0xFF);

    let mut code = vec![0x21, modrm_disp32(1)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 0xFF00).unwrap();
        vm.set_reg32(1, 0x0F0F);
    });
    assert_eq!(vm.read_u32(mem).unwrap(), 0x0F00);

    let mut code = vec![0x30, modrm_disp32(2)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u8(mem, 0x0F).unwrap();
        vm.set_reg8(2, 0xF0);
    });
    assert_eq!(vm.read_u8(mem).unwrap(), 0xFF);

    let code = vec![0x0C, 0x0F];
    let vm = step(&code, |vm| vm.set_reg8(0, 0xF0));
    assert_eq!(vm.reg8(0), 0xFF);

    let code = vec![0x24, 0x0F];
    let vm = step(&code, |vm| vm.set_reg8(0, 0xF0));
    assert_eq!(vm.reg8(0), 0x00);

    let code = vec![0x34, 0x0F];
    let vm = step(&code, |vm| vm.set_reg8(0, 0xF0));
    assert_eq!(vm.reg8(0), 0xFF);

    let code = [0xA8, 0x00];
    let vm = step(&code, |vm| vm.set_reg8(0, 0));
    assert!(vm.zf());

    let code = vec![0x84, modrm_reg_rm(1, 0)];
    let vm = step(&code, |vm| {
        vm.set_reg8(0, 0);
        vm.set_reg8(1, 0);
    });
    assert!(vm.zf());

    let mut code = vec![0xA9];
    code.extend_from_slice(&0u32.to_le_bytes());
    let vm = step(&code, |vm| vm.set_reg32(0, 0));
    assert!(vm.zf());

    let mut code = vec![0xF6, modrm_disp32(2)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u8(mem, 0x0F).unwrap();
    });
    assert_eq!(vm.read_u8(mem).unwrap(), 0xF0);

    let mut code = vec![0xF7, modrm_disp32(2)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 0x0000_FFFF).unwrap();
    });
    assert_eq!(vm.read_u32(mem).unwrap(), 0xFFFF_0000);
}

// Validate operand-size override for TEST with high bits set.
#[test]
fn test_operand_size_override_flags() {
    let code = [0x66, 0x85, 0xC0];
    let vm = step(&code, |vm| vm.set_reg32(0, 0x1234_0000));
    assert!(vm.zf());
}

// Cover operand-size override for OR/AND/XOR instruction forms.
#[test]
fn logic_operand_size_override_16() {
    let mem = addr(0x300);

    let mut code = vec![0x66, 0x09, modrm_disp32(1)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 0xABCD_00F0).unwrap();
        vm.set_reg16(1, 0x0F00);
    });
    assert_eq!(vm.read_u32(mem).unwrap(), 0xABCD_0FF0);

    let mut code = vec![0x66, 0x23, modrm_disp32(2)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 0x1234_F0F0).unwrap();
        vm.set_reg16(2, 0x0FF0);
    });
    assert_eq!(vm.reg16(2), 0x00F0);

    let mut code = vec![0x66, 0x83, modrm_disp32(6)];
    code.extend_from_slice(&mem.to_le_bytes());
    code.push(0x0F);
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 0xAAAA_00F0).unwrap();
    });
    assert_eq!(vm.read_u32(mem).unwrap(), 0xAAAA_00FF);

    let mut code = vec![0x66, 0x0D];
    code.extend_from_slice(&0x00F0u16.to_le_bytes());
    let vm = step(&code, |vm| vm.set_reg16(0, 0x0F00));
    assert_eq!(vm.reg16(0), 0x0FF0);
}

// Exercise F7 mul/imul/div/idiv variants.
#[test]
fn f7_mul_imul_div_idiv() {
    let mem = addr(0x258);

    let mut code = vec![0xF7, modrm_disp32(4)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 4).unwrap();
        vm.set_reg32(0, 3);
    });
    assert_eq!(vm.reg32(0), 12);
    assert_eq!(vm.reg32(2), 0);

    let mut code = vec![0xF7, modrm_disp32(5)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 3).unwrap();
        vm.set_reg32(0, 0xFFFF_FFFE);
    });
    assert_eq!(vm.reg32(0), 0xFFFF_FFFA);
    assert_eq!(vm.reg32(2), 0xFFFF_FFFF);

    let mut code = vec![0xF7, modrm_disp32(6)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 3).unwrap();
        vm.set_reg32(0, 20);
        vm.set_reg32(2, 0);
    });
    assert_eq!(vm.reg32(0), 6);
    assert_eq!(vm.reg32(2), 2);

    let mut code = vec![0xF7, modrm_disp32(7)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 3).unwrap();
        vm.set_reg32(0, 0xFFFF_FFEC);
        vm.set_reg32(2, 0xFFFF_FFFF);
    });
    assert_eq!(vm.reg32(0), 0xFFFF_FFFA);
    assert_eq!(vm.reg32(2), 0xFFFF_FFFE);
}

// Exercise 0F AF signed multiply register form.
#[test]
fn imul_r32_rm32_variants() {
    let mem = addr(0x270);

    let mut code = vec![0x0F, 0xAF, modrm_disp32(1)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 4).unwrap();
        vm.set_reg32(1, 3);
    });
    assert_eq!(vm.reg32(1), 12);

    let mut code = vec![0x0F, 0xAF, modrm_disp32(2)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 3).unwrap();
        vm.set_reg32(2, 0xFFFF_FFFE);
    });
    assert_eq!(vm.reg32(2), 0xFFFF_FFFA);
}

// Cover mov and lea register/memory variants.
#[test]
fn mov_and_lea_variants() {
    let mem = addr(0x260);
    let mut code = vec![0x88, modrm_disp32(1)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.set_reg8(1, 0xAA);
    });
    assert_eq!(vm.read_u8(mem).unwrap(), 0xAA);

    let mut code = vec![0x8B, modrm_disp32(2)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 0x1234_5678).unwrap();
    });
    assert_eq!(vm.reg32(2), 0x1234_5678);

    let code = [0xB0, 0x7F];
    let vm = step(&code, |_| {});
    assert_eq!(vm.reg8(0), 0x7F);

    let mut code = vec![0xB8];
    code.extend_from_slice(&0xDEAD_BEEFu32.to_le_bytes());
    let vm = step(&code, |_| {});
    assert_eq!(vm.reg32(0), 0xDEAD_BEEF);

    let code = [0x66, 0xB8, 0x34, 0x12];
    let vm = step(&code, |_| {});
    assert_eq!(vm.reg32(0), 0x1234);

    let mut code = vec![0xC6, modrm_disp32(0)];
    code.extend_from_slice(&mem.to_le_bytes());
    code.push(0x11);
    let vm = step(&code, |_| {});
    assert_eq!(vm.read_u8(mem).unwrap(), 0x11);

    let mut code = vec![0xC7, modrm_disp32(0)];
    code.extend_from_slice(&mem.to_le_bytes());
    code.extend_from_slice(&0x1122_3344u32.to_le_bytes());
    let vm = step(&code, |_| {});
    assert_eq!(vm.read_u32(mem).unwrap(), 0x1122_3344);

    let mut code = vec![0x66, 0xC7, modrm_disp32(0)];
    code.extend_from_slice(&mem.to_le_bytes());
    code.extend_from_slice(&0x1234u16.to_le_bytes());
    let vm = step(&code, |_| {});
    assert_eq!(vm.read_u32(mem).unwrap() & 0xFFFF, 0x1234);

    let mut code = vec![0xA1];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 0x5566_7788).unwrap();
    });
    assert_eq!(vm.reg32(0), 0x5566_7788);

    let mut code = vec![0xA3];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.set_reg32(0, 0x99AA_BBCC);
    });
    assert_eq!(vm.read_u32(mem).unwrap(), 0x99AA_BBCC);

    let mut code = vec![0x8D, modrm_disp32(3)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |_| {});
    assert_eq!(vm.reg32(3), mem);

    let mut code = vec![0x66, 0x8C, modrm_disp32(2)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 0xFFFF_FFFF).unwrap();
    });
    assert_eq!(vm.read_u32(mem).unwrap() & 0xFFFF, 0);

    let mut code = vec![0x0F, 0xB6, modrm_disp32(1)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u8(mem, 0xF0).unwrap();
    });
    assert_eq!(vm.reg32(1), 0xF0);

    let mut code = vec![0x0F, 0xB7, modrm_disp32(1)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 0x0000_F0F1).unwrap();
    });
    assert_eq!(vm.reg32(1), 0xF0F1);

    let src = addr(0x300);
    let dst = addr(0x308);
    let code = [0xA5];
    let vm = step(&code, |vm| {
        vm.set_reg32(6, src);
        vm.set_reg32(7, dst);
        vm.write_u32(src, 0xCAFEBABE).unwrap();
    });
    assert_eq!(vm.read_u32(dst).unwrap(), 0xCAFEBABE);
    assert_eq!(vm.reg32(6), src + 4);
    assert_eq!(vm.reg32(7), dst + 4);

    let dst = addr(0x310);
    let code = [0xAB];
    let vm = step(&code, |vm| {
        vm.set_reg32(0, 0xAABB_CCDD);
        vm.set_reg32(7, dst);
    });
    assert_eq!(vm.read_u32(dst).unwrap(), 0xAABB_CCDD);
    assert_eq!(vm.reg32(7), dst + 4);

    let dst = addr(0x320);
    let code = [0xF3, 0xAB];
    let vm = step(&code, |vm| {
        vm.set_reg32(0, 0x0102_0304);
        vm.set_reg32(1, 2);
        vm.set_reg32(7, dst);
    });
    assert_eq!(vm.read_u32(dst).unwrap(), 0x0102_0304);
    assert_eq!(vm.read_u32(dst + 4).unwrap(), 0x0102_0304);
    assert_eq!(vm.reg32(1), 0);
    assert_eq!(vm.reg32(7), dst + 8);
}

// Cover push/pop/leave and stack effects.
#[test]
fn stack_variants() {
    let code = [0x50];
    let vm = step(&code, |vm| vm.set_reg32(0, 0x1234_5678));
    let esp = vm.reg32(4);
    assert_eq!(vm.read_u32(esp).unwrap(), 0x1234_5678);

    let code = [0x58];
    let vm = step(&code, |vm| {
        let esp = vm.reg32(4).wrapping_sub(4);
        vm.set_reg32(4, esp);
        vm.write_u32(esp, 0xDEAD_BEEF).unwrap();
    });
    assert_eq!(vm.reg32(0), 0xDEAD_BEEF);

    let code = [0x6A, 0x7F];
    let vm = step(&code, |_| {});
    let esp = vm.reg32(4);
    assert_eq!(vm.read_u32(esp).unwrap(), 0x7F);

    let mut code = vec![0x68];
    code.extend_from_slice(&0x1122_3344u32.to_le_bytes());
    let vm = step(&code, |_| {});
    let esp = vm.reg32(4);
    assert_eq!(vm.read_u32(esp).unwrap(), 0x1122_3344);

    let mem = addr(0x320);
    let mut code = vec![0x8F, modrm_disp32(0)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        let esp = vm.reg32(4).wrapping_sub(4);
        vm.set_reg32(4, esp);
        vm.write_u32(esp, 0x5566_7788).unwrap();
    });
    assert_eq!(vm.read_u32(mem).unwrap(), 0x5566_7788);

    let mut code = vec![0xFF, modrm_disp32(6)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 0xFACE_CAFE).unwrap();
    });
    let esp = vm.reg32(4);
    assert_eq!(vm.read_u32(esp).unwrap(), 0xFACE_CAFE);

    let code = [0x9C];
    let vm = step(&code, |vm| vm.set_flags(true, true, true, true));
    let esp = vm.reg32(4);
    let flags = vm.read_u32(esp).unwrap();
    assert!(flags & 1 != 0);

    let code = [0xC9];
    let vm = step(&code, |vm| {
        vm.set_reg32(5, vm.reg32(4).wrapping_sub(4));
        vm.write_u32(vm.reg32(5), 0xAABB_CCDD).unwrap();
    });
    assert_eq!(vm.reg32(5), 0xAABB_CCDD);
}

// Cover call/jmp/ret/conditional flows.
#[test]
fn control_flow_variants() {
    let code = [0x74, 0x02];
    let vm = step(&code, |vm| vm.set_flags(true, false, false, false));
    assert_eq!(vm.eip(), TEST_BASE + 4);

    let code = [0x0F, 0x85, 0x02, 0x00, 0x00, 0x00];
    let vm = step(&code, |vm| vm.set_flags(false, false, false, false));
    assert_eq!(vm.eip(), TEST_BASE + 8);

    let mut code = vec![0xE8];
    let next = TEST_BASE + 5;
    let target = TEST_BASE + 0x20;
    let rel = (target as i32 - next as i32) as u32;
    code.extend_from_slice(&rel.to_le_bytes());
    let vm = step(&code, |_| {});
    assert_eq!(vm.eip(), target);
    assert_eq!(vm.read_u32(vm.reg32(4)).unwrap(), next);

    let mut code = vec![0xE9];
    let next = TEST_BASE + 5;
    let target = TEST_BASE + 0x30;
    let rel = (target as i32 - next as i32) as u32;
    code.extend_from_slice(&rel.to_le_bytes());
    let vm = step(&code, |_| {});
    assert_eq!(vm.eip(), target);

    let code = [0xEB, 0x02];
    let vm = step(&code, |_| {});
    assert_eq!(vm.eip(), TEST_BASE + 4);

    let code = vec![0xFF, modrm_reg_rm(2, 0)];
    let target = TEST_BASE + 0x40;
    let vm = step(&code, |vm| vm.set_reg32(0, target));
    assert_eq!(vm.eip(), target);

    let code = vec![0xFF, modrm_reg_rm(4, 1)];
    let target = TEST_BASE + 0x44;
    let vm = step(&code, |vm| vm.set_reg32(1, target));
    assert_eq!(vm.eip(), target);

    let code = [0xC3];
    let vm = step(&code, |vm| {
        let esp = vm.reg32(4).wrapping_sub(4);
        vm.set_reg32(4, esp);
        vm.write_u32(esp, TEST_BASE + 0x88).unwrap();
    });
    assert_eq!(vm.eip(), TEST_BASE + 0x88);

    let code = [0xC2, 0x08, 0x00];
    let vm = step(&code, |vm| {
        let esp = vm.reg32(4).wrapping_sub(4);
        vm.set_reg32(4, esp);
        vm.write_u32(esp, TEST_BASE + 0x90).unwrap();
    });
    assert_eq!(vm.eip(), TEST_BASE + 0x90);
    assert_eq!(vm.reg32(4), TEST_BASE + 0x10000 - 4 + 8);

    let mut code = vec![0x0F, 0x94, modrm_disp32(0)];
    let mem = addr(0x340);
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| vm.set_flags(true, false, false, false));
    assert_eq!(vm.read_u8(mem).unwrap(), 1);

    let mut code = vec![0x0F, 0x95, modrm_disp32(0)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| vm.set_flags(true, false, false, false));
    assert_eq!(vm.read_u8(mem).unwrap(), 0);
}

// Cover cmovcc opcode variants.
#[test]
fn cmovcc_variants() {
    let code = vec![0x0F, 0x4F, modrm_reg_rm(1, 0)];
    let vm = step(&code, |vm| {
        vm.set_flags(false, false, false, false);
        vm.set_reg32(0, 0x1234_5678);
        vm.set_reg32(1, 0);
    });
    assert_eq!(vm.reg32(1), 0x1234_5678);

    let code = vec![0x0F, 0x4F, modrm_reg_rm(1, 0)];
    let vm = step(&code, |vm| {
        vm.set_flags(true, false, false, false);
        vm.set_reg32(0, 0x8765_4321);
        vm.set_reg32(1, 0);
    });
    assert_eq!(vm.reg32(1), 0);
}

// Cover shift/rotate opcode variants and counts.
#[test]
fn shift_and_rotate_variants() {
    let mem = addr(0x360);
    let mut code = vec![0xC1, modrm_disp32(0)];
    code.extend_from_slice(&mem.to_le_bytes());
    code.push(1);
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 0x8000_0001).unwrap();
    });
    assert_eq!(vm.read_u32(mem).unwrap(), 0x0000_0003);

    let mut code = vec![0xC1, modrm_disp32(1)];
    code.extend_from_slice(&mem.to_le_bytes());
    code.push(1);
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 0x8000_0001).unwrap();
    });
    assert_eq!(vm.read_u32(mem).unwrap(), 0xC000_0000);

    let mut code = vec![0xC1, modrm_disp32(2)];
    code.extend_from_slice(&mem.to_le_bytes());
    code.push(1);
    let vm = step(&code, |vm| {
        vm.set_flags(false, false, false, true);
        vm.write_u32(mem, 0).unwrap();
    });
    assert_eq!(vm.read_u32(mem).unwrap(), 1);

    let mut code = vec![0xC1, modrm_disp32(4)];
    code.extend_from_slice(&mem.to_le_bytes());
    code.push(1);
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 0x0000_0001).unwrap();
    });
    assert_eq!(vm.read_u32(mem).unwrap(), 0x0000_0002);

    let mut code = vec![0xD1, modrm_disp32(4)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 0x0000_0001).unwrap();
    });
    assert_eq!(vm.read_u32(mem).unwrap(), 0x0000_0002);

    let mut code = vec![0xC1, modrm_disp32(5)];
    code.extend_from_slice(&mem.to_le_bytes());
    code.push(1);
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 0x8000_0000).unwrap();
    });
    assert_eq!(vm.read_u32(mem).unwrap(), 0x4000_0000);

    let mut code = vec![0xD3, modrm_disp32(7)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 0x8000_0000).unwrap();
        vm.set_reg8(1, 1);
    });
    assert_eq!(vm.read_u32(mem).unwrap(), 0xC000_0000);

    // Cover 8-bit shift/rotate opcode variants.
    let mem8 = addr(0x364);
    let mut code = vec![0xC0, modrm_disp32(0)];
    code.extend_from_slice(&mem8.to_le_bytes());
    code.push(1);
    let vm = step(&code, |vm| {
        vm.write_u8(mem8, 0x81).unwrap();
    });
    assert_eq!(vm.read_u8(mem8).unwrap(), 0x03);

    let mut code = vec![0xC0, modrm_disp32(4)];
    code.extend_from_slice(&mem8.to_le_bytes());
    code.push(1);
    let vm = step(&code, |vm| {
        vm.write_u8(mem8, 0x01).unwrap();
    });
    assert_eq!(vm.read_u8(mem8).unwrap(), 0x02);

    let mut code = vec![0xD0, modrm_disp32(5)];
    code.extend_from_slice(&mem8.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u8(mem8, 0x80).unwrap();
    });
    assert_eq!(vm.read_u8(mem8).unwrap(), 0x40);

    let mut code = vec![0xD2, modrm_disp32(7)];
    code.extend_from_slice(&mem8.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u8(mem8, 0x80).unwrap();
        vm.set_reg8(1, 1);
    });
    assert_eq!(vm.read_u8(mem8).unwrap(), 0xC0);
}

// Cover atomic/system opcodes and side effects.
#[test]
fn atomic_and_system_variants() {
    let code = vec![0x87, modrm_reg_rm(1, 0)];
    let vm = step(&code, |vm| {
        vm.set_reg32(0, 1);
        vm.set_reg32(1, 2);
    });
    assert_eq!(vm.reg32(0), 2);
    assert_eq!(vm.reg32(1), 1);

    let code = [0x91];
    let vm = step(&code, |vm| {
        vm.set_reg32(0, 1);
        vm.set_reg32(1, 2);
    });
    assert_eq!(vm.reg32(0), 2);
    assert_eq!(vm.reg32(1), 1);

    let mem = addr(0x380);
    let mut code = vec![0x0F, 0xB1, modrm_disp32(1)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 1).unwrap();
        vm.set_reg32(0, 1);
        vm.set_reg32(1, 9);
    });
    assert_eq!(vm.read_u32(mem).unwrap(), 9);

    let mut code = vec![0x0F, 0xC1, modrm_disp32(1)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 5).unwrap();
        vm.set_reg32(1, 2);
    });
    assert_eq!(vm.read_u32(mem).unwrap(), 7);
    assert_eq!(vm.reg32(1), 5);

    let code = [0x90];
    let vm = step(&code, |_| {});
    assert_eq!(vm.eip(), TEST_BASE + 1);

    let code = [0xCC];
    let vm = step(&code, |_| {});
    assert_eq!(vm.eip(), TEST_BASE + 1);

    let code = [0xCD, 0x80];
    let vm = step(&code, |_| {});
    assert_eq!(vm.eip(), TEST_BASE + 2);

    let code = [0x0F, 0xA2];
    let vm = step(&code, |vm| vm.set_reg32(0, 0));
    assert_eq!(vm.reg32(0), 1);

    let code = [0x0F, 0x01, 0xD0];
    let vm = step(&code, |_| {});
    assert_eq!(vm.reg32(0), 0);
    assert_eq!(vm.reg32(2), 0);

    let code = [0xD6];
    let vm = step(&code, |vm| vm.set_flags(false, false, false, true));
    assert_eq!(vm.reg8(0), 0xFF);

    let code = [0xD6];
    let vm = step(&code, |vm| vm.set_flags(false, false, false, false));
    assert_eq!(vm.reg8(0), 0x00);

    let code = [0x99];
    let vm = step(&code, |vm| vm.set_reg32(0, 0x8000_0000));
    assert_eq!(vm.reg32(2), 0xFFFF_FFFF);
}

// Cover FPU escape opcodes without side effects.
#[test]
fn fpu_escape_variants() {
    let code = [0xDB, 0xC0];
    let vm = step(&code, |_| {});
    assert_eq!(vm.eip(), TEST_BASE + 2);
}

// Cover SSE opcodes with ModRM decoding.
#[test]
fn sse_stub_variants() {
    let code = [0x0F, 0x57, 0xC0];
    let vm = step(&code, |_| {});
    assert_eq!(vm.eip(), TEST_BASE + 3);
}

// Cover bit test and modify opcode variants.
#[test]
fn bit_test_and_modify_variants() {
    let code = vec![0x0F, 0xBA, modrm_reg_rm(4, 0), 0x01];
    let vm = step(&code, |vm| vm.set_reg32(0, 0x2));
    assert!(vm.cf());
    assert_eq!(vm.reg32(0), 0x2);

    let code = vec![0x0F, 0xBA, modrm_reg_rm(5, 0), 0x03];
    let vm = step(&code, |vm| vm.set_reg32(0, 0));
    assert!(!vm.cf());
    assert_eq!(vm.reg32(0), 0x8);
}

// Cover imul and group1 arithmetic encodings.
#[test]
fn imul_and_group1_variants() {
    let mem = addr(0x3A0);
    let code = vec![0x6B, modrm_reg_rm(0, 0), 0x04];
    let vm = step(&code, |vm| {
        vm.set_reg32(0, 2);
    });
    assert_eq!(vm.reg32(0), 8);

    let mut code = vec![0x69, modrm_reg_rm(1, 0)];
    code.extend_from_slice(&4u32.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.set_reg32(0, 3);
    });
    assert_eq!(vm.reg32(1), 12);

    let mut code = vec![0x80, modrm_disp32(0)];
    code.extend_from_slice(&mem.to_le_bytes());
    code.push(1);
    let vm = step(&code, |vm| {
        vm.write_u8(mem, 1).unwrap();
    });
    assert_eq!(vm.read_u8(mem).unwrap(), 2);

    let mut code = vec![0x80, modrm_disp32(1)];
    code.extend_from_slice(&mem.to_le_bytes());
    code.push(1);
    let vm = step(&code, |vm| {
        vm.write_u8(mem, 1).unwrap();
    });
    assert_eq!(vm.read_u8(mem).unwrap(), 1 | 1);

    let mut code = vec![0x80, modrm_disp32(2)];
    code.extend_from_slice(&mem.to_le_bytes());
    code.push(1);
    let vm = step(&code, |vm| {
        vm.write_u8(mem, 1).unwrap();
        vm.set_flags(false, false, false, true);
    });
    assert_eq!(vm.read_u8(mem).unwrap(), 3);

    let mut code = vec![0x80, modrm_disp32(3)];
    code.extend_from_slice(&mem.to_le_bytes());
    code.push(1);
    let vm = step(&code, |vm| {
        vm.write_u8(mem, 2).unwrap();
        vm.set_flags(false, false, false, true);
    });
    assert_eq!(vm.read_u8(mem).unwrap(), 0);

    let mut code = vec![0x80, modrm_disp32(4)];
    code.extend_from_slice(&mem.to_le_bytes());
    code.push(1);
    let vm = step(&code, |vm| {
        vm.write_u8(mem, 3).unwrap();
    });
    assert_eq!(vm.read_u8(mem).unwrap(), 1);

    let mut code = vec![0x80, modrm_disp32(5)];
    code.extend_from_slice(&mem.to_le_bytes());
    code.push(1);
    let vm = step(&code, |vm| {
        vm.write_u8(mem, 3).unwrap();
    });
    assert_eq!(vm.read_u8(mem).unwrap(), 2);

    let mut code = vec![0x80, modrm_disp32(6)];
    code.extend_from_slice(&mem.to_le_bytes());
    code.push(1);
    let vm = step(&code, |vm| {
        vm.write_u8(mem, 3).unwrap();
    });
    assert_eq!(vm.read_u8(mem).unwrap(), 2);

    let mut code = vec![0x80, modrm_disp32(7)];
    code.extend_from_slice(&mem.to_le_bytes());
    code.push(3);
    let vm = step(&code, |vm| {
        vm.write_u8(mem, 3).unwrap();
    });
    assert!(vm.zf());

    let mut code = vec![0x81, modrm_disp32(0)];
    code.extend_from_slice(&mem.to_le_bytes());
    code.extend_from_slice(&1u32.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 1).unwrap();
    });
    assert_eq!(vm.read_u32(mem).unwrap(), 2);
}

// Cover additional logic opcodes and immediates.
#[test]
fn logic_more_variants() {
    let mem = addr(0x248);
    let mut code = vec![0x09, modrm_disp32(0)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 0x0F0F).unwrap();
        vm.set_reg32(0, 0xF0F0);
    });
    assert_eq!(vm.read_u32(mem).unwrap(), 0xFFFF);

    let code = vec![0x0A, modrm_reg_rm(1, 0)];
    let vm = step(&code, |vm| {
        vm.set_reg8(0, 0x0F);
        vm.set_reg8(1, 0xF0);
    });
    assert_eq!(vm.reg8(1), 0xFF);

    let code = vec![0x0B, modrm_reg_rm(2, 0)];
    let vm = step(&code, |vm| {
        vm.set_reg32(0, 0x0F0F);
        vm.set_reg32(2, 0xF0F0);
    });
    assert_eq!(vm.reg32(2), 0xFFFF);

    let mut code = vec![0x20, modrm_disp32(3)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u8(mem, 0xF0).unwrap();
        vm.set_reg8(3, 0x0F);
    });
    assert_eq!(vm.read_u8(mem).unwrap(), 0x00);

    let code = vec![0x22, modrm_reg_rm(4, 0)];
    let vm = step(&code, |vm| {
        vm.set_reg8(0, 0xF0);
        vm.set_reg8(4, 0x0F);
    });
    assert_eq!(vm.reg8(4), 0x00);

    let code = vec![0x23, modrm_reg_rm(5, 0)];
    let vm = step(&code, |vm| {
        vm.set_reg32(0, 0x0F0F);
        vm.set_reg32(5, 0xF0F0);
    });
    assert_eq!(vm.reg32(5), 0x0000);

    let code = vec![0x32, modrm_reg_rm(6, 0)];
    let vm = step(&code, |vm| {
        vm.set_reg8(0, 0xFF);
        vm.set_reg8(6, 0x0F);
    });
    assert_eq!(vm.reg8(6), 0xF0);

    let code = vec![0x33, modrm_reg_rm(6, 0)];
    let vm = step(&code, |vm| {
        vm.set_reg32(0, 0xFFFF);
        vm.set_reg32(6, 0x0F0F);
    });
    assert_eq!(vm.reg32(6), 0xF0F0);

    let code = vec![0x31, modrm_reg_rm(1, 0)];
    let vm = step(&code, |vm| {
        vm.set_reg32(0, 0x00FF);
        vm.set_reg32(1, 0x0F0F);
    });
    assert_eq!(vm.reg32(0), 0x0FF0);

    let mut code = vec![0x0D];
    code.extend_from_slice(&0x0101u32.to_le_bytes());
    let vm = step(&code, |vm| vm.set_reg32(0, 0x0010));
    assert_eq!(vm.reg32(0), 0x0111);

    let mut code = vec![0x25];
    code.extend_from_slice(&0x0F0Fu32.to_le_bytes());
    let vm = step(&code, |vm| vm.set_reg32(0, 0xF0F0));
    assert_eq!(vm.reg32(0), 0x0000);

    let mut code = vec![0x35];
    code.extend_from_slice(&0x00FFu32.to_le_bytes());
    let vm = step(&code, |vm| vm.set_reg32(0, 0x0F0F));
    assert_eq!(vm.reg32(0), 0x0FF0);

    let code = vec![0x85, modrm_reg_rm(0, 1)];
    let vm = step(&code, |vm| {
        vm.set_reg32(0, 0);
        vm.set_reg32(1, 0);
    });
    assert!(vm.zf());
}

// Cover additional mov variants and immediates.
#[test]
fn mov_more_variants() {
    let mem = addr(0x270);
    let mut code = vec![0x89, modrm_disp32(1)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| vm.set_reg32(1, 0x1111_2222));
    assert_eq!(vm.read_u32(mem).unwrap(), 0x1111_2222);

    let mut code = vec![0x8A, modrm_disp32(2)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u8(mem, 0x7E).unwrap();
    });
    assert_eq!(vm.reg8(2), 0x7E);
}

// Cover additional sub encodings and flags.
#[test]
fn sub_more_variants() {
    let mem = addr(0x228);
    let code = vec![0x1A, modrm_reg_rm(0, 1)];
    let vm = step(&code, |vm| {
        vm.set_flags(false, false, false, true);
        vm.set_reg8(0, 0);
        vm.set_reg8(1, 1);
    });
    assert_eq!(vm.reg8(0), 0xFE);

    let code = vec![0x19, modrm_reg_rm(1, 0)];
    let vm = step(&code, |vm| {
        vm.set_flags(false, false, false, true);
        vm.set_reg32(1, 0);
        vm.set_reg32(0, 1);
    });
    assert_eq!(vm.reg32(0), 0);

    let code = [0x1C, 0x01];
    let vm = step(&code, |vm| {
        vm.set_flags(false, false, false, true);
        vm.set_reg8(0, 1);
    });
    assert_eq!(vm.reg8(0), 0xFF);

    let mut code = vec![0x1D];
    code.extend_from_slice(&1u32.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.set_flags(false, false, false, true);
        vm.set_reg32(0, 1);
    });
    assert_eq!(vm.reg32(0), 0xFFFF_FFFF);

    let mut code = vec![0x39, modrm_disp32(0)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 1).unwrap();
        vm.set_reg32(0, 1);
    });
    assert!(vm.zf());

    let mut code = vec![0x3A, modrm_disp32(2)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u8(mem, 1).unwrap();
        vm.set_reg8(2, 1);
    });
    assert!(vm.zf());

    let mut code = vec![0x66, 0x39, modrm_disp32(3)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 0x0000_0001).unwrap();
        vm.set_reg16(3, 1);
    });
    assert!(vm.zf());

    let mut code = vec![0x80, modrm_disp32(5)];
    code.extend_from_slice(&mem.to_le_bytes());
    code.push(1);
    let vm = step(&code, |vm| vm.write_u8(mem, 3).unwrap());
    assert_eq!(vm.read_u8(mem).unwrap(), 2);

    let mut code = vec![0x80, modrm_disp32(3)];
    code.extend_from_slice(&mem.to_le_bytes());
    code.push(1);
    let vm = step(&code, |vm| {
        vm.set_flags(false, false, false, true);
        vm.write_u8(mem, 1).unwrap();
    });
    assert_eq!(vm.read_u8(mem).unwrap(), 0xFF);

    let mut code = vec![0x81, modrm_disp32(5)];
    code.extend_from_slice(&mem.to_le_bytes());
    code.extend_from_slice(&1u32.to_le_bytes());
    let vm = step(&code, |vm| vm.write_u32(mem, 3).unwrap());
    assert_eq!(vm.read_u32(mem).unwrap(), 2);

    let mut code = vec![0x81, modrm_disp32(3)];
    code.extend_from_slice(&mem.to_le_bytes());
    code.extend_from_slice(&1u32.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.set_flags(false, false, false, true);
        vm.write_u32(mem, 1).unwrap();
    });
    assert_eq!(vm.read_u32(mem).unwrap(), 0xFFFF_FFFF);

    let mut code = vec![0x80, modrm_disp32(7)];
    code.extend_from_slice(&mem.to_le_bytes());
    code.push(2);
    let vm = step(&code, |vm| {
        vm.write_u8(mem, 2).unwrap();
    });
    assert!(vm.zf());

    let mut code = vec![0x81, modrm_disp32(7)];
    code.extend_from_slice(&mem.to_le_bytes());
    code.extend_from_slice(&2u32.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 2).unwrap();
    });
    assert!(vm.zf());

    let mut code = vec![0xFF, modrm_disp32(0)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 1).unwrap();
    });
    assert_eq!(vm.read_u32(mem).unwrap(), 2);
}

// Cover CL-based shift variants and edge cases.
#[test]
fn shift_cl_more_variants() {
    let mem = addr(0x370);
    let mut code = vec![0xD3, modrm_disp32(1)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 0x8000_0001).unwrap();
        vm.set_reg8(1, 1);
    });
    assert_eq!(vm.read_u32(mem).unwrap(), 0xC000_0000);

    let mut code = vec![0xD3, modrm_disp32(4)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 1).unwrap();
        vm.set_reg8(1, 1);
    });
    assert_eq!(vm.read_u32(mem).unwrap(), 2);

    let mut code = vec![0xD3, modrm_disp32(5)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 0x8000_0000).unwrap();
        vm.set_reg8(1, 1);
    });
    assert_eq!(vm.read_u32(mem).unwrap(), 0x4000_0000);
}

// Cover group1 32-bit immediates and encodings.
#[test]
fn group1_32_more_variants() {
    let mem = addr(0x3B0);
    let mut code = vec![0x81, modrm_disp32(1)];
    code.extend_from_slice(&mem.to_le_bytes());
    code.extend_from_slice(&1u32.to_le_bytes());
    let vm = step(&code, |vm| vm.write_u32(mem, 1).unwrap());
    assert_eq!(vm.read_u32(mem).unwrap(), 1);

    let mut code = vec![0x81, modrm_disp32(2)];
    code.extend_from_slice(&mem.to_le_bytes());
    code.extend_from_slice(&1u32.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 1).unwrap();
        vm.set_flags(false, false, false, true);
    });
    assert_eq!(vm.read_u32(mem).unwrap(), 3);

    let mut code = vec![0x81, modrm_disp32(3)];
    code.extend_from_slice(&mem.to_le_bytes());
    code.extend_from_slice(&1u32.to_le_bytes());
    let vm = step(&code, |vm| {
        vm.write_u32(mem, 2).unwrap();
        vm.set_flags(false, false, false, true);
    });
    assert_eq!(vm.read_u32(mem).unwrap(), 0);

    let mut code = vec![0x81, modrm_disp32(4)];
    code.extend_from_slice(&mem.to_le_bytes());
    code.extend_from_slice(&1u32.to_le_bytes());
    let vm = step(&code, |vm| vm.write_u32(mem, 3).unwrap());
    assert_eq!(vm.read_u32(mem).unwrap(), 1);

    let mut code = vec![0x81, modrm_disp32(6)];
    code.extend_from_slice(&mem.to_le_bytes());
    code.extend_from_slice(&1u32.to_le_bytes());
    let vm = step(&code, |vm| vm.write_u32(mem, 3).unwrap());
    assert_eq!(vm.read_u32(mem).unwrap(), 2);

    let mut code = vec![0x81, modrm_disp32(7)];
    code.extend_from_slice(&mem.to_le_bytes());
    code.extend_from_slice(&2u32.to_le_bytes());
    let vm = step(&code, |vm| vm.write_u32(mem, 2).unwrap());
    assert!(vm.zf());

    let mut code = vec![0x83, modrm_disp32(0)];
    code.extend_from_slice(&mem.to_le_bytes());
    code.push(0xFF);
    let vm = step(&code, |vm| vm.write_u32(mem, 2).unwrap());
    assert_eq!(vm.read_u32(mem).unwrap(), 1);
}

// Cover group F6/F7 test/neg/not variants.
#[test]
fn group_f6_f7_test_variants() {
    let mem = addr(0x3C0);
    let mut code = vec![0xF6, modrm_disp32(0)];
    code.extend_from_slice(&mem.to_le_bytes());
    code.push(0x0F);
    let vm = step(&code, |vm| vm.write_u8(mem, 0x00).unwrap());
    assert!(vm.zf());

    let mut code = vec![0xF7, modrm_disp32(0)];
    code.extend_from_slice(&mem.to_le_bytes());
    code.extend_from_slice(&0x0F0Fu32.to_le_bytes());
    let vm = step(&code, |vm| vm.write_u32(mem, 0).unwrap());
    assert!(vm.zf());

    let mut code = vec![0xF6, modrm_disp32(2)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| vm.write_u8(mem, 0x0F).unwrap());
    assert_eq!(vm.read_u8(mem).unwrap(), 0xF0);

    let mut code = vec![0xF7, modrm_disp32(2)];
    code.extend_from_slice(&mem.to_le_bytes());
    let vm = step(&code, |vm| vm.write_u32(mem, 0x0F0F_0F0F).unwrap());
    assert_eq!(vm.read_u32(mem).unwrap(), 0xF0F0_F0F0);
}
