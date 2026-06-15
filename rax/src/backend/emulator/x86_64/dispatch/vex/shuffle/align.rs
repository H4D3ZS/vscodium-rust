//! VEX integer instruction implementation for x86_64 emulator.

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::super::cpu::{InsnContext, X86_64Vcpu};

impl X86_64Vcpu {
    pub(in crate::backend::emulator::x86_64) fn execute_vex_palignr(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let imm8 = ctx.consume_u8()? as usize;
        let xmm_dst = reg as usize;
        let xmm_src1 = vvvv as usize;

        let (src2_lo, src2_hi) = if is_memory {
            (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
        } else {
            (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
        };
        let src1_lo = self.regs.xmm[xmm_src1][0];
        let src1_hi = self.regs.xmm[xmm_src1][1];

        let (dst_lo, dst_hi) = self.palignr_lane(src1_lo, src1_hi, src2_lo, src2_hi, imm8);
        self.regs.xmm[xmm_dst][0] = dst_lo;
        self.regs.xmm[xmm_dst][1] = dst_hi;

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
            let (dst_hi2, dst_hi3) =
                self.palignr_lane(src1_hi2, src1_hi3, src2_hi2, src2_hi3, imm8);
            self.regs.ymm_high[xmm_dst][0] = dst_hi2;
            self.regs.ymm_high[xmm_dst][1] = dst_hi3;
        } else {
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    fn palignr_lane(
        &self,
        src1_lo: u64,
        src1_hi: u64,
        src2_lo: u64,
        src2_hi: u64,
        imm8: usize,
    ) -> (u64, u64) {
        if imm8 >= 32 {
            return (0, 0);
        }
        let mut bytes = [0u8; 32];
        bytes[0..8].copy_from_slice(&src2_lo.to_le_bytes());
        bytes[8..16].copy_from_slice(&src2_hi.to_le_bytes());
        bytes[16..24].copy_from_slice(&src1_lo.to_le_bytes());
        bytes[24..32].copy_from_slice(&src1_hi.to_le_bytes());

        let mut out = [0u8; 16];
        for i in 0..16 {
            let idx = imm8 + i;
            out[i] = if idx < 32 { bytes[idx] } else { 0 };
        }

        let lo = u64::from_le_bytes(out[0..8].try_into().unwrap());
        let hi = u64::from_le_bytes(out[8..16].try_into().unwrap());
        (lo, hi)
    }
}
