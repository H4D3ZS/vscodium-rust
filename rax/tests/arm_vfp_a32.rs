use rax::arm::decoder::{Aarch32Decoder, DecodedInsn, Mnemonic};
use rax::arm::execution::{ArmMemory, FlatMemory};
use rax::arm::vfp::RoundingMode;
use rax::arm::{Armv7Cpu, ExceptionType, ExecResult, ExecutionState, Executor};

fn exec_one(cpu: &mut Armv7Cpu, mem: &mut FlatMemory, raw: u32) -> ExecResult {
    let insn = Aarch32Decoder::decode(raw).unwrap();
    Executor::new(cpu, mem).execute(&insn)
}

#[test]
fn decodes_vfp_load_store_before_generic_coprocessor() {
    let vstr = Aarch32Decoder::decode(0xED01_0A00).unwrap();
    let vldr = Aarch32Decoder::decode(0xED11_0A00).unwrap();

    assert_eq!(vstr.mnemonic, Mnemonic::VSTR);
    assert_eq!(vldr.mnemonic, Mnemonic::VLDR);
}

#[test]
fn vmov_core_to_scalar_and_back_round_trips_bits() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    cpu.regs[0] = 0x3fc0_0000;
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEE00_0A10),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 0x3fc0_0000);

    cpu.regs[1] = 0;
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEE10_1A10),
        ExecResult::Continue
    ));
    assert_eq!(cpu.regs[1], 0x3fc0_0000);
}

#[test]
fn vmov_core_to_double_lane_preserves_other_lane() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xEE20_0B10).unwrap().mnemonic,
        Mnemonic::VMOV
    );
    assert_eq!(
        Aarch32Decoder::decode(0xEE31_2B10).unwrap().mnemonic,
        Mnemonic::VMOV
    );

    cpu.vfp.write_d_bits(0, 0x1122_3344_5566_7788);
    cpu.regs[0] = 0xaabb_ccdd;
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEE20_0B10),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0xaabb_ccdd_5566_7788);

    cpu.vfp.write_d_bits(1, 0x1234_5678_9abc_def0);
    cpu.regs[2] = 0;
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEE31_2B10),
        ExecResult::Continue
    ));
    assert_eq!(cpu.regs[2], 0x1234_5678);
}

#[test]
fn vmov_core_to_neon_byte_halfword_lanes_and_back_extends() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xEE62_4B30).unwrap().mnemonic,
        Mnemonic::VMOV
    );
    assert_eq!(
        Aarch32Decoder::decode(0xEE72_5B30).unwrap().mnemonic,
        Mnemonic::VMOV
    );
    assert_eq!(
        Aarch32Decoder::decode(0xEEF2_6B30).unwrap().mnemonic,
        Mnemonic::VMOV
    );
    assert_eq!(
        Aarch32Decoder::decode(0xEE02_4B50).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    assert_eq!(
        Aarch32Decoder::decode(0xEE92_4B10).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.vfp.write_d_bits(2, 0x1122_3344_5566_7788);
    cpu.regs[4] = 0xffff_ff80;
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEE62_4B30),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(2), 0x1122_8044_5566_7788);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEE72_5B30),
        ExecResult::Continue
    ));
    assert_eq!(cpu.regs[5], 0xffff_ff80);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEF2_6B30),
        ExecResult::Continue
    ));
    assert_eq!(cpu.regs[6], 0x80);

    cpu.vfp.write_d_bits(3, 0xaaaa_bbbb_cccc_dddd);
    cpu.regs[7] = 0xffff_8001;
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEE23_7B30),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(3), 0xaaaa_8001_cccc_dddd);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEE33_8B30),
        ExecResult::Continue
    ));
    assert_eq!(cpu.regs[8], 0xffff_8001);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEB3_9B30),
        ExecResult::Continue
    ));
    assert_eq!(cpu.regs[9], 0x8001);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEE02_4B50),
        ExecResult::Undefined
    ));
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEE92_4B10),
        ExecResult::Undefined
    ));
}

#[test]
fn vmov_double_to_core_pair_round_trips_bits() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xEC41_0B10).unwrap().mnemonic,
        Mnemonic::VMOV
    );
    assert_eq!(
        Aarch32Decoder::decode(0xEC43_2B18).unwrap().mnemonic,
        Mnemonic::VMOV
    );

    cpu.regs[0] = 0x89ab_cdef;
    cpu.regs[1] = 0x0123_4567;
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEC41_0B10),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0123_4567_89ab_cdef);

    cpu.regs[0] = 0;
    cpu.regs[1] = 0;
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEC51_0B10),
        ExecResult::Continue
    ));
    assert_eq!(cpu.regs[0], 0x89ab_cdef);
    assert_eq!(cpu.regs[1], 0x0123_4567);

    cpu.regs[2] = 0x3333_4444;
    cpu.regs[3] = 0x1111_2222;
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEC43_2B18),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(8), 0x1111_2222_3333_4444);
}

#[test]
fn vmov_single_pair_to_core_pair_round_trips_bits() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xEC41_0A10).unwrap().mnemonic,
        Mnemonic::VMOV
    );
    assert_eq!(
        Aarch32Decoder::decode(0xEC43_2A18).unwrap().mnemonic,
        Mnemonic::VMOV
    );

    cpu.regs[0] = 0x3f80_0000;
    cpu.regs[1] = 0x4000_0000;
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEC41_0A10),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 0x3f80_0000);
    assert_eq!(cpu.vfp.read_s_bits(1), 0x4000_0000);

    cpu.regs[0] = 0;
    cpu.regs[1] = 0;
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEC51_0A10),
        ExecResult::Continue
    ));
    assert_eq!(cpu.regs[0], 0x3f80_0000);
    assert_eq!(cpu.regs[1], 0x4000_0000);

    cpu.regs[2] = 0x4040_0000;
    cpu.regs[3] = 0x4080_0000;
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEC43_2A18),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(16), 0x4040_0000);
    assert_eq!(cpu.vfp.read_s_bits(17), 0x4080_0000);
}

#[test]
fn vmov_immediate_materializes_scalar_constants() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xEEB7_0900).unwrap().mnemonic,
        Mnemonic::VMOV
    );
    assert_eq!(
        Aarch32Decoder::decode(0xEEB7_0A00).unwrap().mnemonic,
        Mnemonic::VMOV
    );
    assert_eq!(
        Aarch32Decoder::decode(0xEEB7_0B00).unwrap().mnemonic,
        Mnemonic::VMOV
    );

    cpu.vfp.write_s_bits(0, 0xaaaa_0000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEB7_0900),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 0xaaaa_3c00);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEB7_0A00),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 1.0f32.to_bits());

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEB7_0B00),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 1.0f64.to_bits());

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEB8_8B00),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(8), (-2.0f64).to_bits());
}

#[test]
fn scalar_fp16_vmov_register_preserves_halves() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xEEB0_0960).unwrap().mnemonic,
        Mnemonic::VMOV
    );

    cpu.vfp.write_s_bits(0, 0xaaaa_1111);
    cpu.vfp.write_s_bits(1, 0xbbbb_4000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEB0_0960),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 0xaaaa_4000);
    assert_eq!(cpu.vfp.read_s_bits(1), 0xbbbb_4000);
}

#[test]
fn scalar_vadd_f32_executes_against_vfp_register_file() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    cpu.vfp.write_s(0, 1.5);
    cpu.vfp.write_s(1, 2.25);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEE30_0A20),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 3.75f32.to_bits());

    cpu.vfp.write_s(0, 4.0);
    cpu.vfp.write_s(1, 5.0);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEE70_0A20),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(1), 9.0f32.to_bits());
}

#[test]
fn scalar_multiply_accumulate_forms_execute() {
    let cases = [
        (0xEE10_0AC1, Mnemonic::VNMLA),
        (0xEE10_0A81, Mnemonic::VNMLS),
        (0xEE20_0AC1, Mnemonic::VNMUL),
        (0xEEA0_0A81, Mnemonic::VFMA),
        (0xEEA0_0AC1, Mnemonic::VFMS),
        (0xEE90_0AC1, Mnemonic::VFNMA),
        (0xEE90_0A81, Mnemonic::VFNMS),
        (0xEEA1_0B02, Mnemonic::VFMA),
        (0xEEA1_0B42, Mnemonic::VFMS),
        (0xEE91_0B42, Mnemonic::VFNMA),
        (0xEE91_0B02, Mnemonic::VFNMS),
    ];
    for (raw, mnemonic) in cases {
        assert_eq!(Aarch32Decoder::decode(raw).unwrap().mnemonic, mnemonic);
    }

    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    cpu.vfp.write_s(1, 2.0);
    cpu.vfp.write_s(2, 3.0);

    cpu.vfp.write_s(0, 10.0);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEE00_0A81),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 16.0f32.to_bits());

    cpu.vfp.write_s(0, 10.0);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEE00_0AC1),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 4.0f32.to_bits());

    cpu.vfp.write_s(0, 10.0);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEE10_0AC1),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), (-16.0f32).to_bits());

    cpu.vfp.write_s(0, 10.0);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEE10_0A81),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), (-4.0f32).to_bits());

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEE20_0AC1),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), (-6.0f32).to_bits());

    cpu.vfp.write_s(0, 10.0);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEA0_0A81),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 16.0f32.to_bits());

    cpu.vfp.write_s(0, 10.0);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEA0_0AC1),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 4.0f32.to_bits());

    cpu.vfp.write_s(0, 10.0);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEE90_0AC1),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), (-16.0f32).to_bits());

    cpu.vfp.write_s(0, 10.0);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEE90_0A81),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), (-4.0f32).to_bits());
}

#[test]
fn scalar_multiply_accumulate_forms_execute_f64() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    let cases = [
        (0xEE11_0B42, Mnemonic::VNMLA),
        (0xEE11_0B02, Mnemonic::VNMLS),
        (0xEE21_0B42, Mnemonic::VNMUL),
    ];
    for (raw, mnemonic) in cases {
        assert_eq!(Aarch32Decoder::decode(raw).unwrap().mnemonic, mnemonic);
    }

    cpu.vfp.write_d(1, 1.5);
    cpu.vfp.write_d(2, 4.0);

    cpu.vfp.write_d(0, 10.0);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEE01_0B02),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 16.0f64.to_bits());

    cpu.vfp.write_d(0, 10.0);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEE01_0B42),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 4.0f64.to_bits());

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEE21_0B42),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), (-6.0f64).to_bits());

    cpu.vfp.write_d(0, 10.0);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEA1_0B02),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 16.0f64.to_bits());

    cpu.vfp.write_d(0, 10.0);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEA1_0B42),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 4.0f64.to_bits());

    cpu.vfp.write_d(0, 10.0);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEE91_0B42),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), (-16.0f64).to_bits());

    cpu.vfp.write_d(0, 10.0);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEE91_0B02),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), (-4.0f64).to_bits());
}

#[test]
fn scalar_fp16_arithmetic_unary_and_accumulate_execute() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    let cases = [
        (0xEE30_0981, Mnemonic::VADD),
        (0xEE30_09C1, Mnemonic::VSUB),
        (0xEE20_0981, Mnemonic::VMUL),
        (0xEE10_09C1, Mnemonic::VNMLA),
        (0xEE10_0981, Mnemonic::VNMLS),
        (0xEE20_09C1, Mnemonic::VNMUL),
        (0xEE80_0981, Mnemonic::VDIV),
        (0xEEB0_09E0, Mnemonic::VABS),
        (0xEEB1_0960, Mnemonic::VNEG),
        (0xEEB1_09E0, Mnemonic::VSQRT),
        (0xEE00_0981, Mnemonic::VMLA),
    ];
    for (raw, mnemonic) in cases {
        assert_eq!(Aarch32Decoder::decode(raw).unwrap().mnemonic, mnemonic);
    }

    cpu.vfp.write_s_bits(0, 0xaaaa_0000);
    cpu.vfp.write_h_bits(1, 0x3e00); // 1.5
    cpu.vfp.write_h_bits(2, 0x4000); // 2.0
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEE30_0981),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 0xaaaa_4300); // 3.5

    cpu.vfp.write_s_bits(0, 0xbbbb_0000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEE30_09C1),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 0xbbbb_b800); // -0.5

    cpu.vfp.write_s_bits(0, 0xcccc_4900); // 10.0
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEE10_09C1),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 0xcccc_ca80); // -13.0

    cpu.vfp.write_s_bits(0, 0xdddd_4900); // 10.0
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEE10_0981),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 0xdddd_c700); // -7.0

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEE20_09C1),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 0xdddd_c200); // -3.0

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEE20_0981),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_h_bits(0), 0x4200); // 3.0

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEE80_0981),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_h_bits(0), 0x3a00); // 0.75

    cpu.vfp.write_h_bits(1, 0xc200); // -3.0
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEB0_09E0),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_h_bits(0), 0x4200);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEB1_0960),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_h_bits(0), 0x4200);

    cpu.vfp.write_h_bits(1, 0x4000); // 2.0
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEB1_09E0),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_h_bits(0), 0x3da8); // sqrt(2), rounded to binary16

    cpu.vfp.write_h_bits(0, 0x4500); // 5.0
    cpu.vfp.write_h_bits(1, 0x3e00); // 1.5
    cpu.vfp.write_h_bits(2, 0x4000); // 2.0
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEE00_0981),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_h_bits(0), 0x4800); // 8.0
}

#[test]
fn scalar_vsel_decodes_and_selects_from_fpscr_flags() {
    let cases = [
        (0xFE00_0981, Mnemonic::VSELEQ),
        (0xFE10_0981, Mnemonic::VSELVS),
        (0xFE20_0981, Mnemonic::VSELGE),
        (0xFE30_0981, Mnemonic::VSELGT),
        (0xFE00_0A81, Mnemonic::VSELEQ),
        (0xFE10_0A81, Mnemonic::VSELVS),
        (0xFE20_0A81, Mnemonic::VSELGE),
        (0xFE30_0A81, Mnemonic::VSELGT),
        (0xFE01_0B02, Mnemonic::VSELEQ),
        (0xFE11_0B02, Mnemonic::VSELVS),
        (0xFE21_0B02, Mnemonic::VSELGE),
        (0xFE31_0B02, Mnemonic::VSELGT),
    ];
    for (raw, mnemonic) in cases {
        assert_eq!(Aarch32Decoder::decode(raw).unwrap().mnemonic, mnemonic);
    }

    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    cpu.vfp.write_s_bits(0, 0xaaaa_0000);
    cpu.vfp.write_h_bits(1, 0x3e00); // 1.5
    cpu.vfp.write_h_bits(2, 0x4000); // 2.0

    cpu.vfp.fpscr.set_nzcv(false, true, false, false);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFE00_0981),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 0xaaaa_3e00);

    cpu.vfp.fpscr.set_nzcv(false, false, false, false);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFE00_0981),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 0xaaaa_4000);

    cpu.vfp.write_s_bits(1, 0x1111_1111);
    cpu.vfp.write_s_bits(2, 0x2222_2222);

    cpu.vfp.fpscr.set_nzcv(false, true, false, false);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFE00_0A81),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 0x1111_1111);

    cpu.vfp.fpscr.set_nzcv(false, false, false, false);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFE00_0A81),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 0x2222_2222);

    cpu.vfp.fpscr.set_nzcv(false, false, false, true);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFE10_0A81),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 0x1111_1111);

    cpu.vfp.fpscr.set_nzcv(true, false, false, true);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFE20_0A81),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 0x1111_1111);

    cpu.vfp.fpscr.set_nzcv(true, true, false, true);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFE30_0A81),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 0x2222_2222);

    cpu.vfp.write_d_bits(1, 0x1111_2222_3333_4444);
    cpu.vfp.write_d_bits(2, 0xaaaa_bbbb_cccc_dddd);
    cpu.vfp.fpscr.set_nzcv(false, false, false, true);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFE11_0B02),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x1111_2222_3333_4444);
}

#[test]
fn scalar_vsel_size_zero_decodes_undefined_not_generic_coprocessor() {
    let insn = Aarch32Decoder::decode(0xFE00_0800).unwrap();
    assert_eq!(insn.mnemonic, Mnemonic::UNDEFINED);

    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFE00_0800),
        ExecResult::Undefined
    ));
}

#[test]
fn scalar_vrint_size_zero_decodes_undefined_not_generic_coprocessor() {
    let insn = Aarch32Decoder::decode(0xFEB8_0860).unwrap();
    assert_eq!(insn.mnemonic, Mnemonic::UNDEFINED);

    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFEB8_0860),
        ExecResult::Undefined
    ));
}

#[test]
fn scalar_vmaxnm_vminnm_decode_and_execute_number_minmax() {
    let cases = [
        (0xFE80_0981, Mnemonic::VMAXNM_F16),
        (0xFE80_09C1, Mnemonic::VMINNM_F16),
        (0xFE80_0A81, Mnemonic::VMAXNM_F32),
        (0xFE80_0AC1, Mnemonic::VMINNM_F32),
        (0xFE81_0B02, Mnemonic::VMAXNM_F64),
        (0xFE81_0B42, Mnemonic::VMINNM_F64),
    ];
    for (raw, mnemonic) in cases {
        assert_eq!(Aarch32Decoder::decode(raw).unwrap().mnemonic, mnemonic);
    }

    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    cpu.vfp.write_h_bits(1, 0xc200); // -3.0
    cpu.vfp.write_h_bits(2, 0x4500); // 5.0
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFE80_0981),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_h_bits(0), 0x4500);

    cpu.vfp.write_h_bits(1, 0xc200);
    cpu.vfp.write_h_bits(2, 0x4500);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFE80_09C1),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_h_bits(0), 0xc200);

    cpu.vfp.fpscr.set_ioc(false);
    cpu.vfp.write_h_bits(1, 0x7e00); // NaN
    cpu.vfp.write_h_bits(2, 0x4700); // 7.0
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFE80_0981),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_h_bits(0), 0x4700);
    assert!(cpu.vfp.fpscr.ioc());

    cpu.vfp.fpscr.set_ioc(false);
    cpu.vfp.write_h_bits(1, 0x8000); // -0.0
    cpu.vfp.write_h_bits(2, 0x0000); // 0.0
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFE80_0981),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_h_bits(0), 0x0000);
    assert!(!cpu.vfp.fpscr.ioc());

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFE80_09C1),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_h_bits(0), 0x8000);

    cpu.vfp.write_s(1, -3.0);
    cpu.vfp.write_s(2, 5.0);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFE80_0A81),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 5.0f32.to_bits());

    cpu.vfp.write_s(1, -3.0);
    cpu.vfp.write_s(2, 5.0);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFE80_0AC1),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), (-3.0f32).to_bits());

    cpu.vfp.fpscr.set_ioc(false);
    cpu.vfp.write_s(1, f32::NAN);
    cpu.vfp.write_s(2, 7.0);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFE80_0A81),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 7.0f32.to_bits());
    assert!(cpu.vfp.fpscr.ioc());

    cpu.vfp.fpscr.set_ioc(false);
    cpu.vfp.write_s(1, -0.0);
    cpu.vfp.write_s(2, 0.0);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFE80_0A81),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 0.0f32.to_bits());
    assert!(!cpu.vfp.fpscr.ioc());

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFE80_0AC1),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), (-0.0f32).to_bits());

    cpu.vfp.write_d(1, -8.0);
    cpu.vfp.write_d(2, 4.0);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFE81_0B02),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 4.0f64.to_bits());

    cpu.vfp.write_d(1, -8.0);
    cpu.vfp.write_d(2, 4.0);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFE81_0B42),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), (-8.0f64).to_bits());
}

#[test]
fn scalar_unary_and_compare_update_vfp_state() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    cpu.vfp.write_s(1, -9.0);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEB0_1AE0),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(2), 9.0f32.to_bits());

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEF1_1A41),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(3), (-9.0f32).to_bits());

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEB1_2AE1),
        ExecResult::Continue
    ));
    assert!(cpu.vfp.read_s(4).is_nan());
    assert!(cpu.vfp.fpscr.ioc());

    cpu.vfp.write_s(4, 1.0);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEB4_2A61),
        ExecResult::Continue
    ));
    assert!(cpu.vfp.fpscr.c());
    assert!(!cpu.vfp.fpscr.z());
}

#[test]
fn scalar_compare_variants_decode_and_update_flags() {
    let cases = [
        (0xEEB4_0960, Mnemonic::VCMP),
        (0xEEB4_09E0, Mnemonic::VCMPE),
        (0xEEB5_0940, Mnemonic::VCMP),
        (0xEEB5_09C0, Mnemonic::VCMPE),
        (0xEEB4_0A60, Mnemonic::VCMP),
        (0xEEB4_0AE0, Mnemonic::VCMPE),
        (0xEEB5_0A40, Mnemonic::VCMP),
        (0xEEB5_0AC0, Mnemonic::VCMPE),
        (0xEEB4_0B41, Mnemonic::VCMP),
        (0xEEB4_0BC1, Mnemonic::VCMPE),
        (0xEEB5_0B40, Mnemonic::VCMP),
        (0xEEB5_0BC0, Mnemonic::VCMPE),
    ];
    for (raw, mnemonic) in cases {
        assert_eq!(Aarch32Decoder::decode(raw).unwrap().mnemonic, mnemonic);
    }

    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    cpu.vfp.write_h_bits(0, 0x3e00); // 1.5
    cpu.vfp.write_h_bits(1, 0x4000); // 2.0
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEB4_09E0),
        ExecResult::Continue
    ));
    assert!(cpu.vfp.fpscr.n());
    assert!(!cpu.vfp.fpscr.z());
    assert!(!cpu.vfp.fpscr.c());
    assert!(!cpu.vfp.fpscr.v());

    cpu.vfp.write_h_bits(0, 0xbe00); // -1.5
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEB5_0940),
        ExecResult::Continue
    ));
    assert!(cpu.vfp.fpscr.n());
    assert!(!cpu.vfp.fpscr.c());

    cpu.vfp.write_h_bits(0, 0x7e00); // NaN
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEB4_0960),
        ExecResult::Continue
    ));
    assert!(!cpu.vfp.fpscr.n());
    assert!(!cpu.vfp.fpscr.z());
    assert!(cpu.vfp.fpscr.c());
    assert!(cpu.vfp.fpscr.v());
    assert!(!cpu.vfp.fpscr.ioc());

    cpu.vfp.fpscr.set_ioc(false);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEB4_09E0),
        ExecResult::Continue
    ));
    assert!(cpu.vfp.fpscr.ioc());

    cpu.vfp.fpscr.set_ioc(false);
    cpu.vfp.write_h_bits(0, 0x7d00); // signaling NaN
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEB4_0960),
        ExecResult::Continue
    ));
    assert!(cpu.vfp.fpscr.ioc());

    cpu.vfp.write_s(0, 2.0);
    cpu.vfp.write_s(1, 2.0);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEB4_0AE0),
        ExecResult::Continue
    ));
    assert!(cpu.vfp.fpscr.z());
    assert!(cpu.vfp.fpscr.c());

    cpu.vfp.fpscr.set_ioc(false);
    cpu.vfp.write_s_bits(0, 0x7fa0_0001); // signaling NaN
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEB4_0A60),
        ExecResult::Continue
    ));
    assert!(cpu.vfp.fpscr.ioc());

    cpu.vfp.write_d(0, -1.0);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEB5_0BC0),
        ExecResult::Continue
    ));
    assert!(cpu.vfp.fpscr.n());
    assert!(!cpu.vfp.fpscr.c());

    cpu.vfp.fpscr.set_ioc(false);
    cpu.vfp.write_d_bits(0, 0x7ff8_0000_0000_0001); // quiet NaN
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEB4_0B41),
        ExecResult::Continue
    ));
    assert!(!cpu.vfp.fpscr.ioc());

    cpu.vfp.fpscr.set_ioc(false);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEB4_0BC1),
        ExecResult::Continue
    ));
    assert!(cpu.vfp.fpscr.ioc());
}

