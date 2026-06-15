//! AVX10.1 and AVX10.2 instruction lowering.
//!
//! This module lowers SMIR operations to EVEX-encoded AVX10 machine code.

use crate::smir::lower::{CodeBuffer, LowerError};
use crate::smir::ops::OpKind;
use crate::smir::types::*;

/// Result type for AVX10 lowering operations
pub type Avx10LowerResult<T> = Result<T, LowerError>;

// ============================================================================
// EVEX Encoder
// ============================================================================

/// EVEX instruction encoder
pub struct EvexEncoder<'a> {
    code: &'a mut CodeBuffer,
}

impl<'a> EvexEncoder<'a> {
    pub fn new(code: &'a mut CodeBuffer) -> Self {
        Self { code }
    }

    /// Encode EVEX prefix
    ///
    /// EVEX format:
    /// P0: 62h
    /// P1: R X B R' 0 0 m m
    /// P2: W v v v v 1 p p
    /// P3: z L' L b V' a a a
    pub fn emit_evex(
        &mut self,
        map: u8, // 1=0F, 2=0F38, 3=0F3A, 5=MAP5
        pp: u8,  // 0=none, 1=66, 2=F3, 3=F2
        w: bool,
        vl: VecWidth,
        dst: u8,  // destination register (0-31)
        src1: u8, // vvvv source register (0-31)
        src2: u8, // r/m source register (0-31)
        mask: u8, // opmask k0-k7
        zeroing: bool,
    ) {
        // Extract register bits
        let r = (dst >> 3) & 1; // bit 3 of dst
        let r_prime = (dst >> 4) & 1; // bit 4 of dst
        let x = (src2 >> 4) & 1; // bit 4 of src2 (index)
        let b = (src2 >> 3) & 1; // bit 3 of src2
        let vvvv = src1 & 0x0F;
        let v_prime = (src1 >> 4) & 1;

        let ll = match vl {
            VecWidth::V128 => 0,
            VecWidth::V256 => 1,
            VecWidth::V512 => 2,
            VecWidth::V64 => 0,
        };

        // Build P0
        self.code.emit_u8(0x62);

        // Build P1: ~R ~X ~B ~R' 0 0 m m
        let p1 =
            ((r ^ 1) << 7) | ((x ^ 1) << 6) | ((b ^ 1) << 5) | ((r_prime ^ 1) << 4) | (map & 0x03);
        self.code.emit_u8(p1);

        // Build P2: W ~vvvv 1 pp
        let vvvv_inv = (!vvvv) & 0x0F;
        let p2 = ((w as u8) << 7) | (vvvv_inv << 3) | 0x04 | (pp & 0x03);
        self.code.emit_u8(p2);

        // Build P3: z L'L b ~V' aaa
        let p3 = ((zeroing as u8) << 7)
            | (ll << 5)
            | 0 // b bit (broadcast) - could add later
            | ((v_prime ^ 1) << 3)
            | (mask & 0x07);
        self.code.emit_u8(p3);
    }

    /// Emit opcode byte
    pub fn emit_opcode(&mut self, opcode: u8) {
        self.code.emit_u8(opcode);
    }

    /// Emit immediate byte
    pub fn emit_imm8(&mut self, imm: u8) {
        self.code.emit_u8(imm);
    }

    /// Emit ModR/M byte for register-register operation
    pub fn emit_modrm_rr(&mut self, reg: u8, rm: u8) {
        let modrm = 0xC0 | ((reg & 0x07) << 3) | (rm & 0x07);
        self.code.emit_u8(modrm);
    }

    /// Emit ModR/M and optional SIB for memory operand
    pub fn emit_modrm_mem(&mut self, reg: u8, base: u8, disp: i32) {
        let reg_bits = reg & 0x07;
        let base_bits = base & 0x07;

        // Determine mod bits based on displacement
        let (mod_bits, disp_bytes) = if disp == 0 && base_bits != 5 {
            (0, 0)
        } else if disp >= -128 && disp <= 127 {
            (1, 1)
        } else {
            (2, 4)
        };

        // Check if SIB is needed (RSP/R12 as base)
        if base_bits == 4 {
            self.code.emit_u8((mod_bits << 6) | (reg_bits << 3) | 4);
            self.code.emit_u8(0x24); // SIB: scale=0, index=RSP(4), base=RSP(4)
        } else {
            self.code
                .emit_u8((mod_bits << 6) | (reg_bits << 3) | base_bits);
        }

        // Emit displacement
        match disp_bytes {
            1 => self.code.emit_i8(disp as i8),
            4 => self.code.emit_i32(disp),
            _ => {}
        }
    }
}

