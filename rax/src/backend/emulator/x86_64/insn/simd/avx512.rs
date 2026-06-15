//! AVX-512 SIMD instruction implementations (EVEX-encoded).

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::cpu::{InsnContext, X86_64Vcpu};

/// Helper to get ZMM register value (all 8 qwords)
fn get_zmm(vcpu: &X86_64Vcpu, reg: u8) -> [u64; 8] {
    if reg < 16 {
        // ZMM0-15: composed of xmm[0..2], ymm_high[0..2], zmm_high[0..4]
        let r = reg as usize;
        [
            vcpu.regs.xmm[r][0],
            vcpu.regs.xmm[r][1],
            vcpu.regs.ymm_high[r][0],
            vcpu.regs.ymm_high[r][1],
            vcpu.regs.zmm_high[r][0],
            vcpu.regs.zmm_high[r][1],
            vcpu.regs.zmm_high[r][2],
            vcpu.regs.zmm_high[r][3],
        ]
    } else {
        // ZMM16-31: stored in zmm_ext
        let r = (reg - 16) as usize;
        vcpu.regs.zmm_ext[r]
    }
}

/// Helper to set ZMM register value (all 8 qwords)
fn set_zmm(vcpu: &mut X86_64Vcpu, reg: u8, val: [u64; 8]) {
    if reg < 16 {
        let r = reg as usize;
        vcpu.regs.xmm[r][0] = val[0];
        vcpu.regs.xmm[r][1] = val[1];
        vcpu.regs.ymm_high[r][0] = val[2];
        vcpu.regs.ymm_high[r][1] = val[3];
        vcpu.regs.zmm_high[r][0] = val[4];
        vcpu.regs.zmm_high[r][1] = val[5];
        vcpu.regs.zmm_high[r][2] = val[6];
        vcpu.regs.zmm_high[r][3] = val[7];
    } else {
        let r = (reg - 16) as usize;
        vcpu.regs.zmm_ext[r] = val;
    }
}

/// Helper to get XMM register value (2 qwords)
fn get_xmm(vcpu: &X86_64Vcpu, reg: u8) -> [u64; 2] {
    if reg < 16 {
        vcpu.regs.xmm[reg as usize]
    } else {
        let r = (reg - 16) as usize;
        [vcpu.regs.zmm_ext[r][0], vcpu.regs.zmm_ext[r][1]]
    }
}

/// Helper to set XMM register value (2 qwords), zeroing upper bits
fn set_xmm_zero_upper(vcpu: &mut X86_64Vcpu, reg: u8, val: [u64; 2]) {
    if reg < 16 {
        let r = reg as usize;
        vcpu.regs.xmm[r] = val;
        vcpu.regs.ymm_high[r] = [0, 0];
        vcpu.regs.zmm_high[r] = [0, 0, 0, 0];
    } else {
        let r = (reg - 16) as usize;
        vcpu.regs.zmm_ext[r] = [val[0], val[1], 0, 0, 0, 0, 0, 0];
    }
}

/// Helper to get YMM register value (4 qwords)
fn get_ymm(vcpu: &X86_64Vcpu, reg: u8) -> [u64; 4] {
    if reg < 16 {
        let r = reg as usize;
        [
            vcpu.regs.xmm[r][0],
            vcpu.regs.xmm[r][1],
            vcpu.regs.ymm_high[r][0],
            vcpu.regs.ymm_high[r][1],
        ]
    } else {
        let r = (reg - 16) as usize;
        [
            vcpu.regs.zmm_ext[r][0],
            vcpu.regs.zmm_ext[r][1],
            vcpu.regs.zmm_ext[r][2],
            vcpu.regs.zmm_ext[r][3],
        ]
    }
}

/// Helper to set YMM register value (4 qwords), zeroing upper bits
fn set_ymm_zero_upper(vcpu: &mut X86_64Vcpu, reg: u8, val: [u64; 4]) {
    if reg < 16 {
        let r = reg as usize;
        vcpu.regs.xmm[r][0] = val[0];
        vcpu.regs.xmm[r][1] = val[1];
        vcpu.regs.ymm_high[r][0] = val[2];
        vcpu.regs.ymm_high[r][1] = val[3];
        vcpu.regs.zmm_high[r] = [0, 0, 0, 0];
    } else {
        let r = (reg - 16) as usize;
        vcpu.regs.zmm_ext[r] = [val[0], val[1], val[2], val[3], 0, 0, 0, 0];
    }
}

fn evex_vl_bytes(ll: u8) -> usize {
    match ll {
        0 => 16,
        1 => 32,
        2 => 64,
        _ => 16,
    }
}

fn evex_mask(vcpu: &X86_64Vcpu, aaa: u8, num_elems: usize) -> u64 {
    let full_mask = if num_elems == 64 {
        u64::MAX
    } else {
        (1u64 << num_elems) - 1
    };
    if aaa == 0 {
        full_mask
    } else {
        vcpu.regs.k[aaa as usize] & full_mask
    }
}

fn read_reg_bytes(vcpu: &X86_64Vcpu, reg: u8, vl_bytes: usize) -> [u8; 64] {
    let mut data = [0u8; 64];
    match vl_bytes {
        16 => {
            let vals = get_xmm(vcpu, reg);
            for i in 0..2 {
                let start = i * 8;
                data[start..start + 8].copy_from_slice(&vals[i].to_le_bytes());
            }
        }
        32 => {
            let vals = get_ymm(vcpu, reg);
            for i in 0..4 {
                let start = i * 8;
                data[start..start + 8].copy_from_slice(&vals[i].to_le_bytes());
            }
        }
        64 => {
            let vals = get_zmm(vcpu, reg);
            for i in 0..8 {
                let start = i * 8;
                data[start..start + 8].copy_from_slice(&vals[i].to_le_bytes());
            }
        }
        _ => {}
    }
    data
}

