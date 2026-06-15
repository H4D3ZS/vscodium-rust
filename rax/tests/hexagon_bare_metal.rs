use std::sync::Arc;

use vm_memory::{Bytes, GuestAddress, GuestMemoryMmap};

use rax::backend::emulator::hexagon::HexagonVcpu;
use rax::config::{Endianness, HexagonIsa};
use rax::cpu::{CpuState, HexagonRegisters, VCpu, VcpuExit};
use rax::devices::map::HEXAGON_DEBUG_MMIO_BASE;

const CODE_ADDR: u32 = 0x1000;

const MOVIMM_POS: [u8; 16] = [23, 22, 20, 19, 18, 17, 16, 13, 12, 11, 10, 9, 8, 7, 6, 5];
const LOAD_OFF_POS: [u8; 11] = [26, 25, 13, 12, 11, 10, 9, 8, 7, 6, 5];
const STORE_OFF_POS: [u8; 11] = [26, 25, 13, 7, 6, 5, 4, 3, 2, 1, 0];
const IMM10_POS: [u8; 10] = [21, 13, 12, 11, 10, 9, 8, 7, 6, 5];

fn scatter_bits(value: u32, positions: &[u8]) -> u32 {
    let mut out = 0u32;
    for (idx, &bit) in positions.iter().enumerate() {
        let shift = (positions.len() - 1 - idx) as u32;
        let set = (value >> shift) & 1;
        out |= set << bit;
    }
    out
}

fn with_parse_end(word: u32) -> u32 {
    word | (0x3 << 14)
}

fn encode_movimm(dst: u8, imm: u32) -> u32 {
    let mut word = 0u32;
    word |= 0x7 << 28;
    word |= 0b1000 << 24;
    word |= (dst as u32) & 0x1f;
    word |= scatter_bits(imm & 0xffff, &MOVIMM_POS);
    with_parse_end(word)
}

fn encode_add(dst: u8, src1: u8, src2: u8) -> u32 {
    let mut word = 0u32;
    word |= 0xf << 28;
    word |= 0b0011 << 24;
    word |= 0b000 << 21;
    word |= (src1 as u32) << 16;
    word |= (src2 as u32) << 8;
    word |= (dst as u32) & 0x1f;
    with_parse_end(word)
}

fn encode_rtype(base: u32, dst: u8, src_s: u8, src_t: u8) -> u32 {
    let mut word = base;
    word |= (dst as u32) & 0x1f;
    word |= (src_s as u32) << 16;
    word |= (src_t as u32) << 8;
    with_parse_end(word)
}

fn encode_unary(base: u32, dst: u8, src: u8) -> u32 {
    let mut word = base;
    word |= (dst as u32) & 0x1f;
    word |= (src as u32) << 16;
    with_parse_end(word)
}

fn encode_imm10(base: u32, dst: u8, src: u8, imm: u32) -> u32 {
    let mut word = base;
    word |= (dst as u32) & 0x1f;
    word |= (src as u32) << 16;
    word |= scatter_bits(imm & 0x3ff, &IMM10_POS);
    with_parse_end(word)
}

fn encode_orir(dst: u8, src: u8, imm: u32) -> u32 {
    encode_imm10(0x7680_0000, dst, src, imm)
}

fn encode_subri(dst: u8, src: u8, imm: u32) -> u32 {
    encode_imm10(0x7640_0000, dst, src, imm)
}

fn encode_abs(dst: u8, src: u8) -> u32 {
    encode_unary(0x8c80_0080, dst, src)
}

fn encode_abssat(dst: u8, src: u8) -> u32 {
    encode_unary(0x8c80_00a0, dst, src)
}

fn encode_negsat(dst: u8, src: u8) -> u32 {
    encode_unary(0x8c80_00c0, dst, src)
}

fn encode_max(dst: u8, src1: u8, src2: u8) -> u32 {
    encode_rtype(0xd5c0_0000, dst, src1, src2)
}

