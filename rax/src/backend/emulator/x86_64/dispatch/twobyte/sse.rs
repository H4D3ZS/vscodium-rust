//! Two-byte opcode instruction implementation for x86_64 emulator.

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::aes;
use super::super::super::cpu::{InsnContext, X86_64Vcpu};
use super::super::super::flags;
use super::super::super::insn;

// x86 MIN/MAX semantics differ from IEEE/Rust `f32::min`/`f32::max`.
// SDM: MIN returns `(dst < src) ? dst : src` and MAX returns `(dst > src) ? dst : src`.
// The comparison is FALSE whenever either operand is NaN (unordered) or on a
// signed-zero tie, so the SECOND operand (`src`) is returned in those cases. This
// is unlike Rust's `min`/`max`, which return the non-NaN operand. KVM (hardware)
// follows the SDM, so we must too.
#[inline(always)]
fn x86_min_f32(dst: f32, src: f32) -> f32 {
    if dst < src { dst } else { src }
}
#[inline(always)]
fn x86_max_f32(dst: f32, src: f32) -> f32 {
    if dst > src { dst } else { src }
}
#[inline(always)]
fn x86_min_f64(dst: f64, src: f64) -> f64 {
    if dst < src { dst } else { src }
}
#[inline(always)]
fn x86_max_f64(dst: f64, src: f64) -> f64 {
    if dst > src { dst } else { src }
}

impl X86_64Vcpu {
    pub(in crate::backend::emulator::x86_64) fn execute_sse_add(
        &mut self,
        ctx: &mut InsnContext,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;

        if ctx.rep_prefix == Some(0xF3) {
            // ADDSS - scalar single
            let src = if is_memory {
                f32::from_bits(self.read_mem(addr, 4)? as u32)
            } else {
                f32::from_bits(self.regs.xmm[rm as usize][0] as u32)
            };
            let dst = f32::from_bits(self.regs.xmm[xmm_dst][0] as u32);
            let result = dst + src;
            self.regs.xmm[xmm_dst][0] =
                (self.regs.xmm[xmm_dst][0] & !0xFFFFFFFF) | result.to_bits() as u64;
        } else if ctx.rep_prefix == Some(0xF2) {
            // ADDSD - scalar double
            let src = if is_memory {
                f64::from_bits(self.read_mem(addr, 8)?)
            } else {
                f64::from_bits(self.regs.xmm[rm as usize][0])
            };
            let dst = f64::from_bits(self.regs.xmm[xmm_dst][0]);
            self.regs.xmm[xmm_dst][0] = (dst + src).to_bits();
        } else if ctx.operand_size_override {
            // ADDPD - packed double (2 x f64)
            use super::super::super::simd_native;

            let src: simd_native::Xmm = if is_memory {
                [self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?]
            } else {
                [self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1]]
            };

            let mut dst: simd_native::Xmm = [self.regs.xmm[xmm_dst][0], self.regs.xmm[xmm_dst][1]];

            simd_native::addpd_xmm(&mut dst, &src);

            self.regs.xmm[xmm_dst][0] = dst[0];
            self.regs.xmm[xmm_dst][1] = dst[1];
        } else {
            // ADDPS - packed single (4 x f32)
            // Use native SIMD passthrough when available
            use super::super::super::simd_native;

            let src: simd_native::Xmm = if is_memory {
                [self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?]
            } else {
                [self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1]]
            };

            let mut dst: simd_native::Xmm = [self.regs.xmm[xmm_dst][0], self.regs.xmm[xmm_dst][1]];

            // Native SSE if available, scalar fallback otherwise
            simd_native::addps_xmm(&mut dst, &src);

