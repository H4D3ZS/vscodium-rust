//! AVX10.1 and AVX10.2 instruction lifter.
//!
//! This module lifts EVEX-encoded AVX10 instructions to SMIR operations.
//! AVX10 unifies AVX-512 features across all vector lengths.
//!
//! ## AVX10.1 Instructions
//! - VNNI: VPDPBUSD, VPDPBUSDS, VPDPWSSD, VPDPWSSDS
//! - IFMA: VPMADD52LUQ, VPMADD52HUQ
//! - VPOPCNTDQ: VPOPCNTB, VPOPCNTW, VPOPCNTD, VPOPCNTQ
//! - VBMI: VPERMB, VPERMI2B, VPERMT2B
//! - BITALG: VPSHUFBITQMB
//! - BF16: VDPBF16PS, VCVTNEPS2BF16, VCVTNE2PS2BF16
//! - FP16: VADDPH, VMULPH, VSUBPH, VDIVPH
//!
//! ## AVX10.2 Instructions
//! - Saturation conversions: VCVTTPS2IBS, VCVTTPS2IUBS, VCVTTPD2QQS, VCVTTPD2UQQS
//! - VMPSADBW
//! - VMINMAX: VMINMAXPS, VMINMAXPD, VMINMAXSS, VMINMAXSD
//! - Media acceleration: VPDPB*/VPDPW* variants

use crate::smir::lift::{ControlFlow, LiftContext, LiftError, LiftResult};
use crate::smir::ops::{OpKind, SmirOp, X86OpHint, X86SsePrefix, X86VecMap};
use crate::smir::types::*;

// ============================================================================
// EVEX Prefix Decoding
// ============================================================================

/// Decoded EVEX prefix information
#[derive(Clone, Copy, Debug)]
pub struct EvexPrefix {
    /// R bit (extends ModRM.reg)
    pub r: bool,
    /// X bit (extends SIB.index)
    pub x: bool,
    /// B bit (extends ModRM.rm)
    pub b: bool,
    /// R' bit (high bit of reg extension for 32 registers)
    pub r_prime: bool,
    /// Opcode map (1=0F, 2=0F38, 3=0F3A)
    pub map: u8,
    /// W bit (operand width)
    pub w: bool,
    /// VVVV register (inverted)
    pub vvvv: u8,
    /// PP prefix simulation (0=none, 1=66, 2=F3, 3=F2)
    pub pp: u8,
    /// z bit (zeroing masking)
    pub z: bool,
    /// L'L bits (vector length)
    pub ll: u8,
    /// b bit (broadcast/rounding)
    pub b_bit: bool,
    /// V' bit (high bit of vvvv)
    pub v_prime: bool,
    /// aaa (opmask register k0-k7)
    pub aaa: u8,
    /// Number of bytes consumed
    pub bytes: usize,
}

impl EvexPrefix {
    /// Decode EVEX prefix from bytes
    pub fn decode(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 4 || bytes[0] != 0x62 {
            return None;
        }

        let p0 = bytes[1];
        let p1 = bytes[2];
        let p2 = bytes[3];

        // P0: R X B R' m2 0 m1 m0 (or R X B R' 0 0 m1 m0 for legacy)
        // For AVX-512-FP16 (MAP5, MAP6), bit 2 can be 1
        // For legacy EVEX, bits 3:2 must be 00
        // Extract full 3-bit map field for MAP5/MAP6 support
        let map = p0 & 0x07;

        // Validate: bit 3 must be 0 (reserved)
        if (p0 & 0x08) != 0 {
            return None;
        }

        // For maps 1-3, bit 2 must also be 0
        // For map 5-6 (FP16), bit 2 can be 1
        if map != 5 && map != 6 && (p0 & 0x04) != 0 {
            return None;
        }

        Some(EvexPrefix {
            r: (p0 & 0x80) == 0,       // inverted
            x: (p0 & 0x40) == 0,       // inverted
            b: (p0 & 0x20) == 0,       // inverted
            r_prime: (p0 & 0x10) == 0, // inverted
            map,

            // P1: W v v v v 1 p p
            w: (p1 & 0x80) != 0,
            vvvv: (!p1 >> 3) & 0x0F,
            pp: p1 & 0x03,

            // P2: z L' L b V' a a a
            z: (p2 & 0x80) != 0,
            ll: (p2 >> 5) & 0x03,
            b_bit: (p2 & 0x10) != 0,
            v_prime: (p2 & 0x08) == 0, // inverted
            aaa: p2 & 0x07,

            bytes: 4,
        })
    }

