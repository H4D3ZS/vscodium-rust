//! VEX integer instruction implementation for x86_64 emulator.

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::super::cpu::{InsnContext, X86_64Vcpu};

impl X86_64Vcpu {
    pub(in crate::backend::emulator::x86_64) fn execute_vex_pmuldq(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;
        let xmm_src1 = vvvv as usize;

        let (src2_lo, src2_hi) = if is_memory {
            (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
        } else {
            (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
        };
        let src1_lo = self.regs.xmm[xmm_src1][0];
        let src1_hi = self.regs.xmm[xmm_src1][1];

        // Use low dword of each qword
        let a0 = src1_lo as i32 as i64;
        let b0 = src2_lo as i32 as i64;
        let a1 = src1_hi as i32 as i64;
        let b1 = src2_hi as i32 as i64;

        self.regs.xmm[xmm_dst][0] = (a0 * b0) as u64;
        self.regs.xmm[xmm_dst][1] = (a1 * b1) as u64;

        if vex_l == 1 {
            let (src2_hi2, src2_hi3) = if is_memory {
                (self.read_mem(addr + 16, 8)?, self.read_mem(addr + 24, 8)?)
            } else {
                (
                    self.regs.ymm_high[rm as usize][0],
                    self.regs.ymm_high[rm as usize][1],
                )
            };
            let src1_hi2 = self.regs.ymm_high[xmm_src1][0];
            let src1_hi3 = self.regs.ymm_high[xmm_src1][1];
            let a2 = src1_hi2 as i32 as i64;
            let b2 = src2_hi2 as i32 as i64;
            let a3 = src1_hi3 as i32 as i64;
            let b3 = src2_hi3 as i32 as i64;
            self.regs.ymm_high[xmm_dst][0] = (a2 * b2) as u64;
            self.regs.ymm_high[xmm_dst][1] = (a3 * b3) as u64;
        } else {
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_pmulld(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;
        let xmm_src1 = vvvv as usize;

        let (src2_lo, src2_hi) = if is_memory {
            (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
        } else {
            (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
        };
        let src1_lo = self.regs.xmm[xmm_src1][0];
        let src1_hi = self.regs.xmm[xmm_src1][1];

        self.regs.xmm[xmm_dst][0] = self.pmulld_64(src1_lo, src2_lo);
        self.regs.xmm[xmm_dst][1] = self.pmulld_64(src1_hi, src2_hi);

        if vex_l == 1 {
            let (src2_hi2, src2_hi3) = if is_memory {
                (self.read_mem(addr + 16, 8)?, self.read_mem(addr + 24, 8)?)
            } else {
                (
                    self.regs.ymm_high[rm as usize][0],
                    self.regs.ymm_high[rm as usize][1],
                )
            };
            let src1_hi2 = self.regs.ymm_high[xmm_src1][0];
            let src1_hi3 = self.regs.ymm_high[xmm_src1][1];
            self.regs.ymm_high[xmm_dst][0] = self.pmulld_64(src1_hi2, src2_hi2);
            self.regs.ymm_high[xmm_dst][1] = self.pmulld_64(src1_hi3, src2_hi3);
        } else {
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    fn pmulld_64(&self, a: u64, b: u64) -> u64 {
        let a0 = a as u32;
        let a1 = (a >> 32) as u32;
        let b0 = b as u32;
        let b1 = (b >> 32) as u32;
        let r0 = a0.wrapping_mul(b0);
        let r1 = a1.wrapping_mul(b1);
        (r0 as u64) | ((r1 as u64) << 32)
    }
}