fn encode_maxu(dst: u8, src1: u8, src2: u8) -> u32 {
    encode_rtype(0xd5c0_0080, dst, src1, src2)
}

fn encode_min(dst: u8, src1: u8, src2: u8) -> u32 {
    encode_rtype(0xd5a0_0000, dst, src1, src2)
}

fn encode_minu(dst: u8, src1: u8, src2: u8) -> u32 {
    encode_rtype(0xd5a0_0080, dst, src1, src2)
}

fn encode_andn(dst: u8, src1: u8, src2: u8) -> u32 {
    encode_rtype(0xf180_0000, dst, src1, src2)
}

fn encode_orn(dst: u8, src1: u8, src2: u8) -> u32 {
    encode_rtype(0xf1a0_0000, dst, src1, src2)
}

fn encode_combine_hh(dst: u8, src_s: u8, src_t: u8) -> u32 {
    encode_rtype(0xf380_0000, dst, src_s, src_t)
}

fn encode_combine_hl(dst: u8, src_s: u8, src_t: u8) -> u32 {
    encode_rtype(0xf3a0_0000, dst, src_s, src_t)
}

fn encode_combine_lh(dst: u8, src_s: u8, src_t: u8) -> u32 {
    encode_rtype(0xf3c0_0000, dst, src_s, src_t)
}

fn encode_combine_ll(dst: u8, src_s: u8, src_t: u8) -> u32 {
    encode_rtype(0xf3e0_0000, dst, src_s, src_t)
}

fn encode_combinew(dst: u8, src_s: u8, src_t: u8) -> u32 {
    encode_rtype(0xf500_0000, dst, src_s, src_t)
}

fn encode_packhl(dst: u8, src_s: u8, src_t: u8) -> u32 {
    encode_rtype(0xf580_0000, dst, src_s, src_t)
}

fn encode_load(base: u8, dst: u8, op: u32, offset: u32) -> u32 {
    let mut word = 0u32;
    word |= 0x9 << 28;
    word |= (op & 0xf) << 21;
    word |= (base as u32) << 16;
    word |= (dst as u32) & 0x1f;
    word |= scatter_bits(offset & 0x7ff, &LOAD_OFF_POS);
    with_parse_end(word)
}

fn encode_store(base: u8, src: u8, op: u32, offset: u32) -> u32 {
    let mut word = 0u32;
    word |= 0xa << 28;
    word |= (op & 0xf) << 21;
    word |= (base as u32) << 16;
    word |= (src as u32) << 8;
    word |= scatter_bits(offset & 0x7ff, &STORE_OFF_POS);
    with_parse_end(word)
}

fn encode_trap0() -> u32 {
    let mut word = 0u32;
    word |= 0x5 << 28;
    word |= 0b0100 << 24;
    with_parse_end(word)
}

fn duplex_reg(reg: u8) -> u16 {
    match reg {
        0..=7 => reg as u16,
        16..=23 => (reg - 8) as u16,
        _ => panic!("duplex reg {reg} out of range"),
    }
}

fn duplex_pair_reg(reg_low: u8) -> u16 {
    match reg_low {
        0 => 0,
        2 => 1,
        4 => 2,
        6 => 3,
        16 => 4,
        18 => 5,
        20 => 6,
        22 => 7,
        _ => panic!("duplex pair reg {reg_low} out of range"),
    }
}

fn encode_duplex_l1_word(base: u8, dst: u8, offset_words: u8) -> u16 {
    let mut sub = 0u16;
    sub |= (offset_words as u16 & 0xf) << 8;
    sub |= duplex_reg(base) << 4;
    sub |= duplex_reg(dst);
    sub
}

fn encode_duplex_s1_word(base: u8, src: u8, offset_words: u8) -> u16 {
    let mut sub = 0u16;
    sub |= (offset_words as u16 & 0xf) << 8;
    sub |= duplex_reg(base) << 4;
    sub |= duplex_reg(src);
    sub
}