#[test]
fn scalar_conversions_decode_and_execute() {
    let cases = [
        (0xEEB8_09E0, Mnemonic::VCVT_F16_S32),
        (0xEEB8_0960, Mnemonic::VCVT_F16_U32),
        (0xEEB8_0AE0, Mnemonic::VCVT_F32_S32),
        (0xEEB8_0A60, Mnemonic::VCVT_F32_U32),
        (0xEEBD_09E0, Mnemonic::VCVT_S32_F16),
        (0xEEBC_09E0, Mnemonic::VCVT_U32_F16),
        (0xEEBD_0AE0, Mnemonic::VCVT_S32_F32),
        (0xEEBC_0AE0, Mnemonic::VCVT_U32_F32),
        (0xEEB8_0BE0, Mnemonic::VCVT_F64_S32),
        (0xEEB8_0B60, Mnemonic::VCVT_F64_U32),
        (0xEEBD_0BC1, Mnemonic::VCVT_S32_F64),
        (0xEEBC_0BC1, Mnemonic::VCVT_U32_F64),
        (0xEEB7_0AE0, Mnemonic::VCVT_F64_F32),
        (0xEEB7_0BC1, Mnemonic::VCVT_F32_F64),
    ];
    for (raw, mnemonic) in cases {
        assert_eq!(Aarch32Decoder::decode(raw).unwrap().mnemonic, mnemonic);
    }

    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    cpu.vfp.write_s_bits(0, 0xaaaa_0000);
    cpu.vfp.write_s_bits(1, (-3i32) as u32);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEB8_09E0),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 0xaaaa_c200);

    cpu.vfp.write_s_bits(0, 0xbbbb_0000);
    cpu.vfp.write_s_bits(1, 5);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEB8_0960),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 0xbbbb_4500);

    cpu.vfp.write_s_bits(1, (-42i32) as u32);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEB8_0AE0),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), (-42.0f32).to_bits());

    cpu.vfp.write_s_bits(1, 42);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEB8_0A60),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 42.0f32.to_bits());

    cpu.vfp.write_h_bits(1, 0xc100); // -2.5
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEBD_09E0),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), (-2i32) as u32);

    cpu.vfp.fpscr.set_ioc(false);
    cpu.vfp.write_h_bits(1, 0xbc00); // -1.0
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEBC_09E0),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 0);
    assert!(cpu.vfp.fpscr.ioc());

    cpu.vfp.write_s(1, -7.75);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEBD_0AE0),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), (-7i32) as u32);

    cpu.vfp.write_s(1, -1.0);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEBC_0AE0),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 0);
    assert!(cpu.vfp.fpscr.ioc());
}

#[test]
fn scalar_conversions_cover_double_precision_register_routing() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    cpu.vfp.write_s_bits(1, (-123i32) as u32);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEB8_0BE0),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), (-123.0f64).to_bits());

    cpu.vfp.write_s_bits(1, 123);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEB8_0B60),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 123.0f64.to_bits());

    cpu.vfp.write_d(1, 5.9);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEBD_0BC1),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 5);

    cpu.vfp.write_d(1, 9.25);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEBC_0BC1),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 9);

    cpu.vfp.write_s(1, 3.5);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEB7_0AE0),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 3.5f64.to_bits());

    cpu.vfp.write_d(1, 6.25);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEB7_0BC1),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 6.25f32.to_bits());
}

#[test]
fn scalar_vcvtr_decodes_and_obeys_fpscr_rounding_mode() {
    let cases = [
        (0xEEBD_0960, Mnemonic::VCVTR_S32_F16),
        (0xEEBC_0960, Mnemonic::VCVTR_U32_F16),
        (0xEEBD_0A60, Mnemonic::VCVTR_S32_F32),
        (0xEEBC_0A60, Mnemonic::VCVTR_U32_F32),
        (0xEEBD_0B41, Mnemonic::VCVTR_S32_F64),
        (0xEEBC_0B41, Mnemonic::VCVTR_U32_F64),
    ];
    for (raw, mnemonic) in cases {
        assert_eq!(Aarch32Decoder::decode(raw).unwrap().mnemonic, mnemonic);
    }

    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    cpu.vfp.fpscr.set_rmode(RoundingMode::RoundPlusInf);
    cpu.vfp.write_h_bits(1, 0x4080); // 2.25
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEBD_0960),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 3);

    cpu.vfp.fpscr.set_rmode(RoundingMode::RoundMinusInf);
    cpu.vfp.write_h_bits(1, 0xc080); // -2.25
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEBD_0960),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), (-3i32) as u32);

    cpu.vfp.fpscr.set_rmode(RoundingMode::RoundPlusInf);
    cpu.vfp.write_s(1, 2.25);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEBD_0AE0),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 2);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEBD_0A60),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 3);

    cpu.vfp.fpscr.set_rmode(RoundingMode::RoundMinusInf);
    cpu.vfp.write_d(1, -2.25);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEBD_0B41),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), (-3i32) as u32);

    cpu.vfp.fpscr.set_rmode(RoundingMode::RoundNearest);
    cpu.vfp.write_s(1, 2.5);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEBD_0A60),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 2);

    cpu.vfp.write_s(1, 3.5);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEBD_0A60),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 4);
}

#[test]
fn scalar_directed_conversions_decode_and_round_explicitly() {
    let cases = [
        (0xFEBC_09E0, Mnemonic::VCVTA_S32_F16),
        (0xFEBC_0960, Mnemonic::VCVTA_U32_F16),
        (0xFEBC_0AE0, Mnemonic::VCVTA_S32_F32),
        (0xFEBC_0A60, Mnemonic::VCVTA_U32_F32),
        (0xFEBC_0BC1, Mnemonic::VCVTA_S32_F64),
        (0xFEBC_0B41, Mnemonic::VCVTA_U32_F64),
        (0xFEBF_09E0, Mnemonic::VCVTM_S32_F16),
        (0xFEBF_0960, Mnemonic::VCVTM_U32_F16),
        (0xFEBF_0AE0, Mnemonic::VCVTM_S32_F32),
        (0xFEBF_0A60, Mnemonic::VCVTM_U32_F32),
        (0xFEBF_0BC1, Mnemonic::VCVTM_S32_F64),
        (0xFEBF_0B41, Mnemonic::VCVTM_U32_F64),
        (0xFEBD_09E0, Mnemonic::VCVTN_S32_F16),
        (0xFEBD_0960, Mnemonic::VCVTN_U32_F16),
        (0xFEBD_0AE0, Mnemonic::VCVTN_S32_F32),
        (0xFEBD_0A60, Mnemonic::VCVTN_U32_F32),
        (0xFEBD_0BC1, Mnemonic::VCVTN_S32_F64),
        (0xFEBD_0B41, Mnemonic::VCVTN_U32_F64),
        (0xFEBE_09E0, Mnemonic::VCVTP_S32_F16),
        (0xFEBE_0960, Mnemonic::VCVTP_U32_F16),
        (0xFEBE_0AE0, Mnemonic::VCVTP_S32_F32),
        (0xFEBE_0A60, Mnemonic::VCVTP_U32_F32),
        (0xFEBE_0BC1, Mnemonic::VCVTP_S32_F64),
        (0xFEBE_0B41, Mnemonic::VCVTP_U32_F64),
    ];
    for (raw, mnemonic) in cases {
        assert_eq!(Aarch32Decoder::decode(raw).unwrap().mnemonic, mnemonic);
    }

    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    cpu.vfp.write_h_bits(1, 0xc100); // -2.5
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFEBC_09E0),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), (-3i32) as u32);

    cpu.vfp.write_h_bits(1, 0x4100); // 2.5
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFEBD_09E0),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 2);

    cpu.vfp.write_h_bits(1, 0xc080); // -2.25
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFEBF_09E0),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), (-3i32) as u32);

    cpu.vfp.write_h_bits(1, 0x4080); // 2.25
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFEBE_0960),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 3);

    cpu.vfp.write_s(1, -2.5);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFEBC_0AE0),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), (-3i32) as u32);

    cpu.vfp.write_s(1, 2.5);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFEBD_0AE0),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 2);

    cpu.vfp.write_s(1, -2.1);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFEBF_0AE0),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), (-3i32) as u32);

    cpu.vfp.write_s(1, -2.1);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFEBE_0AE0),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), (-2i32) as u32);

    cpu.vfp.write_s(1, 2.1);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFEBE_0A60),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 3);

    cpu.vfp.write_d(1, 3.5);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFEBC_0BC1),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 4);
}

#[test]
fn scalar_vrint_decode_and_round_to_integral_float() {
    let cases = [
        (0xFEB8_0960, Mnemonic::VRINTA_F16),
        (0xFEB8_0A60, Mnemonic::VRINTA_F32),
        (0xFEB8_0B41, Mnemonic::VRINTA_F64),
        (0xFEBB_0960, Mnemonic::VRINTM_F16),
        (0xFEBB_0A60, Mnemonic::VRINTM_F32),
        (0xFEBB_0B41, Mnemonic::VRINTM_F64),
        (0xFEB9_0960, Mnemonic::VRINTN_F16),
        (0xFEB9_0A60, Mnemonic::VRINTN_F32),
        (0xFEB9_0B41, Mnemonic::VRINTN_F64),
        (0xFEBA_0960, Mnemonic::VRINTP_F16),
        (0xFEBA_0A60, Mnemonic::VRINTP_F32),
        (0xFEBA_0B41, Mnemonic::VRINTP_F64),
        (0xEEB6_0960, Mnemonic::VRINTR_F16),
        (0xEEB6_0A60, Mnemonic::VRINTR_F32),
        (0xEEB6_0B41, Mnemonic::VRINTR_F64),
        (0xEEB7_0960, Mnemonic::VRINTX_F16),
        (0xEEB7_0A60, Mnemonic::VRINTX_F32),
        (0xEEB7_0B41, Mnemonic::VRINTX_F64),
        (0xEEB6_09E0, Mnemonic::VRINTZ_F16),
        (0xEEB6_0AE0, Mnemonic::VRINTZ_F32),
        (0xEEB6_0BC1, Mnemonic::VRINTZ_F64),
    ];
    for (raw, mnemonic) in cases {
        assert_eq!(Aarch32Decoder::decode(raw).unwrap().mnemonic, mnemonic);
    }

    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    cpu.vfp.write_s(1, -2.5);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFEB8_0A60),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s(0), -3.0);

    cpu.vfp.write_s(1, 2.5);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFEB9_0A60),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s(0), 2.0);

    cpu.vfp.write_s(1, -2.1);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFEBB_0A60),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s(0), -3.0);

    cpu.vfp.write_s(1, -2.1);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFEBA_0A60),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s(0), -2.0);

    cpu.vfp.fpscr.set_rmode(RoundingMode::RoundPlusInf);
    cpu.vfp.write_s(1, 2.1);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEB6_0A60),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s(0), 3.0);

    cpu.vfp.write_s(1, -2.9);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEB6_0AE0),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s(0), -2.0);

    cpu.vfp.fpscr.set_ixc(false);
    cpu.vfp.fpscr.set_rmode(RoundingMode::RoundNearest);
    cpu.vfp.write_s(1, 2.25);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEB7_0A60),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s(0), 2.0);
    assert!(cpu.vfp.fpscr.ixc());

    cpu.vfp.write_s_bits(0, 0xaaaa_0000);
    cpu.vfp.write_s_bits(1, 0xbbbb_c100);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFEB8_0960),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 0xaaaa_c200);

    cpu.vfp.fpscr.set_ixc(false);
    cpu.vfp.fpscr.set_rmode(RoundingMode::RoundNearest);
    cpu.vfp.write_s_bits(0, 0xaaaa_0000);
    cpu.vfp.write_s_bits(1, 0xbbbb_4080);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEB7_0960),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 0xaaaa_4000);
    assert!(cpu.vfp.fpscr.ixc());

    cpu.vfp.write_d(1, 3.5);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFEB8_0B41),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d(0), 4.0);
}

#[test]
fn neon_vrintx_vrintz_round_f32_and_f16_lanes() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);
    let pack_f32x2 =
        |low: f32, high: f32| u64::from(high.to_bits()) << 32 | u64::from(low.to_bits());

    assert_eq!(
        Aarch32Decoder::decode(0xF3BA_0482).unwrap().mnemonic,
        Mnemonic::VRINTX_F32
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3BA_0582).unwrap().mnemonic,
        Mnemonic::VRINTZ_F32
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B6_0482).unwrap().mnemonic,
        Mnemonic::VRINTX_F16
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3BA_0402).unwrap().mnemonic,
        Mnemonic::VRINTP_F32
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3BA_0502).unwrap().mnemonic,
        Mnemonic::VRINTN_F32
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3BA_0682).unwrap().mnemonic,
        Mnemonic::VRINTZ_F32
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3BA_0782).unwrap().mnemonic,
        Mnemonic::VRINTM_F32
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B2_0482).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3BA_15C2).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.vfp.write_d_bits(2, pack_f32x2(1.75, -1.75));
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3BA_0482),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), pack_f32x2(2.0, -2.0));

    cpu.vfp.write_d_bits(2, pack_f32x2(1.25, -1.25));
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3BA_0402),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), pack_f32x2(2.0, -1.0));

    cpu.vfp.write_d_bits(2, pack_f32x2(1.5, 2.5));
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3BA_0502),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), pack_f32x2(2.0, 2.0));

    cpu.vfp.write_d_bits(2, pack_f32x2(1.75, -1.75));
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3BA_0682),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), pack_f32x2(1.0, -1.0));

    cpu.vfp.write_d_bits(2, pack_f32x2(1.25, -1.25));
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3BA_0782),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), pack_f32x2(1.0, -2.0));

    cpu.vfp.write_d_bits(2, pack_f32x2(1.75, -1.75));
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3BA_0582),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), pack_f32x2(1.0, -1.0));

    cpu.vfp.write_d_bits(2, 0xc060_4060_bf00_3f00);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B6_0482),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0xc000_4000_c000_4000);

    cpu.vfp.write_d_bits(2, pack_f32x2(1.25, -1.25));
    cpu.vfp.write_d_bits(3, pack_f32x2(2.5, -2.5));
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3BA_05C2),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), pack_f32x2(1.0, -1.0));
    assert_eq!(cpu.vfp.read_d_bits(1), pack_f32x2(2.0, -2.0));
}

#[test]
fn neon_directed_float_to_integer_conversions_round_f32_and_f16_lanes() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);
    let pack_f32x2 =
        |low: f32, high: f32| u64::from(high.to_bits()) << 32 | u64::from(low.to_bits());

    assert_eq!(
        Aarch32Decoder::decode(0xF3BB_0002).unwrap().mnemonic,
        Mnemonic::VCVTN_S32_F32
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3BB_0102).unwrap().mnemonic,
        Mnemonic::VCVTP_S32_F32
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3BB_0202).unwrap().mnemonic,
        Mnemonic::VCVTM_S32_F32
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3BB_0302).unwrap().mnemonic,
        Mnemonic::VCVT_S32_F32
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3BB_0082).unwrap().mnemonic,
        Mnemonic::VCVTN_U32_F32
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B7_0002).unwrap().mnemonic,
        Mnemonic::VCVTN_S32_F16
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B3_0002).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3BB_10C2).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.vfp.write_d_bits(2, pack_f32x2(1.5, -2.5));
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3BB_0002),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0xffff_fffe_0000_0002);

    cpu.vfp.write_d_bits(2, pack_f32x2(1.25, -2.25));
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3BB_0102),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0xffff_fffe_0000_0002);

    cpu.vfp.write_d_bits(2, pack_f32x2(1.25, -2.25));
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3BB_0202),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0xffff_fffd_0000_0001);

    cpu.vfp.write_d_bits(2, pack_f32x2(1.75, -2.75));
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3BB_0302),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0xffff_fffe_0000_0001);

    cpu.vfp.fpscr.set_ioc(false);
    cpu.vfp.write_d_bits(2, pack_f32x2(1.5, -2.5));
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3BB_0082),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0000_0000_0000_0002);
    assert!(cpu.vfp.fpscr.ioc());

    cpu.vfp.write_d_bits(2, 0xbe00_4100_c100_3e00);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B7_0002),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0xfffe_0002_fffe_0002);

    cpu.vfp.fpscr.set_ioc(false);
    cpu.vfp.write_d_bits(2, 0x4300_4100_c100_3e00);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B7_0082),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0004_0002_0000_0002);
    assert!(cpu.vfp.fpscr.ioc());

    cpu.vfp.write_d_bits(2, pack_f32x2(1.5, -2.5));
    cpu.vfp.write_d_bits(3, pack_f32x2(3.5, -4.5));
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3BB_0042),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0xffff_fffe_0000_0002);
    assert_eq!(cpu.vfp.read_d_bits(1), 0xffff_fffc_0000_0004);
}

#[test]
fn scalar_half_precision_conversions_decode_and_preserve_halves() {
    let cases = [
        (0xEEB2_0A60, Mnemonic::VCVTB_F32_F16),
        (0xEEB2_0AE0, Mnemonic::VCVTT_F32_F16),
        (0xEEB3_0A60, Mnemonic::VCVTB_F16_F32),
        (0xEEB3_0AE0, Mnemonic::VCVTT_F16_F32),
    ];
    for (raw, mnemonic) in cases {
        assert_eq!(Aarch32Decoder::decode(raw).unwrap().mnemonic, mnemonic);
    }

    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    cpu.vfp.write_s_bits(1, 0xc000_3e00);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEB2_0A60),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 1.5f32.to_bits());

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEB2_0AE0),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), (-2.0f32).to_bits());

    cpu.vfp.write_s(1, 1.5);
    cpu.vfp.write_s_bits(0, 0xaaaa_bbbb);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEB3_0A60),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 0xaaaa_3e00);

    cpu.vfp.write_s(1, -2.0);
    cpu.vfp.write_s_bits(0, 0x1111_2222);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEB3_0AE0),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 0xc000_2222);
}

#[test]
fn scalar_fixed_point_conversions_decode_and_execute() {
    let cases = [
        (0xEEBA_0ACE, Mnemonic::VCVT_F32_S32_FIXED),
        (0xEEBB_0ACE, Mnemonic::VCVT_F32_U32_FIXED),
        (0xEEBE_0ACE, Mnemonic::VCVT_S32_F32_FIXED),
        (0xEEBF_0ACE, Mnemonic::VCVT_U32_F32_FIXED),
        (0xEEBA_0BCC, Mnemonic::VCVT_F64_S32_FIXED),
        (0xEEBB_0BCC, Mnemonic::VCVT_F64_U32_FIXED),
        (0xEEBE_0BCC, Mnemonic::VCVT_S32_F64_FIXED),
        (0xEEBF_0BCC, Mnemonic::VCVT_U32_F64_FIXED),
    ];
    for (raw, mnemonic) in cases {
        let insn = Aarch32Decoder::decode(raw).unwrap();
        assert_eq!(insn.mnemonic, mnemonic);
    }

    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    cpu.vfp.write_s_bits(0, (-24i32) as u32);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEBA_0ACE),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), (-1.5f32).to_bits());

    cpu.vfp.write_s_bits(0, 24);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEBB_0ACE),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 1.5f32.to_bits());

    cpu.vfp.write_s(0, -1.75);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEBE_0ACE),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), (-28i32) as u32);

    cpu.vfp.write_s(0, 1.75);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEBF_0ACE),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 28);

    cpu.vfp.write_d_bits(0, 0xffff_ffff_ffff_ff00);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEBA_0BCC),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), (-1.0f64).to_bits());

    cpu.vfp.write_d_bits(0, 0xaaaa_5555_0000_0300);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEBB_0BCC),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 3.0f64.to_bits());

    cpu.vfp.write_d(0, -1.5);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEBE_0BCC),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0) as u32, (-384i32) as u32);

    cpu.vfp.write_d(0, 2.25);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEBF_0BCC),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0) as u32, 576);
}

#[test]
fn vldr_vstr_store_and_load_double_little_endian() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    cpu.regs[1] = 0x100;
    cpu.vfp.write_d_bits(0, 0x0123_4567_89ab_cdef);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xED01_0B00),
        ExecResult::Continue
    ));
    assert_eq!(mem.read_word(0x100).unwrap(), 0x89ab_cdef);
    assert_eq!(mem.read_word(0x104).unwrap(), 0x0123_4567);

    cpu.vfp.write_d_bits(0, 0);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xED11_0B00),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0123_4567_89ab_cdef);
}

#[test]
fn vldr_vstr_halfword_use_fp16_offsets_and_zero_extend_loads() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xED81_0903).unwrap().mnemonic,
        Mnemonic::VSTR
    );
    assert_eq!(
        Aarch32Decoder::decode(0xED91_0903).unwrap().mnemonic,
        Mnemonic::VLDR
    );

    cpu.regs[1] = 0x100;
    cpu.vfp.write_s_bits(0, 0xaaaa_3c00);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xED81_0903),
        ExecResult::Continue
    ));
    assert_eq!(mem.read_halfword(0x106).unwrap(), 0x3c00);
    assert_eq!(mem.read_halfword(0x108).unwrap(), 0);

    mem.write_halfword(0x106, 0xc100).unwrap();
    cpu.vfp.write_s_bits(0, 0xffff_ffff);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xED91_0903),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_s_bits(0), 0x0000_c100);
}