    /// Get vector width from L'L bits
    pub fn vec_width(&self) -> VecWidth {
        match self.ll {
            0 => VecWidth::V128,
            1 => VecWidth::V256,
            2 | 3 => VecWidth::V512,
            _ => VecWidth::V128,
        }
    }

    /// Get destination register index
    pub fn dest_reg(&self, modrm_reg: u8) -> u8 {
        let mut reg = modrm_reg & 0x07;
        if self.r {
            reg |= 0x08;
        }
        if self.r_prime {
            reg |= 0x10;
        }
        reg
    }

    /// Get source register index from r/m field
    pub fn rm_reg(&self, modrm_rm: u8) -> u8 {
        let mut rm = modrm_rm & 0x07;
        if self.b {
            rm |= 0x08;
        }
        if self.x {
            rm |= 0x10;
        }
        rm
    }

    /// Get source1 register from vvvv field
    pub fn src1_reg(&self) -> u8 {
        let mut vvvv = self.vvvv;
        if self.v_prime {
            vvvv |= 0x10;
        }
        vvvv
    }

    /// Get SSE prefix equivalent
    pub fn sse_prefix(&self) -> X86SsePrefix {
        match self.pp {
            0 => X86SsePrefix::None,
            1 => X86SsePrefix::OpSize,
            2 => X86SsePrefix::Rep,
            3 => X86SsePrefix::Repne,
            _ => X86SsePrefix::None,
        }
    }

    /// Get opcode map
    pub fn vec_map(&self) -> Option<X86VecMap> {
        match self.map {
            1 => Some(X86VecMap::Map0F),
            2 => Some(X86VecMap::Map0F38),
            3 => Some(X86VecMap::Map0F3A),
            _ => None,
        }
    }

    /// Create encoding info for roundtrip
    pub fn encoding_info(&self, opcode: u8) -> Avx10Encoding {
        Avx10Encoding {
            map: self.map,
            pp: self.pp,
            w: self.w,
            opcode,
            vl: self.vec_width(),
            mask: if self.aaa != 0 { Some(self.aaa) } else { None },
            zeroing: self.z,
            rounding: if self.b_bit && self.ll == 2 {
                Some(self.ll)
            } else {
                None
            },
        }
    }
}

// ============================================================================
// AVX10 Lifter
// ============================================================================

/// AVX10 instruction lifter
pub struct Avx10Lifter;

impl Avx10Lifter {
    /// Create a new AVX10 lifter
    pub fn new() -> Self {
        Self
    }