// ============================================================================
// AVX10 Lowerer
// ============================================================================

/// AVX10 instruction lowerer
pub struct Avx10Lowerer;

impl Avx10Lowerer {
    pub fn new() -> Self {
        Self
    }

    /// Try to lower an SMIR operation to AVX10 machine code
    /// Returns None if not an AVX10 operation
    pub fn try_lower(&self, op: &OpKind, code: &mut CodeBuffer) -> Option<Avx10LowerResult<()>> {
        match op {
            // AVX10.1 VNNI
            OpKind::VDotProduct {
                dst,
                src1,
                src2,
                src_elem,
                width,
                src1_unsigned,
                saturate,
                ..
            } => Some(self.lower_vdotproduct(
                code,
                dst,
                src1,
                src2,
                *src_elem,
                *width,
                *src1_unsigned,
                *saturate,
            )),

            // AVX10.1 IFMA
            OpKind::VMultiplyAdd52 {
                dst,
                src1,
                src2,
                width,
                high,
                ..
            } => Some(self.lower_vpmadd52(code, dst, src1, src2, *width, *high)),

            // AVX10.1 VPOPCNT
            OpKind::VPopcnt {
                dst,
                src,
                elem,
                width,
            } => Some(self.lower_vpopcnt(code, dst, src, *elem, *width)),

            // AVX10.1 VBMI permute
            OpKind::VPermute {
                dst,
                src1,
                src2,
                indices,
                elem,
                width,
                overwrite_table,
            } => Some(self.lower_vpermute(
                code,
                dst,
                src1,
                src2,
                indices,
                *elem,
                *width,
                *overwrite_table,
            )),

            // AVX10.1 BITALG
            OpKind::VShuffleBitQM {
                dst,
                src,
                indices,
                width,
            } => Some(self.lower_vpshufbitqmb(code, dst, src, indices, *width)),

            // AVX10.1 BF16
            OpKind::VDotProductBF16 {
                dst,
                src1,
                src2,
                width,
                ..
            } => Some(self.lower_vdpbf16ps(code, dst, src1, src2, *width)),

            OpKind::VCvtFP32ToBF16 {
                dst,
                src1,
                src2,
                width,
            } => Some(self.lower_vcvtfp32tobf16(code, dst, src1, src2.as_ref(), *width)),

            // AVX10.1 FP16
            OpKind::VFP16Arith {
                dst,
                src1,
                src2,
                op,
                width,
            } => Some(self.lower_vfp16_arith(code, dst, src1, src2, *op, *width)),

            // AVX10.2 saturation conversions
            OpKind::VCvtFpToIntSat {
                dst,
                src,
                fp_elem,
                int_elem,
                width,
                signed,
            } => Some(
                self.lower_vcvt_fp_to_int_sat(code, dst, src, *fp_elem, *int_elem, *width, *signed),
            ),

            // AVX10.2 VMINMAX
            OpKind::VMinMax {
                dst,
                src1,
                src2,
                elem,
                width,
                imm,
            } => Some(self.lower_vminmax(code, dst, src1, src2, *elem, *width, *imm)),

            // AVX10.2 VMPSADBW
            OpKind::VMpsadbw {
                dst,
                src1,
                src2,
                width,
                imm,
            } => Some(self.lower_vmpsadbw(code, dst, src1, src2, *width, *imm)),

            // AVX10.2 Media acceleration
            OpKind::VDotProductExt {
                dst,
                src1,
                src2,
                src_elem,
                width,
                src1_signed,
                src2_signed,
                saturate,
                ..
            } => Some(self.lower_vdotproduct_ext(
                code,
                dst,
                src1,
                src2,
                *src_elem,
                *width,
                *src1_signed,
                *src2_signed,
                *saturate,
            )),

            _ => None,
        }
    }

    // ========================================================================
    // VNNI Instructions
    // ========================================================================

