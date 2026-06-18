use rax::arm::{AArch64Config, AArch64Cpu, Aarch64SysReg, ArmCpu, CpuExit, FlatMemory};

// Spec: tests/arm/a64/arithmetic/floating_point/half_precision/arithmetic/add/spec.asl

fn create_test_cpu() -> AArch64Cpu {
    let memory = FlatMemory::new(0, 0x1000_0000);
    AArch64Cpu::new(AArch64Config::default(), Box::new(memory))
}

fn write_insn(cpu: &mut AArch64Cpu, addr: u64, insn: u32) {
    cpu.write_memory(addr, &insn.to_le_bytes()).unwrap();
}

fn encode_movz_x(rd: u32, imm16: u16, shift: u32) -> u32 {
    debug_assert!(rd < 32);
    debug_assert!(shift % 16 == 0);
    let hw = shift / 16;
    debug_assert!(hw <= 3);

    0xD2800000 | ((imm16 as u32) << 5) | (hw << 21) | rd
}

fn encode_msr_sysreg(rt: u32, reg: Aarch64SysReg) -> u32 {
    debug_assert!(rt < 32);
    0xD5000000 | reg.encoding().msr_encoding() | (rt & 0x1F)
}

fn setup_fp_simd(cpu: &mut AArch64Cpu, addr: u64) -> u64 {
    let mov = encode_movz_x(0, 0x30, 16);
    let msr = encode_msr_sysreg(0, Aarch64SysReg::CPACR_EL1);

    write_insn(cpu, addr, mov);
    write_insn(cpu, addr + 4, msr);
    addr + 8
}

fn set_qreg(cpu: &mut AArch64Cpu, reg: u8, value: u128) {
    let low = value as u64;
    let high = (value >> 64) as u64;
    cpu.set_simd_reg(reg, low, high).unwrap();
}

fn get_qreg(cpu: &AArch64Cpu, reg: u8) -> u128 {
    let (low, high) = cpu.get_simd_reg(reg).unwrap();
    (high as u128) << 64 | (low as u128)
}

fn encode_vec_add_fp16(q: u32, u: u32, rm: u32, rn: u32, rd: u32) -> u32 {
    debug_assert!(q <= 1);
    debug_assert!(u <= 1);
    debug_assert!(rm < 32);
    debug_assert!(rn < 32);
    debug_assert!(rd < 32);

    let mut insn = 0u32;
    insn |= 0b01110 << 24;
    insn |= (q & 1) << 30;
    insn |= (u & 1) << 29;
    insn |= 1 << 22;
    insn |= (rm & 0x1f) << 16;
    insn |= 0b000101 << 10;
    insn |= (rn & 0x1f) << 5;
    insn |= rd & 0x1f;
    insn
}

fn encode_vec_add_fp(q: u32, u: u32, sz: u32, rm: u32, rn: u32, rd: u32) -> u32 {
    debug_assert!(q <= 1);
    debug_assert!(u <= 1);
    debug_assert!(sz <= 1);
    debug_assert!(rm < 32);
    debug_assert!(rn < 32);
    debug_assert!(rd < 32);

    let mut insn = 0u32;
    insn |= 0b01110 << 24;
    insn |= (q & 1) << 30;
    insn |= (u & 1) << 29;
    insn |= (sz & 1) << 22;
    insn |= 1 << 21;
    insn |= (rm & 0x1f) << 16;
    insn |= 0b110101 << 10;
    insn |= (rn & 0x1f) << 5;
    insn |= rd & 0x1f;
    insn
}

fn pack_f32x2(a: f32, b: f32) -> u64 {
    (a.to_bits() as u64) | ((b.to_bits() as u64) << 32)
}

fn pack_f32x4(a: f32, b: f32, c: f32, d: f32) -> u128 {
    (a.to_bits() as u128)
        | ((b.to_bits() as u128) << 32)
        | ((c.to_bits() as u128) << 64)
        | ((d.to_bits() as u128) << 96)
}

fn pack_f64x2(a: f64, b: f64) -> u128 {
    (a.to_bits() as u128) | ((b.to_bits() as u128) << 64)
}

fn pack_u16x8(lanes: [u16; 8]) -> u128 {
    let mut value = 0u128;
    for (idx, lane) in lanes.iter().enumerate() {
        value |= (*lane as u128) << (idx * 16);
    }
    value
}

#[test]
fn test_vec_add_fp32_elementwise_q0() {
    let insn = encode_vec_add_fp(0, 0, 0, 2, 1, 0);
    let mut cpu = create_test_cpu();
    let next_pc = setup_fp_simd(&mut cpu, 0);
    write_insn(&mut cpu, next_pc, insn);

    set_qreg(&mut cpu, 1, pack_f32x2(1.5, -2.0) as u128);
    set_qreg(&mut cpu, 2, pack_f32x2(2.25, 3.5) as u128);

    cpu.step().unwrap();
    cpu.step().unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue);

    let expected = pack_f32x2(1.5 + 2.25, -2.0 + 3.5);
    let result = get_qreg(&cpu, 0) as u64;
    assert_eq!(result, expected);
}