fn write_reg_bytes(vcpu: &mut X86_64Vcpu, reg: u8, vl_bytes: usize, data: &[u8; 64]) {
    match vl_bytes {
        16 => {
            let mut vals = [0u64; 2];
            for i in 0..2 {
                let start = i * 8;
                vals[i] = u64::from_le_bytes([
                    data[start],
                    data[start + 1],
                    data[start + 2],
                    data[start + 3],
                    data[start + 4],
                    data[start + 5],
                    data[start + 6],
                    data[start + 7],
                ]);
            }
            set_xmm_zero_upper(vcpu, reg, vals);
        }
        32 => {
            let mut vals = [0u64; 4];
            for i in 0..4 {
                let start = i * 8;
                vals[i] = u64::from_le_bytes([
                    data[start],
                    data[start + 1],
                    data[start + 2],
                    data[start + 3],
                    data[start + 4],
                    data[start + 5],
                    data[start + 6],
                    data[start + 7],
                ]);
            }
            set_ymm_zero_upper(vcpu, reg, vals);
        }
        64 => {
            let mut vals = [0u64; 8];
            for i in 0..8 {
                let start = i * 8;
                vals[i] = u64::from_le_bytes([
                    data[start],
                    data[start + 1],
                    data[start + 2],
                    data[start + 3],
                    data[start + 4],
                    data[start + 5],
                    data[start + 6],
                    data[start + 7],
                ]);
            }
            set_zmm(vcpu, reg, vals);
        }
        _ => {}
    }
}

fn load_mem_bytes(
    vcpu: &mut X86_64Vcpu,
    addr: u64,
    elem_size: usize,
    num_elems: usize,
) -> Result<[u8; 64]> {
    let mut data = [0u8; 64];
    for i in 0..num_elems {
        let value = vcpu.read_mem(addr + (i * elem_size) as u64, elem_size as u8)?;
        let start = i * elem_size;
        if elem_size == 4 {
            let bytes = (value as u32).to_le_bytes();
            data[start..start + elem_size].copy_from_slice(&bytes);
        } else {
            let bytes = value.to_le_bytes();
            data[start..start + elem_size].copy_from_slice(&bytes);
        }
    }
    Ok(data)
}

fn store_mem_bytes(
    vcpu: &mut X86_64Vcpu,
    addr: u64,
    elem_size: usize,
    num_elems: usize,
    data: &[u8; 64],
) -> Result<()> {
    for i in 0..num_elems {
        let start = i * elem_size;
        let value = if elem_size == 4 {
            u32::from_le_bytes([
                data[start],
                data[start + 1],
                data[start + 2],
                data[start + 3],
            ]) as u64
        } else {
            u64::from_le_bytes([
                data[start],
                data[start + 1],
                data[start + 2],
                data[start + 3],
                data[start + 4],
                data[start + 5],
                data[start + 6],
                data[start + 7],
            ])
        };
        vcpu.write_mem(addr + (i * elem_size) as u64, value, elem_size as u8)?;
    }
    Ok(())
}