    fn lower_vdotproduct(
        &self,
        code: &mut CodeBuffer,
        dst: &VReg,
        src1: &VReg,
        src2: &VReg,
        src_elem: VecElementType,
        width: VecWidth,
        src1_unsigned: bool,
        saturate: bool,
    ) -> Avx10LowerResult<()> {
        let dst_reg = self.vreg_to_zmm(dst)?;
        let src1_reg = self.vreg_to_zmm(src1)?;
        let src2_reg = self.vreg_to_zmm(src2)?;

        let opcode = match (src_elem, saturate) {
            (VecElementType::I8, false) => 0x50,  // VPDPBUSD
            (VecElementType::I8, true) => 0x51,   // VPDPBUSDS
            (VecElementType::I16, false) => 0x52, // VPDPWSSD
            (VecElementType::I16, true) => 0x53,  // VPDPWSSDS
            _ => {
                return Err(LowerError::UnsupportedOperation(
                    "VNNI: invalid element type".to_string(),
                ));
            }
        };

        let mut enc = EvexEncoder::new(code);
        enc.emit_evex(
            2,     // map 0F38
            1,     // pp = 66
            false, // W = 0
            width, dst_reg, src1_reg, src2_reg, 0,     // no mask
            false, // no zeroing
        );
        enc.emit_opcode(opcode);
        enc.emit_modrm_rr(dst_reg, src2_reg);

        Ok(())
    }

    // ========================================================================
    // IFMA Instructions
    // ========================================================================

    fn lower_vpmadd52(
        &self,
        code: &mut CodeBuffer,
        dst: &VReg,
        src1: &VReg,
        src2: &VReg,
        width: VecWidth,
        high: bool,
    ) -> Avx10LowerResult<()> {
        let dst_reg = self.vreg_to_zmm(dst)?;
        let src1_reg = self.vreg_to_zmm(src1)?;
        let src2_reg = self.vreg_to_zmm(src2)?;

        let opcode = if high { 0xB5 } else { 0xB4 };

        let mut enc = EvexEncoder::new(code);
        enc.emit_evex(
            2,    // map 0F38
            1,    // pp = 66
            true, // W = 1
            width, dst_reg, src1_reg, src2_reg, 0, false,
        );
        enc.emit_opcode(opcode);
        enc.emit_modrm_rr(dst_reg, src2_reg);

        Ok(())
    }

    // ========================================================================
    // VPOPCNT Instructions
    // ========================================================================

    fn lower_vpopcnt(
        &self,
        code: &mut CodeBuffer,
        dst: &VReg,
        src: &VReg,
        elem: VecElementType,
        width: VecWidth,
    ) -> Avx10LowerResult<()> {
        let dst_reg = self.vreg_to_zmm(dst)?;
        let src_reg = self.vreg_to_zmm(src)?;

        let (opcode, w) = match elem {
            VecElementType::I8 => (0x54, false),
            VecElementType::I16 => (0x54, true),
            VecElementType::I32 => (0x55, false),
            VecElementType::I64 => (0x55, true),
            _ => {
                return Err(LowerError::UnsupportedOperation(
                    "VPOPCNT: invalid element type".to_string(),
                ));
            }
        };

        let mut enc = EvexEncoder::new(code);
        enc.emit_evex(
            2, // map 0F38
            1, // pp = 66
            w, width, dst_reg, 0, // no vvvv source
            src_reg, 0, false,
        );
        enc.emit_opcode(opcode);
        enc.emit_modrm_rr(dst_reg, src_reg);

        Ok(())
    }

    // ========================================================================
    // VBMI Instructions
    // ========================================================================

    fn lower_vpermute(
        &self,
        code: &mut CodeBuffer,
        dst: &VReg,
        src1: &VReg,
        src2: &Option<VReg>,
        indices: &VReg,
        _elem: VecElementType,
        width: VecWidth,
        overwrite_table: bool,
    ) -> Avx10LowerResult<()> {
        let dst_reg = self.vreg_to_zmm(dst)?;
        let src1_reg = self.vreg_to_zmm(src1)?;
        let indices_reg = self.vreg_to_zmm(indices)?;

        let opcode = match (src2.is_some(), overwrite_table) {
            (false, _) => 0x8D,    // VPERMB
            (true, false) => 0x75, // VPERMI2B
            (true, true) => 0x7D,  // VPERMT2B
        };

        let src2_reg = if let Some(s2) = src2 {
            self.vreg_to_zmm(s2)?
        } else {
            indices_reg
        };

        let mut enc = EvexEncoder::new(code);
        enc.emit_evex(
            2,     // map 0F38
            1,     // pp = 66
            false, // W = 0
            width, dst_reg, src1_reg, src2_reg, 0, false,
        );
        enc.emit_opcode(opcode);
        enc.emit_modrm_rr(dst_reg, src2_reg);

        Ok(())
    }