fn encode_duplex_l2_dword_sp(dst_low: u8, offset_units: u8) -> u16 {
    let mut sub = 0u16;
    sub |= 0b11110 << 8;
    sub |= (offset_units as u16 & 0x1f) << 3;
    sub |= duplex_pair_reg(dst_low);
    sub
}

fn encode_duplex_s2_dword_sp(src_low: u8, offset_units: i8) -> u16 {
    let imm = (offset_units as i16) & 0x3f;
    let mut sub = 0u16;
    sub |= 0b0101 << 9;
    sub |= (imm as u16 & 0x3f) << 3;
    sub |= duplex_pair_reg(src_low);
    sub
}

fn encode_duplex_a_tfr(src: u8, dst: u8) -> u16 {
    let mut sub = 0u16;
    sub |= 0b100 << 10;
    sub |= duplex_reg(src) << 4;
    sub |= duplex_reg(dst);
    sub
}

fn encode_duplex_a_cmpeqi(src: u8, imm2: u8) -> u16 {
    let mut sub = 0u16;
    sub |= 0b110 << 10;
    sub |= 0b01 << 8;
    sub |= duplex_reg(src) << 4;
    sub |= (imm2 as u16) & 0x3;
    sub
}

fn encode_duplex_a_clrtnew(dst: u8) -> u16 {
    let mut sub = 0u16;
    sub |= 0b110 << 10;
    sub |= 1 << 9;
    sub |= 0b100 << 4;
    sub |= duplex_reg(dst);
    sub
}

fn encode_duplex_a_combinezr(dst_low: u8, src: u8) -> u16 {
    let mut sub = 0u16;
    sub |= 0b111 << 10;
    sub |= 1 << 8;
    sub |= duplex_reg(src) << 4;
    sub |= duplex_pair_reg(dst_low);
    sub
}

fn encode_duplex(slot1: u16, slot0: u16, iclass: u8) -> u32 {
    let mut word = 0u32;
    word |= (slot0 as u32) & 0x1fff;
    word |= ((slot1 as u32) & 0x1fff) << 16;
    word |= ((iclass as u32) & 0x1) << 13;
    word |= (((iclass as u32) >> 1) & 0x7) << 29;
    word
}

fn write_words(mem: &GuestMemoryMmap, addr: u32, words: &[u32], endian: Endianness) {
    let mut offset = 0u32;
    for &word in words {
        let bytes = match endian {
            Endianness::Little => word.to_le_bytes(),
            Endianness::Big => word.to_be_bytes(),
        };
        mem.write_slice(&bytes, GuestAddress((addr + offset) as u64))
            .unwrap();
        offset += 4;
    }
}

fn write_bytes(mem: &GuestMemoryMmap, addr: u32, data: &[u8]) {
    mem.write_slice(data, GuestAddress(addr as u64)).unwrap();
}

fn setup_hexagon_vm(code: &[u32], endian: Endianness) -> (HexagonVcpu, Arc<GuestMemoryMmap>) {
    let mem_size = 64 * 1024;
    let regions = vec![(GuestAddress(0), mem_size)];
    let mem = Arc::new(GuestMemoryMmap::<()>::from_ranges(&regions).unwrap());

    write_words(&mem, CODE_ADDR, code, endian);

    let mut vcpu = HexagonVcpu::new(0, mem.clone(), HexagonIsa::V68, endian);
    let mut regs = HexagonRegisters::default();
    regs.set_pc(CODE_ADDR);
    vcpu.set_state(&CpuState::hexagon(regs)).unwrap();

    (vcpu, mem)
}

fn run_until_shutdown(vcpu: &mut HexagonVcpu) {
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::Shutdown => return,
            VcpuExit::MmioRead { .. } | VcpuExit::MmioWrite { .. } => {
                panic!("unexpected mmio exit in bare-metal test");
            }
            VcpuExit::Hlt => panic!("unexpected halt in bare-metal test"),
            _ => continue,
        }
    }
}

