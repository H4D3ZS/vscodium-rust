//! EVEX-encoded (AVX-512) instruction dispatch.
//!
//! EVEX prefix format (after 0x62):
//! - P0: R X B R' 0 m m m
//! - P1: W v v v v 1 p p
//! - P2: z L' L b V' a a a
//!
//! mm field (opcode map):
//! - 1: 0F (two-byte opcode)
//! - 2: 0F 38 (three-byte opcode)
//! - 3: 0F 3A (three-byte opcode with immediate)
//! - 5: MAP5 (AVX-512 FP16)
//! - 6: MAP6 (AVX-512 FP16)

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::cpu::{InsnContext, X86_64Vcpu};
use super::super::{flags, insn};

impl X86_64Vcpu {
    /// Execute EVEX-encoded instruction.
    /// mm: opcode map (1=0F, 2=0F38, 3=0F3A)
    pub(in crate::backend::emulator::x86_64) fn execute_evex(
        &mut self,
        ctx: &mut InsnContext,
        mm: u8,
    ) -> Result<Option<VcpuExit>> {
        let opcode = ctx.consume_u8()?;

        // Record precise opcode key for profiling
        #[cfg(feature = "profiling")]
        crate::profiling::set_current_opcode_key(crate::profiling::OpcodeKey::Evex {
            map: mm,
            opcode,
        });

        match mm {
            1 => self.execute_evex_0f(ctx, opcode),
            2 => self.execute_evex_0f38(ctx, opcode),
            3 => self.execute_evex_0f3a(ctx, opcode),
            4 => self.execute_evex_map4_apx(ctx, opcode), // APX GPR instructions
            5 => self.execute_evex_map5(ctx, opcode),
            _ => Err(Error::Emulator(format!(
                "Invalid EVEX mm field {} at RIP={:#x}",
                mm, self.regs.rip
            ))),
        }
    }

    /// EVEX 0F opcode map (mm=1)
    fn execute_evex_0f(&mut self, ctx: &mut InsnContext, opcode: u8) -> Result<Option<VcpuExit>> {
        let evex = ctx
            .evex
            .ok_or_else(|| Error::Emulator("EVEX context missing".to_string()))?;

        match opcode {
            // VMOVUPS/VMOVAPS load (0x10/0x28)
            0x10 | 0x28 if evex.pp == 0 => self.execute_evex_mov_load(ctx, opcode == 0x28),
            // VMOVUPD/VMOVAPD load (0x10/0x28 with 66 prefix)
            0x10 | 0x28 if evex.pp == 1 => self.execute_evex_mov_load(ctx, opcode == 0x28),
            // VMOVUPS/VMOVAPS store (0x11/0x29)
            0x11 | 0x29 if evex.pp == 0 => self.execute_evex_mov_store(ctx, opcode == 0x29),
            // VMOVUPD/VMOVAPD store (0x11/0x29 with 66 prefix)
            0x11 | 0x29 if evex.pp == 1 => self.execute_evex_mov_store(ctx, opcode == 0x29),
            // VADDPS (pp=0/W=0) / VADDPD (pp=1/W=1) (0x58)
            0x58 if evex.pp == 1 || evex.w => self.execute_evex_fp_arith_pd(ctx, |a, b| a + b),
            0x58 => self.execute_evex_fp_arith_ps(ctx, |a, b| a + b),
            // VMULPS / VMULPD (0x59)
            0x59 if evex.pp == 1 || evex.w => self.execute_evex_fp_arith_pd(ctx, |a, b| a * b),
            0x59 => self.execute_evex_fp_arith_ps(ctx, |a, b| a * b),
            // VSUBPS / VSUBPD (0x5C)
            0x5C if evex.pp == 1 || evex.w => self.execute_evex_fp_arith_pd(ctx, |a, b| a - b),
            0x5C => self.execute_evex_fp_arith_ps(ctx, |a, b| a - b),
            // VDIVPS / VDIVPD (0x5E)
            0x5E if evex.pp == 1 || evex.w => self.execute_evex_fp_arith_pd(ctx, |a, b| a / b),
            0x5E => self.execute_evex_fp_arith_ps(ctx, |a, b| a / b),
            // VXORPS/VXORPD (0x57)
            0x57 => self.execute_evex_bitwise_xor(ctx),

            // ================================================================
            // Broadened EVEX coverage: integer/logical/compare/move/broadcast/shift
            // All of the following require the 66 implied prefix (pp=1) unless noted.
            // ================================================================

            // VMOVDQA32/64 load (0x6F pp=1): W0=DQA32 (dword), W1=DQA64 (qword)
            0x6F if evex.pp == 1 => {
                let es = if evex.w { 8 } else { 4 };
                insn::simd::evex_mov_masked_load(self, ctx, es, true)
            }
            // VMOVDQU32/64 load (0x6F pp=2/F3): W0=DQU32, W1=DQU64
            0x6F if evex.pp == 2 => {
                let es = if evex.w { 8 } else { 4 };
                insn::simd::evex_mov_masked_load(self, ctx, es, false)
            }
            // VMOVDQU8/16 load (0x6F pp=3/F2): W0=DQU8 (byte), W1=DQU16 (word)
            0x6F if evex.pp == 3 => {
                let es = if evex.w { 2 } else { 1 };
                insn::simd::evex_mov_masked_load(self, ctx, es, false)
            }
            // VMOVDQA32/64 store (0x7F pp=1)
            0x7F if evex.pp == 1 => {
                let es = if evex.w { 8 } else { 4 };
                insn::simd::evex_mov_masked_store(self, ctx, es, true)
            }
            // VMOVDQU32/64 store (0x7F pp=2/F3)
            0x7F if evex.pp == 2 => {
                let es = if evex.w { 8 } else { 4 };
                insn::simd::evex_mov_masked_store(self, ctx, es, false)
            }
            // VMOVDQU8/16 store (0x7F pp=3/F2)
            0x7F if evex.pp == 3 => {
                let es = if evex.w { 2 } else { 1 };
                insn::simd::evex_mov_masked_store(self, ctx, es, false)
            }

            // Logical: VPANDD/Q (0xDB), VPANDND/Q (0xDF), VPORD/Q (0xEB), VPXORD/Q (0xEF)
            0xDB if evex.pp == 1 => insn::simd::evex_int_arith(self, ctx, insn::simd::IntOp::And),
            0xDF if evex.pp == 1 => insn::simd::evex_int_arith(self, ctx, insn::simd::IntOp::Andn),
            0xEB if evex.pp == 1 => insn::simd::evex_int_arith(self, ctx, insn::simd::IntOp::Or),
            0xEF if evex.pp == 1 => insn::simd::evex_int_arith(self, ctx, insn::simd::IntOp::Xor),

            // Integer add: VPADDB/W/D/Q (0xFC/0xFD/0xFE/0xD4)
            0xFC if evex.pp == 1 => insn::simd::evex_int_arith(self, ctx, insn::simd::IntOp::AddB),
            0xFD if evex.pp == 1 => insn::simd::evex_int_arith(self, ctx, insn::simd::IntOp::AddW),
            0xFE if evex.pp == 1 => insn::simd::evex_int_arith(self, ctx, insn::simd::IntOp::AddD),
            0xD4 if evex.pp == 1 => insn::simd::evex_int_arith(self, ctx, insn::simd::IntOp::AddQ),
            // Integer sub: VPSUBB/W/D/Q (0xF8/0xF9/0xFA/0xFB)
            0xF8 if evex.pp == 1 => insn::simd::evex_int_arith(self, ctx, insn::simd::IntOp::SubB),
            0xF9 if evex.pp == 1 => insn::simd::evex_int_arith(self, ctx, insn::simd::IntOp::SubW),
            0xFA if evex.pp == 1 => insn::simd::evex_int_arith(self, ctx, insn::simd::IntOp::SubD),
            0xFB if evex.pp == 1 => insn::simd::evex_int_arith(self, ctx, insn::simd::IntOp::SubQ),
            // VPMULLW (0xD5)
            0xD5 if evex.pp == 1 => insn::simd::evex_int_arith(self, ctx, insn::simd::IntOp::MullW),

            // Compare into mask (fixed predicate forms), pp=1 (66):
            // VPCMPEQB/W/D (0x74/0x75/0x76), VPCMPGTB/W/D (0x64/0x65/0x66)
            0x74 if evex.pp == 1 => {
                insn::simd::evex_int_cmp(self, ctx, 1, true, insn::simd::CmpPred::Eq, false)
            }
            0x75 if evex.pp == 1 => {
                insn::simd::evex_int_cmp(self, ctx, 2, true, insn::simd::CmpPred::Eq, false)
            }
            0x76 if evex.pp == 1 => {
                insn::simd::evex_int_cmp(self, ctx, 4, true, insn::simd::CmpPred::Eq, false)
            }
            0x64 if evex.pp == 1 => {
                insn::simd::evex_int_cmp(self, ctx, 1, true, insn::simd::CmpPred::Gt, false)
            }
            0x65 if evex.pp == 1 => {
                insn::simd::evex_int_cmp(self, ctx, 2, true, insn::simd::CmpPred::Gt, false)
            }
            0x66 if evex.pp == 1 => {
                insn::simd::evex_int_cmp(self, ctx, 4, true, insn::simd::CmpPred::Gt, false)
            }

            // Packed shift by immediate (group opcodes 0x72/0x73 with /reg selecting op)
            // 0x72: VPSRLD(/2), VPSRAD(/4), VPSLLD(/6)  (dword, or qword for SRA via W1)
            // 0x73: VPSRLQ(/2), VPSLLQ(/6)              (qword)
            0x72 if evex.pp == 1 => {
                // Need the /reg field to pick the operation.
                let modrm = ctx.peek_u8()?;
                let sub = (modrm >> 3) & 0x7;
                let es = 4;
                match sub {
                    2 => insn::simd::evex_shift_imm(self, ctx, insn::simd::ShiftKind::Srl, es),
                    4 => {
                        // VPSRAD (W0=dword) / VPSRAQ (W1=qword)
                        let es = if evex.w { 8 } else { 4 };
                        insn::simd::evex_shift_imm(self, ctx, insn::simd::ShiftKind::Sra, es)
                    }
                    6 => insn::simd::evex_shift_imm(self, ctx, insn::simd::ShiftKind::Sll, es),
                    _ => Err(Error::Emulator(format!(
                        "Unimplemented EVEX 0F 72 /{} at RIP={:#x}",
                        sub, self.regs.rip
                    ))),
                }
            }
            0x73 if evex.pp == 1 => {
                let modrm = ctx.peek_u8()?;
                let sub = (modrm >> 3) & 0x7;
                let es = 8;
                match sub {
                    2 => insn::simd::evex_shift_imm(self, ctx, insn::simd::ShiftKind::Srl, es),
                    6 => insn::simd::evex_shift_imm(self, ctx, insn::simd::ShiftKind::Sll, es),
                    _ => Err(Error::Emulator(format!(
                        "Unimplemented EVEX 0F 73 /{} at RIP={:#x}",
                        sub, self.regs.rip
                    ))),
                }
            }
            // Packed shift by xmm count: VPSRLD/Q (0xD2/0xD3), VPSRAD/Q (0xE2),
            // VPSLLD/Q (0xF2/0xF3).
            0xD2 if evex.pp == 1 => {
                insn::simd::evex_shift_var(self, ctx, insn::simd::ShiftKind::Srl, 4)
            }
            0xD3 if evex.pp == 1 => {
                insn::simd::evex_shift_var(self, ctx, insn::simd::ShiftKind::Srl, 8)
            }
            0xE2 if evex.pp == 1 => {
                let es = if evex.w { 8 } else { 4 };
                insn::simd::evex_shift_var(self, ctx, insn::simd::ShiftKind::Sra, es)
            }
            0xF2 if evex.pp == 1 => {
                insn::simd::evex_shift_var(self, ctx, insn::simd::ShiftKind::Sll, 4)
            }
            0xF3 if evex.pp == 1 => {
                insn::simd::evex_shift_var(self, ctx, insn::simd::ShiftKind::Sll, 8)
            }

            _ => Err(Error::Emulator(format!(
                "Unimplemented EVEX.0F opcode {:#04x} at RIP={:#x}",
                opcode, self.regs.rip
            ))),
        }
    }