#[test]
fn neon_logical_register_ops_cover_d_and_q_forms() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF201_0112).unwrap().mnemonic,
        Mnemonic::VAND
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF218_611A).unwrap().mnemonic,
        Mnemonic::VBIC
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF224_3115).unwrap().mnemonic,
        Mnemonic::VORR
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF302_0154).unwrap().mnemonic,
        Mnemonic::VEOR
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF202_1154).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.vfp.write_d_bits(1, 0xffff_0000_f0f0_0f0f);
    cpu.vfp.write_d_bits(2, 0x0f0f_f0f0_ffff_0000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF201_0112),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0f0f_0000_f0f0_0000);

    cpu.vfp.write_d_bits(8, 0xffff_00ff_aa55_55aa);
    cpu.vfp.write_d_bits(10, 0x00ff_00ff_0f0f_f0f0);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF218_611A),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(6), 0xff00_0000_a050_050a);

    cpu.vfp.write_d_bits(4, 0x0000_1111_2222_3333);
    cpu.vfp.write_d_bits(5, 0xaaaa_0000_5555_0000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF224_3115),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(3), 0xaaaa_1111_7777_3333);

    cpu.vfp.write_d_bits(2, 0x0123_4567_89ab_cdef);
    cpu.vfp.write_d_bits(3, 0xfedc_ba98_7654_3210);
    cpu.vfp.write_d_bits(4, 0xffff_0000_ffff_0000);
    cpu.vfp.write_d_bits(5, 0x0000_ffff_0000_ffff);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF302_0154),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0xfedc_4567_7654_cdef);
    assert_eq!(cpu.vfp.read_d_bits(1), 0xfedc_4567_7654_cdef);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF202_1154),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_logical_register_extra_ops_use_destination_bits_correctly() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF238_611A).unwrap().mnemonic,
        Mnemonic::VORN
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF312_0114).unwrap().mnemonic,
        Mnemonic::VBSL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF322_0114).unwrap().mnemonic,
        Mnemonic::VBIT
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF332_0114).unwrap().mnemonic,
        Mnemonic::VBIF
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF312_1154).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.vfp.write_d_bits(8, 0x0000_ffff_0000_ffff);
    cpu.vfp.write_d_bits(10, 0xffff_0000_00ff_00ff);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF238_611A),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(6), 0x0000_ffff_ff00_ffff);

    cpu.vfp.write_d_bits(0, 0xffff_0000_ffff_0000);
    cpu.vfp.write_d_bits(2, 0x1111_2222_3333_4444);
    cpu.vfp.write_d_bits(4, 0xaaaa_bbbb_cccc_dddd);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF312_0114),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x1111_bbbb_3333_dddd);

    cpu.vfp.write_d_bits(0, 0xaaaa_bbbb_cccc_dddd);
    cpu.vfp.write_d_bits(2, 0x1111_2222_3333_4444);
    cpu.vfp.write_d_bits(4, 0xffff_ffff_0000_0000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF322_0114),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x1111_2222_cccc_dddd);

    cpu.vfp.write_d_bits(0, 0xaaaa_bbbb_cccc_dddd);
    cpu.vfp.write_d_bits(2, 0x1111_2222_3333_4444);
    cpu.vfp.write_d_bits(4, 0xffff_ffff_0000_0000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF332_0114),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0xaaaa_bbbb_3333_4444);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF312_1154),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_modified_immediate_ops_materialize_and_merge_constants() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF387_001F).unwrap().mnemonic,
        Mnemonic::VMOV
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF387_265F).unwrap().mnemonic,
        Mnemonic::VMOV
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF387_291F).unwrap().mnemonic,
        Mnemonic::VORR
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF387_373F).unwrap().mnemonic,
        Mnemonic::VBIC
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF387_503F).unwrap().mnemonic,
        Mnemonic::VMVN
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF281_0C12).unwrap().mnemonic,
        Mnemonic::VMOV
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF281_1D12).unwrap().mnemonic,
        Mnemonic::VMOV
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF281_4C32).unwrap().mnemonic,
        Mnemonic::VMVN
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF280_0E31).unwrap().mnemonic,
        Mnemonic::VMOV
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF287_0F10).unwrap().mnemonic,
        Mnemonic::VMOV
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF380_2F50).unwrap().mnemonic,
        Mnemonic::VMOV
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF387_365F).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF387_001F),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0000_00ff_0000_00ff);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF387_265F),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(2), 0xff00_0000_ff00_0000);
    assert_eq!(cpu.vfp.read_d_bits(3), 0xff00_0000_ff00_0000);

    cpu.vfp.write_d_bits(2, 0x1234_0000_abcd_0000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF387_291F),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(2), 0x12ff_00ff_abff_00ff);

    cpu.vfp.write_d_bits(3, 0xffff_ffff_ffff_ffff);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF387_373F),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(3), 0x00ff_ffff_00ff_ffff);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF387_503F),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(5), 0xffff_ff00_ffff_ff00);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF281_0C12),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0000_12ff_0000_12ff);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF281_1D12),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(1), 0x0012_ffff_0012_ffff);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF281_4C32),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(4), 0xffff_ed00_ffff_ed00);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF280_0E31),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0000_0000_0000_00ff);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF287_0F10),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x3f80_0000_3f80_0000);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF380_2F50),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(2), 0xc000_0000_c000_0000);
    assert_eq!(cpu.vfp.read_d_bits(3), 0xc000_0000_c000_0000);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF387_365F),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_vmvn_register_inverts_d_and_q_registers() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF3B0_0582).unwrap().mnemonic,
        Mnemonic::VMVN
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B4_05C2).unwrap().mnemonic,
        Mnemonic::VMVN
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B0_75A2).unwrap().mnemonic,
        Mnemonic::VMVN
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B0_15C2).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.vfp.write_d_bits(2, 0x00ff_55aa_f0f0_0f0f);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B0_0582),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0xff00_aa55_0f0f_f0f0);

    cpu.vfp.write_d_bits(2, 0x0123_4567_89ab_cdef);
    cpu.vfp.write_d_bits(3, 0xfedc_ba98_7654_3210);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B4_05C2),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0xfedc_ba98_7654_3210);
    assert_eq!(cpu.vfp.read_d_bits(1), 0x0123_4567_89ab_cdef);

    cpu.vfp.write_d_bits(18, 0xaaaa_0000_5555_ffff);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B0_75A2),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(7), 0x5555_ffff_aaaa_0000);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B0_15C2),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_vcls_register_counts_leading_sign_bits_by_lane() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF3B0_0402).unwrap().mnemonic,
        Mnemonic::VCLS
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B4_1403).unwrap().mnemonic,
        Mnemonic::VCLS
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B8_0442).unwrap().mnemonic,
        Mnemonic::VCLS
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B0_7422).unwrap().mnemonic,
        Mnemonic::VCLS
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3BC_0402).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B0_1442).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.vfp.write_d_bits(2, 0x807f_ff00_40c0_01fe);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B0_0402),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0000_0707_0001_0606);

    cpu.vfp.write_d_bits(3, 0x8000_7fff_ffff_0001);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B4_1403),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(1), 0x0000_0000_000f_000e);

    cpu.vfp.write_d_bits(2, 0x8000_0000_7fff_ffff);
    cpu.vfp.write_d_bits(3, 0xffff_ffff_0000_0001);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B8_0442),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0000_0000_0000_0000);
    assert_eq!(cpu.vfp.read_d_bits(1), 0x0000_001f_0000_001e);

    cpu.vfp.write_d_bits(18, 0x00ff_807f_01fe_c040);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B0_7422),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(7), 0x0707_0000_0606_0100);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3BC_0402),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_vrev_register_reverses_elements_within_containers() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF3B0_0002).unwrap().mnemonic,
        Mnemonic::VREV64
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B4_1003).unwrap().mnemonic,
        Mnemonic::VREV64
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B0_4085).unwrap().mnemonic,
        Mnemonic::VREV32
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B0_7122).unwrap().mnemonic,
        Mnemonic::VREV16
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B4_0102).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B8_0082).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3BC_0002).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B0_1042).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.vfp.write_d_bits(2, 0x0123_4567_89ab_cdef);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B0_0002),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0xefcd_ab89_6745_2301);

    cpu.vfp.write_d_bits(3, 0x0123_4567_89ab_cdef);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B4_1003),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(1), 0xcdef_89ab_4567_0123);

    cpu.vfp.write_d_bits(2, 0x0123_4567_89ab_cdef);
    cpu.vfp.write_d_bits(3, 0xfedc_ba98_7654_3210);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B8_0042),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x89ab_cdef_0123_4567);
    assert_eq!(cpu.vfp.read_d_bits(1), 0x7654_3210_fedc_ba98);

    cpu.vfp.write_d_bits(5, 0x0123_4567_89ab_cdef);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B0_4085),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(4), 0x6745_2301_efcd_ab89);

    cpu.vfp.write_d_bits(8, 0x0123_4567_89ab_cdef);
    cpu.vfp.write_d_bits(9, 0xfedc_ba98_7654_3210);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B4_40C8),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(4), 0x4567_0123_cdef_89ab);
    assert_eq!(cpu.vfp.read_d_bits(5), 0xba98_fedc_3210_7654);

    cpu.vfp.write_d_bits(18, 0x0123_4567_89ab_cdef);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B0_7122),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(7), 0x2301_6745_ab89_efcd);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B4_0102),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_vswp_swaps_d_and_q_registers() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF3B2_0002).unwrap().mnemonic,
        Mnemonic::VSWP
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B2_0042).unwrap().mnemonic,
        Mnemonic::VSWP
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B2_7022).unwrap().mnemonic,
        Mnemonic::VSWP
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B2_1042).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B6_0002).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.vfp.write_d_bits(0, 0x1111_2222_3333_4444);
    cpu.vfp.write_d_bits(2, 0xaaaa_bbbb_cccc_dddd);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B2_0002),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0xaaaa_bbbb_cccc_dddd);
    assert_eq!(cpu.vfp.read_d_bits(2), 0x1111_2222_3333_4444);

    cpu.vfp.write_d_bits(0, 0x0000_0000_0000_0001);
    cpu.vfp.write_d_bits(1, 0x0000_0000_0000_0002);
    cpu.vfp.write_d_bits(2, 0x0000_0000_0000_0003);
    cpu.vfp.write_d_bits(3, 0x0000_0000_0000_0004);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B2_0042),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0000_0000_0000_0003);
    assert_eq!(cpu.vfp.read_d_bits(1), 0x0000_0000_0000_0004);
    assert_eq!(cpu.vfp.read_d_bits(2), 0x0000_0000_0000_0001);
    assert_eq!(cpu.vfp.read_d_bits(3), 0x0000_0000_0000_0002);

    cpu.vfp.write_d_bits(7, 0x7777_7777_7777_7777);
    cpu.vfp.write_d_bits(18, 0x1818_1818_1818_1818);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B2_7022),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(7), 0x1818_1818_1818_1818);
    assert_eq!(cpu.vfp.read_d_bits(18), 0x7777_7777_7777_7777);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B2_1042),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_pairwise_permute_transposes_unzips_and_zips_vectors() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF3B2_0082).unwrap().mnemonic,
        Mnemonic::VTRN
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B2_0102).unwrap().mnemonic,
        Mnemonic::VUZP
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B2_0182).unwrap().mnemonic,
        Mnemonic::VZIP
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3BA_0102).unwrap().mnemonic,
        Mnemonic::VTRN
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3BA_0182).unwrap().mnemonic,
        Mnemonic::VTRN
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3BE_0082).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B2_10C2).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.vfp.write_d_bits(0, 0x0706_0504_0302_0100);
    cpu.vfp.write_d_bits(2, 0x1716_1514_1312_1110);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B2_0082),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x1606_1404_1202_1000);
    assert_eq!(cpu.vfp.read_d_bits(2), 0x1707_1505_1303_1101);

    cpu.vfp.write_d_bits(0, 0x0706_0504_0302_0100);
    cpu.vfp.write_d_bits(2, 0x1716_1514_1312_1110);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B2_0102),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x1614_1210_0604_0200);
    assert_eq!(cpu.vfp.read_d_bits(2), 0x1715_1311_0705_0301);

    cpu.vfp.write_d_bits(0, 0x0706_0504_0302_0100);
    cpu.vfp.write_d_bits(2, 0x1716_1514_1312_1110);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B2_0182),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x1303_1202_1101_1000);
    assert_eq!(cpu.vfp.read_d_bits(2), 0x1707_1606_1505_1404);

    cpu.vfp.write_d_bits(0, 0x0706_0504_0302_0100);
    cpu.vfp.write_d_bits(1, 0x0f0e_0d0c_0b0a_0908);
    cpu.vfp.write_d_bits(2, 0x2726_2524_2322_2120);
    cpu.vfp.write_d_bits(3, 0x2f2e_2d2c_2b2a_2928);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B6_0142),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0d0c_0908_0504_0100);
    assert_eq!(cpu.vfp.read_d_bits(1), 0x2d2c_2928_2524_2120);
    assert_eq!(cpu.vfp.read_d_bits(2), 0x0f0e_0b0a_0706_0302);
    assert_eq!(cpu.vfp.read_d_bits(3), 0x2f2e_2b2a_2726_2322);

    cpu.vfp.write_d_bits(0, 0x0706_0504_0302_0100);
    cpu.vfp.write_d_bits(1, 0x0f0e_0d0c_0b0a_0908);
    cpu.vfp.write_d_bits(2, 0x2726_2524_2322_2120);
    cpu.vfp.write_d_bits(3, 0x2f2e_2d2c_2b2a_2928);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3BA_01C2),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x2322_2120_0302_0100);
    assert_eq!(cpu.vfp.read_d_bits(1), 0x2726_2524_0706_0504);
    assert_eq!(cpu.vfp.read_d_bits(2), 0x2b2a_2928_0b0a_0908);
    assert_eq!(cpu.vfp.read_d_bits(3), 0x2f2e_2d2c_0f0e_0d0c);

    cpu.vfp.write_d_bits(0, 0x0706_0504_0302_0100);
    cpu.vfp.write_d_bits(2, 0x2726_2524_2322_2120);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3BA_0102),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x2322_2120_0302_0100);
    assert_eq!(cpu.vfp.read_d_bits(2), 0x2726_2524_0706_0504);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3BE_0082),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_vqabs_vqneg_saturate_signed_lanes_and_set_qc() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF3B0_0702).unwrap().mnemonic,
        Mnemonic::VQABS
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B4_1703).unwrap().mnemonic,
        Mnemonic::VQABS
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B8_0742).unwrap().mnemonic,
        Mnemonic::VQABS
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B0_0782).unwrap().mnemonic,
        Mnemonic::VQNEG
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B4_07C2).unwrap().mnemonic,
        Mnemonic::VQNEG
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3BC_0702).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B0_17C2).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.vfp.write_d_bits(2, 0xfe7f_0201_00ff_8180);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B0_0702),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x027f_0201_0001_7f7f);
    assert!(cpu.vfp.fpscr.qc());

    cpu.vfp.fpscr.set_qc(false);
    cpu.vfp.write_d_bits(2, 0xffff_ffff_8000_0000);
    cpu.vfp.write_d_bits(3, 0x7fff_ffff_0000_0002);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B8_0742),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0000_0001_7fff_ffff);
    assert_eq!(cpu.vfp.read_d_bits(1), 0x7fff_ffff_0000_0002);
    assert!(cpu.vfp.fpscr.qc());

    cpu.vfp.fpscr.set_qc(false);
    cpu.vfp.write_d_bits(2, 0x7fff_0001_fffe_8000);
    cpu.vfp.write_d_bits(3, 0xedcc_1234_ffff_0000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B4_07C2),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x8001_ffff_0002_7fff);
    assert_eq!(cpu.vfp.read_d_bits(1), 0x1234_edcc_0001_0000);
    assert!(cpu.vfp.fpscr.qc());

    cpu.vfp.fpscr.set_qc(false);
    cpu.vfp.write_d_bits(18, 0x0000_0005_8000_0000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B8_77A2),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(7), 0xffff_fffb_7fff_ffff);
    assert!(cpu.vfp.fpscr.qc());

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3BC_0702),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_vabs_vneg_abs_and_negate_integer_and_float_lanes() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF3B1_0301).unwrap().mnemonic,
        Mnemonic::VABS
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B5_0342).unwrap().mnemonic,
        Mnemonic::VABS
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B9_2387).unwrap().mnemonic,
        Mnemonic::VNEG
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B9_8709).unwrap().mnemonic,
        Mnemonic::VABS
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B5_0701).unwrap().mnemonic,
        Mnemonic::VABS
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3F9_07E2).unwrap().mnemonic,
        Mnemonic::VNEG
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B5_0781).unwrap().mnemonic,
        Mnemonic::VNEG
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3BD_0301).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B5_0341).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.vfp.write_d_bits(1, 0x7f80_ff01_8000_0002);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B1_0301),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x7f80_0101_8000_0002);

    cpu.vfp.write_d_bits(10, 0x8000_7fff_ffff_0001);
    cpu.vfp.write_d_bits(11, 0x1234_edcc_0000_8001);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B5_83CA),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(8), 0x8000_8001_0001_ffff);
    assert_eq!(cpu.vfp.read_d_bits(9), 0xedcc_1234_0000_7fff);

    cpu.vfp.write_d_bits(
        14,
        ((0xffc1_2345u64) << 32) | u64::from((-1.5f32).to_bits()),
    );
    cpu.vfp.write_d_bits(
        15,
        ((0x8000_0000u64) << 32) | u64::from((-0.0f32).to_bits()),
    );
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B9_C74E),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(12),
        ((0x7fc1_2345u64) << 32) | u64::from(1.5f32.to_bits())
    );
    assert_eq!(
        cpu.vfp.read_d_bits(13),
        ((0x0000_0000u64) << 32) | u64::from(0.0f32.to_bits())
    );

    cpu.vfp.write_d_bits(
        18,
        ((2.0f32.to_bits() as u64) << 32) | u64::from((-3.0f32).to_bits()),
    );
    cpu.vfp
        .write_d_bits(19, ((0x7fc0_1111u64) << 32) | u64::from(0.0f32.to_bits()));
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3F9_07E2),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(16),
        (((-2.0f32).to_bits() as u64) << 32) | u64::from(3.0f32.to_bits())
    );
    assert_eq!(
        cpu.vfp.read_d_bits(17),
        ((0xffc0_1111u64) << 32) | u64::from((-0.0f32).to_bits())
    );

    cpu.vfp.write_d_bits(1, 0xfe00_bc00_8000_3c00);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B5_0701),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x7e00_3c00_0000_3c00);

    cpu.vfp.write_d_bits(1, 0x7e00_3c00_0000_8000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B5_0781),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0xfe00_bc00_8000_0000);

    cpu.vfp.write_h_bits(1, 0xbc00);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEB0_09E0),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_h_bits(0), 0x3c00);
}

#[test]
fn neon_vqadd_vqsub_saturate_integer_lanes_and_set_qc() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF202_0013).unwrap().mnemonic,
        Mnemonic::VQADD
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF312_0013).unwrap().mnemonic,
        Mnemonic::VQADD
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF302_0213).unwrap().mnemonic,
        Mnemonic::VQSUB
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF212_0254).unwrap().mnemonic,
        Mnemonic::VQSUB
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF202_1054).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.vfp.write_d_bits(2, 0xc040_f010_fe80_017f);
    cpu.vfp.write_d_bits(3, 0xc040_f020_80ff_7f01);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF202_0013),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x807f_e030_8080_7f7f);
    assert!(cpu.vfp.fpscr.qc());

    cpu.vfp.fpscr.set_qc(false);
    cpu.vfp.write_d_bits(2, 0x1234_8000_0001_ffff);
    cpu.vfp.write_d_bits(3, 0xedcc_8000_0002_0001);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF312_0013),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0xffff_ffff_0003_ffff);
    assert!(cpu.vfp.fpscr.qc());

    cpu.vfp.fpscr.set_qc(false);
    cpu.vfp.write_d_bits(2, 0x7fff_ffff_ffff_ffff);
    cpu.vfp.write_d_bits(3, 0x0000_0000_0000_0001);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF232_0013),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x7fff_ffff_ffff_ffff);
    assert!(cpu.vfp.fpscr.qc());

    cpu.vfp.fpscr.set_qc(false);
    cpu.vfp.write_d_bits(2, 0x0001_80ff_7f00_1000);
    cpu.vfp.write_d_bits(3, 0x0102_0101_8001_2001);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF302_0213),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0000_7ffe_0000_0000);
    assert!(cpu.vfp.fpscr.qc());

    cpu.vfp.fpscr.set_qc(false);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF202_1054),
        ExecResult::Undefined
    ));
    assert!(!cpu.vfp.fpscr.qc());
}

#[test]
fn neon_vqdmulh_vqrdmulh_vector_saturates_and_rounds_signed_lanes() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF212_0B03).unwrap().mnemonic,
        Mnemonic::VQDMULH
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF312_0B03).unwrap().mnemonic,
        Mnemonic::VQRDMULH
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF322_0B44).unwrap().mnemonic,
        Mnemonic::VQRDMULH
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF202_0B03).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF232_0B03).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF322_1B44).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.vfp.write_d_bits(2, 0x7fff_8000_3001_1001);
    cpu.vfp.write_d_bits(3, 0x7fff_8000_1001_3001);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF212_0B03),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x7ffe_7fff_0600_0600);
    assert!(cpu.vfp.fpscr.qc());

    cpu.vfp.fpscr.set_qc(false);
    cpu.vfp.write_d_bits(2, 0x7fff_8000_3001_1001);
    cpu.vfp.write_d_bits(3, 0x7fff_8000_1001_3001);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF312_0B03),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x7ffe_7fff_0601_0601);
    assert!(cpu.vfp.fpscr.qc());

    cpu.vfp.fpscr.set_qc(false);
    cpu.vfp.write_d_bits(2, 0xc000_0000_4000_0000);
    cpu.vfp.write_d_bits(3, 0x8000_0000_7fff_ffff);
    cpu.vfp.write_d_bits(4, 0x4000_0000_4000_0000);
    cpu.vfp.write_d_bits(5, 0x8000_0000_7fff_ffff);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF322_0B44),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0xe000_0000_2000_0000);
    assert_eq!(cpu.vfp.read_d_bits(1), 0x7fff_ffff_7fff_fffe);
    assert!(cpu.vfp.fpscr.qc());

    cpu.vfp.fpscr.set_qc(false);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF202_0B03),
        ExecResult::Undefined
    ));
    assert!(!cpu.vfp.fpscr.qc());
}

#[test]
fn neon_vqdmulh_vqrdmulh_scalar_by_lane_uses_indexed_operand() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF292_0C63).unwrap().mnemonic,
        Mnemonic::VQDMULH
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF292_0D63).unwrap().mnemonic,
        Mnemonic::VQRDMULH
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3A2_0D64).unwrap().mnemonic,
        Mnemonic::VQRDMULH
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF282_0C43).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF2B2_0C43).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3A2_1D64).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.vfp.fpscr.set_qc(false);
    cpu.vfp.write_d_bits(2, 0x7fff_8000_3001_1001);
    cpu.vfp.write_d_bits(3, 0x4444_3001_2222_1111);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF292_0C63),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x3000_cfff_1200_0600);
    assert!(!cpu.vfp.fpscr.qc());

    cpu.vfp.write_d_bits(2, 0x7fff_8000_3001_1001);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF292_0D63),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x3001_cfff_1201_0601);
    assert!(!cpu.vfp.fpscr.qc());

    cpu.vfp.write_d_bits(2, 0xc000_0000_4000_0000);
    cpu.vfp.write_d_bits(3, 0x8000_0000_7fff_ffff);
    cpu.vfp.write_d_bits(4, 0x8000_0000_1234_5678);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3A2_0D64),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x4000_0000_c000_0000);
    assert_eq!(cpu.vfp.read_d_bits(1), 0x7fff_ffff_8000_0001);
    assert!(cpu.vfp.fpscr.qc());

    cpu.vfp.fpscr.set_qc(false);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF282_0C43),
        ExecResult::Undefined
    ));
    assert!(!cpu.vfp.fpscr.qc());
}

