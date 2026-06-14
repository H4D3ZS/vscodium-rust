//! VEX-encoded (AVX) instruction dispatch for the x86_64 CPU emulator.

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::cpu::{InsnContext, X86_64Vcpu};
use super::super::insn;

impl X86_64Vcpu {
    /// Execute 2-byte VEX-encoded instructions (0xC5 prefix)
    pub(in crate::backend::emulator::x86_64) fn execute_vex2(
        &mut self,
        ctx: &mut InsnContext,
    ) -> Result<Option<VcpuExit>> {
        // VEX 2-byte prefix: 0xC5 [R vvvv L pp]
        // Implied: m-mmmm = 1 (0F escape), X = 1, B = 1, W = 0
        let vex1 = ctx.consume_u8()?;
        let opcode = ctx.consume_u8()?;

        let vex_r = (vex1 >> 7) & 1;
        let vex_l = (vex1 >> 2) & 1;
        let vex_pp = vex1 & 0x03;
        let vvvv = ((vex1 >> 3) & 0x0F) ^ 0x0F;

        // 2-byte VEX implies m-mmmm=1 (0F), X=B=1, W=0
        let m_mmmm: u8 = 1;
        let vex_w: u8 = 0;

        // Set up REX (R is from VEX, X and B are implied 1 which means REX.X=0, REX.B=0)
        let rex_r = (vex_r ^ 1) & 1;
        let rex = 0x40 | (rex_r << 2);
        ctx.rex = Some(rex);
        ctx.op_size = 4; // W=0 implies 32-bit operand size
        ctx.rip_relative_offset = 1;

        self.execute_vex_common(ctx, m_mmmm, vex_pp, vex_l, vex_w, vvvv, opcode)
    }

    /// Execute 3-byte VEX-encoded instructions (0xC4 prefix)
    pub(in crate::backend::emulator::x86_64) fn execute_vex3(
        &mut self,
        ctx: &mut InsnContext,
    ) -> Result<Option<VcpuExit>> {
        // VEX 3-byte prefix (0xC4)
        let vex1 = ctx.consume_u8()?;
        let vex2 = ctx.consume_u8()?;
        let opcode = ctx.consume_u8()?;

        let vex_r = (vex1 >> 7) & 1;
        let vex_x = (vex1 >> 6) & 1;
        let vex_b = (vex1 >> 5) & 1;
        let m_mmmm = vex1 & 0x1F;

        let vex_w = (vex2 >> 7) & 1;
        let vex_l = (vex2 >> 2) & 1;
        let vex_pp = vex2 & 0x03;

        // Set up REX and operand size from VEX
        let rex_r = (vex_r ^ 1) & 1;
        let rex_x = (vex_x ^ 1) & 1;
        let rex_b = (vex_b ^ 1) & 1;
        let mut rex = 0x40 | (rex_r << 2) | (rex_x << 1) | rex_b;
        if vex_w != 0 {
            rex |= 0x08;
        }
        ctx.rex = Some(rex);
        ctx.op_size = if vex_w != 0 { 8 } else { 4 };
        ctx.rip_relative_offset = 1;

        // VEX.vvvv register (inverted in VEX encoding)
        let vvvv = ((vex2 >> 3) & 0x0F) ^ 0x0F;

        self.execute_vex_common(ctx, m_mmmm, vex_pp, vex_l, vex_w, vvvv, opcode)
    }