/// VPMULLQ - Multiply Packed Signed Quadword Integers and Store Low Result
/// EVEX.128/256/512.66.0F38.W1 40 /r
///
/// Multiplies packed signed qword integers and stores the low 64 bits
/// of each 128-bit result.
pub fn vpmullq(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let evex = ctx
        .evex
        .ok_or_else(|| Error::Emulator("VPMULLQ requires EVEX prefix".to_string()))?;

    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;

    // Calculate full register indices using EVEX extension bits
    let dest = (reg & 0x07) | if evex.r { 0 } else { 8 } | if evex.r_prime { 0 } else { 16 };
    let src1 = (evex.vvvv ^ 0xF) | if evex.v_prime { 0 } else { 16 };
    let src2_base = (rm & 0x07) | if evex.b { 0 } else { 8 } | if evex.x { 0 } else { 16 };

    let vl = match evex.ll {
        0 => 128,
        1 => 256,
        2 => 512,
        _ => 128,
    };
    let num_elements = vl / 64; // Number of qword elements

    // Get source 1 register
    let src1_val = get_zmm(vcpu, src1);

    // Get source 2 (register or memory, with broadcast support)
    let src2_val = if is_memory {
        if evex.broadcast {
            // Broadcast single qword to all elements
            let elem = vcpu.read_mem(addr, 8)?;
            [elem, elem, elem, elem, elem, elem, elem, elem]
        } else {
            // Read vector from memory
            let mut val = [0u64; 8];
            for i in 0..num_elements {
                val[i] = vcpu.read_mem(addr + (i as u64) * 8, 8)?;
            }
            val
        }
    } else {
        get_zmm(vcpu, src2_base)
    };

    // Get opmask register value (k0 means no masking)
    let mask = if evex.aaa == 0 {
        0xFF // No masking
    } else {
        vcpu.regs.k[evex.aaa as usize] as u8
    };

    // Get current destination value (for merge-masking)
    let dest_val = get_zmm(vcpu, dest);

    // Perform multiplication
    let mut result = [0u64; 8];
    for i in 0..num_elements {
        let bit = 1 << i;
        if mask & bit != 0 {
            // Multiply and keep low 64 bits
            let a = src1_val[i] as i64;
            let b = src2_val[i] as i64;
            result[i] = a.wrapping_mul(b) as u64;
        } else if evex.z {
            // Zeroing-masking
            result[i] = 0;
        } else {
            // Merge-masking: keep original value
            result[i] = dest_val[i];
        }
    }

    // Zero upper elements beyond vector length
    for i in num_elements..8 {
        result[i] = 0;
    }

    // Write result based on vector length
    match vl {
        128 => set_xmm_zero_upper(vcpu, dest, [result[0], result[1]]),
        256 => set_ymm_zero_upper(vcpu, dest, [result[0], result[1], result[2], result[3]]),
        512 => set_zmm(vcpu, dest, result),
        _ => {}
    }

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// VPMULLD (EVEX) - Multiply Packed Signed Dword Integers and Store Low Result
/// EVEX.128/256/512.66.0F38.W0 40 /r
pub fn vpmulld_evex(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let evex = ctx
        .evex
        .ok_or_else(|| Error::Emulator("VPMULLD requires EVEX prefix".to_string()))?;

    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;

    // Calculate full register indices
    let dest = (reg & 0x07) | if evex.r { 0 } else { 8 } | if evex.r_prime { 0 } else { 16 };
    let src1 = (evex.vvvv ^ 0xF) | if evex.v_prime { 0 } else { 16 };
    let src2_base = (rm & 0x07) | if evex.b { 0 } else { 8 } | if evex.x { 0 } else { 16 };

    let vl = match evex.ll {
        0 => 128,
        1 => 256,
        2 => 512,
        _ => 128,
    };
    let num_elements = vl / 32; // Number of dword elements

    // Get source 1 register
    let src1_val = get_zmm(vcpu, src1);

    // Get source 2 (register or memory, with broadcast support)
    let src2_val = if is_memory {
        if evex.broadcast {
            // Broadcast single dword to all elements
            let elem = vcpu.read_mem(addr, 4)? as u32;
            let mut val = [0u64; 8];
            for i in 0..8 {
                val[i] = ((elem as u64) << 32) | (elem as u64);
            }
            val
        } else {
            // Read vector from memory
            let mut val = [0u64; 8];
            for i in 0..(num_elements / 2) {
                val[i] = vcpu.read_mem(addr + (i as u64) * 8, 8)?;
            }
            val
        }
    } else {
        get_zmm(vcpu, src2_base)
    };

    // Get opmask register value
    let mask = if evex.aaa == 0 {
        0xFFFF // No masking
    } else {
        vcpu.regs.k[evex.aaa as usize] as u16
    };

    // Get current destination value
    let dest_val = get_zmm(vcpu, dest);

    // Perform multiplication on dwords
    let mut result = [0u64; 8];
    for qword_idx in 0..(num_elements / 2) {
        let src1_qword = src1_val[qword_idx];
        let src2_qword = src2_val[qword_idx];

        // Process two dwords per qword
        for dword_idx in 0..2 {
            let elem_idx = qword_idx * 2 + dword_idx;
            let bit = 1 << elem_idx;
            let shift = dword_idx * 32;

            if mask & bit != 0 {
                let a = ((src1_qword >> shift) & 0xFFFFFFFF) as i32;
                let b = ((src2_qword >> shift) & 0xFFFFFFFF) as i32;
                let prod = a.wrapping_mul(b) as u32;
                result[qword_idx] |= (prod as u64) << shift;
            } else if evex.z {
                // Zeroing: leave as 0
            } else {
                // Merge: keep original dword
                // NB: `<<` binds tighter than `&` in Rust, so the mask+shift-back
                // must be parenthesized explicitly.
                result[qword_idx] |= ((dest_val[qword_idx] >> shift) & 0xFFFFFFFF) << shift;
            }
        }
    }

    // Write result based on vector length
    match vl {
        128 => set_xmm_zero_upper(vcpu, dest, [result[0], result[1]]),
        256 => set_ymm_zero_upper(vcpu, dest, [result[0], result[1], result[2], result[3]]),
        512 => set_zmm(vcpu, dest, result),
        _ => {}
    }

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

pub fn vcompress_evex(
    vcpu: &mut X86_64Vcpu,
    ctx: &mut InsnContext,
    elem_size: usize,
    name: &str,
) -> Result<Option<VcpuExit>> {
    let evex = ctx
        .evex
        .ok_or_else(|| Error::Emulator(format!("{} requires EVEX prefix", name)))?;

    if evex.vvvv != 0xF {
        return Err(Error::Emulator(format!(
            "{} requires EVEX.vvvv=1111b",
            name
        )));
    }

    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;

    let src = (reg & 0x07) | if evex.r { 0 } else { 8 } | if evex.r_prime { 0 } else { 16 };
    let dest = (rm & 0x07) | if evex.b { 0 } else { 8 } | if evex.x { 0 } else { 16 };

    let vl_bytes = evex_vl_bytes(evex.ll);
    if elem_size == 0 || vl_bytes % elem_size != 0 {
        return Err(Error::Emulator(format!("{} invalid element size", name)));
    }
    let num_elems = vl_bytes / elem_size;
    let mask = evex_mask(vcpu, evex.aaa, num_elems);

    let src_bytes = read_reg_bytes(vcpu, src, vl_bytes);
    let mut out_bytes = [0u8; 64];
    let mut out_count = 0usize;

    for j in 0..num_elems {
        if (mask >> j) & 1 != 0 {
            let src_start = j * elem_size;
            let dst_start = out_count * elem_size;
            out_bytes[dst_start..dst_start + elem_size]
                .copy_from_slice(&src_bytes[src_start..src_start + elem_size]);
            out_count += 1;
        }
    }

    if is_memory {
        if evex.z {
            return Err(Error::Emulator(format!(
                "{} memory destination does not allow EVEX.z",
                name
            )));
        }
        store_mem_bytes(vcpu, addr, elem_size, out_count, &out_bytes)?;
    } else {
        let compressed_len = out_count * elem_size;
        if !evex.z && compressed_len < vl_bytes {
            let dest_bytes = read_reg_bytes(vcpu, dest, vl_bytes);
            out_bytes[compressed_len..vl_bytes]
                .copy_from_slice(&dest_bytes[compressed_len..vl_bytes]);
        }
        write_reg_bytes(vcpu, dest, vl_bytes, &out_bytes);
    }

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

pub fn vexpand_evex(
    vcpu: &mut X86_64Vcpu,
    ctx: &mut InsnContext,
    elem_size: usize,
    name: &str,
) -> Result<Option<VcpuExit>> {
    let evex = ctx
        .evex
        .ok_or_else(|| Error::Emulator(format!("{} requires EVEX prefix", name)))?;

    if evex.vvvv != 0xF {
        return Err(Error::Emulator(format!(
            "{} requires EVEX.vvvv=1111b",
            name
        )));
    }

    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;

    let dest = (reg & 0x07) | if evex.r { 0 } else { 8 } | if evex.r_prime { 0 } else { 16 };
    let src = (rm & 0x07) | if evex.b { 0 } else { 8 } | if evex.x { 0 } else { 16 };

    let vl_bytes = evex_vl_bytes(evex.ll);
    if elem_size == 0 || vl_bytes % elem_size != 0 {
        return Err(Error::Emulator(format!("{} invalid element size", name)));
    }
    let num_elems = vl_bytes / elem_size;
    let mask = evex_mask(vcpu, evex.aaa, num_elems);

    let src_bytes = if is_memory {
        load_mem_bytes(vcpu, addr, elem_size, num_elems)?
    } else {
        read_reg_bytes(vcpu, src, vl_bytes)
    };

    let mut out_bytes = if evex.z {
        [0u8; 64]
    } else {
        read_reg_bytes(vcpu, dest, vl_bytes)
    };

    let mut src_index = 0usize;
    for j in 0..num_elems {
        let dst_start = j * elem_size;
        if (mask >> j) & 1 != 0 {
            let src_start = src_index * elem_size;
            out_bytes[dst_start..dst_start + elem_size]
                .copy_from_slice(&src_bytes[src_start..src_start + elem_size]);
            src_index += 1;
        } else if evex.z {
            out_bytes[dst_start..dst_start + elem_size].fill(0);
        }
    }

    write_reg_bytes(vcpu, dest, vl_bytes, &out_bytes);

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// ============================================================================
// Broadened AVX-512 EVEX integer / logical / compare / broadcast / move / shift
// coverage with k-mask (merge/zero) handling, VL (128/256/512) upper-zeroing,
// and broadcast for the dword/qword element forms.
// ============================================================================

/// Resolve VL in bytes (16/32/64) from EVEX.L'L.
#[inline]
fn vl_bytes_of(ll: u8) -> usize {
    match ll {
        0 => 16,
        1 => 32,
        2 => 64,
        _ => 16,
    }
}

/// Write a result byte-buffer back to the destination vector register with the
/// proper VL upper-zeroing semantics (128/256 zero the unused high bits).
#[inline]
fn write_vec_vl(vcpu: &mut X86_64Vcpu, dest: u8, vl_bytes: usize, data: &[u8; 64]) {
    write_reg_bytes(vcpu, dest, vl_bytes, data);
}

/// Decode the three vector operands of a typical EVEX `op dest{k}, src1, src2/m`
/// instruction. Returns (dest, src1, src2 reg, is_memory, addr).
#[inline]
fn evex_three_op(evex: &super::super::super::cpu::EvexPrefix, reg: u8, rm: u8) -> (u8, u8, u8) {
    let dest = (reg & 0x07) | if evex.r { 0 } else { 8 } | if evex.r_prime { 0 } else { 16 };
    let src1 = (evex.vvvv ^ 0xF) | if evex.v_prime { 0 } else { 16 };
    let src2 = (rm & 0x07) | if evex.b { 0 } else { 8 } | if evex.x { 0 } else { 16 };
    (dest, src1, src2)
}

/// Element-wise integer arithmetic/logical operation kind.
#[derive(Clone, Copy)]
pub enum IntOp {
    AddB,
    AddW,
    AddD,
    AddQ,
    SubB,
    SubW,
    SubD,
    SubQ,
    MullW,
    MullD,
    And,
    Or,
    Xor,
    Andn,
}

impl IntOp {
    /// Element size in bytes for masking granularity.
    fn elem_size(self) -> usize {
        match self {
            IntOp::AddB | IntOp::SubB => 1,
            IntOp::AddW | IntOp::SubW | IntOp::MullW => 2,
            IntOp::AddD
            | IntOp::SubD
            | IntOp::MullD
            | IntOp::And
            | IntOp::Or
            | IntOp::Xor
            | IntOp::Andn => 4,
            IntOp::AddQ | IntOp::SubQ => 8,
        }
    }

    /// True when this op uses a dword/qword element width and therefore supports
    /// embedded broadcast (`{1toN}`) of a single element from memory.
    fn supports_broadcast(self) -> bool {
        matches!(
            self,
            IntOp::AddD
                | IntOp::SubD
                | IntOp::MullD
                | IntOp::And
                | IntOp::Or
                | IntOp::Xor
                | IntOp::Andn
                | IntOp::AddQ
                | IntOp::SubQ
        )
    }
}

/// Compute a single element result given the element bytes of src1 and src2.
fn int_op_elem(op: IntOp, a: &[u8], b: &[u8], out: &mut [u8]) {
    match op {
        IntOp::AddB => out[0] = a[0].wrapping_add(b[0]),
        IntOp::SubB => out[0] = a[0].wrapping_sub(b[0]),
        IntOp::AddW => {
            let r = u16::from_le_bytes([a[0], a[1]]).wrapping_add(u16::from_le_bytes([b[0], b[1]]));
            out[0..2].copy_from_slice(&r.to_le_bytes());
        }
        IntOp::SubW => {
            let r = u16::from_le_bytes([a[0], a[1]]).wrapping_sub(u16::from_le_bytes([b[0], b[1]]));
            out[0..2].copy_from_slice(&r.to_le_bytes());
        }
        IntOp::MullW => {
            let r = (i16::from_le_bytes([a[0], a[1]]) as i32)
                .wrapping_mul(i16::from_le_bytes([b[0], b[1]]) as i32) as u16;
            out[0..2].copy_from_slice(&r.to_le_bytes());
        }
        IntOp::AddD => {
            let r = u32::from_le_bytes([a[0], a[1], a[2], a[3]])
                .wrapping_add(u32::from_le_bytes([b[0], b[1], b[2], b[3]]));
            out[0..4].copy_from_slice(&r.to_le_bytes());
        }
        IntOp::SubD => {
            let r = u32::from_le_bytes([a[0], a[1], a[2], a[3]])
                .wrapping_sub(u32::from_le_bytes([b[0], b[1], b[2], b[3]]));
            out[0..4].copy_from_slice(&r.to_le_bytes());
        }
        IntOp::MullD => {
            let r = (i32::from_le_bytes([a[0], a[1], a[2], a[3]]))
                .wrapping_mul(i32::from_le_bytes([b[0], b[1], b[2], b[3]]))
                as u32;
            out[0..4].copy_from_slice(&r.to_le_bytes());
        }
        IntOp::And => {
            for i in 0..4 {
                out[i] = a[i] & b[i];
            }
        }
        IntOp::Or => {
            for i in 0..4 {
                out[i] = a[i] | b[i];
            }
        }
        IntOp::Xor => {
            for i in 0..4 {
                out[i] = a[i] ^ b[i];
            }
        }
        IntOp::Andn => {
            // NOT(a) AND b  (a == src1 == vvvv)
            for i in 0..4 {
                out[i] = (!a[i]) & b[i];
            }
        }
        IntOp::AddQ => {
            let r = u64::from_le_bytes([a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7]])
                .wrapping_add(u64::from_le_bytes([
                    b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7],
                ]));
            out[0..8].copy_from_slice(&r.to_le_bytes());
        }
        IntOp::SubQ => {
            let r = u64::from_le_bytes([a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7]])
                .wrapping_sub(u64::from_le_bytes([
                    b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7],
                ]));
            out[0..8].copy_from_slice(&r.to_le_bytes());
        }
    }
}