#[test]
fn hexagon_bare_metal_word_ops() {
    let code = [
        encode_movimm(0, 0x2000),
        encode_movimm(1, 0x1234),
        encode_store(0, 1, 0b1100, 0),
        encode_load(0, 2, 0b1100, 0),
        encode_add(3, 1, 2),
        encode_trap0(),
    ];

    let (mut vcpu, _) = setup_hexagon_vm(&code, Endianness::Little);
    run_until_shutdown(&mut vcpu);

    let state = vcpu.get_state().unwrap();
    let regs = match state {
        CpuState::Hexagon(state) => state.regs,
        _ => panic!("expected hexagon state"),
    };

    assert_eq!(regs.r[2], 0x1234);
    assert_eq!(regs.r[3], 0x2468);
}

#[test]
fn hexagon_bare_metal_doubleword_ops() {
    let code = [
        encode_movimm(0, 0x3000),
        encode_movimm(2, 0x1111),
        encode_movimm(3, 0x2222),
        encode_store(0, 3, 0b1110, 0),
        encode_load(0, 5, 0b1110, 0),
        encode_trap0(),
    ];

    let (mut vcpu, _) = setup_hexagon_vm(&code, Endianness::Little);
    run_until_shutdown(&mut vcpu);

    let state = vcpu.get_state().unwrap();
    let regs = match state {
        CpuState::Hexagon(state) => state.regs,
        _ => panic!("expected hexagon state"),
    };

    assert_eq!(regs.r[4], 0x1111);
    assert_eq!(regs.r[5], 0x2222);
}

#[test]
fn hexagon_bare_metal_big_endian_word_ops() {
    let code = [
        encode_movimm(0, 0x3000),
        encode_load(0, 1, 0b1100, 0),
        encode_movimm(2, 0x1234),
        encode_store(0, 2, 0b1100, 1),
        encode_trap0(),
    ];

    let (mut vcpu, mem) = setup_hexagon_vm(&code, Endianness::Big);
    write_bytes(&mem, 0x3000, &[0x12, 0x34, 0x56, 0x78]);

    run_until_shutdown(&mut vcpu);

    let state = vcpu.get_state().unwrap();
    let regs = match state {
        CpuState::Hexagon(state) => state.regs,
        _ => panic!("expected hexagon state"),
    };

    assert_eq!(regs.r[1], 0x1234_5678);

    let mut stored = [0u8; 4];
    mem.read_slice(&mut stored, GuestAddress(0x3004)).unwrap();
    assert_eq!(stored, [0x00, 0x00, 0x12, 0x34]);
}

#[test]
fn hexagon_bare_metal_big_endian_doubleword_ops() {
    let code = [
        encode_movimm(0, 0x4000),
        encode_load(0, 5, 0b1110, 0),
        encode_store(0, 5, 0b1110, 1),
        encode_trap0(),
    ];

    let (mut vcpu, mem) = setup_hexagon_vm(&code, Endianness::Big);
    write_bytes(
        &mem,
        0x4000,
        &[0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88],
    );

    run_until_shutdown(&mut vcpu);

    let state = vcpu.get_state().unwrap();
    let regs = match state {
        CpuState::Hexagon(state) => state.regs,
        _ => panic!("expected hexagon state"),
    };

    assert_eq!(regs.r[4], 0x5566_7788);
    assert_eq!(regs.r[5], 0x1122_3344);

    let mut stored = [0u8; 8];
    mem.read_slice(&mut stored, GuestAddress(0x4008)).unwrap();
    assert_eq!(stored, [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88]);
}

#[test]
fn hexagon_bare_metal_big_endian_duplex_word_ops() {
    let duplex = encode_duplex(
        encode_duplex_s1_word(0, 2, 0),
        encode_duplex_l1_word(0, 1, 1),
        0x8,
    );
    let code = [
        encode_movimm(0, 0x5000),
        encode_movimm(2, 0x1234),
        duplex,
        encode_trap0(),
    ];

    let (mut vcpu, mem) = setup_hexagon_vm(&code, Endianness::Big);
    write_bytes(&mem, 0x5004, &[0xde, 0xad, 0xbe, 0xef]);

    run_until_shutdown(&mut vcpu);

    let state = vcpu.get_state().unwrap();
    let regs = match state {
        CpuState::Hexagon(state) => state.regs,
        _ => panic!("expected hexagon state"),
    };

    assert_eq!(regs.r[1], 0xdead_beef);

    let mut stored = [0u8; 4];
    mem.read_slice(&mut stored, GuestAddress(0x5000)).unwrap();
    assert_eq!(stored, [0x00, 0x00, 0x12, 0x34]);
}

