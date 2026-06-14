//! VEX gather instruction implementation for x86_64 emulator.

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::cpu::{InsnContext, X86_64Vcpu};

impl X86_64Vcpu {
    pub(in crate::backend::emulator::x86_64) fn execute_vex_gather(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vex_w: u8,
        vvvv: u8,
        opcode: u8,
    ) -> Result<Option<VcpuExit>> {
        let (dst_reg, index_reg, base_addr, scale) = self.decode_vsib(ctx)?;
        let mask_reg = vvvv as usize;

        let index_size = match opcode {
            0x90 | 0x92 => 4,
            0x91 | 0x93 => 8,
            _ => unreachable!(),
        };
        let data_size = if vex_w == 0 { 4 } else { 8 };
        let full_count = match (data_size, vex_l) {
            (4, 0) => 4,
            (4, _) => 8,
            (8, 0) => 2,
            (8, _) => 4,
            _ => 0,
        };
        let elem_count = if index_size > data_size {
            full_count / 2
        } else {
            full_count
        };

        let indices: Vec<i64> = if index_size == 4 {
            self.read_indices_i32(index_reg, vex_l).to_vec()
        } else {
            self.read_indices_i64(index_reg, vex_l).to_vec()
        };

        if data_size == 4 {
            let mut dest = self.read_dwords(dst_reg, vex_l);
            let mut mask = self.read_mask_dwords(mask_reg, vex_l);
            for i in 0..elem_count {
                if (mask[i] & 0x8000_0000) != 0 {
                    let offset = (indices[i] as i128 * scale as i128) as i64;
                    let addr = (base_addr as i64).wrapping_add(offset) as u64;
                    dest[i] = self.read_mem(addr, 4)? as u32;
                    mask[i] = 0;
                }
            }
            if index_size > data_size {
                for i in elem_count..full_count {
                    dest[i] = 0;
                    mask[i] = 0;
                }
            }
            self.write_dwords(dst_reg, &dest, vex_l);
            self.write_mask_dwords(mask_reg, &mask, vex_l);
        } else {
            let mut dest = self.read_qwords(dst_reg, vex_l);
            let mut mask = self.read_mask_qwords(mask_reg, vex_l);
            for i in 0..elem_count {
                if (mask[i] & 0x8000_0000_0000_0000) != 0 {
                    let offset = (indices[i] as i128 * scale as i128) as i64;
                    let addr = (base_addr as i64).wrapping_add(offset) as u64;
                    dest[i] = self.read_mem(addr, 8)?;
                    mask[i] = 0;
                }
            }
            if index_size > data_size {
                for i in elem_count..full_count {
                    dest[i] = 0;
                    mask[i] = 0;
                }
            }
            self.write_qwords(dst_reg, &dest, vex_l);
            self.write_mask_qwords(mask_reg, &mask, vex_l);
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    fn decode_vsib(&self, ctx: &mut InsnContext) -> Result<(usize, usize, u64, i64)> {
        let modrm = ctx.consume_u8()?;
        let mod_bits = modrm >> 6;
        if mod_bits == 3 {
            return Err(Error::Emulator("VSIB requires memory operand".to_string()));
        }
        let reg = ((modrm >> 3) & 0x07) | ctx.rex_r();
        let rm_field = modrm & 0x07;
        if rm_field != 4 {
            return Err(Error::Emulator("VSIB requires SIB byte".to_string()));
        }

        let sib = ctx.consume_u8()?;
        let scale = 1i64 << (sib >> 6);
        let index = ((sib >> 3) & 0x07) | (ctx.rex.map_or(0, |r| (r & 0x02) << 2));
        let base_reg = (sib & 0x07) | ctx.rex_b();

        let mut base = if base_reg == 5 && mod_bits == 0 {
            0
        } else {
            self.get_reg(base_reg, 8) as i64
        };

        match mod_bits {
            0 => {
                if base_reg == 5 {
                    let disp = ctx.consume_u32()? as i32 as i64;
                    base = base.wrapping_add(disp);
                }
            }
            1 => {
                let disp = ctx.consume_u8()? as i8 as i64;
                base = base.wrapping_add(disp);
            }
            2 => {
                let disp = ctx.consume_u32()? as i32 as i64;
                base = base.wrapping_add(disp);
            }
            _ => {}
        }

        Ok((reg as usize, index as usize, base as u64, scale))
    }

    fn read_indices_i32(&self, reg: usize, vex_l: u8) -> [i64; 8] {
        let mut out = [0i64; 8];
        let lo0 = self.regs.xmm[reg][0];
        let lo1 = self.regs.xmm[reg][1];
        out[0] = (lo0 as u32 as i32) as i64;
        out[1] = ((lo0 >> 32) as u32 as i32) as i64;
        out[2] = (lo1 as u32 as i32) as i64;
        out[3] = ((lo1 >> 32) as u32 as i32) as i64;
        if vex_l == 1 {
            let hi0 = self.regs.ymm_high[reg][0];
            let hi1 = self.regs.ymm_high[reg][1];
            out[4] = (hi0 as u32 as i32) as i64;
            out[5] = ((hi0 >> 32) as u32 as i32) as i64;
            out[6] = (hi1 as u32 as i32) as i64;
            out[7] = ((hi1 >> 32) as u32 as i32) as i64;
        }
        out
    }

    fn read_indices_i64(&self, reg: usize, vex_l: u8) -> [i64; 4] {
        let mut out = [0i64; 4];
        out[0] = self.regs.xmm[reg][0] as i64;
        out[1] = self.regs.xmm[reg][1] as i64;
        if vex_l == 1 {
            out[2] = self.regs.ymm_high[reg][0] as i64;
            out[3] = self.regs.ymm_high[reg][1] as i64;
        }
        out
    }

    fn read_dwords(&self, reg: usize, vex_l: u8) -> [u32; 8] {
        let mut out = [0u32; 8];
        let lo0 = self.regs.xmm[reg][0];
        let lo1 = self.regs.xmm[reg][1];
        out[0] = lo0 as u32;
        out[1] = (lo0 >> 32) as u32;
        out[2] = lo1 as u32;
        out[3] = (lo1 >> 32) as u32;
        if vex_l == 1 {
            let hi0 = self.regs.ymm_high[reg][0];
            let hi1 = self.regs.ymm_high[reg][1];
            out[4] = hi0 as u32;
            out[5] = (hi0 >> 32) as u32;
            out[6] = hi1 as u32;
            out[7] = (hi1 >> 32) as u32;
        }
        out
    }

    fn write_dwords(&mut self, reg: usize, values: &[u32; 8], vex_l: u8) {
        self.regs.xmm[reg][0] = (values[0] as u64) | ((values[1] as u64) << 32);
        self.regs.xmm[reg][1] = (values[2] as u64) | ((values[3] as u64) << 32);
        if vex_l == 1 {
            self.regs.ymm_high[reg][0] = (values[4] as u64) | ((values[5] as u64) << 32);
            self.regs.ymm_high[reg][1] = (values[6] as u64) | ((values[7] as u64) << 32);
        } else {
            self.regs.ymm_high[reg][0] = 0;
            self.regs.ymm_high[reg][1] = 0;
        }
    }

    fn read_qwords(&self, reg: usize, vex_l: u8) -> [u64; 4] {
        let mut out = [0u64; 4];
        out[0] = self.regs.xmm[reg][0];
        out[1] = self.regs.xmm[reg][1];
        if vex_l == 1 {
            out[2] = self.regs.ymm_high[reg][0];
            out[3] = self.regs.ymm_high[reg][1];
        }
        out
    }

    fn write_qwords(&mut self, reg: usize, values: &[u64; 4], vex_l: u8) {
        self.regs.xmm[reg][0] = values[0];
        self.regs.xmm[reg][1] = values[1];
        if vex_l == 1 {
            self.regs.ymm_high[reg][0] = values[2];
            self.regs.ymm_high[reg][1] = values[3];
        } else {
            self.regs.ymm_high[reg][0] = 0;
            self.regs.ymm_high[reg][1] = 0;
        }
    }

    fn read_mask_dwords(&self, reg: usize, vex_l: u8) -> [u32; 8] {
        self.read_dwords(reg, vex_l)
    }

    fn write_mask_dwords(&mut self, reg: usize, values: &[u32; 8], vex_l: u8) {
        self.write_dwords(reg, values, vex_l);
    }

    fn read_mask_qwords(&self, reg: usize, vex_l: u8) -> [u64; 4] {
        self.read_qwords(reg, vex_l)
    }

    fn write_mask_qwords(&mut self, reg: usize, values: &[u64; 4], vex_l: u8) {
        self.write_qwords(reg, values, vex_l);
    }
}
