//! VEX integer instruction implementation for x86_64 emulator.

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::super::cpu::{InsnContext, X86_64Vcpu};

impl X86_64Vcpu {
    pub(in crate::backend::emulator::x86_64) fn execute_vex_pabs(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        opcode: u8,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;

        let (src_lo, src_hi) = if is_memory {
            (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
        } else {
            (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
        };

        let elem_bits = match opcode {
            0x1C => 8,  // PABSB
            0x1D => 16, // PABSW
            0x1E => 32, // PABSD
            _ => unreachable!(),
        };

        self.regs.xmm[xmm_dst][0] = self.pabs_64(src_lo, elem_bits);
        self.regs.xmm[xmm_dst][1] = self.pabs_64(src_hi, elem_bits);

        if vex_l == 1 {
            let (src_hi2, src_hi3) = if is_memory {
                (self.read_mem(addr + 16, 8)?, self.read_mem(addr + 24, 8)?)
            } else {
                (
                    self.regs.ymm_high[rm as usize][0],
                    self.regs.ymm_high[rm as usize][1],
                )
            };
            self.regs.ymm_high[xmm_dst][0] = self.pabs_64(src_hi2, elem_bits);
            self.regs.ymm_high[xmm_dst][1] = self.pabs_64(src_hi3, elem_bits);
        } else {
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    pub(in crate::backend::emulator::x86_64) fn pabs_64(&self, v: u64, elem_bits: u32) -> u64 {
        let elem_count = 64 / elem_bits;
        let mask = (1u64 << elem_bits) - 1;
        let mut result = 0u64;
        for i in 0..elem_count {
            let shift = i * elem_bits;
            let val = (v >> shift) & mask;
            let abs_val = match elem_bits {
                8 => (val as i8).wrapping_abs() as u8 as u64,
                16 => (val as i16).wrapping_abs() as u16 as u64,
                32 => (val as i32).wrapping_abs() as u32 as u64,
                _ => val,
            };
            result |= (abs_val & mask) << shift;
        }
        result
    }
}