    /// Common VEX instruction execution logic
    fn execute_vex_common(
        &mut self,
        ctx: &mut InsnContext,
        m_mmmm: u8,
        vex_pp: u8,
        vex_l: u8,
        vex_w: u8,
        vvvv: u8,
        opcode: u8,
    ) -> Result<Option<VcpuExit>> {
        // Record precise opcode key for profiling
        #[cfg(feature = "profiling")]
        crate::profiling::set_current_opcode_key(crate::profiling::OpcodeKey::Vex {
            map: m_mmmm,
            opcode,
        });

        // Note: We allow VEX.L=1 (256-bit YMM) operations as we have implementations
        // for the common AVX instructions. The individual handlers support L=1.
        // We reject EVEX (AVX-512) separately in the EVEX dispatcher.

        // VEX.LZ.F2.0F3A.W{0,1} F0 /r ib (RORX)
        if m_mmmm == 0x3 && vex_pp == 0x3 && vex_l == 0 && opcode == 0xF0 {
            return insn::bmi::rorx(self, ctx);
        }

        // VEX.LZ.0F38 BMI1/BMI2 instructions
        if m_mmmm == 0x2 && vex_l == 0 {
            match (vex_pp, opcode) {
                // TBM group: VEX.NDD.LZ.0F38 01 /1,/2,/3,/4,/6,/7
                (0, 0x01) => return insn::bmi::tbm_01_group(self, ctx, vvvv),
                // BLCI: VEX.NDD.LZ.0F38 02 /6
                (0, 0x02) => return insn::bmi::tbm_blci(self, ctx, vvvv),
                // ANDN: VEX.LZ.0F38.W{0,1} F2 /r
                (0, 0xF2) => return insn::bmi::andn(self, ctx, vvvv),
                // BLSI/BLSMSK/BLSR: VEX.LZ.0F38.W{0,1} F3 /1,/2,/3
                (0, 0xF3) => return insn::bmi::blsi_blsmsk_blsr(self, ctx, vvvv),
                // BZHI: VEX.LZ.0F38.W{0,1} F5 /r
                (0, 0xF5) => return insn::bmi::bzhi(self, ctx, vvvv),
                // BEXTR: VEX.LZ.0F38.W{0,1} F7 /r
                (0, 0xF7) => return insn::bmi::bextr(self, ctx, vvvv),
                // MULX: VEX.LZ.F2.0F38.W{0,1} F6 /r
                (3, 0xF6) => return insn::bmi::mulx(self, ctx, vvvv),
                // PDEP: VEX.LZ.F2.0F38.W{0,1} F5 /r
                (3, 0xF5) => return insn::bmi::pdep(self, ctx, vvvv),
                // PEXT: VEX.LZ.F3.0F38.W{0,1} F5 /r
                (2, 0xF5) => return insn::bmi::pext(self, ctx, vvvv),
                // SARX: VEX.LZ.F3.0F38.W{0,1} F7 /r
                (2, 0xF7) => return insn::bmi::sarx(self, ctx, vvvv),
                // SHRX: VEX.LZ.F2.0F38.W{0,1} F7 /r
                (3, 0xF7) => return insn::bmi::shrx(self, ctx, vvvv),
                // SHLX: VEX.LZ.66.0F38.W{0,1} F7 /r
                (1, 0xF7) => return insn::bmi::shlx(self, ctx, vvvv),
                _ => {}
            }
        }

        // VEX.0F38 SIMD instructions (m=2) - variable shifts (supports L=0/1)
        if m_mmmm == 0x2 && vex_pp == 1 {
            match opcode {
                // VPSRLVD/VPSRLVQ (0x45), VPSRAVD (0x46), VPSLLVD/VPSLLVQ (0x47)
                0x45 | 0x46 | 0x47 => {
                    return self.execute_vex_variable_shift(ctx, vex_l, vvvv, vex_w, opcode);
                }
                _ => {}
            }
        }

        // VEX.0F encoded SSE/AVX instructions (m=1)
        if m_mmmm == 0x1 {
            // AVX-512 opmask (KMOV) instructions - VEX.L0.0F
            if vex_l == 0 {
                match (vex_pp, vex_w, opcode) {
                    // KMOVW k1, k2/m16 - VEX.L0.0F.W0 90 /r
                    (0, 0, 0x90) => return self.execute_kmov_load(ctx, 16),
                    // KMOVB k1, k2/m8 - VEX.L0.66.0F.W0 90 /r
                    (1, 0, 0x90) => return self.execute_kmov_load(ctx, 8),
                    // KMOVD k1, k2/m32 - VEX.L0.66.0F.W1 90 /r
                    (1, 1, 0x90) => return self.execute_kmov_load(ctx, 32),
                    // KMOVD k1, k2/m32 - VEX.L0.F2.0F.W0 90 /r (alternate encoding)
                    (3, 0, 0x90) => return self.execute_kmov_load(ctx, 32),
                    // KMOVQ k1, k2/m64 - VEX.L0.F2.0F.W1 90 /r
                    (3, 1, 0x90) => return self.execute_kmov_load(ctx, 64),

                    // KMOVW m16, k1 - VEX.L0.0F.W0 91 /r
                    (0, 0, 0x91) => return self.execute_kmov_store(ctx, 16),
                    // KMOVB m8, k1 - VEX.L0.66.0F.W0 91 /r
                    (1, 0, 0x91) => return self.execute_kmov_store(ctx, 8),
                    // KMOVD m32, k1 - VEX.L0.66.0F.W1 91 /r
                    (1, 1, 0x91) => return self.execute_kmov_store(ctx, 32),
                    // KMOVD m32, k1 - VEX.L0.F2.0F.W0 91 /r
                    (3, 0, 0x91) => return self.execute_kmov_store(ctx, 32),
                    // KMOVQ m64, k1 - VEX.L0.F2.0F.W1 91 /r
                    (3, 1, 0x91) => return self.execute_kmov_store(ctx, 64),

                    // KMOVW k1, r32 - VEX.L0.0F.W0 92 /r
                    (0, 0, 0x92) => return self.execute_kmov_from_gpr(ctx, 16),
                    // KMOVB k1, r32 - VEX.L0.66.0F.W0 92 /r
                    (1, 0, 0x92) => return self.execute_kmov_from_gpr(ctx, 8),
                    // KMOVD k1, r32 - VEX.L0.F2.0F.W0 92 /r
                    (3, 0, 0x92) => return self.execute_kmov_from_gpr(ctx, 32),
                    // KMOVQ k1, r64 - VEX.L0.F2.0F.W1 92 /r
                    (3, 1, 0x92) => return self.execute_kmov_from_gpr(ctx, 64),

                    // KMOVW r32, k1 - VEX.L0.0F.W0 93 /r
                    (0, 0, 0x93) => return self.execute_kmov_to_gpr(ctx, 16),
                    // KMOVB r32, k1 - VEX.L0.66.0F.W0 93 /r
                    (1, 0, 0x93) => return self.execute_kmov_to_gpr(ctx, 8),
                    // KMOVD r32, k1 - VEX.L0.F2.0F.W0 93 /r
                    (3, 0, 0x93) => return self.execute_kmov_to_gpr(ctx, 32),
                    // KMOVQ r64, k1 - VEX.L0.F2.0F.W1 93 /r
                    (3, 1, 0x93) => return self.execute_kmov_to_gpr(ctx, 64),

                    _ => {}
                }
            }

            // AVX-512 opmask logical instructions - VEX.L1.0F
            if vex_l == 1 {
                // Determine mask size from pp and W:
                // pp=0, W=0: 16-bit (W suffix)
                // pp=0, W=1: 64-bit (Q suffix)
                // pp=1, W=0: 8-bit (B suffix)
                // pp=1, W=1: 32-bit (D suffix)
                let mask_bits = match (vex_pp, vex_w) {
                    (0, 0) => 16,
                    (0, 1) => 64,
                    (1, 0) => 8,
                    (1, 1) => 32,
                    _ => 0, // pp=2 or pp=3 not used for these ops
                };

                if mask_bits > 0 {
                    match opcode {
                        // KAND* - bitwise AND
                        0x41 => {
                            return self.execute_kmask_binop(ctx, vvvv, mask_bits, |a, b| a & b);
                        }
                        // KANDN* - bitwise AND NOT
                        0x42 => {
                            return self.execute_kmask_binop(ctx, vvvv, mask_bits, |a, b| !a & b);
                        }
                        // KNOT* - bitwise NOT (unary, vvvv should be 1111)
                        0x44 => return self.execute_kmask_unaryop(ctx, mask_bits, |a| !a),
                        // KOR* - bitwise OR
                        0x45 => {
                            return self.execute_kmask_binop(ctx, vvvv, mask_bits, |a, b| a | b);
                        }
                        // KXNOR* - bitwise XNOR
                        0x46 => {
                            return self.execute_kmask_binop(ctx, vvvv, mask_bits, |a, b| !(a ^ b));
                        }
                        // KXOR* - bitwise XOR
                        0x47 => {
                            return self.execute_kmask_binop(ctx, vvvv, mask_bits, |a, b| a ^ b);
                        }
                        // KADD* - add (wrapping)
                        0x4A => {
                            return self.execute_kmask_binop(ctx, vvvv, mask_bits, |a, b| {
                                a.wrapping_add(b)
                            });
                        }
                        _ => {}
                    }
                }
            }

            match (vex_pp, opcode) {
                // VMOVD/VMOVQ load - VEX.66.0F 6E /r (GPR/mem -> xmm)
                (1, 0x6E) => return insn::simd::vmovd_load(self, ctx, vex_w),
                // VMOVD/VMOVQ store - VEX.66.0F 7E /r (xmm -> GPR/mem)
                (1, 0x7E) => return insn::simd::vmovd_store(self, ctx, vex_w),
                // VMOVQ load - VEX.F3.0F 7E /r (xmm/m64 -> xmm, zero-extend)
                (2, 0x7E) => return insn::simd::vmovq_load(self, ctx),
                // VMOVQ store - VEX.66.0F D6 /r (xmm -> xmm/m64)
                (1, 0xD6) => return insn::simd::vmovq_store(self, ctx),
                // VMOVDQA load - VEX.66.0F 6F /r
                (1, 0x6F) => return insn::simd::vmovdqa_load(self, ctx, vex_l),
                // VMOVDQU load - VEX.F3.0F 6F /r
                (2, 0x6F) => return insn::simd::vmovdqu_load(self, ctx, vex_l),
                // VMOVDQA store - VEX.66.0F 7F /r
                (1, 0x7F) => return insn::simd::vmovdqa_store(self, ctx, vex_l),
                // VMOVDQU store - VEX.F3.0F 7F /r
                (2, 0x7F) => return insn::simd::vmovdqu_store(self, ctx, vex_l),

                // VEX arithmetic: VADDPS/PD (0x58), VMULPS/PD (0x59), VSUBPS/PD (0x5C), VDIVPS/PD (0x5E)
                // VSQRTPS/PD (0x51), VMINPS/PD (0x5D), VMAXPS/PD (0x5F)
                // pp=0: PS (packed single), pp=1: PD (packed double)
                // pp=2: SS (scalar single), pp=3: SD (scalar double)
                (_, 0x51) | (_, 0x58) | (_, 0x59) | (_, 0x5C) | (_, 0x5D) | (_, 0x5E) | (_, 0x5F) => {
                    return self.execute_vex_arith(ctx, vex_pp, vex_l, vvvv, opcode);
                }

                // VEX logical: VANDPS/PD (0x54), VANDNPS/PD (0x55), VORPS/PD (0x56), VXORPS/PD (0x57)
                (_, 0x54) | (_, 0x55) | (_, 0x56) | (_, 0x57) => {
                    return self.execute_vex_logical(ctx, vex_pp, vex_l, vvvv, opcode);
                }

                // VEX unpack: VUNPCKLPS/PD (0x14), VUNPCKHPS/PD (0x15)
                (_, 0x14) | (_, 0x15) => {
                    return self.execute_vex_unpack(ctx, vex_pp, vex_l, vvvv, opcode);
                }

                // VEX conversion 0x5A: VCVTPS2PD, VCVTPD2PS, VCVTSS2SD, VCVTSD2SS
                (_, 0x5A) => {
                    return self.execute_vex_cvt_fp(ctx, vex_pp, vex_l, vvvv);
                }

                // VEX conversion 0x5B: VCVTDQ2PS, VCVTPS2DQ, VCVTTPS2DQ
                (_, 0x5B) => {
                    return self.execute_vex_cvt_dq_ps(ctx, vex_pp, vex_l);
                }

                // VEX conversion 0xE6: VCVTTPD2DQ, VCVTDQ2PD, VCVTPD2DQ
                (_, 0xE6) => {
                    return self.execute_vex_cvt_pd_dq(ctx, vex_pp, vex_l);
                }

                // VEX scalar int-to-float: VCVTSI2SS (0x2A with F3), VCVTSI2SD (0x2A with F2)
                (2, 0x2A) | (3, 0x2A) => {
                    return self.execute_vex_cvtsi2s(ctx, vex_pp, vex_w, vvvv);
                }

                // VEX truncating scalar float-to-int: VCVTTSS2SI (0x2C with F3), VCVTTSD2SI (0x2C with F2)
                (2, 0x2C) | (3, 0x2C) => {
                    return self.execute_vex_cvtts2si(ctx, vex_pp, vex_w);
                }

                // VEX rounding scalar float-to-int: VCVTSS2SI (0x2D with F3), VCVTSD2SI (0x2D with F2)
                (2, 0x2D) | (3, 0x2D) => {
                    return self.execute_vex_cvts2si(ctx, vex_pp, vex_w);
                }

                // VEX scalar moves: VMOVSS/VMOVSD (0x10/0x11 with F3/F2)
                (2, 0x10) | (2, 0x11) | (3, 0x10) | (3, 0x11) => {
                    return self.execute_vex_movss_sd(ctx, vex_pp, vex_l, vvvv, opcode);
                }

                // VEX duplicate moves: VMOVSLDUP/VMOVSHDUP/VMOVDDUP
                (2, 0x12) => {
                    return self.execute_vex_movsldup(ctx, vex_l, vvvv);
                }
                (2, 0x16) => {
                    return self.execute_vex_movshdup(ctx, vex_l, vvvv);
                }
                (3, 0x12) => {
                    return self.execute_vex_movddup(ctx, vex_l, vvvv);
                }

                // VSHUFPS (0xC6 with NP), VSHUFPD (0xC6 with 66)
                (0, 0xC6) | (1, 0xC6) => {
                    return self.execute_vex_shufp(ctx, vex_pp, vex_l, vvvv);
                }

                // VMOVMSKPS (0x50 with NP), VMOVMSKPD (0x50 with 66)
                (0, 0x50) | (1, 0x50) => {
                    return self.execute_vex_movmskp(ctx, vex_pp, vex_l);
                }
                // VPMOVMSKB (0xD7 with 66)
                (1, 0xD7) => {
                    return self.execute_vex_pmovmskb(ctx, vex_l, vvvv);
                }

                // VRSQRTPS (0x52 with NP), VRSQRTSS (0x52 with F3)
                (0, 0x52) | (2, 0x52) => {
                    return self.execute_vex_rsqrt(ctx, vex_pp, vex_l, vvvv);
                }

                // VRCPPS (0x53 with NP), VRCPSS (0x53 with F3)
                (0, 0x53) | (2, 0x53) => {
                    return self.execute_vex_rcp(ctx, vex_pp, vex_l, vvvv);
                }

                // VZEROUPPER/VZEROALL (0x77)
                (0, 0x77) => {
                    return self.execute_vex_vzero(ctx, vex_l);
                }

                // VLDMXCSR/VSTMXCSR (0xAE)
                (0, 0xAE) => {
                    return self.execute_vex_ldst_mxcsr(ctx);
                }

                // VADDSUBPD (0xD0 with 66), VADDSUBPS (0xD0 with F2)
                (1, 0xD0) | (3, 0xD0) => {
                    return self.execute_vex_addsubp(ctx, vex_pp, vex_l, vvvv);
                }

                // VHADDPD (0x7C with 66), VHADDPS (0x7C with F2)
                (1, 0x7C) | (3, 0x7C) => {
                    return self.execute_vex_haddp(ctx, vex_pp, vex_l, vvvv);
                }

                // VHSUBPD (0x7D with 66), VHSUBPS (0x7D with F2)
                (1, 0x7D) | (3, 0x7D) => {
                    return self.execute_vex_hsubp(ctx, vex_pp, vex_l, vvvv);
                }

                // VEX move: VMOVAPS/VMOVUPS (0x28/0x29/0x10/0x11)
                (0, 0x10) | (0, 0x28) => {
                    // VMOVUPS (0x10) or VMOVAPS (0x28) load
                    let aligned = opcode == 0x28;
                    let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                    let xmm_dst = reg as usize;

                    if vex_l == 0 {
                        if is_memory {
                            if aligned && addr & 0xF != 0 {
                                return Err(Error::Emulator(format!(
                                    "VMOVAPS: unaligned memory access at {:#x}", addr
                                )));
                            }
                            self.regs.xmm[xmm_dst][0] = self.read_mem(addr, 8)?;
                            self.regs.xmm[xmm_dst][1] = self.read_mem(addr + 8, 8)?;
                        } else {
                            let xmm_src = rm as usize;
                            self.regs.xmm[xmm_dst][0] = self.regs.xmm[xmm_src][0];
                            self.regs.xmm[xmm_dst][1] = self.regs.xmm[xmm_src][1];
                        }
                        self.regs.ymm_high[xmm_dst][0] = 0;
                        self.regs.ymm_high[xmm_dst][1] = 0;
                    } else {
                        if is_memory {
                            if aligned && addr & 0x1F != 0 {
                                return Err(Error::Emulator(format!(
                                    "VMOVAPS: unaligned memory access at {:#x}", addr
                                )));
                            }
                            self.regs.xmm[xmm_dst][0] = self.read_mem(addr, 8)?;
                            self.regs.xmm[xmm_dst][1] = self.read_mem(addr + 8, 8)?;
                            self.regs.ymm_high[xmm_dst][0] = self.read_mem(addr + 16, 8)?;
                            self.regs.ymm_high[xmm_dst][1] = self.read_mem(addr + 24, 8)?;
                        } else {
                            let xmm_src = rm as usize;
                            self.regs.xmm[xmm_dst][0] = self.regs.xmm[xmm_src][0];
                            self.regs.xmm[xmm_dst][1] = self.regs.xmm[xmm_src][1];
                            self.regs.ymm_high[xmm_dst][0] = self.regs.ymm_high[xmm_src][0];
                            self.regs.ymm_high[xmm_dst][1] = self.regs.ymm_high[xmm_src][1];
                        }
                    }
                    self.regs.rip += ctx.cursor as u64;
                    return Ok(None);
                }
                (0, 0x11) | (0, 0x29) => {
                    // VMOVUPS (0x11) or VMOVAPS (0x29) store
                    let aligned = opcode == 0x29;
                    let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                    let xmm_src = reg as usize;

                    if vex_l == 0 {
                        if is_memory {
                            if aligned && addr & 0xF != 0 {
                                return Err(Error::Emulator(format!(
                                    "VMOVAPS: unaligned memory access at {:#x}", addr
                                )));
                            }
                            self.write_mem(addr, self.regs.xmm[xmm_src][0], 8)?;
                            self.write_mem(addr + 8, self.regs.xmm[xmm_src][1], 8)?;
                        } else {
                            let xmm_dst = rm as usize;
                            self.regs.xmm[xmm_dst][0] = self.regs.xmm[xmm_src][0];
                            self.regs.xmm[xmm_dst][1] = self.regs.xmm[xmm_src][1];
                            self.regs.ymm_high[xmm_dst][0] = 0;
                            self.regs.ymm_high[xmm_dst][1] = 0;
                        }
                    } else {
                        if is_memory {
                            if aligned && addr & 0x1F != 0 {
                                return Err(Error::Emulator(format!(
                                    "VMOVAPS: unaligned memory access at {:#x}", addr
                                )));
                            }
                            self.write_mem(addr, self.regs.xmm[xmm_src][0], 8)?;
                            self.write_mem(addr + 8, self.regs.xmm[xmm_src][1], 8)?;
                            self.write_mem(addr + 16, self.regs.ymm_high[xmm_src][0], 8)?;
                            self.write_mem(addr + 24, self.regs.ymm_high[xmm_src][1], 8)?;
                        } else {
                            let xmm_dst = rm as usize;
                            self.regs.xmm[xmm_dst][0] = self.regs.xmm[xmm_src][0];
                            self.regs.xmm[xmm_dst][1] = self.regs.xmm[xmm_src][1];
                            self.regs.ymm_high[xmm_dst][0] = self.regs.ymm_high[xmm_src][0];
                            self.regs.ymm_high[xmm_dst][1] = self.regs.ymm_high[xmm_src][1];
                        }
                    }
                    self.regs.rip += ctx.cursor as u64;
                    return Ok(None);
                }
                // VMOVAPD/VMOVUPD (pp=1, 66 prefix)
                (1, 0x10) | (1, 0x28) => {
                    let aligned = opcode == 0x28;
                    let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                    let xmm_dst = reg as usize;

                    if vex_l == 0 {
                        if is_memory {
                            if aligned && addr & 0xF != 0 {
                                return Err(Error::Emulator(format!(
                                    "VMOVAPD: unaligned memory access at {:#x}", addr
                                )));
                            }
                            self.regs.xmm[xmm_dst][0] = self.read_mem(addr, 8)?;
                            self.regs.xmm[xmm_dst][1] = self.read_mem(addr + 8, 8)?;
                        } else {
                            let xmm_src = rm as usize;
                            self.regs.xmm[xmm_dst][0] = self.regs.xmm[xmm_src][0];
                            self.regs.xmm[xmm_dst][1] = self.regs.xmm[xmm_src][1];
                        }
                        self.regs.ymm_high[xmm_dst][0] = 0;
                        self.regs.ymm_high[xmm_dst][1] = 0;
                    } else {
                        if is_memory {
                            if aligned && addr & 0x1F != 0 {
                                return Err(Error::Emulator(format!(
                                    "VMOVAPD: unaligned memory access at {:#x}", addr
                                )));
                            }
                            self.regs.xmm[xmm_dst][0] = self.read_mem(addr, 8)?;
                            self.regs.xmm[xmm_dst][1] = self.read_mem(addr + 8, 8)?;
                            self.regs.ymm_high[xmm_dst][0] = self.read_mem(addr + 16, 8)?;
                            self.regs.ymm_high[xmm_dst][1] = self.read_mem(addr + 24, 8)?;
                        } else {
                            let xmm_src = rm as usize;
                            self.regs.xmm[xmm_dst][0] = self.regs.xmm[xmm_src][0];
                            self.regs.xmm[xmm_dst][1] = self.regs.xmm[xmm_src][1];
                            self.regs.ymm_high[xmm_dst][0] = self.regs.ymm_high[xmm_src][0];
                            self.regs.ymm_high[xmm_dst][1] = self.regs.ymm_high[xmm_src][1];
                        }
                    }
                    self.regs.rip += ctx.cursor as u64;
                    return Ok(None);
                }
                (1, 0x11) | (1, 0x29) => {
                    let aligned = opcode == 0x29;
                    let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                    let xmm_src = reg as usize;

                    if vex_l == 0 {
                        if is_memory {
                            if aligned && addr & 0xF != 0 {
                                return Err(Error::Emulator(format!(
                                    "VMOVAPD: unaligned memory access at {:#x}", addr
                                )));
                            }
                            self.write_mem(addr, self.regs.xmm[xmm_src][0], 8)?;
                            self.write_mem(addr + 8, self.regs.xmm[xmm_src][1], 8)?;
                        } else {
                            let xmm_dst = rm as usize;
                            self.regs.xmm[xmm_dst][0] = self.regs.xmm[xmm_src][0];
                            self.regs.xmm[xmm_dst][1] = self.regs.xmm[xmm_src][1];
                            self.regs.ymm_high[xmm_dst][0] = 0;
                            self.regs.ymm_high[xmm_dst][1] = 0;
                        }
                    } else {
                        if is_memory {
                            if aligned && addr & 0x1F != 0 {
                                return Err(Error::Emulator(format!(
                                    "VMOVAPD: unaligned memory access at {:#x}", addr
                                )));
                            }
                            self.write_mem(addr, self.regs.xmm[xmm_src][0], 8)?;
                            self.write_mem(addr + 8, self.regs.xmm[xmm_src][1], 8)?;
                            self.write_mem(addr + 16, self.regs.ymm_high[xmm_src][0], 8)?;
                            self.write_mem(addr + 24, self.regs.ymm_high[xmm_src][1], 8)?;
                        } else {
                            let xmm_dst = rm as usize;
                            self.regs.xmm[xmm_dst][0] = self.regs.xmm[xmm_src][0];
                            self.regs.xmm[xmm_dst][1] = self.regs.xmm[xmm_src][1];
                            self.regs.ymm_high[xmm_dst][0] = self.regs.ymm_high[xmm_src][0];
                            self.regs.ymm_high[xmm_dst][1] = self.regs.ymm_high[xmm_src][1];
                        }
                    }
                    self.regs.rip += ctx.cursor as u64;
                    return Ok(None);
                }

                // VPSHUFD/VPSHUFHW/VPSHUFLW (0x70)
                (_, 0x70) => {
                    return self.execute_vex_shuffle(ctx, vex_pp, vex_l);
                }

                // VCMPPS/VCMPPD/VCMPSS/VCMPSD (0xC2)
                (_, 0xC2) => {
                    return self.execute_vex_cmp(ctx, vex_pp, vex_l, vvvv);
                }

                // VCOMISS/VUCOMISS/VCOMISD/VUCOMISD (0x2E/0x2F)
                (0, 0x2E) | (1, 0x2E) => {
                    return self.execute_vex_ucomis(ctx, vex_pp);
                }
                (0, 0x2F) | (1, 0x2F) => {
                    return self.execute_vex_comis(ctx, vex_pp);
                }

                // VMOVLPS/VMOVHPS/VMOVHLPS/VMOVLHPS (0x12/0x16 with NP)
                (0, 0x12) | (0, 0x16) => {
                    return self.execute_vex_movlps_hps(ctx, vex_l, vvvv, opcode);
                }
                // VMOVLPD/VMOVHPD (0x12/0x16 with 66)
                (1, 0x12) | (1, 0x16) => {
                    return self.execute_vex_movlpd_hpd(ctx, vex_l, vvvv, opcode);
                }

                // VEX packed integer shift by immediate (Group 12/13/14)
                // VPSLLW/VPSRAW/VPSRLW (0x71), VPSLLD/VPSRAD/VPSRLD (0x72), VPSLLQ/VPSRLQ/VPSLLDQ/VPSRLDQ (0x73)
                (1, 0x71) | (1, 0x72) | (1, 0x73) => {
                    return self.execute_vex_packed_shift_imm(ctx, vex_l, vvvv, opcode);
                }

                // VEX packed integer shift by XMM count
                // VPSRLW (0xD1), VPSRLD (0xD2), VPSRLQ (0xD3) - right logical
                // VPSRAW (0xE1), VPSRAD (0xE2) - right arithmetic
                // VPSLLW (0xF1), VPSLLD (0xF2), VPSLLQ (0xF3) - left logical
                (1, 0xD1) | (1, 0xD2) | (1, 0xD3) |
                (1, 0xE1) | (1, 0xE2) |
                (1, 0xF1) | (1, 0xF2) | (1, 0xF3) => {
                    return self.execute_vex_packed_shift_xmm(ctx, vex_l, vvvv, opcode);
                }

                // AVX2 packed integer unpack instructions (0x60-0x62, 0x68-0x6D)
                (1, 0x60) | (1, 0x61) | (1, 0x62) |
                (1, 0x68) | (1, 0x69) | (1, 0x6A) |
                (1, 0x6C) | (1, 0x6D) => {
                    return self.execute_vex_punpck(ctx, vex_l, vvvv, opcode);
                }

                // VPACKSSWB/VPACKUSWB/VPACKSSDW
                (1, 0x63) => {
                    return self.execute_vex_packsswb(ctx, vex_l, vvvv);
                }
                (1, 0x67) => {
                    return self.execute_vex_packuswb(ctx, vex_l, vvvv);
                }
                (1, 0x6B) => {
                    return self.execute_vex_packssdw(ctx, vex_l, vvvv);
                }

                // AVX2 packed integer compare (0x74-0x76)
                (1, 0x74) | (1, 0x75) | (1, 0x76) => {
                    return self.execute_vex_pcmpeq(ctx, vex_l, vvvv, opcode);
                }
                // AVX2 packed integer compare greater-than (0x64-0x66)
                (1, 0x64) | (1, 0x65) | (1, 0x66) => {
                    return self.execute_vex_pcmpgt(ctx, vex_l, vvvv, opcode);
                }

                // AVX2 packed integer arithmetic (0xD4-0xFE)
                (1, 0xD4) | (1, 0xD5) | // VPADDQ, VPMULLW
                (1, 0xD8) | (1, 0xD9) | // VPSUBUSB, VPSUBUSW
                (1, 0xDA) | (1, 0xDB) | // VPMINUB, VPAND
                (1, 0xDC) | (1, 0xDD) | // VPADDUSB, VPADDUSW
                (1, 0xDE) | (1, 0xDF) | // VPMAXUB, VPANDN
                (1, 0xE0) | (1, 0xE3) | // VPAVGB, VPAVGW
                (1, 0xE4) | (1, 0xE5) | // VPMULHUW, VPMULHW
                (1, 0xE8) | (1, 0xE9) | // VPSUBSB, VPSUBSW
                (1, 0xEA) | (1, 0xEB) | // VPMINSW, VPOR
                (1, 0xEC) | (1, 0xED) | // VPADDSB, VPADDSW
                (1, 0xEE) | (1, 0xEF) | // VPMAXSW, VPXOR
                (1, 0xF4) | (1, 0xF5) | // VPMULUDQ, VPMADDWD
                (1, 0xF6) | // VPSADBW
                (1, 0xF8) | (1, 0xF9) | (1, 0xFA) | (1, 0xFB) | // VPSUBB/W/D/Q
                (1, 0xFC) | (1, 0xFD) | (1, 0xFE) => { // VPADDB/W/D
                    return self.execute_vex_packed_int_arith(ctx, vex_l, vvvv, opcode);
                }

                // VMOVNTPS/VMOVNTPD (0x2B), VMOVNTDQ (0xE7)
                (0, 0x2B) | (1, 0x2B) | (1, 0xE7) => {
                    return self.execute_vex_movnt_store(ctx, vex_l, vvvv);
                }

                _ => {}
            }
        }

        // VEX.0F3A encoded instructions (m_mmmm=3)
        if m_mmmm == 0x3 && vex_pp == 1 {
            match opcode {
                // VINSERTF128 ymm1, ymm2, xmm3/m128, imm8 (VEX.66.0F3A.W0 18 /r ib)
                0x18 => {
                    return self.execute_vinsertf128(ctx, vex_l, vvvv);
                }
                // VEXTRACTF128 xmm1/m128, ymm2, imm8 (VEX.66.0F3A.W0 19 /r ib)
                0x19 => {
                    return self.execute_vextractf128(ctx, vex_l);
                }
                // VPERM2F128 ymm1, ymm2, ymm3/m256, imm8 (VEX.66.0F3A.W0 06 /r ib)
                0x06 => {
                    return self.execute_vperm2f128(ctx, vex_l, vvvv);
                }
                // VINSERTI128 ymm1, ymm2, xmm3/m128, imm8 (VEX.66.0F3A.W0 38 /r ib)
                0x38 => {
                    return self.execute_vinsertf128(ctx, vex_l, vvvv);
                }
                // VEXTRACTI128 xmm1/m128, ymm2, imm8 (VEX.66.0F3A.W0 39 /r ib)
                0x39 => {
                    return self.execute_vextractf128(ctx, vex_l);
                }
                // VPERM2I128 ymm1, ymm2, ymm3/m256, imm8 (VEX.66.0F3A.W0 46 /r ib)
                0x46 => {
                    return self.execute_vperm2f128(ctx, vex_l, vvvv);
                }
                // VPBLENDD ymm1/xmm1, ymm2/xmm2, ymm3/m, imm8
                0x02 => {
                    return self.execute_vex_blendd(ctx, vex_l, vvvv);
                }
                // VPBLENDW ymm1/xmm1, ymm2/xmm2, ymm3/m, imm8
                0x0E => {
                    return self.execute_vex_blendw(ctx, vex_l, vvvv);
                }
                // VPALIGNR ymm1/xmm1, ymm2/xmm2, ymm3/m, imm8
                0x0F => {
                    return self.execute_vex_palignr(ctx, vex_l, vvvv);
                }
                // VMPSADBW ymm1/xmm1, ymm2/xmm2, ymm3/m, imm8
                0x42 => {
                    return self.execute_vex_mpsadbw(ctx, vex_l, vvvv);
                }
                // VPBLENDVB ymm1/xmm1, ymm2/xmm2, ymm3/m, xmm4
                0x4C => {
                    return self.execute_vex_pblendvb(ctx, vex_l, vvvv);
                }
                // VPERMILPS/VPERMILPD (imm8)
                0x04 => {
                    return self.execute_vex_permilps_imm(ctx, vex_l, vvvv);
                }
                0x05 => {
                    return self.execute_vex_permilpd_imm(ctx, vex_l, vvvv);
                }
                // VBLENDPS/VBLENDPD (imm8)
                0x0C | 0x0D => {
                    return self.execute_vex_blend_imm(ctx, vex_l, vvvv, opcode);
                }
                // VBLENDVPS/VBLENDVPD (mask in imm8[7:4])
                0x4A | 0x4B => {
                    return self.execute_vex_blendv(ctx, vex_l, vvvv, opcode);
                }
                // VROUNDPS/VROUNDPD/VROUNDSS/VROUNDSD (imm8)
                0x08 | 0x09 => {
                    return self.execute_vex_roundp(ctx, vex_l, vvvv, opcode);
                }
                0x0A | 0x0B => {
                    return self.execute_vex_rounds(ctx, vvvv, opcode);
                }
                // VDPPS/VDPPD (imm8)
                0x40 | 0x41 => {
                    return self.execute_vex_dp(ctx, vex_l, vvvv, opcode);
                }
                // VPERMQ/VPERMPD (imm8)
                0x00 | 0x01 => {
                    if vex_w == 1 {
                        return self.execute_vex_permqd_imm(ctx, vex_l, vvvv);
                    }
                }
                _ => {}
            }
        }

        // VEX.0F38 SIMD instructions (m_mmmm=2, pp=1 for 66 prefix)
        if m_mmmm == 0x2 && vex_pp == 1 {
            match opcode {
                // VPSHUFB: shuffle bytes
                0x00 => {
                    return self.execute_vex_pshufb(ctx, vex_l, vvvv);
                }
                // VPHADDW: horizontal add words
                0x01 => {
                    return self.execute_vex_phadd(ctx, vex_l, vvvv, 16, false);
                }
                // VPHADDD: horizontal add dwords
                0x02 => {
                    return self.execute_vex_phadd(ctx, vex_l, vvvv, 32, false);
                }
                // VPHADDSW: horizontal add signed words with saturation
                0x03 => {
                    return self.execute_vex_phadd(ctx, vex_l, vvvv, 16, true);
                }
                // VPMADDUBSW: multiply and add unsigned/signed bytes to words
                0x04 => {
                    return self.execute_vex_pmaddubsw(ctx, vex_l, vvvv);
                }
                // VPHSUBW: horizontal subtract words
                0x05 => {
                    return self.execute_vex_phsub(ctx, vex_l, vvvv, 16, false);
                }
                // VPHSUBD: horizontal subtract dwords
                0x06 => {
                    return self.execute_vex_phsub(ctx, vex_l, vvvv, 32, false);
                }
                // VPHSUBSW: horizontal subtract signed words with saturation
                0x07 => {
                    return self.execute_vex_phsub(ctx, vex_l, vvvv, 16, true);
                }
                // VPSIGNB/W/D: negate/zero/copy based on sign
                0x08 | 0x09 | 0x0A => {
                    return self.execute_vex_psign(ctx, vex_l, vvvv, opcode);
                }
                // VPMULHRSW: multiply high with rounding and scale
                0x0B => {
                    return self.execute_vex_pmulhrsw(ctx, vex_l, vvvv);
                }
                // VPERMILPS: permute single-precision floating-point
                0x0C => {
                    return self.execute_vex_permilps_reg(ctx, vex_l, vvvv);
                }
                // VPERMILPD: permute double-precision floating-point
                0x0D => {
                    return self.execute_vex_permilpd_reg(ctx, vex_l, vvvv);
                }
                // VBROADCASTSS/VBROADCASTSD
                0x18 | 0x19 => {
                    return self.execute_vex_broadcast_fp(ctx, vex_l, vvvv, opcode);
                }
                // VTESTPS/VTESTPD
                0x0E | 0x0F => {
                    return self.execute_vex_vtest(ctx, vex_l, vvvv, opcode);
                }
                // VMOVNTDQA
                0x2A => {
                    return self.execute_vex_movntdqa(ctx, vex_l, vvvv);
                }
                // VMASKMOVPS/VMASKMOVPD
                0x2C | 0x2D | 0x2E | 0x2F => {
                    return self.execute_vex_maskmov_fp(ctx, vex_l, vvvv, opcode);
                }
                // VPBROADCASTB/W/D/Q: broadcast
                0x78 | 0x79 | 0x58 | 0x59 => {
                    return self.execute_vex_broadcast(ctx, vex_l, opcode);
                }
                // VBROADCASTI128: broadcast 128-bit integer to 256-bit
                0x5A => {
                    return self.execute_vex_broadcast_i128(ctx, vex_l);
                }
                // VPMOVSXBW/BD/BQ/WD/WQ/DQ: packed move with sign extension
                0x20 | 0x21 | 0x22 | 0x23 | 0x24 | 0x25 => {
                    return self.execute_vex_pmovsx(ctx, vex_l, opcode);
                }
                // VPMOVZXBW/BD/BQ/WD/WQ/DQ: packed move with zero extension
                0x30 | 0x31 | 0x32 | 0x33 | 0x34 | 0x35 => {
                    return self.execute_vex_pmovzx(ctx, vex_l, opcode);
                }
                // VPMULDQ: multiply packed signed dword integers
                0x28 => {
                    return self.execute_vex_pmuldq(ctx, vex_l, vvvv);
                }
                // VPCMPEQQ: compare packed quadwords for equal
                0x29 => {
                    return self.execute_vex_pcmpeqq(ctx, vex_l, vvvv);
                }
                // VPACKUSDW: pack dwords to unsigned words with saturation
                0x2B => {
                    return self.execute_vex_packusdw(ctx, vex_l, vvvv);
                }
                // VPTEST: logical compare and set flags
                0x17 => {
                    return self.execute_vex_ptest(ctx, vex_l, vvvv);
                }
                // VPHMINPOSUW: horizontal min/pos
                0x41 => {
                    return self.execute_vex_phminposuw(ctx, vex_l, vvvv);
                }
                // VPMINSB: minimum of signed bytes
                0x38 => {
                    return self.execute_vex_pminmax_sb(ctx, vex_l, vvvv, false);
                }
                // VPMINSD: minimum of signed dwords
                0x39 => {
                    return self.execute_vex_pminmax_sd(ctx, vex_l, vvvv, false);
                }
                // VPMINUW: minimum of unsigned words
                0x3A => {
                    return self.execute_vex_pminmax_uw(ctx, vex_l, vvvv, false);
                }
                // VPMINUD: minimum of unsigned dwords
                0x3B => {
                    return self.execute_vex_pminmax_ud(ctx, vex_l, vvvv, false);
                }
                // VPMAXSB: maximum of signed bytes
                0x3C => {
                    return self.execute_vex_pminmax_sb(ctx, vex_l, vvvv, true);
                }
                // VPMAXSD: maximum of signed dwords
                0x3D => {
                    return self.execute_vex_pminmax_sd(ctx, vex_l, vvvv, true);
                }
                // VPMAXUW: maximum of unsigned words
                0x3E => {
                    return self.execute_vex_pminmax_uw(ctx, vex_l, vvvv, true);
                }
                // VPMAXUD: maximum of unsigned dwords
                0x3F => {
                    return self.execute_vex_pminmax_ud(ctx, vex_l, vvvv, true);
                }
                // VPMULLD: multiply packed dword integers
                0x40 => {
                    return self.execute_vex_pmulld(ctx, vex_l, vvvv);
                }
                // VPCMPGTQ: compare packed qwords for greater than
                0x37 => {
                    return self.execute_vex_pcmpgtq(ctx, vex_l, vvvv);
                }
                // VPABSB/W/D: packed absolute value
                0x1C | 0x1D | 0x1E => {
                    return self.execute_vex_pabs(ctx, vex_l, opcode);
                }
                // VPERMPS/VPERMD
                0x16 => {
                    return self.execute_vex_permps(ctx, vex_l, vvvv);
                }
                0x36 => {
                    return self.execute_vex_permd(ctx, vex_l, vvvv);
                }
                // VPMASKMOVD/VPMASKMOVQ
                0x8C | 0x8E => {
                    return self.execute_vex_pmaskmov(ctx, vex_l, vex_w, vvvv, opcode);
                }
                // VGATHERDPD/VGATHERQPD/VGATHERDPS/VGATHERQPS
                0x90 | 0x91 | 0x92 | 0x93 => {
                    return self.execute_vex_gather(ctx, vex_l, vex_w, vvvv, opcode);
                }
                // FMA instructions
                0x96 | 0x97 | 0x98 | 0x99 | 0x9A | 0x9B | 0x9C | 0x9D | 0x9E | 0x9F | 0xA6
                | 0xA7 | 0xA8 | 0xA9 | 0xAA | 0xAB | 0xAC | 0xAD | 0xAE | 0xAF | 0xB6 | 0xB7
                | 0xB8 | 0xB9 | 0xBA | 0xBB | 0xBC | 0xBD | 0xBE | 0xBF => {
                    return self.execute_vex_fma(ctx, vex_l, vex_w, vvvv, opcode);
                }
                _ => {}
            }
        }

        self.inject_undefined_instruction()
    }
}

// Sub-modules
mod arith;
mod compare;
mod convert;
mod fma;
mod gather;
mod integer;
mod logical;
mod misc;
mod r#move;
mod shift;
mod shuffle;