#[test]
fn hexagon_bare_metal_duplex_doubleword_stack_ops() {
    let duplex = encode_duplex(
        encode_duplex_s2_dword_sp(0, 0),
        encode_duplex_l2_dword_sp(2, 1),
        0xd,
    );
    let code = [
        encode_movimm(29, 0x6000),
        encode_movimm(0, 0x1111),
        encode_movimm(1, 0x2222),
        duplex,
        encode_trap0(),
    ];

    let (mut vcpu, mem) = setup_hexagon_vm(&code, Endianness::Little);
    write_bytes(
        &mem,
        0x6008,
        &[0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x11, 0x22],
    );

    run_until_shutdown(&mut vcpu);

    let state = vcpu.get_state().unwrap();
    let regs = match state {
        CpuState::Hexagon(state) => state.regs,
        _ => panic!("expected hexagon state"),
    };

    assert_eq!(regs.r[2], 0xddcc_bbaa);
    assert_eq!(regs.r[3], 0x2211_ffee);

    let mut stored = [0u8; 8];
    mem.read_slice(&mut stored, GuestAddress(0x6000)).unwrap();
    assert_eq!(stored, [0x11, 0x11, 0x00, 0x00, 0x22, 0x22, 0x00, 0x00]);
}

#[test]
fn hexagon_bare_metal_big_endian_duplex_doubleword_stack_ops() {
    let duplex = encode_duplex(
        encode_duplex_s2_dword_sp(0, 0),
        encode_duplex_l2_dword_sp(2, 1),
        0xd,
    );
    let code = [
        encode_movimm(29, 0x7000),
        encode_movimm(0, 0x1122),
        encode_movimm(1, 0x3344),
        duplex,
        encode_trap0(),
    ];

    let (mut vcpu, mem) = setup_hexagon_vm(&code, Endianness::Big);
    write_bytes(
        &mem,
        0x7008,
        &[0x00, 0x00, 0x55, 0x66, 0x00, 0x00, 0x77, 0x88],
    );

    run_until_shutdown(&mut vcpu);

    let state = vcpu.get_state().unwrap();
    let regs = match state {
        CpuState::Hexagon(state) => state.regs,
        _ => panic!("expected hexagon state"),
    };

    assert_eq!(regs.r[2], 0x0000_7788);
    assert_eq!(regs.r[3], 0x0000_5566);

    let mut stored = [0u8; 8];
    mem.read_slice(&mut stored, GuestAddress(0x7000)).unwrap();
    assert_eq!(stored, [0x00, 0x00, 0x33, 0x44, 0x00, 0x00, 0x11, 0x22]);
}

#[test]
fn hexagon_bare_metal_duplex_a_subinsn_ops() {
    let duplex_pred = encode_duplex(
        encode_duplex_a_cmpeqi(2, 2),
        encode_duplex_a_clrtnew(3),
        0x3,
    );
    let duplex_combine = encode_duplex(
        encode_duplex_a_tfr(4, 5),
        encode_duplex_a_combinezr(0, 4),
        0x3,
    );
    let code = [
        encode_movimm(2, 2),
        encode_movimm(3, 0x1234),
        encode_movimm(4, 0x5678),
        duplex_pred,
        duplex_combine,
        encode_trap0(),
    ];

    let (mut vcpu, _) = setup_hexagon_vm(&code, Endianness::Little);
    run_until_shutdown(&mut vcpu);

    let state = vcpu.get_state().unwrap();
    let regs = match state {
        CpuState::Hexagon(state) => state.regs,
        _ => panic!("expected hexagon state"),
    };

    assert_eq!(regs.r[3], 0);
    assert_eq!(regs.r[0], 0x5678);
    assert_eq!(regs.r[1], 0);
    assert_eq!(regs.r[5], 0x5678);
}

