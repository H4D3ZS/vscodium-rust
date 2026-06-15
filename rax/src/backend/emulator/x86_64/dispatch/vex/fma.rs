//! VEX FMA instruction implementation for x86_64 emulator.

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::cpu::{InsnContext, X86_64Vcpu};

#[derive(Copy, Clone)]
enum FmaKind {
    Add,
    Sub,
    Nmadd,
    Nmsub,
    AddSub,
    SubAdd,
}

#[derive(Copy, Clone)]
enum FmaOrder {
    Order132,
    Order213,
    Order231,
}

fn decode_fma(opcode: u8) -> Option<(FmaKind, FmaOrder, bool)> {
    let (kind, order, is_scalar) = match opcode {
        0x96 => (FmaKind::AddSub, FmaOrder::Order132, false),
        0xA6 => (FmaKind::AddSub, FmaOrder::Order213, false),
        0xB6 => (FmaKind::AddSub, FmaOrder::Order231, false),
        0x97 => (FmaKind::SubAdd, FmaOrder::Order132, false),
        0xA7 => (FmaKind::SubAdd, FmaOrder::Order213, false),
        0xB7 => (FmaKind::SubAdd, FmaOrder::Order231, false),
        _ => {
            let is_scalar = (opcode & 0x01) != 0;
            let base = opcode & 0xFE;
            let (kind, order) = match base {
                0x98 => (FmaKind::Add, FmaOrder::Order132),
                0x9A => (FmaKind::Sub, FmaOrder::Order132),
                0x9C => (FmaKind::Nmadd, FmaOrder::Order132),
                0x9E => (FmaKind::Nmsub, FmaOrder::Order132),
                0xA8 => (FmaKind::Add, FmaOrder::Order213),
                0xAA => (FmaKind::Sub, FmaOrder::Order213),
                0xAC => (FmaKind::Nmadd, FmaOrder::Order213),
                0xAE => (FmaKind::Nmsub, FmaOrder::Order213),
                0xB8 => (FmaKind::Add, FmaOrder::Order231),
                0xBA => (FmaKind::Sub, FmaOrder::Order231),
                0xBC => (FmaKind::Nmadd, FmaOrder::Order231),
                0xBE => (FmaKind::Nmsub, FmaOrder::Order231),
                _ => return None,
            };
            (kind, order, is_scalar)
        }
    };

    Some((kind, order, is_scalar))
}

fn select_operands_f32(order: FmaOrder, src1: f32, src2: f32, src3: f32) -> (f32, f32, f32) {
    match order {
        FmaOrder::Order132 => (src1, src3, src2),
        FmaOrder::Order213 => (src2, src1, src3),
        FmaOrder::Order231 => (src2, src3, src1),
    }
}

fn select_operands_f64(order: FmaOrder, src1: f64, src2: f64, src3: f64) -> (f64, f64, f64) {
    match order {
        FmaOrder::Order132 => (src1, src3, src2),
        FmaOrder::Order213 => (src2, src1, src3),
        FmaOrder::Order231 => (src2, src3, src1),
    }
}

fn fma_calc_f32(kind: FmaKind, lane: usize, a: f32, b: f32, c: f32) -> f32 {
    match kind {
        FmaKind::Add => a.mul_add(b, c),
        FmaKind::Sub => a.mul_add(b, -c),
        FmaKind::Nmadd => (-a).mul_add(b, c),
        FmaKind::Nmsub => (-a).mul_add(b, -c),
        FmaKind::AddSub => {
            let c_adj = if (lane & 1) == 0 { -c } else { c };
            a.mul_add(b, c_adj)
        }
        FmaKind::SubAdd => {
            let c_adj = if (lane & 1) == 0 { c } else { -c };
            a.mul_add(b, c_adj)
        }
    }
}

fn fma_calc_f64(kind: FmaKind, lane: usize, a: f64, b: f64, c: f64) -> f64 {
    match kind {
        FmaKind::Add => a.mul_add(b, c),
        FmaKind::Sub => a.mul_add(b, -c),
        FmaKind::Nmadd => (-a).mul_add(b, c),
        FmaKind::Nmsub => (-a).mul_add(b, -c),
        FmaKind::AddSub => {
            let c_adj = if (lane & 1) == 0 { -c } else { c };
            a.mul_add(b, c_adj)
        }
        FmaKind::SubAdd => {
            let c_adj = if (lane & 1) == 0 { c } else { -c };
            a.mul_add(b, c_adj)
        }
    }
}

fn fma_f32_pair(
    kind: FmaKind,
    order: FmaOrder,
    lane_base: usize,
    src1: u64,
    src2: u64,
    src3: u64,
) -> u64 {
    let s1_0 = f32::from_bits(src1 as u32);
    let s1_1 = f32::from_bits((src1 >> 32) as u32);
    let s2_0 = f32::from_bits(src2 as u32);
    let s2_1 = f32::from_bits((src2 >> 32) as u32);
    let s3_0 = f32::from_bits(src3 as u32);
    let s3_1 = f32::from_bits((src3 >> 32) as u32);

    let (a0, b0, c0) = select_operands_f32(order, s1_0, s2_0, s3_0);
    let (a1, b1, c1) = select_operands_f32(order, s1_1, s2_1, s3_1);
    let r0 = fma_calc_f32(kind, lane_base, a0, b0, c0);
    let r1 = fma_calc_f32(kind, lane_base + 1, a1, b1, c1);
    r0.to_bits() as u64 | ((r1.to_bits() as u64) << 32)
}