/// Generic EVEX integer arithmetic / logical instruction with per-element
/// k-masking (merge/zero), VL handling and (for D/Q forms) embedded broadcast.
pub fn evex_int_arith(
    vcpu: &mut X86_64Vcpu,
    ctx: &mut InsnContext,
    op: IntOp,
) -> Result<Option<VcpuExit>> {
    let evex = ctx
        .evex
        .ok_or_else(|| Error::Emulator("EVEX integer op requires EVEX prefix".to_string()))?;

    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let (dest, src1, src2_reg) = evex_three_op(&evex, reg, rm);

    let vl_bytes = vl_bytes_of(evex.ll);
    let elem_size = op.elem_size();
    let num_elems = vl_bytes / elem_size;

    let src1_bytes = read_reg_bytes(vcpu, src1, vl_bytes);

    // Load src2 from register or memory (with embedded broadcast for D/Q forms).
    let src2_bytes = if is_memory {
        if evex.broadcast && op.supports_broadcast() {
            // Broadcast a single element across all lanes.
            let elem = vcpu.read_mem(addr, elem_size as u8)?;
            let elem_le = elem.to_le_bytes();
            let mut data = [0u8; 64];
            for i in 0..num_elems {
                let base = i * elem_size;
                data[base..base + elem_size].copy_from_slice(&elem_le[..elem_size]);
            }
            data
        } else {
            load_mem_bytes(vcpu, addr, elem_size, num_elems)?
        }
    } else {
        read_reg_bytes(vcpu, src2_reg, vl_bytes)
    };

    let dest_old = read_reg_bytes(vcpu, dest, vl_bytes);
    let mask = evex_mask(vcpu, evex.aaa, num_elems);

    let mut result = [0u8; 64];
    for i in 0..num_elems {
        let base = i * elem_size;
        if (mask >> i) & 1 != 0 {
            int_op_elem(
                op,
                &src1_bytes[base..base + elem_size],
                &src2_bytes[base..base + elem_size],
                &mut result[base..base + elem_size],
            );
        } else if evex.z {
            // Zeroing: leave as 0.
        } else {
            result[base..base + elem_size].copy_from_slice(&dest_old[base..base + elem_size]);
        }
    }

    write_vec_vl(vcpu, dest, vl_bytes, &result);

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// Integer compare predicate (for the EQ/GT fixed forms and the imm8 VPCMP form).
#[derive(Clone, Copy, PartialEq)]
pub enum CmpPred {
    Eq,
    Lt,
    Le,
    /// "False" (never true) – predicate 3 for VPCMP.
    FalseP,
    Ne,
    Nlt,
    Nle,
    /// "True" (always true) – predicate 7 for VPCMP.
    TrueP,
    /// Greater-than (used by the dedicated VPCMPGT* forms).
    Gt,
}

impl CmpPred {
    fn from_imm(imm: u8) -> CmpPred {
        match imm & 0x7 {
            0 => CmpPred::Eq,
            1 => CmpPred::Lt,
            2 => CmpPred::Le,
            3 => CmpPred::FalseP,
            4 => CmpPred::Ne,
            5 => CmpPred::Nlt,
            6 => CmpPred::Nle,
            _ => CmpPred::TrueP,
        }
    }
}

/// Evaluate the compare predicate over two signed/unsigned integers represented
/// as i128 (signed) or u128 (unsigned) ordering.
fn cmp_eval_signed(pred: CmpPred, a: i128, b: i128) -> bool {
    match pred {
        CmpPred::Eq => a == b,
        CmpPred::Lt => a < b,
        CmpPred::Le => a <= b,
        CmpPred::FalseP => false,
        CmpPred::Ne => a != b,
        CmpPred::Nlt => a >= b,
        CmpPred::Nle => a > b,
        CmpPred::TrueP => true,
        CmpPred::Gt => a > b,
    }
}

fn cmp_eval_unsigned(pred: CmpPred, a: u128, b: u128) -> bool {
    match pred {
        CmpPred::Eq => a == b,
        CmpPred::Lt => a < b,
        CmpPred::Le => a <= b,
        CmpPred::FalseP => false,
        CmpPred::Ne => a != b,
        CmpPred::Nlt => a >= b,
        CmpPred::Nle => a > b,
        CmpPred::TrueP => true,
        CmpPred::Gt => a > b,
    }
}

/// Read a single element as a signed i128 (sign-extended) from a byte slice.
fn elem_signed(bytes: &[u8], elem_size: usize) -> i128 {
    match elem_size {
        1 => bytes[0] as i8 as i128,
        2 => i16::from_le_bytes([bytes[0], bytes[1]]) as i128,
        4 => i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as i128,
        8 => i64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]) as i128,
        _ => 0,
    }
}

