//! SSE/SSE2 conversion instructions: CVT*.

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::super::cpu::{InsnContext, X86_64Vcpu};

// =============================================================================
// Packed Float ↔ Float Conversions
// =============================================================================

/// CVTPS2PD xmm1, xmm2/m64 (NP 0F 5A /r)
/// Convert two packed single-precision FP values to packed double-precision FP values
pub fn cvtps2pd(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;

    // Read 64 bits (two f32 values) from source
    let src_low = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        let xmm_src = rm as usize;
        vcpu.regs.xmm[xmm_src][0]
    };

    // Extract the two f32 values
    let f0 = f32::from_bits((src_low & 0xFFFFFFFF) as u32);
    let f1 = f32::from_bits(((src_low >> 32) & 0xFFFFFFFF) as u32);

    // Convert to f64
    let d0 = f0 as f64;
    let d1 = f1 as f64;

    // Store as two f64 values (128 bits total)
    vcpu.regs.xmm[xmm_dst][0] = d0.to_bits();
    vcpu.regs.xmm[xmm_dst][1] = d1.to_bits();

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// CVTPD2PS xmm1, xmm2/m128 (66 0F 5A /r)
/// Convert two packed double-precision FP values to packed single-precision FP values
pub fn cvtpd2ps(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;

    // Read 128 bits (two f64 values) from source
    let (src_low, src_high) = if is_memory {
        (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
    } else {
        let xmm_src = rm as usize;
        (vcpu.regs.xmm[xmm_src][0], vcpu.regs.xmm[xmm_src][1])
    };

    // Convert the two f64 values to f32
    let d0 = f64::from_bits(src_low);
    let d1 = f64::from_bits(src_high);
    let f0 = d0 as f32;
    let f1 = d1 as f32;

    // Store two f32 values in low 64 bits, zero high 64 bits
    vcpu.regs.xmm[xmm_dst][0] = (f0.to_bits() as u64) | ((f1.to_bits() as u64) << 32);
    vcpu.regs.xmm[xmm_dst][1] = 0;

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// Scalar Float ↔ Float Conversions
// =============================================================================

/// CVTSS2SD xmm1, xmm2/m32 (F3 0F 5A /r)
/// Convert scalar single-precision FP value to scalar double-precision FP value
pub fn cvtss2sd(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;

    // Read 32-bit float from source
    let src = if is_memory {
        vcpu.read_mem(addr, 4)? as u32
    } else {
        let xmm_src = rm as usize;
        vcpu.regs.xmm[xmm_src][0] as u32
    };

    let f = f32::from_bits(src);
    let d = f as f64;

    // Store in low 64 bits, preserve high 64 bits of destination
    vcpu.regs.xmm[xmm_dst][0] = d.to_bits();
    // High part preserved (not modified)

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// CVTSD2SS xmm1, xmm2/m64 (F2 0F 5A /r)
/// Convert scalar double-precision FP value to scalar single-precision FP value
pub fn cvtsd2ss(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;

    // Read 64-bit double from source
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        let xmm_src = rm as usize;
        vcpu.regs.xmm[xmm_src][0]
    };

    let d = f64::from_bits(src);
    let f = d as f32;

    // Store in low 32 bits of low qword, preserve bits 32-127 of destination
    let preserved = vcpu.regs.xmm[xmm_dst][0] & 0xFFFFFFFF00000000;
    vcpu.regs.xmm[xmm_dst][0] = preserved | (f.to_bits() as u64);
    // High part preserved (not modified)

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// Packed Integer ↔ Float Conversions
// =============================================================================

/// CVTDQ2PS xmm1, xmm2/m128 (NP 0F 5B /r)
/// Convert four packed signed doubleword integers to packed single-precision FP values
pub fn cvtdq2ps(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;

    // Read 128 bits (four i32 values) from source
    let (src_low, src_high) = if is_memory {
        (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
    } else {
        let xmm_src = rm as usize;
        (vcpu.regs.xmm[xmm_src][0], vcpu.regs.xmm[xmm_src][1])
    };

    // Extract four i32 values
    let i0 = src_low as i32;
    let i1 = (src_low >> 32) as i32;
    let i2 = src_high as i32;
    let i3 = (src_high >> 32) as i32;

    // Convert to f32
    let f0 = i0 as f32;
    let f1 = i1 as f32;
    let f2 = i2 as f32;
    let f3 = i3 as f32;

    // Pack into destination
    vcpu.regs.xmm[xmm_dst][0] = (f0.to_bits() as u64) | ((f1.to_bits() as u64) << 32);
    vcpu.regs.xmm[xmm_dst][1] = (f2.to_bits() as u64) | ((f3.to_bits() as u64) << 32);

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// CVTPS2DQ xmm1, xmm2/m128 (66 0F 5B /r)
/// Convert four packed single-precision FP values to packed signed doubleword integers
pub fn cvtps2dq(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;

    // Read 128 bits (four f32 values) from source
    let (src_low, src_high) = if is_memory {
        (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
    } else {
        let xmm_src = rm as usize;
        (vcpu.regs.xmm[xmm_src][0], vcpu.regs.xmm[xmm_src][1])
    };

    // Extract four f32 values
    let f0 = f32::from_bits((src_low & 0xFFFFFFFF) as u32);
    let f1 = f32::from_bits(((src_low >> 32) & 0xFFFFFFFF) as u32);
    let f2 = f32::from_bits((src_high & 0xFFFFFFFF) as u32);
    let f3 = f32::from_bits(((src_high >> 32) & 0xFFFFFFFF) as u32);

    // Convert to i32 with rounding (uses current MXCSR rounding mode, default: nearest even)
    let i0 = float_to_int32_round(f0);
    let i1 = float_to_int32_round(f1);
    let i2 = float_to_int32_round(f2);
    let i3 = float_to_int32_round(f3);

    // Pack into destination
    vcpu.regs.xmm[xmm_dst][0] = (i0 as u32 as u64) | ((i1 as u32 as u64) << 32);
    vcpu.regs.xmm[xmm_dst][1] = (i2 as u32 as u64) | ((i3 as u32 as u64) << 32);

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// CVTTPS2DQ xmm1, xmm2/m128 (F3 0F 5B /r)
/// Convert four packed single-precision FP values to packed signed doubleword integers (truncate)
pub fn cvttps2dq(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;

    // Read 128 bits (four f32 values) from source
    let (src_low, src_high) = if is_memory {
        (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
    } else {
        let xmm_src = rm as usize;
        (vcpu.regs.xmm[xmm_src][0], vcpu.regs.xmm[xmm_src][1])
    };

    // Extract four f32 values
    let f0 = f32::from_bits((src_low & 0xFFFFFFFF) as u32);
    let f1 = f32::from_bits(((src_low >> 32) & 0xFFFFFFFF) as u32);
    let f2 = f32::from_bits((src_high & 0xFFFFFFFF) as u32);
    let f3 = f32::from_bits(((src_high >> 32) & 0xFFFFFFFF) as u32);

    // Convert to i32 with truncation
    let i0 = float_to_int32_trunc(f0);
    let i1 = float_to_int32_trunc(f1);
    let i2 = float_to_int32_trunc(f2);
    let i3 = float_to_int32_trunc(f3);

    // Pack into destination
    vcpu.regs.xmm[xmm_dst][0] = (i0 as u32 as u64) | ((i1 as u32 as u64) << 32);
    vcpu.regs.xmm[xmm_dst][1] = (i2 as u32 as u64) | ((i3 as u32 as u64) << 32);

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// CVTDQ2PD xmm1, xmm2/m64 (F3 0F E6 /r)
/// Convert two packed signed doubleword integers to packed double-precision FP values
pub fn cvtdq2pd(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;

    // Read 64 bits (two i32 values) from source
    let src_low = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        let xmm_src = rm as usize;
        vcpu.regs.xmm[xmm_src][0]
    };

    // Extract two i32 values from low 64 bits
    let i0 = src_low as i32;
    let i1 = (src_low >> 32) as i32;

    // Convert to f64
    let d0 = i0 as f64;
    let d1 = i1 as f64;

    // Store as two f64 values (128 bits total)
    vcpu.regs.xmm[xmm_dst][0] = d0.to_bits();
    vcpu.regs.xmm[xmm_dst][1] = d1.to_bits();

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// CVTPD2DQ xmm1, xmm2/m128 (F2 0F E6 /r)
/// Convert two packed double-precision FP values to packed signed doubleword integers
pub fn cvtpd2dq(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;

    // Read 128 bits (two f64 values) from source
    let (src_low, src_high) = if is_memory {
        (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
    } else {
        let xmm_src = rm as usize;
        (vcpu.regs.xmm[xmm_src][0], vcpu.regs.xmm[xmm_src][1])
    };

    // Extract two f64 values
    let d0 = f64::from_bits(src_low);
    let d1 = f64::from_bits(src_high);

    // Convert to i32 with rounding
    let i0 = double_to_int32_round(d0);
    let i1 = double_to_int32_round(d1);

    // Pack into low 64 bits, zero high 64 bits
    vcpu.regs.xmm[xmm_dst][0] = (i0 as u32 as u64) | ((i1 as u32 as u64) << 32);
    vcpu.regs.xmm[xmm_dst][1] = 0;

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// CVTTPD2DQ xmm1, xmm2/m128 (66 0F E6 /r)
/// Convert two packed double-precision FP values to packed signed doubleword integers (truncate)
pub fn cvttpd2dq(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;

    // Read 128 bits (two f64 values) from source
    let (src_low, src_high) = if is_memory {
        (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
    } else {
        let xmm_src = rm as usize;
        (vcpu.regs.xmm[xmm_src][0], vcpu.regs.xmm[xmm_src][1])
    };

    // Extract two f64 values
    let d0 = f64::from_bits(src_low);
    let d1 = f64::from_bits(src_high);

    // Convert to i32 with truncation
    let i0 = double_to_int32_trunc(d0);
    let i1 = double_to_int32_trunc(d1);

    // Pack into low 64 bits, zero high 64 bits
    vcpu.regs.xmm[xmm_dst][0] = (i0 as u32 as u64) | ((i1 as u32 as u64) << 32);
    vcpu.regs.xmm[xmm_dst][1] = 0;

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// Scalar Integer ↔ Float Conversions
// =============================================================================

/// CVTSI2SS xmm1, r/m32 or r/m64 (F3 0F 2A /r)
/// Convert signed integer to scalar single-precision FP value
pub fn cvtsi2ss(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;

    // Check REX.W for 64-bit vs 32-bit source
    let is_64bit = ctx.rex.map_or(false, |r| r & 0x08 != 0);

    let value = if is_memory {
        if is_64bit {
            vcpu.read_mem(addr, 8)? as i64
        } else {
            vcpu.read_mem(addr, 4)? as i32 as i64
        }
    } else {
        if is_64bit {
            vcpu.get_reg(rm, 8) as i64
        } else {
            vcpu.get_reg(rm, 4) as i32 as i64
        }
    };

    let f = value as f32;

    // Store in low 32 bits of low qword, preserve bits 32-127 of destination
    let preserved = vcpu.regs.xmm[xmm_dst][0] & 0xFFFFFFFF00000000;
    vcpu.regs.xmm[xmm_dst][0] = preserved | (f.to_bits() as u64);
    // High part preserved (not modified)

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// CVTSI2SD xmm1, r/m32 or r/m64 (F2 0F 2A /r)
/// Convert signed integer to scalar double-precision FP value
pub fn cvtsi2sd(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;

    // Check REX.W for 64-bit vs 32-bit source
    let is_64bit = ctx.rex.map_or(false, |r| r & 0x08 != 0);

    let value = if is_memory {
        if is_64bit {
            vcpu.read_mem(addr, 8)? as i64
        } else {
            vcpu.read_mem(addr, 4)? as i32 as i64
        }
    } else {
        if is_64bit {
            vcpu.get_reg(rm, 8) as i64
        } else {
            vcpu.get_reg(rm, 4) as i32 as i64
        }
    };

    let d = value as f64;

    // Store in low 64 bits, preserve high 64 bits of destination
    vcpu.regs.xmm[xmm_dst][0] = d.to_bits();
    // High part preserved (not modified)

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// CVTSS2SI r32/r64, xmm1/m32 (F3 0F 2D /r)
/// Convert scalar single-precision FP value to signed integer
pub fn cvtss2si(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;

    // Check REX.W for 64-bit vs 32-bit destination
    let is_64bit = ctx.rex.map_or(false, |r| r & 0x08 != 0);

    // Read 32-bit float from source
    let src = if is_memory {
        vcpu.read_mem(addr, 4)? as u32
    } else {
        let xmm_src = rm as usize;
        vcpu.regs.xmm[xmm_src][0] as u32
    };

    let f = f32::from_bits(src);

    // Convert with rounding
    let result = if is_64bit {
        float_to_int64_round(f) as u64
    } else {
        float_to_int32_round(f) as u32 as u64
    };

    vcpu.set_reg(reg, result, if is_64bit { 8 } else { 4 });
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// CVTSD2SI r32/r64, xmm1/m64 (F2 0F 2D /r)
/// Convert scalar double-precision FP value to signed integer
pub fn cvtsd2si(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;

    // Check REX.W for 64-bit vs 32-bit destination
    let is_64bit = ctx.rex.map_or(false, |r| r & 0x08 != 0);

    // Read 64-bit double from source
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        let xmm_src = rm as usize;
        vcpu.regs.xmm[xmm_src][0]
    };

    let d = f64::from_bits(src);

    // Convert with rounding
    let result = if is_64bit {
        double_to_int64_round(d) as u64
    } else {
        double_to_int32_round(d) as u32 as u64
    };

    vcpu.set_reg(reg, result, if is_64bit { 8 } else { 4 });
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// CVTTSS2SI r32/r64, xmm1/m32 (F3 0F 2C /r)
/// Convert scalar single-precision FP value to signed integer (truncate)
pub fn cvttss2si(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;

    // Check REX.W for 64-bit vs 32-bit destination
    let is_64bit = ctx.rex.map_or(false, |r| r & 0x08 != 0);

    // Read 32-bit float from source
    let src = if is_memory {
        vcpu.read_mem(addr, 4)? as u32
    } else {
        let xmm_src = rm as usize;
        vcpu.regs.xmm[xmm_src][0] as u32
    };

    let f = f32::from_bits(src);

    // Convert with truncation
    let result = if is_64bit {
        float_to_int64_trunc(f) as u64
    } else {
        float_to_int32_trunc(f) as u32 as u64
    };

    vcpu.set_reg(reg, result, if is_64bit { 8 } else { 4 });
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// CVTTSD2SI r32/r64, xmm1/m64 (F2 0F 2C /r)
/// Convert scalar double-precision FP value to signed integer (truncate)
pub fn cvttsd2si(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;

    // Check REX.W for 64-bit vs 32-bit destination
    let is_64bit = ctx.rex.map_or(false, |r| r & 0x08 != 0);

    // Read 64-bit double from source
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        let xmm_src = rm as usize;
        vcpu.regs.xmm[xmm_src][0]
    };

    let d = f64::from_bits(src);

    // Convert with truncation
    let result = if is_64bit {
        double_to_int64_trunc(d) as u64
    } else {
        double_to_int32_trunc(d) as u32 as u64
    };

    vcpu.set_reg(reg, result, if is_64bit { 8 } else { 4 });
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// MMX ↔ SSE Conversion Instructions
// =============================================================================

/// CVTPI2PS xmm, mm/m64 (NP 0F 2A /r)
/// Convert two packed signed doubleword integers from MMX to packed single-precision FP
pub fn cvtpi2ps(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;

    // Read 64 bits (two i32 values) from MMX or memory
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        let mm_idx = (rm & 0x07) as usize;
        vcpu.regs.mm[mm_idx]
    };

    // Extract two i32 values
    let i0 = src as i32;
    let i1 = (src >> 32) as i32;

    // Convert to f32
    let f0 = i0 as f32;
    let f1 = i1 as f32;

    // Store in low 64 bits, preserve high 64 bits of destination
    vcpu.regs.xmm[xmm_dst][0] = (f0.to_bits() as u64) | ((f1.to_bits() as u64) << 32);
    // High part preserved

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// CVTPS2PI mm, xmm/m64 (NP 0F 2D /r)
/// Convert two packed single-precision FP values to packed signed doubleword integers in MMX
pub fn cvtps2pi(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let mm_dst = (reg & 0x07) as usize;

    // Read 64 bits (two f32 values) from XMM or memory
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        let xmm_src = rm as usize;
        vcpu.regs.xmm[xmm_src][0]
    };

    // Extract two f32 values
    let f0 = f32::from_bits((src & 0xFFFFFFFF) as u32);
    let f1 = f32::from_bits(((src >> 32) & 0xFFFFFFFF) as u32);

    // Convert to i32 with rounding
    let i0 = float_to_int32_round(f0);
    let i1 = float_to_int32_round(f1);

    // Store in MMX register
    vcpu.regs.mm[mm_dst] = (i0 as u32 as u64) | ((i1 as u32 as u64) << 32);

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// CVTTPS2PI mm, xmm/m64 (NP 0F 2C /r)
/// Convert two packed single-precision FP values to packed signed doubleword integers (truncate)
pub fn cvttps2pi(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let mm_dst = (reg & 0x07) as usize;

    // Read 64 bits (two f32 values) from XMM or memory
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        let xmm_src = rm as usize;
        vcpu.regs.xmm[xmm_src][0]
    };

    // Extract two f32 values
    let f0 = f32::from_bits((src & 0xFFFFFFFF) as u32);
    let f1 = f32::from_bits(((src >> 32) & 0xFFFFFFFF) as u32);

    // Convert to i32 with truncation
    let i0 = float_to_int32_trunc(f0);
    let i1 = float_to_int32_trunc(f1);

    // Store in MMX register
    vcpu.regs.mm[mm_dst] = (i0 as u32 as u64) | ((i1 as u32 as u64) << 32);

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// CVTPI2PD xmm, mm/m64 (66 0F 2A /r)
/// Convert two packed signed doubleword integers from MMX to packed double-precision FP
pub fn cvtpi2pd(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;

    // Read 64 bits (two i32 values) from MMX or memory
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        let mm_idx = (rm & 0x07) as usize;
        vcpu.regs.mm[mm_idx]
    };

    // Extract two i32 values
    let i0 = src as i32;
    let i1 = (src >> 32) as i32;

    // Convert to f64
    let d0 = i0 as f64;
    let d1 = i1 as f64;

    // Store as two f64 values (128 bits total)
    vcpu.regs.xmm[xmm_dst][0] = d0.to_bits();
    vcpu.regs.xmm[xmm_dst][1] = d1.to_bits();

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// CVTPD2PI mm, xmm/m128 (66 0F 2D /r)
/// Convert two packed double-precision FP values to packed signed doubleword integers in MMX
pub fn cvtpd2pi(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let mm_dst = (reg & 0x07) as usize;

    // Read 128 bits (two f64 values) from XMM or memory
    let (src_low, src_high) = if is_memory {
        (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
    } else {
        let xmm_src = rm as usize;
        (vcpu.regs.xmm[xmm_src][0], vcpu.regs.xmm[xmm_src][1])
    };

    // Convert two f64 values to i32 with rounding
    let d0 = f64::from_bits(src_low);
    let d1 = f64::from_bits(src_high);
    let i0 = double_to_int32_round(d0);
    let i1 = double_to_int32_round(d1);

    // Store in MMX register
    vcpu.regs.mm[mm_dst] = (i0 as u32 as u64) | ((i1 as u32 as u64) << 32);

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// CVTTPD2PI mm, xmm/m128 (66 0F 2C /r)
/// Convert two packed double-precision FP values to packed signed doubleword integers (truncate)
pub fn cvttpd2pi(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let mm_dst = (reg & 0x07) as usize;

    // Read 128 bits (two f64 values) from XMM or memory
    let (src_low, src_high) = if is_memory {
        (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
    } else {
        let xmm_src = rm as usize;
        (vcpu.regs.xmm[xmm_src][0], vcpu.regs.xmm[xmm_src][1])
    };

    // Convert two f64 values to i32 with truncation
    let d0 = f64::from_bits(src_low);
    let d1 = f64::from_bits(src_high);
    let i0 = double_to_int32_trunc(d0);
    let i1 = double_to_int32_trunc(d1);

    // Store in MMX register
    vcpu.regs.mm[mm_dst] = (i0 as u32 as u64) | ((i1 as u32 as u64) << 32);

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// Helper functions for float-to-int conversions
// =============================================================================

/// Convert f32 to i32 with rounding to nearest even (default MXCSR mode)
fn float_to_int32_round(f: f32) -> i32 {
    if f.is_nan() || f.is_infinite() || f > i32::MAX as f32 || f < i32::MIN as f32 {
        // Return 0x80000000 for out-of-range/special values (per Intel spec)
        i32::MIN
    } else {
        // Round to nearest even
        f.round_ties_even() as i32
    }
}

/// Convert f32 to i32 with truncation
fn float_to_int32_trunc(f: f32) -> i32 {
    if f.is_nan() || f.is_infinite() || f > i32::MAX as f32 || f < i32::MIN as f32 {
        i32::MIN
    } else {
        f.trunc() as i32
    }
}

/// Convert f32 to i64 with rounding to nearest even
fn float_to_int64_round(f: f32) -> i64 {
    if f.is_nan() || f.is_infinite() || f > i64::MAX as f32 || f < i64::MIN as f32 {
        i64::MIN
    } else {
        f.round_ties_even() as i64
    }
}

/// Convert f32 to i64 with truncation
fn float_to_int64_trunc(f: f32) -> i64 {
    if f.is_nan() || f.is_infinite() || f > i64::MAX as f32 || f < i64::MIN as f32 {
        i64::MIN
    } else {
        f.trunc() as i64
    }
}

/// Convert f64 to i32 with rounding to nearest even
fn double_to_int32_round(d: f64) -> i32 {
    if d.is_nan() || d.is_infinite() || d > i32::MAX as f64 || d < i32::MIN as f64 {
        i32::MIN
    } else {
        d.round_ties_even() as i32
    }
}

/// Convert f64 to i32 with truncation
fn double_to_int32_trunc(d: f64) -> i32 {
    if d.is_nan() || d.is_infinite() || d > i32::MAX as f64 || d < i32::MIN as f64 {
        i32::MIN
    } else {
        d.trunc() as i32
    }
}

/// Convert f64 to i64 with rounding to nearest even
fn double_to_int64_round(d: f64) -> i64 {
    if d.is_nan() || d.is_infinite() || d > i64::MAX as f64 || d < i64::MIN as f64 {
        i64::MIN
    } else {
        d.round_ties_even() as i64
    }
}

/// Convert f64 to i64 with truncation
fn double_to_int64_trunc(d: f64) -> i64 {
    if d.is_nan() || d.is_infinite() || d > i64::MAX as f64 || d < i64::MIN as f64 {
        i64::MIN
    } else {
        d.trunc() as i64
    }
}