#[test]
fn neon_vdup_register_and_scalar_duplicate_lanes() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xEEC0_2B10).unwrap().mnemonic,
        Mnemonic::VDUP
    );
    assert_eq!(
        Aarch32Decoder::decode(0xEEA4_3B30).unwrap().mnemonic,
        Mnemonic::VDUP
    );
    assert_eq!(
        Aarch32Decoder::decode(0xEE81_4B90).unwrap().mnemonic,
        Mnemonic::VDUP
    );
    assert_eq!(
        Aarch32Decoder::decode(0xEEC0_2B30).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    assert_eq!(
        Aarch32Decoder::decode(0xEEA1_2B10).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    assert_eq!(
        Aarch32Decoder::decode(0xEEC0_FB10).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.regs[2] = 0x1122_3344;
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEC0_2B10),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x4444_4444_4444_4444);

    cpu.regs[3] = 0x8877_6655;
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEA4_3B30),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(4), 0x6655_6655_6655_6655);
    assert_eq!(cpu.vfp.read_d_bits(5), 0x6655_6655_6655_6655);

    cpu.regs[4] = 0x89ab_cdef;
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEE81_4B90),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(17), 0x89ab_cdef_89ab_cdef);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEC0_2B30),
        ExecResult::Undefined
    ));

    assert_eq!(
        Aarch32Decoder::decode(0xF3B5_0C03).unwrap().mnemonic,
        Mnemonic::VDUP
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3BE_4C47).unwrap().mnemonic,
        Mnemonic::VDUP
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3FC_2C05).unwrap().mnemonic,
        Mnemonic::VDUP
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B0_0C00).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B8_0C00).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B5_1C43).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.vfp.write_d_bits(3, 0x8877_6655_4433_2211);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B5_0C03),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x3333_3333_3333_3333);

    cpu.vfp.write_d_bits(7, 0x7788_5566_3344_1122);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3BE_4C47),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(4), 0x7788_7788_7788_7788);
    assert_eq!(cpu.vfp.read_d_bits(5), 0x7788_7788_7788_7788);

    cpu.vfp.write_d_bits(5, 0xfeed_face_1234_5678);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3FC_2C05),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(18), 0xfeed_face_feed_face);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B0_0C00),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_vclz_register_counts_leading_zeros_by_lane() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF3B0_0482).unwrap().mnemonic,
        Mnemonic::VCLZ
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B4_1483).unwrap().mnemonic,
        Mnemonic::VCLZ
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B8_04C2).unwrap().mnemonic,
        Mnemonic::VCLZ
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B0_74A2).unwrap().mnemonic,
        Mnemonic::VCLZ
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3BC_0482).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B0_14C2).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.vfp.write_d_bits(2, 0x8000_0001_00f0_0f00);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B0_0482),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0008_0807_0800_0408);

    cpu.vfp.write_d_bits(3, 0x0001_8000_00ff_0f00);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B4_1483),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(1), 0x000f_0000_0008_0004);

    cpu.vfp.write_d_bits(2, 0x0000_0001_8000_0000);
    cpu.vfp.write_d_bits(3, 0x0000_0000_00f0_8000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B8_04C2),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0000_001f_0000_0000);
    assert_eq!(cpu.vfp.read_d_bits(1), 0x0000_0020_0000_0008);

    cpu.vfp.write_d_bits(18, 0x0000_0000_00f0_8000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B0_74A2),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(7), 0x0808_0808_0800_0008);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3BC_0482),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_vcnt_register_counts_bits_in_byte_lanes() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF3B0_0502).unwrap().mnemonic,
        Mnemonic::VCNT
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B0_0542).unwrap().mnemonic,
        Mnemonic::VCNT
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B4_0502).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B0_1542).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.vfp.write_d_bits(2, 0xff00_550f_8001_7e81);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B0_0502),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0800_0404_0101_0602);

    cpu.vfp.write_d_bits(2, 0x0123_4567_89ab_cdef);
    cpu.vfp.write_d_bits(3, 0xf0f0_0000_ffff_1357);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B0_0542),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0103_0305_0305_0507);
    assert_eq!(cpu.vfp.read_d_bits(1), 0x0404_0000_0808_0305);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B4_0502),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_vext_extracts_bytes_from_d_and_q_register_pairs() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF2B2_0304).unwrap().mnemonic,
        Mnemonic::VEXT
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF2B2_0944).unwrap().mnemonic,
        Mnemonic::VEXT
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF2B2_0804).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF2B2_1044).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.vfp.write_d_bits(2, 0x0807_0605_0403_0201);
    cpu.vfp.write_d_bits(4, 0x1817_1615_1413_1211);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF2B2_0304),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x1312_1108_0706_0504);

    cpu.vfp.write_d_bits(2, 0x0706_0504_0302_0100);
    cpu.vfp.write_d_bits(3, 0x0f0e_0d0c_0b0a_0908);
    cpu.vfp.write_d_bits(4, 0x1716_1514_1312_1110);
    cpu.vfp.write_d_bits(5, 0x1f1e_1d1c_1b1a_1918);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF2B2_0944),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x100f_0e0d_0c0b_0a09);
    assert_eq!(cpu.vfp.read_d_bits(1), 0x1817_1615_1413_1211);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF2B2_0804),
        ExecResult::Undefined
    ));
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF2B2_1044),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_vtbl_vtbx_lookup_bytes_from_consecutive_d_tables() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF3B0_4907).unwrap().mnemonic,
        Mnemonic::VTBL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B2_5848).unwrap().mnemonic,
        Mnemonic::VTBX
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3BF_0980).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.vfp.write_d_bits(0, 0x0706_0504_0302_0100);
    cpu.vfp.write_d_bits(1, 0x1716_1514_1312_1110);
    cpu.vfp.write_d_bits(7, 0x0c03_ff10_0f08_0700);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B0_4907),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(4), 0x1403_0000_1710_0700);

    cpu.vfp.write_d_bits(2, 0x8786_8584_8382_8180);
    cpu.vfp.write_d_bits(5, 0x2211_ffee_ddcc_bbaa);
    cpu.vfp.write_d_bits(8, 0x0906_04ff_0807_0100);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B2_5848),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(5), 0x2286_84ee_dd87_8180);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3BF_0980),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_fp_vmax_vmin_handle_f32_f16_and_q_lanes() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF202_0F04).unwrap().mnemonic,
        Mnemonic::VMAX
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF222_0F04).unwrap().mnemonic,
        Mnemonic::VMIN
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF218_6F0A).unwrap().mnemonic,
        Mnemonic::VMAX
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF202_1F44).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.vfp.write_d_bits(
        2,
        ((-0.0f32).to_bits() as u64) << 32 | 1.0f32.to_bits() as u64,
    );
    cpu.vfp
        .write_d_bits(4, (0.0f32.to_bits() as u64) << 32 | 2.0f32.to_bits() as u64);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF202_0F04),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(0),
        (0.0f32.to_bits() as u64) << 32 | 2.0f32.to_bits() as u64
    );

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF222_0F04),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(0),
        ((-0.0f32).to_bits() as u64) << 32 | 1.0f32.to_bits() as u64
    );

    cpu.vfp.write_d_bits(8, 0x7e00_c200_3c00_4000);
    cpu.vfp.write_d_bits(10, 0x4400_3c00_c000_4200);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF218_6F0A),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(6), 0x7e00_3c00_3c00_4200);

    cpu.vfp
        .write_d_bits(2, (3.0f32.to_bits() as u64) << 32 | 1.0f32.to_bits() as u64);
    cpu.vfp
        .write_d_bits(3, (5.0f32.to_bits() as u64) << 32 | 7.0f32.to_bits() as u64);
    cpu.vfp
        .write_d_bits(4, (4.0f32.to_bits() as u64) << 32 | 2.0f32.to_bits() as u64);
    cpu.vfp
        .write_d_bits(5, (6.0f32.to_bits() as u64) << 32 | 8.0f32.to_bits() as u64);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF202_0F44),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(0),
        (4.0f32.to_bits() as u64) << 32 | 2.0f32.to_bits() as u64
    );
    assert_eq!(
        cpu.vfp.read_d_bits(1),
        (6.0f32.to_bits() as u64) << 32 | 8.0f32.to_bits() as u64
    );

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF202_1F44),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_fp_vabd_computes_absolute_difference_for_f32_f16_and_q_lanes() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF322_0D04).unwrap().mnemonic,
        Mnemonic::VABD
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF338_6D0A).unwrap().mnemonic,
        Mnemonic::VABD
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF322_0D44).unwrap().mnemonic,
        Mnemonic::VABD
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF322_1D44).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.vfp.write_d_bits(
        2,
        ((-4.0f32).to_bits() as u64) << 32 | 1.5f32.to_bits() as u64,
    );
    cpu.vfp
        .write_d_bits(4, (1.0f32.to_bits() as u64) << 32 | 3.5f32.to_bits() as u64);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF322_0D04),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(0),
        (5.0f32.to_bits() as u64) << 32 | 2.0f32.to_bits() as u64
    );

    cpu.vfp.write_d_bits(8, 0x7e00_3c00_c200_4000);
    cpu.vfp.write_d_bits(10, 0x4400_c000_4200_3c00);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF338_6D0A),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(6), 0x7e00_4200_4600_3c00);

    cpu.vfp
        .write_d_bits(2, (7.0f32.to_bits() as u64) << 32 | 1.0f32.to_bits() as u64);
    cpu.vfp
        .write_d_bits(3, (5.0f32.to_bits() as u64) << 32 | 3.0f32.to_bits() as u64);
    cpu.vfp
        .write_d_bits(4, (8.0f32.to_bits() as u64) << 32 | 2.0f32.to_bits() as u64);
    cpu.vfp
        .write_d_bits(5, (9.0f32.to_bits() as u64) << 32 | 6.0f32.to_bits() as u64);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF322_0D44),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(0),
        (1.0f32.to_bits() as u64) << 32 | 1.0f32.to_bits() as u64
    );
    assert_eq!(
        cpu.vfp.read_d_bits(1),
        (4.0f32.to_bits() as u64) << 32 | 3.0f32.to_bits() as u64
    );

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF322_1D44),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_integer_vabd_vaba_cover_signed_unsigned_sizes_and_q_forms() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF201_0702).unwrap().mnemonic,
        Mnemonic::VABD
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF304_3705).unwrap().mnemonic,
        Mnemonic::VABD
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF314_3715).unwrap().mnemonic,
        Mnemonic::VABA
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF222_0754).unwrap().mnemonic,
        Mnemonic::VABA
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF222_1754).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF231_0702).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.vfp.write_d_bits(1, 0xc040_ff10_f000_7f80);
    cpu.vfp.write_d_bits(2, 0x40c0_01f0_10ff_807f);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF201_0702),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x8080_0220_2001_ffff);

    cpu.vfp.write_d_bits(4, 0xfa64_1450_ff00_c801);
    cpu.vfp.write_d_bits(5, 0x0063_dc0a_01fa_6403);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF304_3705),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(3), 0xfa01_c846_fefa_6402);

    cpu.vfp.write_d_bits(3, 0x0010_8000_0001_fffe);
    cpu.vfp.write_d_bits(4, 0xffff_0000_c350_0001);
    cpu.vfp.write_d_bits(5, 0x0001_ffff_03e8_0003);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF314_3715),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(3), 0x000e_7fff_bf69_0000);

    cpu.vfp.write_d_bits(0, 0x0000_0005_ffff_fffe);
    cpu.vfp.write_d_bits(1, 0x0000_0010_8000_0000);
    cpu.vfp.write_d_bits(2, 0x0000_0064_8000_0000);
    cpu.vfp.write_d_bits(3, 0x7fff_ffff_ffff_ffff);
    cpu.vfp.write_d_bits(4, 0xffff_ff9c_7fff_ffff);
    cpu.vfp.write_d_bits(5, 0x8000_0000_0000_0001);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF222_0754),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0000_00cd_ffff_fffd);
    assert_eq!(cpu.vfp.read_d_bits(1), 0x0000_000f_8000_0002);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF222_1754),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_integer_vabdl_vabal_widen_absolute_differences() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF282_0703).unwrap().mnemonic,
        Mnemonic::VABDL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF39A_870B).unwrap().mnemonic,
        Mnemonic::VABDL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF2E2_05A3).unwrap().mnemonic,
        Mnemonic::VABAL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3CA_85AB).unwrap().mnemonic,
        Mnemonic::VABAL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF282_1703).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.vfp.write_d_bits(2, 0xc040_f010_ff00_7f80);
    cpu.vfp.write_d_bits(3, 0x40c0_10f0_0101_807f);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF282_0703),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0002_0001_00ff_00ff);
    assert_eq!(cpu.vfp.read_d_bits(1), 0x0080_0080_0020_0020);

    cpu.vfp.write_d_bits(10, 0x0000_ffff_c350_0001);
    cpu.vfp.write_d_bits(11, 0xffff_0001_03e8_0003);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF39A_870B),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(8), 0x0000_bf68_0000_0002);
    assert_eq!(cpu.vfp.read_d_bits(9), 0x0000_ffff_0000_fffe);

    cpu.vfp.write_d_bits(16, 5);
    cpu.vfp.write_d_bits(17, 6);
    cpu.vfp.write_d_bits(18, 0x8000_0000_ffff_fffe);
    cpu.vfp.write_d_bits(19, 0x7fff_ffff_0000_0001);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF2E2_05A3),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(16), 8);
    assert_eq!(cpu.vfp.read_d_bits(17), 0x0000_0001_0000_0005);

    cpu.vfp.write_d_bits(24, 0x0028_001e_0014_000a);
    cpu.vfp.write_d_bits(25, 0x0050_0046_003c_0032);
    cpu.vfp.write_d_bits(26, 0x0807_0605_0403_0201);
    cpu.vfp.write_d_bits(27, 0x0002_0804_0a03_0101);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3CA_85AB),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(24), 0x002e_001e_0015_000a);
    assert_eq!(cpu.vfp.read_d_bits(25), 0x0058_004b_003e_0033);

    let invalid_size = DecodedInsn::new(Mnemonic::VABDL, ExecutionState::Aarch32, 0xF2B2_0703, 4);
    assert!(matches!(
        Executor::new(&mut cpu, &mut mem).execute(&invalid_size),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_integer_vmax_vmin_cover_signed_unsigned_sizes_and_q_forms() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF201_0602).unwrap().mnemonic,
        Mnemonic::VMAX
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF304_3605).unwrap().mnemonic,
        Mnemonic::VMAX
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF218_661A).unwrap().mnemonic,
        Mnemonic::VMIN
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF322_0654).unwrap().mnemonic,
        Mnemonic::VMIN
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF222_1654).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF231_0602).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.vfp.write_d_bits(1, 0xc040_ff10_f000_7f80);
    cpu.vfp.write_d_bits(2, 0x40c0_01f0_10ff_807f);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF201_0602),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x4040_0110_1000_7f7f);

    cpu.vfp.write_d_bits(4, 0xfa64_1450_ff00_c801);
    cpu.vfp.write_d_bits(5, 0x0063_dc0a_01fa_6403);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF304_3605),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(3), 0xfa64_dc50_fffa_c803);

    cpu.vfp.write_d_bits(8, 0x7fff_8000_0001_fffe);
    cpu.vfp.write_d_bits(10, 0x8000_7fff_ffff_0002);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF218_661A),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(6), 0x8000_8000_ffff_fffe);

    cpu.vfp.write_d_bits(2, 0x0000_0005_ffff_fffe);
    cpu.vfp.write_d_bits(3, 0x0000_0010_8000_0000);
    cpu.vfp.write_d_bits(4, 0x0000_0006_ffff_fffd);
    cpu.vfp.write_d_bits(5, 0x0000_0020_7fff_ffff);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF322_0654),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0000_0005_ffff_fffd);
    assert_eq!(cpu.vfp.read_d_bits(1), 0x0000_0010_7fff_ffff);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF222_1654),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_halving_add_sub_cover_signed_unsigned_rounding_and_q_forms() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF201_0002).unwrap().mnemonic,
        Mnemonic::VHADD
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF308_414C).unwrap().mnemonic,
        Mnemonic::VRHADD
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF314_3205).unwrap().mnemonic,
        Mnemonic::VHSUB
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF222_1244).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF231_0002).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.vfp.write_d_bits(1, 0xc040_ff10_f000_7f80);
    cpu.vfp.write_d_bits(2, 0x40c0_01f0_10ff_807f);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF201_0002),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0000_0000_00ff_ffff);

    cpu.vfp.write_d_bits(4, 0x9c40_ffff_0001_0000);
    cpu.vfp.write_d_bits(5, 0xc350_0001_ffff_0002);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF314_3005),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(3), 0xafc8_8000_8000_0001);

    cpu.vfp.write_d_bits(4, 0x9c40_000a_ffff_0064);
    cpu.vfp.write_d_bits(5, 0x7530_0006_0001_0002);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF314_3205),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(3), 0x1388_0002_7fff_0031);

    cpu.vfp.write_d_bits(8, 0x0706_0504_0302_0100);
    cpu.vfp.write_d_bits(9, 0x0100_fffe_fdfc_fbfa);
    cpu.vfp.write_d_bits(12, 0x0807_0605_0403_0201);
    cpu.vfp.write_d_bits(13, 0x0605_0403_0201_00ff);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF308_414C),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(4), 0x0807_0605_0403_0201);
    assert_eq!(cpu.vfp.read_d_bits(5), 0x0403_8281_807f_7efd);

    cpu.vfp.write_d_bits(2, ((-10i32 as u32 as u64) << 32) | 10);
    cpu.vfp
        .write_d_bits(3, ((i32::MAX as u32 as u64) << 32) | i32::MIN as u32 as u64);
    cpu.vfp.write_d_bits(4, (20u64 << 32) | 4);
    cpu.vfp.write_d_bits(5, ((-1i32 as u32 as u64) << 32) | 1);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF222_0244),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0xffff_fff1_0000_0003);
    assert_eq!(cpu.vfp.read_d_bits(1), 0x4000_0000_bfff_ffff);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF222_1244),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_pairwise_integer_ops_cover_add_min_max_and_invalid_shapes() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF201_0B12).unwrap().mnemonic,
        Mnemonic::VPADD
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF201_0A02).unwrap().mnemonic,
        Mnemonic::VPMAX
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF228_6A1A).unwrap().mnemonic,
        Mnemonic::VPMIN
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF201_0B52).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF231_0B12).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.vfp.write_d_bits(1, 0x01ff_8080_0afa_0201);
    cpu.vfp.write_d_bits(2, 0x02ff_64c8_281e_140a);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF201_0B12),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x012c_461e_0000_0403);

    cpu.vfp.write_d_bits(4, 0x0002_ffff_0002_0001);
    cpu.vfp.write_d_bits(5, 0x1111_1234_8000_8000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF214_3B15),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(3), 0x2345_0000_0001_0003);

    cpu.vfp.write_d_bits(1, 0xc040_f010_ff00_7f80);
    cpu.vfp.write_d_bits(2, 0x9d9c_6564_8180_ff01);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF201_0A02),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x9d65_8101_4010_007f);

    cpu.vfp.write_d_bits(4, 0x7fff_8000_ffff_0001);
    cpu.vfp.write_d_bits(5, 0x0001_abcd_5678_1234);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF314_3A05),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(3), 0xabcd_5678_8000_ffff);

    cpu.vfp.write_d_bits(8, ((-20i32 as u32 as u64) << 32) | 10);
    cpu.vfp.write_d_bits(
        10,
        ((i32::MAX as u32 as u64) << 32) | i32::MIN as u32 as u64,
    );
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF228_6A1A),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(6), 0x8000_0000_ffff_ffec);

    cpu.vfp.write_d_bits(12, 0x80ff_0001_0203_0405);
    cpu.vfp.write_d_bits(13, 0x463c_3228_011e_140a);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF30C_BA1D),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(11), 0x3c28_010a_8000_0204);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF201_0B52),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_fp_pairwise_add_handles_f32_and_f16_lanes() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF301_0D02).unwrap().mnemonic,
        Mnemonic::VPADD
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF311_0D02).unwrap().mnemonic,
        Mnemonic::VPADD
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF305_4D06).unwrap().mnemonic,
        Mnemonic::VPADD
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF301_0D42).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.vfp.write_d_bits(
        1,
        u64::from(2.5f32.to_bits()) << 32 | u64::from(1.25f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        2,
        u64::from(20.0f32.to_bits()) << 32 | u64::from(10.0f32.to_bits()),
    );
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF301_0D02),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(0),
        u64::from(30.0f32.to_bits()) << 32 | u64::from(3.75f32.to_bits())
    );

    cpu.vfp.write_d_bits(
        5,
        u64::from(8.0f32.to_bits()) << 32 | u64::from(4.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        6,
        u64::from(64.0f32.to_bits()) << 32 | u64::from(32.0f32.to_bits()),
    );
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF305_4D06),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(4),
        u64::from(96.0f32.to_bits()) << 32 | u64::from(12.0f32.to_bits())
    );

    cpu.vfp.write_d_bits(1, 0x4400_4200_4000_3c00);
    cpu.vfp.write_d_bits(2, 0x4800_4700_4600_4500);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF311_0D02),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x4b80_4980_4700_4200);
}

#[test]
fn neon_fp_pairwise_minmax_handle_f32_and_f16_lanes() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF301_0F02).unwrap().mnemonic,
        Mnemonic::VPMAX
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF311_0F02).unwrap().mnemonic,
        Mnemonic::VPMAX
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF325_4F06).unwrap().mnemonic,
        Mnemonic::VPMIN
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF331_0F02).unwrap().mnemonic,
        Mnemonic::VPMIN
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF301_0F42).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.vfp.write_d_bits(
        1,
        u64::from(7.5f32.to_bits()) << 32 | u64::from((-1.0f32).to_bits()),
    );
    cpu.vfp.write_d_bits(
        2,
        u64::from(0.5f32.to_bits()) << 32 | u64::from(12.0f32.to_bits()),
    );
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF301_0F02),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(0),
        u64::from(12.0f32.to_bits()) << 32 | u64::from(7.5f32.to_bits())
    );

    cpu.vfp.write_d_bits(
        5,
        u64::from(8.0f32.to_bits()) << 32 | u64::from((-4.0f32).to_bits()),
    );
    cpu.vfp.write_d_bits(
        6,
        u64::from(64.0f32.to_bits()) << 32 | u64::from(32.0f32.to_bits()),
    );
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF325_4F06),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(4),
        u64::from(32.0f32.to_bits()) << 32 | u64::from((-4.0f32).to_bits())
    );

    cpu.vfp.write_d_bits(1, 0x4400_4200_4000_bc00);
    cpu.vfp.write_d_bits(2, 0x4800_4700_c600_4500);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF311_0F02),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x4800_4500_4400_4000);

    cpu.vfp.write_d_bits(1, 0x4400_4200_4000_bc00);
    cpu.vfp.write_d_bits(2, 0x4800_4700_c600_4500);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF331_0F02),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x4700_c600_4200_bc00);
}