    /// EVEX move load (VMOVAPS/VMOVUPS, VMOVAPD/VMOVUPD)
    fn execute_evex_mov_load(
        &mut self,
        ctx: &mut InsnContext,
        aligned: bool,
    ) -> Result<Option<VcpuExit>> {
        let evex = ctx.evex.unwrap();
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;

        // Calculate full destination register (5 bits for ZMM16-31)
        let zmm_dst = if !evex.r { reg + 8 } else { reg };
        let zmm_dst = if !evex.r_prime { zmm_dst + 16 } else { zmm_dst } as usize;

        // Vector length from L'L
        let vl = match evex.ll {
            0 => 16, // 128-bit (XMM)
            1 => 32, // 256-bit (YMM)
            2 => 64, // 512-bit (ZMM)
            _ => 64,
        };

        if is_memory {
            // Check alignment for VMOVAPS/VMOVAPD
            if aligned && (addr % vl as u64) != 0 {
                return Err(Error::Emulator(format!(
                    "VMOVAPS: unaligned memory access at {:#x}",
                    addr
                )));
            }
            // Load from memory to ZMM register
            self.load_zmm_from_mem(zmm_dst, addr, vl)?;
        } else {
            // Register to register move
            let zmm_src = if !evex.b { rm + 8 } else { rm };
            let zmm_src = zmm_src as usize; // ZMM16-31 not encoded in rm for reg-reg
            self.copy_zmm(zmm_dst, zmm_src, vl);
        }

        // Zero upper bits if not 512-bit
        if vl < 64 && zmm_dst < 16 {
            if vl <= 16 {
                self.regs.ymm_high[zmm_dst][0] = 0;
                self.regs.ymm_high[zmm_dst][1] = 0;
            }
            self.regs.zmm_high[zmm_dst] = [0; 4];
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// EVEX move store (VMOVAPS/VMOVUPS, VMOVAPD/VMOVUPD)
    fn execute_evex_mov_store(
        &mut self,
        ctx: &mut InsnContext,
        aligned: bool,
    ) -> Result<Option<VcpuExit>> {
        let evex = ctx.evex.unwrap();
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;

        // Source register
        let zmm_src = if !evex.r { reg + 8 } else { reg };
        let zmm_src = if !evex.r_prime { zmm_src + 16 } else { zmm_src } as usize;

        // Vector length from L'L
        let vl = match evex.ll {
            0 => 16, // 128-bit (XMM)
            1 => 32, // 256-bit (YMM)
            2 => 64, // 512-bit (ZMM)
            _ => 64,
        };

        if is_memory {
            // Check alignment for VMOVAPS/VMOVAPD
            if aligned && (addr % vl as u64) != 0 {
                return Err(Error::Emulator(format!(
                    "VMOVAPS: unaligned memory access at {:#x}",
                    addr
                )));
            }
            // Store ZMM register to memory
            self.store_zmm_to_mem(zmm_src, addr, vl)?;
        } else {
            // Register to register move (destination is rm)
            let zmm_dst = if !evex.b { rm + 8 } else { rm } as usize;
            self.copy_zmm(zmm_dst, zmm_src, vl);

            // Zero upper bits if not 512-bit
            if vl < 64 && zmm_dst < 16 {
                if vl <= 16 {
                    self.regs.ymm_high[zmm_dst][0] = 0;
                    self.regs.ymm_high[zmm_dst][1] = 0;
                }
                self.regs.zmm_high[zmm_dst] = [0; 4];
            }
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// EVEX single-precision FP arithmetic (VADDPS, VMULPS, VSUBPS, VDIVPS)
    fn execute_evex_fp_arith_ps<F>(
        &mut self,
        ctx: &mut InsnContext,
        op: F,
    ) -> Result<Option<VcpuExit>>
    where
        F: Fn(f32, f32) -> f32,
    {
        let evex = ctx.evex.unwrap();
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;

        // Destination register (5 bits): reg + EVEX.R + EVEX.R'
        let zmm_dst = if !evex.r { reg + 8 } else { reg };
        let zmm_dst = if !evex.r_prime { zmm_dst + 16 } else { zmm_dst } as usize;

        // Source1 from EVEX.vvvv (stored inverted) extended by EVEX.V'
        let zmm_src1 = ctx.evex_vvvv() as usize;

        // Vector length from L'L
        let vl = match evex.ll {
            0 => 16, // 128-bit
            1 => 32, // 256-bit
            2 => 64, // 512-bit
            _ => 64,
        };

        // Number of f32 elements
        let num_elems = vl / 4;

        // Load source2 (register operand also honors V'/X extension to 0-31)
        let src2 = if is_memory {
            self.load_zmm_data(addr, vl)?
        } else {
            let zmm_src2 = Self::evex_rm_vec_reg(&evex, rm);
            self.get_zmm_data(zmm_src2, vl)
        };

        // Get source1
        let src1 = self.get_zmm_data(zmm_src1, vl);

        // Original destination contents (for merge masking)
        let dest_old = self.get_zmm_data(zmm_dst, vl);

        // Opmask: k0 => no masking (all elements active)
        let mask = Self::evex_kmask(&evex, &self.regs.k, num_elems);

        // Perform masked operation
        let mut result = [0u8; 64];
        for i in 0..num_elems {
            let base = i * 4;
            if (mask >> i) & 1 != 0 {
                let a = f32::from_le_bytes([
                    src1[base],
                    src1[base + 1],
                    src1[base + 2],
                    src1[base + 3],
                ]);
                let b = f32::from_le_bytes([
                    src2[base],
                    src2[base + 1],
                    src2[base + 2],
                    src2[base + 3],
                ]);
                let r = op(a, b);
                result[base..base + 4].copy_from_slice(&r.to_le_bytes());
            } else if evex.z {
                // Zeroing-masking: element becomes 0
            } else {
                // Merge-masking: keep original destination element
                result[base..base + 4].copy_from_slice(&dest_old[base..base + 4]);
            }
        }

        // Store result
        self.set_zmm_data(zmm_dst, &result[..vl], vl);

        // Zero upper bits if not 512-bit (for ZMM0-15)
        if vl < 64 && zmm_dst < 16 {
            if vl <= 16 {
                self.regs.ymm_high[zmm_dst][0] = 0;
                self.regs.ymm_high[zmm_dst][1] = 0;
            }
            self.regs.zmm_high[zmm_dst] = [0; 4];
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// EVEX double-precision FP arithmetic (VADDPD, VMULPD, VSUBPD, VDIVPD)
    fn execute_evex_fp_arith_pd<F>(
        &mut self,
        ctx: &mut InsnContext,
        op: F,
    ) -> Result<Option<VcpuExit>>
    where
        F: Fn(f64, f64) -> f64,
    {
        let evex = ctx.evex.unwrap();
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;

        // Destination register (5 bits): reg + EVEX.R + EVEX.R'
        let zmm_dst = if !evex.r { reg + 8 } else { reg };
        let zmm_dst = if !evex.r_prime { zmm_dst + 16 } else { zmm_dst } as usize;

        // Source1 from EVEX.vvvv (stored inverted) extended by EVEX.V'
        let zmm_src1 = ctx.evex_vvvv() as usize;

        // Vector length from L'L
        let vl = match evex.ll {
            0 => 16, // 128-bit
            1 => 32, // 256-bit
            2 => 64, // 512-bit
            _ => 64,
        };

        // Number of f64 elements
        let num_elems = vl / 8;

        // Load source2 (register operand also honors V'/X extension to 0-31)
        let src2 = if is_memory {
            self.load_zmm_data(addr, vl)?
        } else {
            let zmm_src2 = Self::evex_rm_vec_reg(&evex, rm);
            self.get_zmm_data(zmm_src2, vl)
        };

        // Get source1
        let src1 = self.get_zmm_data(zmm_src1, vl);

        // Original destination contents (for merge masking)
        let dest_old = self.get_zmm_data(zmm_dst, vl);

        // Opmask: k0 => no masking (all elements active)
        let mask = Self::evex_kmask(&evex, &self.regs.k, num_elems);

        // Perform masked operation
        let mut result = [0u8; 64];
        for i in 0..num_elems {
            let base = i * 8;
            if (mask >> i) & 1 != 0 {
                let a = f64::from_le_bytes([
                    src1[base],
                    src1[base + 1],
                    src1[base + 2],
                    src1[base + 3],
                    src1[base + 4],
                    src1[base + 5],
                    src1[base + 6],
                    src1[base + 7],
                ]);
                let b = f64::from_le_bytes([
                    src2[base],
                    src2[base + 1],
                    src2[base + 2],
                    src2[base + 3],
                    src2[base + 4],
                    src2[base + 5],
                    src2[base + 6],
                    src2[base + 7],
                ]);
                let r = op(a, b);
                result[base..base + 8].copy_from_slice(&r.to_le_bytes());
            } else if evex.z {
                // Zeroing-masking: element becomes 0
            } else {
                // Merge-masking: keep original destination element
                result[base..base + 8].copy_from_slice(&dest_old[base..base + 8]);
            }
        }

        // Store result
        self.set_zmm_data(zmm_dst, &result[..vl], vl);

        // Zero upper bits if not 512-bit (for ZMM0-15)
        if vl < 64 && zmm_dst < 16 {
            if vl <= 16 {
                self.regs.ymm_high[zmm_dst][0] = 0;
                self.regs.ymm_high[zmm_dst][1] = 0;
            }
            self.regs.zmm_high[zmm_dst] = [0; 4];
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// Resolve the full 0-31 vector register index for an EVEX r/m register operand.
    /// rm (3 bits) extended by EVEX.B (bit 3) and EVEX.X (bit 4, V' for reg-reg).
    #[inline]
    fn evex_rm_vec_reg(evex: &super::super::cpu::EvexPrefix, rm: u8) -> usize {
        let base = if !evex.b { rm + 8 } else { rm };
        let base = if !evex.x { base + 16 } else { base };
        base as usize
    }

    /// Compute the active-element opmask for an EVEX op.
    /// k0 (aaa == 0) means "no masking": all elements active.
    #[inline]
    fn evex_kmask(evex: &super::super::cpu::EvexPrefix, k: &[u64], num_elems: usize) -> u64 {
        let full = if num_elems >= 64 {
            u64::MAX
        } else {
            (1u64 << num_elems) - 1
        };
        if evex.aaa == 0 {
            full
        } else {
            k[evex.aaa as usize] & full
        }
    }

    /// EVEX bitwise XOR (VXORPS, VXORPD)
    fn execute_evex_bitwise_xor(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let evex = ctx.evex.unwrap();
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;

        // Destination register (5 bits)
        let zmm_dst = if !evex.r { reg + 8 } else { reg };
        let zmm_dst = if !evex.r_prime { zmm_dst + 16 } else { zmm_dst } as usize;

        // Source1 from vvvv (inverted), extended by EVEX.V'
        let zmm_src1 = ctx.evex_vvvv() as usize;

        // Vector length from L'L
        let vl = match evex.ll {
            0 => 16, // 128-bit
            1 => 32, // 256-bit
            2 => 64, // 512-bit
            _ => 64,
        };

        // Load source2 (register operand honors V'/X extension to 0-31)
        let src2 = if is_memory {
            self.load_zmm_data(addr, vl)?
        } else {
            let zmm_src2 = Self::evex_rm_vec_reg(&evex, rm);
            self.get_zmm_data(zmm_src2, vl)
        };

        // Get source1
        let src1 = self.get_zmm_data(zmm_src1, vl);

        // Perform bitwise XOR
        let mut result = [0u8; 64];
        for i in 0..vl {
            result[i] = src1[i] ^ src2[i];
        }

        // Store result
        self.set_zmm_data(zmm_dst, &result[..vl], vl);

        // Zero upper bits if not 512-bit (for ZMM0-15)
        if vl < 64 && zmm_dst < 16 {
            if vl <= 16 {
                self.regs.ymm_high[zmm_dst][0] = 0;
                self.regs.ymm_high[zmm_dst][1] = 0;
            }
            self.regs.zmm_high[zmm_dst] = [0; 4];
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    // ZMM register helper functions

    fn load_zmm_from_mem(&mut self, zmm: usize, addr: u64, vl: usize) -> Result<()> {
        if zmm < 16 {
            // ZMM0-15: load into xmm + ymm_high + zmm_high
            self.regs.xmm[zmm][0] = self.read_mem(addr, 8)?;
            self.regs.xmm[zmm][1] = self.read_mem(addr + 8, 8)?;
            if vl > 16 {
                self.regs.ymm_high[zmm][0] = self.read_mem(addr + 16, 8)?;
                self.regs.ymm_high[zmm][1] = self.read_mem(addr + 24, 8)?;
            }
            if vl > 32 {
                self.regs.zmm_high[zmm][0] = self.read_mem(addr + 32, 8)?;
                self.regs.zmm_high[zmm][1] = self.read_mem(addr + 40, 8)?;
                self.regs.zmm_high[zmm][2] = self.read_mem(addr + 48, 8)?;
                self.regs.zmm_high[zmm][3] = self.read_mem(addr + 56, 8)?;
            }
        } else {
            // ZMM16-31: load into zmm_ext
            let idx = zmm - 16;
            for i in 0..(vl / 8) {
                self.regs.zmm_ext[idx][i] = self.read_mem(addr + (i * 8) as u64, 8)?;
            }
        }
        Ok(())
    }

    fn store_zmm_to_mem(&mut self, zmm: usize, addr: u64, vl: usize) -> Result<()> {
        if zmm < 16 {
            self.write_mem(addr, self.regs.xmm[zmm][0], 8)?;
            self.write_mem(addr + 8, self.regs.xmm[zmm][1], 8)?;
            if vl > 16 {
                self.write_mem(addr + 16, self.regs.ymm_high[zmm][0], 8)?;
                self.write_mem(addr + 24, self.regs.ymm_high[zmm][1], 8)?;
            }
            if vl > 32 {
                self.write_mem(addr + 32, self.regs.zmm_high[zmm][0], 8)?;
                self.write_mem(addr + 40, self.regs.zmm_high[zmm][1], 8)?;
                self.write_mem(addr + 48, self.regs.zmm_high[zmm][2], 8)?;
                self.write_mem(addr + 56, self.regs.zmm_high[zmm][3], 8)?;
            }
        } else {
            let idx = zmm - 16;
            for i in 0..(vl / 8) {
                self.write_mem(addr + (i * 8) as u64, self.regs.zmm_ext[idx][i], 8)?;
            }
        }
        Ok(())
    }

    fn copy_zmm(&mut self, dst: usize, src: usize, vl: usize) {
        if dst < 16 && src < 16 {
            self.regs.xmm[dst] = self.regs.xmm[src];
            if vl > 16 {
                self.regs.ymm_high[dst] = self.regs.ymm_high[src];
            }
            if vl > 32 {
                self.regs.zmm_high[dst] = self.regs.zmm_high[src];
            }
        } else if dst >= 16 && src >= 16 {
            let d = dst - 16;
            let s = src - 16;
            for i in 0..(vl / 8) {
                self.regs.zmm_ext[d][i] = self.regs.zmm_ext[s][i];
            }
        } else if dst < 16 && src >= 16 {
            let s = src - 16;
            self.regs.xmm[dst][0] = self.regs.zmm_ext[s][0];
            self.regs.xmm[dst][1] = self.regs.zmm_ext[s][1];
            if vl > 16 {
                self.regs.ymm_high[dst][0] = self.regs.zmm_ext[s][2];
                self.regs.ymm_high[dst][1] = self.regs.zmm_ext[s][3];
            }
            if vl > 32 {
                self.regs.zmm_high[dst][0] = self.regs.zmm_ext[s][4];
                self.regs.zmm_high[dst][1] = self.regs.zmm_ext[s][5];
                self.regs.zmm_high[dst][2] = self.regs.zmm_ext[s][6];
                self.regs.zmm_high[dst][3] = self.regs.zmm_ext[s][7];
            }
        } else {
            // dst >= 16 && src < 16
            let d = dst - 16;
            self.regs.zmm_ext[d][0] = self.regs.xmm[src][0];
            self.regs.zmm_ext[d][1] = self.regs.xmm[src][1];
            if vl > 16 {
                self.regs.zmm_ext[d][2] = self.regs.ymm_high[src][0];
                self.regs.zmm_ext[d][3] = self.regs.ymm_high[src][1];
            }
            if vl > 32 {
                self.regs.zmm_ext[d][4] = self.regs.zmm_high[src][0];
                self.regs.zmm_ext[d][5] = self.regs.zmm_high[src][1];
                self.regs.zmm_ext[d][6] = self.regs.zmm_high[src][2];
                self.regs.zmm_ext[d][7] = self.regs.zmm_high[src][3];
            }
        }
    }

    fn get_zmm_data(&self, zmm: usize, vl: usize) -> [u8; 64] {
        let mut data = [0u8; 64];
        if zmm < 16 {
            data[0..8].copy_from_slice(&self.regs.xmm[zmm][0].to_le_bytes());
            data[8..16].copy_from_slice(&self.regs.xmm[zmm][1].to_le_bytes());
            if vl > 16 {
                data[16..24].copy_from_slice(&self.regs.ymm_high[zmm][0].to_le_bytes());
                data[24..32].copy_from_slice(&self.regs.ymm_high[zmm][1].to_le_bytes());
            }
            if vl > 32 {
                data[32..40].copy_from_slice(&self.regs.zmm_high[zmm][0].to_le_bytes());
                data[40..48].copy_from_slice(&self.regs.zmm_high[zmm][1].to_le_bytes());
                data[48..56].copy_from_slice(&self.regs.zmm_high[zmm][2].to_le_bytes());
                data[56..64].copy_from_slice(&self.regs.zmm_high[zmm][3].to_le_bytes());
            }
        } else {
            let idx = zmm - 16;
            for i in 0..(vl / 8) {
                let start = i * 8;
                data[start..start + 8].copy_from_slice(&self.regs.zmm_ext[idx][i].to_le_bytes());
            }
        }
        data
    }

    fn load_zmm_data(&mut self, addr: u64, vl: usize) -> Result<[u8; 64]> {
        let mut data = [0u8; 64];
        for i in 0..(vl / 8) {
            let val = self.read_mem(addr + (i * 8) as u64, 8)?;
            let start = i * 8;
            data[start..start + 8].copy_from_slice(&val.to_le_bytes());
        }
        Ok(data)
    }

    fn set_zmm_data(&mut self, zmm: usize, data: &[u8], vl: usize) {
        // Helper to read u64 from data with zero-padding for short slices
        let read_u64 = |offset: usize| -> u64 {
            let mut bytes = [0u8; 8];
            let end = (offset + 8).min(data.len());
            if offset < data.len() {
                bytes[..end - offset].copy_from_slice(&data[offset..end]);
            }
            u64::from_le_bytes(bytes)
        };

        if zmm < 16 {
            self.regs.xmm[zmm][0] = read_u64(0);
            if vl > 8 {
                self.regs.xmm[zmm][1] = read_u64(8);
            } else {
                self.regs.xmm[zmm][1] = 0;
            }
            if vl > 16 {
                self.regs.ymm_high[zmm][0] = read_u64(16);
                self.regs.ymm_high[zmm][1] = read_u64(24);
            }
            if vl > 32 {
                self.regs.zmm_high[zmm][0] = read_u64(32);
                self.regs.zmm_high[zmm][1] = read_u64(40);
                self.regs.zmm_high[zmm][2] = read_u64(48);
                self.regs.zmm_high[zmm][3] = read_u64(56);
            }
        } else {
            let idx = zmm - 16;
            for i in 0..(vl / 8) {
                self.regs.zmm_ext[idx][i] = read_u64(i * 8);
            }
        }
    }

    /// EVEX 0F38 opcode map (mm=2)
    fn execute_evex_0f38(&mut self, ctx: &mut InsnContext, opcode: u8) -> Result<Option<VcpuExit>> {
        let evex = ctx
            .evex
            .ok_or_else(|| Error::Emulator("EVEX context missing".to_string()))?;

        match opcode {
            // VPMULLD/VPMULLQ (0x40)
            // W=0: VPMULLD (32-bit elements)
            // W=1: VPMULLQ (64-bit elements)
            0x40 if evex.pp == 1 => {
                if evex.w {
                    insn::simd::vpmullq(self, ctx)
                } else {
                    insn::simd::vpmulld_evex(self, ctx)
                }
            }
            // Broadcasts (pp=1 / 66):
            // VBROADCASTSS (0x18, W0): broadcast 32-bit float
            0x18 if evex.pp == 1 && !evex.w => insn::simd::evex_broadcast(self, ctx, 4),
            // VBROADCASTSD (0x19, W1): broadcast 64-bit double
            0x19 if evex.pp == 1 && evex.w => insn::simd::evex_broadcast(self, ctx, 8),
            // VPBROADCASTD (0x58, W0): broadcast 32-bit integer
            0x58 if evex.pp == 1 && !evex.w => insn::simd::evex_broadcast(self, ctx, 4),
            // VPBROADCASTQ (0x59, W1): broadcast 64-bit integer
            0x59 if evex.pp == 1 && evex.w => insn::simd::evex_broadcast(self, ctx, 8),
            // VPBROADCASTB (0x78, W0): broadcast 8-bit integer
            0x78 if evex.pp == 1 && !evex.w => insn::simd::evex_broadcast(self, ctx, 1),
            // VPBROADCASTW (0x79, W0): broadcast 16-bit integer
            0x79 if evex.pp == 1 && !evex.w => insn::simd::evex_broadcast(self, ctx, 2),

            // VPCMPEQQ (0x29, W1): qword equality compare into mask
            0x29 if evex.pp == 1 && evex.w => {
                insn::simd::evex_int_cmp(self, ctx, 8, true, insn::simd::CmpPred::Eq, false)
            }
            // VPCMPGTQ (0x37, W1): qword signed greater-than compare into mask
            0x37 if evex.pp == 1 && evex.w => {
                insn::simd::evex_int_cmp(self, ctx, 8, true, insn::simd::CmpPred::Gt, false)
            }

            // VEXPANDPS/VEXPANDPD (0x88)
            0x88 if evex.pp == 1 => {
                if evex.w {
                    insn::simd::vexpand_evex(self, ctx, 8, "VEXPANDPD")
                } else {
                    insn::simd::vexpand_evex(self, ctx, 4, "VEXPANDPS")
                }
            }
            // VPEXPANDD/VPEXPANDQ (0x89)
            0x89 if evex.pp == 1 => {
                if evex.w {
                    insn::simd::vexpand_evex(self, ctx, 8, "VPEXPANDQ")
                } else {
                    insn::simd::vexpand_evex(self, ctx, 4, "VPEXPANDD")
                }
            }
            // VCOMPRESSPS/VCOMPRESSPD (0x8A)
            0x8A if evex.pp == 1 => {
                if evex.w {
                    insn::simd::vcompress_evex(self, ctx, 8, "VCOMPRESSPD")
                } else {
                    insn::simd::vcompress_evex(self, ctx, 4, "VCOMPRESSPS")
                }
            }
            // VPCOMPRESSD/VPCOMPRESSQ (0x8B)
            0x8B if evex.pp == 1 => {
                if evex.w {
                    insn::simd::vcompress_evex(self, ctx, 8, "VPCOMPRESSQ")
                } else {
                    insn::simd::vcompress_evex(self, ctx, 4, "VPCOMPRESSD")
                }
            }

            // ============================================================================
            // AVX10.1 VNNI Instructions
            // ============================================================================

            // VPDPBUSD (0x50) - Multiply and Add Unsigned and Signed Bytes
            0x50 if evex.pp == 1 => self.execute_vpdpbusd(ctx, false),
            // VPDPBUSDS (0x51) - Multiply and Add Unsigned and Signed Bytes with Saturation
            0x51 if evex.pp == 1 => self.execute_vpdpbusd(ctx, true),
            // VPDPWSSD (0x52) - Multiply and Add Signed Word Integers
            0x52 if evex.pp == 1 && !evex.w => self.execute_vpdpwssd(ctx, false),
            // VPDPWSSDS (0x53) - Multiply and Add Signed Word Integers with Saturation
            0x53 if evex.pp == 1 => self.execute_vpdpwssd(ctx, true),

            // ============================================================================
            // AVX10.1 IFMA Instructions
            // ============================================================================

            // VPMADD52LUQ (0xB4) - Packed Multiply of Unsigned 52-bit and Add Low Qword
            0xB4 if evex.pp == 1 && evex.w => self.execute_vpmadd52(ctx, false),
            // VPMADD52HUQ (0xB5) - Packed Multiply of Unsigned 52-bit and Add High Qword
            0xB5 if evex.pp == 1 && evex.w => self.execute_vpmadd52(ctx, true),

            // ============================================================================
            // AVX10.1 VPOPCNTDQ Instructions
            // ============================================================================

            // VPOPCNTB/W (0x54) - Population count for packed bytes/words
            0x54 if evex.pp == 1 => {
                if evex.w {
                    self.execute_vpopcnt(ctx, 2) // VPOPCNTW
                } else {
                    self.execute_vpopcnt(ctx, 1) // VPOPCNTB
                }
            }
            // VPOPCNTD/Q (0x55) - Population count for packed dwords/qwords
            0x55 if evex.pp == 1 => {
                if evex.w {
                    self.execute_vpopcnt(ctx, 8) // VPOPCNTQ
                } else {
                    self.execute_vpopcnt(ctx, 4) // VPOPCNTD
                }
            }

            // ============================================================================
            // AVX10.1 VBMI Instructions
            // ============================================================================

            // VPERMB (0x8D) - Permute Packed Bytes Elements
            0x8D if evex.pp == 1 && !evex.w => self.execute_vpermb(ctx),
            // VPERMI2B (0x75) - Full Permute of Bytes from Two Tables Overwriting Index
            0x75 if evex.pp == 1 && !evex.w => self.execute_vpermi2b(ctx),
            // VPERMT2B (0x7D) - Full Permute of Bytes from Two Tables Overwriting Table
            0x7D if evex.pp == 1 && !evex.w => self.execute_vpermt2b(ctx),

            // ============================================================================
            // AVX10.1 BITALG Instructions
            // ============================================================================

            // VPSHUFBITQMB (0x8F) - Shuffle Bits from Quadword Elements Using Byte Indexes into Mask
            0x8F if evex.pp == 1 && !evex.w => self.execute_vpshufbitqmb(ctx),

            // ============================================================================
            // AVX10.1 BF16 Instructions
            // ============================================================================

            // VDPBF16PS (0x52) - Dot Product of BF16 Pairs Accumulated into FP32
            0x52 if evex.pp == 2 && !evex.w => self.execute_vdpbf16ps(ctx),
            // VCVTNEPS2BF16 (0x72) - Convert Packed Single to BF16
            0x72 if evex.pp == 2 && !evex.w => self.execute_vcvtneps2bf16(ctx),
            // VCVTNE2PS2BF16 (0x72) - Convert Two Packed Single to BF16
            0x72 if evex.pp == 3 && !evex.w => self.execute_vcvtne2ps2bf16(ctx),

            // ============================================================================
            // AVX10.2 Saturation Conversion Instructions
            // ============================================================================

            // VCVTTPS2IBS (0x68) - Convert with Truncation Packed Single to Signed Byte with Saturation
            0x68 if evex.pp == 0 && !evex.w => self.execute_vcvttps2ibs(ctx),
            // VCVTTPS2IUBS (0x6A) - Convert with Truncation Packed Single to Unsigned Byte with Saturation
            0x6A if evex.pp == 0 && !evex.w => self.execute_vcvttps2iubs(ctx),
            // VCVTTPD2QQS (0x6D) - Convert with Truncation Packed Double to Signed Qword with Saturation
            0x6D if evex.pp == 1 && evex.w => self.execute_vcvttpd2qqs(ctx),
            // VCVTTPD2UQQS (0x6C) - Convert with Truncation Packed Double to Unsigned Qword with Saturation
            0x6C if evex.pp == 1 && evex.w => self.execute_vcvttpd2uqqs(ctx),

            // ============================================================================
            // AVX10.2 Media Acceleration Instructions (VPDPB*/VPDPW*)
            // ============================================================================

            // VPDPBSSD (0x50) - Multiply and Add Signed Byte Integers
            0x50 if evex.pp == 2 && !evex.w => self.execute_vpdpbssd(ctx, false),
            // VPDPBSSDS (0x51) - Multiply and Add Signed Byte Integers with Saturation
            0x51 if evex.pp == 2 && !evex.w => self.execute_vpdpbssd(ctx, true),
            // VPDPBSUD (0x50) - Multiply and Add Signed/Unsigned Byte Integers
            0x50 if evex.pp == 2 && evex.w => self.execute_vpdpbsud(ctx, false),
            // VPDPBSUDS (0x51) - Multiply and Add Signed/Unsigned Byte Integers with Saturation
            0x51 if evex.pp == 2 && evex.w => self.execute_vpdpbsud(ctx, true),
            // VPDPBUUD (0x50) - Multiply and Add Unsigned Byte Integers
            0x50 if evex.pp == 0 && evex.w => self.execute_vpdpbuud(ctx, false),
            // VPDPBUUDS (0x51) - Multiply and Add Unsigned Byte Integers with Saturation
            0x51 if evex.pp == 0 && evex.w => self.execute_vpdpbuud(ctx, true),
            // VPDPWSUD (0xD2) - Multiply and Add Signed/Unsigned Word Integers
            0xD2 if evex.pp == 2 && !evex.w => self.execute_vpdpwsud(ctx, false),
            // VPDPWSUDS (0xD3) - Multiply and Add Signed/Unsigned Word Integers with Saturation
            0xD3 if evex.pp == 2 && !evex.w => self.execute_vpdpwsud(ctx, true),
            // VPDPWUSD (0xD2) - Multiply and Add Unsigned/Signed Word Integers
            0xD2 if evex.pp == 1 && !evex.w => self.execute_vpdpwusd(ctx, false),
            // VPDPWUSDS (0xD3) - Multiply and Add Unsigned/Signed Word Integers with Saturation
            0xD3 if evex.pp == 1 && !evex.w => self.execute_vpdpwusd(ctx, true),
            // VPDPWUUD (0xD2) - Multiply and Add Unsigned Word Integers
            0xD2 if evex.pp == 0 && !evex.w => self.execute_vpdpwuud(ctx, false),
            // VPDPWUUDS (0xD3) - Multiply and Add Unsigned Word Integers with Saturation
            0xD3 if evex.pp == 0 && !evex.w => self.execute_vpdpwuud(ctx, true),

            _ => Err(Error::Emulator(format!(
                "Unimplemented EVEX.0F38 opcode {:#04x} (W={}) at RIP={:#x}",
                opcode, evex.w as u8, self.regs.rip
            ))),
        }
    }

    /// EVEX 0F3A opcode map (mm=3)
    fn execute_evex_0f3a(&mut self, ctx: &mut InsnContext, opcode: u8) -> Result<Option<VcpuExit>> {
        let evex = ctx
            .evex
            .ok_or_else(|| Error::Emulator("EVEX context missing".to_string()))?;

        match opcode {
            // ============================================================================
            // EVEX integer compare with imm8 predicate (write into k-mask)
            // ============================================================================

            // VPCMPUD (0x1E, W0) / VPCMPUQ (0x1E, W1): unsigned dword/qword
            0x1E if evex.pp == 1 => {
                let es = if evex.w { 8 } else { 4 };
                insn::simd::evex_int_cmp(self, ctx, es, false, insn::simd::CmpPred::Eq, true)
            }
            // VPCMPD (0x1F, W0) / VPCMPQ (0x1F, W1): signed dword/qword
            0x1F if evex.pp == 1 => {
                let es = if evex.w { 8 } else { 4 };
                insn::simd::evex_int_cmp(self, ctx, es, true, insn::simd::CmpPred::Eq, true)
            }
            // VPCMPUB (0x3E, W0) / VPCMPUW (0x3E, W1): unsigned byte/word
            0x3E if evex.pp == 1 => {
                let es = if evex.w { 2 } else { 1 };
                insn::simd::evex_int_cmp(self, ctx, es, false, insn::simd::CmpPred::Eq, true)
            }
            // VPCMPB (0x3F, W0) / VPCMPW (0x3F, W1): signed byte/word
            0x3F if evex.pp == 1 => {
                let es = if evex.w { 2 } else { 1 };
                insn::simd::evex_int_cmp(self, ctx, es, true, insn::simd::CmpPred::Eq, true)
            }

            // ============================================================================
            // AVX10.2 VMPSADBW Instruction
            // ============================================================================

            // VMPSADBW (0x42) - Compute Multiple Packed Sums of Absolute Difference
            0x42 if evex.pp == 1 => self.execute_vmpsadbw(ctx),

            // ============================================================================
            // AVX10.2 VMINMAX Instructions
            // ============================================================================

            // VMINMAXPS (0x52) - Minimum/Maximum of Packed Single-Precision Floats
            0x52 if evex.pp == 0 && !evex.w => self.execute_vminmax_ps(ctx),
            // VMINMAXPD (0x52) - Minimum/Maximum of Packed Double-Precision Floats
            0x52 if evex.pp == 1 && evex.w => self.execute_vminmax_pd(ctx),
            // VMINMAXSS (0x53) - Minimum/Maximum of Scalar Single-Precision Float
            0x53 if evex.pp == 0 && !evex.w => self.execute_vminmax_ss(ctx),
            // VMINMAXSD (0x53) - Minimum/Maximum of Scalar Double-Precision Float
            0x53 if evex.pp == 1 && evex.w => self.execute_vminmax_sd(ctx),

            _ => Err(Error::Emulator(format!(
                "Unimplemented EVEX.0F3A opcode {:#04x} at RIP={:#x}",
                opcode, self.regs.rip
            ))),
        }
    }

    /// EVEX MAP5 opcode map (mm=5) - AVX-512 FP16 instructions
    fn execute_evex_map5(&mut self, ctx: &mut InsnContext, opcode: u8) -> Result<Option<VcpuExit>> {
        let evex = ctx
            .evex
            .ok_or_else(|| Error::Emulator("EVEX context missing".to_string()))?;

        // MAP5 instructions are FP16 (half-precision) arithmetic
        // pp=0 (NP), W=0 for packed FP16
        match opcode {
            // VADDPH (0x58)
            0x58 if evex.pp == 0 => self.execute_evex_fp16_arith(ctx, |a, b| a + b),
            // VMULPH (0x59)
            0x59 if evex.pp == 0 => self.execute_evex_fp16_arith(ctx, |a, b| a * b),
            // VSUBPH (0x5C)
            0x5C if evex.pp == 0 => self.execute_evex_fp16_arith(ctx, |a, b| a - b),
            // VDIVPH (0x5E)
            0x5E if evex.pp == 0 => self.execute_evex_fp16_arith(ctx, |a, b| a / b),
            _ => Err(Error::Emulator(format!(
                "Unimplemented EVEX.MAP5 opcode {:#04x} (pp={}) at RIP={:#x}",
                opcode, evex.pp, self.regs.rip
            ))),
        }
    }

    /// EVEX FP16 (half-precision) arithmetic (VADDPH, VSUBPH, VMULPH, VDIVPH)
    fn execute_evex_fp16_arith<F>(
        &mut self,
        ctx: &mut InsnContext,
        op: F,
    ) -> Result<Option<VcpuExit>>
    where
        F: Fn(f32, f32) -> f32,
    {
        let evex = ctx.evex.unwrap();
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;

        // Destination register (5 bits)
        let zmm_dst = if !evex.r { reg + 8 } else { reg };
        let zmm_dst = if !evex.r_prime { zmm_dst + 16 } else { zmm_dst } as usize;

        // Source1 from vvvv (inverted), extended by EVEX.V'
        let zmm_src1 = ctx.evex_vvvv() as usize;

        // Vector length from L'L
        let vl = match evex.ll {
            0 => 16, // 128-bit (8 FP16 values)
            1 => 32, // 256-bit (16 FP16 values)
            2 => 64, // 512-bit (32 FP16 values)
            _ => 64,
        };

        // Number of FP16 elements (2 bytes each)
        let num_elems = vl / 2;

        // Load source2
        let src2 = if is_memory {
            self.load_zmm_data(addr, vl)?
        } else {
            let zmm_src2 = if !evex.b { rm + 8 } else { rm } as usize;
            self.get_zmm_data(zmm_src2, vl)
        };

        // Get source1
        let src1 = self.get_zmm_data(zmm_src1, vl);

        // Perform operation on each FP16 element
        let mut result = [0u8; 64];
        for i in 0..num_elems {
            // Convert FP16 to f32, perform operation, convert back to FP16
            let a_fp16 = u16::from_le_bytes([src1[i * 2], src1[i * 2 + 1]]);
            let b_fp16 = u16::from_le_bytes([src2[i * 2], src2[i * 2 + 1]]);
            let a = fp16_to_f32(a_fp16);
            let b = fp16_to_f32(b_fp16);
            let r = op(a, b);
            let r_fp16 = f32_to_fp16(r);
            let bytes = r_fp16.to_le_bytes();
            result[i * 2..i * 2 + 2].copy_from_slice(&bytes);
        }

        // Store result
        self.set_zmm_data(zmm_dst, &result[..vl], vl);

        // Zero upper bits if not 512-bit (for ZMM0-15)
        if vl < 64 && zmm_dst < 16 {
            if vl <= 16 {
                self.regs.ymm_high[zmm_dst][0] = 0;
                self.regs.ymm_high[zmm_dst][1] = 0;
            }
            self.regs.zmm_high[zmm_dst] = [0; 4];
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    // ============================================================================
    // AVX10.1 VNNI Instruction Implementations
    // ============================================================================

    /// VPDPBUSD/VPDPBUSDS - Multiply and Add Unsigned and Signed Bytes
    fn execute_vpdpbusd(
        &mut self,
        ctx: &mut InsnContext,
        saturate: bool,
    ) -> Result<Option<VcpuExit>> {
        let evex = ctx.evex.unwrap();
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;

        // Destination/accumulator register
        let zmm_dst = if !evex.r { reg + 8 } else { reg };
        let zmm_dst = if !evex.r_prime { zmm_dst + 16 } else { zmm_dst } as usize;

        // Source1 from vvvv (first multiplicand)
        let zmm_src1 = (evex.vvvv ^ 0xF) as usize;

        // Vector length from L'L
        let vl = match evex.ll {
            0 => 16,
            1 => 32,
            2 => 64,
            _ => 64,
        };

        let num_dwords = vl / 4;

        // Load source2
        let src2 = if is_memory {
            self.load_zmm_data(addr, vl)?
        } else {
            let zmm_src2 = if !evex.b { rm + 8 } else { rm } as usize;
            self.get_zmm_data(zmm_src2, vl)
        };

        let src1 = self.get_zmm_data(zmm_src1, vl);
        let mut dst = self.get_zmm_data(zmm_dst, vl);

        // Process each dword
        for i in 0..num_dwords {
            let base = i * 4;
            // Each dword contains 4 bytes
            let mut sum =
                i32::from_le_bytes([dst[base], dst[base + 1], dst[base + 2], dst[base + 3]]) as i64;

            for j in 0..4 {
                let a = src1[base + j] as u8 as i32; // unsigned byte
                let b = src2[base + j] as i8 as i32; // signed byte
                sum += (a * b) as i64;
            }

            let result = if saturate {
                sum.clamp(i32::MIN as i64, i32::MAX as i64) as i32
            } else {
                sum as i32
            };

            let bytes = result.to_le_bytes();
            dst[base..base + 4].copy_from_slice(&bytes);
        }

        self.set_zmm_data(zmm_dst, &dst[..vl], vl);

        if vl < 64 && zmm_dst < 16 {
            if vl <= 16 {
                self.regs.ymm_high[zmm_dst][0] = 0;
                self.regs.ymm_high[zmm_dst][1] = 0;
            }
            self.regs.zmm_high[zmm_dst] = [0; 4];
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// VPDPWSSD/VPDPWSSDS - Multiply and Add Signed Word Integers
    fn execute_vpdpwssd(
        &mut self,
        ctx: &mut InsnContext,
        saturate: bool,
    ) -> Result<Option<VcpuExit>> {
        let evex = ctx.evex.unwrap();
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;

        let zmm_dst = if !evex.r { reg + 8 } else { reg };
        let zmm_dst = if !evex.r_prime { zmm_dst + 16 } else { zmm_dst } as usize;

        let zmm_src1 = (evex.vvvv ^ 0xF) as usize;

        let vl = match evex.ll {
            0 => 16,
            1 => 32,
            2 => 64,
            _ => 64,
        };

        let num_dwords = vl / 4;

        let src2 = if is_memory {
            self.load_zmm_data(addr, vl)?
        } else {
            let zmm_src2 = if !evex.b { rm + 8 } else { rm } as usize;
            self.get_zmm_data(zmm_src2, vl)
        };

        let src1 = self.get_zmm_data(zmm_src1, vl);
        let mut dst = self.get_zmm_data(zmm_dst, vl);

        for i in 0..num_dwords {
            let base = i * 4;
            let mut sum =
                i32::from_le_bytes([dst[base], dst[base + 1], dst[base + 2], dst[base + 3]]) as i64;

            // Two pairs of signed words per dword
            let a0 = i16::from_le_bytes([src1[base], src1[base + 1]]) as i32;
            let b0 = i16::from_le_bytes([src2[base], src2[base + 1]]) as i32;
            let a1 = i16::from_le_bytes([src1[base + 2], src1[base + 3]]) as i32;
            let b1 = i16::from_le_bytes([src2[base + 2], src2[base + 3]]) as i32;

            sum += (a0 * b0 + a1 * b1) as i64;

            let result = if saturate {
                sum.clamp(i32::MIN as i64, i32::MAX as i64) as i32
            } else {
                sum as i32
            };

            let bytes = result.to_le_bytes();
            dst[base..base + 4].copy_from_slice(&bytes);
        }

        self.set_zmm_data(zmm_dst, &dst[..vl], vl);

        if vl < 64 && zmm_dst < 16 {
            if vl <= 16 {
                self.regs.ymm_high[zmm_dst][0] = 0;
                self.regs.ymm_high[zmm_dst][1] = 0;
            }
            self.regs.zmm_high[zmm_dst] = [0; 4];
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    // ============================================================================
    // AVX10.1 IFMA Instruction Implementations
    // ============================================================================

    /// VPMADD52LUQ/VPMADD52HUQ - Packed Multiply of Unsigned 52-bit and Add
    fn execute_vpmadd52(&mut self, ctx: &mut InsnContext, high: bool) -> Result<Option<VcpuExit>> {
        let evex = ctx.evex.unwrap();
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;

        let zmm_dst = if !evex.r { reg + 8 } else { reg };
        let zmm_dst = if !evex.r_prime { zmm_dst + 16 } else { zmm_dst } as usize;

        let zmm_src1 = (evex.vvvv ^ 0xF) as usize;

        let vl = match evex.ll {
            0 => 16,
            1 => 32,
            2 => 64,
            _ => 64,
        };

        let num_qwords = vl / 8;

        let src2 = if is_memory {
            self.load_zmm_data(addr, vl)?
        } else {
            let zmm_src2 = if !evex.b { rm + 8 } else { rm } as usize;
            self.get_zmm_data(zmm_src2, vl)
        };

        let src1 = self.get_zmm_data(zmm_src1, vl);
        let mut dst = self.get_zmm_data(zmm_dst, vl);

        for i in 0..num_qwords {
            let base = i * 8;
            let a = u64::from_le_bytes([
                src1[base],
                src1[base + 1],
                src1[base + 2],
                src1[base + 3],
                src1[base + 4],
                src1[base + 5],
                src1[base + 6],
                src1[base + 7],
            ]) & 0x000F_FFFF_FFFF_FFFF; // 52-bit mask

            let b = u64::from_le_bytes([
                src2[base],
                src2[base + 1],
                src2[base + 2],
                src2[base + 3],
                src2[base + 4],
                src2[base + 5],
                src2[base + 6],
                src2[base + 7],
            ]) & 0x000F_FFFF_FFFF_FFFF;

            let d = u64::from_le_bytes([
                dst[base],
                dst[base + 1],
                dst[base + 2],
                dst[base + 3],
                dst[base + 4],
                dst[base + 5],
                dst[base + 6],
                dst[base + 7],
            ]);

            // 52x52 multiplication gives 104-bit result
            let product = (a as u128) * (b as u128);
            let result = if high {
                // High 52 bits of 104-bit product, added to dest
                d.wrapping_add(((product >> 52) & 0x000F_FFFF_FFFF_FFFF) as u64)
            } else {
                // Low 52 bits of 104-bit product, added to dest
                d.wrapping_add((product & 0x000F_FFFF_FFFF_FFFF) as u64)
            };

            let bytes = result.to_le_bytes();
            dst[base..base + 8].copy_from_slice(&bytes);
        }

        self.set_zmm_data(zmm_dst, &dst[..vl], vl);

        if vl < 64 && zmm_dst < 16 {
            if vl <= 16 {
                self.regs.ymm_high[zmm_dst][0] = 0;
                self.regs.ymm_high[zmm_dst][1] = 0;
            }
            self.regs.zmm_high[zmm_dst] = [0; 4];
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    // ============================================================================
    // AVX10.1 VPOPCNTDQ Instruction Implementations
    // ============================================================================

    /// VPOPCNTB/W/D/Q - Population count for packed elements
    fn execute_vpopcnt(
        &mut self,
        ctx: &mut InsnContext,
        elem_size: usize,
    ) -> Result<Option<VcpuExit>> {
        let evex = ctx.evex.unwrap();
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;

        let zmm_dst = if !evex.r { reg + 8 } else { reg };
        let zmm_dst = if !evex.r_prime { zmm_dst + 16 } else { zmm_dst } as usize;

        let vl = match evex.ll {
            0 => 16,
            1 => 32,
            2 => 64,
            _ => 64,
        };

        let src = if is_memory {
            self.load_zmm_data(addr, vl)?
        } else {
            let zmm_src = if !evex.b { rm + 8 } else { rm } as usize;
            self.get_zmm_data(zmm_src, vl)
        };

        let mut dst = [0u8; 64];
        let num_elems = vl / elem_size;

        for i in 0..num_elems {
            let base = i * elem_size;
            let mut count = 0u64;

            for j in 0..elem_size {
                count += src[base + j].count_ones() as u64;
            }

            match elem_size {
                1 => dst[base] = count as u8,
                2 => {
                    let bytes = (count as u16).to_le_bytes();
                    dst[base..base + 2].copy_from_slice(&bytes);
                }
                4 => {
                    let bytes = (count as u32).to_le_bytes();
                    dst[base..base + 4].copy_from_slice(&bytes);
                }
                8 => {
                    let bytes = count.to_le_bytes();
                    dst[base..base + 8].copy_from_slice(&bytes);
                }
                _ => {}
            }
        }

        self.set_zmm_data(zmm_dst, &dst[..vl], vl);

        if vl < 64 && zmm_dst < 16 {
            if vl <= 16 {
                self.regs.ymm_high[zmm_dst][0] = 0;
                self.regs.ymm_high[zmm_dst][1] = 0;
            }
            self.regs.zmm_high[zmm_dst] = [0; 4];
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    // ============================================================================
    // AVX10.1 VBMI Instruction Implementations
    // ============================================================================

    /// VPERMB - Permute Packed Bytes Elements
    fn execute_vpermb(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let evex = ctx.evex.unwrap();
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;

        let zmm_dst = if !evex.r { reg + 8 } else { reg };
        let zmm_dst = if !evex.r_prime { zmm_dst + 16 } else { zmm_dst } as usize;

        let zmm_idx = (evex.vvvv ^ 0xF) as usize;

        let vl = match evex.ll {
            0 => 16,
            1 => 32,
            2 => 64,
            _ => 64,
        };

        let src = if is_memory {
            self.load_zmm_data(addr, vl)?
        } else {
            let zmm_src = if !evex.b { rm + 8 } else { rm } as usize;
            self.get_zmm_data(zmm_src, vl)
        };

        let idx = self.get_zmm_data(zmm_idx, vl);
        let mut dst = [0u8; 64];

        for i in 0..vl {
            let index = (idx[i] as usize) % vl;
            dst[i] = src[index];
        }

        self.set_zmm_data(zmm_dst, &dst[..vl], vl);

        if vl < 64 && zmm_dst < 16 {
            if vl <= 16 {
                self.regs.ymm_high[zmm_dst][0] = 0;
                self.regs.ymm_high[zmm_dst][1] = 0;
            }
            self.regs.zmm_high[zmm_dst] = [0; 4];
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// VPERMI2B - Full Permute of Bytes from Two Tables Overwriting Index
    fn execute_vpermi2b(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let evex = ctx.evex.unwrap();
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;

        let zmm_idx = if !evex.r { reg + 8 } else { reg };
        let zmm_idx = if !evex.r_prime { zmm_idx + 16 } else { zmm_idx } as usize;

        let zmm_src1 = (evex.vvvv ^ 0xF) as usize;

        let vl = match evex.ll {
            0 => 16,
            1 => 32,
            2 => 64,
            _ => 64,
        };

        let src2 = if is_memory {
            self.load_zmm_data(addr, vl)?
        } else {
            let zmm_src2 = if !evex.b { rm + 8 } else { rm } as usize;
            self.get_zmm_data(zmm_src2, vl)
        };

        let src1 = self.get_zmm_data(zmm_src1, vl);
        let idx = self.get_zmm_data(zmm_idx, vl);

        let mut dst = [0u8; 64];
        let table_size = vl * 2;

        for i in 0..vl {
            let index = (idx[i] as usize) % table_size;
            dst[i] = if index < vl {
                src1[index]
            } else {
                src2[index - vl]
            };
        }

        self.set_zmm_data(zmm_idx, &dst[..vl], vl);

        if vl < 64 && zmm_idx < 16 {
            if vl <= 16 {
                self.regs.ymm_high[zmm_idx][0] = 0;
                self.regs.ymm_high[zmm_idx][1] = 0;
            }
            self.regs.zmm_high[zmm_idx] = [0; 4];
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// VPERMT2B - Full Permute of Bytes from Two Tables Overwriting Table
    fn execute_vpermt2b(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let evex = ctx.evex.unwrap();
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;

        let zmm_dst = if !evex.r { reg + 8 } else { reg };
        let zmm_dst = if !evex.r_prime { zmm_dst + 16 } else { zmm_dst } as usize;

        let zmm_idx = (evex.vvvv ^ 0xF) as usize;

        let vl = match evex.ll {
            0 => 16,
            1 => 32,
            2 => 64,
            _ => 64,
        };

        let src2 = if is_memory {
            self.load_zmm_data(addr, vl)?
        } else {
            let zmm_src2 = if !evex.b { rm + 8 } else { rm } as usize;
            self.get_zmm_data(zmm_src2, vl)
        };

        let src1 = self.get_zmm_data(zmm_dst, vl);
        let idx = self.get_zmm_data(zmm_idx, vl);

        let mut dst = [0u8; 64];
        let table_size = vl * 2;

        for i in 0..vl {
            let index = (idx[i] as usize) % table_size;
            dst[i] = if index < vl {
                src1[index]
            } else {
                src2[index - vl]
            };
        }

        self.set_zmm_data(zmm_dst, &dst[..vl], vl);

        if vl < 64 && zmm_dst < 16 {
            if vl <= 16 {
                self.regs.ymm_high[zmm_dst][0] = 0;
                self.regs.ymm_high[zmm_dst][1] = 0;
            }
            self.regs.zmm_high[zmm_dst] = [0; 4];
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    // ============================================================================
    // AVX10.1 BITALG Instruction Implementations
    // ============================================================================

    /// VPSHUFBITQMB - Shuffle Bits from Quadword Elements Using Byte Indexes into Mask
    fn execute_vpshufbitqmb(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let evex = ctx.evex.unwrap();
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;

        let k_dst = reg as usize & 0x7;

        let zmm_src1 = (evex.vvvv ^ 0xF) as usize;

        let vl = match evex.ll {
            0 => 16,
            1 => 32,
            2 => 64,
            _ => 64,
        };

        let src2 = if is_memory {
            self.load_zmm_data(addr, vl)?
        } else {
            let zmm_src2 = if !evex.b { rm + 8 } else { rm } as usize;
            self.get_zmm_data(zmm_src2, vl)
        };

        let src1 = self.get_zmm_data(zmm_src1, vl);
        let mut result: u64 = 0;

        // Process each qword
        for qword_idx in 0..(vl / 8) {
            let qword_base = qword_idx * 8;
            let mut qword = 0u64;
            for i in 0..8 {
                qword |= (src1[qword_base + i] as u64) << (i * 8);
            }

            // Each byte in src2 selects a bit from the corresponding qword
            for byte_idx in 0..8 {
                let bit_index = src2[qword_base + byte_idx] & 0x3F; // 6-bit index
                let bit = (qword >> bit_index) & 1;
                result |= bit << (qword_idx * 8 + byte_idx);
            }
        }

        self.regs.k[k_dst] = result;

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    // ============================================================================
    // AVX10.1 BF16 Instruction Implementations
    // ============================================================================

    /// VDPBF16PS - Dot Product of BF16 Pairs Accumulated into FP32
    fn execute_vdpbf16ps(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let evex = ctx.evex.unwrap();
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;

        let zmm_dst = if !evex.r { reg + 8 } else { reg };
        let zmm_dst = if !evex.r_prime { zmm_dst + 16 } else { zmm_dst } as usize;

        let zmm_src1 = (evex.vvvv ^ 0xF) as usize;

        let vl = match evex.ll {
            0 => 16,
            1 => 32,
            2 => 64,
            _ => 64,
        };

        let num_floats = vl / 4;

        let src2 = if is_memory {
            self.load_zmm_data(addr, vl)?
        } else {
            let zmm_src2 = if !evex.b { rm + 8 } else { rm } as usize;
            self.get_zmm_data(zmm_src2, vl)
        };

        let src1 = self.get_zmm_data(zmm_src1, vl);
        let mut dst = self.get_zmm_data(zmm_dst, vl);

        for i in 0..num_floats {
            let base = i * 4;
            // Read accumulator as f32
            let acc = f32::from_le_bytes([dst[base], dst[base + 1], dst[base + 2], dst[base + 3]]);

            // Two BF16 values per dword
            let a0 = bf16_to_f32(u16::from_le_bytes([src1[base], src1[base + 1]]));
            let b0 = bf16_to_f32(u16::from_le_bytes([src2[base], src2[base + 1]]));
            let a1 = bf16_to_f32(u16::from_le_bytes([src1[base + 2], src1[base + 3]]));
            let b1 = bf16_to_f32(u16::from_le_bytes([src2[base + 2], src2[base + 3]]));

            let result = acc + a0 * b0 + a1 * b1;
            let bytes = result.to_le_bytes();
            dst[base..base + 4].copy_from_slice(&bytes);
        }

        self.set_zmm_data(zmm_dst, &dst[..vl], vl);

        if vl < 64 && zmm_dst < 16 {
            if vl <= 16 {
                self.regs.ymm_high[zmm_dst][0] = 0;
                self.regs.ymm_high[zmm_dst][1] = 0;
            }
            self.regs.zmm_high[zmm_dst] = [0; 4];
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// VCVTNEPS2BF16 - Convert Packed Single-Precision to BF16
    fn execute_vcvtneps2bf16(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let evex = ctx.evex.unwrap();
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;

        let zmm_dst = if !evex.r { reg + 8 } else { reg };
        let zmm_dst = if !evex.r_prime { zmm_dst + 16 } else { zmm_dst } as usize;

        let vl = match evex.ll {
            0 => 16,
            1 => 32,
            2 => 64,
            _ => 64,
        };

        let src = if is_memory {
            self.load_zmm_data(addr, vl)?
        } else {
            let zmm_src = if !evex.b { rm + 8 } else { rm } as usize;
            self.get_zmm_data(zmm_src, vl)
        };

        let num_floats = vl / 4;
        let dst_vl = vl / 2; // Output is half the size
        let mut dst = [0u8; 64];

        for i in 0..num_floats {
            let src_base = i * 4;
            let f = f32::from_le_bytes([
                src[src_base],
                src[src_base + 1],
                src[src_base + 2],
                src[src_base + 3],
            ]);
            let bf16 = f32_to_bf16(f);
            let dst_base = i * 2;
            let bytes = bf16.to_le_bytes();
            dst[dst_base..dst_base + 2].copy_from_slice(&bytes);
        }

        self.set_zmm_data(zmm_dst, &dst[..dst_vl], dst_vl);

        // Always zero upper bits for this conversion
        if zmm_dst < 16 {
            if dst_vl <= 16 {
                self.regs.ymm_high[zmm_dst][0] = 0;
                self.regs.ymm_high[zmm_dst][1] = 0;
            }
            self.regs.zmm_high[zmm_dst] = [0; 4];
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// VCVTNE2PS2BF16 - Convert Two Packed Single-Precision to BF16
    fn execute_vcvtne2ps2bf16(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let evex = ctx.evex.unwrap();
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;

        let zmm_dst = if !evex.r { reg + 8 } else { reg };
        let zmm_dst = if !evex.r_prime { zmm_dst + 16 } else { zmm_dst } as usize;

        let zmm_src1 = (evex.vvvv ^ 0xF) as usize;

        let vl = match evex.ll {
            0 => 16,
            1 => 32,
            2 => 64,
            _ => 64,
        };

        let src2 = if is_memory {
            self.load_zmm_data(addr, vl)?
        } else {
            let zmm_src2 = if !evex.b { rm + 8 } else { rm } as usize;
            self.get_zmm_data(zmm_src2, vl)
        };

        let src1 = self.get_zmm_data(zmm_src1, vl);

        let num_floats = vl / 4;
        let mut dst = [0u8; 64];

        // First half from src2
        for i in 0..num_floats {
            let src_base = i * 4;
            let f = f32::from_le_bytes([
                src2[src_base],
                src2[src_base + 1],
                src2[src_base + 2],
                src2[src_base + 3],
            ]);
            let bf16 = f32_to_bf16(f);
            let dst_base = i * 2;
            let bytes = bf16.to_le_bytes();
            dst[dst_base..dst_base + 2].copy_from_slice(&bytes);
        }

        // Second half from src1
        for i in 0..num_floats {
            let src_base = i * 4;
            let f = f32::from_le_bytes([
                src1[src_base],
                src1[src_base + 1],
                src1[src_base + 2],
                src1[src_base + 3],
            ]);
            let bf16 = f32_to_bf16(f);
            let dst_base = (vl / 2) + i * 2;
            let bytes = bf16.to_le_bytes();
            dst[dst_base..dst_base + 2].copy_from_slice(&bytes);
        }

        self.set_zmm_data(zmm_dst, &dst[..vl], vl);

        if vl < 64 && zmm_dst < 16 {
            if vl <= 16 {
                self.regs.ymm_high[zmm_dst][0] = 0;
                self.regs.ymm_high[zmm_dst][1] = 0;
            }
            self.regs.zmm_high[zmm_dst] = [0; 4];
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    // ============================================================================
    // AVX10.2 VMPSADBW Instruction Implementation
    // ============================================================================

    /// VMPSADBW - Compute Multiple Packed Sums of Absolute Difference
    fn execute_vmpsadbw(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let evex = ctx.evex.unwrap();
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let imm8 = ctx.consume_u8()?;

        let zmm_dst = if !evex.r { reg + 8 } else { reg };
        let zmm_dst = if !evex.r_prime { zmm_dst + 16 } else { zmm_dst } as usize;

        let zmm_src1 = (evex.vvvv ^ 0xF) as usize;

        let vl = match evex.ll {
            0 => 16,
            1 => 32,
            2 => 64,
            _ => 64,
        };

        let src2 = if is_memory {
            self.load_zmm_data(addr, vl)?
        } else {
            let zmm_src2 = if !evex.b { rm + 8 } else { rm } as usize;
            self.get_zmm_data(zmm_src2, vl)
        };

        let src1 = self.get_zmm_data(zmm_src1, vl);
        let mut dst = [0u8; 64];

        // Process 128-bit lanes
        // imm8 encoding: bits [2:0] select src1 offset, bits [5:3] select src2 offset
        // Each lane uses the same block selection from imm8
        let num_lanes = vl / 16;
        let src1_blk = (imm8 & 0x3) as usize; // bits [1:0]
        let src2_blk = ((imm8 >> 2) & 0x3) as usize; // bits [3:2]

        for lane in 0..num_lanes {
            let lane_base = lane * 16;

            // Source block offsets within the lane
            let src1_offset = lane_base + (src1_blk * 4);

            // Calculate 8 SAD values per lane
            for i in 0..8 {
                let mut sad: u16 = 0;
                // src2 uses a sliding window of 4 consecutive bytes starting at blk2*4 + i
                let src2_start = lane_base + (src2_blk * 4) + i;
                for j in 0..4 {
                    let a = src1[src1_offset + j] as i16;
                    let b_idx = src2_start + j;
                    // Handle wrap-around within lane
                    let b = if b_idx < lane_base + 16 {
                        src2[b_idx] as i16
                    } else {
                        0 // Zero-pad beyond lane boundary
                    };
                    sad += (a - b).unsigned_abs();
                }
                let dst_offset = lane_base + i * 2;
                let bytes = sad.to_le_bytes();
                dst[dst_offset..dst_offset + 2].copy_from_slice(&bytes);
            }
        }

        self.set_zmm_data(zmm_dst, &dst[..vl], vl);

        if vl < 64 && zmm_dst < 16 {
            if vl <= 16 {
                self.regs.ymm_high[zmm_dst][0] = 0;
                self.regs.ymm_high[zmm_dst][1] = 0;
            }
            self.regs.zmm_high[zmm_dst] = [0; 4];
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    // ============================================================================
    // AVX10.2 VMINMAX Instruction Implementations
    // ============================================================================

    /// VMINMAXPS - Minimum/Maximum of Packed Single-Precision Floats
    fn execute_vminmax_ps(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let evex = ctx.evex.unwrap();
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let imm8 = ctx.consume_u8()?;

        let zmm_dst = if !evex.r { reg + 8 } else { reg };
        let zmm_dst = if !evex.r_prime { zmm_dst + 16 } else { zmm_dst } as usize;

        let zmm_src1 = (evex.vvvv ^ 0xF) as usize;

        let vl = match evex.ll {
            0 => 16,
            1 => 32,
            2 => 64,
            _ => 64,
        };

        let src2 = if is_memory {
            self.load_zmm_data(addr, vl)?
        } else {
            let zmm_src2 = if !evex.b { rm + 8 } else { rm } as usize;
            self.get_zmm_data(zmm_src2, vl)
        };

        let src1 = self.get_zmm_data(zmm_src1, vl);
        let mut dst = [0u8; 64];

        let num_elems = vl / 4;
        let is_min = (imm8 & 0x1) == 0;

        for i in 0..num_elems {
            let base = i * 4;
            let a =
                f32::from_le_bytes([src1[base], src1[base + 1], src1[base + 2], src1[base + 3]]);
            let b =
                f32::from_le_bytes([src2[base], src2[base + 1], src2[base + 2], src2[base + 3]]);

            let result = if is_min { a.min(b) } else { a.max(b) };
            let bytes = result.to_le_bytes();
            dst[base..base + 4].copy_from_slice(&bytes);
        }

        self.set_zmm_data(zmm_dst, &dst[..vl], vl);

        if vl < 64 && zmm_dst < 16 {
            if vl <= 16 {
                self.regs.ymm_high[zmm_dst][0] = 0;
                self.regs.ymm_high[zmm_dst][1] = 0;
            }
            self.regs.zmm_high[zmm_dst] = [0; 4];
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// VMINMAXPD - Minimum/Maximum of Packed Double-Precision Floats
    fn execute_vminmax_pd(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let evex = ctx.evex.unwrap();
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let imm8 = ctx.consume_u8()?;

        let zmm_dst = if !evex.r { reg + 8 } else { reg };
        let zmm_dst = if !evex.r_prime { zmm_dst + 16 } else { zmm_dst } as usize;

        let zmm_src1 = (evex.vvvv ^ 0xF) as usize;

        let vl = match evex.ll {
            0 => 16,
            1 => 32,
            2 => 64,
            _ => 64,
        };

        let src2 = if is_memory {
            self.load_zmm_data(addr, vl)?
        } else {
            let zmm_src2 = if !evex.b { rm + 8 } else { rm } as usize;
            self.get_zmm_data(zmm_src2, vl)
        };

        let src1 = self.get_zmm_data(zmm_src1, vl);
        let mut dst = [0u8; 64];

        let num_elems = vl / 8;
        let is_min = (imm8 & 0x1) == 0;

        for i in 0..num_elems {
            let base = i * 8;
            let a = f64::from_le_bytes([
                src1[base],
                src1[base + 1],
                src1[base + 2],
                src1[base + 3],
                src1[base + 4],
                src1[base + 5],
                src1[base + 6],
                src1[base + 7],
            ]);
            let b = f64::from_le_bytes([
                src2[base],
                src2[base + 1],
                src2[base + 2],
                src2[base + 3],
                src2[base + 4],
                src2[base + 5],
                src2[base + 6],
                src2[base + 7],
            ]);

            let result = if is_min { a.min(b) } else { a.max(b) };
            let bytes = result.to_le_bytes();
            dst[base..base + 8].copy_from_slice(&bytes);
        }

        self.set_zmm_data(zmm_dst, &dst[..vl], vl);

        if vl < 64 && zmm_dst < 16 {
            if vl <= 16 {
                self.regs.ymm_high[zmm_dst][0] = 0;
                self.regs.ymm_high[zmm_dst][1] = 0;
            }
            self.regs.zmm_high[zmm_dst] = [0; 4];
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// VMINMAXSS - Minimum/Maximum of Scalar Single-Precision Float
    fn execute_vminmax_ss(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let evex = ctx.evex.unwrap();
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let imm8 = ctx.consume_u8()?;

        let zmm_dst = if !evex.r { reg + 8 } else { reg };
        let zmm_dst = if !evex.r_prime { zmm_dst + 16 } else { zmm_dst } as usize;

        let zmm_src1 = (evex.vvvv ^ 0xF) as usize;

        let b_val = if is_memory {
            let bytes = self.load_zmm_data(addr, 4)?;
            f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
        } else {
            let zmm_src2 = if !evex.b { rm + 8 } else { rm } as usize;
            let src2 = self.get_zmm_data(zmm_src2, 16);
            f32::from_le_bytes([src2[0], src2[1], src2[2], src2[3]])
        };

        let src1 = self.get_zmm_data(zmm_src1, 16);
        let a_val = f32::from_le_bytes([src1[0], src1[1], src1[2], src1[3]]);

        let is_min = (imm8 & 0x1) == 0;
        let result = if is_min {
            a_val.min(b_val)
        } else {
            a_val.max(b_val)
        };

        // Copy src1 to dst, then overwrite lowest element
        let mut dst = self.get_zmm_data(zmm_src1, 16);
        let bytes = result.to_le_bytes();
        dst[0..4].copy_from_slice(&bytes);

        self.set_zmm_data(zmm_dst, &dst[..16], 16);

        // Zero upper bits
        if zmm_dst < 16 {
            self.regs.ymm_high[zmm_dst][0] = 0;
            self.regs.ymm_high[zmm_dst][1] = 0;
            self.regs.zmm_high[zmm_dst] = [0; 4];
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// VMINMAXSD - Minimum/Maximum of Scalar Double-Precision Float
    fn execute_vminmax_sd(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let evex = ctx.evex.unwrap();
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let imm8 = ctx.consume_u8()?;

        let zmm_dst = if !evex.r { reg + 8 } else { reg };
        let zmm_dst = if !evex.r_prime { zmm_dst + 16 } else { zmm_dst } as usize;

        let zmm_src1 = (evex.vvvv ^ 0xF) as usize;

        let b_val = if is_memory {
            let bytes = self.load_zmm_data(addr, 8)?;
            f64::from_le_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ])
        } else {
            let zmm_src2 = if !evex.b { rm + 8 } else { rm } as usize;
            let src2 = self.get_zmm_data(zmm_src2, 16);
            f64::from_le_bytes([
                src2[0], src2[1], src2[2], src2[3], src2[4], src2[5], src2[6], src2[7],
            ])
        };

        let src1 = self.get_zmm_data(zmm_src1, 16);
        let a_val = f64::from_le_bytes([
            src1[0], src1[1], src1[2], src1[3], src1[4], src1[5], src1[6], src1[7],
        ]);

        let is_min = (imm8 & 0x1) == 0;
        let result = if is_min {
            a_val.min(b_val)
        } else {
            a_val.max(b_val)
        };

        // Copy src1 to dst, then overwrite lowest element
        let mut dst = self.get_zmm_data(zmm_src1, 16);
        let bytes = result.to_le_bytes();
        dst[0..8].copy_from_slice(&bytes);

        self.set_zmm_data(zmm_dst, &dst[..16], 16);

        // Zero upper bits
        if zmm_dst < 16 {
            self.regs.ymm_high[zmm_dst][0] = 0;
            self.regs.ymm_high[zmm_dst][1] = 0;
            self.regs.zmm_high[zmm_dst] = [0; 4];
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    // ============================================================================
    // AVX10.2 Saturation Conversion Instruction Implementations
    // ============================================================================

    /// VCVTTPS2IBS - Convert with Truncation Packed Single to Signed Byte with Saturation
    fn execute_vcvttps2ibs(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let evex = ctx.evex.unwrap();
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;

        let zmm_dst = if !evex.r { reg + 8 } else { reg };
        let zmm_dst = if !evex.r_prime { zmm_dst + 16 } else { zmm_dst } as usize;

        let vl = match evex.ll {
            0 => 16,
            1 => 32,
            2 => 64,
            _ => 64,
        };

        let src = if is_memory {
            self.load_zmm_data(addr, vl)?
        } else {
            let zmm_src = if !evex.b { rm + 8 } else { rm } as usize;
            self.get_zmm_data(zmm_src, vl)
        };

        let num_floats = vl / 4;
        let dst_vl = vl / 4; // Output is 1/4 the size
        let mut dst = [0u8; 64];

        for i in 0..num_floats {
            let src_base = i * 4;
            let f = f32::from_le_bytes([
                src[src_base],
                src[src_base + 1],
                src[src_base + 2],
                src[src_base + 3],
            ]);
            // Truncate and saturate to i8
            let val = f.trunc() as i32;
            let saturated = val.clamp(i8::MIN as i32, i8::MAX as i32) as i8;
            dst[i] = saturated as u8;
        }

        self.set_zmm_data(zmm_dst, &dst[..dst_vl], dst_vl);

        // Zero upper bits
        if zmm_dst < 16 {
            if dst_vl <= 16 {
                self.regs.ymm_high[zmm_dst][0] = 0;
                self.regs.ymm_high[zmm_dst][1] = 0;
            }
            self.regs.zmm_high[zmm_dst] = [0; 4];
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// VCVTTPS2IUBS - Convert with Truncation Packed Single to Unsigned Byte with Saturation
    fn execute_vcvttps2iubs(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let evex = ctx.evex.unwrap();
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;

        let zmm_dst = if !evex.r { reg + 8 } else { reg };
        let zmm_dst = if !evex.r_prime { zmm_dst + 16 } else { zmm_dst } as usize;

        let vl = match evex.ll {
            0 => 16,
            1 => 32,
            2 => 64,
            _ => 64,
        };

        let src = if is_memory {
            self.load_zmm_data(addr, vl)?
        } else {
            let zmm_src = if !evex.b { rm + 8 } else { rm } as usize;
            self.get_zmm_data(zmm_src, vl)
        };

        let num_floats = vl / 4;
        let dst_vl = vl / 4;
        let mut dst = [0u8; 64];

        for i in 0..num_floats {
            let src_base = i * 4;
            let f = f32::from_le_bytes([
                src[src_base],
                src[src_base + 1],
                src[src_base + 2],
                src[src_base + 3],
            ]);
            // Truncate and saturate to u8
            let val = f.trunc() as i32;
            let saturated = val.clamp(0, u8::MAX as i32) as u8;
            dst[i] = saturated;
        }

        self.set_zmm_data(zmm_dst, &dst[..dst_vl], dst_vl);

        if zmm_dst < 16 {
            if dst_vl <= 16 {
                self.regs.ymm_high[zmm_dst][0] = 0;
                self.regs.ymm_high[zmm_dst][1] = 0;
            }
            self.regs.zmm_high[zmm_dst] = [0; 4];
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// VCVTTPD2QQS - Convert with Truncation Packed Double to Signed Qword with Saturation
    fn execute_vcvttpd2qqs(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let evex = ctx.evex.unwrap();
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;

        let zmm_dst = if !evex.r { reg + 8 } else { reg };
        let zmm_dst = if !evex.r_prime { zmm_dst + 16 } else { zmm_dst } as usize;

        let vl = match evex.ll {
            0 => 16,
            1 => 32,
            2 => 64,
            _ => 64,
        };

        let src = if is_memory {
            self.load_zmm_data(addr, vl)?
        } else {
            let zmm_src = if !evex.b { rm + 8 } else { rm } as usize;
            self.get_zmm_data(zmm_src, vl)
        };

        let num_doubles = vl / 8;
        let mut dst = [0u8; 64];

        for i in 0..num_doubles {
            let base = i * 8;
            let f = f64::from_le_bytes([
                src[base],
                src[base + 1],
                src[base + 2],
                src[base + 3],
                src[base + 4],
                src[base + 5],
                src[base + 6],
                src[base + 7],
            ]);
            // Truncate and saturate to i64
            let val = f.trunc();
            let saturated = if val >= i64::MAX as f64 {
                i64::MAX
            } else if val <= i64::MIN as f64 {
                i64::MIN
            } else {
                val as i64
            };
            let bytes = saturated.to_le_bytes();
            dst[base..base + 8].copy_from_slice(&bytes);
        }

        self.set_zmm_data(zmm_dst, &dst[..vl], vl);

        if vl < 64 && zmm_dst < 16 {
            if vl <= 16 {
                self.regs.ymm_high[zmm_dst][0] = 0;
                self.regs.ymm_high[zmm_dst][1] = 0;
            }
            self.regs.zmm_high[zmm_dst] = [0; 4];
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// VCVTTPD2UQQS - Convert with Truncation Packed Double to Unsigned Qword with Saturation
    fn execute_vcvttpd2uqqs(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let evex = ctx.evex.unwrap();
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;

        let zmm_dst = if !evex.r { reg + 8 } else { reg };
        let zmm_dst = if !evex.r_prime { zmm_dst + 16 } else { zmm_dst } as usize;

        let vl = match evex.ll {
            0 => 16,
            1 => 32,
            2 => 64,
            _ => 64,
        };

        let src = if is_memory {
            self.load_zmm_data(addr, vl)?
        } else {
            let zmm_src = if !evex.b { rm + 8 } else { rm } as usize;
            self.get_zmm_data(zmm_src, vl)
        };

        let num_doubles = vl / 8;
        let mut dst = [0u8; 64];

        for i in 0..num_doubles {
            let base = i * 8;
            let f = f64::from_le_bytes([
                src[base],
                src[base + 1],
                src[base + 2],
                src[base + 3],
                src[base + 4],
                src[base + 5],
                src[base + 6],
                src[base + 7],
            ]);
            // Truncate and saturate to u64
            let val = f.trunc();
            let saturated = if val >= u64::MAX as f64 {
                u64::MAX
            } else if val < 0.0 {
                0
            } else {
                val as u64
            };
            let bytes = saturated.to_le_bytes();
            dst[base..base + 8].copy_from_slice(&bytes);
        }

        self.set_zmm_data(zmm_dst, &dst[..vl], vl);

        if vl < 64 && zmm_dst < 16 {
            if vl <= 16 {
                self.regs.ymm_high[zmm_dst][0] = 0;
                self.regs.ymm_high[zmm_dst][1] = 0;
            }
            self.regs.zmm_high[zmm_dst] = [0; 4];
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    // ============================================================================
    // AVX10.2 Media Acceleration Instruction Implementations
    // ============================================================================

    /// VPDPBSSD/VPDPBSSDS - Multiply and Add Signed Byte Integers
    fn execute_vpdpbssd(
        &mut self,
        ctx: &mut InsnContext,
        saturate: bool,
    ) -> Result<Option<VcpuExit>> {
        let evex = ctx.evex.unwrap();
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;

        let zmm_dst = if !evex.r { reg + 8 } else { reg };
        let zmm_dst = if !evex.r_prime { zmm_dst + 16 } else { zmm_dst } as usize;

        let zmm_src1 = (evex.vvvv ^ 0xF) as usize;

        let vl = match evex.ll {
            0 => 16,
            1 => 32,
            2 => 64,
            _ => 64,
        };

        let num_dwords = vl / 4;

        let src2 = if is_memory {
            self.load_zmm_data(addr, vl)?
        } else {
            let zmm_src2 = if !evex.b { rm + 8 } else { rm } as usize;
            self.get_zmm_data(zmm_src2, vl)
        };

        let src1 = self.get_zmm_data(zmm_src1, vl);
        let mut dst = self.get_zmm_data(zmm_dst, vl);

        for i in 0..num_dwords {
            let base = i * 4;
            let mut sum =
                i32::from_le_bytes([dst[base], dst[base + 1], dst[base + 2], dst[base + 3]]) as i64;

            for j in 0..4 {
                let a = src1[base + j] as i8 as i32; // signed byte
                let b = src2[base + j] as i8 as i32; // signed byte
                sum += (a * b) as i64;
            }

            let result = if saturate {
                sum.clamp(i32::MIN as i64, i32::MAX as i64) as i32
            } else {
                sum as i32
            };

            let bytes = result.to_le_bytes();
            dst[base..base + 4].copy_from_slice(&bytes);
        }

        self.set_zmm_data(zmm_dst, &dst[..vl], vl);

        if vl < 64 && zmm_dst < 16 {
            if vl <= 16 {
                self.regs.ymm_high[zmm_dst][0] = 0;
                self.regs.ymm_high[zmm_dst][1] = 0;
            }
            self.regs.zmm_high[zmm_dst] = [0; 4];
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// VPDPBSUD/VPDPBSUDS - Multiply and Add Signed/Unsigned Byte Integers
    fn execute_vpdpbsud(
        &mut self,
        ctx: &mut InsnContext,
        saturate: bool,
    ) -> Result<Option<VcpuExit>> {
        let evex = ctx.evex.unwrap();
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;

        let zmm_dst = if !evex.r { reg + 8 } else { reg };
        let zmm_dst = if !evex.r_prime { zmm_dst + 16 } else { zmm_dst } as usize;

        let zmm_src1 = (evex.vvvv ^ 0xF) as usize;

        let vl = match evex.ll {
            0 => 16,
            1 => 32,
            2 => 64,
            _ => 64,
        };

        let num_dwords = vl / 4;

        let src2 = if is_memory {
            self.load_zmm_data(addr, vl)?
        } else {
            let zmm_src2 = if !evex.b { rm + 8 } else { rm } as usize;
            self.get_zmm_data(zmm_src2, vl)
        };

        let src1 = self.get_zmm_data(zmm_src1, vl);
        let mut dst = self.get_zmm_data(zmm_dst, vl);

        for i in 0..num_dwords {
            let base = i * 4;
            let mut sum =
                i32::from_le_bytes([dst[base], dst[base + 1], dst[base + 2], dst[base + 3]]) as i64;

            for j in 0..4 {
                let a = src1[base + j] as i8 as i32; // signed byte
                let b = src2[base + j] as u8 as i32; // unsigned byte
                sum += (a * b) as i64;
            }

            let result = if saturate {
                sum.clamp(i32::MIN as i64, i32::MAX as i64) as i32
            } else {
                sum as i32
            };

            let bytes = result.to_le_bytes();
            dst[base..base + 4].copy_from_slice(&bytes);
        }

        self.set_zmm_data(zmm_dst, &dst[..vl], vl);

        if vl < 64 && zmm_dst < 16 {
            if vl <= 16 {
                self.regs.ymm_high[zmm_dst][0] = 0;
                self.regs.ymm_high[zmm_dst][1] = 0;
            }
            self.regs.zmm_high[zmm_dst] = [0; 4];
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// VPDPBUUD/VPDPBUUDS - Multiply and Add Unsigned Byte Integers
    fn execute_vpdpbuud(
        &mut self,
        ctx: &mut InsnContext,
        saturate: bool,
    ) -> Result<Option<VcpuExit>> {
        let evex = ctx.evex.unwrap();
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;

        let zmm_dst = if !evex.r { reg + 8 } else { reg };
        let zmm_dst = if !evex.r_prime { zmm_dst + 16 } else { zmm_dst } as usize;

        let zmm_src1 = (evex.vvvv ^ 0xF) as usize;

        let vl = match evex.ll {
            0 => 16,
            1 => 32,
            2 => 64,
            _ => 64,
        };

        let num_dwords = vl / 4;

        let src2 = if is_memory {
            self.load_zmm_data(addr, vl)?
        } else {
            let zmm_src2 = if !evex.b { rm + 8 } else { rm } as usize;
            self.get_zmm_data(zmm_src2, vl)
        };

        let src1 = self.get_zmm_data(zmm_src1, vl);
        let mut dst = self.get_zmm_data(zmm_dst, vl);

        for i in 0..num_dwords {
            let base = i * 4;
            let mut sum =
                u32::from_le_bytes([dst[base], dst[base + 1], dst[base + 2], dst[base + 3]]) as u64;

            for j in 0..4 {
                let a = src1[base + j] as u32; // unsigned byte
                let b = src2[base + j] as u32; // unsigned byte
                sum += (a * b) as u64;
            }

            let result = if saturate {
                sum.min(u32::MAX as u64) as u32
            } else {
                sum as u32
            };

            let bytes = result.to_le_bytes();
            dst[base..base + 4].copy_from_slice(&bytes);
        }

        self.set_zmm_data(zmm_dst, &dst[..vl], vl);

        if vl < 64 && zmm_dst < 16 {
            if vl <= 16 {
                self.regs.ymm_high[zmm_dst][0] = 0;
                self.regs.ymm_high[zmm_dst][1] = 0;
            }
            self.regs.zmm_high[zmm_dst] = [0; 4];
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// VPDPWSUD/VPDPWSUDS - Multiply and Add Signed/Unsigned Word Integers
    fn execute_vpdpwsud(
        &mut self,
        ctx: &mut InsnContext,
        saturate: bool,
    ) -> Result<Option<VcpuExit>> {
        let evex = ctx.evex.unwrap();
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;

        let zmm_dst = if !evex.r { reg + 8 } else { reg };
        let zmm_dst = if !evex.r_prime { zmm_dst + 16 } else { zmm_dst } as usize;

        let zmm_src1 = (evex.vvvv ^ 0xF) as usize;

        let vl = match evex.ll {
            0 => 16,
            1 => 32,
            2 => 64,
            _ => 64,
        };

        let num_dwords = vl / 4;

        let src2 = if is_memory {
            self.load_zmm_data(addr, vl)?
        } else {
            let zmm_src2 = if !evex.b { rm + 8 } else { rm } as usize;
            self.get_zmm_data(zmm_src2, vl)
        };

        let src1 = self.get_zmm_data(zmm_src1, vl);
        let mut dst = self.get_zmm_data(zmm_dst, vl);

        for i in 0..num_dwords {
            let base = i * 4;
            let mut sum =
                i32::from_le_bytes([dst[base], dst[base + 1], dst[base + 2], dst[base + 3]]) as i64;

            // Two pairs of words per dword
            let a0 = i16::from_le_bytes([src1[base], src1[base + 1]]) as i32; // signed
            let b0 = u16::from_le_bytes([src2[base], src2[base + 1]]) as i32; // unsigned
            let a1 = i16::from_le_bytes([src1[base + 2], src1[base + 3]]) as i32; // signed
            let b1 = u16::from_le_bytes([src2[base + 2], src2[base + 3]]) as i32; // unsigned

            sum += (a0 * b0 + a1 * b1) as i64;

            let result = if saturate {
                sum.clamp(i32::MIN as i64, i32::MAX as i64) as i32
            } else {
                sum as i32
            };

            let bytes = result.to_le_bytes();
            dst[base..base + 4].copy_from_slice(&bytes);
        }

        self.set_zmm_data(zmm_dst, &dst[..vl], vl);

        if vl < 64 && zmm_dst < 16 {
            if vl <= 16 {
                self.regs.ymm_high[zmm_dst][0] = 0;
                self.regs.ymm_high[zmm_dst][1] = 0;
            }
            self.regs.zmm_high[zmm_dst] = [0; 4];
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// VPDPWUSD/VPDPWUSDS - Multiply and Add Unsigned/Signed Word Integers
    fn execute_vpdpwusd(
        &mut self,
        ctx: &mut InsnContext,
        saturate: bool,
    ) -> Result<Option<VcpuExit>> {
        let evex = ctx.evex.unwrap();
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;

        let zmm_dst = if !evex.r { reg + 8 } else { reg };
        let zmm_dst = if !evex.r_prime { zmm_dst + 16 } else { zmm_dst } as usize;

        let zmm_src1 = (evex.vvvv ^ 0xF) as usize;

        let vl = match evex.ll {
            0 => 16,
            1 => 32,
            2 => 64,
            _ => 64,
        };

        let num_dwords = vl / 4;

        let src2 = if is_memory {
            self.load_zmm_data(addr, vl)?
        } else {
            let zmm_src2 = if !evex.b { rm + 8 } else { rm } as usize;
            self.get_zmm_data(zmm_src2, vl)
        };

        let src1 = self.get_zmm_data(zmm_src1, vl);
        let mut dst = self.get_zmm_data(zmm_dst, vl);

        for i in 0..num_dwords {
            let base = i * 4;
            let mut sum =
                i32::from_le_bytes([dst[base], dst[base + 1], dst[base + 2], dst[base + 3]]) as i64;

            // Two pairs of words per dword
            let a0 = u16::from_le_bytes([src1[base], src1[base + 1]]) as i32; // unsigned
            let b0 = i16::from_le_bytes([src2[base], src2[base + 1]]) as i32; // signed
            let a1 = u16::from_le_bytes([src1[base + 2], src1[base + 3]]) as i32; // unsigned
            let b1 = i16::from_le_bytes([src2[base + 2], src2[base + 3]]) as i32; // signed

            sum += (a0 * b0 + a1 * b1) as i64;

            let result = if saturate {
                sum.clamp(i32::MIN as i64, i32::MAX as i64) as i32
            } else {
                sum as i32
            };

            let bytes = result.to_le_bytes();
            dst[base..base + 4].copy_from_slice(&bytes);
        }

        self.set_zmm_data(zmm_dst, &dst[..vl], vl);

        if vl < 64 && zmm_dst < 16 {
            if vl <= 16 {
                self.regs.ymm_high[zmm_dst][0] = 0;
                self.regs.ymm_high[zmm_dst][1] = 0;
            }
            self.regs.zmm_high[zmm_dst] = [0; 4];
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// VPDPWUUD/VPDPWUUDS - Multiply and Add Unsigned Word Integers
    fn execute_vpdpwuud(
        &mut self,
        ctx: &mut InsnContext,
        saturate: bool,
    ) -> Result<Option<VcpuExit>> {
        let evex = ctx.evex.unwrap();
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;

        let zmm_dst = if !evex.r { reg + 8 } else { reg };
        let zmm_dst = if !evex.r_prime { zmm_dst + 16 } else { zmm_dst } as usize;

        let zmm_src1 = (evex.vvvv ^ 0xF) as usize;

        let vl = match evex.ll {
            0 => 16,
            1 => 32,
            2 => 64,
            _ => 64,
        };

        let num_dwords = vl / 4;

        let src2 = if is_memory {
            self.load_zmm_data(addr, vl)?
        } else {
            let zmm_src2 = if !evex.b { rm + 8 } else { rm } as usize;
            self.get_zmm_data(zmm_src2, vl)
        };

        let src1 = self.get_zmm_data(zmm_src1, vl);
        let mut dst = self.get_zmm_data(zmm_dst, vl);

        for i in 0..num_dwords {
            let base = i * 4;
            let mut sum =
                u32::from_le_bytes([dst[base], dst[base + 1], dst[base + 2], dst[base + 3]]) as u64;

            // Two pairs of words per dword
            let a0 = u16::from_le_bytes([src1[base], src1[base + 1]]) as u32; // unsigned
            let b0 = u16::from_le_bytes([src2[base], src2[base + 1]]) as u32; // unsigned
            let a1 = u16::from_le_bytes([src1[base + 2], src1[base + 3]]) as u32; // unsigned
            let b1 = u16::from_le_bytes([src2[base + 2], src2[base + 3]]) as u32; // unsigned

            sum += (a0 * b0 + a1 * b1) as u64;

            let result = if saturate {
                sum.min(u32::MAX as u64) as u32
            } else {
                sum as u32
            };

            let bytes = result.to_le_bytes();
            dst[base..base + 4].copy_from_slice(&bytes);
        }

        self.set_zmm_data(zmm_dst, &dst[..vl], vl);

        if vl < 64 && zmm_dst < 16 {
            if vl <= 16 {
                self.regs.ymm_high[zmm_dst][0] = 0;
                self.regs.ymm_high[zmm_dst][1] = 0;
            }
            self.regs.zmm_high[zmm_dst] = [0; 4];
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    // ============================================================================
    // APX EVEX-MAP4 Instruction Implementations (GPR Instructions)
    // ============================================================================

    /// EVEX MAP4 opcode map (mm=4) - APX GPR instructions
    /// APX extends EVEX encoding to support:
    /// - EGPR (R16-R31) via B4, X4, R4 bits
    /// - NDD (New Data Destination) - 3-operand forms where vvvv is destination
    /// - NF (No Flags) - arithmetic without updating RFLAGS
    fn execute_evex_map4_apx(
        &mut self,
        ctx: &mut InsnContext,
        opcode: u8,
    ) -> Result<Option<VcpuExit>> {
        let evex = ctx
            .evex
            .ok_or_else(|| Error::Emulator("EVEX context missing".to_string()))?;

        // APX uses ND (New Data Destination) for 3-operand forms
        // and NF (No Flags) for flag-suppressing variants
        let ndd = evex.nd; // 3-operand form
        let nf = evex.nf; // No flags update

        match opcode {
            // ADD variants (0x00-0x03)
            0x00 | 0x01 | 0x02 | 0x03 => self.execute_apx_alu(ctx, opcode, ndd, nf, ApxAluOp::Add),

            // OR variants (0x08-0x0B)
            0x08 | 0x09 | 0x0A | 0x0B => self.execute_apx_alu(ctx, opcode, ndd, nf, ApxAluOp::Or),

            // ADC variants (0x10-0x13)
            0x10 | 0x11 | 0x12 | 0x13 => self.execute_apx_alu(ctx, opcode, ndd, nf, ApxAluOp::Adc),

            // SBB variants (0x18-0x1B)
            0x18 | 0x19 | 0x1A | 0x1B => self.execute_apx_alu(ctx, opcode, ndd, nf, ApxAluOp::Sbb),

            // AND variants (0x20-0x23)
            0x20 | 0x21 | 0x22 | 0x23 => self.execute_apx_alu(ctx, opcode, ndd, nf, ApxAluOp::And),

            // SUB variants (0x28-0x2B)
            0x28 | 0x29 | 0x2A | 0x2B => self.execute_apx_alu(ctx, opcode, ndd, nf, ApxAluOp::Sub),

            // XOR variants (0x30-0x33)
            0x30 | 0x31 | 0x32 | 0x33 => self.execute_apx_alu(ctx, opcode, ndd, nf, ApxAluOp::Xor),

            // CCMP variants (0x38-0x3B)
            0x38 | 0x39 | 0x3A | 0x3B => self.execute_apx_ccmp(ctx, opcode),

            // MAP4 conditionals: SETZUcc, EVEX SETcc, CMOV_ND, and CFCMOV.
            0x40..=0x4F => self.execute_apx_conditional_map4(ctx, opcode & 0x0F),

            // CTEST variants (0x84-0x85)
            0x84 | 0x85 => self.execute_apx_ctest(ctx, opcode),

            // MOVBE reg, reg (0x61)
            0x61 => self.execute_apx_movbe(ctx, ndd, nf),

            // POPCNT with NF shares MAP4 opcode 0x88 with MOV r/m8,r8.
            0x88 if nf => self.execute_apx_count(ctx, opcode, ndd, nf),

            // MOV variants (0x88-0x8B)
            0x88 | 0x89 | 0x8A | 0x8B => self.execute_apx_mov(ctx, opcode),

            // LEA (0x8D)
            0x8D => self.execute_apx_lea(ctx),

            // POP2 (0x8F)
            0x8F => self.execute_apx_pop2(ctx),

            // IMUL (0x69, 0x6B)
            0x69 => self.execute_apx_imul_imm(ctx, ndd, nf, true),
            0x6B => self.execute_apx_imul_imm(ctx, ndd, nf, false),
            0xAF => self.execute_apx_imul(ctx, ndd, nf),

            // SHLD/SHRD double shifts (0x24, 0x2C imm8; 0xA5, 0xAD CL)
            0x24 | 0x2C | 0xA5 | 0xAD => self.execute_apx_double_shift(ctx, opcode, ndd, nf),

            // Group 1 immediate ALU operations (0x80, 0x81, 0x82, 0x83 /0..7)
            0x80 | 0x81 | 0x82 | 0x83 => self.execute_apx_group1_imm(ctx, opcode, ndd, nf),

            // Shift variants (0xC0, 0xC1, 0xD0-0xD3)
            0xC0 | 0xC1 => self.execute_apx_shift_imm(ctx, opcode, ndd, nf),
            0xD0 | 0xD1 | 0xD2 | 0xD3 => self.execute_apx_shift_cl(ctx, opcode, ndd, nf),

            // TZCNT/LZCNT with NF
            0xF4 | 0xF5 => self.execute_apx_count(ctx, opcode, ndd, nf),

            // Group 3 NOT/NEG (0xF6, 0xF7 /2,/3)
            0xF6 | 0xF7 => self.execute_apx_group3(ctx, opcode, ndd, nf),

            // INC/DEC (0xFE, 0xFF /0,/1) and PUSH2 (0xFF /6)
            0xFE | 0xFF => self.execute_apx_group_ff(ctx, opcode, ndd, nf),

            _ => Err(Error::Emulator(format!(
                "Unimplemented APX MAP4 opcode {:#x} at RIP={:#x}",
                opcode, self.regs.rip
            ))),
        }
    }

    /// Generic APX ALU operation with NDD and NF support
    fn execute_apx_alu(
        &mut self,
        ctx: &mut InsnContext,
        opcode: u8,
        ndd: bool,
        nf: bool,
        alu_op: ApxAluOp,
    ) -> Result<Option<VcpuExit>> {
        // Determine operand size from opcode and EVEX.W
        let is_byte = (opcode & 0x01) == 0;
        let op_size = if is_byte {
            1
        } else if ctx.evex_w() {
            8
        } else {
            4
        };

        // Determine direction (reg->rm or rm->reg)
        let reg_is_src = (opcode & 0x02) == 0;

        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;

        // Apply EVEX register extensions for EGPR (R16-R31)
        let reg = reg | ctx.evex_dest_reg();
        let rm = if is_memory {
            rm
        } else {
            rm | ctx.evex_rm_reg()
        };

        // Get source values
        let (src1, src2) = if reg_is_src {
            let r_val = self.get_reg(reg, op_size);
            let rm_val = if is_memory {
                self.read_mem(addr, op_size)?
            } else {
                self.get_reg(rm, op_size)
            };
            (rm_val, r_val)
        } else {
            let r_val = self.get_reg(reg, op_size);
            let rm_val = if is_memory {
                self.read_mem(addr, op_size)?
            } else {
                self.get_reg(rm, op_size)
            };
            (r_val, rm_val)
        };

        // Perform ALU operation
        let cf_in = (self.regs.rflags & 0x001) != 0;
        let cf_val = u64::from(cf_in);
        let result = match alu_op {
            ApxAluOp::Add => src1.wrapping_add(src2),
            ApxAluOp::Adc => src1.wrapping_add(src2).wrapping_add(cf_val),
            ApxAluOp::Or => src1 | src2,
            ApxAluOp::And => src1 & src2,
            ApxAluOp::Sub => src1.wrapping_sub(src2),
            ApxAluOp::Sbb => src1.wrapping_sub(src2).wrapping_sub(cf_val),
            ApxAluOp::Xor => src1 ^ src2,
        };

        // Determine destination
        if ndd {
            // NDD mode: destination is from vvvv field
            let dest = ctx.evex_vvvv();
            self.set_reg(dest, result, op_size);
        } else if reg_is_src {
            // Destination is r/m
            if is_memory {
                self.write_mem(addr, result, op_size)?;
            } else {
                self.set_reg(rm, result, op_size);
            }
        } else {
            // Destination is reg
            self.set_reg(reg, result, op_size);
        }

        // Update flags unless NF is set
        if !nf {
            match alu_op {
                ApxAluOp::Adc => {
                    flags::update_flags_adc(
                        &mut self.regs.rflags,
                        src1,
                        src2,
                        cf_in,
                        result,
                        op_size,
                    );
                }
                ApxAluOp::Sbb => {
                    flags::update_flags_sbb(
                        &mut self.regs.rflags,
                        src1,
                        src2,
                        cf_in,
                        result,
                        op_size,
                    );
                }
                _ => self.update_flags_alu(result, src1, src2, op_size, alu_op),
            }
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    fn apx_ccmp_condition_and_default_flags(ctx: &InsnContext) -> Result<(u8, u8)> {
        let evex = ctx
            .evex
            .ok_or_else(|| Error::Emulator("EVEX context missing".to_string()))?;
        let cc = ((evex.v_prime as u8) << 3) | evex.aaa;
        let dfv = evex.vvvv;
        Ok((cc, dfv))
    }

    fn apply_apx_ccmp_default_flags(&mut self, dfv: u8) {
        let mut flags = self.regs.rflags & !0x8D5; // CF, PF, AF, ZF, SF, OF
        if dfv & 0x1 != 0 {
            flags |= 0x001; // CF
        }
        if dfv & 0x2 != 0 {
            flags |= 0x040; // ZF
        }
        if dfv & 0x4 != 0 {
            flags |= 0x080; // SF
        }
        if dfv & 0x8 != 0 {
            flags |= 0x800; // OF
        }
        self.regs.rflags = flags;
    }

    /// APX CCMP operation.
    fn execute_apx_ccmp(&mut self, ctx: &mut InsnContext, opcode: u8) -> Result<Option<VcpuExit>> {
        let is_byte = (opcode & 0x01) == 0;
        let op_size = if is_byte {
            1
        } else if ctx.evex_w() {
            8
        } else {
            4
        };
        let reg_is_src = (opcode & 0x02) == 0;
        let (cc, dfv) = Self::apx_ccmp_condition_and_default_flags(ctx)?;

        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let reg = reg | ctx.evex_dest_reg();
        let rm = if is_memory {
            rm
        } else {
            rm | ctx.evex_rm_reg()
        };

        if self.check_condition(cc) {
            let (src1, src2) = if reg_is_src {
                let r_val = self.get_reg(reg, op_size);
                let rm_val = if is_memory {
                    self.read_mem(addr, op_size)?
                } else {
                    self.get_reg(rm, op_size)
                };
                (rm_val, r_val)
            } else {
                let r_val = self.get_reg(reg, op_size);
                let rm_val = if is_memory {
                    self.read_mem(addr, op_size)?
                } else {
                    self.get_reg(rm, op_size)
                };
                (r_val, rm_val)
            };

            let result = src1.wrapping_sub(src2);
            self.update_flags_alu(result, src1, src2, op_size, ApxAluOp::Sub);
        } else {
            self.apply_apx_ccmp_default_flags(dfv);
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// APX CTEST operation.
    fn execute_apx_ctest(&mut self, ctx: &mut InsnContext, opcode: u8) -> Result<Option<VcpuExit>> {
        let is_byte = opcode == 0x84;
        let op_size = if is_byte {
            1
        } else if ctx.evex_w() {
            8
        } else {
            4
        };
        let (cc, dfv) = Self::apx_ccmp_condition_and_default_flags(ctx)?;

        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let reg = reg | ctx.evex_dest_reg();
        let rm = if is_memory {
            rm
        } else {
            rm | ctx.evex_rm_reg()
        };

        if self.check_condition(cc) {
            let src1 = self.get_reg(reg, op_size);
            let src2 = if is_memory {
                self.read_mem(addr, op_size)?
            } else {
                self.get_reg(rm, op_size)
            };

            let result = src1 & src2;
            self.update_flags_alu(result, src1, src2, op_size, ApxAluOp::And);
        } else {
            self.apply_apx_ccmp_default_flags(dfv);
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// APX SETZUcc operation.
    fn execute_apx_setzucc(&mut self, ctx: &mut InsnContext, cc: u8) -> Result<Option<VcpuExit>> {
        let (_, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let value = if self.check_condition(cc) { 1 } else { 0 };

        if is_memory {
            self.write_mem(addr, value, 1)?;
        } else {
            let rm = rm | ctx.evex_rm_reg();
            self.set_reg(rm, value, 8);
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    fn execute_apx_evex_setcc(
        &mut self,
        ctx: &mut InsnContext,
        cc: u8,
    ) -> Result<Option<VcpuExit>> {
        let (_, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let value = if self.check_condition(cc) { 1 } else { 0 };

        if is_memory {
            self.write_mem(addr, value, 1)?;
        } else {
            let rm = rm | ctx.evex_rm_reg();
            self.set_reg(rm, value, 1);
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    fn execute_apx_conditional_map4(
        &mut self,
        ctx: &mut InsnContext,
        cc: u8,
    ) -> Result<Option<VcpuExit>> {
        let evex = ctx
            .evex
            .ok_or_else(|| Error::Emulator("EVEX context missing".to_string()))?;

        if evex.pp == 0x02 {
            return self.inject_invalid_opcode();
        }

        if evex.pp == 0x03 && !evex.nf {
            if evex.nd {
                self.execute_apx_setzucc(ctx, cc)
            } else {
                self.execute_apx_evex_setcc(ctx, cc)
            }
        } else {
            self.execute_apx_cmovcc(ctx, cc, evex.nd, evex.nf)
        }
    }

    fn execute_apx_cmovcc(
        &mut self,
        ctx: &mut InsnContext,
        cc: u8,
        ndd: bool,
        nf: bool,
    ) -> Result<Option<VcpuExit>> {
        let op_size = Self::apx_scalar_op_size(ctx);
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let reg = reg | ctx.evex_dest_reg();
        let rm = if is_memory {
            rm
        } else {
            rm | ctx.evex_rm_reg()
        };

        if ndd {
            let dst = ctx.evex_vvvv();
            let src1 = self.get_reg(reg, op_size);

            if nf {
                if self.check_condition(cc) {
                    let src2 = if is_memory {
                        self.read_mem(addr, op_size)?
                    } else {
                        self.get_reg(rm, op_size)
                    };
                    self.set_reg(dst, src2, op_size);
                } else {
                    self.set_reg(dst, src1, op_size);
                }
            } else {
                let src2 = if is_memory {
                    self.read_mem(addr, op_size)?
                } else {
                    self.get_reg(rm, op_size)
                };
                let result = if self.check_condition(cc) { src2 } else { src1 };
                self.set_reg(dst, result, op_size);
            }
        } else if nf {
            let src = self.get_reg(reg, op_size);

            if is_memory {
                if self.check_condition(cc) {
                    self.write_mem(addr, src, op_size)?;
                }
            } else {
                let result = if self.check_condition(cc) { src } else { 0 };
                self.set_reg(rm, result, op_size);
            }
        } else {
            let dst = reg;

            if self.check_condition(cc) {
                let src = if is_memory {
                    self.read_mem(addr, op_size)?
                } else {
                    self.get_reg(rm, op_size)
                };
                self.set_reg(dst, src, op_size);
            } else {
                self.set_reg(dst, 0, op_size);
            }
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// APX MOV operation
    fn execute_apx_mov(&mut self, ctx: &mut InsnContext, opcode: u8) -> Result<Option<VcpuExit>> {
        let is_byte = (opcode & 0x01) == 0;
        let op_size = if is_byte {
            1
        } else if ctx.evex_w() {
            8
        } else {
            4
        };
        let reg_is_src = (opcode & 0x02) == 0;

        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let reg = reg | ctx.evex_dest_reg();
        let rm = if is_memory {
            rm
        } else {
            rm | ctx.evex_rm_reg()
        };

        if reg_is_src {
            // MOV r/m, r
            let value = self.get_reg(reg, op_size);
            if is_memory {
                self.write_mem(addr, value, op_size)?;
            } else {
                self.set_reg(rm, value, op_size);
            }
        } else {
            // MOV r, r/m
            let value = if is_memory {
                self.read_mem(addr, op_size)?
            } else {
                self.get_reg(rm, op_size)
            };
            self.set_reg(reg, value, op_size);
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    fn apx_scalar_op_size(ctx: &InsnContext) -> u8 {
        if ctx.evex_w() {
            8
        } else if ctx.operand_size_override {
            2
        } else {
            4
        }
    }

    fn inject_invalid_opcode(&mut self) -> Result<Option<VcpuExit>> {
        self.inject_exception(6, None)?;
        Ok(None)
    }

    /// APX MOVBE reg, reg.
    fn execute_apx_movbe(
        &mut self,
        ctx: &mut InsnContext,
        ndd: bool,
        nf: bool,
    ) -> Result<Option<VcpuExit>> {
        if ndd || nf {
            return self.inject_invalid_opcode();
        }

        let op_size = Self::apx_scalar_op_size(ctx);
        let (reg, rm, is_memory, _, _) = self.decode_modrm(ctx)?;
        if is_memory {
            return self.inject_invalid_opcode();
        }

        let dest = rm | ctx.evex_rm_reg();
        let src = reg | ctx.evex_dest_reg();
        let value = self.get_reg(src, op_size);
        let result = match op_size {
            2 => (value as u16).swap_bytes() as u64,
            4 => (value as u32).swap_bytes() as u64,
            8 => value.swap_bytes(),
            _ => unreachable!(),
        };

        self.set_reg(dest, result, op_size);
        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// APX NF POPCNT/LZCNT/TZCNT.
    fn execute_apx_count(
        &mut self,
        ctx: &mut InsnContext,
        opcode: u8,
        ndd: bool,
        nf: bool,
    ) -> Result<Option<VcpuExit>> {
        if ndd || !nf {
            return self.inject_invalid_opcode();
        }

        let op_size = Self::apx_scalar_op_size(ctx);
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let dest = reg | ctx.evex_dest_reg();
        let src = if is_memory {
            self.read_mem(addr, op_size)?
        } else {
            let src_reg = rm | ctx.evex_rm_reg();
            self.get_reg(src_reg, op_size)
        };

        let bit_count = (op_size * 8) as u64;
        let result = match opcode {
            0x88 => match op_size {
                2 => (src as u16).count_ones() as u64,
                4 => (src as u32).count_ones() as u64,
                8 => src.count_ones() as u64,
                _ => unreachable!(),
            },
            0xF4 => {
                if src == 0 {
                    bit_count
                } else {
                    src.trailing_zeros() as u64
                }
            }
            0xF5 => {
                if src == 0 {
                    bit_count
                } else {
                    match op_size {
                        2 => (src as u16).leading_zeros() as u64,
                        4 => (src as u32).leading_zeros() as u64,
                        8 => src.leading_zeros() as u64,
                        _ => unreachable!(),
                    }
                }
            }
            _ => unreachable!(),
        };

        self.set_reg(dest, result, op_size);
        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// APX LEA operation
    fn execute_apx_lea(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let op_size = if ctx.evex_w() { 8 } else { 4 };
        let modrm_start = ctx.cursor;
        let (reg, _, is_memory, _, _) = self.decode_modrm(ctx)?;

        if !is_memory {
            return self.inject_invalid_opcode();
        }

        // Recalculate address without actually reading memory. LEA yields the
        // segment OFFSET and must ignore any FS/GS override.
        let (addr, _) = self.decode_lea_addr(ctx, modrm_start)?;
        let reg = reg | ctx.evex_dest_reg();

        self.set_reg(reg, addr, op_size);
        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// APX POP2 - Pop two registers atomically
    fn execute_apx_pop2(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let modrm = ctx.consume_u8()?;
        if (modrm >> 6) != 3 {
            return self.inject_invalid_opcode();
        }

        // Extract register operands
        let reg1 = (modrm & 0x07) | ctx.evex_rm_reg();
        let reg2 = ctx.evex_vvvv();

        // Pop reg1 first (from RSP), then reg2 (from RSP+8)
        let val1 = self.read_mem(self.regs.rsp, 8)?;
        let val2 = self.read_mem(self.regs.rsp + 8, 8)?;
        self.regs.rsp = self.regs.rsp.wrapping_add(16);

        self.set_reg(reg1, val1, 8);
        self.set_reg(reg2, val2, 8);

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// APX PUSH2 - Push two registers atomically.
    fn execute_apx_push2(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let modrm = ctx.consume_u8()?;
        if (modrm >> 6) != 3 {
            return self.inject_invalid_opcode();
        }

        let reg1 = (modrm & 0x07) | ctx.evex_rm_reg();
        let reg2 = ctx.evex_vvvv();
        let val1 = self.get_reg(reg1, 8);
        let val2 = self.get_reg(reg2, 8);
        let new_rsp = self.regs.rsp.wrapping_sub(16);

        self.write_mem(new_rsp, val1, 8)?;
        self.write_mem(new_rsp + 8, val2, 8)?;
        self.regs.rsp = new_rsp;
        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// APX IMUL with immediate
    fn execute_apx_imul_imm(
        &mut self,
        ctx: &mut InsnContext,
        ndd: bool,
        nf: bool,
        imm32: bool,
    ) -> Result<Option<VcpuExit>> {
        let op_size = if ctx.evex_w() { 8 } else { 4 };
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let reg = reg | ctx.evex_dest_reg();

        let src = if is_memory {
            self.read_mem(addr, op_size)?
        } else {
            let rm = rm | ctx.evex_rm_reg();
            self.get_reg(rm, op_size)
        };

        let imm = if imm32 {
            ctx.consume_u32()? as i32 as i64 as u64
        } else {
            ctx.consume_u8()? as i8 as i64 as u64
        };

        let result = if op_size == 8 {
            (src as i64).wrapping_mul(imm as i64) as u64
        } else {
            ((src as i32).wrapping_mul(imm as i32)) as u64
        };

        let dest_reg = if ndd { ctx.evex_vvvv() } else { reg };
        self.set_reg(dest_reg, result, op_size);

        if !nf {
            // Set OF/CF if result overflowed
            let sign_extended = if op_size == 8 {
                (result as i64) as i128 == (src as i64 as i128) * (imm as i64 as i128)
            } else {
                (result as i32) as i64 == (src as i32 as i64) * (imm as i32 as i64)
            };
            let flags = self.regs.rflags & !(0x801); // Clear OF, CF
            self.regs.rflags = if sign_extended { flags } else { flags | 0x801 };
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// APX IMUL with register/memory source.
    fn execute_apx_imul(
        &mut self,
        ctx: &mut InsnContext,
        ndd: bool,
        nf: bool,
    ) -> Result<Option<VcpuExit>> {
        let op_size = if ctx.evex_w() { 8 } else { 4 };
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let reg = reg | ctx.evex_dest_reg();

        let src1 = self.get_reg(reg, op_size);
        let src2 = if is_memory {
            self.read_mem(addr, op_size)?
        } else {
            let rm = rm | ctx.evex_rm_reg();
            self.get_reg(rm, op_size)
        };

        let (result, overflow) = if op_size == 8 {
            let product = (src1 as i64 as i128) * (src2 as i64 as i128);
            let result = product as i64 as u64;
            (result, product != result as i64 as i128)
        } else {
            let product = (src1 as i32 as i64) * (src2 as i32 as i64);
            let result = product as i32 as u64;
            (result, product != result as i32 as i64)
        };

        let dest_reg = if ndd { ctx.evex_vvvv() } else { reg };
        self.set_reg(dest_reg, result, op_size);

        if !nf {
            let flags = self.regs.rflags & !0x801; // Clear OF, CF
            self.regs.rflags = if overflow { flags | 0x801 } else { flags };
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// APX SHLD/SHRD double shifts.
    fn execute_apx_double_shift(
        &mut self,
        ctx: &mut InsnContext,
        opcode: u8,
        ndd: bool,
        nf: bool,
    ) -> Result<Option<VcpuExit>> {
        let op_size = if ctx.evex_w() { 8 } else { 4 };
        let width = op_size as u32 * 8;
        let mask = if op_size == 8 {
            u64::MAX
        } else {
            (1u64 << width) - 1
        };
        let is_shrd = matches!(opcode, 0x2C | 0xAD);
        let count_mask = if op_size == 8 { 0x3F } else { 0x1F };

        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let src1_reg = rm | ctx.evex_rm_reg();
        let src2_reg = reg | ctx.evex_dest_reg();
        let src1 = if is_memory {
            self.read_mem(addr, op_size)?
        } else {
            self.get_reg(src1_reg, op_size)
        } & mask;
        let src2 = self.get_reg(src2_reg, op_size) & mask;
        let count = if matches!(opcode, 0x24 | 0x2C) {
            ctx.consume_u8()? & count_mask
        } else {
            (self.regs.rcx as u8) & count_mask
        };

        let result = if count == 0 {
            src1
        } else {
            let count = count as u32;
            if is_shrd {
                ((src1 >> count) | (src2 << (width - count))) & mask
            } else {
                ((src1 << count) | (src2 >> (width - count))) & mask
            }
        };

        if ndd {
            self.set_reg(ctx.evex_vvvv(), result, op_size);
        } else if is_memory {
            self.write_mem(addr, result, op_size)?;
        } else {
            self.set_reg(src1_reg, result, op_size);
        }

        if !nf && count != 0 {
            self.update_apx_double_shift_flags(result, src1, count, op_size, is_shrd);
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// APX group 1 immediate ALU operations.
    fn execute_apx_group1_imm(
        &mut self,
        ctx: &mut InsnContext,
        opcode: u8,
        ndd: bool,
        nf: bool,
    ) -> Result<Option<VcpuExit>> {
        let op_size = if matches!(opcode, 0x80 | 0x82) {
            1
        } else if ctx.evex_w() {
            8
        } else {
            4
        };

        let (op, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let op = op & 0x07;
        let src_reg = rm | ctx.evex_rm_reg();
        let src = if is_memory {
            self.read_mem(addr, op_size)?
        } else {
            self.get_reg(src_reg, op_size)
        };

        let imm = match opcode {
            0x80 | 0x82 => ctx.consume_u8()? as u64,
            0x81 if op_size == 8 => ctx.consume_u32()? as i32 as i64 as u64,
            0x81 => ctx.consume_u32()? as u64,
            0x83 => ctx.consume_u8()? as i8 as i64 as u64,
            _ => unreachable!(),
        };

        let cf_in = (self.regs.rflags & 0x001) != 0;
        let result = match op {
            0 => src.wrapping_add(imm),
            1 => src | imm,
            2 => src.wrapping_add(imm).wrapping_add(u64::from(cf_in)),
            3 => src.wrapping_sub(imm).wrapping_sub(u64::from(cf_in)),
            4 => src & imm,
            5 | 7 => src.wrapping_sub(imm),
            6 => src ^ imm,
            _ => unreachable!(),
        };

        if op != 7 {
            if ndd {
                self.set_reg(ctx.evex_vvvv(), result, op_size);
            } else if is_memory {
                self.write_mem(addr, result, op_size)?;
            } else {
                self.set_reg(src_reg, result, op_size);
            }
        }

        if !nf {
            match op {
                0 => self.update_flags_alu(result, src, imm, op_size, ApxAluOp::Add),
                1 => self.update_flags_alu(result, src, imm, op_size, ApxAluOp::Or),
                2 => {
                    flags::update_flags_adc(&mut self.regs.rflags, src, imm, cf_in, result, op_size)
                }
                3 => {
                    flags::update_flags_sbb(&mut self.regs.rflags, src, imm, cf_in, result, op_size)
                }
                4 => self.update_flags_alu(result, src, imm, op_size, ApxAluOp::And),
                5 | 7 => self.update_flags_alu(result, src, imm, op_size, ApxAluOp::Sub),
                6 => self.update_flags_alu(result, src, imm, op_size, ApxAluOp::Xor),
                _ => unreachable!(),
            }
            self.clear_lazy_flags();
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// APX shift with immediate
    fn execute_apx_shift_imm(
        &mut self,
        ctx: &mut InsnContext,
        opcode: u8,
        ndd: bool,
        nf: bool,
    ) -> Result<Option<VcpuExit>> {
        let is_byte = opcode == 0xC0;
        let op_size = if is_byte {
            1
        } else if ctx.evex_w() {
            8
        } else {
            4
        };

        let modrm = ctx.peek_u8()?;
        let shift_type = (modrm >> 3) & 0x07;
        let (_, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let rm = rm | ctx.evex_rm_reg();
        let imm = ctx.consume_u8()?;

        let src = if is_memory {
            self.read_mem(addr, op_size)?
        } else {
            self.get_reg(rm, op_size)
        };

        let shift_mask = if op_size == 8 { 0x3F } else { 0x1F };
        let count = (imm as u64) & shift_mask;

        if shift_type <= 3 {
            self.materialize_flags();
        }
        let result = self.perform_shift(src, count, shift_type, op_size);

        let dest = if ndd { ctx.evex_vvvv() } else { rm };

        if ndd || !is_memory {
            self.set_reg(dest, result, op_size);
        } else {
            self.write_mem(addr, result, op_size)?;
        }

        if !nf && count != 0 {
            self.update_flags_shift(result, src, count, shift_type, op_size);
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// APX shift by CL
    fn execute_apx_shift_cl(
        &mut self,
        ctx: &mut InsnContext,
        opcode: u8,
        ndd: bool,
        nf: bool,
    ) -> Result<Option<VcpuExit>> {
        let is_byte = (opcode & 0x01) == 0;
        let op_size = if is_byte {
            1
        } else if ctx.evex_w() {
            8
        } else {
            4
        };
        let by_one = (opcode & 0x02) == 0;

        let modrm = ctx.peek_u8()?;
        let shift_type = (modrm >> 3) & 0x07;
        let (_, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let rm = rm | ctx.evex_rm_reg();

        let src = if is_memory {
            self.read_mem(addr, op_size)?
        } else {
            self.get_reg(rm, op_size)
        };

        let shift_mask = if op_size == 8 { 0x3F } else { 0x1F };
        let count = if by_one {
            1
        } else {
            self.regs.rcx & shift_mask
        };

        if shift_type <= 3 {
            self.materialize_flags();
        }
        let result = self.perform_shift(src, count, shift_type, op_size);

        let dest = if ndd { ctx.evex_vvvv() } else { rm };

        if ndd || !is_memory {
            self.set_reg(dest, result, op_size);
        } else {
            self.write_mem(addr, result, op_size)?;
        }

        if !nf && count != 0 {
            self.update_flags_shift(result, src, count, shift_type, op_size);
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// APX group 3 NOT/NEG.
    fn execute_apx_group3(
        &mut self,
        ctx: &mut InsnContext,
        opcode: u8,
        ndd: bool,
        nf: bool,
    ) -> Result<Option<VcpuExit>> {
        let op_size = if opcode == 0xF6 {
            1
        } else if ctx.evex_w() {
            8
        } else {
            4
        };
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let op_type = reg & 0x07;
        let src_reg = rm | ctx.evex_rm_reg();
        let src = if is_memory {
            self.read_mem(addr, op_size)?
        } else {
            self.get_reg(src_reg, op_size)
        };

        if matches!(op_type, 4..=7) {
            if !nf || ndd {
                return self.inject_invalid_opcode();
            }
            if self.execute_apx_group3_implicit(op_type, src, op_size)? {
                self.regs.rip += ctx.cursor as u64;
            }
            return Ok(None);
        }

        if !matches!(op_type, 2 | 3) {
            return Err(Error::Emulator(format!(
                "Unimplemented APX group3 opcode {:#x} /{} at RIP={:#x}",
                opcode, op_type, self.regs.rip
            )));
        }

        let result = if op_type == 2 {
            !src
        } else {
            match op_size {
                1 => (src as i8).wrapping_neg() as u8 as u64,
                2 => (src as i16).wrapping_neg() as u16 as u64,
                4 => (src as i32).wrapping_neg() as u32 as u64,
                8 => (src as i64).wrapping_neg() as u64,
                _ => src,
            }
        };

        if ndd {
            self.set_reg(ctx.evex_vvvv(), result, op_size);
        } else if is_memory {
            self.write_mem(addr, result, op_size)?;
        } else {
            self.set_reg(src_reg, result, op_size);
        }

        if op_type == 3 && !nf {
            flags::update_flags_sub(&mut self.regs.rflags, 0, src, result, op_size);
            self.clear_lazy_flags();
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    fn execute_apx_group3_implicit(&mut self, op_type: u8, src: u64, op_size: u8) -> Result<bool> {
        match (op_type, op_size) {
            (4, 1) => {
                let result = (self.regs.rax as u8 as u16) * (src as u8 as u16);
                self.set_reg(0, result as u64, 2);
            }
            (4, 4) => {
                let result = (self.regs.rax as u32 as u64) * (src as u32 as u64);
                self.set_reg(0, result as u32 as u64, 4);
                self.set_reg(2, (result >> 32) as u32 as u64, 4);
            }
            (4, 8) => {
                let result = (self.regs.rax as u128) * (src as u128);
                self.set_reg(0, result as u64, 8);
                self.set_reg(2, (result >> 64) as u64, 8);
            }
            (5, 1) => {
                let result = (self.regs.rax as u8 as i8 as i16) * (src as u8 as i8 as i16);
                self.set_reg(0, result as u16 as u64, 2);
            }
            (5, 4) => {
                let result = (self.regs.rax as u32 as i32 as i64) * (src as u32 as i32 as i64);
                self.set_reg(0, result as u32 as u64, 4);
                self.set_reg(2, (result >> 32) as u32 as u64, 4);
            }
            (5, 8) => {
                let result = (self.regs.rax as i64 as i128) * (src as i64 as i128);
                self.set_reg(0, result as u64, 8);
                self.set_reg(2, (result >> 64) as u64, 8);
            }
            (6, 1) => {
                let divisor = src as u8 as u16;
                if divisor == 0 {
                    self.inject_exception(0, None)?;
                    return Ok(false);
                }
                let dividend = self.regs.rax as u16;
                let quotient = dividend / divisor;
                let remainder = dividend % divisor;
                if quotient > u8::MAX as u16 {
                    self.inject_exception(0, None)?;
                    return Ok(false);
                }
                self.set_reg(0, ((remainder << 8) | quotient) as u64, 2);
            }
            (6, 4) => {
                let divisor = src as u32 as u64;
                if divisor == 0 {
                    self.inject_exception(0, None)?;
                    return Ok(false);
                }
                let dividend =
                    ((self.regs.rdx as u32 as u64) << 32) | (self.regs.rax as u32 as u64);
                let quotient = dividend / divisor;
                let remainder = dividend % divisor;
                if quotient > u32::MAX as u64 {
                    self.inject_exception(0, None)?;
                    return Ok(false);
                }
                self.set_reg(0, quotient as u32 as u64, 4);
                self.set_reg(2, remainder as u32 as u64, 4);
            }
            (6, 8) => {
                let divisor = src as u128;
                if divisor == 0 {
                    self.inject_exception(0, None)?;
                    return Ok(false);
                }
                let dividend = ((self.regs.rdx as u128) << 64) | (self.regs.rax as u128);
                let quotient = dividend / divisor;
                let remainder = dividend % divisor;
                if quotient > u64::MAX as u128 {
                    self.inject_exception(0, None)?;
                    return Ok(false);
                }
                self.set_reg(0, quotient as u64, 8);
                self.set_reg(2, remainder as u64, 8);
            }
            (7, 1) => {
                let divisor = src as u8 as i8 as i16;
                if divisor == 0 {
                    self.inject_exception(0, None)?;
                    return Ok(false);
                }
                let dividend = self.regs.rax as u16 as i16;
                let (quotient, remainder) =
                    match (dividend.checked_div(divisor), dividend.checked_rem(divisor)) {
                        (Some(q), Some(r)) => (q, r),
                        _ => {
                            self.inject_exception(0, None)?;
                            return Ok(false);
                        }
                    };
                if quotient < i8::MIN as i16 || quotient > i8::MAX as i16 {
                    self.inject_exception(0, None)?;
                    return Ok(false);
                }
                let ax = ((remainder as i8 as u8 as u16) << 8) | (quotient as i8 as u8 as u16);
                self.set_reg(0, ax as u64, 2);
            }
            (7, 4) => {
                let divisor = src as u32 as i32 as i64;
                if divisor == 0 {
                    self.inject_exception(0, None)?;
                    return Ok(false);
                }
                let dividend =
                    (((self.regs.rdx as u32 as u64) << 32) | (self.regs.rax as u32 as u64)) as i64;
                let (quotient, remainder) =
                    match (dividend.checked_div(divisor), dividend.checked_rem(divisor)) {
                        (Some(q), Some(r)) => (q, r),
                        _ => {
                            self.inject_exception(0, None)?;
                            return Ok(false);
                        }
                    };
                if quotient < i32::MIN as i64 || quotient > i32::MAX as i64 {
                    self.inject_exception(0, None)?;
                    return Ok(false);
                }
                self.set_reg(0, quotient as u32 as u64, 4);
                self.set_reg(2, remainder as u32 as u64, 4);
            }
            (7, 8) => {
                let divisor = src as i64 as i128;
                if divisor == 0 {
                    self.inject_exception(0, None)?;
                    return Ok(false);
                }
                let dividend = (((self.regs.rdx as u128) << 64) | (self.regs.rax as u128)) as i128;
                let (quotient, remainder) =
                    match (dividend.checked_div(divisor), dividend.checked_rem(divisor)) {
                        (Some(q), Some(r)) => (q, r),
                        _ => {
                            self.inject_exception(0, None)?;
                            return Ok(false);
                        }
                    };
                if quotient < i64::MIN as i128 || quotient > i64::MAX as i128 {
                    self.inject_exception(0, None)?;
                    return Ok(false);
                }
                self.set_reg(0, quotient as u64, 8);
                self.set_reg(2, remainder as u64, 8);
            }
            _ => {
                return Err(Error::Emulator(format!(
                    "Unsupported APX group3 implicit /{} size {}",
                    op_type, op_size
                )));
            }
        }
        Ok(true)
    }

    /// APX INC/DEC
    fn execute_apx_group_ff(
        &mut self,
        ctx: &mut InsnContext,
        opcode: u8,
        ndd: bool,
        nf: bool,
    ) -> Result<Option<VcpuExit>> {
        let modrm = ctx.peek_u8()?;
        let op_type = (modrm >> 3) & 0x07;
        if opcode == 0xFF && op_type == 6 {
            return self.execute_apx_push2(ctx);
        }
        if op_type > 1 {
            return Err(Error::Emulator(format!(
                "Unimplemented APX group opcode {:#x} /{} at RIP={:#x}",
                opcode, op_type, self.regs.rip
            )));
        }
        self.execute_apx_inc_dec(ctx, opcode, ndd, nf)
    }

    fn execute_apx_inc_dec(
        &mut self,
        ctx: &mut InsnContext,
        opcode: u8,
        ndd: bool,
        nf: bool,
    ) -> Result<Option<VcpuExit>> {
        let is_byte = opcode == 0xFE;
        let op_size = if is_byte {
            1
        } else if ctx.evex_w() {
            8
        } else {
            4
        };

        let modrm = ctx.peek_u8()?;
        let op_type = (modrm >> 3) & 0x07;
        let is_dec = op_type == 1;

        let (_, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let rm = rm | ctx.evex_rm_reg();

        let src = if is_memory {
            self.read_mem(addr, op_size)?
        } else {
            self.get_reg(rm, op_size)
        };

        let result = if is_dec {
            src.wrapping_sub(1)
        } else {
            src.wrapping_add(1)
        };

        let dest = if ndd { ctx.evex_vvvv() } else { rm };

        if ndd || !is_memory {
            self.set_reg(dest, result, op_size);
        } else {
            self.write_mem(addr, result, op_size)?;
        }

        if !nf {
            // INC/DEC don't affect CF
            let old_cf = self.regs.rflags & 0x001;
            self.update_flags_alu(
                result,
                src,
                1,
                op_size,
                if is_dec { ApxAluOp::Sub } else { ApxAluOp::Add },
            );
            self.regs.rflags = (self.regs.rflags & !0x001) | old_cf;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// Helper: perform shift operation
    fn perform_shift(&self, src: u64, count: u64, shift_type: u8, op_size: u8) -> u64 {
        if count == 0 {
            return src;
        }

        let width = op_size as u32 * 8;
        let mask = if width == 64 {
            u64::MAX
        } else {
            (1u64 << width) - 1
        };
        let src = src & mask;

        match shift_type {
            0 => {
                let count = (count as u32) % width;
                if count == 0 {
                    return src;
                }
                ((src << count) | (src >> (width - count))) & mask
            }
            1 => {
                let count = (count as u32) % width;
                if count == 0 {
                    return src;
                }
                ((src >> count) | (src << (width - count))) & mask
            }
            2 => {
                // RCL
                let count = (count as u32) % (width + 1);
                if count == 0 {
                    return src;
                }

                let mut result = src;
                let mut carry = (self.regs.rflags & flags::bits::CF) != 0;
                for _ in 0..count {
                    let msb = (result >> (width - 1)) & 1 != 0;
                    result = ((result << 1) | carry as u64) & mask;
                    carry = msb;
                }
                result
            }
            3 => {
                // RCR
                let count = (count as u32) % (width + 1);
                if count == 0 {
                    return src;
                }

                let mut result = src;
                let mut carry = (self.regs.rflags & flags::bits::CF) != 0;
                for _ in 0..count {
                    let lsb = result & 1 != 0;
                    result = (result >> 1) | ((carry as u64) << (width - 1));
                    carry = lsb;
                }
                result & mask
            }
            4 | 6 => (src << count) & mask, // SHL/SAL
            5 => src >> count,              // SHR
            7 => {
                // SAR - arithmetic shift right
                if count as u32 >= width {
                    return if (src & (1u64 << (width - 1))) != 0 {
                        mask
                    } else {
                        0
                    };
                }
                match op_size {
                    1 => ((src as i8) >> count) as u8 as u64,
                    2 => ((src as i16) >> count) as u16 as u64,
                    4 => ((src as i32) >> count) as u32 as u64,
                    8 => ((src as i64) >> count) as u64,
                    _ => src,
                }
            }
            _ => src,
        }
    }

    /// Update flags for ALU operations
    fn update_flags_alu(
        &mut self,
        result: u64,
        src1: u64,
        src2: u64,
        op_size: u8,
        alu_op: ApxAluOp,
    ) {
        let sign_bit: u64 = match op_size {
            1 => 0x80,
            2 => 0x8000,
            4 => 0x8000_0000,
            8 => 0x8000_0000_0000_0000,
            _ => 0x8000_0000,
        };
        let max_val: u64 = match op_size {
            1 => 0xFF,
            2 => 0xFFFF,
            4 => 0xFFFF_FFFF,
            8 => u64::MAX,
            _ => 0xFFFF_FFFF,
        };

        let masked_result = result & max_val;

        // ZF - zero flag
        let zf = masked_result == 0;
        // SF - sign flag
        let sf = (masked_result & sign_bit) != 0;
        // PF - parity flag (low byte)
        let pf = (result as u8).count_ones() % 2 == 0;

        // CF and OF depend on operation
        let (cf, of) = match alu_op {
            ApxAluOp::Add | ApxAluOp::Adc => {
                let cf = result > max_val || result < src1;
                let of = ((!(src1 ^ src2)) & (src1 ^ result) & sign_bit) != 0;
                (cf, of)
            }
            ApxAluOp::Sub | ApxAluOp::Sbb => {
                let cf = src1 < src2;
                let of = ((src1 ^ src2) & (src1 ^ result) & sign_bit) != 0;
                (cf, of)
            }
            ApxAluOp::And | ApxAluOp::Or | ApxAluOp::Xor => {
                (false, false) // Logical ops clear CF and OF
            }
        };

        // Update RFLAGS
        let mut flags = self.regs.rflags;
        flags &= !(0x8D5); // Clear CF, PF, ZF, SF, OF
        if cf {
            flags |= 0x001;
        }
        if pf {
            flags |= 0x004;
        }
        if zf {
            flags |= 0x040;
        }
        if sf {
            flags |= 0x080;
        }
        if of {
            flags |= 0x800;
        }
        self.regs.rflags = flags;
    }

    /// Update flags for shift operations
    fn update_flags_shift(
        &mut self,
        result: u64,
        src: u64,
        count: u64,
        shift_type: u8,
        op_size: u8,
    ) {
        let sign_bit: u64 = match op_size {
            1 => 0x80,
            2 => 0x8000,
            4 => 0x8000_0000,
            8 => 0x8000_0000_0000_0000,
            _ => 0x8000_0000,
        };
        let max_val: u64 = match op_size {
            1 => 0xFF,
            2 => 0xFFFF,
            4 => 0xFFFF_FFFF,
            8 => u64::MAX,
            _ => 0xFFFF_FFFF,
        };

        let masked_result = result & max_val;

        let bits = op_size as u64 * 8;
        if shift_type <= 3 {
            let rotate_count = if shift_type <= 1 {
                count % bits
            } else {
                count % (bits + 1)
            };
            if rotate_count == 0 {
                return;
            }

            let cf = match shift_type {
                0 => (masked_result & 1) != 0,                // ROL
                1 => (masked_result & sign_bit) != 0,         // ROR
                2 => (src >> (bits - rotate_count)) & 1 != 0, // RCL
                3 => (src >> (rotate_count - 1)) & 1 != 0,    // RCR
                _ => unreachable!(),
            };
            let of = if rotate_count == 1 {
                match shift_type {
                    0 => ((masked_result >> (bits - 1)) ^ masked_result) & 1 != 0,
                    1 | 3 => {
                        ((masked_result >> (bits - 1)) ^ (masked_result >> (bits - 2))) & 1 != 0
                    }
                    2 => ((masked_result & sign_bit) != 0) ^ cf,
                    _ => unreachable!(),
                }
            } else {
                (self.regs.rflags & flags::bits::OF) != 0
            };

            if cf {
                self.regs.rflags |= flags::bits::CF;
            } else {
                self.regs.rflags &= !flags::bits::CF;
            }
            if of {
                self.regs.rflags |= flags::bits::OF;
            } else {
                self.regs.rflags &= !flags::bits::OF;
            }
            self.clear_lazy_flags();
            return;
        }

        // ZF, SF, PF from result
        let zf = masked_result == 0;
        let sf = (masked_result & sign_bit) != 0;
        let pf = (result as u8).count_ones() % 2 == 0;

        // CF depends on shift type and direction
        let cf = match shift_type {
            4 | 6 => count <= bits && (src >> (bits - count)) & 1 != 0,
            5 => count <= bits && (src >> (count - 1)) & 1 != 0,
            7 => {
                if count <= bits {
                    (src >> (count - 1)) & 1 != 0
                } else {
                    (src >> (bits - 1)) & 1 != 0
                }
            }
            _ => unreachable!(),
        };

        // OF is only defined for count=1
        let of = if count == 1 {
            match shift_type {
                4 | 6 => (masked_result & sign_bit) != (src & sign_bit), // SHL: sign change
                5 => (src & sign_bit) != 0,                              // SHR: old sign
                7 => false,                                              // SAR: always 0
                _ => unreachable!(),
            }
        } else {
            false // Undefined for count > 1, we clear it
        };

        let mut flags = self.regs.rflags;
        flags &= !(0x8D5);
        if cf {
            flags |= 0x001;
        }
        if pf {
            flags |= 0x004;
        }
        if zf {
            flags |= 0x040;
        }
        if sf {
            flags |= 0x080;
        }
        if of {
            flags |= 0x800;
        }
        self.regs.rflags = flags;
    }

    fn update_apx_double_shift_flags(
        &mut self,
        result: u64,
        src1: u64,
        count: u8,
        op_size: u8,
        is_shrd: bool,
    ) {
        let width = op_size as u32 * 8;
        let sign_bit = 1u64 << (width - 1);
        let cf = if is_shrd {
            ((src1 >> (count - 1)) & 1) != 0
        } else {
            ((src1 >> (width - count as u32)) & 1) != 0
        };
        let of = count == 1 && (((result ^ src1) & sign_bit) != 0);

        flags::update_flags_logic(&mut self.regs.rflags, result, op_size);
        if cf {
            self.regs.rflags |= flags::bits::CF;
        } else {
            self.regs.rflags &= !flags::bits::CF;
        }
        if of {
            self.regs.rflags |= flags::bits::OF;
        } else {
            self.regs.rflags &= !flags::bits::OF;
        }
        self.clear_lazy_flags();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Arc;

    use vm_memory::{GuestAddress, GuestMemoryMmap};

    use crate::backend::emulator::x86_64::flags;

    const CODE: u64 = 0x1000;
    const DATA: u64 = 0x2000;
    const INVALID: u64 = 0x2_0000;

    fn long_mode_vcpu(code: &[u8]) -> X86_64Vcpu {
        let mem =
            Arc::new(GuestMemoryMmap::<()>::from_ranges(&[(GuestAddress(0), 0x10000)]).unwrap());
        let mut vcpu = X86_64Vcpu::new(0, mem);
        vcpu.regs.rip = CODE;
        vcpu.regs.rflags = 0x2;
        vcpu.sregs.efer = 0x400;
        vcpu.sregs.cs.l = true;
        vcpu.sregs.cs.db = false;

        let sregs = vcpu.sregs.clone();
        vcpu.mmu.write(CODE, code, &sregs).unwrap();
        vcpu
    }

    fn step_ok(vcpu: &mut X86_64Vcpu) {
        assert!(vcpu.step().unwrap().is_none());
    }

    fn write_u64(vcpu: &mut X86_64Vcpu, addr: u64, value: u64) {
        let sregs = vcpu.sregs.clone();
        vcpu.mmu.write_u64(addr, value, &sregs).unwrap();
    }

    fn read_u64(vcpu: &mut X86_64Vcpu, addr: u64) -> u64 {
        let sregs = vcpu.sregs.clone();
        vcpu.mmu.read_u64(addr, &sregs).unwrap()
    }

    #[test]
    fn apx_map4_setzu_and_evex_setcc_split_by_nd_like_llvm() {
        // LLVM 20: `setzub %al` => 62 f4 7f 18 42 c0.
        let mut vcpu = long_mode_vcpu(&[0x62, 0xF4, 0x7F, 0x18, 0x42, 0xC0]);
        vcpu.regs.rax = 0xAAAA_BBBB_CCCC_DDDD;
        vcpu.regs.rflags = 0x2 | flags::bits::CF;
        step_ok(&mut vcpu);
        assert_eq!(vcpu.regs.rax, 1);
        assert_eq!(vcpu.regs.rip, CODE + 6);

        // LLVM 20: `{evex} setb %al` => 62 f4 7f 08 42 c0.
        let mut vcpu = long_mode_vcpu(&[0x62, 0xF4, 0x7F, 0x08, 0x42, 0xC0]);
        vcpu.regs.rax = 0x1122_3344_5566_77FF;
        vcpu.regs.rflags = 0x2;
        step_ok(&mut vcpu);
        assert_eq!(vcpu.regs.rax, 0x1122_3344_5566_7700);
        assert_eq!(vcpu.regs.rip, CODE + 6);
    }

    #[test]
    fn apx_cmov_nd_uses_vvvv_destination_like_llvm() {
        // LLVM 20: `cmovbq %rbx, %rax, %r8` => 62 f4 bc 18 42 c3.
        let mut vcpu = long_mode_vcpu(&[0x62, 0xF4, 0xBC, 0x18, 0x42, 0xC3]);
        vcpu.regs.rax = 0x1111;
        vcpu.regs.rbx = 0x2222;
        vcpu.regs.r8 = 0x3333;
        vcpu.regs.rflags = 0x2 | flags::bits::CF;
        step_ok(&mut vcpu);
        assert_eq!(vcpu.regs.r8, 0x2222);

        let mut vcpu = long_mode_vcpu(&[0x62, 0xF4, 0xBC, 0x18, 0x42, 0xC3]);
        vcpu.regs.rax = 0x1111;
        vcpu.regs.rbx = 0x2222;
        vcpu.regs.r8 = 0x3333;
        vcpu.regs.rflags = 0x2;
        step_ok(&mut vcpu);
        assert_eq!(vcpu.regs.r8, 0x1111);
    }

    #[test]
    fn apx_cfcmov_two_operand_directions_and_false_zero_like_llvm() {
        // LLVM 20: clear NF decodes as `cfcmovbq %rax, %rbx`
        // from 62 f4 fc 08 42 d8: dst=ModRM.reg, src=r/m.
        let mut vcpu = long_mode_vcpu(&[0x62, 0xF4, 0xFC, 0x08, 0x42, 0xD8]);
        vcpu.regs.rax = 0xAAAA;
        vcpu.regs.rbx = 0xBBBB;
        vcpu.regs.rflags = 0x2 | flags::bits::CF;
        step_ok(&mut vcpu);
        assert_eq!(vcpu.regs.rbx, 0xAAAA);

        let mut vcpu = long_mode_vcpu(&[0x62, 0xF4, 0xFC, 0x08, 0x42, 0xD8]);
        vcpu.regs.rax = 0xAAAA;
        vcpu.regs.rbx = 0xBBBB;
        vcpu.regs.rflags = 0x2;
        step_ok(&mut vcpu);
        assert_eq!(vcpu.regs.rbx, 0);

        // LLVM 20: `cfcmovbq %rbx, %rax` => 62 f4 fc 0c 42 d8.
        let mut vcpu = long_mode_vcpu(&[0x62, 0xF4, 0xFC, 0x0C, 0x42, 0xD8]);
        vcpu.regs.rax = 0xAAAA;
        vcpu.regs.rbx = 0xBBBB;
        vcpu.regs.rflags = 0x2 | flags::bits::CF;
        step_ok(&mut vcpu);
        assert_eq!(vcpu.regs.rax, 0xBBBB);

        let mut vcpu = long_mode_vcpu(&[0x62, 0xF4, 0xFC, 0x0C, 0x42, 0xD8]);
        vcpu.regs.rax = 0xAAAA;
        vcpu.regs.rbx = 0xBBBB;
        vcpu.regs.rflags = 0x2;
        step_ok(&mut vcpu);
        assert_eq!(vcpu.regs.rax, 0);
    }

    #[test]
    fn apx_cfcmov_memory_source_suppresses_false_fault_like_llvm() {
        // LLVM 20: `cfcmovbq (%rbx), %rax` => 62 f4 fc 08 42 03.
        let mut vcpu = long_mode_vcpu(&[0x62, 0xF4, 0xFC, 0x08, 0x42, 0x03]);
        vcpu.regs.rax = 0xAAAA;
        vcpu.regs.rbx = INVALID;
        vcpu.regs.rflags = 0x2;
        step_ok(&mut vcpu);
        assert_eq!(vcpu.regs.rax, 0);

        let mut vcpu = long_mode_vcpu(&[0x62, 0xF4, 0xFC, 0x08, 0x42, 0x03]);
        write_u64(&mut vcpu, DATA, 0xDEAD_BEEF_CAFE_BABE);
        vcpu.regs.rbx = DATA;
        vcpu.regs.rflags = 0x2 | flags::bits::CF;
        step_ok(&mut vcpu);
        assert_eq!(vcpu.regs.rax, 0xDEAD_BEEF_CAFE_BABE);

        // LLVM 20: `cfcmovbq (%rbx), %rax, %r8` => 62 f4 bc 1c 42 03.
        let mut vcpu = long_mode_vcpu(&[0x62, 0xF4, 0xBC, 0x1C, 0x42, 0x03]);
        vcpu.regs.rax = 0x1234_5678;
        vcpu.regs.rbx = INVALID;
        vcpu.regs.r8 = 0xFFFF;
        vcpu.regs.rflags = 0x2;
        step_ok(&mut vcpu);
        assert_eq!(vcpu.regs.r8, 0x1234_5678);
    }

    #[test]
    fn apx_cfcmov_memory_destination_suppresses_false_fault_like_llvm() {
        // LLVM 20: `cfcmovbq %rbx, (%rax)` => 62 f4 fc 0c 42 18.
        let mut vcpu = long_mode_vcpu(&[0x62, 0xF4, 0xFC, 0x0C, 0x42, 0x18]);
        vcpu.regs.rax = INVALID;
        vcpu.regs.rbx = 0xDEAD_BEEF_CAFE_BABE;
        vcpu.regs.rflags = 0x2;
        step_ok(&mut vcpu);

        let mut vcpu = long_mode_vcpu(&[0x62, 0xF4, 0xFC, 0x0C, 0x42, 0x18]);
        vcpu.regs.rax = DATA;
        vcpu.regs.rbx = 0xDEAD_BEEF_CAFE_BABE;
        vcpu.regs.rflags = 0x2 | flags::bits::CF;
        step_ok(&mut vcpu);
        assert_eq!(read_u64(&mut vcpu, DATA), 0xDEAD_BEEF_CAFE_BABE);
    }

    #[test]
    fn apx_cmov_nd_memory_source_still_faults_when_false() {
        // LLVM 20: `cmovbq (%rbx), %rax, %r8` => 62 f4 bc 18 42 03.
        let mut vcpu = long_mode_vcpu(&[0x62, 0xF4, 0xBC, 0x18, 0x42, 0x03]);
        vcpu.regs.rax = 0x1234;
        vcpu.regs.rbx = INVALID;
        vcpu.regs.r8 = 0xFFFF;
        vcpu.regs.rflags = 0x2;

        assert!(vcpu.step().is_err());
        assert_eq!(vcpu.regs.r8, 0xFFFF);
    }

    #[test]
    fn apx_conditional_map4_rejects_invalid_pp2_like_llvm() {
        // LLVM rejects PP=2 for the MAP4 conditional range.
        let mut vcpu = long_mode_vcpu(&[0x62, 0xF4, 0x7E, 0x18, 0x42, 0xC0]);
        let err = vcpu.step().unwrap_err();
        assert!(
            format!("{err:?}").contains("IDT entry 6 not present"),
            "{err:?}"
        );
    }
}

/// APX ALU operation types
#[derive(Clone, Copy)]
enum ApxAluOp {
    Add,
    Adc,
    Or,
    And,
    Sub,
    Sbb,
    Xor,
}

/// Convert IEEE 754 half-precision (FP16) to single-precision (f32)
fn fp16_to_f32(h: u16) -> f32 {
    let sign = ((h >> 15) & 1) as u32;
    let exp = ((h >> 10) & 0x1F) as u32;
    let mant = (h & 0x3FF) as u32;

    let f32_bits = if exp == 0 {
        if mant == 0 {
            // Zero (preserve sign)
            sign << 31
        } else {
            // Denormalized number - normalize it
            let mut m = mant;
            let mut e = 0i32;
            while (m & 0x400) == 0 {
                m <<= 1;
                e += 1;
            }
            m &= 0x3FF; // Remove implicit bit
            let new_exp = (127 - 15 - e) as u32;
            (sign << 31) | (new_exp << 23) | (m << 13)
        }
    } else if exp == 0x1F {
        // Infinity or NaN
        (sign << 31) | (0xFF << 23) | (mant << 13)
    } else {
        // Normalized number
        // FP16 exponent bias is 15, f32 is 127
        let new_exp = exp + 127 - 15;
        (sign << 31) | (new_exp << 23) | (mant << 13)
    };

    f32::from_bits(f32_bits)
}

/// Convert single-precision (f32) to IEEE 754 half-precision (FP16)
fn f32_to_fp16(f: f32) -> u16 {
    let bits = f.to_bits();
    let sign = ((bits >> 31) & 1) as u16;
    let exp = ((bits >> 23) & 0xFF) as i32;
    let mant = (bits & 0x7FFFFF) as u32;

    if exp == 0xFF {
        // Infinity or NaN
        if mant == 0 {
            // Infinity
            (sign << 15) | (0x1F << 10)
        } else {
            // NaN - preserve some mantissa bits
            (sign << 15) | (0x1F << 10) | ((mant >> 13) as u16 & 0x3FF).max(1)
        }
    } else if exp == 0 {
        // Zero or denormalized f32 (becomes zero in FP16)
        sign << 15
    } else {
        // Normalized number
        let new_exp = exp - 127 + 15;
        if new_exp >= 0x1F {
            // Overflow - return infinity
            (sign << 15) | (0x1F << 10)
        } else if new_exp <= 0 {
            // Underflow - return zero or denormalized
            if new_exp < -10 {
                // Too small, return zero
                sign << 15
            } else {
                // Denormalized
                let shift = 1 - new_exp;
                let m = (0x800000 | mant) >> (13 + shift);
                (sign << 15) | (m as u16 & 0x3FF)
            }
        } else {
            // Normal case
            let new_mant = (mant >> 13) as u16;
            (sign << 15) | ((new_exp as u16) << 10) | (new_mant & 0x3FF)
        }
    }
}

/// Convert BFloat16 (BF16) to single-precision (f32)
fn bf16_to_f32(bf: u16) -> f32 {
    // BF16 is simply the upper 16 bits of f32
    f32::from_bits((bf as u32) << 16)
}

/// Convert single-precision (f32) to BFloat16 (BF16)
fn f32_to_bf16(f: f32) -> u16 {
    // BF16 is the upper 16 bits of f32 with round-to-nearest-even
    let bits = f.to_bits();

    // Check for NaN and preserve signaling NaN
    if (bits & 0x7FFFFFFF) > 0x7F800000 {
        // NaN - ensure we keep a non-zero mantissa
        return ((bits >> 16) as u16) | 0x0040;
    }

    // Round to nearest even
    let rounding_bias = 0x7FFF + ((bits >> 16) & 1);
    ((bits.wrapping_add(rounding_bias)) >> 16) as u16
}