fn fma_f64_lane(
    kind: FmaKind,
    order: FmaOrder,
    lane: usize,
    src1: u64,
    src2: u64,
    src3: u64,
) -> u64 {
    let s1 = f64::from_bits(src1);
    let s2 = f64::from_bits(src2);
    let s3 = f64::from_bits(src3);
    let (a, b, c) = select_operands_f64(order, s1, s2, s3);
    fma_calc_f64(kind, lane, a, b, c).to_bits()
}

impl X86_64Vcpu {
    pub(in crate::backend::emulator::x86_64) fn execute_vex_fma(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vex_w: u8,
        vvvv: u8,
        opcode: u8,
    ) -> Result<Option<VcpuExit>> {
        let (kind, order, is_scalar) = decode_fma(opcode).ok_or_else(|| {
            Error::Emulator(format!("unimplemented VEX FMA opcode {:#x}", opcode))
        })?;
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;
        let xmm_src2 = vvvv as usize;
        let is_double = vex_w != 0;

        if is_scalar {
            let src1_lo = self.regs.xmm[xmm_dst][0];
            let src1_hi = self.regs.xmm[xmm_dst][1];
            if is_double {
                let src1 = f64::from_bits(src1_lo);
                let src2 = f64::from_bits(self.regs.xmm[xmm_src2][0]);
                let src3 = if is_memory {
                    f64::from_bits(self.read_mem(addr, 8)?)
                } else {
                    f64::from_bits(self.regs.xmm[rm as usize][0])
                };
                let (a, b, c) = select_operands_f64(order, src1, src2, src3);
                let result = fma_calc_f64(kind, 0, a, b, c);
                self.regs.xmm[xmm_dst][0] = result.to_bits();
            } else {
                let src1 = f32::from_bits(src1_lo as u32);
                let src2 = f32::from_bits(self.regs.xmm[xmm_src2][0] as u32);
                let src3 = if is_memory {
                    f32::from_bits(self.read_mem(addr, 4)? as u32)
                } else {
                    f32::from_bits(self.regs.xmm[rm as usize][0] as u32)
                };
                let (a, b, c) = select_operands_f32(order, src1, src2, src3);
                let result = fma_calc_f32(kind, 0, a, b, c);
                self.regs.xmm[xmm_dst][0] = (src1_lo & !0xFFFF_FFFF) | result.to_bits() as u64;
            }
            self.regs.xmm[xmm_dst][1] = src1_hi;
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        } else {
            let src1_lo = self.regs.xmm[xmm_dst][0];
            let src1_hi = self.regs.xmm[xmm_dst][1];
            let src2_lo = self.regs.xmm[xmm_src2][0];
            let src2_hi = self.regs.xmm[xmm_src2][1];
            let (src3_lo, src3_hi) = if is_memory {
                (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
            } else {
                (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
            };

            if is_double {
                self.regs.xmm[xmm_dst][0] = fma_f64_lane(kind, order, 0, src1_lo, src2_lo, src3_lo);
                self.regs.xmm[xmm_dst][1] = fma_f64_lane(kind, order, 1, src1_hi, src2_hi, src3_hi);

                if vex_l == 1 {
                    let src1_hi2 = self.regs.ymm_high[xmm_dst][0];
                    let src1_hi3 = self.regs.ymm_high[xmm_dst][1];
                    let src2_hi2 = self.regs.ymm_high[xmm_src2][0];
                    let src2_hi3 = self.regs.ymm_high[xmm_src2][1];
                    let (src3_hi2, src3_hi3) = if is_memory {
                        (self.read_mem(addr + 16, 8)?, self.read_mem(addr + 24, 8)?)
                    } else {
                        (
                            self.regs.ymm_high[rm as usize][0],
                            self.regs.ymm_high[rm as usize][1],
                        )
                    };
                    self.regs.ymm_high[xmm_dst][0] =
                        fma_f64_lane(kind, order, 2, src1_hi2, src2_hi2, src3_hi2);
                    self.regs.ymm_high[xmm_dst][1] =
                        fma_f64_lane(kind, order, 3, src1_hi3, src2_hi3, src3_hi3);
                } else {
                    self.regs.ymm_high[xmm_dst][0] = 0;
                    self.regs.ymm_high[xmm_dst][1] = 0;
                }
            } else {
                self.regs.xmm[xmm_dst][0] = fma_f32_pair(kind, order, 0, src1_lo, src2_lo, src3_lo);
                self.regs.xmm[xmm_dst][1] = fma_f32_pair(kind, order, 2, src1_hi, src2_hi, src3_hi);

                if vex_l == 1 {
                    let src1_hi2 = self.regs.ymm_high[xmm_dst][0];
                    let src1_hi3 = self.regs.ymm_high[xmm_dst][1];
                    let src2_hi2 = self.regs.ymm_high[xmm_src2][0];
                    let src2_hi3 = self.regs.ymm_high[xmm_src2][1];
                    let (src3_hi2, src3_hi3) = if is_memory {
                        (self.read_mem(addr + 16, 8)?, self.read_mem(addr + 24, 8)?)
                    } else {
                        (
                            self.regs.ymm_high[rm as usize][0],
                            self.regs.ymm_high[rm as usize][1],
                        )
                    };
                    self.regs.ymm_high[xmm_dst][0] =
                        fma_f32_pair(kind, order, 4, src1_hi2, src2_hi2, src3_hi2);
                    self.regs.ymm_high[xmm_dst][1] =
                        fma_f32_pair(kind, order, 6, src1_hi3, src2_hi3, src3_hi3);
                } else {
                    self.regs.ymm_high[xmm_dst][0] = 0;
                    self.regs.ymm_high[xmm_dst][1] = 0;
                }
            }
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }
}