#[test]
fn neon_pairwise_add_long_widens_pairs_and_accumulates() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF3B0_0201).unwrap().mnemonic,
        Mnemonic::VPADDL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B4_02C2).unwrap().mnemonic,
        Mnemonic::VPADDL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B0_4605).unwrap().mnemonic,
        Mnemonic::VPADAL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B4_86CA).unwrap().mnemonic,
        Mnemonic::VPADAL
    );

    cpu.vfp.write_d_bits(1, 0x80c0_4040_7f80_ff01);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B0_0201),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0xff40_0080_ffff_0000);

    cpu.vfp.write_d_bits(2, 0x8000_8000_ffff_0001);
    cpu.vfp.write_d_bits(3, 0x0001_ffff_0001_1234);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B4_02C2),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0001_0000_0001_0000);
    assert_eq!(cpu.vfp.read_d_bits(1), 0x0001_0000_0000_1235);

    cpu.vfp.write_d_bits(3, 0xffff_ffff_0000_0001);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B8_2203),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(2), 0);

    cpu.vfp.write_d_bits(4, 0xffff_8000_7fff_0001);
    cpu.vfp.write_d_bits(5, 0x8080_017f_ffff_0201);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B0_4605),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(4), 0xfeff_8080_7ffd_0004);

    cpu.vfp.write_d_bits(8, 0xffff_ffff_0000_0001);
    cpu.vfp.write_d_bits(9, 0x0000_0000_8000_0000);
    cpu.vfp.write_d_bits(10, 0x0001_ffff_0002_0001);
    cpu.vfp.write_d_bits(11, 0x0001_1234_8000_8000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B4_86CA),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(8), 0x0000_ffff_0000_0004);
    assert_eq!(cpu.vfp.read_d_bits(9), 0x0000_1235_8001_0000);

    cpu.vfp.write_d_bits(6, 5);
    cpu.vfp.write_d_bits(7, 0x0000_0001_7fff_ffff);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B8_6607),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(6), 0x0000_0000_8000_0005);

    assert_eq!(
        Aarch32Decoder::decode(0xF3B4_12C2).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    let invalid_size = DecodedInsn::new(Mnemonic::VPADDL, ExecutionState::Aarch32, 0xF3BC_0201, 4);
    assert!(matches!(
        Executor::new(&mut cpu, &mut mem).execute(&invalid_size),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_long_wide_add_sub_cover_signed_unsigned_and_invalid_shapes() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF282_0004).unwrap().mnemonic,
        Mnemonic::VADDL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF38C_620E).unwrap().mnemonic,
        Mnemonic::VSUBL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF398_410A).unwrap().mnemonic,
        Mnemonic::VADDW
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF2AA_830C).unwrap().mnemonic,
        Mnemonic::VSUBW
    );
    cpu.vfp.write_d_bits(2, 0xc040_f010_01ff_7f80);
    cpu.vfp.write_d_bits(4, 0x40c0_10f0_ff02_0101);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF282_0004),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0000_0001_0080_ff81);
    assert_eq!(cpu.vfp.read_d_bits(1), 0x0000_0000_0000_0000);

    cpu.vfp.write_d_bits(12, 0xc864_0580_01ff_000a);
    cpu.vfp.write_d_bits(14, 0x64c8_0a7f_02ff_0103);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF38C_620E),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(6), 0xffff_0000_ffff_0007);
    assert_eq!(cpu.vfp.read_d_bits(7), 0x0064_ff9c_fffb_0001);

    cpu.vfp.write_d_bits(8, (0xffff_0000u64 << 32) | 100);
    cpu.vfp.write_d_bits(9, (1u64 << 32) | 0x7fff_ffff);
    cpu.vfp.write_d_bits(10, 0x8000_ffff_0002_0001);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF398_410A),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(4), 0xffff_0002_0000_0065);
    assert_eq!(cpu.vfp.read_d_bits(5), 0x0000_8001_8000_fffe);

    cpu.vfp.write_d_bits(10, 10);
    cpu.vfp.write_d_bits(11, -20i64 as u64);
    cpu.vfp.write_d_bits(12, ((-5i32 as u32 as u64) << 32) | 3);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF2AA_830C),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(8), 7);
    assert_eq!(cpu.vfp.read_d_bits(9), -15i64 as u64);

    let invalid_odd_dest =
        DecodedInsn::new(Mnemonic::VADDL, ExecutionState::Aarch32, 0xF282_1004, 4);
    assert!(matches!(
        Executor::new(&mut cpu, &mut mem).execute(&invalid_odd_dest),
        ExecResult::Undefined
    ));
    let invalid_size = DecodedInsn::new(Mnemonic::VADDL, ExecutionState::Aarch32, 0xF2B2_0004, 4);
    assert!(matches!(
        Executor::new(&mut cpu, &mut mem).execute(&invalid_size),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_narrow_add_sub_keep_high_half_with_rounding() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF282_0404).unwrap().mnemonic,
        Mnemonic::VADDHN
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF392_84A4).unwrap().mnemonic,
        Mnemonic::VRADDHN
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF2AC_E6AE).unwrap().mnemonic,
        Mnemonic::VSUBHN
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3A8_260C).unwrap().mnemonic,
        Mnemonic::VRSUBHN
    );

    cpu.vfp.write_d_bits(2, 0x7fff_8000_ff00_00ff);
    cpu.vfp.write_d_bits(3, 0x00f0_0100_ffff_1234);
    cpu.vfp.write_d_bits(4, 0x0001_8000_0200_0001);
    cpu.vfp.write_d_bits(5, 0x0010_ff00_0001_edcc);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF282_0404),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0100_0000_8000_0101);

    cpu.vfp.write_d_bits(18, 0xffff_0000_0000_ffff);
    cpu.vfp.write_d_bits(19, 0x8000_0000_7fff_ffff);
    cpu.vfp.write_d_bits(20, 0x0002_0000_0000_0001);
    cpu.vfp.write_d_bits(21, 0x8000_0000_0000_0001);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF392_84A4),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(8), 0x0000_8000_0001_0001);

    cpu.vfp.write_d_bits(28, 0x0000_0001_0000_0000);
    cpu.vfp.write_d_bits(29, 0xffff_ffff_ffff_ffff);
    cpu.vfp.write_d_bits(30, 0x0000_0000_0000_0001);
    cpu.vfp.write_d_bits(31, 0x0000_0000_ffff_ffff);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF2AC_E6AE),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(14), 0xffff_ffff_0000_0000);

    cpu.vfp.write_d_bits(8, 0x0000_0000_0000_0000);
    cpu.vfp.write_d_bits(9, 0x0000_0000_ffff_ffff);
    cpu.vfp.write_d_bits(12, 0x0000_0000_0000_0001);
    cpu.vfp.write_d_bits(13, 0x0000_0000_0000_0000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3A8_260C),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(2), 0x0000_0001_0000_0000);

    let invalid_odd_source =
        DecodedInsn::new(Mnemonic::VADDHN, ExecutionState::Aarch32, 0xF283_0404, 4);
    assert!(matches!(
        Executor::new(&mut cpu, &mut mem).execute(&invalid_odd_source),
        ExecResult::Undefined
    ));
    let invalid_size = DecodedInsn::new(Mnemonic::VADDHN, ExecutionState::Aarch32, 0xF2B2_0404, 4);
    assert!(matches!(
        Executor::new(&mut cpu, &mut mem).execute(&invalid_size),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_integer_multiply_accumulate_subtract_wraps_by_lane_width() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF201_0912).unwrap().mnemonic,
        Mnemonic::VMUL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF201_0902).unwrap().mnemonic,
        Mnemonic::VMLA
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF328_690A).unwrap().mnemonic,
        Mnemonic::VMLS
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF291_084A).unwrap().mnemonic,
        Mnemonic::VMUL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3A2_0844).unwrap().mnemonic,
        Mnemonic::VMUL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF297_606B).unwrap().mnemonic,
        Mnemonic::VMLA
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3AA_8466).unwrap().mnemonic,
        Mnemonic::VMLS
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF301_0912).unwrap().mnemonic,
        Mnemonic::VMUL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF302_0954).unwrap().mnemonic,
        Mnemonic::VMUL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF222_1954).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF281_0842).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3A2_1844).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF302_1954).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.vfp.write_d_bits(1, 0x0605_0403_7f80_ff02);
    cpu.vfp.write_d_bits(2, 0x2233_40ff_0202_0203);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF201_0912),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0xccff_00fd_fe00_fe06);

    cpu.vfp.write_d_bits(4, 0x4000_8000_0002_ffff);
    cpu.vfp.write_d_bits(5, 0x0004_0002_ffff_0002);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF214_3915),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(3), 0x0000_0000_fffe_fffe);

    cpu.vfp.write_d_bits(0, 0x0080_ff01);
    cpu.vfp.write_d_bits(1, 0x0504_0302);
    cpu.vfp.write_d_bits(2, 0x0908_0706);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF201_0902),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x2da0_140d);

    cpu.vfp.write_d_bits(6, 0xffff_ffff_0000_0064);
    cpu.vfp.write_d_bits(8, 0x8000_0000_0000_000a);
    cpu.vfp.write_d_bits(10, 0x0000_0002_0000_0003);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF328_690A),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(6), 0xffff_ffff_0000_0046);

    cpu.vfp.write_d_bits(2, 0x0000_0002_0000_0001);
    cpu.vfp.write_d_bits(3, 0x8000_0000_ffff_ffff);
    cpu.vfp.write_d_bits(4, 0x0000_0004_0000_0003);
    cpu.vfp.write_d_bits(5, 0x0000_0002_0000_0002);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF222_0954),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0000_0008_0000_0003);
    assert_eq!(cpu.vfp.read_d_bits(1), 0x0000_0000_ffff_fffe);

    cpu.vfp.write_d_bits(1, 0x0004_0003_0002_0001);
    cpu.vfp.write_d_bits(2, 0x0000_0000_0005_0006);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF291_084A),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0014_000f_000a_0005);

    cpu.vfp.write_d_bits(2, 0xffff_ffff_0000_0001);
    cpu.vfp.write_d_bits(3, 0x8000_0000_0000_0003);
    cpu.vfp.write_d_bits(4, 0x0000_0002_0000_0004);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3A2_0844),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0xffff_fffc_0000_0004);
    assert_eq!(cpu.vfp.read_d_bits(1), 0x0000_0000_0000_000c);

    cpu.vfp.write_d_bits(3, 0x0003_0000_0000_0000);
    cpu.vfp.write_d_bits(6, 0x000a_000a_000a_000a);
    cpu.vfp.write_d_bits(7, 0x0004_0003_0002_0001);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF297_606B),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(6), 0x0016_0013_0010_000d);

    cpu.vfp.write_d_bits(6, 0x0000_0002_0000_0001);
    cpu.vfp.write_d_bits(8, 0x0000_00c8_0000_0064);
    cpu.vfp.write_d_bits(9, 0x0000_0190_0000_012c);
    cpu.vfp.write_d_bits(10, 0x0000_0002_0000_0001);
    cpu.vfp.write_d_bits(11, 0x0000_0004_0000_0003);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3AA_8466),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(8), 0x0000_00c4_0000_0062);
    assert_eq!(cpu.vfp.read_d_bits(9), 0x0000_0188_0000_0126);

    cpu.vfp.write_d_bits(1, 0xfe12_aa01_0f80_ff57);
    cpu.vfp.write_d_bits(2, 0xef34_55c3_1180_0283);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF301_0912),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x4a28_22c3_ff00_fe79);

    cpu.vfp.write_d_bits(2, 0xfe12_aa01_0f80_ff57);
    cpu.vfp.write_d_bits(3, 0x0f03_f070_aa55_0101);
    cpu.vfp.write_d_bits(4, 0xef34_55c3_1180_0283);
    cpu.vfp.write_d_bits(5, 0x1120_0f81_55aa_ff02);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF302_0954),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x4a28_22c3_ff00_fe79);
    assert_eq!(cpu.vfp.read_d_bits(1), 0xff60_5070_2222_ff02);

    let invalid_size = DecodedInsn::new(Mnemonic::VMUL, ExecutionState::Aarch32, 0xF231_0912, 4);
    assert!(matches!(
        Executor::new(&mut cpu, &mut mem).execute(&invalid_size),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_long_multiply_accumulate_subtract_widens_products() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF282_0C04).unwrap().mnemonic,
        Mnemonic::VMULL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF282_0804).unwrap().mnemonic,
        Mnemonic::VMLAL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3D4_0AA6).unwrap().mnemonic,
        Mnemonic::VMLSL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF292_0A43).unwrap().mnemonic,
        Mnemonic::VMULL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3AA_8265).unwrap().mnemonic,
        Mnemonic::VMLAL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF29E_C66F).unwrap().mnemonic,
        Mnemonic::VMLSL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF28A_8E0B).unwrap().mnemonic,
        Mnemonic::VMULL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF28A_9E0B).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.vfp.write_d_bits(2, 0x0605_0403_7f80_ff02);
    cpu.vfp.write_d_bits(4, 0x2233_40ff_0202_0203);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF282_0C04),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x00fe_ff00_fffe_0006);
    assert_eq!(cpu.vfp.read_d_bits(1), 0x00cc_00ff_0100_fffd);

    cpu.vfp.write_d_bits(8, 0x4000_8000_0002_ffff);
    cpu.vfp.write_d_bits(10, 0x0004_0002_ffff_0002);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF398_4C0A),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(4), 0x0001_fffe_0001_fffe);
    assert_eq!(cpu.vfp.read_d_bits(5), 0x0001_0000_0001_0000);

    cpu.vfp.write_d_bits(12, 0x8000_0000_0000_000a);
    cpu.vfp.write_d_bits(14, 0x0000_0002_0000_0003);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF2AC_8C0E),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(8), 0x0000_0000_0000_001e);
    assert_eq!(cpu.vfp.read_d_bits(9), 0xffff_ffff_0000_0000);

    cpu.vfp.write_d_bits(0, 0x0000_8000_ffff_0001);
    cpu.vfp.write_d_bits(1, 0);
    cpu.vfp.write_d_bits(2, 0x7f80_ff02);
    cpu.vfp.write_d_bits(4, 0x0202_0203);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF282_0804),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x00fe_7f00_fffd_0007);

    cpu.vfp.write_d_bits(16, 0xffff_ffff_0000_0064);
    cpu.vfp.write_d_bits(17, 0x0000_0000_8000_0000);
    cpu.vfp.write_d_bits(20, 0x4000_8000_0002_ffff);
    cpu.vfp.write_d_bits(22, 0x0004_0002_ffff_0002);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3D4_0AA6),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(16), 0xfffe_0001_fffe_0066);
    assert_eq!(cpu.vfp.read_d_bits(17), 0xffff_0000_7fff_0000);

    cpu.vfp.write_d_bits(2, 0xfffc_0003_fffe_0001);
    cpu.vfp.write_d_bits(3, 0x0000_0000_0000_0005);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF292_0A43),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0xffff_fff6_0000_0005);
    assert_eq!(cpu.vfp.read_d_bits(1), 0xffff_ffec_0000_000f);

    cpu.vfp.write_d_bits(5, 0x0000_0005_0000_0001);
    cpu.vfp.write_d_bits(8, 0x0000_0000_0000_000a);
    cpu.vfp.write_d_bits(9, 0x0000_0000_0000_0014);
    cpu.vfp.write_d_bits(10, 0x0000_0004_0000_0003);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3AA_8265),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(8), 25);
    assert_eq!(cpu.vfp.read_d_bits(9), 40);

    cpu.vfp.write_d_bits(7, 0x0003_0000_0000_0000);
    cpu.vfp.write_d_bits(12, 0x0000_0064_0000_0064);
    cpu.vfp.write_d_bits(13, 0x0000_0064_0000_0064);
    cpu.vfp.write_d_bits(14, 0x0004_fffd_0002_0001);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF29E_C66F),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(12), 0x0000_005e_0000_0061);
    assert_eq!(cpu.vfp.read_d_bits(13), 0x0000_0058_0000_006d);

    cpu.vfp.write_d_bits(10, 0xfe12_aa01_0f80_ff57);
    cpu.vfp.write_d_bits(11, 0xef34_55c3_1180_0283);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF28A_8E0B),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(8), 0x00ff_4000_01fe_2b79);
    assert_eq!(cpu.vfp.read_d_bits(9), 0x5a4a_0328_2222_00c3);

    let invalid_odd_dest =
        DecodedInsn::new(Mnemonic::VMULL, ExecutionState::Aarch32, 0xF282_1C04, 4);
    assert!(matches!(
        Executor::new(&mut cpu, &mut mem).execute(&invalid_odd_dest),
        ExecResult::Undefined
    ));
    let invalid_size = DecodedInsn::new(Mnemonic::VMULL, ExecutionState::Aarch32, 0xF2B2_0C04, 4);
    assert!(matches!(
        Executor::new(&mut cpu, &mut mem).execute(&invalid_size),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_saturating_doubling_long_multiply_sets_qc_on_saturation() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF292_0D04).unwrap().mnemonic,
        Mnemonic::VQDMULL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF29C_890E).unwrap().mnemonic,
        Mnemonic::VQDMLAL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF2D4_0BA6).unwrap().mnemonic,
        Mnemonic::VQDMLSL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF2D2_0BCB).unwrap().mnemonic,
        Mnemonic::VQDMULL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF2D6_43C5).unwrap().mnemonic,
        Mnemonic::VQDMLAL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF2EA_87C7).unwrap().mnemonic,
        Mnemonic::VQDMLSL
    );

    cpu.vfp.fpscr.set_qc(false);
    cpu.vfp.write_d_bits(2, 0x4000_8000_ffff_0002);
    cpu.vfp.write_d_bits(4, 0x0004_8000_0002_0003);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF292_0D04),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0xffff_fffc_0000_000c);
    assert_eq!(cpu.vfp.read_d_bits(1), 0x0002_0000_7fff_ffff);
    assert!(cpu.vfp.fpscr.qc());

    cpu.vfp.fpscr.set_qc(false);
    cpu.vfp.write_d_bits(8, 0x8000_0000_0000_000a);
    cpu.vfp.write_d_bits(10, 0x8000_0000_0000_0003);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF2A8_4D0A),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(4), 0x0000_0000_0000_003c);
    assert_eq!(cpu.vfp.read_d_bits(5), 0x7fff_ffff_ffff_ffff);
    assert!(cpu.vfp.fpscr.qc());

    cpu.vfp.fpscr.set_qc(false);
    cpu.vfp.write_d_bits(8, 0xffff_ffff_0000_0001);
    cpu.vfp.write_d_bits(9, 0x8000_0000_7fff_ffff);
    cpu.vfp.write_d_bits(12, 0x8000_4000_ffff_0002);
    cpu.vfp.write_d_bits(14, 0x8000_0004_0002_0003);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF29C_890E),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(8), 0xffff_fffb_0000_000d);
    assert_eq!(cpu.vfp.read_d_bits(9), 0x0000_0000_7fff_ffff);
    assert!(cpu.vfp.fpscr.qc());

    cpu.vfp.fpscr.set_qc(false);
    cpu.vfp.write_d_bits(16, 0x0000_0000_0000_0064);
    cpu.vfp.write_d_bits(17, 0x7fff_ffff_8000_0000);
    cpu.vfp.write_d_bits(20, 0x8000_4000_ffff_0002);
    cpu.vfp.write_d_bits(22, 0x8000_0004_0002_0003);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF2D4_0BA6),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(16), 0x0000_0004_0000_0058);
    assert_eq!(cpu.vfp.read_d_bits(17), 0xffff_ffff_8000_0000);
    assert!(cpu.vfp.fpscr.qc());

    cpu.vfp.fpscr.set_qc(false);
    cpu.vfp.write_d_bits(3, 0x0000_0000_8000_0000);
    cpu.vfp.write_d_bits(18, 0x4000_ffff_8000_0002);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF2D2_0BCB),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(16), 0x7fff_ffff_fffe_0000);
    assert_eq!(cpu.vfp.read_d_bits(17), 0xc000_0000_0001_0000);
    assert!(cpu.vfp.fpscr.qc());

    cpu.vfp.fpscr.set_qc(false);
    cpu.vfp.write_d_bits(5, 0x0000_0000_0000_0003);
    cpu.vfp.write_d_bits(20, 0x0000_0014_0000_000a);
    cpu.vfp.write_d_bits(21, 0x0000_0028_0000_001e);
    cpu.vfp.write_d_bits(22, 0x4000_ffff_0002_0001);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF2D6_43C5),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(20), 0x0000_0020_0000_0010);
    assert_eq!(cpu.vfp.read_d_bits(21), 0x0001_8028_0000_0018);
    assert!(!cpu.vfp.fpscr.qc());

    cpu.vfp.fpscr.set_qc(false);
    cpu.vfp.write_d_bits(7, 5);
    cpu.vfp.write_d_bits(24, 100);
    cpu.vfp.write_d_bits(25, 200);
    cpu.vfp.write_d_bits(26, 0x0000_0004_0000_0003);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF2EA_87C7),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(24), 70);
    assert_eq!(cpu.vfp.read_d_bits(25), 160);
    assert!(!cpu.vfp.fpscr.qc());

    let invalid_size = DecodedInsn::new(Mnemonic::VQDMULL, ExecutionState::Aarch32, 0xF282_0D04, 4);
    assert!(matches!(
        Executor::new(&mut cpu, &mut mem).execute(&invalid_size),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_shift_right_immediate_handles_signed_unsigned_rounding_and_q_forms() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF38D_4015).unwrap().mnemonic,
        Mnemonic::VSHR
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF29C_6017).unwrap().mnemonic,
        Mnemonic::VSHR
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF38D_8219).unwrap().mnemonic,
        Mnemonic::VRSHR
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3AF_4058).unwrap().mnemonic,
        Mnemonic::VSHR
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3AF_5058).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.vfp.write_d_bits(5, 0x0003_0407_087f_80ff);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF38D_4015),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(4), 0x0000_0000_010f_101f);

    cpu.vfp.write_d_bits(7, 0x0010_7fff_ffff_8000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF29C_6017),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(6), 0x0001_07ff_ffff_f800);

    cpu.vfp.write_d_bits(9, 0x0003_0407_087f_80ff);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF38D_8219),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(8), 0x0000_0101_0110_1020);

    cpu.vfp.write_d_bits(11, 0x0017_7fff_ffff_8000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF29C_A21B),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(10), 0x0001_0800_0000_f800);

    cpu.vfp.write_d_bits(8, 0x8000_0000_ffff_ffff);
    cpu.vfp.write_d_bits(9, 0x0001_ffff_0002_0000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3AF_4058),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(4), 0x0000_4000_0000_7fff);
    assert_eq!(cpu.vfp.read_d_bits(5), 0x0000_0000_0000_0001);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3AF_5058),
        ExecResult::Undefined
    ));
    let invalid_imm = DecodedInsn::new(Mnemonic::VSHR, ExecutionState::Aarch32, 0xF307_4015, 4);
    assert!(matches!(
        Executor::new(&mut cpu, &mut mem).execute(&invalid_imm),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_shift_accumulate_immediate_adds_shifted_lanes_to_destination() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF28F_0111).unwrap().mnemonic,
        Mnemonic::VSRA
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF39C_0152).unwrap().mnemonic,
        Mnemonic::VSRA
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF2B8_2313).unwrap().mnemonic,
        Mnemonic::VRSRA
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF38D_4358).unwrap().mnemonic,
        Mnemonic::VRSRA
    );

    cpu.vfp.write_d_bits(0, 0x0a00_80ff_0403_0201);
    cpu.vfp.write_d_bits(1, 0x8104_fe01_7f80_ff02);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF28F_0111),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0xca02_7fff_43c3_0102);

    cpu.vfp.write_d_bits(0, 0xffff_ff00_0002_0001);
    cpu.vfp.write_d_bits(1, 0xff00_000f_1000_0000);
    cpu.vfp.write_d_bits(2, 0x0001_8000_ffff_0010);
    cpu.vfp.write_d_bits(3, 0x00f0_fff0_000f_1000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF39C_0152),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0xffff_0700_1001_0002);
    assert_eq!(cpu.vfp.read_d_bits(1), 0xff0f_100e_1000_0100);

    cpu.vfp.write_d_bits(2, 0x8000_0000_0000_0001);
    cpu.vfp.write_d_bits(3, 0xffff_ff00_0000_0180);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF2B8_2313),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(2), 0x7fff_ffff_0000_0003);

    cpu.vfp.write_d_bits(4, 0x0706_0504_0302_0100);
    cpu.vfp.write_d_bits(5, 0x6050_4030_2010_00ff);
    cpu.vfp.write_d_bits(8, 0x80ff_0807_0403_0100);
    cpu.vfp.write_d_bits(9, 0x0120_407f_fe12_1110);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF38D_4358),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(4), 0x1726_0605_0402_0100);
    assert_eq!(cpu.vfp.read_d_bits(5), 0x6054_4840_4012_0201);

    assert_eq!(
        Aarch32Decoder::decode(0xF39C_1152).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    let invalid_imm = DecodedInsn::new(Mnemonic::VSRA, ExecutionState::Aarch32, 0xF307_0111, 4);
    assert!(matches!(
        Executor::new(&mut cpu, &mut mem).execute(&invalid_imm),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_saturating_shift_left_immediate_saturates_and_sets_qc() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF289_0711).unwrap().mnemonic,
        Mnemonic::VQSHL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF394_0752).unwrap().mnemonic,
        Mnemonic::VQSHL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF2A8_2713).unwrap().mnemonic,
        Mnemonic::VQSHL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF38A_4615).unwrap().mnemonic,
        Mnemonic::VQSHLU
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF393_865A).unwrap().mnemonic,
        Mnemonic::VQSHLU
    );

    cpu.vfp.fpscr.set_qc(false);
    cpu.vfp.write_d_bits(1, 0x8120_ffc0_807f_4001);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF289_0711),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x8040_fe80_807f_7f02);
    assert!(cpu.vfp.fpscr.qc());

    cpu.vfp.fpscr.set_qc(false);
    cpu.vfp.write_d_bits(2, 0x8000_ffff_1000_0001);
    cpu.vfp.write_d_bits(3, 0xf000_0fff_0100_000f);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF394_0752),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0xffff_ffff_ffff_0010);
    assert_eq!(cpu.vfp.read_d_bits(1), 0xffff_fff0_1000_00f0);
    assert!(cpu.vfp.fpscr.qc());

    cpu.vfp.fpscr.set_qc(false);
    cpu.vfp.write_d_bits(3, 0x0080_0000_0000_0001);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF2A8_2713),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(2), 0x7fff_ffff_0000_0100);
    assert!(cpu.vfp.fpscr.qc());

    cpu.vfp.fpscr.set_qc(false);
    cpu.vfp.write_d_bits(5, 0x1000_80ff_7f40_2001);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF38A_4615),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(4), 0x4000_0000_ffff_8004);
    assert!(cpu.vfp.fpscr.qc());

    cpu.vfp.fpscr.set_qc(false);
    cpu.vfp.write_d_bits(10, 0x8000_7fff_1000_0001);
    cpu.vfp.write_d_bits(11, 0x4000_2000_0002_ffff);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF393_865A),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(8), 0x0000_ffff_8000_0008);
    assert_eq!(cpu.vfp.read_d_bits(9), 0xffff_ffff_0010_0000);
    assert!(cpu.vfp.fpscr.qc());

    assert_eq!(
        Aarch32Decoder::decode(0xF394_1752).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    let invalid_imm = DecodedInsn::new(Mnemonic::VQSHL, ExecutionState::Aarch32, 0xF207_0711, 4);
    assert!(matches!(
        Executor::new(&mut cpu, &mut mem).execute(&invalid_imm),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_shift_left_and_insert_immediate_update_expected_bits() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF289_0511).unwrap().mnemonic,
        Mnemonic::VSHL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF38A_C51D).unwrap().mnemonic,
        Mnemonic::VSLI
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF39B_E41F).unwrap().mnemonic,
        Mnemonic::VSRI
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF2AC_1552).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.vfp.write_d_bits(1, 0x11aa_55ff_807f_0201);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF289_0511),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x2254_aafe_00fe_0402);

    cpu.vfp.write_d_bits(3, 0xffff_8000_1234_0001);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF295_2513),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(2), 0xffe0_0000_4680_0020);

    cpu.vfp.write_d_bits(2, 0x000f_ffff_0000_0001);
    cpu.vfp.write_d_bits(3, 0xffff_ffff_8000_0000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF2AC_0552),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0xffff_f000_0000_1000);
    assert_eq!(cpu.vfp.read_d_bits(1), 0xffff_f000_0000_0000);

    cpu.vfp.write_d_bits(12, 0xcc33_f00f_5aa5_00ff);
    cpu.vfp.write_d_bits(13, 0x11aa_55ff_807f_0201);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF38A_C51D),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(12), 0x44ab_54ff_02fd_0807);

    cpu.vfp.write_d_bits(14, 0x5555_aaaa_0000_ffff);
    cpu.vfp.write_d_bits(15, 0x8000_0001_ffff_1234);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF39B_E41F),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(14), 0x5400_a800_07ff_f891);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF2AC_1552),
        ExecResult::Undefined
    ));
    let invalid_zero_shift =
        DecodedInsn::new(Mnemonic::VSHL, ExecutionState::Aarch32, 0xF288_0511, 4);
    assert!(matches!(
        Executor::new(&mut cpu, &mut mem).execute(&invalid_zero_shift),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_shift_narrow_immediate_keeps_shifted_low_half() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF28D_0812).unwrap().mnemonic,
        Mnemonic::VSHRN
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF28D_7878).unwrap().mnemonic,
        Mnemonic::VRSHRN
    );

    cpu.vfp.write_d_bits(2, 0x7fff_8000_ff00_00ff);
    cpu.vfp.write_d_bits(3, 0x00f0_0100_ffff_1234);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF28D_0812),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x1e20_ff46_ff00_e01f);

    cpu.vfp.write_d_bits(8, 0xffff_0000_0000_ffff);
    cpu.vfp.write_d_bits(9, 0x7fff_ffff_8000_0000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF294_3818),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(3), 0xffff_0000_fff0_000f);

    cpu.vfp.write_d_bits(16, 0x0000_0001_0000_0000);
    cpu.vfp.write_d_bits(17, 0xffff_ffff_ffff_ffff);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF2A0_6830),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(6), 0xffff_ffff_0000_0001);

    cpu.vfp.write_d_bits(24, 0x0008_0007_0004_0003);
    cpu.vfp.write_d_bits(25, 0x7fff_8000_ff00_00ff);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF28D_7878),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(7), 0x0000_e020_0101_0100);

    let invalid_odd_source =
        DecodedInsn::new(Mnemonic::VSHRN, ExecutionState::Aarch32, 0xF28D_0813, 4);
    assert!(matches!(
        Executor::new(&mut cpu, &mut mem).execute(&invalid_odd_source),
        ExecResult::Undefined
    ));
    let invalid_imm = DecodedInsn::new(Mnemonic::VSHRN, ExecutionState::Aarch32, 0xF207_0812, 4);
    assert!(matches!(
        Executor::new(&mut cpu, &mut mem).execute(&invalid_imm),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_saturating_shift_narrow_immediate_saturates_and_sets_qc() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF28C_8932).unwrap().mnemonic,
        Mnemonic::VQSHRN
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF38C_9934).unwrap().mnemonic,
        Mnemonic::VQSHRN
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF28B_C97A).unwrap().mnemonic,
        Mnemonic::VQRSHRN
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF38C_1814).unwrap().mnemonic,
        Mnemonic::VQSHRUN
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF393_4870).unwrap().mnemonic,
        Mnemonic::VQRSHRUN
    );

    cpu.vfp.fpscr.set_qc(false);
    cpu.vfp.write_d_bits(18, 0x8000_f800_0800_07f0);
    cpu.vfp.write_d_bits(19, 0x0010_000f_ffff_7fff);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF28C_8932),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(8), 0x0100_ff7f_8080_7f7f);
    assert!(cpu.vfp.fpscr.qc());

    cpu.vfp.fpscr.set_qc(false);
    cpu.vfp.write_d_bits(20, 0x000f_ffff_1000_0ff0);
    cpu.vfp.write_d_bits(21, 0x0100_00f0_8000_0010);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF38C_9934),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(9), 0x100f_ff01_00ff_ffff);
    assert!(cpu.vfp.fpscr.qc());

    cpu.vfp.fpscr.set_qc(false);
    cpu.vfp.write_d_bits(26, 0xf800_f801_07f0_07ef);
    cpu.vfp.write_d_bits(27, 0x0010_000f_8000_7fff);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF28B_C97A),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(12), 0x0100_807f_c0c0_403f);
    assert!(cpu.vfp.fpscr.qc());

    cpu.vfp.fpscr.set_qc(false);
    cpu.vfp.write_d_bits(4, 0xf800_ffff_0800_07f0);
    cpu.vfp.write_d_bits(5, 0x8000_7fff_0010_000f);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF38C_1814),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(1), 0x00ff_0100_0000_807f);
    assert!(cpu.vfp.fpscr.qc());

    cpu.vfp.fpscr.set_qc(false);
    cpu.vfp.write_d_bits(16, 0x0010_0000_000f_fff0);
    cpu.vfp.write_d_bits(17, 0);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF393_4870),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(4), 0x0000_0000_0080_0080);
    assert!(!cpu.vfp.fpscr.qc());

    let invalid_odd_source =
        DecodedInsn::new(Mnemonic::VQSHRN, ExecutionState::Aarch32, 0xF28C_8933, 4);
    assert!(matches!(
        Executor::new(&mut cpu, &mut mem).execute(&invalid_odd_source),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_fp_convert_between_f32_and_i32_lanes() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF3BB_0601).unwrap().mnemonic,
        Mnemonic::VCVT_F32_S32
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3BB_2683).unwrap().mnemonic,
        Mnemonic::VCVT_F32_U32
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3BB_4705).unwrap().mnemonic,
        Mnemonic::VCVT_S32_F32
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3BB_6787).unwrap().mnemonic,
        Mnemonic::VCVT_U32_F32
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3BB_0642).unwrap().mnemonic,
        Mnemonic::VCVT_F32_S32
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3BB_4746).unwrap().mnemonic,
        Mnemonic::VCVT_S32_F32
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B6_0602).unwrap().mnemonic,
        Mnemonic::VCVT_F16_F32
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B6_4703).unwrap().mnemonic,
        Mnemonic::VCVT_F32_F16
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3BB_1642).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.vfp
        .write_d_bits(1, (42u64 << 32) | u64::from((-3i32) as u32));
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3BB_0601),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(0),
        u64::from(42.0f32.to_bits()) << 32 | u64::from((-3.0f32).to_bits())
    );

    cpu.vfp.write_d_bits(3, (100u64 << 32) | 7);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3BB_2683),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(2),
        u64::from(100.0f32.to_bits()) << 32 | u64::from(7.0f32.to_bits())
    );

    cpu.vfp.write_d_bits(
        5,
        u64::from(9.75f32.to_bits()) << 32 | u64::from((-2.5f32).to_bits()),
    );
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3BB_4705),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(4),
        (9u64 << 32) | u64::from((-2i32) as u32)
    );

    cpu.vfp.write_d_bits(
        7,
        u64::from(12.9f32.to_bits()) << 32 | u64::from(3.25f32.to_bits()),
    );
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3BB_6787),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(6), (12u64 << 32) | 3);

    cpu.vfp
        .write_d_bits(2, (20u64 << 32) | u64::from((-10i32) as u32));
    cpu.vfp
        .write_d_bits(3, (40u64 << 32) | u64::from((-30i32) as u32));
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3BB_0642),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(0),
        u64::from(20.0f32.to_bits()) << 32 | u64::from((-10.0f32).to_bits())
    );
    assert_eq!(
        cpu.vfp.read_d_bits(1),
        u64::from(40.0f32.to_bits()) << 32 | u64::from((-30.0f32).to_bits())
    );

    cpu.vfp.write_d_bits(
        6,
        u64::from(4.75f32.to_bits()) << 32 | u64::from((-1.25f32).to_bits()),
    );
    cpu.vfp.write_d_bits(
        7,
        u64::from(8.5f32.to_bits()) << 32 | u64::from((-6.5f32).to_bits()),
    );
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3BB_4746),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(4),
        (4u64 << 32) | u64::from((-1i32) as u32)
    );
    assert_eq!(
        cpu.vfp.read_d_bits(5),
        (8u64 << 32) | u64::from((-6i32) as u32)
    );

    cpu.vfp.write_d_bits(
        2,
        u64::from(2.0f32.to_bits()) << 32 | u64::from(1.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        3,
        u64::from((-4.0f32).to_bits()) << 32 | u64::from(0.5f32.to_bits()),
    );
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B6_0602),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0xc400_3800_4000_3c00);

    cpu.vfp.write_d_bits(3, 0xc400_3800_4000_3c00);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B6_4703),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(4),
        u64::from(2.0f32.to_bits()) << 32 | u64::from(1.0f32.to_bits())
    );
    assert_eq!(
        cpu.vfp.read_d_bits(5),
        u64::from((-4.0f32).to_bits()) << 32 | u64::from(0.5f32.to_bits())
    );
}