    // ========================================================================
    // BITALG Instructions
    // ========================================================================

    fn lower_vpshufbitqmb(
        &self,
        code: &mut CodeBuffer,
        dst: &VReg,
        src: &VReg,
        indices: &VReg,
        width: VecWidth,
    ) -> Avx10LowerResult<()> {
        let dst_reg = self.vreg_to_k(dst)?;
        let src_reg = self.vreg_to_zmm(src)?;
        let indices_reg = self.vreg_to_zmm(indices)?;

        let mut enc = EvexEncoder::new(code);
        enc.emit_evex(
            2,     // map 0F38
            1,     // pp = 66
            false, // W = 0
            width,
            dst_reg,
            src_reg,
            indices_reg,
            0,
            false,
        );
        enc.emit_opcode(0x8F);
        enc.emit_modrm_rr(dst_reg, indices_reg);

        Ok(())
    }

    // ========================================================================
    // BF16 Instructions
    // ========================================================================

    fn lower_vdpbf16ps(
        &self,
        code: &mut CodeBuffer,
        dst: &VReg,
        src1: &VReg,
        src2: &VReg,
        width: VecWidth,
    ) -> Avx10LowerResult<()> {
        let dst_reg = self.vreg_to_zmm(dst)?;
        let src1_reg = self.vreg_to_zmm(src1)?;
        let src2_reg = self.vreg_to_zmm(src2)?;

        let mut enc = EvexEncoder::new(code);
        enc.emit_evex(
            2,     // map 0F38
            2,     // pp = F3
            false, // W = 0
            width, dst_reg, src1_reg, src2_reg, 0, false,
        );
        enc.emit_opcode(0x52);
        enc.emit_modrm_rr(dst_reg, src2_reg);

        Ok(())
    }

    fn lower_vcvtfp32tobf16(
        &self,
        code: &mut CodeBuffer,
        dst: &VReg,
        src1: &VReg,
        src2: Option<&VReg>,
        width: VecWidth,
    ) -> Avx10LowerResult<()> {
        let dst_reg = self.vreg_to_zmm(dst)?;
        let src1_reg = self.vreg_to_zmm(src1)?;

        let (pp, src2_reg) = if let Some(s2) = src2 {
            // VCVTNE2PS2BF16
            (3, self.vreg_to_zmm(s2)?) // F2
        } else {
            // VCVTNEPS2BF16
            (2, src1_reg) // F3
        };

        let mut enc = EvexEncoder::new(code);
        enc.emit_evex(
            2, // map 0F38
            pp, false, // W = 0
            width, dst_reg, src1_reg, src2_reg, 0, false,
        );
        enc.emit_opcode(0x72);
        enc.emit_modrm_rr(dst_reg, src2_reg);

        Ok(())
    }

    // ========================================================================
    // FP16 Instructions
    // ========================================================================

    fn lower_vfp16_arith(
        &self,
        code: &mut CodeBuffer,
        dst: &VReg,
        src1: &VReg,
        src2: &VReg,
        op: Avx10FP16Op,
        width: VecWidth,
    ) -> Avx10LowerResult<()> {
        let dst_reg = self.vreg_to_zmm(dst)?;
        let src1_reg = self.vreg_to_zmm(src1)?;
        let src2_reg = self.vreg_to_zmm(src2)?;

        let opcode = match op {
            Avx10FP16Op::Add => 0x58,
            Avx10FP16Op::Mul => 0x59,
            Avx10FP16Op::Sub => 0x5C,
            Avx10FP16Op::Div => 0x5E,
            _ => {
                return Err(LowerError::UnsupportedOperation(
                    "FP16: unsupported op".to_string(),
                ));
            }
        };

        let mut enc = EvexEncoder::new(code);
        enc.emit_evex(
            5,     // MAP5
            0,     // pp = none
            false, // W = 0
            width, dst_reg, src1_reg, src2_reg, 0, false,
        );
        enc.emit_opcode(opcode);
        enc.emit_modrm_rr(dst_reg, src2_reg);

        Ok(())
    }