#[test]
fn hexagon_bare_metal_v68_alu_ops() {
    let code = [
        encode_movimm(1, 0x0f0f),
        encode_movimm(2, 0x00ff),
        encode_andn(3, 1, 2),
        encode_orn(4, 1, 2),
        encode_movimm(5, 0x0050),
        encode_orir(6, 5, 0x0a5),
        encode_subri(7, 5, 0x064),
        encode_movimm(8, 0xfff6),
        encode_abs(9, 8),
        encode_abssat(10, 8),
        encode_negsat(11, 5),
        encode_movimm(12, 0xfffe),
        encode_movimm(13, 0x0001),
        encode_max(14, 12, 13),
        encode_min(15, 12, 13),
        encode_maxu(16, 12, 13),
        encode_minu(17, 12, 13),
        encode_trap0(),
    ];

    let (mut vcpu, _) = setup_hexagon_vm(&code, Endianness::Little);
    run_until_shutdown(&mut vcpu);

    let state = vcpu.get_state().unwrap();
    let regs = match state {
        CpuState::Hexagon(state) => state.regs,
        _ => panic!("expected hexagon state"),
    };

    assert_eq!(regs.r[3], 0x00f0);
    assert_eq!(regs.r[4], 0xffff_f0ff);
    assert_eq!(regs.r[6], 0x00f5);
    assert_eq!(regs.r[7], 0x0014);
    assert_eq!(regs.r[9], 10);
    assert_eq!(regs.r[10], 10);
    assert_eq!(regs.r[11], 0xffff_ffb0);
    assert_eq!(regs.r[14], 1);
    assert_eq!(regs.r[15], 0xffff_fffe);
    assert_eq!(regs.r[16], 0xffff_fffe);
    assert_eq!(regs.r[17], 1);
}

#[test]
fn hexagon_bare_metal_v68_saturating_edges() {
    let code = [
        encode_movimm(0, 0x2000),
        encode_load(0, 1, 0b1100, 0),
        encode_abs(2, 1),
        encode_abssat(3, 1),
        encode_negsat(4, 1),
        encode_trap0(),
    ];

    let (mut vcpu, mem) = setup_hexagon_vm(&code, Endianness::Little);
    write_bytes(&mem, 0x2000, &0x8000_0000u32.to_le_bytes());

    run_until_shutdown(&mut vcpu);

    let state = vcpu.get_state().unwrap();
    let regs = match state {
        CpuState::Hexagon(state) => state.regs,
        _ => panic!("expected hexagon state"),
    };

    assert_eq!(regs.r[1], 0x8000_0000);
    assert_eq!(regs.r[2], 0x8000_0000);
    assert_eq!(regs.r[3], 0x7fff_ffff);
    assert_eq!(regs.r[4], 0x7fff_ffff);
}

#[test]
fn hexagon_bare_metal_combine_ops() {
    let code = [
        encode_movimm(0, 0x2000),
        encode_load(0, 4, 0b1100, 0),
        encode_load(0, 5, 0b1100, 1),
        encode_combine_hh(6, 4, 5),
        encode_combine_hl(7, 4, 5),
        encode_combine_lh(8, 4, 5),
        encode_combine_ll(9, 4, 5),
        encode_combinew(10, 4, 5),
        encode_trap0(),
    ];

    let (mut vcpu, mem) = setup_hexagon_vm(&code, Endianness::Little);
    write_bytes(
        &mem,
        0x2000,
        &[0x44, 0x33, 0x22, 0x11, 0x88, 0x77, 0x66, 0x55],
    );

    run_until_shutdown(&mut vcpu);

    let state = vcpu.get_state().unwrap();
    let regs = match state {
        CpuState::Hexagon(state) => state.regs,
        _ => panic!("expected hexagon state"),
    };

    assert_eq!(regs.r[4], 0x1122_3344);
    assert_eq!(regs.r[5], 0x5566_7788);
    assert_eq!(regs.r[6], 0x5566_1122);
    assert_eq!(regs.r[7], 0x5566_3344);
    assert_eq!(regs.r[8], 0x7788_1122);
    assert_eq!(regs.r[9], 0x7788_3344);
    assert_eq!(regs.r[10], 0x5566_7788);
    assert_eq!(regs.r[11], 0x1122_3344);
}