#[test]
fn test_vec_add_fp32_elementwise_q1() {
    let insn = encode_vec_add_fp(1, 0, 0, 2, 1, 0);
    let mut cpu = create_test_cpu();
    let next_pc = setup_fp_simd(&mut cpu, 0);
    write_insn(&mut cpu, next_pc, insn);

    set_qreg(&mut cpu, 1, pack_f32x4(1.0, 2.0, 3.0, 4.0));
    set_qreg(&mut cpu, 2, pack_f32x4(0.5, -1.0, 2.5, -4.0));

    cpu.step().unwrap();
    cpu.step().unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue);

    let expected = pack_f32x4(1.5, 1.0, 5.5, 0.0);
    let result = get_qreg(&cpu, 0);
    assert_eq!(result, expected);
}

#[test]
fn test_vec_add_fp64_elementwise_q1() {
    let insn = encode_vec_add_fp(1, 0, 1, 2, 1, 0);
    let mut cpu = create_test_cpu();
    let next_pc = setup_fp_simd(&mut cpu, 0);
    write_insn(&mut cpu, next_pc, insn);

    set_qreg(&mut cpu, 1, pack_f64x2(1.25, -2.0));
    set_qreg(&mut cpu, 2, pack_f64x2(0.75, 4.5));

    cpu.step().unwrap();
    cpu.step().unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue);

    let expected = pack_f64x2(2.0, 2.5);
    let result = get_qreg(&cpu, 0);
    assert_eq!(result, expected);
}

#[test]
fn test_vec_add_fp32_pairwise_q0() {
    let insn = encode_vec_add_fp(0, 1, 0, 2, 1, 0);
    let mut cpu = create_test_cpu();
    let next_pc = setup_fp_simd(&mut cpu, 0);
    write_insn(&mut cpu, next_pc, insn);

    set_qreg(&mut cpu, 1, pack_f32x2(1.0, 2.0) as u128);
    set_qreg(&mut cpu, 2, pack_f32x2(3.0, 4.0) as u128);

    cpu.step().unwrap();
    cpu.step().unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue);

    let expected = pack_f32x2(3.0, 7.0);
    let result = get_qreg(&cpu, 0) as u64;
    assert_eq!(result, expected);
}

#[test]
fn test_vec_add_fp32_pairwise_q1() {
    let insn = encode_vec_add_fp(1, 1, 0, 2, 1, 0);
    let mut cpu = create_test_cpu();
    let next_pc = setup_fp_simd(&mut cpu, 0);
    write_insn(&mut cpu, next_pc, insn);

    set_qreg(&mut cpu, 1, pack_f32x4(1.0, 2.0, 3.0, 4.0));
    set_qreg(&mut cpu, 2, pack_f32x4(5.0, 6.0, 7.0, 8.0));

    cpu.step().unwrap();
    cpu.step().unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue);

    let expected = pack_f32x4(3.0, 7.0, 11.0, 15.0);
    let result = get_qreg(&cpu, 0);
    assert_eq!(result, expected);
}

#[test]
fn test_vec_add_fp16_elementwise_q1() {
    let insn = encode_vec_add_fp16(1, 0, 2, 1, 0);
    let mut cpu = create_test_cpu();
    let next_pc = setup_fp_simd(&mut cpu, 0);
    write_insn(&mut cpu, next_pc, insn);

    const H_1: u16 = 0x3C00;
    const H_0_5: u16 = 0x3800;
    const H_2: u16 = 0x4000;
    const H_4: u16 = 0x4400;
    const H_8: u16 = 0x4800;

    let input = [H_1, H_1, H_0_5, H_0_5, H_2, H_2, H_4, H_4];
    set_qreg(&mut cpu, 1, pack_u16x8(input));
    set_qreg(&mut cpu, 2, pack_u16x8(input));

    cpu.step().unwrap();
    cpu.step().unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue);

    let expected = pack_u16x8([H_2, H_2, H_1, H_1, H_4, H_4, H_8, H_8]);
    let result = get_qreg(&cpu, 0);
    assert_eq!(result, expected);
}

#[test]
fn test_vec_add_fp16_pairwise_q1() {
    let insn = encode_vec_add_fp16(1, 1, 2, 1, 0);
    let mut cpu = create_test_cpu();
    let next_pc = setup_fp_simd(&mut cpu, 0);
    write_insn(&mut cpu, next_pc, insn);

    const H_1: u16 = 0x3C00;
    const H_0_5: u16 = 0x3800;
    const H_2: u16 = 0x4000;
    const H_4: u16 = 0x4400;
    const H_8: u16 = 0x4800;

    let input = [H_1, H_1, H_0_5, H_0_5, H_2, H_2, H_4, H_4];
    set_qreg(&mut cpu, 1, pack_u16x8(input));
    set_qreg(&mut cpu, 2, pack_u16x8(input));

    cpu.step().unwrap();
    cpu.step().unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue);

    let expected = pack_u16x8([H_2, H_1, H_4, H_8, H_2, H_1, H_4, H_8]);
    let result = get_qreg(&cpu, 0);
    assert_eq!(result, expected);
}
