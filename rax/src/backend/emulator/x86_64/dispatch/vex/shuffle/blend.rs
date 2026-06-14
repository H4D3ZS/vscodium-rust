//! VEX blend instruction implementations.

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::super::super::cpu::{InsnContext, X86_64Vcpu};

impl X86_64Vcpu {
    pub(in crate::backend::emulator::x86_64) fn execute_vex_blend_imm(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
        opcode: u8,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let imm8 = ctx.consume_u8()?;
        let xmm_dst = reg as usize;
        let xmm_src1 = vvvv as usize;
        let count = if vex_l == 1 { 8 } else { 4 };

        if opcode == 0x0C {
            // VBLENDPS
            let mut src1 = [0u32; 8];
            let mut src2 = [0u32; 8];
            let lo = self.regs.xmm[xmm_src1][0];
            let hi = self.regs.xmm[xmm_src1][1];
            src1[0] = lo as u32;
            src1[1] = (lo >> 32) as u32;
            src1[2] = hi as u32;
            src1[3] = (hi >> 32) as u32;
            if vex_l == 1 {
                let hi2 = self.regs.ymm_high[xmm_src1][0];
                let hi3 = self.regs.ymm_high[xmm_src1][1];
                src1[4] = hi2 as u32;
                src1[5] = (hi2 >> 32) as u32;
                src1[6] = hi3 as u32;
                src1[7] = (hi3 >> 32) as u32;
            }

            if is_memory {
                for i in 0..count {
                    src2[i] = self.read_mem(addr + (i * 4) as u64, 4)? as u32;
                }
            } else {
                let xmm_src2 = rm as usize;
                let lo2 = self.regs.xmm[xmm_src2][0];
                let hi2 = self.regs.xmm[xmm_src2][1];
                src2[0] = lo2 as u32;
                src2[1] = (lo2 >> 32) as u32;
                src2[2] = hi2 as u32;
                src2[3] = (hi2 >> 32) as u32;
                if vex_l == 1 {
                    let hi3 = self.regs.ymm_high[xmm_src2][0];
                    let hi4 = self.regs.ymm_high[xmm_src2][1];
                    src2[4] = hi3 as u32;
                    src2[5] = (hi3 >> 32) as u32;
                    src2[6] = hi4 as u32;
                    src2[7] = (hi4 >> 32) as u32;
                }
            }

            let mut dst = [0u32; 8];
            for i in 0..count {
                let use_src2 = ((imm8 >> i) & 1) != 0;
                dst[i] = if use_src2 { src2[i] } else { src1[i] };
            }

            self.regs.xmm[xmm_dst][0] = (dst[0] as u64) | ((dst[1] as u64) << 32);
            self.regs.xmm[xmm_dst][1] = (dst[2] as u64) | ((dst[3] as u64) << 32);
            if vex_l == 1 {
                self.regs.ymm_high[xmm_dst][0] = (dst[4] as u64) | ((dst[5] as u64) << 32);
                self.regs.ymm_high[xmm_dst][1] = (dst[6] as u64) | ((dst[7] as u64) << 32);
            } else {
                self.regs.ymm_high[xmm_dst][0] = 0;
                self.regs.ymm_high[xmm_dst][1] = 0;
            }
        } else {
            // VBLENDPD
            let count_q = if vex_l == 1 { 4 } else { 2 };
            let mut src1 = [0u64; 4];
            let mut src2 = [0u64; 4];
            src1[0] = self.regs.xmm[xmm_src1][0];
            src1[1] = self.regs.xmm[xmm_src1][1];
            if vex_l == 1 {
                src1[2] = self.regs.ymm_high[xmm_src1][0];
                src1[3] = self.regs.ymm_high[xmm_src1][1];
            }

            if is_memory {
                for i in 0..count_q {
                    src2[i] = self.read_mem(addr + (i * 8) as u64, 8)?;
                }
            } else {
                let xmm_src2 = rm as usize;
                src2[0] = self.regs.xmm[xmm_src2][0];
                src2[1] = self.regs.xmm[xmm_src2][1];
                if vex_l == 1 {
                    src2[2] = self.regs.ymm_high[xmm_src2][0];
                    src2[3] = self.regs.ymm_high[xmm_src2][1];
                }
            }

            let mut dst = [0u64; 4];
            for i in 0..count_q {
                let use_src2 = ((imm8 >> i) & 1) != 0;
                dst[i] = if use_src2 { src2[i] } else { src1[i] };
            }

            self.regs.xmm[xmm_dst][0] = dst[0];
            self.regs.xmm[xmm_dst][1] = dst[1];
            if vex_l == 1 {
                self.regs.ymm_high[xmm_dst][0] = dst[2];
                self.regs.ymm_high[xmm_dst][1] = dst[3];
            } else {
                self.regs.ymm_high[xmm_dst][0] = 0;
                self.regs.ymm_high[xmm_dst][1] = 0;
            }
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_blendv(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
        opcode: u8,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let imm8 = ctx.consume_u8()?;
        let mask_reg = ((imm8 >> 4) & 0x0F) as usize;
        let xmm_dst = reg as usize;
        let xmm_src1 = vvvv as usize;
        let count = if vex_l == 1 { 8 } else { 4 };

        if opcode == 0x4A {
            // VBLENDVPS
            let mut src1 = [0u32; 8];
            let mut src2 = [0u32; 8];
            let mut mask = [0u32; 8];
            let lo = self.regs.xmm[xmm_src1][0];
            let hi = self.regs.xmm[xmm_src1][1];
            src1[0] = lo as u32;
            src1[1] = (lo >> 32) as u32;
            src1[2] = hi as u32;
            src1[3] = (hi >> 32) as u32;
            let mask_lo = self.regs.xmm[mask_reg][0];
            let mask_hi = self.regs.xmm[mask_reg][1];
            mask[0] = mask_lo as u32;
            mask[1] = (mask_lo >> 32) as u32;
            mask[2] = mask_hi as u32;
            mask[3] = (mask_hi >> 32) as u32;
            if vex_l == 1 {
                let hi2 = self.regs.ymm_high[xmm_src1][0];
                let hi3 = self.regs.ymm_high[xmm_src1][1];
                src1[4] = hi2 as u32;
                src1[5] = (hi2 >> 32) as u32;
                src1[6] = hi3 as u32;
                src1[7] = (hi3 >> 32) as u32;
                let mask_hi2 = self.regs.ymm_high[mask_reg][0];
                let mask_hi3 = self.regs.ymm_high[mask_reg][1];
                mask[4] = mask_hi2 as u32;
                mask[5] = (mask_hi2 >> 32) as u32;
                mask[6] = mask_hi3 as u32;
                mask[7] = (mask_hi3 >> 32) as u32;
            }

            if is_memory {
                for i in 0..count {
                    src2[i] = self.read_mem(addr + (i * 4) as u64, 4)? as u32;
                }
            } else {
                let xmm_src2 = rm as usize;
                let lo2 = self.regs.xmm[xmm_src2][0];
                let hi2 = self.regs.xmm[xmm_src2][1];
                src2[0] = lo2 as u32;
                src2[1] = (lo2 >> 32) as u32;
                src2[2] = hi2 as u32;
                src2[3] = (hi2 >> 32) as u32;
                if vex_l == 1 {
                    let hi3 = self.regs.ymm_high[xmm_src2][0];
                    let hi4 = self.regs.ymm_high[xmm_src2][1];
                    src2[4] = hi3 as u32;
                    src2[5] = (hi3 >> 32) as u32;
                    src2[6] = hi4 as u32;
                    src2[7] = (hi4 >> 32) as u32;
                }
            }

            let mut dst = [0u32; 8];
            for i in 0..count {
                let use_src2 = (mask[i] & 0x8000_0000) != 0;
                dst[i] = if use_src2 { src2[i] } else { src1[i] };
            }

            self.regs.xmm[xmm_dst][0] = (dst[0] as u64) | ((dst[1] as u64) << 32);
            self.regs.xmm[xmm_dst][1] = (dst[2] as u64) | ((dst[3] as u64) << 32);
            if vex_l == 1 {
                self.regs.ymm_high[xmm_dst][0] = (dst[4] as u64) | ((dst[5] as u64) << 32);
                self.regs.ymm_high[xmm_dst][1] = (dst[6] as u64) | ((dst[7] as u64) << 32);
            } else {
                self.regs.ymm_high[xmm_dst][0] = 0;
                self.regs.ymm_high[xmm_dst][1] = 0;
            }
        } else {
            // VBLENDVPD
            let count_q = if vex_l == 1 { 4 } else { 2 };
            let mut src1 = [0u64; 4];
            let mut src2 = [0u64; 4];
            let mut mask = [0u64; 4];
            src1[0] = self.regs.xmm[xmm_src1][0];
            src1[1] = self.regs.xmm[xmm_src1][1];
            mask[0] = self.regs.xmm[mask_reg][0];
            mask[1] = self.regs.xmm[mask_reg][1];
            if vex_l == 1 {
                src1[2] = self.regs.ymm_high[xmm_src1][0];
                src1[3] = self.regs.ymm_high[xmm_src1][1];
                mask[2] = self.regs.ymm_high[mask_reg][0];
                mask[3] = self.regs.ymm_high[mask_reg][1];
            }

            if is_memory {
                for i in 0..count_q {
                    src2[i] = self.read_mem(addr + (i * 8) as u64, 8)?;
                }
            } else {
                let xmm_src2 = rm as usize;
                src2[0] = self.regs.xmm[xmm_src2][0];
                src2[1] = self.regs.xmm[xmm_src2][1];
                if vex_l == 1 {
                    src2[2] = self.regs.ymm_high[xmm_src2][0];
                    src2[3] = self.regs.ymm_high[xmm_src2][1];
                }
            }

            let mut dst = [0u64; 4];
            for i in 0..count_q {
                let use_src2 = (mask[i] >> 63) != 0;
                dst[i] = if use_src2 { src2[i] } else { src1[i] };
            }

            self.regs.xmm[xmm_dst][0] = dst[0];
            self.regs.xmm[xmm_dst][1] = dst[1];
            if vex_l == 1 {
                self.regs.ymm_high[xmm_dst][0] = dst[2];
                self.regs.ymm_high[xmm_dst][1] = dst[3];
            } else {
                self.regs.ymm_high[xmm_dst][0] = 0;
                self.regs.ymm_high[xmm_dst][1] = 0;
            }
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }
}