#[test]
fn hexagon_bare_metal_packhl_ops() {
    let code = [
        encode_movimm(0, 0x4000),
        encode_load(0, 2, 0b1100, 0),
        encode_load(0, 3, 0b1100, 1),
        encode_packhl(4, 2, 3),
        encode_trap0(),
    ];

    let (mut vcpu, mem) = setup_hexagon_vm(&code, Endianness::Little);
    write_bytes(
        &mem,
        0x4000,
        &[0x44, 0x33, 0x22, 0x11, 0x88, 0x77, 0x66, 0x55],
    );

    run_until_shutdown(&mut vcpu);

    let state = vcpu.get_state().unwrap();
    let regs = match state {
        CpuState::Hexagon(state) => state.regs,
        _ => panic!("expected hexagon state"),
    };

    assert_eq!(regs.r[4], 0x3344_7788);
    assert_eq!(regs.r[5], 0x1122_5566);
}

#[test]
fn hexagon_bare_metal_debug_mmio_write() {
    let code = [
        encode_movimm(1, 0x3000),
        encode_load(1, 0, 0b1100, 0),
        encode_movimm(2, 0x0041),
        encode_store(0, 2, 0b1000, 0),
        encode_trap0(),
    ];

    let (mut vcpu, mem) = setup_hexagon_vm(&code, Endianness::Little);
    let addr_bytes = (HEXAGON_DEBUG_MMIO_BASE as u32).to_le_bytes();
    write_bytes(&mem, 0x3000, &addr_bytes);

    let mut saw_mmio = false;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::Shutdown => break,
            VcpuExit::MmioWrite { addr, data } => {
                assert_eq!(addr, HEXAGON_DEBUG_MMIO_BASE);
                assert_eq!(data, vec![0x41]);
                saw_mmio = true;
            }
            VcpuExit::MmioRead { .. } => panic!("unexpected mmio read in bare-metal test"),
            VcpuExit::Hlt => panic!("unexpected halt in bare-metal test"),
            _ => continue,
        }
    }

    assert!(saw_mmio);
}

#[test]
fn hexagon_bare_metal_debug_mmio_read() {
    let code = [
        encode_movimm(1, 0x5000),
        encode_load(1, 0, 0b1100, 0),
        encode_load(0, 2, 0b1001, 0),
        encode_trap0(),
    ];

    let (mut vcpu, mem) = setup_hexagon_vm(&code, Endianness::Little);
    let addr_bytes = (HEXAGON_DEBUG_MMIO_BASE as u32).to_le_bytes();
    write_bytes(&mem, 0x5000, &addr_bytes);

    let mut saw_mmio = false;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::Shutdown => break,
            VcpuExit::MmioRead { addr, size } => {
                assert_eq!(addr, HEXAGON_DEBUG_MMIO_BASE);
                assert_eq!(size, 1);
                vcpu.complete_io_in(&[0]);
                saw_mmio = true;
            }
            VcpuExit::MmioWrite { .. } => panic!("unexpected mmio write in bare-metal test"),
            VcpuExit::Hlt => panic!("unexpected halt in bare-metal test"),
            _ => continue,
        }
    }

    let state = vcpu.get_state().unwrap();
    let regs = match state {
        CpuState::Hexagon(state) => state.regs,
        _ => panic!("expected hexagon state"),
    };

    assert!(saw_mmio);
    assert_eq!(regs.r[2], 0);
}