    /// Try to lift an EVEX-encoded instruction as AVX10
    /// Returns None if not an AVX10 instruction
    pub fn try_lift(
        &self,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Option<Result<LiftResult, LiftError>> {
        // Check for EVEX prefix
        let evex = EvexPrefix::decode(bytes)?;
        let opcode_offset = evex.bytes;

        if bytes.len() <= opcode_offset {
            return Some(Err(LiftError::Incomplete {
                addr: pc,
                have: bytes.len(),
                need: opcode_offset + 1,
            }));
        }

        let opcode = bytes[opcode_offset];
        let remaining = &bytes[opcode_offset + 1..];

        // Dispatch based on map and opcode
        match evex.map {
            // Map 2 = 0F 38
            2 => self.lift_map2(&evex, opcode, remaining, pc, ctx),
            // Map 3 = 0F 3A
            3 => self.lift_map3(&evex, opcode, remaining, pc, ctx),
            // Map 5 = FP16 (MAP5)
            5 => self.lift_map5(&evex, opcode, remaining, pc, ctx),
            _ => None,
        }
    }

    /// Lift 0F38-map instructions
    fn lift_map2(
        &self,
        evex: &EvexPrefix,
        opcode: u8,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Option<Result<LiftResult, LiftError>> {
        match (evex.pp, opcode, evex.w) {
            // VNNI instructions (66.0F38.W0)
            (1, 0x50, false) => Some(self.lift_vpdpbusd(evex, bytes, pc, ctx, false)),
            (1, 0x51, false) => Some(self.lift_vpdpbusd(evex, bytes, pc, ctx, true)),
            (1, 0x52, false) => Some(self.lift_vpdpwssd(evex, bytes, pc, ctx, false)),
            (1, 0x53, false) => Some(self.lift_vpdpwssd(evex, bytes, pc, ctx, true)),

            // IFMA instructions (66.0F38.W1)
            (1, 0xB4, true) => Some(self.lift_vpmadd52(evex, bytes, pc, ctx, false)),
            (1, 0xB5, true) => Some(self.lift_vpmadd52(evex, bytes, pc, ctx, true)),

            // VPOPCNT instructions
            (1, 0x54, false) => Some(self.lift_vpopcnt(evex, bytes, pc, ctx, VecElementType::I8)),
            (1, 0x54, true) => Some(self.lift_vpopcnt(evex, bytes, pc, ctx, VecElementType::I16)),
            (1, 0x55, false) => Some(self.lift_vpopcnt(evex, bytes, pc, ctx, VecElementType::I32)),
            (1, 0x55, true) => Some(self.lift_vpopcnt(evex, bytes, pc, ctx, VecElementType::I64)),

            // VBMI - byte permute
            (1, 0x8D, false) => Some(self.lift_vpermb(evex, bytes, pc, ctx)),
            (1, 0x75, false) => Some(self.lift_vpermi2b(evex, bytes, pc, ctx)),
            (1, 0x7D, false) => Some(self.lift_vpermt2b(evex, bytes, pc, ctx)),

            // BITALG
            (1, 0x8F, false) => Some(self.lift_vpshufbitqmb(evex, bytes, pc, ctx)),

            // BF16 instructions
            (2, 0x52, false) => Some(self.lift_vdpbf16ps(evex, bytes, pc, ctx)), // F3.0F38.W0
            (2, 0x72, false) => Some(self.lift_vcvtneps2bf16(evex, bytes, pc, ctx)), // F3.0F38.W0
            (3, 0x72, false) => Some(self.lift_vcvtne2ps2bf16(evex, bytes, pc, ctx)), // F2.0F38.W0

            // AVX10.2 saturation conversions
            (0, 0x68, false) => Some(self.lift_vcvttps2ibs(evex, bytes, pc, ctx, true)),
            (0, 0x6A, false) => Some(self.lift_vcvttps2ibs(evex, bytes, pc, ctx, false)),
            (1, 0x6C, true) => Some(self.lift_vcvttpd2qqs(evex, bytes, pc, ctx, false)),
            (1, 0x6D, true) => Some(self.lift_vcvttpd2qqs(evex, bytes, pc, ctx, true)),

            // AVX10.2 media acceleration (byte variants)
            (2, 0x50, false) => Some(self.lift_vpdpbssd(evex, bytes, pc, ctx, true, true, false)),
            (2, 0x51, false) => Some(self.lift_vpdpbssd(evex, bytes, pc, ctx, true, true, true)),
            (2, 0x50, true) => Some(self.lift_vpdpbssd(evex, bytes, pc, ctx, true, false, false)),
            (2, 0x51, true) => Some(self.lift_vpdpbssd(evex, bytes, pc, ctx, true, false, true)),
            (0, 0x50, true) => Some(self.lift_vpdpbssd(evex, bytes, pc, ctx, false, false, false)),
            (0, 0x51, true) => Some(self.lift_vpdpbssd(evex, bytes, pc, ctx, false, false, true)),

            // AVX10.2 media acceleration (word variants)
            (2, 0xD2, false) => Some(self.lift_vpdpwext(evex, bytes, pc, ctx, true, false, false)),
            (2, 0xD3, false) => Some(self.lift_vpdpwext(evex, bytes, pc, ctx, true, false, true)),
            (1, 0xD2, false) => Some(self.lift_vpdpwext(evex, bytes, pc, ctx, false, true, false)),
            (1, 0xD3, false) => Some(self.lift_vpdpwext(evex, bytes, pc, ctx, false, true, true)),
            (0, 0xD2, false) => Some(self.lift_vpdpwext(evex, bytes, pc, ctx, false, false, false)),
            (0, 0xD3, false) => Some(self.lift_vpdpwext(evex, bytes, pc, ctx, false, false, true)),

            _ => None,
        }
    }

    /// Lift 0F3A-map instructions
    fn lift_map3(
        &self,
        evex: &EvexPrefix,
        opcode: u8,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Option<Result<LiftResult, LiftError>> {
        match (evex.pp, opcode, evex.w) {
            // VMPSADBW
            (1, 0x42, _) => Some(self.lift_vmpsadbw(evex, bytes, pc, ctx)),

            // VMINMAX
            (0, 0x52, false) => Some(self.lift_vminmax(evex, bytes, pc, ctx, VecElementType::F32)),
            (1, 0x52, true) => Some(self.lift_vminmax(evex, bytes, pc, ctx, VecElementType::F64)),
            (0, 0x53, false) => {
                Some(self.lift_vminmax_scalar(evex, bytes, pc, ctx, VecElementType::F32))
            }
            (1, 0x53, true) => {
                Some(self.lift_vminmax_scalar(evex, bytes, pc, ctx, VecElementType::F64))
            }

            _ => None,
        }
    }

    /// Lift MAP5 instructions (FP16)
    fn lift_map5(
        &self,
        evex: &EvexPrefix,
        opcode: u8,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Option<Result<LiftResult, LiftError>> {
        match (evex.pp, opcode) {
            (0, 0x58) => Some(self.lift_vfp16_arith(evex, bytes, pc, ctx, Avx10FP16Op::Add)),
            (0, 0x59) => Some(self.lift_vfp16_arith(evex, bytes, pc, ctx, Avx10FP16Op::Mul)),
            (0, 0x5C) => Some(self.lift_vfp16_arith(evex, bytes, pc, ctx, Avx10FP16Op::Sub)),
            (0, 0x5E) => Some(self.lift_vfp16_arith(evex, bytes, pc, ctx, Avx10FP16Op::Div)),
            _ => None,
        }
    }

    // ========================================================================
    // VNNI Instructions
    // ========================================================================

    /// Lift VPDPBUSD/VPDPBUSDS
    fn lift_vpdpbusd(
        &self,
        evex: &EvexPrefix,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
        saturate: bool,
    ) -> Result<LiftResult, LiftError> {
        let (modrm, consumed) = self.decode_modrm(bytes, pc)?;

        let dst_reg = evex.dest_reg(modrm.reg);
        let src1_reg = evex.src1_reg();
        let src2_reg = evex.rm_reg(modrm.rm);
        let width = evex.vec_width();

        let dst = self.zmm(dst_reg);
        let acc = dst; // accumulates into dst
        let src1 = self.zmm(src1_reg);
        let src2 = if modrm.is_memory {
            let tmp = ctx.alloc_vreg();
            // Memory operand - would need address calculation
            tmp
        } else {
            self.zmm(src2_reg)
        };

        let op = SmirOp::new(
            ctx.next_op_id(),
            pc,
            OpKind::VDotProduct {
                dst,
                acc,
                src1,
                src2,
                src_elem: VecElementType::I8,
                acc_elem: VecElementType::I32,
                width,
                src1_unsigned: true,
                saturate,
            },
        );

        Ok(LiftResult::fallthrough(vec![op], evex.bytes + 1 + consumed))
    }

    /// Lift VPDPWSSD/VPDPWSSDS
    fn lift_vpdpwssd(
        &self,
        evex: &EvexPrefix,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
        saturate: bool,
    ) -> Result<LiftResult, LiftError> {
        let (modrm, consumed) = self.decode_modrm(bytes, pc)?;

        let dst_reg = evex.dest_reg(modrm.reg);
        let src1_reg = evex.src1_reg();
        let src2_reg = evex.rm_reg(modrm.rm);
        let width = evex.vec_width();

        let dst = self.zmm(dst_reg);
        let acc = dst;
        let src1 = self.zmm(src1_reg);
        let src2 = self.zmm(src2_reg);

        let op = SmirOp::new(
            ctx.next_op_id(),
            pc,
            OpKind::VDotProduct {
                dst,
                acc,
                src1,
                src2,
                src_elem: VecElementType::I16,
                acc_elem: VecElementType::I32,
                width,
                src1_unsigned: false,
                saturate,
            },
        );

        Ok(LiftResult::fallthrough(vec![op], evex.bytes + 1 + consumed))
    }

    // ========================================================================
    // IFMA Instructions
    // ========================================================================

    /// Lift VPMADD52LUQ/VPMADD52HUQ
    fn lift_vpmadd52(
        &self,
        evex: &EvexPrefix,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
        high: bool,
    ) -> Result<LiftResult, LiftError> {
        let (modrm, consumed) = self.decode_modrm(bytes, pc)?;

        let dst_reg = evex.dest_reg(modrm.reg);
        let src1_reg = evex.src1_reg();
        let src2_reg = evex.rm_reg(modrm.rm);
        let width = evex.vec_width();

        let dst = self.zmm(dst_reg);
        let acc = dst;
        let src1 = self.zmm(src1_reg);
        let src2 = self.zmm(src2_reg);

        let op = SmirOp::new(
            ctx.next_op_id(),
            pc,
            OpKind::VMultiplyAdd52 {
                dst,
                acc,
                src1,
                src2,
                width,
                high,
            },
        );

        Ok(LiftResult::fallthrough(vec![op], evex.bytes + 1 + consumed))
    }

    // ========================================================================
    // VPOPCNT Instructions
    // ========================================================================

    /// Lift VPOPCNTB/W/D/Q
    fn lift_vpopcnt(
        &self,
        evex: &EvexPrefix,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
        elem: VecElementType,
    ) -> Result<LiftResult, LiftError> {
        let (modrm, consumed) = self.decode_modrm(bytes, pc)?;

        let dst_reg = evex.dest_reg(modrm.reg);
        let src_reg = evex.rm_reg(modrm.rm);
        let width = evex.vec_width();

        let dst = self.zmm(dst_reg);
        let src = self.zmm(src_reg);

        let op = SmirOp::new(
            ctx.next_op_id(),
            pc,
            OpKind::VPopcnt {
                dst,
                src,
                elem,
                width,
            },
        );

        Ok(LiftResult::fallthrough(vec![op], evex.bytes + 1 + consumed))
    }

    // ========================================================================
    // VBMI Instructions
    // ========================================================================

    /// Lift VPERMB
    fn lift_vpermb(
        &self,
        evex: &EvexPrefix,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let (modrm, consumed) = self.decode_modrm(bytes, pc)?;

        let dst_reg = evex.dest_reg(modrm.reg);
        let src1_reg = evex.src1_reg();
        let src2_reg = evex.rm_reg(modrm.rm);
        let width = evex.vec_width();

        let dst = self.zmm(dst_reg);
        let src1 = self.zmm(src1_reg);
        let indices = self.zmm(src2_reg);

        let op = SmirOp::new(
            ctx.next_op_id(),
            pc,
            OpKind::VPermute {
                dst,
                src1,
                src2: None,
                indices,
                elem: VecElementType::I8,
                width,
                overwrite_table: false,
            },
        );

        Ok(LiftResult::fallthrough(vec![op], evex.bytes + 1 + consumed))
    }

    /// Lift VPERMI2B
    fn lift_vpermi2b(
        &self,
        evex: &EvexPrefix,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let (modrm, consumed) = self.decode_modrm(bytes, pc)?;

        let dst_reg = evex.dest_reg(modrm.reg);
        let src1_reg = evex.src1_reg();
        let src2_reg = evex.rm_reg(modrm.rm);
        let width = evex.vec_width();

        let dst = self.zmm(dst_reg);
        let src1 = self.zmm(src1_reg);
        let src2 = self.zmm(src2_reg);

        let op = SmirOp::new(
            ctx.next_op_id(),
            pc,
            OpKind::VPermute {
                dst,
                src1,
                src2: Some(src2),
                indices: dst, // indices come from dst
                elem: VecElementType::I8,
                width,
                overwrite_table: false,
            },
        );

        Ok(LiftResult::fallthrough(vec![op], evex.bytes + 1 + consumed))
    }

    /// Lift VPERMT2B
    fn lift_vpermt2b(
        &self,
        evex: &EvexPrefix,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let (modrm, consumed) = self.decode_modrm(bytes, pc)?;

        let dst_reg = evex.dest_reg(modrm.reg);
        let src1_reg = evex.src1_reg();
        let src2_reg = evex.rm_reg(modrm.rm);
        let width = evex.vec_width();

        let dst = self.zmm(dst_reg);
        let src1 = self.zmm(src1_reg);
        let src2 = self.zmm(src2_reg);

        let op = SmirOp::new(
            ctx.next_op_id(),
            pc,
            OpKind::VPermute {
                dst,
                src1: dst, // table comes from dst
                src2: Some(src2),
                indices: src1,
                elem: VecElementType::I8,
                width,
                overwrite_table: true,
            },
        );

        Ok(LiftResult::fallthrough(vec![op], evex.bytes + 1 + consumed))
    }

    // ========================================================================
    // BITALG Instructions
    // ========================================================================

    /// Lift VPSHUFBITQMB
    fn lift_vpshufbitqmb(
        &self,
        evex: &EvexPrefix,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let (modrm, consumed) = self.decode_modrm(bytes, pc)?;

        let dst_reg = evex.dest_reg(modrm.reg);
        let src1_reg = evex.src1_reg();
        let src2_reg = evex.rm_reg(modrm.rm);
        let width = evex.vec_width();

        // Destination is a k register
        let dst = VReg::Arch(ArchReg::X86(X86Reg::K(dst_reg & 0x07)));
        let src = self.zmm(src1_reg);
        let indices = self.zmm(src2_reg);

        let op = SmirOp::new(
            ctx.next_op_id(),
            pc,
            OpKind::VShuffleBitQM {
                dst,
                src,
                indices,
                width,
            },
        );

        Ok(LiftResult::fallthrough(vec![op], evex.bytes + 1 + consumed))
    }

    // ========================================================================
    // BF16 Instructions
    // ========================================================================

    /// Lift VDPBF16PS
    fn lift_vdpbf16ps(
        &self,
        evex: &EvexPrefix,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let (modrm, consumed) = self.decode_modrm(bytes, pc)?;

        let dst_reg = evex.dest_reg(modrm.reg);
        let src1_reg = evex.src1_reg();
        let src2_reg = evex.rm_reg(modrm.rm);
        let width = evex.vec_width();

        let dst = self.zmm(dst_reg);
        let acc = dst;
        let src1 = self.zmm(src1_reg);
        let src2 = self.zmm(src2_reg);

        let op = SmirOp::new(
            ctx.next_op_id(),
            pc,
            OpKind::VDotProductBF16 {
                dst,
                acc,
                src1,
                src2,
                width,
            },
        );

        Ok(LiftResult::fallthrough(vec![op], evex.bytes + 1 + consumed))
    }

    /// Lift VCVTNEPS2BF16
    fn lift_vcvtneps2bf16(
        &self,
        evex: &EvexPrefix,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let (modrm, consumed) = self.decode_modrm(bytes, pc)?;

        let dst_reg = evex.dest_reg(modrm.reg);
        let src_reg = evex.rm_reg(modrm.rm);
        let width = evex.vec_width();

        let dst = self.zmm(dst_reg);
        let src = self.zmm(src_reg);

        let op = SmirOp::new(
            ctx.next_op_id(),
            pc,
            OpKind::VCvtFP32ToBF16 {
                dst,
                src1: src,
                src2: None,
                width,
            },
        );

        Ok(LiftResult::fallthrough(vec![op], evex.bytes + 1 + consumed))
    }

    /// Lift VCVTNE2PS2BF16
    fn lift_vcvtne2ps2bf16(
        &self,
        evex: &EvexPrefix,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let (modrm, consumed) = self.decode_modrm(bytes, pc)?;

        let dst_reg = evex.dest_reg(modrm.reg);
        let src1_reg = evex.src1_reg();
        let src2_reg = evex.rm_reg(modrm.rm);
        let width = evex.vec_width();

        let dst = self.zmm(dst_reg);
        let src1 = self.zmm(src1_reg);
        let src2 = self.zmm(src2_reg);

        let op = SmirOp::new(
            ctx.next_op_id(),
            pc,
            OpKind::VCvtFP32ToBF16 {
                dst,
                src1,
                src2: Some(src2),
                width,
            },
        );

        Ok(LiftResult::fallthrough(vec![op], evex.bytes + 1 + consumed))
    }

    // ========================================================================
    // FP16 Instructions
    // ========================================================================

    /// Lift VADDPH/VSUBPH/VMULPH/VDIVPH
    fn lift_vfp16_arith(
        &self,
        evex: &EvexPrefix,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
        op_type: Avx10FP16Op,
    ) -> Result<LiftResult, LiftError> {
        let (modrm, consumed) = self.decode_modrm(bytes, pc)?;

        let dst_reg = evex.dest_reg(modrm.reg);
        let src1_reg = evex.src1_reg();
        let src2_reg = evex.rm_reg(modrm.rm);
        let width = evex.vec_width();

        let dst = self.zmm(dst_reg);
        let src1 = self.zmm(src1_reg);
        let src2 = self.zmm(src2_reg);

        let op = SmirOp::new(
            ctx.next_op_id(),
            pc,
            OpKind::VFP16Arith {
                dst,
                src1,
                src2,
                op: op_type,
                width,
            },
        );

        Ok(LiftResult::fallthrough(vec![op], evex.bytes + 1 + consumed))
    }

    // ========================================================================
    // AVX10.2 Saturation Conversions
    // ========================================================================

    /// Lift VCVTTPS2IBS/VCVTTPS2IUBS
    fn lift_vcvttps2ibs(
        &self,
        evex: &EvexPrefix,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
        signed: bool,
    ) -> Result<LiftResult, LiftError> {
        let (modrm, consumed) = self.decode_modrm(bytes, pc)?;

        let dst_reg = evex.dest_reg(modrm.reg);
        let src_reg = evex.rm_reg(modrm.rm);
        let width = evex.vec_width();

        let dst = self.zmm(dst_reg);
        let src = self.zmm(src_reg);

        let op = SmirOp::new(
            ctx.next_op_id(),
            pc,
            OpKind::VCvtFpToIntSat {
                dst,
                src,
                fp_elem: VecElementType::F32,
                int_elem: VecElementType::I8,
                width,
                signed,
            },
        );

        Ok(LiftResult::fallthrough(vec![op], evex.bytes + 1 + consumed))
    }

    /// Lift VCVTTPD2QQS/VCVTTPD2UQQS
    fn lift_vcvttpd2qqs(
        &self,
        evex: &EvexPrefix,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
        signed: bool,
    ) -> Result<LiftResult, LiftError> {
        let (modrm, consumed) = self.decode_modrm(bytes, pc)?;

        let dst_reg = evex.dest_reg(modrm.reg);
        let src_reg = evex.rm_reg(modrm.rm);
        let width = evex.vec_width();

        let dst = self.zmm(dst_reg);
        let src = self.zmm(src_reg);

        let op = SmirOp::new(
            ctx.next_op_id(),
            pc,
            OpKind::VCvtFpToIntSat {
                dst,
                src,
                fp_elem: VecElementType::F64,
                int_elem: VecElementType::I64,
                width,
                signed,
            },
        );

        Ok(LiftResult::fallthrough(vec![op], evex.bytes + 1 + consumed))
    }

    // ========================================================================
    // AVX10.2 VMINMAX
    // ========================================================================

    /// Lift VMINMAXPS/VMINMAXPD
    fn lift_vminmax(
        &self,
        evex: &EvexPrefix,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
        elem: VecElementType,
    ) -> Result<LiftResult, LiftError> {
        let (modrm, consumed) = self.decode_modrm(bytes, pc)?;

        if bytes.len() <= consumed {
            return Err(LiftError::Incomplete {
                addr: pc,
                have: bytes.len(),
                need: consumed + 1,
            });
        }
        let imm = bytes[consumed];

        let dst_reg = evex.dest_reg(modrm.reg);
        let src1_reg = evex.src1_reg();
        let src2_reg = evex.rm_reg(modrm.rm);
        let width = evex.vec_width();

        let dst = self.zmm(dst_reg);
        let src1 = self.zmm(src1_reg);
        let src2 = self.zmm(src2_reg);

        let op = SmirOp::new(
            ctx.next_op_id(),
            pc,
            OpKind::VMinMax {
                dst,
                src1,
                src2,
                elem,
                width,
                imm,
            },
        );

        Ok(LiftResult::fallthrough(
            vec![op],
            evex.bytes + 1 + consumed + 1,
        ))
    }

    /// Lift VMINMAXSS/VMINMAXSD (scalar)
    fn lift_vminmax_scalar(
        &self,
        evex: &EvexPrefix,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
        elem: VecElementType,
    ) -> Result<LiftResult, LiftError> {
        // Same as packed but only operates on lowest element
        self.lift_vminmax(evex, bytes, pc, ctx, elem)
    }

    // ========================================================================
    // AVX10.2 VMPSADBW
    // ========================================================================

    /// Lift VMPSADBW
    fn lift_vmpsadbw(
        &self,
        evex: &EvexPrefix,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let (modrm, consumed) = self.decode_modrm(bytes, pc)?;

        if bytes.len() <= consumed {
            return Err(LiftError::Incomplete {
                addr: pc,
                have: bytes.len(),
                need: consumed + 1,
            });
        }
        let imm = bytes[consumed];

        let dst_reg = evex.dest_reg(modrm.reg);
        let src1_reg = evex.src1_reg();
        let src2_reg = evex.rm_reg(modrm.rm);
        let width = evex.vec_width();

        let dst = self.zmm(dst_reg);
        let src1 = self.zmm(src1_reg);
        let src2 = self.zmm(src2_reg);

        let op = SmirOp::new(
            ctx.next_op_id(),
            pc,
            OpKind::VMpsadbw {
                dst,
                src1,
                src2,
                width,
                imm,
            },
        );

        Ok(LiftResult::fallthrough(
            vec![op],
            evex.bytes + 1 + consumed + 1,
        ))
    }

    // ========================================================================
    // AVX10.2 Media Acceleration (Byte)
    // ========================================================================

    /// Lift VPDPBSSD/S, VPDPBSUD/S, VPDPBUUD/S
    fn lift_vpdpbssd(
        &self,
        evex: &EvexPrefix,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
        src1_signed: bool,
        src2_signed: bool,
        saturate: bool,
    ) -> Result<LiftResult, LiftError> {
        let (modrm, consumed) = self.decode_modrm(bytes, pc)?;

        let dst_reg = evex.dest_reg(modrm.reg);
        let src1_reg = evex.src1_reg();
        let src2_reg = evex.rm_reg(modrm.rm);
        let width = evex.vec_width();

        let dst = self.zmm(dst_reg);
        let acc = dst;
        let src1 = self.zmm(src1_reg);
        let src2 = self.zmm(src2_reg);

        let op = SmirOp::new(
            ctx.next_op_id(),
            pc,
            OpKind::VDotProductExt {
                dst,
                acc,
                src1,
                src2,
                src_elem: VecElementType::I8,
                acc_elem: VecElementType::I32,
                width,
                src1_signed,
                src2_signed,
                saturate,
            },
        );

        Ok(LiftResult::fallthrough(vec![op], evex.bytes + 1 + consumed))
    }

    // ========================================================================
    // AVX10.2 Media Acceleration (Word)
    // ========================================================================

    /// Lift VPDPWSUD/S, VPDPWUSD/S, VPDPWUUD/S
    fn lift_vpdpwext(
        &self,
        evex: &EvexPrefix,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
        src1_signed: bool,
        src2_signed: bool,
        saturate: bool,
    ) -> Result<LiftResult, LiftError> {
        let (modrm, consumed) = self.decode_modrm(bytes, pc)?;

        let dst_reg = evex.dest_reg(modrm.reg);
        let src1_reg = evex.src1_reg();
        let src2_reg = evex.rm_reg(modrm.rm);
        let width = evex.vec_width();

        let dst = self.zmm(dst_reg);
        let acc = dst;
        let src1 = self.zmm(src1_reg);
        let src2 = self.zmm(src2_reg);

        let op = SmirOp::new(
            ctx.next_op_id(),
            pc,
            OpKind::VDotProductExt {
                dst,
                acc,
                src1,
                src2,
                src_elem: VecElementType::I16,
                acc_elem: VecElementType::I32,
                width,
                src1_signed,
                src2_signed,
                saturate,
            },
        );

        Ok(LiftResult::fallthrough(vec![op], evex.bytes + 1 + consumed))
    }

    // ========================================================================
    // Helpers
    // ========================================================================

    /// Create ZMM register reference
    fn zmm(&self, reg: u8) -> VReg {
        VReg::Arch(ArchReg::X86(X86Reg::Zmm(reg)))
    }

    /// Decode ModR/M byte
    fn decode_modrm(&self, bytes: &[u8], pc: u64) -> Result<(ModRm, usize), LiftError> {
        if bytes.is_empty() {
            return Err(LiftError::Incomplete {
                addr: pc,
                have: 0,
                need: 1,
            });
        }

        let modrm = bytes[0];
        let mod_bits = modrm >> 6;
        let reg = (modrm >> 3) & 0x07;
        let rm = modrm & 0x07;

        let is_memory = mod_bits != 3;
        let mut consumed = 1;

        // Handle SIB and displacement
        if is_memory {
            if rm == 4 {
                // SIB byte follows
                consumed += 1;
            }
            if mod_bits == 0 && rm == 5 {
                // disp32 (RIP-relative)
                consumed += 4;
            } else if mod_bits == 1 {
                consumed += 1; // disp8
            } else if mod_bits == 2 {
                consumed += 4; // disp32
            }
        }

        Ok((ModRm { reg, rm, is_memory }, consumed))
    }
}

impl Default for Avx10Lifter {
    fn default() -> Self {
        Self::new()
    }
}

/// Simplified ModR/M structure for AVX10 lifting
#[derive(Clone, Copy, Debug)]
pub struct ModRm {
    pub reg: u8,
    pub rm: u8,
    pub is_memory: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evex_decode() {
        // VPDPBUSD zmm1, zmm2, zmm3: 62 F2 6D 48 50 CB
        let bytes = [0x62, 0xF2, 0x6D, 0x48, 0x50, 0xCB];
        let evex = EvexPrefix::decode(&bytes).unwrap();

        assert_eq!(evex.map, 2); // 0F38
        assert_eq!(evex.pp, 1); // 66
        assert!(!evex.w);
        assert_eq!(evex.ll, 2); // 512-bit
        assert_eq!(evex.vec_width(), VecWidth::V512);
    }

    #[test]
    fn test_evex_decode_ymm() {
        // VPDPBUSD ymm1, ymm2, ymm3: 62 F2 6D 28 50 CB
        let bytes = [0x62, 0xF2, 0x6D, 0x28, 0x50, 0xCB];
        let evex = EvexPrefix::decode(&bytes).unwrap();

        assert_eq!(evex.ll, 1); // 256-bit
        assert_eq!(evex.vec_width(), VecWidth::V256);
    }

    #[test]
    fn test_lift_vpdpbusd() {
        let lifter = Avx10Lifter::new();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // VPDPBUSD zmm1, zmm2, zmm3
        let bytes = [0x62, 0xF2, 0x6D, 0x48, 0x50, 0xCB];
        let result = lifter.try_lift(&bytes, 0x1000, &mut ctx);

        assert!(result.is_some());
        let lift_result = result.unwrap().unwrap();
        assert_eq!(lift_result.bytes_consumed, 6);
        assert_eq!(lift_result.ops.len(), 1);

        match &lift_result.ops[0].kind {
            OpKind::VDotProduct {
                src_elem,
                acc_elem,
                saturate,
                ..
            } => {
                assert_eq!(*src_elem, VecElementType::I8);
                assert_eq!(*acc_elem, VecElementType::I32);
                assert!(!saturate);
            }
            _ => panic!("Expected VDotProduct"),
        }
    }
}