    // ========================================================================
    // AVX10.2 Saturation Conversions
    // ========================================================================

    fn lower_vcvt_fp_to_int_sat(
        &self,
        code: &mut CodeBuffer,
        dst: &VReg,
        src: &VReg,
        fp_elem: VecElementType,
        int_elem: VecElementType,
        width: VecWidth,
        signed: bool,
    ) -> Avx10LowerResult<()> {
        let dst_reg = self.vreg_to_zmm(dst)?;
        let src_reg = self.vreg_to_zmm(src)?;

        let (opcode, pp, w) = match (fp_elem, int_elem, signed) {
            (VecElementType::F32, VecElementType::I8, true) => (0x68, 0, false), // VCVTTPS2IBS
            (VecElementType::F32, VecElementType::I8, false) => (0x6A, 0, false), // VCVTTPS2IUBS
            (VecElementType::F64, VecElementType::I64, true) => (0x6D, 1, true), // VCVTTPD2QQS
            (VecElementType::F64, VecElementType::I64, false) => (0x6C, 1, true), // VCVTTPD2UQQS
            _ => {
                return Err(LowerError::UnsupportedOperation(
                    "Saturation conversion: invalid types".to_string(),
                ));
            }
        };

        let mut enc = EvexEncoder::new(code);
        enc.emit_evex(
            2, // map 0F38
            pp, w, width, dst_reg, 0, src_reg, 0, false,
        );
        enc.emit_opcode(opcode);
        enc.emit_modrm_rr(dst_reg, src_reg);

        Ok(())
    }

    // ========================================================================
    // AVX10.2 VMINMAX
    // ========================================================================

    fn lower_vminmax(
        &self,
        code: &mut CodeBuffer,
        dst: &VReg,
        src1: &VReg,
        src2: &VReg,
        elem: VecElementType,
        width: VecWidth,
        imm: u8,
    ) -> Avx10LowerResult<()> {
        let dst_reg = self.vreg_to_zmm(dst)?;
        let src1_reg = self.vreg_to_zmm(src1)?;
        let src2_reg = self.vreg_to_zmm(src2)?;

        let (pp, w) = match elem {
            VecElementType::F32 => (0, false), // VMINMAXPS
            VecElementType::F64 => (1, true),  // VMINMAXPD
            _ => {
                return Err(LowerError::UnsupportedOperation(
                    "VMINMAX: invalid element type".to_string(),
                ));
            }
        };

        let mut enc = EvexEncoder::new(code);
        enc.emit_evex(
            3, // map 0F3A
            pp, w, width, dst_reg, src1_reg, src2_reg, 0, false,
        );
        enc.emit_opcode(0x52);
        enc.emit_modrm_rr(dst_reg, src2_reg);
        enc.emit_imm8(imm);

        Ok(())
    }

    // ========================================================================
    // AVX10.2 VMPSADBW
    // ========================================================================

    fn lower_vmpsadbw(
        &self,
        code: &mut CodeBuffer,
        dst: &VReg,
        src1: &VReg,
        src2: &VReg,
        width: VecWidth,
        imm: u8,
    ) -> Avx10LowerResult<()> {
        let dst_reg = self.vreg_to_zmm(dst)?;
        let src1_reg = self.vreg_to_zmm(src1)?;
        let src2_reg = self.vreg_to_zmm(src2)?;

        let mut enc = EvexEncoder::new(code);
        enc.emit_evex(
            3,     // map 0F3A
            1,     // pp = 66
            false, // W = 0
            width, dst_reg, src1_reg, src2_reg, 0, false,
        );
        enc.emit_opcode(0x42);
        enc.emit_modrm_rr(dst_reg, src2_reg);
        enc.emit_imm8(imm);

        Ok(())
    }

    // ========================================================================
    // AVX10.2 Media Acceleration
    // ========================================================================

