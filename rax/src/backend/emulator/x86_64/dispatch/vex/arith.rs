//! VEX instruction implementation for x86_64 emulator.

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::cpu::{InsnContext, X86_64Vcpu};
use super::super::super::insn;

impl X86_64Vcpu {
    pub(in crate::backend::emulator::x86_64) fn execute_vex_arith(
        &mut self,
        ctx: &mut InsnContext,
        vex_pp: u8,
        vex_l: u8,
        vvvv: u8,
        opcode: u8,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;
        let xmm_src1 = vvvv as usize;

        // Determine operation type based on opcode
        // x86 MIN/MAX follow `(a<b)?a:b` / `(a>b)?a:b`, returning the SECOND operand
        // on any unordered (NaN) compare or signed-zero tie — not Rust's f32::min/max.
        let op: fn(f32, f32) -> f32 = match opcode {
            0x58 => |a, b| a + b,                     // ADD
            0x59 => |a, b| a * b,                     // MUL
            0x5C => |a, b| a - b,                     // SUB
            0x5D => |a, b| if a < b { a } else { b }, // MIN
            0x5E => |a, b| a / b,                     // DIV
            0x5F => |a, b| if a > b { a } else { b }, // MAX
            0x51 => |_a, b| b.sqrt(),                 // SQRT (unary, uses only src2)
            _ => unreachable!(),
        };
        let op_d: fn(f64, f64) -> f64 = match opcode {
            0x58 => |a, b| a + b,
            0x59 => |a, b| a * b,
            0x5C => |a, b| a - b,
            0x5D => |a, b| if a < b { a } else { b },
            0x5E => |a, b| a / b,
            0x5F => |a, b| if a > b { a } else { b },
            0x51 => |_a, b| b.sqrt(),
            _ => unreachable!(),
        };

        match vex_pp {
            0 => {
                // VEX.0F (NP) - packed single (PS)
                let (src2_lo, src2_hi) = if is_memory {
                    (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
                } else {
                    (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
                };
                let src1_lo = self.regs.xmm[xmm_src1][0];
                let src1_hi = self.regs.xmm[xmm_src1][1];

                // Process 4 floats in low 128 bits
                let r0 = op(
                    f32::from_bits(src1_lo as u32),
                    f32::from_bits(src2_lo as u32),
                );
                let r1 = op(
                    f32::from_bits((src1_lo >> 32) as u32),
                    f32::from_bits((src2_lo >> 32) as u32),
                );
                let r2 = op(
                    f32::from_bits(src1_hi as u32),
                    f32::from_bits(src2_hi as u32),
                );
                let r3 = op(
                    f32::from_bits((src1_hi >> 32) as u32),
                    f32::from_bits((src2_hi >> 32) as u32),
                );
                self.regs.xmm[xmm_dst][0] = r0.to_bits() as u64 | ((r1.to_bits() as u64) << 32);
                self.regs.xmm[xmm_dst][1] = r2.to_bits() as u64 | ((r3.to_bits() as u64) << 32);

                if vex_l == 1 {
                    // YMM - process high 128 bits too
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
                    let r4 = op(
                        f32::from_bits(src1_hi2 as u32),
                        f32::from_bits(src2_hi2 as u32),
                    );
                    let r5 = op(
                        f32::from_bits((src1_hi2 >> 32) as u32),
                        f32::from_bits((src2_hi2 >> 32) as u32),
                    );
                    let r6 = op(
                        f32::from_bits(src1_hi3 as u32),
                        f32::from_bits(src2_hi3 as u32),
                    );
                    let r7 = op(
                        f32::from_bits((src1_hi3 >> 32) as u32),
                        f32::from_bits((src2_hi3 >> 32) as u32),
                    );
                    self.regs.ymm_high[xmm_dst][0] =
                        r4.to_bits() as u64 | ((r5.to_bits() as u64) << 32);
                    self.regs.ymm_high[xmm_dst][1] =
                        r6.to_bits() as u64 | ((r7.to_bits() as u64) << 32);
                } else {
                    // VEX.128 clears upper bits
                    self.regs.ymm_high[xmm_dst][0] = 0;
                    self.regs.ymm_high[xmm_dst][1] = 0;
                }
            }
            1 => {
                // VEX.66.0F - packed double (PD)
                let (src2_lo, src2_hi) = if is_memory {
                    (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
                } else {
                    (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
                };
                let src1_lo = self.regs.xmm[xmm_src1][0];
                let src1_hi = self.regs.xmm[xmm_src1][1];

                let r0 = op_d(f64::from_bits(src1_lo), f64::from_bits(src2_lo));
                let r1 = op_d(f64::from_bits(src1_hi), f64::from_bits(src2_hi));
                self.regs.xmm[xmm_dst][0] = r0.to_bits();
                self.regs.xmm[xmm_dst][1] = r1.to_bits();

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
                    let r2 = op_d(f64::from_bits(src1_hi2), f64::from_bits(src2_hi2));
                    let r3 = op_d(f64::from_bits(src1_hi3), f64::from_bits(src2_hi3));
                    self.regs.ymm_high[xmm_dst][0] = r2.to_bits();
                    self.regs.ymm_high[xmm_dst][1] = r3.to_bits();
                } else {
                    self.regs.ymm_high[xmm_dst][0] = 0;
                    self.regs.ymm_high[xmm_dst][1] = 0;
                }
            }
            2 => {
                // VEX.F3.0F - scalar single (SS)
                let src2 = if is_memory {
                    f32::from_bits(self.read_mem(addr, 4)? as u32)
                } else {
                    f32::from_bits(self.regs.xmm[rm as usize][0] as u32)
                };
                let src1 = f32::from_bits(self.regs.xmm[xmm_src1][0] as u32);
                let result = op(src1, src2);
                // Copy src1 to dst, then overwrite low 32 bits
                self.regs.xmm[xmm_dst][0] =
                    (self.regs.xmm[xmm_src1][0] & !0xFFFFFFFF) | result.to_bits() as u64;
                self.regs.xmm[xmm_dst][1] = self.regs.xmm[xmm_src1][1];
                self.regs.ymm_high[xmm_dst][0] = 0;
                self.regs.ymm_high[xmm_dst][1] = 0;
            }
            3 => {
                // VEX.F2.0F - scalar double (SD)
                let src2 = if is_memory {
                    f64::from_bits(self.read_mem(addr, 8)?)
                } else {
                    f64::from_bits(self.regs.xmm[rm as usize][0])
                };
                let src1 = f64::from_bits(self.regs.xmm[xmm_src1][0]);
                let result = op_d(src1, src2);
                self.regs.xmm[xmm_dst][0] = result.to_bits();
                self.regs.xmm[xmm_dst][1] = self.regs.xmm[xmm_src1][1];
                self.regs.ymm_high[xmm_dst][0] = 0;
                self.regs.ymm_high[xmm_dst][1] = 0;
            }
            _ => unreachable!(),
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_roundp(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
        opcode: u8,
    ) -> Result<Option<VcpuExit>> {
        if vvvv != 0 {
            return Err(Error::Emulator(
                "VROUNDPS/PD require VEX.vvvv=1111b".to_string(),
            ));
        }
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let imm8 = ctx.consume_u8()?;
        let xmm_dst = reg as usize;

        if opcode == 0x08 {
            // VROUNDPS
            let count = if vex_l == 1 { 8 } else { 4 };
            let mut src = [0f32; 8];
            if is_memory {
                for i in 0..count {
                    src[i] = f32::from_bits(self.read_mem(addr + (i * 4) as u64, 4)? as u32);
                }
            } else {
                let xmm_src = rm as usize;
                let lo = self.regs.xmm[xmm_src][0];
                let hi = self.regs.xmm[xmm_src][1];
                src[0] = f32::from_bits(lo as u32);
                src[1] = f32::from_bits((lo >> 32) as u32);
                src[2] = f32::from_bits(hi as u32);
                src[3] = f32::from_bits((hi >> 32) as u32);
                if vex_l == 1 {
                    let hi2 = self.regs.ymm_high[xmm_src][0];
                    let hi3 = self.regs.ymm_high[xmm_src][1];
                    src[4] = f32::from_bits(hi2 as u32);
                    src[5] = f32::from_bits((hi2 >> 32) as u32);
                    src[6] = f32::from_bits(hi3 as u32);
                    src[7] = f32::from_bits((hi3 >> 32) as u32);
                }
            }

            let mut dst = [0u32; 8];
            for i in 0..count {
                dst[i] = round_f32(src[i], imm8).to_bits();
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
            // VROUNDPD
            let count = if vex_l == 1 { 4 } else { 2 };
            let mut src = [0f64; 4];
            if is_memory {
                for i in 0..count {
                    src[i] = f64::from_bits(self.read_mem(addr + (i * 8) as u64, 8)?);
                }
            } else {
                let xmm_src = rm as usize;
                src[0] = f64::from_bits(self.regs.xmm[xmm_src][0]);
                src[1] = f64::from_bits(self.regs.xmm[xmm_src][1]);
                if vex_l == 1 {
                    src[2] = f64::from_bits(self.regs.ymm_high[xmm_src][0]);
                    src[3] = f64::from_bits(self.regs.ymm_high[xmm_src][1]);
                }
            }

            let mut dst = [0u64; 4];
            for i in 0..count {
                dst[i] = round_f64(src[i], imm8).to_bits();
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

    pub(in crate::backend::emulator::x86_64) fn execute_vex_rounds(
        &mut self,
        ctx: &mut InsnContext,
        vvvv: u8,
        opcode: u8,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let imm8 = ctx.consume_u8()?;
        let xmm_dst = reg as usize;
        let xmm_src1 = vvvv as usize;

        if opcode == 0x0A {
            // VROUNDSS
            let src = if is_memory {
                f32::from_bits(self.read_mem(addr, 4)? as u32)
            } else {
                f32::from_bits(self.regs.xmm[rm as usize][0] as u32)
            };
            let rounded = round_f32(src, imm8).to_bits() as u64;
            let src1_lo = self.regs.xmm[xmm_src1][0];
            self.regs.xmm[xmm_dst][0] = (src1_lo & !0xFFFF_FFFF) | rounded;
            self.regs.xmm[xmm_dst][1] = self.regs.xmm[xmm_src1][1];
        } else {
            // VROUNDSD
            let src = if is_memory {
                f64::from_bits(self.read_mem(addr, 8)?)
            } else {
                f64::from_bits(self.regs.xmm[rm as usize][0])
            };
            let rounded = round_f64(src, imm8).to_bits();
            self.regs.xmm[xmm_dst][0] = rounded;
            self.regs.xmm[xmm_dst][1] = self.regs.xmm[xmm_src1][1];
        }

        self.regs.ymm_high[xmm_dst][0] = 0;
        self.regs.ymm_high[xmm_dst][1] = 0;
        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_dp(
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

        if opcode == 0x40 {
            // VDPPS
            let lanes = if vex_l == 1 { 2 } else { 1 };
            let mut src1 = [0f32; 8];
            let mut src2 = [0f32; 8];
            let lo1 = self.regs.xmm[xmm_src1][0];
            let hi1 = self.regs.xmm[xmm_src1][1];
            src1[0] = f32::from_bits(lo1 as u32);
            src1[1] = f32::from_bits((lo1 >> 32) as u32);
            src1[2] = f32::from_bits(hi1 as u32);
            src1[3] = f32::from_bits((hi1 >> 32) as u32);
            if vex_l == 1 {
                let hi2 = self.regs.ymm_high[xmm_src1][0];
                let hi3 = self.regs.ymm_high[xmm_src1][1];
                src1[4] = f32::from_bits(hi2 as u32);
                src1[5] = f32::from_bits((hi2 >> 32) as u32);
                src1[6] = f32::from_bits(hi3 as u32);
                src1[7] = f32::from_bits((hi3 >> 32) as u32);
            }

            if is_memory {
                let count = lanes * 4;
                for i in 0..count {
                    src2[i] = f32::from_bits(self.read_mem(addr + (i * 4) as u64, 4)? as u32);
                }
            } else {
                let xmm_src2 = rm as usize;
                let lo2 = self.regs.xmm[xmm_src2][0];
                let hi2 = self.regs.xmm[xmm_src2][1];
                src2[0] = f32::from_bits(lo2 as u32);
                src2[1] = f32::from_bits((lo2 >> 32) as u32);
                src2[2] = f32::from_bits(hi2 as u32);
                src2[3] = f32::from_bits((hi2 >> 32) as u32);
                if vex_l == 1 {
                    let hi3 = self.regs.ymm_high[xmm_src2][0];
                    let hi4 = self.regs.ymm_high[xmm_src2][1];
                    src2[4] = f32::from_bits(hi3 as u32);
                    src2[5] = f32::from_bits((hi3 >> 32) as u32);
                    src2[6] = f32::from_bits(hi4 as u32);
                    src2[7] = f32::from_bits((hi4 >> 32) as u32);
                }
            }

            let mut dst = [0u32; 8];
            for lane in 0..lanes {
                let base = lane * 4;
                let mut sum = 0.0f32;
                for i in 0..4 {
                    if ((imm8 >> (4 + i)) & 1) != 0 {
                        sum += src1[base + i] * src2[base + i];
                    }
                }
                for i in 0..4 {
                    dst[base + i] = if ((imm8 >> i) & 1) != 0 {
                        sum.to_bits()
                    } else {
                        0
                    };
                }
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
            // VDPPD
            if vex_l != 0 {
                return Err(Error::Emulator("VDPPD requires VEX.L=0".to_string()));
            }
            let src1_lo = f64::from_bits(self.regs.xmm[xmm_src1][0]);
            let src1_hi = f64::from_bits(self.regs.xmm[xmm_src1][1]);
            let (src2_lo, src2_hi) = if is_memory {
                (
                    f64::from_bits(self.read_mem(addr, 8)?),
                    f64::from_bits(self.read_mem(addr + 8, 8)?),
                )
            } else {
                let xmm_src2 = rm as usize;
                (
                    f64::from_bits(self.regs.xmm[xmm_src2][0]),
                    f64::from_bits(self.regs.xmm[xmm_src2][1]),
                )
            };
            let mut sum = 0.0f64;
            if ((imm8 >> 4) & 1) != 0 {
                sum += src1_lo * src2_lo;
            }
            if ((imm8 >> 5) & 1) != 0 {
                sum += src1_hi * src2_hi;
            }

            let dst_lo = if (imm8 & 1) != 0 { sum.to_bits() } else { 0 };
            let dst_hi = if (imm8 & 2) != 0 { sum.to_bits() } else { 0 };
            self.regs.xmm[xmm_dst][0] = dst_lo;
            self.regs.xmm[xmm_dst][1] = dst_hi;
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }
}

fn round_mode(imm8: u8) -> u8 {
    if (imm8 & 0x4) != 0 { 0 } else { imm8 & 0x3 }
}

fn round_ties_even_f32(value: f32) -> f32 {
    if !value.is_finite() {
        return value;
    }
    let floor = value.floor();
    let diff = value - floor;
    if diff < 0.5 {
        return floor;
    }
    if diff > 0.5 {
        return floor + 1.0;
    }
    if floor.abs() > i64::MAX as f32 {
        return floor;
    }
    if (floor as i64) % 2 == 0 {
        floor
    } else {
        floor + 1.0
    }
}

fn round_ties_even_f64(value: f64) -> f64 {
    if !value.is_finite() {
        return value;
    }
    let floor = value.floor();
    let diff = value - floor;
    if diff < 0.5 {
        return floor;
    }
    if diff > 0.5 {
        return floor + 1.0;
    }
    if floor.abs() > i64::MAX as f64 {
        return floor;
    }
    if (floor as i64) % 2 == 0 {
        floor
    } else {
        floor + 1.0
    }
}

fn round_f32(value: f32, imm8: u8) -> f32 {
    match round_mode(imm8) {
        0 => round_ties_even_f32(value),
        1 => value.floor(),
        2 => value.ceil(),
        _ => value.trunc(),
    }
}

fn round_f64(value: f64, imm8: u8) -> f64 {
    match round_mode(imm8) {
        0 => round_ties_even_f64(value),
        1 => value.floor(),
        2 => value.ceil(),
        _ => value.trunc(),
    }
}