/// Read a single element as an unsigned u128 from a byte slice.
fn elem_unsigned(bytes: &[u8], elem_size: usize) -> u128 {
    match elem_size {
        1 => bytes[0] as u128,
        2 => u16::from_le_bytes([bytes[0], bytes[1]]) as u128,
        4 => u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as u128,
        8 => u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]) as u128,
        _ => 0,
    }
}

/// Generic EVEX integer compare into a k-mask destination.
///
/// `elem_size`: 1/2/4/8. `signed`: signed vs unsigned ordering. `pred`: predicate.
/// `has_imm`: true for the VPCMP[U]B/W/D/Q imm8 forms (predicate from imm8 &7).
/// Supports embedded broadcast for D/Q element widths.
pub fn evex_int_cmp(
    vcpu: &mut X86_64Vcpu,
    ctx: &mut InsnContext,
    elem_size: usize,
    signed: bool,
    fixed_pred: CmpPred,
    has_imm: bool,
) -> Result<Option<VcpuExit>> {
    let evex = ctx
        .evex
        .ok_or_else(|| Error::Emulator("EVEX compare requires EVEX prefix".to_string()))?;

    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;

    // Destination is a k-mask register (reg field, low 3 bits).
    let k_dst = (reg & 0x7) as usize;
    let src1 = (evex.vvvv ^ 0xF) | if evex.v_prime { 0 } else { 16 };
    let src2_reg = (rm & 0x07) | if evex.b { 0 } else { 8 } | if evex.x { 0 } else { 16 };

    let vl_bytes = vl_bytes_of(evex.ll);
    let num_elems = vl_bytes / elem_size;

    let src1_bytes = read_reg_bytes(vcpu, src1, vl_bytes);

    let broadcast_ok = elem_size == 4 || elem_size == 8;
    let src2_bytes = if is_memory {
        if evex.broadcast && broadcast_ok {
            let elem = vcpu.read_mem(addr, elem_size as u8)?;
            let elem_le = elem.to_le_bytes();
            let mut data = [0u8; 64];
            for i in 0..num_elems {
                let base = i * elem_size;
                data[base..base + elem_size].copy_from_slice(&elem_le[..elem_size]);
            }
            data
        } else {
            load_mem_bytes(vcpu, addr, elem_size, num_elems)?
        }
    } else {
        read_reg_bytes(vcpu, src2_reg, vl_bytes)
    };

    let pred = if has_imm {
        let imm = ctx.consume_u8()?;
        CmpPred::from_imm(imm)
    } else {
        fixed_pred
    };

    let writemask = evex_mask(vcpu, evex.aaa, num_elems);

    let mut result: u64 = 0;
    for i in 0..num_elems {
        // Only compute for active elements (k1 governs which lanes participate).
        if (writemask >> i) & 1 == 0 {
            continue;
        }
        let base = i * elem_size;
        let cond = if signed {
            cmp_eval_signed(
                pred,
                elem_signed(&src1_bytes[base..base + elem_size], elem_size),
                elem_signed(&src2_bytes[base..base + elem_size], elem_size),
            )
        } else {
            cmp_eval_unsigned(
                pred,
                elem_unsigned(&src1_bytes[base..base + elem_size], elem_size),
                elem_unsigned(&src2_bytes[base..base + elem_size], elem_size),
            )
        };
        if cond {
            result |= 1u64 << i;
        }
    }

    // The k-destination is fully written (inactive lanes produce 0 above).
    vcpu.regs.k[k_dst] = result;

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// EVEX broadcast of a scalar element to all lanes (VPBROADCASTB/W/D/Q,
/// VBROADCASTSS/SD). Source is xmm[0] (or memory scalar). Honors masking and VL.
pub fn evex_broadcast(
    vcpu: &mut X86_64Vcpu,
    ctx: &mut InsnContext,
    elem_size: usize,
) -> Result<Option<VcpuExit>> {
    let evex = ctx
        .evex
        .ok_or_else(|| Error::Emulator("EVEX broadcast requires EVEX prefix".to_string()))?;

    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let dest = (reg & 0x07) | if evex.r { 0 } else { 8 } | if evex.r_prime { 0 } else { 16 };
    let src_reg = (rm & 0x07) | if evex.b { 0 } else { 8 } | if evex.x { 0 } else { 16 };

    let vl_bytes = vl_bytes_of(evex.ll);
    let num_elems = vl_bytes / elem_size;

    // Fetch the scalar element bytes.
    let mut elem = [0u8; 8];
    if is_memory {
        let v = vcpu.read_mem(addr, elem_size as u8)?;
        let le = v.to_le_bytes();
        elem[..elem_size].copy_from_slice(&le[..elem_size]);
    } else {
        let src_bytes = read_reg_bytes(vcpu, src_reg, 16);
        elem[..elem_size].copy_from_slice(&src_bytes[..elem_size]);
    }

    let dest_old = read_reg_bytes(vcpu, dest, vl_bytes);
    let mask = evex_mask(vcpu, evex.aaa, num_elems);

    let mut result = [0u8; 64];
    for i in 0..num_elems {
        let base = i * elem_size;
        if (mask >> i) & 1 != 0 {
            result[base..base + elem_size].copy_from_slice(&elem[..elem_size]);
        } else if evex.z {
            // zeroing
        } else {
            result[base..base + elem_size].copy_from_slice(&dest_old[base..base + elem_size]);
        }
    }

    write_vec_vl(vcpu, dest, vl_bytes, &result);

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// EVEX masked move (VMOVDQA32/64, VMOVDQU8/16/32/64, VMOVUPS/PD) - load form
/// (reg <- reg/mem). `elem_size` is the masking granularity. `aligned` enforces
/// alignment for the A-forms when the operand is memory.
pub fn evex_mov_masked_load(
    vcpu: &mut X86_64Vcpu,
    ctx: &mut InsnContext,
    elem_size: usize,
    aligned: bool,
) -> Result<Option<VcpuExit>> {
    let evex = ctx
        .evex
        .ok_or_else(|| Error::Emulator("EVEX masked move requires EVEX prefix".to_string()))?;

    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let dest = (reg & 0x07) | if evex.r { 0 } else { 8 } | if evex.r_prime { 0 } else { 16 };
    let src_reg = (rm & 0x07) | if evex.b { 0 } else { 8 } | if evex.x { 0 } else { 16 };

    let vl_bytes = vl_bytes_of(evex.ll);
    let num_elems = vl_bytes / elem_size;

    if is_memory && aligned && (addr % vl_bytes as u64) != 0 {
        return Err(Error::Emulator(format!(
            "VMOVDQA: unaligned memory access at {:#x}",
            addr
        )));
    }

    let src_bytes = if is_memory {
        load_mem_bytes(vcpu, addr, elem_size, num_elems)?
    } else {
        read_reg_bytes(vcpu, src_reg, vl_bytes)
    };

    let dest_old = read_reg_bytes(vcpu, dest, vl_bytes);
    let mask = evex_mask(vcpu, evex.aaa, num_elems);

    let mut result = [0u8; 64];
    for i in 0..num_elems {
        let base = i * elem_size;
        if (mask >> i) & 1 != 0 {
            result[base..base + elem_size].copy_from_slice(&src_bytes[base..base + elem_size]);
        } else if evex.z {
            // zeroing
        } else {
            result[base..base + elem_size].copy_from_slice(&dest_old[base..base + elem_size]);
        }
    }

    write_vec_vl(vcpu, dest, vl_bytes, &result);

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// EVEX masked move store form (reg/mem <- reg). For memory destinations only
/// active elements are written; for register destinations merge/zero applies.
pub fn evex_mov_masked_store(
    vcpu: &mut X86_64Vcpu,
    ctx: &mut InsnContext,
    elem_size: usize,
    aligned: bool,
) -> Result<Option<VcpuExit>> {
    let evex = ctx
        .evex
        .ok_or_else(|| Error::Emulator("EVEX masked move requires EVEX prefix".to_string()))?;

    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let src = (reg & 0x07) | if evex.r { 0 } else { 8 } | if evex.r_prime { 0 } else { 16 };
    let dest_reg = (rm & 0x07) | if evex.b { 0 } else { 8 } | if evex.x { 0 } else { 16 };

    let vl_bytes = vl_bytes_of(evex.ll);
    let num_elems = vl_bytes / elem_size;

    if is_memory && aligned && (addr % vl_bytes as u64) != 0 {
        return Err(Error::Emulator(format!(
            "VMOVDQA: unaligned memory access at {:#x}",
            addr
        )));
    }

    let src_bytes = read_reg_bytes(vcpu, src, vl_bytes);
    let mask = evex_mask(vcpu, evex.aaa, num_elems);

    if is_memory {
        // Write only active elements; inactive lanes leave memory unchanged.
        for i in 0..num_elems {
            if (mask >> i) & 1 != 0 {
                let base = i * elem_size;
                let value = if elem_size == 4 {
                    u32::from_le_bytes([
                        src_bytes[base],
                        src_bytes[base + 1],
                        src_bytes[base + 2],
                        src_bytes[base + 3],
                    ]) as u64
                } else if elem_size == 8 {
                    u64::from_le_bytes([
                        src_bytes[base],
                        src_bytes[base + 1],
                        src_bytes[base + 2],
                        src_bytes[base + 3],
                        src_bytes[base + 4],
                        src_bytes[base + 5],
                        src_bytes[base + 6],
                        src_bytes[base + 7],
                    ])
                } else if elem_size == 2 {
                    u16::from_le_bytes([src_bytes[base], src_bytes[base + 1]]) as u64
                } else {
                    src_bytes[base] as u64
                };
                vcpu.write_mem(addr + (i * elem_size) as u64, value, elem_size as u8)?;
            }
        }
    } else {
        let dest_old = read_reg_bytes(vcpu, dest_reg, vl_bytes);
        let mut result = [0u8; 64];
        for i in 0..num_elems {
            let base = i * elem_size;
            if (mask >> i) & 1 != 0 {
                result[base..base + elem_size].copy_from_slice(&src_bytes[base..base + elem_size]);
            } else if evex.z {
                // zeroing
            } else {
                result[base..base + elem_size].copy_from_slice(&dest_old[base..base + elem_size]);
            }
        }
        write_vec_vl(vcpu, dest_reg, vl_bytes, &result);
    }

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// Shift kind for the EVEX packed-shift implementation.
#[derive(Clone, Copy)]
pub enum ShiftKind {
    Sll,
    Srl,
    Sra,
}

/// Apply a logical/arithmetic shift to a single element value (as u64), masking
/// to `elem_size`. Shift counts >= bit width produce 0 (logical) or the sign
/// fill (arithmetic), matching x86 packed-shift semantics.
fn shift_elem(kind: ShiftKind, val: u64, count: u64, elem_size: usize) -> u64 {
    let bits = (elem_size * 8) as u64;
    match kind {
        ShiftKind::Sll => {
            if count >= bits {
                0
            } else {
                let r = val << count;
                match elem_size {
                    4 => r & 0xFFFF_FFFF,
                    8 => r,
                    2 => r & 0xFFFF,
                    _ => r & 0xFF,
                }
            }
        }
        ShiftKind::Srl => {
            if count >= bits {
                0
            } else {
                let m = if elem_size == 8 {
                    u64::MAX
                } else {
                    (1u64 << bits) - 1
                };
                (val & m) >> count
            }
        }
        ShiftKind::Sra => {
            let sh = if count >= bits { bits - 1 } else { count };
            match elem_size {
                4 => (((val as u32) as i32) >> sh) as u32 as u64,
                8 => ((val as i64) >> sh) as u64,
                2 => (((val as u16) as i16) >> sh) as u16 as u64,
                _ => (((val as u8) as i8) >> sh) as u8 as u64,
            }
        }
    }
}

/// EVEX packed shift by immediate (VPSLLD/Q, VPSRLD/Q, VPSRAD/Q with imm8).
/// These are the `/r` group forms where the destination comes from EVEX.vvvv
/// (NDD) and the shifted source is the ModR/M r/m operand. Honors masking + VL.
pub fn evex_shift_imm(
    vcpu: &mut X86_64Vcpu,
    ctx: &mut InsnContext,
    kind: ShiftKind,
    elem_size: usize,
) -> Result<Option<VcpuExit>> {
    let evex = ctx
        .evex
        .ok_or_else(|| Error::Emulator("EVEX shift requires EVEX prefix".to_string()))?;

    let (_reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;

    // Destination = EVEX.vvvv (the group-encoded shift writes back to vvvv).
    let dest = (evex.vvvv ^ 0xF) | if evex.v_prime { 0 } else { 16 };
    let src_reg = (rm & 0x07) | if evex.b { 0 } else { 8 } | if evex.x { 0 } else { 16 };

    let vl_bytes = vl_bytes_of(evex.ll);
    let num_elems = vl_bytes / elem_size;

    // src elements, then imm8 shift count.
    let src_bytes = if is_memory {
        if evex.broadcast {
            let elem = vcpu.read_mem(addr, elem_size as u8)?;
            let le = elem.to_le_bytes();
            let mut data = [0u8; 64];
            for i in 0..num_elems {
                let base = i * elem_size;
                data[base..base + elem_size].copy_from_slice(&le[..elem_size]);
            }
            data
        } else {
            load_mem_bytes(vcpu, addr, elem_size, num_elems)?
        }
    } else {
        read_reg_bytes(vcpu, src_reg, vl_bytes)
    };

    let count = ctx.consume_u8()? as u64;

    let dest_old = read_reg_bytes(vcpu, dest, vl_bytes);
    let mask = evex_mask(vcpu, evex.aaa, num_elems);

    let mut result = [0u8; 64];
    for i in 0..num_elems {
        let base = i * elem_size;
        if (mask >> i) & 1 != 0 {
            let val = elem_unsigned(&src_bytes[base..base + elem_size], elem_size) as u64;
            let r = shift_elem(kind, val, count, elem_size);
            let le = r.to_le_bytes();
            result[base..base + elem_size].copy_from_slice(&le[..elem_size]);
        } else if evex.z {
            // zeroing
        } else {
            result[base..base + elem_size].copy_from_slice(&dest_old[base..base + elem_size]);
        }
    }

    write_vec_vl(vcpu, dest, vl_bytes, &result);

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// EVEX packed shift by xmm count (VPSLLD/Q, VPSRLD/Q, VPSRAD/Q with xmm count).
/// dest{k} = src1 shifted by the scalar count in the low 64 bits of src2/m128.
pub fn evex_shift_var(
    vcpu: &mut X86_64Vcpu,
    ctx: &mut InsnContext,
    kind: ShiftKind,
    elem_size: usize,
) -> Result<Option<VcpuExit>> {
    let evex = ctx
        .evex
        .ok_or_else(|| Error::Emulator("EVEX shift requires EVEX prefix".to_string()))?;

    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let (dest, src1, src2_reg) = evex_three_op(&evex, reg, rm);

    let vl_bytes = vl_bytes_of(evex.ll);
    let num_elems = vl_bytes / elem_size;

    let src1_bytes = read_reg_bytes(vcpu, src1, vl_bytes);

    // Count is the full low 64-bit value of the xmm/m128 source.
    let count = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        let c = read_reg_bytes(vcpu, src2_reg, 16);
        u64::from_le_bytes([c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7]])
    };

    let dest_old = read_reg_bytes(vcpu, dest, vl_bytes);
    let mask = evex_mask(vcpu, evex.aaa, num_elems);

    let mut result = [0u8; 64];
    for i in 0..num_elems {
        let base = i * elem_size;
        if (mask >> i) & 1 != 0 {
            let val = elem_unsigned(&src1_bytes[base..base + elem_size], elem_size) as u64;
            let r = shift_elem(kind, val, count, elem_size);
            let le = r.to_le_bytes();
            result[base..base + elem_size].copy_from_slice(&le[..elem_size]);
        } else if evex.z {
            // zeroing
        } else {
            result[base..base + elem_size].copy_from_slice(&dest_old[base..base + elem_size]);
        }
    }

    write_vec_vl(vcpu, dest, vl_bytes, &result);

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