#[test]
fn neon_fp_fixed_convert_between_f32_and_i32_lanes() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF2BC_0E11).unwrap().mnemonic,
        Mnemonic::VCVT_F32_S32_FIXED
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B8_2E13).unwrap().mnemonic,
        Mnemonic::VCVT_F32_U32_FIXED
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF2B4_4F15).unwrap().mnemonic,
        Mnemonic::VCVT_S32_F32_FIXED
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B0_6F17).unwrap().mnemonic,
        Mnemonic::VCVT_U32_F32_FIXED
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF2BC_0E52).unwrap().mnemonic,
        Mnemonic::VCVT_F32_S32_FIXED
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF2B4_4F56).unwrap().mnemonic,
        Mnemonic::VCVT_S32_F32_FIXED
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF2BC_1E52).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.vfp
        .write_d_bits(1, u64::from((-48i32) as u32) << 32 | 32);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF2BC_0E11),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(0),
        u64::from((-3.0f32).to_bits()) << 32 | u64::from(2.0f32.to_bits())
    );

    cpu.vfp.write_d_bits(3, (1024u64 << 32) | 512);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B8_2E13),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(2),
        u64::from(4.0f32.to_bits()) << 32 | u64::from(2.0f32.to_bits())
    );

    cpu.vfp.write_d_bits(
        5,
        u64::from((-2.25f32).to_bits()) << 32 | u64::from(1.5f32.to_bits()),
    );
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF2B4_4F15),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(4),
        u64::from((-9216i32) as u32) << 32 | 6144
    );

    cpu.vfp.write_d_bits(
        7,
        u64::from(2.0f32.to_bits()) << 32 | u64::from(0.5f32.to_bits()),
    );
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B0_6F17),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(6), (131072u64 << 32) | 32768);

    cpu.vfp
        .write_d_bits(2, (128u64 << 32) | u64::from((-64i32) as u32));
    cpu.vfp
        .write_d_bits(3, (256u64 << 32) | u64::from((-192i32) as u32));
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF2BC_0E52),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(0),
        u64::from(8.0f32.to_bits()) << 32 | u64::from((-4.0f32).to_bits())
    );
    assert_eq!(
        cpu.vfp.read_d_bits(1),
        u64::from(16.0f32.to_bits()) << 32 | u64::from((-12.0f32).to_bits())
    );

    cpu.vfp.write_d_bits(
        6,
        u64::from(2.5f32.to_bits()) << 32 | u64::from((-1.25f32).to_bits()),
    );
    cpu.vfp.write_d_bits(
        7,
        u64::from(8.0f32.to_bits()) << 32 | u64::from(0.75f32.to_bits()),
    );
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF2B4_4F56),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(4),
        (10240u64 << 32) | u64::from((-5120i32) as u32)
    );
    assert_eq!(cpu.vfp.read_d_bits(5), (32768u64 << 32) | 3072);
}

#[test]
fn neon_widen_and_narrow_moves_convert_between_d_and_q_registers() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF288_0A11).unwrap().mnemonic,
        Mnemonic::VMOVL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF390_4A16).unwrap().mnemonic,
        Mnemonic::VMOVL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B2_0202).unwrap().mnemonic,
        Mnemonic::VMOVN
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B6_4208).unwrap().mnemonic,
        Mnemonic::VMOVN
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B2_628A).unwrap().mnemonic,
        Mnemonic::VQMOVN
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B6_82CC).unwrap().mnemonic,
        Mnemonic::VQMOVN
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B2_A24E).unwrap().mnemonic,
        Mnemonic::VQMOVUN
    );

    cpu.vfp.write_d_bits(1, 0x02c0_4000_7f80_ff01);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF288_0A11),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x007f_ff80_ffff_0001);
    assert_eq!(cpu.vfp.read_d_bits(1), 0x0002_ffc0_0040_0000);

    cpu.vfp.write_d_bits(6, 0x1234_8000_ffff_0001);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF390_4A16),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(4), 0x0000_ffff_0000_0001);
    assert_eq!(cpu.vfp.read_d_bits(5), 0x0000_1234_0000_8000);

    cpu.vfp.write_d_bits(2, 0x1234_ff00_00ff_0001);
    cpu.vfp.write_d_bits(3, 0xabcd_ffff_7fff_8001);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B2_0202),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0xcdff_ff01_3400_ff01);

    cpu.vfp.write_d_bits(8, 0xffff_0003_0001_0002);
    cpu.vfp.write_d_bits(9, 0x8000_ffff_1234_5678);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B6_4208),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(4), 0xffff_5678_0003_0002);

    cpu.vfp.fpscr.set_qc(false);
    cpu.vfp.write_d_bits(10, 0xff80_0080_007f_0001);
    cpu.vfp.write_d_bits(11, 0x0000_8000_ffff_ff7f);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B2_628A),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(6), 0x0080_ff80_807f_7f01);
    assert!(cpu.vfp.fpscr.qc());

    cpu.vfp.fpscr.set_qc(false);
    cpu.vfp.write_d_bits(12, 0x0000_ffff_0000_0001);
    cpu.vfp.write_d_bits(13, 0xffff_ffff_0001_0000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B6_82CC),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(8), 0xffff_ffff_ffff_0001);
    assert!(cpu.vfp.fpscr.qc());

    cpu.vfp.fpscr.set_qc(false);
    cpu.vfp.write_d_bits(14, 0xffff_0100_00ff_0001);
    cpu.vfp.write_d_bits(15, 0x0000_0080_007f_ff80);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B2_A24E),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(10), 0x0080_7f00_00ff_ff01);
    assert!(cpu.vfp.fpscr.qc());

    assert_eq!(
        Aarch32Decoder::decode(0xF288_1A11).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    let invalid_source = DecodedInsn::new(Mnemonic::VMOVN, ExecutionState::Aarch32, 0xF3B2_0203, 4);
    assert!(matches!(
        Executor::new(&mut cpu, &mut mem).execute(&invalid_source),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_shift_register_handles_signed_counts_rounding_and_q_forms() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF202_0401).unwrap().mnemonic,
        Mnemonic::VSHL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF20A_6508).unwrap().mnemonic,
        Mnemonic::VRSHL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF31C_4548).unwrap().mnemonic,
        Mnemonic::VRSHL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF224_1452).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.vfp.write_d_bits(1, 0x7f80_0302_ff7f_8001);
    cpu.vfp.write_d_bits(2, 0x80f8_0807_fe02_ff01);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF202_0401),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x00ff_0000_fffc_c002);

    cpu.vfp.write_d_bits(4, 0xffff_00ff_8000_0001);
    cpu.vfp.write_d_bits(5, 0xfffc_0004_ffff_0001);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF315_3404),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(3), 0x0fff_0ff0_4000_0002);

    cpu.vfp.write_d_bits(8, 0x0302_80ff_8007_0707);
    cpu.vfp.write_d_bits(10, 0x0807_80f8_01fd_feff);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF20A_6508),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(6), 0x0000_ff00_0001_0204);

    cpu.vfp.write_d_bits(8, 0x0001_8000_0007_0007);
    cpu.vfp.write_d_bits(9, 0x8000_1234_4000_ffff);
    cpu.vfp.write_d_bits(12, 0x0010_0001_fffe_ffff);
    cpu.vfp.write_d_bits(13, 0xfffd_0000_0002_fff0);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF31C_4548),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(4), 0x0000_0000_0002_0004);
    assert_eq!(cpu.vfp.read_d_bits(5), 0x1000_1234_0000_0001);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF224_1452),
        ExecResult::Undefined
    ));
    let invalid_size = DecodedInsn::new(Mnemonic::VSHL, ExecutionState::Aarch32, 0xF232_0401, 4);
    assert!(matches!(
        Executor::new(&mut cpu, &mut mem).execute(&invalid_size),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_saturating_shift_register_saturates_left_shifts_and_sets_qc() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF202_0411).unwrap().mnemonic,
        Mnemonic::VQSHL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF20A_6518).unwrap().mnemonic,
        Mnemonic::VQRSHL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF31C_4558).unwrap().mnemonic,
        Mnemonic::VQRSHL
    );

    cpu.vfp.fpscr.set_qc(false);
    cpu.vfp.write_d_bits(1, 0x7f80_7f80_0102_4040);
    cpu.vfp.write_d_bits(2, 0x0080_feff_0807_0201);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF202_0411),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x7fff_1fc0_7f7f_7f7f);
    assert!(cpu.vfp.fpscr.qc());

    cpu.vfp.fpscr.set_qc(false);
    cpu.vfp.write_d_bits(4, 0x0001_0002_1000_8000);
    cpu.vfp.write_d_bits(5, 0x0010_000f_0004_0001);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF315_3414),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(3), 0xffff_ffff_ffff_ffff);
    assert!(cpu.vfp.fpscr.qc());

    cpu.vfp.fpscr.set_qc(false);
    cpu.vfp.write_d_bits(8, 0x7f80_8001_0240_0707);
    cpu.vfp.write_d_bits(10, 0x00fd_8008_0701_feff);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF20A_6518),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(6), 0x7ff0_ff7f_7f7f_0204);
    assert!(cpu.vfp.fpscr.qc());

    cpu.vfp.fpscr.set_qc(false);
    cpu.vfp.write_d_bits(8, 0x0001_8000_0007_0007);
    cpu.vfp.write_d_bits(9, 0x8000_1234_4000_ffff);
    cpu.vfp.write_d_bits(12, 0x0010_0001_fffe_ffff);
    cpu.vfp.write_d_bits(13, 0xfffd_0000_0002_fff0);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF31C_4558),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(4), 0xffff_ffff_0002_0004);
    assert_eq!(cpu.vfp.read_d_bits(5), 0x1000_1234_ffff_0001);
    assert!(cpu.vfp.fpscr.qc());
}