    fn lower_vdotproduct_ext(
        &self,
        code: &mut CodeBuffer,
        dst: &VReg,
        src1: &VReg,
        src2: &VReg,
        src_elem: VecElementType,
        width: VecWidth,
        src1_signed: bool,
        src2_signed: bool,
        saturate: bool,
    ) -> Avx10LowerResult<()> {
        let dst_reg = self.vreg_to_zmm(dst)?;
        let src1_reg = self.vreg_to_zmm(src1)?;
        let src2_reg = self.vreg_to_zmm(src2)?;

        // Determine pp and W based on signedness
        let (pp, w) = match (src_elem, src1_signed, src2_signed) {
            // Byte variants
            (VecElementType::I8, true, true) => (2, false), // VPDPBSSD F3.W0
            (VecElementType::I8, true, false) => (2, true), // VPDPBSUD F3.W1
            (VecElementType::I8, false, false) => (0, true), // VPDPBUUD NP.W1
            // Word variants
            (VecElementType::I16, true, false) => (2, false), // VPDPWSUD F3.W0
            (VecElementType::I16, false, true) => (1, false), // VPDPWUSD 66.W0
            (VecElementType::I16, false, false) => (0, false), // VPDPWUUD NP.W0
            _ => {
                return Err(LowerError::UnsupportedOperation(
                    "Media accel: invalid types".to_string(),
                ));
            }
        };

        let opcode = match src_elem {
            VecElementType::I8 => {
                if saturate {
                    0x51
                } else {
                    0x50
                }
            }
            VecElementType::I16 => {
                if saturate {
                    0xD3
                } else {
                    0xD2
                }
            }
            _ => {
                return Err(LowerError::UnsupportedOperation(
                    "Media accel: invalid element".to_string(),
                ));
            }
        };

        let mut enc = EvexEncoder::new(code);
        enc.emit_evex(
            2, // map 0F38
            pp, w, width, dst_reg, src1_reg, src2_reg, 0, false,
        );
        enc.emit_opcode(opcode);
        enc.emit_modrm_rr(dst_reg, src2_reg);

        Ok(())
    }

    // ========================================================================
    // Helpers
    // ========================================================================

    fn vreg_to_zmm(&self, vreg: &VReg) -> Avx10LowerResult<u8> {
        match vreg {
            VReg::Arch(ArchReg::X86(X86Reg::Zmm(n))) => Ok(*n),
            VReg::Arch(ArchReg::X86(X86Reg::Ymm(n))) => Ok(*n),
            VReg::Arch(ArchReg::X86(X86Reg::Xmm(n))) => Ok(*n),
            _ => Err(LowerError::InvalidRegister(format!("{:?}", vreg))),
        }
    }

    fn vreg_to_k(&self, vreg: &VReg) -> Avx10LowerResult<u8> {
        match vreg {
            VReg::Arch(ArchReg::X86(X86Reg::K(n))) => Ok(*n),
            _ => Err(LowerError::InvalidRegister(format!("{:?}", vreg))),
        }
    }
}

impl Default for Avx10Lowerer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evex_encode_vpdpbusd() {
        let mut code = CodeBuffer::new();

        // VPDPBUSD zmm1, zmm2, zmm3 should be: 62 F2 6D 48 50 CB
        {
            let mut enc = EvexEncoder::new(&mut code);
            enc.emit_evex(
                2,     // map 0F38
                1,     // pp = 66
                false, // W = 0
                VecWidth::V512,
                1, // zmm1
                2, // zmm2
                3, // zmm3
                0,
                false,
            );
            enc.emit_opcode(0x50);
            enc.emit_modrm_rr(1, 3);
        }

        let bytes = code.as_slice();
        assert_eq!(bytes.len(), 6);
        assert_eq!(bytes[0], 0x62); // EVEX prefix
        assert_eq!(bytes[4], 0x50); // opcode
        assert_eq!(bytes[5], 0xCB); // ModR/M: 11 001 011
    }

    #[test]
    fn test_lower_vdotproduct() {
        let lowerer = Avx10Lowerer::new();
        let mut code = CodeBuffer::new();

        let op = OpKind::VDotProduct {
            dst: VReg::Arch(ArchReg::X86(X86Reg::Zmm(1))),
            acc: VReg::Arch(ArchReg::X86(X86Reg::Zmm(1))),
            src1: VReg::Arch(ArchReg::X86(X86Reg::Zmm(2))),
            src2: VReg::Arch(ArchReg::X86(X86Reg::Zmm(3))),
            src_elem: VecElementType::I8,
            acc_elem: VecElementType::I32,
            width: VecWidth::V512,
            src1_unsigned: true,
            saturate: false,
        };

        let result = lowerer.try_lower(&op, &mut code);
        assert!(result.is_some());
        assert!(result.unwrap().is_ok());
        assert!(code.len() > 0);
    }
}
