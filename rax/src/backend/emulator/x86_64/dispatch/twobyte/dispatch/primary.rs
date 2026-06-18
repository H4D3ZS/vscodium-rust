//! Two-byte opcode instruction implementation for x86_64 emulator.

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::super::cpu::{InsnContext, X86_64Vcpu};
use super::super::super::super::flags;
use super::super::super::super::insn;

impl X86_64Vcpu {
    #[inline(always)]
    pub(in crate::backend::emulator::x86_64) fn execute_0f(
        &mut self,
        ctx: &mut InsnContext,
    ) -> Result<Option<VcpuExit>> {
        let opcode2 = ctx.consume_u8()?;

        // Record precise opcode key for profiling
        #[cfg(feature = "profiling")]
        crate::profiling::set_current_opcode_key(crate::profiling::OpcodeKey::TwoByte(opcode2));

        match opcode2 {
            // System
            0x00 => insn::system::group6(self, ctx),
            0x01 => self.execute_0f01(ctx),
            0x02 => insn::system::lar(self, ctx),
            0x03 => insn::system::lsl(self, ctx),
            0x05 => insn::system::syscall(self, ctx),
            0x06 => insn::system::clts(self, ctx),
            0x07 => insn::system::sysret(self, ctx),
            // INVD/WBINVD - cache invalidation (NOP in emulator)
            0x08 => {
                // INVD - Invalidate internal caches
                self.regs.rip += ctx.cursor as u64;
                Ok(None)
            }
            0x09 => {
                // WBINVD - Write back and invalidate caches
                self.regs.rip += ctx.cursor as u64;
                Ok(None)
            }
            // UD2 - Undefined Instruction (intentional #UD exception)
            0x0B => {
                // Inject #UD so the guest's own handler runs. The kernel encodes
                // WARN()/BUG() as UD2 + a __bug_table entry: do_invalid_op /
                // report_bug then either prints a warning and RESUMES (WARN) or
                // panics (BUG). A genuinely-unreachable UD2 (reached only via a
                // mis-emulated branch) faults as "invalid opcode", which is far
                // more diagnosable than silently skipping it into garbage.
                // (An earlier "skip the first N kernel-text UD2s" workaround
                // masked real emulation bugs and corrupted control flow — removed.)
                // #UD is a fault: RIP stays on the faulting instruction.
                self.inject_exception(6, None)?; // #UD = vector 6
                Ok(None)
            }
            0x20 => insn::system::mov_r_cr(self, ctx),
            0x21 => insn::system::mov_r_dr(self, ctx),
            0x22 => insn::system::mov_cr_r(self, ctx),
            0x23 => insn::system::mov_dr_r(self, ctx),
            0x30 => insn::system::wrmsr(self, ctx),
            0x31 => insn::system::rdtsc(self, ctx),
            0x32 => insn::system::rdmsr(self, ctx),
            0x33 => insn::system::rdpmc(self, ctx),
            0x34 => insn::system::sysenter(self, ctx),
            0x35 => insn::system::sysexit(self, ctx),
            0xA0 => insn::data::push_sreg(self, ctx, 4), // PUSH FS
            0xA1 => insn::data::pop_sreg(self, ctx, 4),  // POP FS
            0xA8 => insn::data::push_sreg(self, ctx, 5), // PUSH GS
            0xA9 => insn::data::pop_sreg(self, ctx, 5),  // POP GS
            0xA2 => insn::system::cpuid(self, ctx),
            0xAE => self.execute_0fae(ctx),

            // Control flow
            0x40..=0x4F => insn::control::cmovcc(self, ctx, opcode2 & 0x0F),
            0x80..=0x8F => insn::control::jcc_rel32(self, ctx, opcode2 & 0x0F),
            0x90..=0x9F => insn::control::setcc(self, ctx, opcode2 & 0x0F),

            // Data movement
            0xB6 => insn::data::movzx_r_rm8(self, ctx),
            0xB7 => insn::data::movzx_r_rm16(self, ctx),
            0xBE => insn::data::movsx_r_rm8(self, ctx),
            0xBF => insn::data::movsx_r_rm16(self, ctx),
            0xC8..=0xCF => insn::data::bswap(self, ctx, opcode2),

            // Arithmetic
            0xAF => insn::arith::imul_r_rm(self, ctx),

            // Bit manipulation
            0xA3 => insn::bit::bt_rm_r(self, ctx),
            0xAB => insn::bit::bts_rm_r(self, ctx),
            0xB3 => insn::bit::btr_rm_r(self, ctx),
            0xBB => insn::bit::btc_rm_r(self, ctx),
            0xBA => insn::bit::group8(self, ctx),
            0xB8 => insn::bit::popcnt(self, ctx),
            // BSF/TZCNT and BSR/LZCNT share opcodes - F3 prefix differentiates
            0xBC => {
                if ctx.rep_prefix == Some(0xF3) {
                    insn::bit::tzcnt(self, ctx)
                } else {
                    insn::bit::bsf(self, ctx)
                }
            }
            0xBD => {
                if ctx.rep_prefix == Some(0xF3) {
                    insn::bit::lzcnt(self, ctx)
                } else {
                    insn::bit::bsr(self, ctx)
                }
            }

            // CMPXCHG
            0xB0 => insn::data::cmpxchg_rm8_r8(self, ctx),
            0xB1 => insn::data::cmpxchg_rm_r(self, ctx),
            // UD1 - Undefined Instruction (intentional #UD exception with ModRM)
            0xB9 => {
                // UD1 has a ModR/M byte but always generates #UD
                // Don't advance RIP - #UD is a fault, exception points to faulting instruction
                let _modrm = ctx.consume_u8()?;
                self.inject_exception(6, None)?; // #UD = vector 6
                Ok(None)
            }

            // XADD
            0xC0 => insn::data::xadd_rm8_r8(self, ctx),
            0xC1 => insn::data::xadd_rm_r(self, ctx),

            // SHLD/SHRD
            0xA4 => insn::shift::shld_imm8(self, ctx),
            0xA5 => insn::shift::shld_cl(self, ctx),
            0xAC => insn::shift::shrd_imm8(self, ctx),
            0xAD => insn::shift::shrd_cl(self, ctx),

            // NOP variants
            0x1C => insn::system::cldemote(self, ctx),
            0x1E => insn::system::endbr(self, ctx),
            0x1F => insn::system::nop_rm(self, ctx),

            // Prefetch hints
            0x0D => insn::simd::prefetchw(self, ctx),
            0x18 => insn::simd::prefetchh(self, ctx),

            // MOVUPS/MOVUPD (0x10/0x11 unaligned), MOVAPS/MOVAPD (0x28/0x29 aligned)
            0x10 => insn::simd::movups_load(self, ctx),
            0x11 => insn::simd::movups_store(self, ctx),
            0x12 => {
                if ctx.rep_prefix == Some(0xF2) {
                    // F2 0F 12: MOVDDUP xmm1, xmm2/m64
                    insn::simd::movddup(self, ctx)
                } else if ctx.rep_prefix == Some(0xF3) {
                    // F3 0F 12: MOVSLDUP xmm1, xmm2/m128
                    insn::simd::movsldup(self, ctx)
                } else {
                    // NP/66 0F 12: MOVLPS/MOVHLPS xmm, m64/xmm
                    insn::simd::movlps_load(self, ctx)
                }
            }
            0x13 => insn::simd::movlps_store(self, ctx),
            0x16 => {
                if ctx.rep_prefix == Some(0xF3) {
                    // F3 0F 16: MOVSHDUP xmm1, xmm2/m128
                    insn::simd::movshdup(self, ctx)
                } else {
                    // NP/66 0F 16: MOVHPS/MOVLHPS xmm, m64/xmm
                    insn::simd::movhps_load(self, ctx)
                }
            }
            0x17 => insn::simd::movhps_store(self, ctx),
            0x28 => insn::simd::movaps_load(self, ctx),
            0x29 => insn::simd::movaps_store(self, ctx),

            // SSE logical operations
            0x54 => insn::simd::andps(self, ctx),
            0x55 => insn::simd::andnps(self, ctx),
            0x56 => insn::simd::orps(self, ctx),
            0x57 => insn::simd::xorps(self, ctx),

            // MOVMSKPS/MOVMSKPD - extract sign bits
            0x50 => self.execute_movmsk(ctx),

            // SSE arithmetic
            0x51 => self.execute_sse_sqrt(ctx),
            0x52 => self.execute_sse_rsqrt(ctx),
            0x53 => self.execute_sse_rcp(ctx),
            0x58 => self.execute_sse_add(ctx),
            0x59 => self.execute_sse_mul(ctx),
            0x5C => self.execute_sse_sub(ctx),
            0x5D => self.execute_sse_min(ctx),
            0x5E => self.execute_sse_div(ctx),
            0x5F => self.execute_sse_max(ctx),

            // SSE unpack
            0x14 => self.execute_sse_unpcklps(ctx),
            0x15 => self.execute_sse_unpckhps(ctx),
            // SSE2/MMX integer unpack
            0x60 | 0x61 | 0x62 | 0x68 | 0x69 | 0x6A | 0x6C | 0x6D => {
                self.execute_punpck(ctx, opcode2)
            }

            // MOVD/MOVQ
            0x6E => {
                if ctx.operand_size_override {
                    // 66 0F 6E: MOVD/MOVQ xmm, r/m32 (or r/m64 with REX.W)
                    insn::simd::movd_xmm_rm(self, ctx)
                } else {
                    // NP 0F 6E: MOVD/MOVQ mm, r/m32 (or r/m64 with REX.W)
                    insn::simd::movd_mm_rm(self, ctx)
                }
            }
            0x7E => {
                if ctx.rep_prefix == Some(0xF3) {
                    // F3 0F 7E: MOVQ xmm1, xmm2/m64
                    insn::simd::movq_xmm_xmm_m64(self, ctx)
                } else if ctx.operand_size_override {
                    // 66 0F 7E: MOVD/MOVQ r/m32, xmm (or r/m64 with REX.W)
                    insn::simd::movd_rm_xmm(self, ctx)
                } else {
                    // NP 0F 7E: MOVD/MOVQ r/m32, mm (or r/m64 with REX.W)
                    insn::simd::movd_rm_mm(self, ctx)
                }
            }
            0xD6 => {
                if ctx.rep_prefix == Some(0xF3) {
                    // F3 0F D6: MOVQ2DQ xmm, mm - move mm to low qword of xmm
                    insn::simd::movq2dq(self, ctx)
                } else if ctx.rep_prefix == Some(0xF2) {
                    // F2 0F D6: MOVDQ2Q mm, xmm - move low qword of xmm to mm
                    insn::simd::movdq2q(self, ctx)
                } else if ctx.operand_size_override {
                    // 66 0F D6: MOVQ xmm2/m64, xmm1
                    insn::simd::movq_xmm_m64_xmm(self, ctx)
                } else {
                    Err(Error::Emulator(format!(
                        "unimplemented 0x0F 0xD6 opcode variant at RIP={:#x}",
                        self.regs.rip
                    )))
                }
            }
            // Packed integer insert/extract
            0xD7 => insn::simd::pmovmskb(self, ctx),
            // Packed integer add (SSE2/MMX)
            0xD4 => insn::simd::paddq_packed(self, ctx),
            0xFC => insn::simd::paddb_packed(self, ctx),
            0xFD => insn::simd::paddw_packed(self, ctx),
            0xFE => insn::simd::paddd_packed(self, ctx),
            // Packed integer saturating add (SSE2/MMX)
            0xEC => insn::simd::paddsb_packed(self, ctx),
            0xED => insn::simd::paddsw_packed(self, ctx),
            0xDC => insn::simd::paddusb_packed(self, ctx),
            0xDD => insn::simd::paddusw_packed(self, ctx),
            // Packed integer subtract (SSE2)
            0xD8 | 0xD9 | 0xE8 | 0xE9 | 0xF8 | 0xF9 | 0xFA | 0xFB => {
                insn::simd::psub_packed(self, ctx, opcode2)
            }
            // Packed integer logical (SSE2/MMX)
            0xDB => insn::simd::pand(self, ctx),
            0xDF => insn::simd::pandn(self, ctx),
            0xEB => insn::simd::por(self, ctx),
            // Packed integer compare (SSE2/MMX)
            0x74 => insn::simd::pcmpeqb(self, ctx),
            0x75 => insn::simd::pcmpeqw(self, ctx),
            0x76 => insn::simd::pcmpeqd(self, ctx),
            0x64 => insn::simd::pcmpgtb(self, ctx),
            0x65 => insn::simd::pcmpgtw(self, ctx),
            0x66 => insn::simd::pcmpgtd(self, ctx),
            // MOVNTQ - non-temporal store MMX (0F E7)
            // MOVNTDQ - non-temporal store XMM (66 0F E7)
            0xE7 => {
                if ctx.operand_size_override {
                    self.execute_movnt_store(ctx)
                } else {
                    insn::simd::movntq(self, ctx)
                }
            }
            // Packed integer min/max (SSE2)
            0xDA => insn::simd::pminub(self, ctx),
            0xDE => insn::simd::pmaxub(self, ctx),
            0xEA => insn::simd::pminsw(self, ctx),
            0xEE => insn::simd::pmaxsw(self, ctx),

            // Packed integer multiply (SSE2/MMX)
            0xD5 => insn::simd::pmullw(self, ctx),  // PMULLW
            0xE4 => insn::simd::pmulhuw(self, ctx), // PMULHUW
            0xE5 => insn::simd::pmulhw(self, ctx),  // PMULHW
            0xF4 => insn::simd::pmuludq(self, ctx), // PMULUDQ
            0xF5 => insn::simd::pmaddwd(self, ctx), // PMADDWD

            // PXOR (SSE2) - XOR packed integers
            0xEF => insn::simd::pxor(self, ctx),

            // SSE/SSE2 Conversion Instructions
            0x5A => {
                if ctx.rep_prefix == Some(0xF3) {
                    // F3 0F 5A: CVTSS2SD xmm1, xmm2/m32
                    insn::simd::cvtss2sd(self, ctx)
                } else if ctx.rep_prefix == Some(0xF2) {
                    // F2 0F 5A: CVTSD2SS xmm1, xmm2/m64
                    insn::simd::cvtsd2ss(self, ctx)
                } else if ctx.operand_size_override {
                    // 66 0F 5A: CVTPD2PS xmm1, xmm2/m128
                    insn::simd::cvtpd2ps(self, ctx)
                } else {
                    // NP 0F 5A: CVTPS2PD xmm1, xmm2/m64
                    insn::simd::cvtps2pd(self, ctx)
                }
            }
            0x5B => {
                if ctx.rep_prefix == Some(0xF3) {
                    // F3 0F 5B: CVTTPS2DQ xmm1, xmm2/m128
                    insn::simd::cvttps2dq(self, ctx)
                } else if ctx.operand_size_override {
                    // 66 0F 5B: CVTPS2DQ xmm1, xmm2/m128
                    insn::simd::cvtps2dq(self, ctx)
                } else {
                    // NP 0F 5B: CVTDQ2PS xmm1, xmm2/m128
                    insn::simd::cvtdq2ps(self, ctx)
                }
            }
            0x2A => {
                if ctx.rep_prefix == Some(0xF3) {
                    // F3 0F 2A: CVTSI2SS xmm1, r/m32 or r/m64
                    insn::simd::cvtsi2ss(self, ctx)
                } else if ctx.rep_prefix == Some(0xF2) {
                    // F2 0F 2A: CVTSI2SD xmm1, r/m32 or r/m64
                    insn::simd::cvtsi2sd(self, ctx)
                } else if ctx.operand_size_override {
                    // 66 0F 2A: CVTPI2PD xmm, mm/m64
                    insn::simd::cvtpi2pd(self, ctx)
                } else {
                    // NP 0F 2A: CVTPI2PS xmm, mm/m64
                    insn::simd::cvtpi2ps(self, ctx)
                }
            }
            0x2C => {
                if ctx.rep_prefix == Some(0xF3) {
                    // F3 0F 2C: CVTTSS2SI r32/r64, xmm1/m32
                    insn::simd::cvttss2si(self, ctx)
                } else if ctx.rep_prefix == Some(0xF2) {
                    // F2 0F 2C: CVTTSD2SI r32/r64, xmm1/m64
                    insn::simd::cvttsd2si(self, ctx)
                } else if ctx.operand_size_override {
                    // 66 0F 2C: CVTTPD2PI mm, xmm/m128
                    insn::simd::cvttpd2pi(self, ctx)
                } else {
                    // NP 0F 2C: CVTTPS2PI mm, xmm/m64
                    insn::simd::cvttps2pi(self, ctx)
                }
            }
            0x2D => {
                if ctx.rep_prefix == Some(0xF3) {
                    // F3 0F 2D: CVTSS2SI r32/r64, xmm1/m32
                    insn::simd::cvtss2si(self, ctx)
                } else if ctx.rep_prefix == Some(0xF2) {
                    // F2 0F 2D: CVTSD2SI r32/r64, xmm1/m64
                    insn::simd::cvtsd2si(self, ctx)
                } else if ctx.operand_size_override {
                    // 66 0F 2D: CVTPD2PI mm, xmm/m128
                    insn::simd::cvtpd2pi(self, ctx)
                } else {
                    // NP 0F 2D: CVTPS2PI mm, xmm/m64
                    insn::simd::cvtps2pi(self, ctx)
                }
            }
            // MOVNTPS/MOVNTPD - non-temporal hint store
            0x2B => self.execute_movnt_store(ctx),
            0x2E => insn::simd::ucomiss_ucomisd(self, ctx),
            // COMISS/COMISD - compare scalar and set EFLAGS
            0x2F => self.execute_comiss(ctx),
            0xE6 => {
                if ctx.rep_prefix == Some(0xF3) {
                    // F3 0F E6: CVTDQ2PD xmm1, xmm2/m64
                    insn::simd::cvtdq2pd(self, ctx)
                } else if ctx.rep_prefix == Some(0xF2) {
                    // F2 0F E6: CVTPD2DQ xmm1, xmm2/m128
                    insn::simd::cvtpd2dq(self, ctx)
                } else if ctx.operand_size_override {
                    // 66 0F E6: CVTTPD2DQ xmm1, xmm2/m128
                    insn::simd::cvttpd2dq(self, ctx)
                } else {
                    Err(Error::Emulator(format!(
                        "unimplemented 0x0F 0xE6 opcode variant at RIP={:#x}",
                        self.regs.rip
                    )))
                }
            }

            // 0F 38 escape - MOVBE and other instructions
            0x38 => self.execute_0f38(ctx),

            // 0F 3A escape - PEXTR*, PINSR*, ROUND*, etc.
            0x3A => self.execute_0f3a(ctx),

            // MOVDQA/MOVDQU/MOVQ load (0x6F)
            0x6F => {
                if ctx.rep_prefix == Some(0xF3) {
                    // F3 0F 6F: MOVDQU xmm, xmm/m128 (unaligned)
                    insn::simd::movdqu_xmm_xmm_m128(self, ctx)
                } else if ctx.operand_size_override {
                    // 66 0F 6F: MOVDQA xmm, xmm/m128 (aligned)
                    insn::simd::movdqa_xmm_xmm_m128(self, ctx)
                } else {
                    // NP 0F 6F: MOVQ mm, mm/m64 (MMX)
                    insn::simd::movq_mm_mm_m64(self, ctx)
                }
            }

            // PSHUFD/PSHUFHW/PSHUFLW (0x70)
            0x70 => self.execute_pshufd(ctx),

            // MOVDQA/MOVDQU/MOVQ store (0x7F)
            0x7F => {
                if ctx.rep_prefix == Some(0xF3) {
                    // F3 0F 7F: MOVDQU xmm/m128, xmm (unaligned)
                    insn::simd::movdqu_xmm_m128_xmm(self, ctx)
                } else if ctx.operand_size_override {
                    // 66 0F 7F: MOVDQA xmm/m128, xmm (aligned)
                    insn::simd::movdqa_xmm_m128_xmm(self, ctx)
                } else {
                    // NP 0F 7F: MOVQ mm/m64, mm (MMX)
                    insn::simd::movq_mm_m64_mm(self, ctx)
                }
            }

            // SSE2/MMX shift immediate (groups 12, 13, 14)
            0x71 => self.execute_shift_imm_group12(ctx),
            0x72 => self.execute_shift_imm_group13(ctx),
            0x73 => self.execute_shift_imm_group14(ctx),

            // SSE3 horizontal add/sub
            0x7C => self.execute_hadd(ctx),
            0x7D => self.execute_hsub(ctx),

            // SSE3 ADDSUBPS/ADDSUBPD
            0xD0 => self.execute_addsubps(ctx),

            // CMPPS/CMPPD/CMPSS/CMPSD (0xC2)
            0xC2 => self.execute_cmpps(ctx),

            // SHUFPS/SHUFPD (0xC6)
            0xC6 => self.execute_shufps(ctx),

            // PINSRW (0xC4)
            0xC4 => self.execute_pinsrw(ctx),

            // PEXTRW (0xC5)
            0xC5 => self.execute_pextrw(ctx),

            // MOVNTI - non-temporal store (0xC3)
            0xC3 => insn::simd::movnti(self, ctx),

            // PSADBW - sum of absolute differences (0xF6)
            0xF6 => insn::simd::psadbw(self, ctx),

            // MASKMOVDQU/MASKMOVQ (0xF7)
            0xF7 => insn::simd::maskmovdqu(self, ctx),

            // LDDQU (F2 0F F0)
            0xF0 if ctx.rep_prefix == Some(0xF2) => insn::simd::lddqu(self, ctx),

            // PAVGB/PAVGW - packed average (0xE0/0xE3)
            0xE0 => insn::simd::pavgb(self, ctx),
            0xE3 => insn::simd::pavgw(self, ctx),

            // PACKSSWB/PACKSSDW - pack with saturation (0x63/0x6B)
            0x63 => insn::simd::packsswb(self, ctx),
            0x6B => insn::simd::packssdw(self, ctx),

            // PACKUSWB - pack unsigned with saturation (0x67)
            0x67 => insn::simd::packuswb(self, ctx),

            // Packed integer shift by XMM count
            // PSRLW/PSRLD/PSRLQ xmm, xmm/m128 (66 0F D1/D2/D3)
            0xD1 => insn::simd::packed_shift_xmm_count(self, ctx, opcode2),
            0xD2 => insn::simd::packed_shift_xmm_count(self, ctx, opcode2),
            0xD3 => insn::simd::packed_shift_xmm_count(self, ctx, opcode2),
            // PSRAW/PSRAD xmm, xmm/m128 (66 0F E1/E2)
            0xE1 => insn::simd::packed_shift_xmm_count(self, ctx, opcode2),
            0xE2 => insn::simd::packed_shift_xmm_count(self, ctx, opcode2),
            // PSLLW/PSLLD/PSLLQ xmm, xmm/m128 (66 0F F1/F2/F3)
            0xF1 => insn::simd::packed_shift_xmm_count(self, ctx, opcode2),
            0xF2 if ctx.rep_prefix.is_none() => {
                insn::simd::packed_shift_xmm_count(self, ctx, opcode2)
            }
            0xF3 if ctx.rep_prefix.is_none() => {
                insn::simd::packed_shift_xmm_count(self, ctx, opcode2)
            }

            // EMMS - Empty MMX State (0F 77)
            0x77 => insn::simd::emms(self, ctx),

            // 0F C7 - Group 9: CMPXCHG8B/16B, RDRAND, RDSEED, etc.
            0xC7 => self.execute_group9(ctx),

            _ => self.inject_undefined_instruction(),
        }
    }
}
