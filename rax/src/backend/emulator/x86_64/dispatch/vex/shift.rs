//! VEX instruction implementation for x86_64 emulator.

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::cpu::{InsnContext, X86_64Vcpu};
use super::super::super::insn;

impl X86_64Vcpu {
    pub(in crate::backend::emulator::x86_64) fn execute_vex_packed_shift_imm(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
        opcode: u8,
    ) -> Result<Option<VcpuExit>> {
        let modrm = ctx.peek_u8()?;
        let reg_op = (modrm >> 3) & 0x07;
        let (_, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let imm8 = ctx.consume_u8()?;

        let xmm_dst = vvvv as usize;
        let xmm_src = rm as usize;

        // Get source values
        let (src_lo, src_hi) = if is_memory {
            (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
        } else {
            (self.regs.xmm[xmm_src][0], self.regs.xmm[xmm_src][1])
        };

        match opcode {
            0x71 => {
                // VPSRLW/VPSRAW/VPSLLW - word shifts
                match reg_op {
                    2 => {
                        // VPSRLW: logical right shift words
                        let shift = imm8 as u32;
                        self.regs.xmm[xmm_dst][0] = self.shift_words_right(src_lo, shift, false);
                        self.regs.xmm[xmm_dst][1] = self.shift_words_right(src_hi, shift, false);
                    }
                    4 => {
                        // VPSRAW: arithmetic right shift words
                        let shift = imm8 as u32;
                        self.regs.xmm[xmm_dst][0] = self.shift_words_right(src_lo, shift, true);
                        self.regs.xmm[xmm_dst][1] = self.shift_words_right(src_hi, shift, true);
                    }
                    6 => {
                        // VPSLLW: logical left shift words
                        let shift = imm8 as u32;
                        self.regs.xmm[xmm_dst][0] = self.shift_words_left(src_lo, shift);
                        self.regs.xmm[xmm_dst][1] = self.shift_words_left(src_hi, shift);
                    }
                    _ => {
                        return Err(Error::Emulator(format!(
                            "unimplemented VEX.0F 71 /{} at RIP={:#x}",
                            reg_op, self.regs.rip
                        )));
                    }
                }
            }
            0x72 => {
                // VPSRLD/VPSRAD/VPSLLD - dword shifts
                match reg_op {
                    2 => {
                        // VPSRLD: logical right shift dwords
                        let shift = imm8 as u32;
                        self.regs.xmm[xmm_dst][0] = self.shift_dwords_right(src_lo, shift, false);
                        self.regs.xmm[xmm_dst][1] = self.shift_dwords_right(src_hi, shift, false);
                    }
                    4 => {
                        // VPSRAD: arithmetic right shift dwords
                        let shift = imm8 as u32;
                        self.regs.xmm[xmm_dst][0] = self.shift_dwords_right(src_lo, shift, true);
                        self.regs.xmm[xmm_dst][1] = self.shift_dwords_right(src_hi, shift, true);
                    }
                    6 => {
                        // VPSLLD: logical left shift dwords
                        let shift = imm8 as u32;
                        self.regs.xmm[xmm_dst][0] = self.shift_dwords_left(src_lo, shift);
                        self.regs.xmm[xmm_dst][1] = self.shift_dwords_left(src_hi, shift);
                    }
                    _ => {
                        return Err(Error::Emulator(format!(
                            "unimplemented VEX.0F 72 /{} at RIP={:#x}",
                            reg_op, self.regs.rip
                        )));
                    }
                }
            }
            0x73 => {
                // VPSRLQ/VPSRLDQ/VPSLLQ/VPSLLDQ - qword/dqword shifts
                match reg_op {
                    2 => {
                        // VPSRLQ: logical right shift qwords
                        let shift = imm8 as u32;
                        self.regs.xmm[xmm_dst][0] = if shift >= 64 { 0 } else { src_lo >> shift };
                        self.regs.xmm[xmm_dst][1] = if shift >= 64 { 0 } else { src_hi >> shift };
                    }
                    3 => {
                        // VPSRLDQ: byte shift right (within each 128-bit lane)
                        let shift = (imm8 as usize).min(16);
                        let (new_lo, new_hi) = self.byte_shift_right_128(src_lo, src_hi, shift);
                        self.regs.xmm[xmm_dst][0] = new_lo;
                        self.regs.xmm[xmm_dst][1] = new_hi;
                    }
                    6 => {
                        // VPSLLQ: logical left shift qwords
                        let shift = imm8 as u32;
                        self.regs.xmm[xmm_dst][0] = if shift >= 64 { 0 } else { src_lo << shift };
                        self.regs.xmm[xmm_dst][1] = if shift >= 64 { 0 } else { src_hi << shift };
                    }
                    7 => {
                        // VPSLLDQ: byte shift left (within each 128-bit lane)
                        let shift = (imm8 as usize).min(16);
                        let (new_lo, new_hi) = self.byte_shift_left_128(src_lo, src_hi, shift);
                        self.regs.xmm[xmm_dst][0] = new_lo;
                        self.regs.xmm[xmm_dst][1] = new_hi;
                    }
                    _ => {
                        return Err(Error::Emulator(format!(
                            "unimplemented VEX.0F 73 /{} at RIP={:#x}",
                            reg_op, self.regs.rip
                        )));
                    }
                }
            }
            _ => unreachable!(),
        }

        // Handle YMM (256-bit) if vex_l == 1
        if vex_l == 1 {
            let (src_hi2, src_hi3) = if is_memory {
                (self.read_mem(addr + 16, 8)?, self.read_mem(addr + 24, 8)?)
            } else {
                (
                    self.regs.ymm_high[xmm_src][0],
                    self.regs.ymm_high[xmm_src][1],
                )
            };

            match opcode {
                0x71 => match reg_op {
                    2 => {
                        let shift = imm8 as u32;
                        self.regs.ymm_high[xmm_dst][0] =
                            self.shift_words_right(src_hi2, shift, false);
                        self.regs.ymm_high[xmm_dst][1] =
                            self.shift_words_right(src_hi3, shift, false);
                    }
                    4 => {
                        let shift = imm8 as u32;
                        self.regs.ymm_high[xmm_dst][0] =
                            self.shift_words_right(src_hi2, shift, true);
                        self.regs.ymm_high[xmm_dst][1] =
                            self.shift_words_right(src_hi3, shift, true);
                    }
                    6 => {
                        let shift = imm8 as u32;
                        self.regs.ymm_high[xmm_dst][0] = self.shift_words_left(src_hi2, shift);
                        self.regs.ymm_high[xmm_dst][1] = self.shift_words_left(src_hi3, shift);
                    }
                    _ => {}
                },
                0x72 => match reg_op {
                    2 => {
                        let shift = imm8 as u32;
                        self.regs.ymm_high[xmm_dst][0] =
                            self.shift_dwords_right(src_hi2, shift, false);
                        self.regs.ymm_high[xmm_dst][1] =
                            self.shift_dwords_right(src_hi3, shift, false);
                    }
                    4 => {
                        let shift = imm8 as u32;
                        self.regs.ymm_high[xmm_dst][0] =
                            self.shift_dwords_right(src_hi2, shift, true);
                        self.regs.ymm_high[xmm_dst][1] =
                            self.shift_dwords_right(src_hi3, shift, true);
                    }
                    6 => {
                        let shift = imm8 as u32;
                        self.regs.ymm_high[xmm_dst][0] = self.shift_dwords_left(src_hi2, shift);
                        self.regs.ymm_high[xmm_dst][1] = self.shift_dwords_left(src_hi3, shift);
                    }
                    _ => {}
                },
                0x73 => match reg_op {
                    2 => {
                        let shift = imm8 as u32;
                        self.regs.ymm_high[xmm_dst][0] =
                            if shift >= 64 { 0 } else { src_hi2 >> shift };
                        self.regs.ymm_high[xmm_dst][1] =
                            if shift >= 64 { 0 } else { src_hi3 >> shift };
                    }
                    3 => {
                        let shift = (imm8 as usize).min(16);
                        let (new_lo, new_hi) = self.byte_shift_right_128(src_hi2, src_hi3, shift);
                        self.regs.ymm_high[xmm_dst][0] = new_lo;
                        self.regs.ymm_high[xmm_dst][1] = new_hi;
                    }
                    6 => {
                        let shift = imm8 as u32;
                        self.regs.ymm_high[xmm_dst][0] =
                            if shift >= 64 { 0 } else { src_hi2 << shift };
                        self.regs.ymm_high[xmm_dst][1] =
                            if shift >= 64 { 0 } else { src_hi3 << shift };
                    }
                    7 => {
                        let shift = (imm8 as usize).min(16);
                        let (new_lo, new_hi) = self.byte_shift_left_128(src_hi2, src_hi3, shift);
                        self.regs.ymm_high[xmm_dst][0] = new_lo;
                        self.regs.ymm_high[xmm_dst][1] = new_hi;
                    }
                    _ => {}
                },
                _ => {}
            }
        } else {
            // VEX.128 clears upper bits
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    // Helper: shift packed words left
    fn shift_words_left(&self, val: u64, shift: u32) -> u64 {
        if shift >= 16 {
            return 0;
        }
        let w0 = ((val as u16) << shift) as u64;
        let w1 = (((val >> 16) as u16) << shift) as u64;
        let w2 = (((val >> 32) as u16) << shift) as u64;
        let w3 = (((val >> 48) as u16) << shift) as u64;
        w0 | (w1 << 16) | (w2 << 32) | (w3 << 48)
    }

    // Helper: shift packed words right (logical or arithmetic)
    fn shift_words_right(&self, val: u64, shift: u32, arith: bool) -> u64 {
        if shift >= 16 {
            return if arith {
                let sign0 = if (val as i16) < 0 { 0xFFFFu64 } else { 0 };
                let sign1 = if ((val >> 16) as i16) < 0 {
                    0xFFFFu64
                } else {
                    0
                };
                let sign2 = if ((val >> 32) as i16) < 0 {
                    0xFFFFu64
                } else {
                    0
                };
                let sign3 = if ((val >> 48) as i16) < 0 {
                    0xFFFFu64
                } else {
                    0
                };
                sign0 | (sign1 << 16) | (sign2 << 32) | (sign3 << 48)
            } else {
                0
            };
        }
        if arith {
            let w0 = (((val as i16) >> shift) as u16) as u64;
            let w1 = ((((val >> 16) as i16) >> shift) as u16) as u64;
            let w2 = ((((val >> 32) as i16) >> shift) as u16) as u64;
            let w3 = ((((val >> 48) as i16) >> shift) as u16) as u64;
            w0 | (w1 << 16) | (w2 << 32) | (w3 << 48)
        } else {
            let w0 = ((val as u16) >> shift) as u64;
            let w1 = (((val >> 16) as u16) >> shift) as u64;
            let w2 = (((val >> 32) as u16) >> shift) as u64;
            let w3 = (((val >> 48) as u16) >> shift) as u64;
            w0 | (w1 << 16) | (w2 << 32) | (w3 << 48)
        }
    }

    // Helper: shift packed dwords left
    fn shift_dwords_left(&self, val: u64, shift: u32) -> u64 {
        if shift >= 32 {
            return 0;
        }
        let d0 = ((val as u32) << shift) as u64;
        let d1 = (((val >> 32) as u32) << shift) as u64;
        d0 | (d1 << 32)
    }

    // Helper: shift packed dwords right (logical or arithmetic)
    fn shift_dwords_right(&self, val: u64, shift: u32, arith: bool) -> u64 {
        if shift >= 32 {
            return if arith {
                let sign0 = if (val as i32) < 0 { 0xFFFFFFFFu64 } else { 0 };
                let sign1 = if ((val >> 32) as i32) < 0 {
                    0xFFFFFFFFu64
                } else {
                    0
                };
                sign0 | (sign1 << 32)
            } else {
                0
            };
        }
        if arith {
            let d0 = (((val as i32) >> shift) as u32) as u64;
            let d1 = ((((val >> 32) as i32) >> shift) as u32) as u64;
            d0 | (d1 << 32)
        } else {
            let d0 = ((val as u32) >> shift) as u64;
            let d1 = (((val >> 32) as u32) >> shift) as u64;
            d0 | (d1 << 32)
        }
    }

    // Helper: byte shift right within 128-bit value
    fn byte_shift_right_128(&self, lo: u64, hi: u64, shift: usize) -> (u64, u64) {
        if shift >= 16 {
            return (0, 0);
        }
        let shift_bits = shift * 8;
        if shift == 0 {
            (lo, hi)
        } else if shift < 8 {
            let new_lo = (lo >> shift_bits) | (hi << (64 - shift_bits));
            let new_hi = hi >> shift_bits;
            (new_lo, new_hi)
        } else {
            let shift_bits = (shift - 8) * 8;
            let new_lo = hi >> shift_bits;
            (new_lo, 0)
        }
    }

    // Helper: byte shift left within 128-bit value
    fn byte_shift_left_128(&self, lo: u64, hi: u64, shift: usize) -> (u64, u64) {
        if shift >= 16 {
            return (0, 0);
        }
        let shift_bits = shift * 8;
        if shift == 0 {
            (lo, hi)
        } else if shift < 8 {
            let new_hi = (hi << shift_bits) | (lo >> (64 - shift_bits));
            let new_lo = lo << shift_bits;
            (new_lo, new_hi)
        } else {
            let shift_bits = (shift - 8) * 8;
            let new_hi = lo << shift_bits;
            (0, new_hi)
        }
    }

    /// VEX packed integer shift by XMM count
    /// VPSRLW/D/Q, VPSRAW/D, VPSLLW/D/Q
    pub(in crate::backend::emulator::x86_64) fn execute_vex_packed_shift_xmm(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
        opcode: u8,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;
        let xmm_src = vvvv as usize;

        // Get shift count from xmm/m128 (only low 64 bits used)
        let count = if is_memory {
            self.read_mem(addr, 8)?
        } else {
            self.regs.xmm[rm as usize][0]
        };

        // Get source data to be shifted
        let src_lo = self.regs.xmm[xmm_src][0];
        let src_hi = self.regs.xmm[xmm_src][1];

        // Apply shift based on opcode
        match opcode {
            // VPSRLW: logical right shift words
            0xD1 => {
                let shift = count.min(16) as u32;
                self.regs.xmm[xmm_dst][0] = self.shift_words_right(src_lo, shift, false);
                self.regs.xmm[xmm_dst][1] = self.shift_words_right(src_hi, shift, false);
            }
            // VPSRLD: logical right shift dwords
            0xD2 => {
                let shift = count.min(32) as u32;
                self.regs.xmm[xmm_dst][0] = self.shift_dwords_right(src_lo, shift, false);
                self.regs.xmm[xmm_dst][1] = self.shift_dwords_right(src_hi, shift, false);
            }
            // VPSRLQ: logical right shift qwords
            0xD3 => {
                let shift = count.min(64) as u32;
                self.regs.xmm[xmm_dst][0] = if shift >= 64 { 0 } else { src_lo >> shift };
                self.regs.xmm[xmm_dst][1] = if shift >= 64 { 0 } else { src_hi >> shift };
            }
            // VPSRAW: arithmetic right shift words
            0xE1 => {
                let shift = count.min(16) as u32;
                self.regs.xmm[xmm_dst][0] = self.shift_words_right(src_lo, shift, true);
                self.regs.xmm[xmm_dst][1] = self.shift_words_right(src_hi, shift, true);
            }
            // VPSRAD: arithmetic right shift dwords
            0xE2 => {
                let shift = count.min(32) as u32;
                self.regs.xmm[xmm_dst][0] = self.shift_dwords_right(src_lo, shift, true);
                self.regs.xmm[xmm_dst][1] = self.shift_dwords_right(src_hi, shift, true);
            }
            // VPSLLW: logical left shift words
            0xF1 => {
                let shift = count.min(16) as u32;
                self.regs.xmm[xmm_dst][0] = self.shift_words_left(src_lo, shift);
                self.regs.xmm[xmm_dst][1] = self.shift_words_left(src_hi, shift);
            }
            // VPSLLD: logical left shift dwords
            0xF2 => {
                let shift = count.min(32) as u32;
                self.regs.xmm[xmm_dst][0] = self.shift_dwords_left(src_lo, shift);
                self.regs.xmm[xmm_dst][1] = self.shift_dwords_left(src_hi, shift);
            }
            // VPSLLQ: logical left shift qwords
            0xF3 => {
                let shift = count.min(64) as u32;
                self.regs.xmm[xmm_dst][0] = if shift >= 64 { 0 } else { src_lo << shift };
                self.regs.xmm[xmm_dst][1] = if shift >= 64 { 0 } else { src_hi << shift };
            }
            _ => unreachable!(),
        }

        // Handle YMM (256-bit) if vex_l == 1
        if vex_l == 1 {
            let src_hi2 = self.regs.ymm_high[xmm_src][0];
            let src_hi3 = self.regs.ymm_high[xmm_src][1];

            match opcode {
                0xD1 => {
                    let shift = count.min(16) as u32;
                    self.regs.ymm_high[xmm_dst][0] = self.shift_words_right(src_hi2, shift, false);
                    self.regs.ymm_high[xmm_dst][1] = self.shift_words_right(src_hi3, shift, false);
                }
                0xD2 => {
                    let shift = count.min(32) as u32;
                    self.regs.ymm_high[xmm_dst][0] = self.shift_dwords_right(src_hi2, shift, false);
                    self.regs.ymm_high[xmm_dst][1] = self.shift_dwords_right(src_hi3, shift, false);
                }
                0xD3 => {
                    let shift = count.min(64) as u32;
                    self.regs.ymm_high[xmm_dst][0] = if shift >= 64 { 0 } else { src_hi2 >> shift };
                    self.regs.ymm_high[xmm_dst][1] = if shift >= 64 { 0 } else { src_hi3 >> shift };
                }
                0xE1 => {
                    let shift = count.min(16) as u32;
                    self.regs.ymm_high[xmm_dst][0] = self.shift_words_right(src_hi2, shift, true);
                    self.regs.ymm_high[xmm_dst][1] = self.shift_words_right(src_hi3, shift, true);
                }
                0xE2 => {
                    let shift = count.min(32) as u32;
                    self.regs.ymm_high[xmm_dst][0] = self.shift_dwords_right(src_hi2, shift, true);
                    self.regs.ymm_high[xmm_dst][1] = self.shift_dwords_right(src_hi3, shift, true);
                }
                0xF1 => {
                    let shift = count.min(16) as u32;
                    self.regs.ymm_high[xmm_dst][0] = self.shift_words_left(src_hi2, shift);
                    self.regs.ymm_high[xmm_dst][1] = self.shift_words_left(src_hi3, shift);
                }
                0xF2 => {
                    let shift = count.min(32) as u32;
                    self.regs.ymm_high[xmm_dst][0] = self.shift_dwords_left(src_hi2, shift);
                    self.regs.ymm_high[xmm_dst][1] = self.shift_dwords_left(src_hi3, shift);
                }
                0xF3 => {
                    let shift = count.min(64) as u32;
                    self.regs.ymm_high[xmm_dst][0] = if shift >= 64 { 0 } else { src_hi2 << shift };
                    self.regs.ymm_high[xmm_dst][1] = if shift >= 64 { 0 } else { src_hi3 << shift };
                }
                _ => {}
            }
        } else {
            // VEX.128 clears upper bits
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// VEX variable shift instructions (per-element shift counts)
    /// VPSRLVD/Q, VPSRAVD, VPSLLVD/Q
    pub(in crate::backend::emulator::x86_64) fn execute_vex_variable_shift(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
        vex_w: u8,
        opcode: u8,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;
        let xmm_src = vvvv as usize;

        // Get shift counts from xmm/m128 or ymm/m256
        let (count_lo, count_hi) = if is_memory {
            (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
        } else {
            (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
        };

        // Get source data
        let src_lo = self.regs.xmm[xmm_src][0];
        let src_hi = self.regs.xmm[xmm_src][1];

        if vex_w == 0 {
            // Dword operations
            match opcode {
                0x45 => {
                    // VPSRLVD: variable right logical shift dwords
                    self.regs.xmm[xmm_dst][0] =
                        self.variable_shift_dwords(src_lo, count_lo, false, false);
                    self.regs.xmm[xmm_dst][1] =
                        self.variable_shift_dwords(src_hi, count_hi, false, false);
                }
                0x46 => {
                    // VPSRAVD: variable right arithmetic shift dwords
                    self.regs.xmm[xmm_dst][0] =
                        self.variable_shift_dwords(src_lo, count_lo, false, true);
                    self.regs.xmm[xmm_dst][1] =
                        self.variable_shift_dwords(src_hi, count_hi, false, true);
                }
                0x47 => {
                    // VPSLLVD: variable left shift dwords
                    self.regs.xmm[xmm_dst][0] =
                        self.variable_shift_dwords(src_lo, count_lo, true, false);
                    self.regs.xmm[xmm_dst][1] =
                        self.variable_shift_dwords(src_hi, count_hi, true, false);
                }
                _ => unreachable!(),
            }
        } else {
            // Qword operations
            match opcode {
                0x45 => {
                    // VPSRLVQ: variable right logical shift qwords
                    self.regs.xmm[xmm_dst][0] = self.variable_shift_qword(src_lo, count_lo, false);
                    self.regs.xmm[xmm_dst][1] = self.variable_shift_qword(src_hi, count_hi, false);
                }
                0x47 => {
                    // VPSLLVQ: variable left shift qwords
                    self.regs.xmm[xmm_dst][0] = self.variable_shift_qword(src_lo, count_lo, true);
                    self.regs.xmm[xmm_dst][1] = self.variable_shift_qword(src_hi, count_hi, true);
                }
                _ => {
                    return Err(Error::Emulator(format!(
                        "VPSRAVQ (W1 opcode 0x46) not supported in VEX (AVX2)"
                    )));
                }
            }
        }

        // Handle YMM (256-bit)
        if vex_l == 1 {
            let (count_hi2, count_hi3) = if is_memory {
                (self.read_mem(addr + 16, 8)?, self.read_mem(addr + 24, 8)?)
            } else {
                (
                    self.regs.ymm_high[rm as usize][0],
                    self.regs.ymm_high[rm as usize][1],
                )
            };
            let src_hi2 = self.regs.ymm_high[xmm_src][0];
            let src_hi3 = self.regs.ymm_high[xmm_src][1];

            if vex_w == 0 {
                match opcode {
                    0x45 => {
                        self.regs.ymm_high[xmm_dst][0] =
                            self.variable_shift_dwords(src_hi2, count_hi2, false, false);
                        self.regs.ymm_high[xmm_dst][1] =
                            self.variable_shift_dwords(src_hi3, count_hi3, false, false);
                    }
                    0x46 => {
                        self.regs.ymm_high[xmm_dst][0] =
                            self.variable_shift_dwords(src_hi2, count_hi2, false, true);
                        self.regs.ymm_high[xmm_dst][1] =
                            self.variable_shift_dwords(src_hi3, count_hi3, false, true);
                    }
                    0x47 => {
                        self.regs.ymm_high[xmm_dst][0] =
                            self.variable_shift_dwords(src_hi2, count_hi2, true, false);
                        self.regs.ymm_high[xmm_dst][1] =
                            self.variable_shift_dwords(src_hi3, count_hi3, true, false);
                    }
                    _ => {}
                }
            } else {
                match opcode {
                    0x45 => {
                        self.regs.ymm_high[xmm_dst][0] =
                            self.variable_shift_qword(src_hi2, count_hi2, false);
                        self.regs.ymm_high[xmm_dst][1] =
                            self.variable_shift_qword(src_hi3, count_hi3, false);
                    }
                    0x47 => {
                        self.regs.ymm_high[xmm_dst][0] =
                            self.variable_shift_qword(src_hi2, count_hi2, true);
                        self.regs.ymm_high[xmm_dst][1] =
                            self.variable_shift_qword(src_hi3, count_hi3, true);
                    }
                    _ => {}
                }
            }
        } else {
            // VEX.128 clears upper bits
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    // Helper: variable shift two packed dwords
    // val = two dwords, counts = corresponding shift counts
    // left = true for left shift, arith = true for arithmetic right shift
    fn variable_shift_dwords(&self, val: u64, counts: u64, left: bool, arith: bool) -> u64 {
        let d0 = val as u32;
        let d1 = (val >> 32) as u32;
        let c0 = (counts as u32).min(32);
        let c1 = ((counts >> 32) as u32).min(32);

        let r0 = if left {
            if c0 >= 32 { 0 } else { d0 << c0 }
        } else if arith {
            if c0 >= 32 {
                if (d0 as i32) < 0 { 0xFFFFFFFF } else { 0 }
            } else {
                ((d0 as i32) >> c0) as u32
            }
        } else {
            if c0 >= 32 { 0 } else { d0 >> c0 }
        };

        let r1 = if left {
            if c1 >= 32 { 0 } else { d1 << c1 }
        } else if arith {
            if c1 >= 32 {
                if (d1 as i32) < 0 { 0xFFFFFFFF } else { 0 }
            } else {
                ((d1 as i32) >> c1) as u32
            }
        } else {
            if c1 >= 32 { 0 } else { d1 >> c1 }
        };

        (r0 as u64) | ((r1 as u64) << 32)
    }

    // Helper: variable shift a qword
    fn variable_shift_qword(&self, val: u64, count: u64, left: bool) -> u64 {
        let c = count.min(64) as u32;
        if left {
            if c >= 64 { 0 } else { val << c }
        } else {
            if c >= 64 { 0 } else { val >> c }
        }
    }
}