#[test]
fn neon_integer_compare_writes_lane_masks_for_signed_unsigned_and_q_forms() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF301_0812).unwrap().mnemonic,
        Mnemonic::VCEQ
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF312_0854).unwrap().mnemonic,
        Mnemonic::VCEQ
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF204_3305).unwrap().mnemonic,
        Mnemonic::VCGT
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF318_434C).unwrap().mnemonic,
        Mnemonic::VCGT
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF228_631A).unwrap().mnemonic,
        Mnemonic::VCGE
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF30C_8370).unwrap().mnemonic,
        Mnemonic::VCGE
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF201_0812).unwrap().mnemonic,
        Mnemonic::VTST
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF212_0854).unwrap().mnemonic,
        Mnemonic::VTST
    );

    cpu.vfp.write_d_bits(1, 0x0700_80ff_0403_0201);
    cpu.vfp.write_d_bits(2, 0x0701_7fff_0503_0001);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF301_0812),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0xff00_00ff_00ff_00ff);

    cpu.vfp.write_d_bits(2, 0x8000_ffff_0002_0001);
    cpu.vfp.write_d_bits(3, 0x000a_0009_0008_0007);
    cpu.vfp.write_d_bits(4, 0x7fff_ffff_0003_0001);
    cpu.vfp.write_d_bits(5, 0x000a_000a_0008_0000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF312_0854),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0000_ffff_0000_ffff);
    assert_eq!(cpu.vfp.read_d_bits(1), 0xffff_0000_ffff_0000);

    cpu.vfp.write_d_bits(4, 0x0afb_0500_7f80_ff01);
    cpu.vfp.write_d_bits(5, 0x0afa_0600_7e7f_fe00);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF204_3305),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(3), 0x00ff_0000_ff00_ffff);

    cpu.vfp.write_d_bits(8, 0x8000_0000_ffff_0002);
    cpu.vfp.write_d_bits(9, 0x0005_7fff_0064_0001);
    cpu.vfp.write_d_bits(12, 0x8000_0001_fffe_0001);
    cpu.vfp.write_d_bits(13, 0x0005_8000_0063_0002);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF318_434C),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(4), 0x0000_0000_ffff_ffff);
    assert_eq!(cpu.vfp.read_d_bits(5), 0x0000_0000_ffff_0000);

    cpu.vfp.write_d_bits(8, 0xffff_ffff_0000_0001);
    cpu.vfp.write_d_bits(10, 0x0000_0000_0000_0001);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF228_631A),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(6), 0x0000_0000_ffff_ffff);

    cpu.vfp.write_d_bits(12, 0x0907_0780_ff02_0100);
    cpu.vfp.write_d_bits(13, 0x1110_0f0e_0d0c_0b0a);
    cpu.vfp.write_d_bits(16, 0x0908_0781_fe01_0200);
    cpu.vfp.write_d_bits(17, 0x1011_0f0d_0e0c_0a0b);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF30C_8370),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(8), 0xff00_ff00_ffff_00ff);
    assert_eq!(cpu.vfp.read_d_bits(9), 0xff00_ffff_00ff_ff00);

    cpu.vfp.write_d_bits(1, 0x0100_0f01_8001_00f0);
    cpu.vfp.write_d_bits(2, 0x0001_f001_80ff_000f);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF201_0812),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0000_00ff_ffff_0000);

    cpu.vfp.write_d_bits(2, 0x0001_0001_8000_00f0);
    cpu.vfp.write_d_bits(3, 0x00ff_0101_0000_0000);
    cpu.vfp.write_d_bits(4, 0x0002_0001_8001_000f);
    cpu.vfp.write_d_bits(5, 0xff00_0100_ffff_0000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF212_0854),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0000_ffff_ffff_0000);
    assert_eq!(cpu.vfp.read_d_bits(1), 0x0000_ffff_0000_0000);

    assert_eq!(
        Aarch32Decoder::decode(0xF312_1854).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    let invalid_size = DecodedInsn::new(Mnemonic::VCGT, ExecutionState::Aarch32, 0xF234_3305, 4);
    assert!(matches!(
        Executor::new(&mut cpu, &mut mem).execute(&invalid_size),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_fp_compare_writes_f32_and_f16_lane_masks_and_handles_absolute_forms() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF201_0E02).unwrap().mnemonic,
        Mnemonic::VCEQ
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF211_0E02).unwrap().mnemonic,
        Mnemonic::VCEQ
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF202_0E44).unwrap().mnemonic,
        Mnemonic::VCEQ
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF304_3E05).unwrap().mnemonic,
        Mnemonic::VCGE
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF311_0E02).unwrap().mnemonic,
        Mnemonic::VCGE
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF308_4E4C).unwrap().mnemonic,
        Mnemonic::VCGE
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF328_6E0A).unwrap().mnemonic,
        Mnemonic::VCGT
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF331_0E02).unwrap().mnemonic,
        Mnemonic::VCGT
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF32C_8E60).unwrap().mnemonic,
        Mnemonic::VCGT
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF301_0E12).unwrap().mnemonic,
        Mnemonic::VACGE
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF311_0E12).unwrap().mnemonic,
        Mnemonic::VACGE
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF302_0E54).unwrap().mnemonic,
        Mnemonic::VACGE
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF324_3E15).unwrap().mnemonic,
        Mnemonic::VACGT
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF331_0E12).unwrap().mnemonic,
        Mnemonic::VACGT
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF328_4E5C).unwrap().mnemonic,
        Mnemonic::VACGT
    );

    cpu.vfp.write_d_bits(
        1,
        ((-0.0f32).to_bits() as u64) << 32 | u64::from(1.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        2,
        u64::from(0.0f32.to_bits()) << 32 | u64::from(1.0f32.to_bits()),
    );
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF201_0E02),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0xffff_ffff_ffff_ffff);

    cpu.vfp.write_d_bits(
        4,
        u64::from(f32::NAN.to_bits()) << 32 | u64::from((-1.0f32).to_bits()),
    );
    cpu.vfp.write_d_bits(
        5,
        u64::from(1.0f32.to_bits()) << 32 | u64::from((-2.0f32).to_bits()),
    );
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF304_3E05),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(3), 0x0000_0000_ffff_ffff);

    cpu.vfp.write_d_bits(
        12,
        u64::from(2.0f32.to_bits()) << 32 | u64::from(3.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        13,
        u64::from(f32::NAN.to_bits()) << 32 | u64::from((-1.0f32).to_bits()),
    );
    cpu.vfp.write_d_bits(
        16,
        u64::from(2.0f32.to_bits()) << 32 | u64::from(2.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        17,
        u64::from(1.0f32.to_bits()) << 32 | u64::from((-2.0f32).to_bits()),
    );
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF32C_8E60),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(8), 0x0000_0000_ffff_ffff);
    assert_eq!(cpu.vfp.read_d_bits(9), 0x0000_0000_ffff_ffff);

    cpu.vfp.write_d_bits(
        1,
        u64::from(1.0f32.to_bits()) << 32 | u64::from((-3.0f32).to_bits()),
    );
    cpu.vfp.write_d_bits(
        2,
        u64::from((-2.0f32).to_bits()) << 32 | u64::from(2.0f32.to_bits()),
    );
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF301_0E12),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0000_0000_ffff_ffff);

    cpu.vfp.write_d_bits(
        8,
        u64::from(4.0f32.to_bits()) << 32 | u64::from((-5.0f32).to_bits()),
    );
    cpu.vfp.write_d_bits(
        9,
        u64::from(f32::NAN.to_bits()) << 32 | u64::from((-1.0f32).to_bits()),
    );
    cpu.vfp.write_d_bits(
        12,
        u64::from((-4.0f32).to_bits()) << 32 | u64::from(4.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        13,
        u64::from(0.0f32.to_bits()) << 32 | u64::from(2.0f32.to_bits()),
    );
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF328_4E5C),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(4), 0x0000_0000_ffff_ffff);
    assert_eq!(cpu.vfp.read_d_bits(5), 0);

    cpu.vfp.write_d_bits(1, 0x7e00_4200_bc00_3c00);
    cpu.vfp.write_d_bits(2, 0x3c00_4200_3c00_3c00);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF211_0E02),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0000_ffff_0000_ffff);

    cpu.vfp.write_d_bits(1, 0x4000_4200_bc00_3c00);
    cpu.vfp.write_d_bits(2, 0x4200_4200_3c00_4000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF311_0E02),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0000_ffff_0000_0000);

    cpu.vfp.write_d_bits(1, 0x4400_4200_4000_3c00);
    cpu.vfp.write_d_bits(2, 0x4200_4200_3c00_4000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF331_0E02),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0xffff_0000_ffff_0000);

    cpu.vfp.write_d_bits(1, 0x4400_c200_bc00_3c00);
    cpu.vfp.write_d_bits(2, 0x4200_c000_4000_4000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF311_0E12),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0xffff_ffff_0000_0000);

    cpu.vfp.write_d_bits(1, 0x4400_c200_bc00_3c00);
    cpu.vfp.write_d_bits(2, 0x4200_c200_4000_4000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF331_0E12),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0xffff_0000_0000_0000);

    assert_eq!(
        Aarch32Decoder::decode(0xF302_1E54).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    let invalid_shape = DecodedInsn::new(Mnemonic::VCEQ, ExecutionState::Aarch32, 0xF211_0E12, 4);
    assert!(matches!(
        Executor::new(&mut cpu, &mut mem).execute(&invalid_shape),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_compare_zero_writes_integer_and_fp_lane_masks() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF3B1_0101).unwrap().mnemonic,
        Mnemonic::VCEQ
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B5_0142).unwrap().mnemonic,
        Mnemonic::VCEQ
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B1_2083).unwrap().mnemonic,
        Mnemonic::VCGE
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B5_4048).unwrap().mnemonic,
        Mnemonic::VCGT
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B9_6187).unwrap().mnemonic,
        Mnemonic::VCLE
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B1_824A).unwrap().mnemonic,
        Mnemonic::VCLT
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B9_0401).unwrap().mnemonic,
        Mnemonic::VCGT
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B5_0481).unwrap().mnemonic,
        Mnemonic::VCGE
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B9_0501).unwrap().mnemonic,
        Mnemonic::VCEQ
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B5_0581).unwrap().mnemonic,
        Mnemonic::VCLE
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B9_0601).unwrap().mnemonic,
        Mnemonic::VCLT
    );

    cpu.vfp.write_d_bits(1, 0x0300_0280_00ff_0100);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B1_0101),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x00ff_0000_ff00_00ff);

    cpu.vfp.write_d_bits(2, 0x0000_ffff_0001_0000);
    cpu.vfp.write_d_bits(3, 0xfffe_0002_0000_8000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B5_0142),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0xffff_0000_0000_ffff);
    assert_eq!(cpu.vfp.read_d_bits(1), 0x0000_0000_ffff_0000);

    cpu.vfp.write_d_bits(3, 0x03fe_027f_80ff_0100);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B1_2083),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(2), 0xff00_ffff_0000_ffff);

    cpu.vfp.write_d_bits(8, 0x0002_ffff_0001_0000);
    cpu.vfp.write_d_bits(9, 0x0003_8000_7fff_fffe);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B5_4048),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(4), 0xffff_0000_ffff_0000);
    assert_eq!(cpu.vfp.read_d_bits(5), 0xffff_0000_ffff_0000);

    cpu.vfp.write_d_bits(7, 0xffff_ffff_0000_0000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B9_6187),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(6), 0xffff_ffff_ffff_ffff);

    cpu.vfp.write_d_bits(10, 0x0002_fe7f_8001_00ff);
    cpu.vfp.write_d_bits(11, 0x0005_fb04_fc00_fd03);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B1_824A),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(8), 0x0000_ff00_ff00_00ff);
    assert_eq!(cpu.vfp.read_d_bits(9), 0x0000_ff00_ff00_ff00);

    cpu.vfp.write_d_bits(
        1,
        u64::from(f32::NAN.to_bits()) << 32 | u64::from(1.0f32.to_bits()),
    );
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B9_0401),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0000_0000_ffff_ffff);

    cpu.vfp.write_d_bits(
        1,
        u64::from(0.0f32.to_bits()) << 32 | u64::from((-0.0f32).to_bits()),
    );
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B9_0501),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0xffff_ffff_ffff_ffff);

    cpu.vfp.write_d_bits(1, 0x7e00_8000_0000_3c00);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B5_0401),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0000_0000_0000_ffff);

    cpu.vfp.write_d_bits(1, 0x7e00_8000_0000_3c00);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B5_0481),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0000_ffff_ffff_ffff);

    cpu.vfp.write_d_bits(1, 0x7e00_8000_0000_3c00);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B5_0581),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0000_ffff_ffff_0000);

    cpu.vfp.write_d_bits(1, 0x7e00_8000_0000_3c00);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B5_0601),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF3B5_1142).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    let invalid_size = DecodedInsn::new(Mnemonic::VCEQ, ExecutionState::Aarch32, 0xF3BD_0101, 4);
    assert!(matches!(
        Executor::new(&mut cpu, &mut mem).execute(&invalid_size),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_recip_estimate_handles_unsigned_f32_and_f16_forms() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF3BB_0401).unwrap().mnemonic,
        Mnemonic::VRECPE
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3BB_0442).unwrap().mnemonic,
        Mnemonic::VRECPE
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3BB_2483).unwrap().mnemonic,
        Mnemonic::VRSQRTE
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3BB_44C8).unwrap().mnemonic,
        Mnemonic::VRSQRTE
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3BB_4505).unwrap().mnemonic,
        Mnemonic::VRECPE
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B7_0501).unwrap().mnemonic,
        Mnemonic::VRECPE
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3BB_854A).unwrap().mnemonic,
        Mnemonic::VRECPE
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3BB_6587).unwrap().mnemonic,
        Mnemonic::VRSQRTE
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3B7_0581).unwrap().mnemonic,
        Mnemonic::VRSQRTE
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3BB_C5CE).unwrap().mnemonic,
        Mnemonic::VRSQRTE
    );

    cpu.vfp.write_d_bits(1, 0x4000_0000_8000_0000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3BB_0401),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0xffff_ffff_ff80_0000);

    cpu.vfp.write_d_bits(2, 0x9000_0000_ffff_ffff);
    cpu.vfp.write_d_bits(3, 0x4000_0000_8000_0000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3BB_0442),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0xe300_0000_8000_0000);
    assert_eq!(cpu.vfp.read_d_bits(1), 0xffff_ffff_ff80_0000);

    cpu.vfp.write_d_bits(3, 0x1000_0000_4000_0000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3BB_2483),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(2), 0xffff_ffff_ff80_0000);

    cpu.vfp.write_d_bits(8, 0x8000_0000_ffff_ffff);
    cpu.vfp.write_d_bits(9, 0x1000_0000_4000_0000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3BB_44C8),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(4), 0xb480_0000_8000_0000);
    assert_eq!(cpu.vfp.read_d_bits(5), 0xffff_ffff_ff80_0000);

    cpu.vfp.write_d_bits(5, 0x4080_0000_4000_0000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3BB_4505),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(4), 0x3e7f_8000_3eff_8000);

    cpu.vfp.write_d_bits(10, 0x7f80_0000_0000_0000);
    cpu.vfp.write_d_bits(11, 0x4080_0000_4000_0000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3BB_854A),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(8), 0x0000_0000_7f80_0000);
    assert_eq!(cpu.vfp.read_d_bits(9), 0x3e7f_8000_3eff_8000);

    cpu.vfp.write_d_bits(7, 0xbf80_0000_4000_0000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3BB_6587),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(6), 0x7fc0_0000_3f34_8000);

    cpu.vfp.write_d_bits(14, 0x7f80_0000_0000_0000);
    cpu.vfp.write_d_bits(15, 0xbf80_0000_4000_0000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3BB_C5CE),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(12), 0x0000_0000_7f80_0000);
    assert_eq!(cpu.vfp.read_d_bits(13), 0x7fc0_0000_3f34_8000);

    cpu.vfp.write_d_bits(1, 0x0000_7c00_4400_4000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B7_0501),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x7c00_0000_33fc_37fc);

    cpu.vfp.write_d_bits(1, 0xbc00_7c00_4400_3c00);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3B7_0581),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x7e00_0000_37fc_3bfc);

    assert_eq!(
        Aarch32Decoder::decode(0xF3BB_1442).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    let invalid_size = DecodedInsn::new(Mnemonic::VRECPE, ExecutionState::Aarch32, 0xF3B7_0401, 4);
    assert!(matches!(
        Executor::new(&mut cpu, &mut mem).execute(&invalid_size),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_recip_step_handles_f32_and_f16_fused_step_forms() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF201_0F12).unwrap().mnemonic,
        Mnemonic::VRECPS
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF211_0F12).unwrap().mnemonic,
        Mnemonic::VRECPS
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF202_0F54).unwrap().mnemonic,
        Mnemonic::VRECPS
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF224_3F15).unwrap().mnemonic,
        Mnemonic::VRSQRTS
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF231_0F12).unwrap().mnemonic,
        Mnemonic::VRSQRTS
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF228_4F5C).unwrap().mnemonic,
        Mnemonic::VRSQRTS
    );

    cpu.vfp.write_d_bits(
        1,
        u64::from(f32::INFINITY.to_bits()) << 32 | u64::from(0.5f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        2,
        u64::from(0.0f32.to_bits()) << 32 | u64::from(2.0f32.to_bits()),
    );
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF201_0F12),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(0),
        u64::from(2.0f32.to_bits()) << 32 | u64::from(1.0f32.to_bits())
    );

    cpu.vfp.write_d_bits(
        2,
        u64::from(4.0f32.to_bits()) << 32 | u64::from(1.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        3,
        u64::from(f32::NAN.to_bits()) << 32 | u64::from(0.25f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        4,
        u64::from(0.25f32.to_bits()) << 32 | u64::from(1.5f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        5,
        u64::from(1.0f32.to_bits()) << 32 | u64::from(0.5f32.to_bits()),
    );
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF202_0F54),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(0),
        u64::from(1.0f32.to_bits()) << 32 | u64::from(0.5f32.to_bits())
    );
    assert_eq!(
        cpu.vfp.read_d_bits(1),
        u64::from(f32::NAN.to_bits()) << 32 | u64::from(1.875f32.to_bits())
    );

    cpu.vfp.write_d_bits(
        4,
        u64::from(f32::INFINITY.to_bits()) << 32 | u64::from(1.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        5,
        u64::from(0.0f32.to_bits()) << 32 | u64::from(1.0f32.to_bits()),
    );
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF224_3F15),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(3),
        u64::from(1.5f32.to_bits()) << 32 | u64::from(1.0f32.to_bits())
    );

    cpu.vfp.write_d_bits(
        8,
        u64::from(0.5f32.to_bits()) << 32 | u64::from(1.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        9,
        u64::from(f32::NAN.to_bits()) << 32 | u64::from(2.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        12,
        u64::from(2.0f32.to_bits()) << 32 | u64::from(1.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        13,
        u64::from(1.0f32.to_bits()) << 32 | u64::from(0.5f32.to_bits()),
    );
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF228_4F5C),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(4),
        u64::from(1.0f32.to_bits()) << 32 | u64::from(1.0f32.to_bits())
    );
    assert_eq!(
        cpu.vfp.read_d_bits(5),
        u64::from(f32::NAN.to_bits()) << 32 | u64::from(1.0f32.to_bits())
    );

    cpu.vfp.write_d_bits(1, 0x4000_3c00_7c00_3800);
    cpu.vfp.write_d_bits(2, 0x3800_3c00_0000_4000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF211_0F12),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x3c00_3c00_4000_3c00);

    cpu.vfp.write_d_bits(1, 0x4000_3800_7c00_3c00);
    cpu.vfp.write_d_bits(2, 0x3800_4000_0000_3c00);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF231_0F12),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x3c00_3c00_3e00_3c00);

    assert_eq!(
        Aarch32Decoder::decode(0xF202_1F54).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    let invalid_shape = DecodedInsn::new(Mnemonic::VRECPS, ExecutionState::Aarch32, 0xF301_0F12, 4);
    assert!(matches!(
        Executor::new(&mut cpu, &mut mem).execute(&invalid_shape),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_fp_multiply_accumulate_and_subtract_handle_f32_and_f16_lanes() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF301_0D12).unwrap().mnemonic,
        Mnemonic::VMUL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF311_0D12).unwrap().mnemonic,
        Mnemonic::VMUL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF302_0D54).unwrap().mnemonic,
        Mnemonic::VMUL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF2A1_0942).unwrap().mnemonic,
        Mnemonic::VMUL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF291_0962).unwrap().mnemonic,
        Mnemonic::VMUL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF205_4D16).unwrap().mnemonic,
        Mnemonic::VMLA
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF211_0D12).unwrap().mnemonic,
        Mnemonic::VMLA
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF3A2_0164).unwrap().mnemonic,
        Mnemonic::VMLA
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF291_0142).unwrap().mnemonic,
        Mnemonic::VMLA
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF392_0144).unwrap().mnemonic,
        Mnemonic::VMLA
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF262_0DF4).unwrap().mnemonic,
        Mnemonic::VMLS
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF231_0D12).unwrap().mnemonic,
        Mnemonic::VMLS
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF2A1_0542).unwrap().mnemonic,
        Mnemonic::VMLS
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF291_054A).unwrap().mnemonic,
        Mnemonic::VMLS
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF201_0C12).unwrap().mnemonic,
        Mnemonic::VFMA
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF225_4C16).unwrap().mnemonic,
        Mnemonic::VFMS
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF202_0C54).unwrap().mnemonic,
        Mnemonic::VFMA
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF22A_8C5C).unwrap().mnemonic,
        Mnemonic::VFMS
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF302_1D54).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF202_1C54).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.vfp.write_d_bits(
        1,
        u64::from(3.0f32.to_bits()) << 32 | u64::from(2.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        2,
        u64::from(5.0f32.to_bits()) << 32 | u64::from(4.0f32.to_bits()),
    );
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF301_0D12),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(0),
        u64::from(15.0f32.to_bits()) << 32 | u64::from(8.0f32.to_bits())
    );

    cpu.vfp.write_d_bits(
        2,
        u64::from(3.0f32.to_bits()) << 32 | u64::from(2.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        3,
        u64::from(5.0f32.to_bits()) << 32 | u64::from(4.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        4,
        u64::from(7.0f32.to_bits()) << 32 | u64::from(6.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        5,
        u64::from(9.0f32.to_bits()) << 32 | u64::from(8.0f32.to_bits()),
    );
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF302_0D54),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(0),
        u64::from(21.0f32.to_bits()) << 32 | u64::from(12.0f32.to_bits())
    );
    assert_eq!(
        cpu.vfp.read_d_bits(1),
        u64::from(45.0f32.to_bits()) << 32 | u64::from(32.0f32.to_bits())
    );

    cpu.vfp.write_d_bits(
        1,
        u64::from(3.0f32.to_bits()) << 32 | u64::from(2.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        2,
        u64::from(5.0f32.to_bits()) << 32 | u64::from(4.0f32.to_bits()),
    );
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF2A1_0942),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(0),
        u64::from(12.0f32.to_bits()) << 32 | u64::from(8.0f32.to_bits())
    );

    cpu.vfp.write_d_bits(
        4,
        u64::from(20.0f32.to_bits()) << 32 | u64::from(10.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        5,
        u64::from(4.0f32.to_bits()) << 32 | u64::from(3.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        6,
        u64::from(2.0f32.to_bits()) << 32 | u64::from(5.0f32.to_bits()),
    );
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF205_4D16),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(4),
        u64::from(28.0f32.to_bits()) << 32 | u64::from(25.0f32.to_bits())
    );

    cpu.vfp.write_d_bits(
        0,
        u64::from(20.0f32.to_bits()) << 32 | u64::from(10.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        1,
        u64::from(40.0f32.to_bits()) << 32 | u64::from(30.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        2,
        u64::from(2.0f32.to_bits()) << 32 | u64::from(1.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        3,
        u64::from(4.0f32.to_bits()) << 32 | u64::from(3.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        4,
        u64::from(5.0f32.to_bits()) << 32 | u64::from(6.0f32.to_bits()),
    );
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF3A2_0164),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(0),
        u64::from(30.0f32.to_bits()) << 32 | u64::from(15.0f32.to_bits())
    );
    assert_eq!(
        cpu.vfp.read_d_bits(1),
        u64::from(60.0f32.to_bits()) << 32 | u64::from(45.0f32.to_bits())
    );

    cpu.vfp.write_d_bits(
        16,
        u64::from(200.0f32.to_bits()) << 32 | u64::from(100.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        17,
        u64::from(400.0f32.to_bits()) << 32 | u64::from(300.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        18,
        u64::from(4.0f32.to_bits()) << 32 | u64::from(3.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        19,
        u64::from(6.0f32.to_bits()) << 32 | u64::from(5.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        20,
        u64::from(2.0f32.to_bits()) << 32 | u64::from(7.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        21,
        u64::from(8.0f32.to_bits()) << 32 | u64::from(9.0f32.to_bits()),
    );
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF262_0DF4),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(16),
        u64::from(192.0f32.to_bits()) << 32 | u64::from(79.0f32.to_bits())
    );
    assert_eq!(
        cpu.vfp.read_d_bits(17),
        u64::from(352.0f32.to_bits()) << 32 | u64::from(255.0f32.to_bits())
    );

    cpu.vfp.write_d_bits(
        0,
        u64::from(30.0f32.to_bits()) << 32 | u64::from(20.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        1,
        u64::from(3.0f32.to_bits()) << 32 | u64::from(2.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        2,
        u64::from(5.0f32.to_bits()) << 32 | u64::from(4.0f32.to_bits()),
    );
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF2A1_0542),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(0),
        u64::from(18.0f32.to_bits()) << 32 | u64::from(12.0f32.to_bits())
    );

    cpu.vfp.write_d_bits(
        0,
        u64::from(50.0f32.to_bits()) << 32 | u64::from(10.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        1,
        u64::from(3.0f32.to_bits()) << 32 | u64::from(2.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        2,
        u64::from(5.0f32.to_bits()) << 32 | u64::from(4.0f32.to_bits()),
    );
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF201_0C12),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(0),
        u64::from(65.0f32.to_bits()) << 32 | u64::from(18.0f32.to_bits())
    );

    cpu.vfp.write_d_bits(
        4,
        u64::from(80.0f32.to_bits()) << 32 | u64::from(50.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        5,
        u64::from(4.0f32.to_bits()) << 32 | u64::from(3.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        6,
        u64::from(2.0f32.to_bits()) << 32 | u64::from(5.0f32.to_bits()),
    );
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF225_4C16),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(4),
        u64::from(72.0f32.to_bits()) << 32 | u64::from(35.0f32.to_bits())
    );

    cpu.vfp.write_d_bits(
        0,
        u64::from(20.0f32.to_bits()) << 32 | u64::from(10.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        1,
        u64::from(40.0f32.to_bits()) << 32 | u64::from(30.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        2,
        u64::from(2.0f32.to_bits()) << 32 | u64::from(1.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        3,
        u64::from(4.0f32.to_bits()) << 32 | u64::from(3.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        4,
        u64::from(6.0f32.to_bits()) << 32 | u64::from(5.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        5,
        u64::from(8.0f32.to_bits()) << 32 | u64::from(7.0f32.to_bits()),
    );
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF202_0C54),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(0),
        u64::from(32.0f32.to_bits()) << 32 | u64::from(15.0f32.to_bits())
    );
    assert_eq!(
        cpu.vfp.read_d_bits(1),
        u64::from(72.0f32.to_bits()) << 32 | u64::from(51.0f32.to_bits())
    );

    cpu.vfp.write_d_bits(
        8,
        u64::from(120.0f32.to_bits()) << 32 | u64::from(100.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        9,
        u64::from(180.0f32.to_bits()) << 32 | u64::from(160.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        10,
        u64::from(2.0f32.to_bits()) << 32 | u64::from(1.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        11,
        u64::from(4.0f32.to_bits()) << 32 | u64::from(3.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        12,
        u64::from(6.0f32.to_bits()) << 32 | u64::from(5.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        13,
        u64::from(8.0f32.to_bits()) << 32 | u64::from(7.0f32.to_bits()),
    );
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF22A_8C5C),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(8),
        u64::from(108.0f32.to_bits()) << 32 | u64::from(95.0f32.to_bits())
    );
    assert_eq!(
        cpu.vfp.read_d_bits(9),
        u64::from(148.0f32.to_bits()) << 32 | u64::from(139.0f32.to_bits())
    );

    cpu.vfp.write_d_bits(1, 0x4400_4200_4000_3c00);
    cpu.vfp.write_d_bits(2, 0x4000_4000_4000_4000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF311_0D12),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x4800_4600_4400_4000);

    cpu.vfp.write_d_bits(0, 0x4400_4200_4000_3c00);
    cpu.vfp.write_d_bits(1, 0x4400_4200_4000_3c00);
    cpu.vfp.write_d_bits(2, 0x3c00_3c00_3c00_3c00);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF211_0D12),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x4800_4600_4400_4000);

    cpu.vfp.write_d_bits(0, 0x4400_4200_4000_3c00);
    cpu.vfp.write_d_bits(1, 0x4400_4200_4000_3c00);
    cpu.vfp.write_d_bits(2, 0x3c00_3c00_3c00_3c00);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF231_0D12),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0);

    cpu.vfp.write_d_bits(0, 0x4400_4200_4000_3c00);
    cpu.vfp.write_d_bits(1, 0x4400_4200_4000_3c00);
    cpu.vfp.write_d_bits(2, 0x4400_4200_4000_3c00);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF291_0142),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x4800_4600_4400_4000);

    cpu.vfp.write_d_bits(0, 0x4900_4800_4600_4400);
    cpu.vfp.write_d_bits(1, 0x4400_4200_4000_3c00);
    cpu.vfp.write_d_bits(2, 0x4400_4200_4000_3c00);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF291_054A),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x4000_4000_4000_4000);

    cpu.vfp.write_d_bits(1, 0x4400_4200_4000_3c00);
    cpu.vfp.write_d_bits(2, 0x4400_4200_4000_3c00);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF291_0962),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x4a00_4880_4600_4200);

    cpu.vfp.write_d_bits(0, 0x4400_4200_4000_3c00);
    cpu.vfp.write_d_bits(1, 0x4800_4700_4600_4500);
    cpu.vfp.write_d_bits(2, 0x4400_4200_4000_3c00);
    cpu.vfp.write_d_bits(3, 0x4800_4700_4600_4500);
    cpu.vfp.write_d_bits(4, 0x3c00_3c00_3c00_3c00);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF392_0144),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x4800_4600_4400_4000);
    assert_eq!(cpu.vfp.read_d_bits(1), 0x4c00_4b00_4a00_4900);
}

#[test]
fn neon_fp16_fused_multiply_long_widens_half_products_to_f32_accumulators() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);
    let pack_f32x2 =
        |low: f32, high: f32| u64::from(high.to_bits()) << 32 | u64::from(low.to_bits());

    assert_eq!(
        Aarch32Decoder::decode(0xFC22_0813).unwrap().mnemonic,
        Mnemonic::VFMAL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xFCA2_0813).unwrap().mnemonic,
        Mnemonic::VFMLS
    );
    assert_eq!(
        Aarch32Decoder::decode(0xFE02_0813).unwrap().mnemonic,
        Mnemonic::VFMAL
    );
    assert_eq!(
        Aarch32Decoder::decode(0xFE12_0813).unwrap().mnemonic,
        Mnemonic::VFMLS
    );
    assert_eq!(
        Aarch32Decoder::decode(0xFC20_1850).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    assert_eq!(
        Aarch32Decoder::decode(0xFE00_1850).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.vfp.write_d_bits(0, pack_f32x2(10.0, 20.0));
    cpu.vfp.write_s_bits(4, 0x4000_3c00);
    cpu.vfp.write_s_bits(6, 0x4400_4200);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFC22_0813),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), pack_f32x2(13.0, 28.0));

    cpu.vfp.write_d_bits(0, pack_f32x2(10.0, 20.0));
    cpu.vfp.write_s_bits(4, 0x4000_3c00);
    cpu.vfp.write_s_bits(6, 0x4400_4200);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFCA2_0813),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), pack_f32x2(7.0, 12.0));

    cpu.vfp.write_d_bits(2, pack_f32x2(10.0, 20.0));
    cpu.vfp.write_d_bits(3, pack_f32x2(30.0, 40.0));
    cpu.vfp.write_d_bits(4, 0x4400_4200_4000_3c00);
    cpu.vfp.write_d_bits(6, 0x4800_4600_4400_4200);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFC24_2856),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(2), pack_f32x2(13.0, 28.0));
    assert_eq!(cpu.vfp.read_d_bits(3), pack_f32x2(48.0, 72.0));

    cpu.vfp.write_d_bits(0, pack_f32x2(10.0, 20.0));
    cpu.vfp.write_s_bits(4, 0x4000_3c00);
    cpu.vfp.write_s_bits(6, 0x4400_4200);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFE02_0813),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), pack_f32x2(13.0, 26.0));

    cpu.vfp.write_d_bits(0, pack_f32x2(10.0, 20.0));
    cpu.vfp.write_s_bits(4, 0x4000_3c00);
    cpu.vfp.write_s_bits(6, 0x4400_4200);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFE12_0813),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), pack_f32x2(7.0, 14.0));

    cpu.vfp.write_d_bits(2, pack_f32x2(10.0, 20.0));
    cpu.vfp.write_d_bits(3, pack_f32x2(30.0, 40.0));
    cpu.vfp.write_d_bits(4, 0x4400_4200_4000_3c00);
    cpu.vfp.write_d_bits(6, 0x4800_4600_4400_4200);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xFE04_2876),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(2), pack_f32x2(16.0, 32.0));
    assert_eq!(cpu.vfp.read_d_bits(3), pack_f32x2(48.0, 64.0));
}

