//! VEX compare instruction implementations.

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::cpu::{InsnContext, X86_64Vcpu};
use super::super::super::flags;

impl X86_64Vcpu {
    fn execute_vex_comis_common(
        &mut self,
        ctx: &mut InsnContext,
        vex_pp: u8,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;

        let (unordered, greater, less) = if vex_pp == 1 {
            let a = f64::from_bits(self.regs.xmm[xmm_dst][0]);
            let b = if is_memory {
                f64::from_bits(self.read_mem(addr, 8)?)
            } else {
                f64::from_bits(self.regs.xmm[rm as usize][0])
            };
            (a.is_nan() || b.is_nan(), a > b, a < b)
        } else {
            let a = f32::from_bits(self.regs.xmm[xmm_dst][0] as u32);
            let b = if is_memory {
                f32::from_bits(self.read_mem(addr, 4)? as u32)
            } else {
                f32::from_bits(self.regs.xmm[rm as usize][0] as u32)
            };
            (a.is_nan() || b.is_nan(), a > b, a < b)
        };

        // Clear lazy flags before setting flags directly
        self.clear_lazy_flags();

        let clear_mask = flags::bits::ZF
            | flags::bits::PF
            | flags::bits::CF
            | flags::bits::OF
            | flags::bits::AF
            | flags::bits::SF;
        self.regs.rflags &= !clear_mask;

        if unordered {
            self.regs.rflags |= flags::bits::ZF | flags::bits::PF | flags::bits::CF;
        } else if greater {
            // ZF=PF=CF=0
        } else if less {
            self.regs.rflags |= flags::bits::CF;
        } else {
            self.regs.rflags |= flags::bits::ZF;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_comis(
        &mut self,
        ctx: &mut InsnContext,
        vex_pp: u8,
    ) -> Result<Option<VcpuExit>> {
        self.execute_vex_comis_common(ctx, vex_pp)
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_ucomis(
        &mut self,
        ctx: &mut InsnContext,
        vex_pp: u8,
    ) -> Result<Option<VcpuExit>> {
        self.execute_vex_comis_common(ctx, vex_pp)
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_vtest(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
        opcode: u8,
    ) -> Result<Option<VcpuExit>> {
        if vvvv != 0 {
            return Err(Error::Emulator("VTEST requires VEX.vvvv=1111b".to_string()));
        }
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_src1 = reg as usize;
        let count = if vex_l == 1 { 8 } else { 4 };

        let (mask1, mask2) = if opcode == 0x0E {
            // VTESTPS
            let mut m1 = 0u32;
            let mut m2 = 0u32;
            let lo1 = self.regs.xmm[xmm_src1][0];
            let hi1 = self.regs.xmm[xmm_src1][1];
            m1 |= ((lo1 >> 31) & 1) as u32;
            m1 |= (((lo1 >> 63) & 1) as u32) << 1;
            m1 |= (((hi1 >> 31) & 1) as u32) << 2;
            m1 |= (((hi1 >> 63) & 1) as u32) << 3;
            if vex_l == 1 {
                let hi2 = self.regs.ymm_high[xmm_src1][0];
                let hi3 = self.regs.ymm_high[xmm_src1][1];
                m1 |= (((hi2 >> 31) & 1) as u32) << 4;
                m1 |= (((hi2 >> 63) & 1) as u32) << 5;
                m1 |= (((hi3 >> 31) & 1) as u32) << 6;
                m1 |= (((hi3 >> 63) & 1) as u32) << 7;
            }

            if is_memory {
                for i in 0..count {
                    let val = self.read_mem(addr + (i * 4) as u64, 4)? as u32;
                    if (val & 0x8000_0000) != 0 {
                        m2 |= 1u32 << i;
                    }
                }
            } else {
                let xmm_src2 = rm as usize;
                let lo2 = self.regs.xmm[xmm_src2][0];
                let hi2 = self.regs.xmm[xmm_src2][1];
                m2 |= ((lo2 >> 31) & 1) as u32;
                m2 |= (((lo2 >> 63) & 1) as u32) << 1;
                m2 |= (((hi2 >> 31) & 1) as u32) << 2;
                m2 |= (((hi2 >> 63) & 1) as u32) << 3;
                if vex_l == 1 {
                    let hi3 = self.regs.ymm_high[xmm_src2][0];
                    let hi4 = self.regs.ymm_high[xmm_src2][1];
                    m2 |= (((hi3 >> 31) & 1) as u32) << 4;
                    m2 |= (((hi3 >> 63) & 1) as u32) << 5;
                    m2 |= (((hi4 >> 31) & 1) as u32) << 6;
                    m2 |= (((hi4 >> 63) & 1) as u32) << 7;
                }
            }
            (m1, m2)
        } else {
            // VTESTPD
            let count_q = if vex_l == 1 { 4 } else { 2 };
            let mut m1 = 0u32;
            let mut m2 = 0u32;
            let lo1 = self.regs.xmm[xmm_src1][0];
            let hi1 = self.regs.xmm[xmm_src1][1];
            m1 |= ((lo1 >> 63) & 1) as u32;
            m1 |= (((hi1 >> 63) & 1) as u32) << 1;
            if vex_l == 1 {
                let hi2 = self.regs.ymm_high[xmm_src1][0];
                let hi3 = self.regs.ymm_high[xmm_src1][1];
                m1 |= (((hi2 >> 63) & 1) as u32) << 2;
                m1 |= (((hi3 >> 63) & 1) as u32) << 3;
            }

            if is_memory {
                for i in 0..count_q {
                    let val = self.read_mem(addr + (i * 8) as u64, 8)?;
                    if (val >> 63) != 0 {
                        m2 |= 1u32 << i;
                    }
                }
            } else {
                let xmm_src2 = rm as usize;
                let lo2 = self.regs.xmm[xmm_src2][0];
                let hi2 = self.regs.xmm[xmm_src2][1];
                m2 |= ((lo2 >> 63) & 1) as u32;
                m2 |= (((hi2 >> 63) & 1) as u32) << 1;
                if vex_l == 1 {
                    let hi3 = self.regs.ymm_high[xmm_src2][0];
                    let hi4 = self.regs.ymm_high[xmm_src2][1];
                    m2 |= (((hi3 >> 63) & 1) as u32) << 2;
                    m2 |= (((hi4 >> 63) & 1) as u32) << 3;
                }
            }
            let mask = if count_q == 4 { 0xF } else { 0x3 };
            return self.finish_vtest(m1 & mask, m2 & mask, ctx);
        };

        let mask = if count == 8 { 0xFF } else { 0x0F };
        self.finish_vtest(mask1 & mask, mask2 & mask, ctx)
    }

    fn finish_vtest(
        &mut self,
        mask1: u32,
        mask2: u32,
        ctx: &mut InsnContext,
    ) -> Result<Option<VcpuExit>> {
        let and = mask1 & mask2;
        let andn = mask1 & !mask2;

        // Clear lazy flags before setting flags directly
        self.clear_lazy_flags();

        self.regs.rflags &= !(flags::bits::AF
            | flags::bits::OF
            | flags::bits::PF
            | flags::bits::SF
            | flags::bits::ZF
            | flags::bits::CF);
        if and == 0 {
            self.regs.rflags |= flags::bits::ZF;
        }
        if andn == 0 {
            self.regs.rflags |= flags::bits::CF;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }
}