            self.regs.xmm[xmm_dst][0] = dst[0];
            self.regs.xmm[xmm_dst][1] = dst[1];
        }
        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// SSE packed single/double subtract (0x5C)
    pub(in crate::backend::emulator::x86_64) fn execute_sse_sub(
        &mut self,
        ctx: &mut InsnContext,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;

        if ctx.rep_prefix == Some(0xF3) {
            let src = if is_memory {
                f32::from_bits(self.read_mem(addr, 4)? as u32)
            } else {
                f32::from_bits(self.regs.xmm[rm as usize][0] as u32)
            };
            let dst = f32::from_bits(self.regs.xmm[xmm_dst][0] as u32);
            self.regs.xmm[xmm_dst][0] =
                (self.regs.xmm[xmm_dst][0] & !0xFFFFFFFF) | (dst - src).to_bits() as u64;
        } else if ctx.rep_prefix == Some(0xF2) {
            let src = if is_memory {
                f64::from_bits(self.read_mem(addr, 8)?)
            } else {
                f64::from_bits(self.regs.xmm[rm as usize][0])
            };
            let dst = f64::from_bits(self.regs.xmm[xmm_dst][0]);
            self.regs.xmm[xmm_dst][0] = (dst - src).to_bits();
        } else if ctx.operand_size_override {
            // SUBPD - packed double (2 x f64)
            use super::super::super::simd_native;

            let src: simd_native::Xmm = if is_memory {
                [self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?]
            } else {
                [self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1]]
            };

            let mut dst: simd_native::Xmm = [self.regs.xmm[xmm_dst][0], self.regs.xmm[xmm_dst][1]];

            simd_native::subpd_xmm(&mut dst, &src);

            self.regs.xmm[xmm_dst][0] = dst[0];
            self.regs.xmm[xmm_dst][1] = dst[1];
        } else {
            // SUBPS - packed single (4 x f32)
            use super::super::super::simd_native;

            let src: simd_native::Xmm = if is_memory {
                [self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?]
            } else {
                [self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1]]
            };

            let mut dst: simd_native::Xmm = [self.regs.xmm[xmm_dst][0], self.regs.xmm[xmm_dst][1]];

            simd_native::subps_xmm(&mut dst, &src);

            self.regs.xmm[xmm_dst][0] = dst[0];
            self.regs.xmm[xmm_dst][1] = dst[1];
        }
        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// SSE packed single/double multiply (0x59)
    pub(in crate::backend::emulator::x86_64) fn execute_sse_mul(
        &mut self,
        ctx: &mut InsnContext,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;

        if ctx.rep_prefix == Some(0xF3) {
            let src = if is_memory {
                f32::from_bits(self.read_mem(addr, 4)? as u32)
            } else {
                f32::from_bits(self.regs.xmm[rm as usize][0] as u32)
            };
            let dst = f32::from_bits(self.regs.xmm[xmm_dst][0] as u32);
            self.regs.xmm[xmm_dst][0] =
                (self.regs.xmm[xmm_dst][0] & !0xFFFFFFFF) | (dst * src).to_bits() as u64;
        } else if ctx.rep_prefix == Some(0xF2) {
            let src = if is_memory {
                f64::from_bits(self.read_mem(addr, 8)?)
            } else {
                f64::from_bits(self.regs.xmm[rm as usize][0])
            };
            let dst = f64::from_bits(self.regs.xmm[xmm_dst][0]);
            self.regs.xmm[xmm_dst][0] = (dst * src).to_bits();
        } else if ctx.operand_size_override {
            // MULPD - packed double (2 x f64)
            use super::super::super::simd_native;

            let src: simd_native::Xmm = if is_memory {
                [self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?]
            } else {
                [self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1]]
            };

            let mut dst: simd_native::Xmm = [self.regs.xmm[xmm_dst][0], self.regs.xmm[xmm_dst][1]];

            simd_native::mulpd_xmm(&mut dst, &src);

            self.regs.xmm[xmm_dst][0] = dst[0];
            self.regs.xmm[xmm_dst][1] = dst[1];
        } else {
            // MULPS - packed single (4 x f32)
            use super::super::super::simd_native;

            let src: simd_native::Xmm = if is_memory {
                [self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?]
            } else {
                [self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1]]
            };

            let mut dst: simd_native::Xmm = [self.regs.xmm[xmm_dst][0], self.regs.xmm[xmm_dst][1]];

            simd_native::mulps_xmm(&mut dst, &src);

            self.regs.xmm[xmm_dst][0] = dst[0];
            self.regs.xmm[xmm_dst][1] = dst[1];
        }
        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// SSE packed single/double divide (0x5E)
    pub(in crate::backend::emulator::x86_64) fn execute_sse_div(
        &mut self,
        ctx: &mut InsnContext,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;

        if ctx.rep_prefix == Some(0xF3) {
            let src = if is_memory {
                f32::from_bits(self.read_mem(addr, 4)? as u32)
            } else {
                f32::from_bits(self.regs.xmm[rm as usize][0] as u32)
            };
            let dst = f32::from_bits(self.regs.xmm[xmm_dst][0] as u32);
            self.regs.xmm[xmm_dst][0] =
                (self.regs.xmm[xmm_dst][0] & !0xFFFFFFFF) | (dst / src).to_bits() as u64;
        } else if ctx.rep_prefix == Some(0xF2) {
            let src = if is_memory {
                f64::from_bits(self.read_mem(addr, 8)?)
            } else {
                f64::from_bits(self.regs.xmm[rm as usize][0])
            };
            let dst = f64::from_bits(self.regs.xmm[xmm_dst][0]);
            self.regs.xmm[xmm_dst][0] = (dst / src).to_bits();
        } else if ctx.operand_size_override {
            // DIVPD - packed double (2 x f64)
            use super::super::super::simd_native;

            let src: simd_native::Xmm = if is_memory {
                [self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?]
            } else {
                [self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1]]
            };

            let mut dst: simd_native::Xmm = [self.regs.xmm[xmm_dst][0], self.regs.xmm[xmm_dst][1]];

            simd_native::divpd_xmm(&mut dst, &src);

            self.regs.xmm[xmm_dst][0] = dst[0];
            self.regs.xmm[xmm_dst][1] = dst[1];
        } else {
            // DIVPS - packed single (4 x f32)
            use super::super::super::simd_native;

            let src: simd_native::Xmm = if is_memory {
                [self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?]
            } else {
                [self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1]]
            };

            let mut dst: simd_native::Xmm = [self.regs.xmm[xmm_dst][0], self.regs.xmm[xmm_dst][1]];

            simd_native::divps_xmm(&mut dst, &src);

            self.regs.xmm[xmm_dst][0] = dst[0];
            self.regs.xmm[xmm_dst][1] = dst[1];
        }
        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// SSE packed single/double sqrt (0x51)
    pub(in crate::backend::emulator::x86_64) fn execute_sse_sqrt(
        &mut self,
        ctx: &mut InsnContext,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;

        if ctx.rep_prefix == Some(0xF3) {
            let src = if is_memory {
                f32::from_bits(self.read_mem(addr, 4)? as u32)
            } else {
                f32::from_bits(self.regs.xmm[rm as usize][0] as u32)
            };
            self.regs.xmm[xmm_dst][0] =
                (self.regs.xmm[xmm_dst][0] & !0xFFFFFFFF) | src.sqrt().to_bits() as u64;
        } else if ctx.rep_prefix == Some(0xF2) {
            let src = if is_memory {
                f64::from_bits(self.read_mem(addr, 8)?)
            } else {
                f64::from_bits(self.regs.xmm[rm as usize][0])
            };
            self.regs.xmm[xmm_dst][0] = src.sqrt().to_bits();
        } else if ctx.operand_size_override {
            let (src_lo, src_hi) = if is_memory {
                (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
            } else {
                (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
            };
            self.regs.xmm[xmm_dst][0] = f64::from_bits(src_lo).sqrt().to_bits();
            self.regs.xmm[xmm_dst][1] = f64::from_bits(src_hi).sqrt().to_bits();
        } else {
            let (src_lo, src_hi) = if is_memory {
                (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
            } else {
                (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
            };
            let r0 = f32::from_bits(src_lo as u32).sqrt();
            let r1 = f32::from_bits((src_lo >> 32) as u32).sqrt();
            let r2 = f32::from_bits(src_hi as u32).sqrt();
            let r3 = f32::from_bits((src_hi >> 32) as u32).sqrt();
            self.regs.xmm[xmm_dst][0] = r0.to_bits() as u64 | ((r1.to_bits() as u64) << 32);
            self.regs.xmm[xmm_dst][1] = r2.to_bits() as u64 | ((r3.to_bits() as u64) << 32);
        }
        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// SSE reciprocal square root (0x52)
    pub(in crate::backend::emulator::x86_64) fn execute_sse_rsqrt(
        &mut self,
        ctx: &mut InsnContext,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;

        if ctx.rep_prefix == Some(0xF3) {
            let src = if is_memory {
                f32::from_bits(self.read_mem(addr, 4)? as u32)
            } else {
                f32::from_bits(self.regs.xmm[rm as usize][0] as u32)
            };
            let result = (1.0f32 / src.sqrt()).to_bits() as u64;
            self.regs.xmm[xmm_dst][0] = (self.regs.xmm[xmm_dst][0] & !0xFFFF_FFFF) | result;
        } else {
            let (src_lo, src_hi) = if is_memory {
                (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
            } else {
                (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
            };
            self.regs.xmm[xmm_dst][0] = rsqrt_packed_f32(src_lo);
            self.regs.xmm[xmm_dst][1] = rsqrt_packed_f32(src_hi);
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// SSE reciprocal (0x53)
    pub(in crate::backend::emulator::x86_64) fn execute_sse_rcp(
        &mut self,
        ctx: &mut InsnContext,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;

        if ctx.rep_prefix == Some(0xF3) {
            let src = if is_memory {
                f32::from_bits(self.read_mem(addr, 4)? as u32)
            } else {
                f32::from_bits(self.regs.xmm[rm as usize][0] as u32)
            };
            let result = (1.0f32 / src).to_bits() as u64;
            self.regs.xmm[xmm_dst][0] = (self.regs.xmm[xmm_dst][0] & !0xFFFF_FFFF) | result;
        } else {
            let (src_lo, src_hi) = if is_memory {
                (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
            } else {
                (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
            };
            self.regs.xmm[xmm_dst][0] = rcp_packed_f32(src_lo);
            self.regs.xmm[xmm_dst][1] = rcp_packed_f32(src_hi);
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// SSE packed single/double min (0x5D)
    pub(in crate::backend::emulator::x86_64) fn execute_sse_min(
        &mut self,
        ctx: &mut InsnContext,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;

        if ctx.rep_prefix == Some(0xF3) {
            let src = if is_memory {
                f32::from_bits(self.read_mem(addr, 4)? as u32)
            } else {
                f32::from_bits(self.regs.xmm[rm as usize][0] as u32)
            };
            let dst = f32::from_bits(self.regs.xmm[xmm_dst][0] as u32);
            self.regs.xmm[xmm_dst][0] =
                (self.regs.xmm[xmm_dst][0] & !0xFFFFFFFF) | x86_min_f32(dst, src).to_bits() as u64;
        } else if ctx.rep_prefix == Some(0xF2) {
            let src = if is_memory {
                f64::from_bits(self.read_mem(addr, 8)?)
            } else {
                f64::from_bits(self.regs.xmm[rm as usize][0])
            };
            let dst = f64::from_bits(self.regs.xmm[xmm_dst][0]);
            self.regs.xmm[xmm_dst][0] = x86_min_f64(dst, src).to_bits();
        } else if ctx.operand_size_override {
            let (src_lo, src_hi) = if is_memory {
                (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
            } else {
                (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
            };
            self.regs.xmm[xmm_dst][0] = x86_min_f64(
                f64::from_bits(self.regs.xmm[xmm_dst][0]),
                f64::from_bits(src_lo),
            )
            .to_bits();
            self.regs.xmm[xmm_dst][1] = x86_min_f64(
                f64::from_bits(self.regs.xmm[xmm_dst][1]),
                f64::from_bits(src_hi),
            )
            .to_bits();
        } else {
            let (src_lo, src_hi) = if is_memory {
                (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
            } else {
                (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
            };
            let (dst_lo, dst_hi) = (self.regs.xmm[xmm_dst][0], self.regs.xmm[xmm_dst][1]);
            let r0 = x86_min_f32(f32::from_bits(dst_lo as u32), f32::from_bits(src_lo as u32));
            let r1 = x86_min_f32(
                f32::from_bits((dst_lo >> 32) as u32),
                f32::from_bits((src_lo >> 32) as u32),
            );
            let r2 = x86_min_f32(f32::from_bits(dst_hi as u32), f32::from_bits(src_hi as u32));
            let r3 = x86_min_f32(
                f32::from_bits((dst_hi >> 32) as u32),
                f32::from_bits((src_hi >> 32) as u32),
            );
            self.regs.xmm[xmm_dst][0] = r0.to_bits() as u64 | ((r1.to_bits() as u64) << 32);
            self.regs.xmm[xmm_dst][1] = r2.to_bits() as u64 | ((r3.to_bits() as u64) << 32);
        }
        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// SSE packed single/double max (0x5F)
    pub(in crate::backend::emulator::x86_64) fn execute_sse_max(
        &mut self,
        ctx: &mut InsnContext,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;

        if ctx.rep_prefix == Some(0xF3) {
            let src = if is_memory {
                f32::from_bits(self.read_mem(addr, 4)? as u32)
            } else {
                f32::from_bits(self.regs.xmm[rm as usize][0] as u32)
            };
            let dst = f32::from_bits(self.regs.xmm[xmm_dst][0] as u32);
            self.regs.xmm[xmm_dst][0] =
                (self.regs.xmm[xmm_dst][0] & !0xFFFFFFFF) | x86_max_f32(dst, src).to_bits() as u64;
        } else if ctx.rep_prefix == Some(0xF2) {
            let src = if is_memory {
                f64::from_bits(self.read_mem(addr, 8)?)
            } else {
                f64::from_bits(self.regs.xmm[rm as usize][0])
            };
            let dst = f64::from_bits(self.regs.xmm[xmm_dst][0]);
            self.regs.xmm[xmm_dst][0] = x86_max_f64(dst, src).to_bits();
        } else if ctx.operand_size_override {
            let (src_lo, src_hi) = if is_memory {
                (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
            } else {
                (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
            };
            self.regs.xmm[xmm_dst][0] = x86_max_f64(
                f64::from_bits(self.regs.xmm[xmm_dst][0]),
                f64::from_bits(src_lo),
            )
            .to_bits();
            self.regs.xmm[xmm_dst][1] = x86_max_f64(
                f64::from_bits(self.regs.xmm[xmm_dst][1]),
                f64::from_bits(src_hi),
            )
            .to_bits();
        } else {
            let (src_lo, src_hi) = if is_memory {
                (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
            } else {
                (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
            };
            let (dst_lo, dst_hi) = (self.regs.xmm[xmm_dst][0], self.regs.xmm[xmm_dst][1]);
            let r0 = x86_max_f32(f32::from_bits(dst_lo as u32), f32::from_bits(src_lo as u32));
            let r1 = x86_max_f32(
                f32::from_bits((dst_lo >> 32) as u32),
                f32::from_bits((src_lo >> 32) as u32),
            );
            let r2 = x86_max_f32(f32::from_bits(dst_hi as u32), f32::from_bits(src_hi as u32));
            let r3 = x86_max_f32(
                f32::from_bits((dst_hi >> 32) as u32),
                f32::from_bits((src_hi >> 32) as u32),
            );
            self.regs.xmm[xmm_dst][0] = r0.to_bits() as u64 | ((r1.to_bits() as u64) << 32);
            self.regs.xmm[xmm_dst][1] = r2.to_bits() as u64 | ((r3.to_bits() as u64) << 32);
        }
        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// SSE UNPCKLPS/UNPCKLPD (0x14) - unpack and interleave low
    pub(in crate::backend::emulator::x86_64) fn execute_sse_unpcklps(
        &mut self,
        ctx: &mut InsnContext,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;
        let (src_lo, _src_hi) = if is_memory {
            (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
        } else {
            (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
        };
        let dst_lo = self.regs.xmm[xmm_dst][0];

        if ctx.operand_size_override {
            // UNPCKLPD - interleave low doubles
            // dst[63:0] = dst[63:0], dst[127:64] = src[63:0]
            self.regs.xmm[xmm_dst][1] = src_lo;
        } else {
            // UNPCKLPS - interleave low singles
            // dst[31:0] = dst[31:0], dst[63:32] = src[31:0]
            // dst[95:64] = dst[63:32], dst[127:96] = src[63:32]
            let d0 = dst_lo as u32;
            let d1 = (dst_lo >> 32) as u32;
            let s0 = src_lo as u32;
            let s1 = (src_lo >> 32) as u32;
            self.regs.xmm[xmm_dst][0] = d0 as u64 | ((s0 as u64) << 32);
            self.regs.xmm[xmm_dst][1] = d1 as u64 | ((s1 as u64) << 32);
        }
        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// SSE UNPCKHPS/UNPCKHPD (0x15) - unpack and interleave high
    pub(in crate::backend::emulator::x86_64) fn execute_sse_unpckhps(
        &mut self,
        ctx: &mut InsnContext,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;
        let (_src_lo, src_hi) = if is_memory {
            (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
        } else {
            (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
        };
        let dst_hi = self.regs.xmm[xmm_dst][1];

        if ctx.operand_size_override {
            // UNPCKHPD - interleave high doubles
            // dst[63:0] = dst[127:64], dst[127:64] = src[127:64]
            self.regs.xmm[xmm_dst][0] = dst_hi;
            self.regs.xmm[xmm_dst][1] = src_hi;
        } else {
            // UNPCKHPS - interleave high singles
            // dst[31:0] = dst[95:64], dst[63:32] = src[95:64]
            // dst[95:64] = dst[127:96], dst[127:96] = src[127:96]
            let d2 = dst_hi as u32;
            let d3 = (dst_hi >> 32) as u32;
            let s2 = src_hi as u32;
            let s3 = (src_hi >> 32) as u32;
            self.regs.xmm[xmm_dst][0] = d2 as u64 | ((s2 as u64) << 32);
            self.regs.xmm[xmm_dst][1] = d3 as u64 | ((s3 as u64) << 32);
        }
        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// SSE2/MMX PUNPCK* (0x60/0x61/0x62/0x68/0x69/0x6A/0x6C/0x6D)
    pub(in crate::backend::emulator::x86_64) fn execute_punpck(
        &mut self,
        ctx: &mut InsnContext,
        opcode: u8,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;

        if ctx.operand_size_override {
            let xmm_dst = reg as usize;
            let (src2_lo, src2_hi) = if is_memory {
                (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
            } else {
                (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
            };
            let src1_lo = self.regs.xmm[xmm_dst][0];
            let src1_hi = self.regs.xmm[xmm_dst][1];

            match opcode {
                0x60 => {
                    self.regs.xmm[xmm_dst][0] = unpack_low_bytes(src1_lo, src2_lo);
                    self.regs.xmm[xmm_dst][1] = unpack_high_bytes(src1_lo, src2_lo);
                }
                0x61 => {
                    self.regs.xmm[xmm_dst][0] = unpack_low_words(src1_lo, src2_lo);
                    self.regs.xmm[xmm_dst][1] = unpack_high_words(src1_lo, src2_lo);
                }
                0x62 => {
                    let d0_src1 = src1_lo as u32;
                    let d0_src2 = src2_lo as u32;
                    let d1_src1 = (src1_lo >> 32) as u32;
                    let d1_src2 = (src2_lo >> 32) as u32;
                    self.regs.xmm[xmm_dst][0] = (d0_src1 as u64) | ((d0_src2 as u64) << 32);
                    self.regs.xmm[xmm_dst][1] = (d1_src1 as u64) | ((d1_src2 as u64) << 32);
                }
                0x68 => {
                    self.regs.xmm[xmm_dst][0] = unpack_low_bytes(src1_hi, src2_hi);
                    self.regs.xmm[xmm_dst][1] = unpack_high_bytes(src1_hi, src2_hi);
                }
                0x69 => {
                    self.regs.xmm[xmm_dst][0] = unpack_low_words(src1_hi, src2_hi);
                    self.regs.xmm[xmm_dst][1] = unpack_high_words(src1_hi, src2_hi);
                }
                0x6A => {
                    let d0_src1 = src1_hi as u32;
                    let d0_src2 = src2_hi as u32;
                    let d1_src1 = (src1_hi >> 32) as u32;
                    let d1_src2 = (src2_hi >> 32) as u32;
                    self.regs.xmm[xmm_dst][0] = (d0_src1 as u64) | ((d0_src2 as u64) << 32);
                    self.regs.xmm[xmm_dst][1] = (d1_src1 as u64) | ((d1_src2 as u64) << 32);
                }
                0x6C => {
                    self.regs.xmm[xmm_dst][0] = src1_lo;
                    self.regs.xmm[xmm_dst][1] = src2_lo;
                }
                0x6D => {
                    self.regs.xmm[xmm_dst][0] = src1_hi;
                    self.regs.xmm[xmm_dst][1] = src2_hi;
                }
                _ => unreachable!(),
            }
        } else {
            if opcode == 0x6C || opcode == 0x6D {
                return Err(Error::Emulator(format!(
                    "unimplemented 0x0F {:#04x} opcode variant at RIP={:#x}",
                    opcode, self.regs.rip
                )));
            }

            let mm_dst = reg as usize & 0x7;
            let mm_src = rm as usize & 0x7;
            let src = if is_memory {
                let size = match opcode {
                    0x60 | 0x61 | 0x62 => 4,
                    _ => 8,
                };
                self.read_mem(addr, size)?
            } else {
                self.regs.mm[mm_src]
            };
            let dst = self.regs.mm[mm_dst];

            let result = match opcode {
                0x60 => unpack_low_bytes(dst, src),
                0x61 => unpack_low_words(dst, src),
                0x62 => {
                    let d0_dst = dst as u32;
                    let d0_src = src as u32;
                    (d0_dst as u64) | ((d0_src as u64) << 32)
                }
                0x68 => unpack_high_bytes(dst, src),
                0x69 => unpack_high_words(dst, src),
                0x6A => {
                    let d1_dst = (dst >> 32) as u32;
                    let d1_src = (src >> 32) as u32;
                    (d1_dst as u64) | ((d1_src as u64) << 32)
                }
                _ => unreachable!(),
            };

            self.regs.mm[mm_dst] = result;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// SSE2/MMX PINSRW (0x0F 0xC4)
    pub(in crate::backend::emulator::x86_64) fn execute_pinsrw(
        &mut self,
        ctx: &mut InsnContext,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let imm8 = ctx.consume_u8()?;
        let word = if is_memory {
            self.read_mem(addr, 2)? as u16
        } else {
            self.get_reg(rm, 2) as u16
        };

        if ctx.operand_size_override {
            let xmm_dst = reg as usize;
            let idx = (imm8 & 0x07) as usize;
            if idx < 4 {
                let shift = idx * 16;
                let mask = !(0xFFFFu64 << shift);
                self.regs.xmm[xmm_dst][0] =
                    (self.regs.xmm[xmm_dst][0] & mask) | ((word as u64) << shift);
            } else {
                let shift = (idx - 4) * 16;
                let mask = !(0xFFFFu64 << shift);
                self.regs.xmm[xmm_dst][1] =
                    (self.regs.xmm[xmm_dst][1] & mask) | ((word as u64) << shift);
            }
        } else {
            let mm_dst = reg as usize & 0x7;
            let idx = (imm8 & 0x03) as usize;
            let shift = idx * 16;
            let mask = !(0xFFFFu64 << shift);
            self.regs.mm[mm_dst] = (self.regs.mm[mm_dst] & mask) | ((word as u64) << shift);
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// SSE2/MMX PEXTRW (0x0F 0xC5)
    pub(in crate::backend::emulator::x86_64) fn execute_pextrw(
        &mut self,
        ctx: &mut InsnContext,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, _addr, _) = self.decode_modrm(ctx)?;
        if is_memory {
            return Err(Error::Emulator(
                "PEXTRW does not support memory source".to_string(),
            ));
        }
        let imm8 = ctx.consume_u8()?;

        let word = if ctx.operand_size_override {
            let xmm_src = rm as usize;
            let idx = (imm8 & 0x07) as usize;
            if idx < 4 {
                ((self.regs.xmm[xmm_src][0] >> (idx * 16)) & 0xFFFF) as u16
            } else {
                ((self.regs.xmm[xmm_src][1] >> ((idx - 4) * 16)) & 0xFFFF) as u16
            }
        } else {
            let mm_src = rm as usize & 0x7;
            let idx = (imm8 & 0x03) as usize;
            ((self.regs.mm[mm_src] >> (idx * 16)) & 0xFFFF) as u16
        };

        let dest_size = if ctx.rex_w() { 8 } else { 4 };
        self.set_reg(reg, word as u64, dest_size);
        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// SSE PSHUFD/PSHUFHW/PSHUFLW (0x0F 0x70)
    pub(in crate::backend::emulator::x86_64) fn execute_pshufd(
        &mut self,
        ctx: &mut InsnContext,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let imm8 = ctx.consume_u8()?;
        let xmm_dst = reg as usize;

        if ctx.rep_prefix == Some(0xF3) {
            // PSHUFHW: shuffle high words, preserve low qword
            // F3 0F 70 /r ib
            let (src_lo, src_hi) = if is_memory {
                (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
            } else {
                (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
            };
            // Low qword unchanged
            self.regs.xmm[xmm_dst][0] = src_lo;
            // High qword: shuffle the 4 words in src_hi
            let w0 = (src_hi >> (((imm8 >> 0) & 3) * 16)) as u16;
            let w1 = (src_hi >> (((imm8 >> 2) & 3) * 16)) as u16;
            let w2 = (src_hi >> (((imm8 >> 4) & 3) * 16)) as u16;
            let w3 = (src_hi >> (((imm8 >> 6) & 3) * 16)) as u16;
            self.regs.xmm[xmm_dst][1] =
                (w0 as u64) | ((w1 as u64) << 16) | ((w2 as u64) << 32) | ((w3 as u64) << 48);
        } else if ctx.rep_prefix == Some(0xF2) {
            // PSHUFLW: shuffle low words, preserve high qword
            // F2 0F 70 /r ib
            let (src_lo, src_hi) = if is_memory {
                (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
            } else {
                (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
            };
            // Low qword: shuffle the 4 words in src_lo
            let w0 = (src_lo >> (((imm8 >> 0) & 3) * 16)) as u16;
            let w1 = (src_lo >> (((imm8 >> 2) & 3) * 16)) as u16;
            let w2 = (src_lo >> (((imm8 >> 4) & 3) * 16)) as u16;
            let w3 = (src_lo >> (((imm8 >> 6) & 3) * 16)) as u16;
            self.regs.xmm[xmm_dst][0] =
                (w0 as u64) | ((w1 as u64) << 16) | ((w2 as u64) << 32) | ((w3 as u64) << 48);
            // High qword unchanged
            self.regs.xmm[xmm_dst][1] = src_hi;
        } else if ctx.operand_size_override {
            // PSHUFD: shuffle all 4 dwords
            // 66 0F 70 /r ib
            let (src_lo, src_hi) = if is_memory {
                (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
            } else {
                (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
            };
            // Combine into 4 dwords for easier indexing
            let dwords: [u32; 4] = [
                src_lo as u32,
                (src_lo >> 32) as u32,
                src_hi as u32,
                (src_hi >> 32) as u32,
            ];
            let d0 = dwords[((imm8 >> 0) & 3) as usize];
            let d1 = dwords[((imm8 >> 2) & 3) as usize];
            let d2 = dwords[((imm8 >> 4) & 3) as usize];
            let d3 = dwords[((imm8 >> 6) & 3) as usize];
            self.regs.xmm[xmm_dst][0] = (d0 as u64) | ((d1 as u64) << 32);
            self.regs.xmm[xmm_dst][1] = (d2 as u64) | ((d3 as u64) << 32);
        } else {
            // PSHUFW: shuffle MMX words (NP 0F 70)
            // mm1, mm2/m64, imm8 - shuffle 4 words in 64-bit MMX register
            let mm_dst = reg as usize & 0x7;
            let mm_src = rm as usize & 0x7;
            let src = if is_memory {
                self.read_mem(addr, 8)?
            } else {
                self.regs.mm[mm_src]
            };
            // Shuffle 4 16-bit words based on imm8
            let w0 = (src >> (((imm8 >> 0) & 3) * 16)) as u16;
            let w1 = (src >> (((imm8 >> 2) & 3) * 16)) as u16;
            let w2 = (src >> (((imm8 >> 4) & 3) * 16)) as u16;
            let w3 = (src >> (((imm8 >> 6) & 3) * 16)) as u16;
            self.regs.mm[mm_dst] =
                (w0 as u64) | ((w1 as u64) << 16) | ((w2 as u64) << 32) | ((w3 as u64) << 48);
        }
        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// SSE CMPPS/CMPPD/CMPSS/CMPSD (0x0F 0xC2)
    pub(in crate::backend::emulator::x86_64) fn execute_cmpps(
        &mut self,
        ctx: &mut InsnContext,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let imm8 = ctx.consume_u8()?;
        let xmm_dst = reg as usize;

        if ctx.rep_prefix == Some(0xF3) {
            // CMPSS - scalar single
            let src = if is_memory {
                f32::from_bits(self.read_mem(addr, 4)? as u32)
            } else {
                f32::from_bits(self.regs.xmm[rm as usize][0] as u32)
            };
            let dst = f32::from_bits(self.regs.xmm[xmm_dst][0] as u32);
            let result = if self.cmp_predicate_f32(dst, src, imm8) {
                0xFFFFFFFFu32
            } else {
                0u32
            };
            self.regs.xmm[xmm_dst][0] = (self.regs.xmm[xmm_dst][0] & !0xFFFFFFFF) | result as u64;
        } else if ctx.rep_prefix == Some(0xF2) {
            // CMPSD - scalar double
            let src = if is_memory {
                f64::from_bits(self.read_mem(addr, 8)?)
            } else {
                f64::from_bits(self.regs.xmm[rm as usize][0])
            };
            let dst = f64::from_bits(self.regs.xmm[xmm_dst][0]);
            let result = if self.cmp_predicate_f64(dst, src, imm8) {
                !0u64
            } else {
                0u64
            };
            self.regs.xmm[xmm_dst][0] = result;
        } else if ctx.operand_size_override {
            // CMPPD - packed double
            let (src_lo, src_hi) = if is_memory {
                (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
            } else {
                (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
            };
            let r0 = if self.cmp_predicate_f64(
                f64::from_bits(self.regs.xmm[xmm_dst][0]),
                f64::from_bits(src_lo),
                imm8,
            ) {
                !0u64
            } else {
                0u64
            };
            let r1 = if self.cmp_predicate_f64(
                f64::from_bits(self.regs.xmm[xmm_dst][1]),
                f64::from_bits(src_hi),
                imm8,
            ) {
                !0u64
            } else {
                0u64
            };
            self.regs.xmm[xmm_dst][0] = r0;
            self.regs.xmm[xmm_dst][1] = r1;
        } else {
            // CMPPS - packed single
            let (src_lo, src_hi) = if is_memory {
                (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
            } else {
                (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
            };
            let dst_lo = self.regs.xmm[xmm_dst][0];
            let dst_hi = self.regs.xmm[xmm_dst][1];
            let r0 = if self.cmp_predicate_f32(
                f32::from_bits(dst_lo as u32),
                f32::from_bits(src_lo as u32),
                imm8,
            ) {
                0xFFFFFFFFu32
            } else {
                0
            };
            let r1 = if self.cmp_predicate_f32(
                f32::from_bits((dst_lo >> 32) as u32),
                f32::from_bits((src_lo >> 32) as u32),
                imm8,
            ) {
                0xFFFFFFFFu32
            } else {
                0
            };
            let r2 = if self.cmp_predicate_f32(
                f32::from_bits(dst_hi as u32),
                f32::from_bits(src_hi as u32),
                imm8,
            ) {
                0xFFFFFFFFu32
            } else {
                0
            };
            let r3 = if self.cmp_predicate_f32(
                f32::from_bits((dst_hi >> 32) as u32),
                f32::from_bits((src_hi >> 32) as u32),
                imm8,
            ) {
                0xFFFFFFFFu32
            } else {
                0
            };
            self.regs.xmm[xmm_dst][0] = r0 as u64 | ((r1 as u64) << 32);
            self.regs.xmm[xmm_dst][1] = r2 as u64 | ((r3 as u64) << 32);
        }
        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// Helper for float comparison predicates (bits 2:0 for SSE, 4:0 for AVX)
    pub(in crate::backend::emulator::x86_64) fn cmp_predicate_f32(
        &self,
        a: f32,
        b: f32,
        pred: u8,
    ) -> bool {
        match pred & 0x1F {
            0x00 => a == b,                                // EQ_OQ
            0x01 => a < b,                                 // LT_OS
            0x02 => a <= b,                                // LE_OS
            0x03 => a.is_nan() || b.is_nan(),              // UNORD_Q
            0x04 => a != b || a.is_nan() || b.is_nan(),    // NEQ_UQ
            0x05 => !(a < b),                              // NLT_US
            0x06 => !(a <= b),                             // NLE_US
            0x07 => !a.is_nan() && !b.is_nan(),            // ORD_Q
            0x08 => a == b || a.is_nan() || b.is_nan(),    // EQ_UQ
            0x09 => !(a >= b),                             // NGE_US
            0x0A => !(a > b),                              // NGT_US
            0x0B => false,                                 // FALSE_OQ
            0x0C => a != b,                                // NEQ_OQ
            0x0D => a >= b,                                // GE_OS
            0x0E => a > b,                                 // GT_OS
            0x0F => true,                                  // TRUE_UQ
            0x10 => a == b,                                // EQ_OS
            0x11 => a < b || a.is_nan() || b.is_nan(),     // LT_OQ
            0x12 => a <= b || a.is_nan() || b.is_nan(),    // LE_OQ
            0x13 => a.is_nan() || b.is_nan(),              // UNORD_S
            0x14 => a != b,                                // NEQ_US
            0x15 => !(a < b) || a.is_nan() || b.is_nan(),  // NLT_UQ
            0x16 => !(a <= b) || a.is_nan() || b.is_nan(), // NLE_UQ
            0x17 => !a.is_nan() && !b.is_nan(),            // ORD_S
            0x18 => a == b,                                // EQ_US
            0x19 => !(a >= b) || a.is_nan() || b.is_nan(), // NGE_UQ
            0x1A => !(a > b) || a.is_nan() || b.is_nan(),  // NGT_UQ
            0x1B => false,                                 // FALSE_OS
            0x1C => a != b || a.is_nan() || b.is_nan(),    // NEQ_OS
            0x1D => a >= b || a.is_nan() || b.is_nan(),    // GE_OQ
            0x1E => a > b || a.is_nan() || b.is_nan(),     // GT_OQ
            0x1F => true,                                  // TRUE_US
            _ => false,
        }
    }

    pub(in crate::backend::emulator::x86_64) fn cmp_predicate_f64(
        &self,
        a: f64,
        b: f64,
        pred: u8,
    ) -> bool {
        match pred & 0x1F {
            0x00 => a == b,
            0x01 => a < b,
            0x02 => a <= b,
            0x03 => a.is_nan() || b.is_nan(),
            0x04 => a != b || a.is_nan() || b.is_nan(),
            0x05 => !(a < b),
            0x06 => !(a <= b),
            0x07 => !a.is_nan() && !b.is_nan(),
            0x08 => a == b || a.is_nan() || b.is_nan(),
            0x09 => !(a >= b),
            0x0A => !(a > b),
            0x0B => false,
            0x0C => a != b,
            0x0D => a >= b,
            0x0E => a > b,
            0x0F => true,
            0x10 => a == b,
            0x11 => a < b || a.is_nan() || b.is_nan(),
            0x12 => a <= b || a.is_nan() || b.is_nan(),
            0x13 => a.is_nan() || b.is_nan(),
            0x14 => a != b,
            0x15 => !(a < b) || a.is_nan() || b.is_nan(),
            0x16 => !(a <= b) || a.is_nan() || b.is_nan(),
            0x17 => !a.is_nan() && !b.is_nan(),
            0x18 => a == b,
            0x19 => !(a >= b) || a.is_nan() || b.is_nan(),
            0x1A => !(a > b) || a.is_nan() || b.is_nan(),
            0x1B => false,
            0x1C => a != b || a.is_nan() || b.is_nan(),
            0x1D => a >= b || a.is_nan() || b.is_nan(),
            0x1E => a > b || a.is_nan() || b.is_nan(),
            0x1F => true,
            _ => false,
        }
    }

    /// MOVNTPS/MOVNTPD - Store packed with non-temporal hint (0x0F 0x2B)
    pub(in crate::backend::emulator::x86_64) fn execute_movnt_store(
        &mut self,
        ctx: &mut InsnContext,
    ) -> Result<Option<VcpuExit>> {
        let (reg, _rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        if !is_memory {
            return Err(Error::Emulator(
                "MOVNTPS/MOVNTPD requires memory destination".to_string(),
            ));
        }
        let xmm_src = reg as usize;
        // Non-temporal hint is ignored in emulator - just store normally
        self.write_mem(addr, self.regs.xmm[xmm_src][0], 8)?;
        self.write_mem(addr + 8, self.regs.xmm[xmm_src][1], 8)?;
        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// COMISS/COMISD - Compare scalar and set EFLAGS (0x0F 0x2F)
    pub(in crate::backend::emulator::x86_64) fn execute_comiss(
        &mut self,
        ctx: &mut InsnContext,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;

        // Clear OF, SF, AF flags; set ZF, PF, CF based on comparison
        // Clear lazy flags before setting flags directly
        self.clear_lazy_flags();
        self.regs.rflags &= !(flags::bits::OF | flags::bits::SF | flags::bits::AF);

        if ctx.operand_size_override {
            // COMISD - compare scalar double
            let src = if is_memory {
                f64::from_bits(self.read_mem(addr, 8)?)
            } else {
                f64::from_bits(self.regs.xmm[rm as usize][0])
            };
            let dst = f64::from_bits(self.regs.xmm[xmm_dst][0]);

            if dst.is_nan() || src.is_nan() {
                // Unordered: ZF=1, PF=1, CF=1
                self.regs.rflags |= flags::bits::ZF | flags::bits::PF | flags::bits::CF;
            } else if dst > src {
                // Greater: ZF=0, PF=0, CF=0
                self.regs.rflags &= !(flags::bits::ZF | flags::bits::PF | flags::bits::CF);
            } else if dst < src {
                // Less: ZF=0, PF=0, CF=1
                self.regs.rflags &= !(flags::bits::ZF | flags::bits::PF);
                self.regs.rflags |= flags::bits::CF;
            } else {
                // Equal: ZF=1, PF=0, CF=0
                self.regs.rflags &= !(flags::bits::PF | flags::bits::CF);
                self.regs.rflags |= flags::bits::ZF;
            }
        } else {
            // COMISS - compare scalar single
            let src = if is_memory {
                f32::from_bits(self.read_mem(addr, 4)? as u32)
            } else {
                f32::from_bits(self.regs.xmm[rm as usize][0] as u32)
            };
            let dst = f32::from_bits(self.regs.xmm[xmm_dst][0] as u32);

            if dst.is_nan() || src.is_nan() {
                self.regs.rflags |= flags::bits::ZF | flags::bits::PF | flags::bits::CF;
            } else if dst > src {
                self.regs.rflags &= !(flags::bits::ZF | flags::bits::PF | flags::bits::CF);
            } else if dst < src {
                self.regs.rflags &= !(flags::bits::ZF | flags::bits::PF);
                self.regs.rflags |= flags::bits::CF;
            } else {
                self.regs.rflags &= !(flags::bits::PF | flags::bits::CF);
                self.regs.rflags |= flags::bits::ZF;
            }
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// MOVMSKPS/MOVMSKPD - Extract sign bits (0x0F 0x50)
    pub(in crate::backend::emulator::x86_64) fn execute_movmsk(
        &mut self,
        ctx: &mut InsnContext,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, _addr, _) = self.decode_modrm(ctx)?;
        if is_memory {
            return Err(Error::Emulator(
                "MOVMSKPS/MOVMSKPD requires register source".to_string(),
            ));
        }
        let xmm_src = rm as usize;

        let result = if ctx.operand_size_override {
            // MOVMSKPD - extract from 2 doubles
            let b0 = ((self.regs.xmm[xmm_src][0] >> 63) & 1) as u32;
            let b1 = ((self.regs.xmm[xmm_src][1] >> 63) & 1) as u32;
            b0 | (b1 << 1)
        } else {
            // MOVMSKPS - extract from 4 singles
            let lo = self.regs.xmm[xmm_src][0];
            let hi = self.regs.xmm[xmm_src][1];
            let b0 = ((lo >> 31) & 1) as u32;
            let b1 = ((lo >> 63) & 1) as u32;
            let b2 = ((hi >> 31) & 1) as u32;
            let b3 = ((hi >> 63) & 1) as u32;
            b0 | (b1 << 1) | (b2 << 2) | (b3 << 3)
        };

        // Store in GPR (zero-extend to 32 or 64 bits)
        let dest_size = if ctx.rex_w() { 8 } else { 4 };
        self.set_reg(reg, result as u64, dest_size);
        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// SHUFPS/SHUFPD - Shuffle packed (0x0F 0xC6)
    pub(in crate::backend::emulator::x86_64) fn execute_shufps(
        &mut self,
        ctx: &mut InsnContext,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let imm8 = ctx.consume_u8()?;
        let xmm_dst = reg as usize;

        if ctx.operand_size_override {
            // SHUFPD - shuffle 2 doubles
            // dst[63:0] = select from dst based on imm8[0]
            // dst[127:64] = select from src based on imm8[1]
            let (src_lo, src_hi) = if is_memory {
                (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
            } else {
                (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
            };
            let dst_lo = self.regs.xmm[xmm_dst][0];
            let dst_hi = self.regs.xmm[xmm_dst][1];

            let r0 = if (imm8 & 1) == 0 { dst_lo } else { dst_hi };
            let r1 = if (imm8 & 2) == 0 { src_lo } else { src_hi };
            self.regs.xmm[xmm_dst][0] = r0;
            self.regs.xmm[xmm_dst][1] = r1;
        } else {
            // SHUFPS - shuffle 4 singles
            // dst[31:0] = dst[(imm8[1:0])*32 +31 : (imm8[1:0])*32]
            // dst[63:32] = dst[(imm8[3:2])*32 +31 : (imm8[3:2])*32]
            // dst[95:64] = src[(imm8[5:4])*32 +31 : (imm8[5:4])*32]
            // dst[127:96] = src[(imm8[7:6])*32 +31 : (imm8[7:6])*32]
            let (src_lo, src_hi) = if is_memory {
                (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
            } else {
                (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
            };
            let dst_lo = self.regs.xmm[xmm_dst][0];
            let dst_hi = self.regs.xmm[xmm_dst][1];

            // Combine into arrays for easier indexing
            let dst_dwords: [u32; 4] = [
                dst_lo as u32,
                (dst_lo >> 32) as u32,
                dst_hi as u32,
                (dst_hi >> 32) as u32,
            ];
            let src_dwords: [u32; 4] = [
                src_lo as u32,
                (src_lo >> 32) as u32,
                src_hi as u32,
                (src_hi >> 32) as u32,
            ];

            let r0 = dst_dwords[((imm8 >> 0) & 3) as usize];
            let r1 = dst_dwords[((imm8 >> 2) & 3) as usize];
            let r2 = src_dwords[((imm8 >> 4) & 3) as usize];
            let r3 = src_dwords[((imm8 >> 6) & 3) as usize];

            self.regs.xmm[xmm_dst][0] = (r0 as u64) | ((r1 as u64) << 32);
            self.regs.xmm[xmm_dst][1] = (r2 as u64) | ((r3 as u64) << 32);
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// HADDPS/HADDPD - Horizontal add (0x0F 0x7C)
    pub(in crate::backend::emulator::x86_64) fn execute_hadd(
        &mut self,
        ctx: &mut InsnContext,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;

        if ctx.operand_size_override {
            // HADDPD (66 0F 7C) - horizontal add packed double
            // dst[63:0] = dst[63:0] + dst[127:64]
            // dst[127:64] = src[63:0] + src[127:64]
            let (src_lo, src_hi) = if is_memory {
                (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
            } else {
                (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
            };
            let dst_lo = self.regs.xmm[xmm_dst][0];
            let dst_hi = self.regs.xmm[xmm_dst][1];

            let r0 = f64::from_bits(dst_lo) + f64::from_bits(dst_hi);
            let r1 = f64::from_bits(src_lo) + f64::from_bits(src_hi);
            self.regs.xmm[xmm_dst][0] = r0.to_bits();
            self.regs.xmm[xmm_dst][1] = r1.to_bits();
        } else if ctx.rep_prefix == Some(0xF2) {
            // HADDPS (F2 0F 7C) - horizontal add packed single
            // dst[31:0] = dst[31:0] + dst[63:32]
            // dst[63:32] = dst[95:64] + dst[127:96]
            // dst[95:64] = src[31:0] + src[63:32]
            // dst[127:96] = src[95:64] + src[127:96]
            let (src_lo, src_hi) = if is_memory {
                (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
            } else {
                (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
            };
            let dst_lo = self.regs.xmm[xmm_dst][0];
            let dst_hi = self.regs.xmm[xmm_dst][1];

            let d0 = f32::from_bits(dst_lo as u32);
            let d1 = f32::from_bits((dst_lo >> 32) as u32);
            let d2 = f32::from_bits(dst_hi as u32);
            let d3 = f32::from_bits((dst_hi >> 32) as u32);
            let s0 = f32::from_bits(src_lo as u32);
            let s1 = f32::from_bits((src_lo >> 32) as u32);
            let s2 = f32::from_bits(src_hi as u32);
            let s3 = f32::from_bits((src_hi >> 32) as u32);

            let r0 = (d0 + d1).to_bits();
            let r1 = (d2 + d3).to_bits();
            let r2 = (s0 + s1).to_bits();
            let r3 = (s2 + s3).to_bits();

            self.regs.xmm[xmm_dst][0] = (r0 as u64) | ((r1 as u64) << 32);
            self.regs.xmm[xmm_dst][1] = (r2 as u64) | ((r3 as u64) << 32);
        } else {
            return Err(Error::Emulator(format!(
                "unimplemented 0x0F 0x7C opcode variant at RIP={:#x}",
                self.regs.rip
            )));
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// HSUBPS/HSUBPD - Horizontal subtract (0x0F 0x7D)
    pub(in crate::backend::emulator::x86_64) fn execute_hsub(
        &mut self,
        ctx: &mut InsnContext,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;

        if ctx.operand_size_override {
            // HSUBPD (66 0F 7D) - horizontal subtract packed double
            let (src_lo, src_hi) = if is_memory {
                (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
            } else {
                (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
            };
            let dst_lo = self.regs.xmm[xmm_dst][0];
            let dst_hi = self.regs.xmm[xmm_dst][1];

            let r0 = f64::from_bits(dst_lo) - f64::from_bits(dst_hi);
            let r1 = f64::from_bits(src_lo) - f64::from_bits(src_hi);
            self.regs.xmm[xmm_dst][0] = r0.to_bits();
            self.regs.xmm[xmm_dst][1] = r1.to_bits();
        } else if ctx.rep_prefix == Some(0xF2) {
            // HSUBPS (F2 0F 7D) - horizontal subtract packed single
            let (src_lo, src_hi) = if is_memory {
                (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
            } else {
                (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
            };
            let dst_lo = self.regs.xmm[xmm_dst][0];
            let dst_hi = self.regs.xmm[xmm_dst][1];

            let d0 = f32::from_bits(dst_lo as u32);
            let d1 = f32::from_bits((dst_lo >> 32) as u32);
            let d2 = f32::from_bits(dst_hi as u32);
            let d3 = f32::from_bits((dst_hi >> 32) as u32);
            let s0 = f32::from_bits(src_lo as u32);
            let s1 = f32::from_bits((src_lo >> 32) as u32);
            let s2 = f32::from_bits(src_hi as u32);
            let s3 = f32::from_bits((src_hi >> 32) as u32);

            let r0 = (d0 - d1).to_bits();
            let r1 = (d2 - d3).to_bits();
            let r2 = (s0 - s1).to_bits();
            let r3 = (s2 - s3).to_bits();

            self.regs.xmm[xmm_dst][0] = (r0 as u64) | ((r1 as u64) << 32);
            self.regs.xmm[xmm_dst][1] = (r2 as u64) | ((r3 as u64) << 32);
        } else {
            return Err(Error::Emulator(format!(
                "unimplemented 0x0F 0x7D opcode variant at RIP={:#x}",
                self.regs.rip
            )));
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }
    /// SSE2/MMX shift immediate group 12 (0x0F 0x71)
    /// PSRLW/PSRAW/PSLLW xmm, imm8
    pub(in crate::backend::emulator::x86_64) fn execute_shift_imm_group12(
        &mut self,
        ctx: &mut InsnContext,
    ) -> Result<Option<VcpuExit>> {
        let modrm = ctx.consume_u8()?;
        let reg = (modrm >> 3) & 0x07; // operation type
        let rm = modrm & 0x07;
        let imm8 = ctx.consume_u8()?;

        if ctx.operand_size_override {
            let xmm = rm as usize;
            let shift = imm8 as u32;

            match reg {
                2 => {
                    // PSRLW - shift right logical words
                    if shift >= 16 {
                        self.regs.xmm[xmm][0] = 0;
                        self.regs.xmm[xmm][1] = 0;
                    } else {
                        self.regs.xmm[xmm][0] = shift_right_words(self.regs.xmm[xmm][0], shift);
                        self.regs.xmm[xmm][1] = shift_right_words(self.regs.xmm[xmm][1], shift);
                    }
                }
                4 => {
                    // PSRAW - shift right arithmetic words
                    self.regs.xmm[xmm][0] = shift_right_arith_words(self.regs.xmm[xmm][0], shift);
                    self.regs.xmm[xmm][1] = shift_right_arith_words(self.regs.xmm[xmm][1], shift);
                }
                6 => {
                    // PSLLW - shift left logical words
                    if shift >= 16 {
                        self.regs.xmm[xmm][0] = 0;
                        self.regs.xmm[xmm][1] = 0;
                    } else {
                        self.regs.xmm[xmm][0] = shift_left_words(self.regs.xmm[xmm][0], shift);
                        self.regs.xmm[xmm][1] = shift_left_words(self.regs.xmm[xmm][1], shift);
                    }
                }
                _ => {
                    return Err(Error::Emulator(format!(
                        "unimplemented 0x0F 0x71 /r{} at RIP={:#x}",
                        reg, self.regs.rip
                    )));
                }
            }
        } else {
            // MMX version
            let mm = (rm & 0x7) as usize;
            let shift = imm8 as u32;
            match reg {
                2 => {
                    if shift >= 16 {
                        self.regs.mm[mm] = 0;
                    } else {
                        self.regs.mm[mm] = shift_right_words(self.regs.mm[mm], shift);
                    }
                }
                4 => {
                    self.regs.mm[mm] = shift_right_arith_words(self.regs.mm[mm], shift);
                }
                6 => {
                    if shift >= 16 {
                        self.regs.mm[mm] = 0;
                    } else {
                        self.regs.mm[mm] = shift_left_words(self.regs.mm[mm], shift);
                    }
                }
                _ => {
                    return Err(Error::Emulator(format!(
                        "unimplemented MMX 0x0F 0x71 /r{} at RIP={:#x}",
                        reg, self.regs.rip
                    )));
                }
            }
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// SSE2/MMX shift immediate group 13 (0x0F 0x72)
    /// PSRLD/PSRAD/PSLLD xmm, imm8
    pub(in crate::backend::emulator::x86_64) fn execute_shift_imm_group13(
        &mut self,
        ctx: &mut InsnContext,
    ) -> Result<Option<VcpuExit>> {
        let modrm = ctx.consume_u8()?;
        let reg = (modrm >> 3) & 0x07;
        let rm = modrm & 0x07;
        let imm8 = ctx.consume_u8()?;

        if ctx.operand_size_override {
            let xmm = rm as usize;
            let shift = imm8 as u32;

            match reg {
                2 => {
                    // PSRLD
                    if shift >= 32 {
                        self.regs.xmm[xmm][0] = 0;
                        self.regs.xmm[xmm][1] = 0;
                    } else {
                        self.regs.xmm[xmm][0] = shift_right_dwords(self.regs.xmm[xmm][0], shift);
                        self.regs.xmm[xmm][1] = shift_right_dwords(self.regs.xmm[xmm][1], shift);
                    }
                }
                4 => {
                    // PSRAD
                    self.regs.xmm[xmm][0] = shift_right_arith_dwords(self.regs.xmm[xmm][0], shift);
                    self.regs.xmm[xmm][1] = shift_right_arith_dwords(self.regs.xmm[xmm][1], shift);
                }
                6 => {
                    // PSLLD
                    if shift >= 32 {
                        self.regs.xmm[xmm][0] = 0;
                        self.regs.xmm[xmm][1] = 0;
                    } else {
                        self.regs.xmm[xmm][0] = shift_left_dwords(self.regs.xmm[xmm][0], shift);
                        self.regs.xmm[xmm][1] = shift_left_dwords(self.regs.xmm[xmm][1], shift);
                    }
                }
                _ => {
                    return Err(Error::Emulator(format!(
                        "unimplemented 0x0F 0x72 /r{} at RIP={:#x}",
                        reg, self.regs.rip
                    )));
                }
            }
        } else {
            // MMX version
            let mm = (rm & 0x7) as usize;
            let shift = imm8 as u32;
            match reg {
                2 => {
                    if shift >= 32 {
                        self.regs.mm[mm] = 0;
                    } else {
                        self.regs.mm[mm] = shift_right_dwords(self.regs.mm[mm], shift);
                    }
                }
                4 => {
                    self.regs.mm[mm] = shift_right_arith_dwords(self.regs.mm[mm], shift);
                }
                6 => {
                    if shift >= 32 {
                        self.regs.mm[mm] = 0;
                    } else {
                        self.regs.mm[mm] = shift_left_dwords(self.regs.mm[mm], shift);
                    }
                }
                _ => {
                    return Err(Error::Emulator(format!(
                        "unimplemented MMX 0x0F 0x72 /r{} at RIP={:#x}",
                        reg, self.regs.rip
                    )));
                }
            }
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// SSE2 shift immediate group 14 (0x0F 0x73)
    /// PSRLQ/PSRLDQ/PSLLQ/PSLLDQ xmm, imm8
    pub(in crate::backend::emulator::x86_64) fn execute_shift_imm_group14(
        &mut self,
        ctx: &mut InsnContext,
    ) -> Result<Option<VcpuExit>> {
        let modrm = ctx.consume_u8()?;
        let reg = (modrm >> 3) & 0x07;
        let rm = modrm & 0x07;
        let imm8 = ctx.consume_u8()?;

        if ctx.operand_size_override {
            let xmm = rm as usize;
            let shift = imm8 as u32;

            match reg {
                2 => {
                    // PSRLQ
                    if shift >= 64 {
                        self.regs.xmm[xmm][0] = 0;
                        self.regs.xmm[xmm][1] = 0;
                    } else {
                        self.regs.xmm[xmm][0] >>= shift;
                        self.regs.xmm[xmm][1] >>= shift;
                    }
                }
                3 => {
                    // PSRLDQ - shift right bytes (whole 128-bit value)
                    let shift_bytes = (imm8 as usize).min(16);
                    let lo = self.regs.xmm[xmm][0];
                    let hi = self.regs.xmm[xmm][1];
                    if shift_bytes >= 16 {
                        self.regs.xmm[xmm][0] = 0;
                        self.regs.xmm[xmm][1] = 0;
                    } else if shift_bytes >= 8 {
                        self.regs.xmm[xmm][0] = hi >> ((shift_bytes - 8) * 8);
                        self.regs.xmm[xmm][1] = 0;
                    } else if shift_bytes == 0 {
                        // No shift, keep values unchanged
                    } else {
                        let shift_bits = shift_bytes * 8;
                        self.regs.xmm[xmm][0] = (lo >> shift_bits) | (hi << (64 - shift_bits));
                        self.regs.xmm[xmm][1] = hi >> shift_bits;
                    }
                }
                6 => {
                    // PSLLQ
                    if shift >= 64 {
                        self.regs.xmm[xmm][0] = 0;
                        self.regs.xmm[xmm][1] = 0;
                    } else {
                        self.regs.xmm[xmm][0] <<= shift;
                        self.regs.xmm[xmm][1] <<= shift;
                    }
                }
                7 => {
                    // PSLLDQ - shift left bytes (whole 128-bit value)
                    let shift_bytes = (imm8 as usize).min(16);
                    let lo = self.regs.xmm[xmm][0];
                    let hi = self.regs.xmm[xmm][1];
                    if shift_bytes >= 16 {
                        self.regs.xmm[xmm][0] = 0;
                        self.regs.xmm[xmm][1] = 0;
                    } else if shift_bytes >= 8 {
                        self.regs.xmm[xmm][1] = lo << ((shift_bytes - 8) * 8);
                        self.regs.xmm[xmm][0] = 0;
                    } else if shift_bytes == 0 {
                        // No shift, keep values unchanged
                    } else {
                        let shift_bits = shift_bytes * 8;
                        self.regs.xmm[xmm][1] = (hi << shift_bits) | (lo >> (64 - shift_bits));
                        self.regs.xmm[xmm][0] = lo << shift_bits;
                    }
                }
                _ => {
                    return Err(Error::Emulator(format!(
                        "unimplemented 0x0F 0x73 /r{} at RIP={:#x}",
                        reg, self.regs.rip
                    )));
                }
            }
        } else {
            // MMX version (only PSRLQ/PSLLQ)
            let mm = (rm & 0x7) as usize;
            let shift = imm8 as u32;
            match reg {
                2 => {
                    if shift >= 64 {
                        self.regs.mm[mm] = 0;
                    } else {
                        self.regs.mm[mm] >>= shift;
                    }
                }
                6 => {
                    if shift >= 64 {
                        self.regs.mm[mm] = 0;
                    } else {
                        self.regs.mm[mm] <<= shift;
                    }
                }
                _ => {
                    return Err(Error::Emulator(format!(
                        "unimplemented MMX 0x0F 0x73 /r{} at RIP={:#x}",
                        reg, self.regs.rip
                    )));
                }
            }
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// ADDSUBPS/ADDSUBPD (0x0F 0xD0) - SSE3
    pub(in crate::backend::emulator::x86_64) fn execute_addsubps(
        &mut self,
        ctx: &mut InsnContext,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;

        if ctx.operand_size_override {
            // ADDSUBPD (66 0F D0) - alternating sub/add on doubles
            // dst[63:0] = dst[63:0] - src[63:0]
            // dst[127:64] = dst[127:64] + src[127:64]
            let (src_lo, src_hi) = if is_memory {
                (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
            } else {
                (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
            };
            let dst_lo = self.regs.xmm[xmm_dst][0];
            let dst_hi = self.regs.xmm[xmm_dst][1];

            let r0 = f64::from_bits(dst_lo) - f64::from_bits(src_lo);
            let r1 = f64::from_bits(dst_hi) + f64::from_bits(src_hi);
            self.regs.xmm[xmm_dst][0] = r0.to_bits();
            self.regs.xmm[xmm_dst][1] = r1.to_bits();
        } else if ctx.rep_prefix == Some(0xF2) {
            // ADDSUBPS (F2 0F D0) - alternating sub/add on singles
            // dst[31:0] = dst[31:0] - src[31:0]
            // dst[63:32] = dst[63:32] + src[63:32]
            // dst[95:64] = dst[95:64] - src[95:64]
            // dst[127:96] = dst[127:96] + src[127:96]
            let (src_lo, src_hi) = if is_memory {
                (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
            } else {
                (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
            };
            let dst_lo = self.regs.xmm[xmm_dst][0];
            let dst_hi = self.regs.xmm[xmm_dst][1];

            let d0 = f32::from_bits(dst_lo as u32);
            let d1 = f32::from_bits((dst_lo >> 32) as u32);
            let d2 = f32::from_bits(dst_hi as u32);
            let d3 = f32::from_bits((dst_hi >> 32) as u32);
            let s0 = f32::from_bits(src_lo as u32);
            let s1 = f32::from_bits((src_lo >> 32) as u32);
            let s2 = f32::from_bits(src_hi as u32);
            let s3 = f32::from_bits((src_hi >> 32) as u32);

            let r0 = (d0 - s0).to_bits();
            let r1 = (d1 + s1).to_bits();
            let r2 = (d2 - s2).to_bits();
            let r3 = (d3 + s3).to_bits();

            self.regs.xmm[xmm_dst][0] = (r0 as u64) | ((r1 as u64) << 32);
            self.regs.xmm[xmm_dst][1] = (r2 as u64) | ((r3 as u64) << 32);
        } else {
            return Err(Error::Emulator(format!(
                "unimplemented 0x0F 0xD0 opcode variant at RIP={:#x}",
                self.regs.rip
            )));
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// Execute Group 9 instructions (0F C7)
    /// /1 = CMPXCHG8B/16B, /6 = RDRAND, /7 = RDSEED (or RDPID with F3 prefix)
    pub(in crate::backend::emulator::x86_64) fn execute_group9(
        &mut self,
        ctx: &mut InsnContext,
    ) -> Result<Option<VcpuExit>> {
        let modrm = ctx.peek_u8()?;
        let reg = (modrm >> 3) & 0x7;

        match reg {
            6 => {
                // RDRAND - Read random number
                self.execute_rdrand(ctx)
            }
            7 => {
                // F3 0F C7 /7 = RDPID, 0F C7 /7 = RDSEED
                if ctx.rep_prefix == Some(0xF3) {
                    self.execute_rdpid(ctx)
                } else {
                    self.execute_rdseed(ctx)
                }
            }
            1 => {
                // CMPXCHG8B/CMPXCHG16B - Compare and exchange
                self.execute_cmpxchg8b_16b(ctx)
            }
            _ => Err(Error::Emulator(format!(
                "unimplemented 0x0F 0xC7 /{} at RIP={:#x}",
                reg, self.regs.rip
            ))),
        }
    }

    /// RDRAND - Read random number
    fn execute_rdrand(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let (_, rm, is_memory, _, _) = self.decode_modrm(ctx)?;

        if is_memory {
            return Err(Error::Emulator(
                "RDRAND requires register operand".to_string(),
            ));
        }

        // Determine operand size
        let size = if ctx.rex_w() {
            8
        } else if ctx.operand_size_override {
            2
        } else {
            4
        };

        // Generate pseudo-random number for emulation
        // Using RDTSC-like approach with system time
        let random_value: u64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0x12345678DEADBEEF)
            .wrapping_mul(0x5851F42D4C957F2D)
            .wrapping_add(self.regs.rip);

        // Write to destination register
        match size {
            2 => {
                let val = (random_value & 0xFFFF) as u16;
                self.set_reg(rm, val as u64, 2);
            }
            4 => {
                let val = (random_value & 0xFFFFFFFF) as u32;
                self.set_reg(rm, val as u64, 4);
            }
            8 => {
                self.set_reg(rm, random_value, 8);
            }
            _ => unreachable!(),
        }

        // Set CF=1 (success), clear OF, SF, ZF, AF, PF
        // Clear lazy flags BEFORE setting flags directly
        self.clear_lazy_flags();
        self.regs.rflags &= !(flags::bits::CF
            | flags::bits::OF
            | flags::bits::SF
            | flags::bits::ZF
            | flags::bits::AF
            | flags::bits::PF);
        self.regs.rflags |= flags::bits::CF;

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// RDSEED - Read random seed
    fn execute_rdseed(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let (_, rm, is_memory, _, _) = self.decode_modrm(ctx)?;

        if is_memory {
            return Err(Error::Emulator(
                "RDSEED requires register operand".to_string(),
            ));
        }

        // Determine operand size
        let size = if ctx.rex_w() {
            8
        } else if ctx.operand_size_override {
            2
        } else {
            4
        };

        // Generate pseudo-random seed for emulation
        // Using RDTSC-like approach with system time
        let random_value: u64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0xDEADBEEF12345678)
            .wrapping_mul(0x2545F4914F6CDD1D)
            .wrapping_add(self.regs.rip);

        // Write to destination register
        match size {
            2 => {
                let val = (random_value & 0xFFFF) as u16;
                self.set_reg(rm, val as u64, 2);
            }
            4 => {
                let val = (random_value & 0xFFFFFFFF) as u32;
                self.set_reg(rm, val as u64, 4);
            }
            8 => {
                self.set_reg(rm, random_value, 8);
            }
            _ => unreachable!(),
        }

        // Set CF=1 (success), clear OF, SF, ZF, AF, PF
        // Clear lazy flags BEFORE setting flags directly
        self.clear_lazy_flags();
        self.regs.rflags &= !(flags::bits::CF
            | flags::bits::OF
            | flags::bits::SF
            | flags::bits::ZF
            | flags::bits::AF
            | flags::bits::PF);
        self.regs.rflags |= flags::bits::CF;

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// RDPID - Read Processor ID (F3 0F C7 /7)
    /// Reads IA32_TSC_AUX MSR into destination register.
    /// Does not modify flags.
    fn execute_rdpid(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let (_, rm, is_memory, _, _) = self.decode_modrm(ctx)?;

        if is_memory {
            return Err(Error::Emulator(
                "RDPID requires register operand".to_string(),
            ));
        }

        // RDPID reads IA32_TSC_AUX MSR which contains the processor ID
        // In an emulator, we return a constant processor ID (0)
        // This is the same value that RDTSCP stores in ECX
        let tsc_aux: u64 = 0; // Processor ID = 0

        // RDPID always uses 32-bit operand size (writes to r32, zeros upper 32 bits)
        // unless REX.W is present, then it uses 64-bit
        let size = if ctx.rex_w() { 8 } else { 4 };
        self.set_reg(rm, tsc_aux, size);

        // RDPID does NOT modify flags
        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// CMPXCHG8B/CMPXCHG16B - Compare and exchange bytes
    fn execute_cmpxchg8b_16b(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let (_, _, is_memory, addr, _) = self.decode_modrm(ctx)?;

        if !is_memory {
            return Err(Error::Emulator(
                "CMPXCHG8B/16B requires memory operand".to_string(),
            ));
        }

        // Clear lazy flags before setting ZF directly
        self.clear_lazy_flags();

        if ctx.rex_w() {
            // CMPXCHG16B - Compare EDX:EAX with m128, if equal set m128 to ECX:EBX
            let mem_lo = self.read_mem(addr, 8)?;
            let mem_hi = self.read_mem(addr + 8, 8)?;
            let cmp_lo = self.regs.rax;
            let cmp_hi = self.regs.rdx;

            if mem_lo == cmp_lo && mem_hi == cmp_hi {
                // Equal - set ZF, store RCX:RBX to memory
                self.write_mem(addr, self.regs.rbx, 8)?;
                self.write_mem(addr + 8, self.regs.rcx, 8)?;
                self.regs.rflags |= flags::bits::ZF;
            } else {
                // Not equal - clear ZF, load memory to RDX:RAX
                self.regs.rax = mem_lo;
                self.regs.rdx = mem_hi;
                self.regs.rflags &= !flags::bits::ZF;
            }
        } else {
            // CMPXCHG8B - Compare EDX:EAX with m64, if equal set m64 to ECX:EBX
            let mem_val = self.read_mem(addr, 8)?;
            let cmp_val = ((self.regs.rdx & 0xFFFFFFFF) << 32) | (self.regs.rax & 0xFFFFFFFF);

            if mem_val == cmp_val {
                // Equal - set ZF, store ECX:EBX to memory
                let store_val = ((self.regs.rcx & 0xFFFFFFFF) << 32) | (self.regs.rbx & 0xFFFFFFFF);
                self.write_mem(addr, store_val, 8)?;
                self.regs.rflags |= flags::bits::ZF;
            } else {
                // Not equal - clear ZF, load memory to EDX:EAX
                self.regs.rax = mem_val & 0xFFFFFFFF;
                self.regs.rdx = mem_val >> 32;
                self.regs.rflags &= !flags::bits::ZF;
            }
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }
}

// Shift helper functions
fn shift_right_words(v: u64, shift: u32) -> u64 {
    let mut result = 0u64;
    for i in 0..4 {
        let w = ((v >> (i * 16)) & 0xFFFF) as u16;
        let shifted = w >> shift;
        result |= (shifted as u64) << (i * 16);
    }
    result
}

fn shift_left_words(v: u64, shift: u32) -> u64 {
    let mut result = 0u64;
    for i in 0..4 {
        let w = ((v >> (i * 16)) & 0xFFFF) as u16;
        let shifted = w << shift;
        result |= (shifted as u64) << (i * 16);
    }
    result
}

fn shift_right_arith_words(v: u64, shift: u32) -> u64 {
    let mut result = 0u64;
    let shift = shift.min(15);
    for i in 0..4 {
        let w = ((v >> (i * 16)) & 0xFFFF) as i16;
        let shifted = (w >> shift) as u16;
        result |= (shifted as u64) << (i * 16);
    }
    result
}

fn shift_right_dwords(v: u64, shift: u32) -> u64 {
    let d0 = (v as u32) >> shift;
    let d1 = ((v >> 32) as u32) >> shift;
    (d0 as u64) | ((d1 as u64) << 32)
}

fn shift_left_dwords(v: u64, shift: u32) -> u64 {
    let d0 = (v as u32) << shift;
    let d1 = ((v >> 32) as u32) << shift;
    (d0 as u64) | ((d1 as u64) << 32)
}

fn shift_right_arith_dwords(v: u64, shift: u32) -> u64 {
    let shift = shift.min(31);
    let d0 = ((v as u32) as i32 >> shift) as u32;
    let d1 = (((v >> 32) as u32) as i32 >> shift) as u32;
    (d0 as u64) | ((d1 as u64) << 32)
}

fn unpack_low_bytes(a: u64, b: u64) -> u64 {
    let a0 = (a >> 0) & 0xFF;
    let b0 = (b >> 0) & 0xFF;
    let a1 = (a >> 8) & 0xFF;
    let b1 = (b >> 8) & 0xFF;
    let a2 = (a >> 16) & 0xFF;
    let b2 = (b >> 16) & 0xFF;
    let a3 = (a >> 24) & 0xFF;
    let b3 = (b >> 24) & 0xFF;
    a0 | (b0 << 8) | (a1 << 16) | (b1 << 24) | (a2 << 32) | (b2 << 40) | (a3 << 48) | (b3 << 56)
}

fn unpack_high_bytes(a: u64, b: u64) -> u64 {
    let a4 = (a >> 32) & 0xFF;
    let b4 = (b >> 32) & 0xFF;
    let a5 = (a >> 40) & 0xFF;
    let b5 = (b >> 40) & 0xFF;
    let a6 = (a >> 48) & 0xFF;
    let b6 = (b >> 48) & 0xFF;
    let a7 = (a >> 56) & 0xFF;
    let b7 = (b >> 56) & 0xFF;
    a4 | (b4 << 8) | (a5 << 16) | (b5 << 24) | (a6 << 32) | (b6 << 40) | (a7 << 48) | (b7 << 56)
}

fn unpack_low_words(a: u64, b: u64) -> u64 {
    let a0 = a & 0xFFFF;
    let b0 = b & 0xFFFF;
    let a1 = (a >> 16) & 0xFFFF;
    let b1 = (b >> 16) & 0xFFFF;
    a0 | (b0 << 16) | (a1 << 32) | (b1 << 48)
}

fn unpack_high_words(a: u64, b: u64) -> u64 {
    let a2 = (a >> 32) & 0xFFFF;
    let b2 = (b >> 32) & 0xFFFF;
    let a3 = (a >> 48) & 0xFFFF;
    let b3 = (b >> 48) & 0xFFFF;
    a2 | (b2 << 16) | (a3 << 32) | (b3 << 48)
}

fn rcp_packed_f32(v: u64) -> u64 {
    let f0 = f32::from_bits(v as u32);
    let f1 = f32::from_bits((v >> 32) as u32);
    let r0 = (1.0f32 / f0).to_bits() as u64;
    let r1 = (1.0f32 / f1).to_bits() as u64;
    r0 | (r1 << 32)
}

fn rsqrt_packed_f32(v: u64) -> u64 {
    let f0 = f32::from_bits(v as u32);
    let f1 = f32::from_bits((v >> 32) as u32);
    let r0 = (1.0f32 / f0.sqrt()).to_bits() as u64;
    let r1 = (1.0f32 / f1.sqrt()).to_bits() as u64;
    r0 | (r1 << 32)
}