#[test]
fn neon_fp_add_sub_handle_f32_and_f16_lanes() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF201_0D02).unwrap().mnemonic,
        Mnemonic::VADD
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF211_0D02).unwrap().mnemonic,
        Mnemonic::VADD
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF202_0D44).unwrap().mnemonic,
        Mnemonic::VADD
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF225_4D06).unwrap().mnemonic,
        Mnemonic::VSUB
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF232_0D44).unwrap().mnemonic,
        Mnemonic::VSUB
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF22A_8D4C).unwrap().mnemonic,
        Mnemonic::VSUB
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF202_1D44).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.vfp.write_d_bits(
        1,
        u64::from(3.5f32.to_bits()) << 32 | u64::from(1.25f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        2,
        u64::from(4.5f32.to_bits()) << 32 | u64::from(2.75f32.to_bits()),
    );
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF201_0D02),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(0),
        u64::from(8.0f32.to_bits()) << 32 | u64::from(4.0f32.to_bits())
    );

    cpu.vfp.write_d_bits(
        2,
        u64::from(4.0f32.to_bits()) << 32 | u64::from(1.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        3,
        u64::from(8.0f32.to_bits()) << 32 | u64::from(2.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        4,
        u64::from(40.0f32.to_bits()) << 32 | u64::from(10.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        5,
        u64::from(80.0f32.to_bits()) << 32 | u64::from(20.0f32.to_bits()),
    );
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF202_0D44),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(0),
        u64::from(44.0f32.to_bits()) << 32 | u64::from(11.0f32.to_bits())
    );
    assert_eq!(
        cpu.vfp.read_d_bits(1),
        u64::from(88.0f32.to_bits()) << 32 | u64::from(22.0f32.to_bits())
    );

    cpu.vfp.write_d_bits(
        5,
        u64::from(9.0f32.to_bits()) << 32 | u64::from(7.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        6,
        u64::from(4.0f32.to_bits()) << 32 | u64::from(2.5f32.to_bits()),
    );
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF225_4D06),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(4),
        u64::from(5.0f32.to_bits()) << 32 | u64::from(4.5f32.to_bits())
    );

    cpu.vfp.write_d_bits(
        10,
        u64::from(100.0f32.to_bits()) << 32 | u64::from(50.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        11,
        u64::from(200.0f32.to_bits()) << 32 | u64::from(150.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        12,
        u64::from(8.0f32.to_bits()) << 32 | u64::from(7.0f32.to_bits()),
    );
    cpu.vfp.write_d_bits(
        13,
        u64::from(6.0f32.to_bits()) << 32 | u64::from(5.0f32.to_bits()),
    );
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF22A_8D4C),
        ExecResult::Continue
    ));
    assert_eq!(
        cpu.vfp.read_d_bits(8),
        u64::from(92.0f32.to_bits()) << 32 | u64::from(43.0f32.to_bits())
    );
    assert_eq!(
        cpu.vfp.read_d_bits(9),
        u64::from(194.0f32.to_bits()) << 32 | u64::from(145.0f32.to_bits())
    );

    cpu.vfp.write_d_bits(1, 0x4400_4200_4000_3c00);
    cpu.vfp.write_d_bits(2, 0x4200_4000_3c00_3800);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF211_0D02),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x4700_4500_4200_3e00);

    cpu.vfp.write_d_bits(2, 0x4800_4600_4400_4000);
    cpu.vfp.write_d_bits(3, 0x4c00_4b00_4a00_4900);
    cpu.vfp.write_d_bits(4, 0x4400_4200_4000_3c00);
    cpu.vfp.write_d_bits(5, 0x4800_4600_4400_4000);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF232_0D44),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x4400_4200_4000_3c00);
    assert_eq!(cpu.vfp.read_d_bits(1), 0x4800_4800_4800_4800);
}

#[test]
fn neon_integer_add_sub_register_ops_cover_element_sizes_and_q_forms() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF201_0802).unwrap().mnemonic,
        Mnemonic::VADD
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF314_3805).unwrap().mnemonic,
        Mnemonic::VSUB
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF222_0844).unwrap().mnemonic,
        Mnemonic::VADD
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF332_0844).unwrap().mnemonic,
        Mnemonic::VSUB
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF222_1844).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.vfp.write_d_bits(1, 0x0807_0605_0403_0201);
    cpu.vfp.write_d_bits(2, 0x0101_0101_0101_0101);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF201_0802),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0908_0706_0504_0302);

    cpu.vfp.write_d_bits(4, 0x0001_0000_8000_ffff);
    cpu.vfp.write_d_bits(5, 0x0001_0001_0001_0001);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF314_3805),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(3), 0x0000_ffff_7fff_fffe);

    cpu.vfp.write_d_bits(2, 0xffff_fff0_0000_0001);
    cpu.vfp.write_d_bits(3, 0x8000_0000_7fff_ffff);
    cpu.vfp.write_d_bits(4, 0x0000_0020_ffff_ffff);
    cpu.vfp.write_d_bits(5, 0x8000_0000_0000_0001);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF222_0844),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0000_0010_0000_0000);
    assert_eq!(cpu.vfp.read_d_bits(1), 0x0000_0000_8000_0000);

    cpu.vfp.write_d_bits(2, 0x0000_0000_0000_0010);
    cpu.vfp.write_d_bits(3, 0x0000_0000_0000_0000);
    cpu.vfp.write_d_bits(4, 0x0000_0000_0000_0001);
    cpu.vfp.write_d_bits(5, 0x0000_0000_0000_0001);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF332_0844),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0000_0000_0000_000f);
    assert_eq!(cpu.vfp.read_d_bits(1), 0xffff_ffff_ffff_ffff);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF222_1844),
        ExecResult::Undefined
    ));
}

#[test]
fn neon_vld1_vst1_multiple_transfer_d_register_bytes_and_writeback() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF401_0A0D).unwrap().mnemonic,
        Mnemonic::VST1
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF421_2A0D).unwrap().mnemonic,
        Mnemonic::VLD1
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF401_072D).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.regs[1] = 0x100;
    cpu.vfp.write_d_bits(0, 0x0807_0605_0403_0201);
    cpu.vfp.write_d_bits(1, 0x1817_1615_1413_1211);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF401_0A0D),
        ExecResult::Continue
    ));
    assert_eq!(cpu.regs[1], 0x110);
    assert_eq!(mem.read_byte(0x100).unwrap(), 0x01);
    assert_eq!(mem.read_byte(0x107).unwrap(), 0x08);
    assert_eq!(mem.read_byte(0x108).unwrap(), 0x11);
    assert_eq!(mem.read_byte(0x10f).unwrap(), 0x18);

    cpu.regs[1] = 0x100;
    cpu.vfp.write_d_bits(2, 0);
    cpu.vfp.write_d_bits(3, 0);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF421_2A0D),
        ExecResult::Continue
    ));
    assert_eq!(cpu.regs[1], 0x110);
    assert_eq!(cpu.vfp.read_d_bits(2), 0x0807_0605_0403_0201);
    assert_eq!(cpu.vfp.read_d_bits(3), 0x1817_1615_1413_1211);

    cpu.regs[1] = 0x200;
    cpu.regs[2] = 0x40;
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF401_0702),
        ExecResult::Continue
    ));
    assert_eq!(cpu.regs[1], 0x240);
}

#[test]
fn neon_vld2_vst2_multiple_interleave_elements_and_writeback() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF401_080D).unwrap().mnemonic,
        Mnemonic::VST2
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF421_280D).unwrap().mnemonic,
        Mnemonic::VLD2
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF401_08CD).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.regs[1] = 0x100;
    cpu.vfp.write_d_bits(0, 0x1716_1514_1312_1110);
    cpu.vfp.write_d_bits(1, 0x2726_2524_2322_2120);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF401_080D),
        ExecResult::Continue
    ));
    assert_eq!(cpu.regs[1], 0x110);
    assert_eq!(mem.read_byte(0x100).unwrap(), 0x10);
    assert_eq!(mem.read_byte(0x101).unwrap(), 0x20);
    assert_eq!(mem.read_byte(0x102).unwrap(), 0x11);
    assert_eq!(mem.read_byte(0x10f).unwrap(), 0x27);

    cpu.regs[1] = 0x100;
    cpu.vfp.write_d_bits(2, 0);
    cpu.vfp.write_d_bits(3, 0);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF421_280D),
        ExecResult::Continue
    ));
    assert_eq!(cpu.regs[1], 0x110);
    assert_eq!(cpu.vfp.read_d_bits(2), 0x1716_1514_1312_1110);
    assert_eq!(cpu.vfp.read_d_bits(3), 0x2726_2524_2322_2120);

    cpu.regs[1] = 0x200;
    cpu.regs[2] = 0x44;
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF401_0302),
        ExecResult::Continue
    ));
    assert_eq!(cpu.regs[1], 0x244);
}

#[test]
fn neon_vld3_vst3_multiple_interleave_elements_and_writeback() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF401_040D).unwrap().mnemonic,
        Mnemonic::VST3
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF421_340D).unwrap().mnemonic,
        Mnemonic::VLD3
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF401_042D).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.regs[1] = 0x100;
    cpu.vfp.write_d_bits(0, 0x1716_1514_1312_1110);
    cpu.vfp.write_d_bits(1, 0x2726_2524_2322_2120);
    cpu.vfp.write_d_bits(2, 0x3736_3534_3332_3130);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF401_040D),
        ExecResult::Continue
    ));
    assert_eq!(cpu.regs[1], 0x118);
    assert_eq!(mem.read_byte(0x100).unwrap(), 0x10);
    assert_eq!(mem.read_byte(0x101).unwrap(), 0x20);
    assert_eq!(mem.read_byte(0x102).unwrap(), 0x30);
    assert_eq!(mem.read_byte(0x117).unwrap(), 0x37);

    cpu.regs[1] = 0x100;
    cpu.vfp.write_d_bits(3, 0);
    cpu.vfp.write_d_bits(4, 0);
    cpu.vfp.write_d_bits(5, 0);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF421_340D),
        ExecResult::Continue
    ));
    assert_eq!(cpu.regs[1], 0x118);
    assert_eq!(cpu.vfp.read_d_bits(3), 0x1716_1514_1312_1110);
    assert_eq!(cpu.vfp.read_d_bits(4), 0x2726_2524_2322_2120);
    assert_eq!(cpu.vfp.read_d_bits(5), 0x3736_3534_3332_3130);

    cpu.regs[1] = 0x200;
    cpu.regs[2] = 0x50;
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF401_0402),
        ExecResult::Continue
    ));
    assert_eq!(cpu.regs[1], 0x250);
}

#[test]
fn neon_vld4_vst4_multiple_interleave_elements_and_writeback() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF401_000D).unwrap().mnemonic,
        Mnemonic::VST4
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF421_400D).unwrap().mnemonic,
        Mnemonic::VLD4
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF401_00CD).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.regs[1] = 0x100;
    cpu.vfp.write_d_bits(0, 0x1716_1514_1312_1110);
    cpu.vfp.write_d_bits(1, 0x2726_2524_2322_2120);
    cpu.vfp.write_d_bits(2, 0x3736_3534_3332_3130);
    cpu.vfp.write_d_bits(3, 0x4746_4544_4342_4140);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF401_000D),
        ExecResult::Continue
    ));
    assert_eq!(cpu.regs[1], 0x120);
    assert_eq!(mem.read_byte(0x100).unwrap(), 0x10);
    assert_eq!(mem.read_byte(0x101).unwrap(), 0x20);
    assert_eq!(mem.read_byte(0x102).unwrap(), 0x30);
    assert_eq!(mem.read_byte(0x103).unwrap(), 0x40);
    assert_eq!(mem.read_byte(0x11f).unwrap(), 0x47);

    cpu.regs[1] = 0x100;
    cpu.vfp.write_d_bits(4, 0);
    cpu.vfp.write_d_bits(5, 0);
    cpu.vfp.write_d_bits(6, 0);
    cpu.vfp.write_d_bits(7, 0);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF421_400D),
        ExecResult::Continue
    ));
    assert_eq!(cpu.regs[1], 0x120);
    assert_eq!(cpu.vfp.read_d_bits(4), 0x1716_1514_1312_1110);
    assert_eq!(cpu.vfp.read_d_bits(5), 0x2726_2524_2322_2120);
    assert_eq!(cpu.vfp.read_d_bits(6), 0x3736_3534_3332_3130);
    assert_eq!(cpu.vfp.read_d_bits(7), 0x4746_4544_4342_4140);

    cpu.regs[1] = 0x200;
    cpu.regs[2] = 0x60;
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF401_0002),
        ExecResult::Continue
    ));
    assert_eq!(cpu.regs[1], 0x260);

    cpu.regs[1] = 0x300;
    cpu.vfp.write_d_bits(0, 0x0807_0605_0403_0201);
    cpu.vfp.write_d_bits(2, 0x1817_1615_1413_1211);
    cpu.vfp.write_d_bits(4, 0x2827_2625_2423_2221);
    cpu.vfp.write_d_bits(6, 0x3837_3635_3433_3231);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF401_010D),
        ExecResult::Continue
    ));
    assert_eq!(mem.read_byte(0x300).unwrap(), 0x01);
    assert_eq!(mem.read_byte(0x301).unwrap(), 0x11);
    assert_eq!(mem.read_byte(0x302).unwrap(), 0x21);
    assert_eq!(mem.read_byte(0x303).unwrap(), 0x31);
    assert_eq!(mem.read_byte(0x31f).unwrap(), 0x38);
}

#[test]
fn neon_vld_all_lanes_replicates_structures_and_writeback() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF4A1_0C2D).unwrap().mnemonic,
        Mnemonic::VLD1
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF4A1_0D6D).unwrap().mnemonic,
        Mnemonic::VLD2
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF4A1_0E1D).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF4A1_0FDD).unwrap().mnemonic,
        Mnemonic::VLD4
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF4A1_0FCD).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.regs[1] = 0x100;
    mem.write_byte(0x100, 0x42).unwrap();
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF4A1_0C2D),
        ExecResult::Continue
    ));
    assert_eq!(cpu.regs[1], 0x101);
    assert_eq!(cpu.vfp.read_d_bits(0), 0x4242_4242_4242_4242);
    assert_eq!(cpu.vfp.read_d_bits(1), 0x4242_4242_4242_4242);

    cpu.regs[1] = 0x120;
    mem.write_halfword(0x120, 0x1234).unwrap();
    mem.write_halfword(0x122, 0xabcd).unwrap();
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF4A1_0D6D),
        ExecResult::Continue
    ));
    assert_eq!(cpu.regs[1], 0x124);
    assert_eq!(cpu.vfp.read_d_bits(0), 0x1234_1234_1234_1234);
    assert_eq!(cpu.vfp.read_d_bits(2), 0xabcd_abcd_abcd_abcd);

    cpu.regs[1] = 0x140;
    cpu.regs[2] = 0x30;
    mem.write_byte(0x140, 0x11).unwrap();
    mem.write_byte(0x141, 0x22).unwrap();
    mem.write_byte(0x142, 0x33).unwrap();
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF4A1_0E02),
        ExecResult::Continue
    ));
    assert_eq!(cpu.regs[1], 0x170);
    assert_eq!(cpu.vfp.read_d_bits(0), 0x1111_1111_1111_1111);
    assert_eq!(cpu.vfp.read_d_bits(1), 0x2222_2222_2222_2222);
    assert_eq!(cpu.vfp.read_d_bits(2), 0x3333_3333_3333_3333);

    cpu.regs[1] = 0x180;
    mem.write_word(0x180, 0x0102_0304).unwrap();
    mem.write_word(0x184, 0x1112_1314).unwrap();
    mem.write_word(0x188, 0x2122_2324).unwrap();
    mem.write_word(0x18c, 0x3132_3334).unwrap();
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF4A1_0FDD),
        ExecResult::Continue
    ));
    assert_eq!(cpu.regs[1], 0x190);
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0102_0304_0102_0304);
    assert_eq!(cpu.vfp.read_d_bits(1), 0x1112_1314_1112_1314);
    assert_eq!(cpu.vfp.read_d_bits(2), 0x2122_2324_2122_2324);
    assert_eq!(cpu.vfp.read_d_bits(3), 0x3132_3334_3132_3334);
}

#[test]
fn neon_vld_vst_single_lane_transfer_selected_elements_and_writeback() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xF4A1_006D).unwrap().mnemonic,
        Mnemonic::VLD1
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF4A1_056D).unwrap().mnemonic,
        Mnemonic::VLD2
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF4A1_021D).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF481_0AC2).unwrap().mnemonic,
        Mnemonic::VST3
    );
    assert_eq!(
        Aarch32Decoder::decode(0xF481_0B3D).unwrap().mnemonic,
        Mnemonic::UNDEFINED
    );

    cpu.regs[1] = 0x100;
    cpu.vfp.write_d_bits(0, 0x0706_0504_0302_0100);
    mem.write_byte(0x100, 0xaa).unwrap();
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF4A1_006D),
        ExecResult::Continue
    ));
    assert_eq!(cpu.regs[1], 0x101);
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0706_0504_aa02_0100);

    cpu.regs[1] = 0x120;
    cpu.vfp.write_d_bits(0, 0x7777_6666_5555_4444);
    cpu.vfp.write_d_bits(2, 0xbbbb_aaaa_9999_8888);
    mem.write_halfword(0x120, 0x1234).unwrap();
    mem.write_halfword(0x122, 0xabcd).unwrap();
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF4A1_056D),
        ExecResult::Continue
    ));
    assert_eq!(cpu.regs[1], 0x124);
    assert_eq!(cpu.vfp.read_d_bits(0), 0x7777_6666_1234_4444);
    assert_eq!(cpu.vfp.read_d_bits(2), 0xbbbb_aaaa_abcd_8888);

    cpu.regs[1] = 0x200;
    cpu.regs[2] = 0x40;
    cpu.vfp.write_d_bits(0, 0x1111_2222_3333_4444);
    cpu.vfp.write_d_bits(2, 0x5555_6666_7777_8888);
    cpu.vfp.write_d_bits(4, 0x9999_aaaa_bbbb_cccc);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xF481_0AC2),
        ExecResult::Continue
    ));
    assert_eq!(cpu.regs[1], 0x240);
    assert_eq!(mem.read_word(0x200).unwrap(), 0x1111_2222);
    assert_eq!(mem.read_word(0x204).unwrap(), 0x5555_6666);
    assert_eq!(mem.read_word(0x208).unwrap(), 0x9999_aaaa);
}

#[test]
fn vldm_vstm_transfer_double_register_lists_and_writeback() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xEC81_0B06).unwrap().mnemonic,
        Mnemonic::VSTM
    );
    assert_eq!(
        Aarch32Decoder::decode(0xECB1_0B06).unwrap().mnemonic,
        Mnemonic::VLDM
    );

    cpu.regs[1] = 0x100;
    cpu.vfp.write_d_bits(0, 0x0123_4567_89ab_cdef);
    cpu.vfp.write_d_bits(1, 0xfedc_ba98_7654_3210);
    cpu.vfp.write_d_bits(2, 0x0bad_f00d_cafe_babe);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xECA1_0B06),
        ExecResult::Continue
    ));
    assert_eq!(cpu.regs[1], 0x118);
    assert_eq!(mem.read_word(0x100).unwrap(), 0x89ab_cdef);
    assert_eq!(mem.read_word(0x104).unwrap(), 0x0123_4567);
    assert_eq!(mem.read_word(0x110).unwrap(), 0xcafe_babe);
    assert_eq!(mem.read_word(0x114).unwrap(), 0x0bad_f00d);

    cpu.vfp.write_d_bits(0, 0);
    cpu.vfp.write_d_bits(1, 0);
    cpu.vfp.write_d_bits(2, 0);
    cpu.regs[1] = 0x100;
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xECB1_0B06),
        ExecResult::Continue
    ));
    assert_eq!(cpu.regs[1], 0x118);
    assert_eq!(cpu.vfp.read_d_bits(0), 0x0123_4567_89ab_cdef);
    assert_eq!(cpu.vfp.read_d_bits(1), 0xfedc_ba98_7654_3210);
    assert_eq!(cpu.vfp.read_d_bits(2), 0x0bad_f00d_cafe_babe);
}

#[test]
fn vpush_vpop_transfer_stack_register_lists() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xED2D_8B04).unwrap().mnemonic,
        Mnemonic::VPUSH
    );
    assert_eq!(
        Aarch32Decoder::decode(0xECBD_8B04).unwrap().mnemonic,
        Mnemonic::VPOP
    );

    cpu.regs[13] = 0x200;
    cpu.vfp.write_d_bits(8, 0x1111_2222_3333_4444);
    cpu.vfp.write_d_bits(9, 0x5555_6666_7777_8888);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xED2D_8B04),
        ExecResult::Continue
    ));
    assert_eq!(cpu.regs[13], 0x1f0);
    assert_eq!(mem.read_word(0x1f0).unwrap(), 0x3333_4444);
    assert_eq!(mem.read_word(0x1f4).unwrap(), 0x1111_2222);
    assert_eq!(mem.read_word(0x1f8).unwrap(), 0x7777_8888);
    assert_eq!(mem.read_word(0x1fc).unwrap(), 0x5555_6666);

    cpu.vfp.write_d_bits(8, 0);
    cpu.vfp.write_d_bits(9, 0);
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xECBD_8B04),
        ExecResult::Continue
    ));
    assert_eq!(cpu.regs[13], 0x200);
    assert_eq!(cpu.vfp.read_d_bits(8), 0x1111_2222_3333_4444);
    assert_eq!(cpu.vfp.read_d_bits(9), 0x5555_6666_7777_8888);
}

#[test]
fn vpush_vpop_transfer_single_register_lists() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    cpu.regs[13] = 0x200;
    cpu.vfp.write_s_bits(16, 0x3f80_0000);
    cpu.vfp.write_s_bits(17, 0x4000_0000);
    cpu.vfp.write_s_bits(18, 0x4040_0000);
    cpu.vfp.write_s_bits(19, 0x4080_0000);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xED2D_8A04),
        ExecResult::Continue
    ));
    assert_eq!(cpu.regs[13], 0x1f0);
    assert_eq!(mem.read_word(0x1f0).unwrap(), 0x3f80_0000);
    assert_eq!(mem.read_word(0x1fc).unwrap(), 0x4080_0000);

    for reg in 16..20 {
        cpu.vfp.write_s_bits(reg, 0);
    }
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xECBD_8A04),
        ExecResult::Continue
    ));
    assert_eq!(cpu.regs[13], 0x200);
    assert_eq!(cpu.vfp.read_s_bits(16), 0x3f80_0000);
    assert_eq!(cpu.vfp.read_s_bits(17), 0x4000_0000);
    assert_eq!(cpu.vfp.read_s_bits(18), 0x4040_0000);
    assert_eq!(cpu.vfp.read_s_bits(19), 0x4080_0000);
}

#[test]
fn vmrs_vmsr_fpscr_and_apsr_flags() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    assert_eq!(
        Aarch32Decoder::decode(0xEEE1_2A10).unwrap().mnemonic,
        Mnemonic::VMSR
    );
    assert_eq!(
        Aarch32Decoder::decode(0xEEF1_FA10).unwrap().mnemonic,
        Mnemonic::VMRS
    );

    cpu.regs[2] = 0xA000_0010;
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEE1_2A10),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.fpscr.bits(), 0xA000_0010);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEF1_FA10),
        ExecResult::Continue
    ));
    assert!(cpu.cpsr.n);
    assert!(!cpu.cpsr.z);
    assert!(cpu.cpsr.c);
    assert!(!cpu.cpsr.v);

    cpu.regs[2] = 0;
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEF0_2A10),
        ExecResult::Continue
    ));
    assert_eq!(cpu.regs[2], cpu.vfp.fpsid);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEF5_2A10),
        ExecResult::Continue
    ));
    assert_eq!(cpu.regs[2], cpu.vfp.mvfr2);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEF6_2A10),
        ExecResult::Continue
    ));
    assert_eq!(cpu.regs[2], cpu.vfp.mvfr1);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEF7_2A10),
        ExecResult::Continue
    ));
    assert_eq!(cpu.regs[2], cpu.vfp.mvfr0);

    cpu.vfp.fpexc = 0;
    cpu.regs[3] = 0x4000_0000;
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEE8_3A10),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.fpexc, 0x4000_0000);

    cpu.vfp.fpexc = 0;
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEF8_2A10),
        ExecResult::Continue
    ));
    assert_eq!(cpu.regs[2], 0);

    cpu.regs[2] = 0xFFFF_FFFF;
    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEE0_2A10),
        ExecResult::Continue
    ));
    assert_eq!(cpu.vfp.fpsid, 0x4103_3070);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEE1_2A10),
        ExecResult::Exception(ExceptionType::UndefinedInstruction)
    ));

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEF2_2A10),
        ExecResult::Undefined
    ));

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEE5_2A10),
        ExecResult::Undefined
    ));

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEF0_FA10),
        ExecResult::Undefined
    ));

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEEE1_FA10),
        ExecResult::Undefined
    ));
}

#[test]
fn disabled_vfp_injects_undefined_exception() {
    let mut cpu = Armv7Cpu::new();
    let mut mem = FlatMemory::new(0x1000, 0);

    cpu.vfp.fpexc = 0;
    cpu.vfp.write_s(0, 1.0);
    cpu.vfp.write_s(1, 2.0);

    assert!(matches!(
        exec_one(&mut cpu, &mut mem, 0xEE30_0A20),
        ExecResult::Exception(ExceptionType::UndefinedInstruction)
    ));
}
