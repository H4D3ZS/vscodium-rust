//! Stack frame instructions: ENTER, LEAVE, BOUND, and EVEX dispatch.

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::cpu::{EvexPrefix, InsnContext, X86_64Vcpu};

/// ENTER imm16, imm8 (0xC8) - Create stack frame
pub fn enter(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let alloc_size = ctx.consume_u16()? as u64;
    let nesting_level = (ctx.consume_u8()? & 0x1F) as u64;

    // Push RBP
    vcpu.push64(vcpu.regs.rbp)?;
    let frame_ptr = vcpu.regs.rsp;

    if nesting_level > 0 {
        // Push nested frame pointers
        for i in 1..nesting_level {
            vcpu.regs.rbp = vcpu.regs.rbp.wrapping_sub(8);
            let ptr = vcpu.read_mem(vcpu.regs.rbp, 8)?;
            vcpu.push64(ptr)?;
            let _ = i;
        }
        vcpu.push64(frame_ptr)?;
    }

    vcpu.regs.rbp = frame_ptr;
    vcpu.regs.rsp = vcpu.regs.rsp.wrapping_sub(alloc_size);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// LEAVE (0xC9)
pub fn leave(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    vcpu.regs.rsp = vcpu.regs.rbp;
    vcpu.regs.rbp = vcpu.pop64()?;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// BOUND (32-bit) or EVEX prefix (64-bit) (0x62)
pub fn bound_or_evex(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    // Check if we're in 64-bit mode by looking at EFER.LMA AND CS.L
    // EFER.LMA = 1 and CS.L = 1 means 64-bit mode (EVEX)
    // EFER.LMA = 1 and CS.L = 0 means compatibility mode (BOUND)
    // EFER.LMA = 0 means legacy/real mode (BOUND)
    let in_long_mode = (vcpu.sregs.efer & 0x400) != 0; // EFER.LMA = bit 10
    let in_64bit_mode = in_long_mode && vcpu.sregs.cs.l;

    if in_64bit_mode {
        // In 64-bit mode, 0x62 is EVEX prefix (AVX-512)
        // Decode 3-byte EVEX payload
        let p0 = ctx.consume_u8()?;
        let p1 = ctx.consume_u8()?;
        let p2 = ctx.consume_u8()?;

        let mm = p0 & 0x07; // mm field (opcode map)
        let apx_mode = mm == 4;

        // Validate EVEX format:
        // P0 bit 3 is fixed zero for standard EVEX, but APX MAP4 reuses it as B4.
        // P1 bit 2 is fixed one for standard EVEX, but APX MAP4 reuses it as X4.
        if ((p0 & 0x08) != 0 && !apx_mode) || ((p1 & 0x04) == 0 && !apx_mode) {
            return Err(Error::Emulator(format!(
                "Invalid EVEX prefix at RIP={:#x}: P0={:#x} P1={:#x}",
                vcpu.regs.rip, p0, p1
            )));
        }

        // Decode P0: R X B R' 0 m m m
        let r = (p0 & 0x80) != 0; // R bit (inverted)
        let x = (p0 & 0x40) != 0; // X bit (inverted)
        let b = (p0 & 0x20) != 0; // B bit (inverted)
        let r_prime = (p0 & 0x10) != 0; // R' bit (inverted)

        // Decode P1: W v v v v 1 p p
        let w = (p1 & 0x80) != 0; // W bit
        let vvvv = (p1 >> 3) & 0x0F; // vvvv field (inverted)
        let pp = p1 & 0x03; // pp field (implied prefix)

        // Decode P2: z L' L b V' a a a
        let z = (p2 & 0x80) != 0; // z bit (zeroing)
        let ll = (p2 >> 5) & 0x03; // L'L field
        let broadcast = (p2 & 0x10) != 0; // b bit
        let v_prime = (p2 & 0x08) != 0; // V' bit (inverted)
        let aaa = p2 & 0x07; // aaa field (opmask)

        // For APX mode, decode additional bits differently:
        // - P2[2] becomes NF (No Flags)
        // - P2[4] (broadcast bit) becomes ND (New Data Destination)
        // - P0[3] becomes B4, the high r/m/base extension bit for EGPR
        // - P1[2] becomes X4, the inverted high SIB index extension bit
        let (nf, nd, b4, x4) = if apx_mode {
            // In APX mode:
            // NF is encoded in P2 bit 2 and is non-inverted.
            // ND is in P2 bit 4 (broadcast position)
            // B4 is encoded in P0 bit 3 and is non-inverted.
            // X4 is encoded in P1 bit 2 and is inverted like EVEX.X.
            let nf_bit = (p2 & 0x04) != 0;
            let nd_bit = broadcast; // ND uses broadcast position when mm=4
            let b4_bit = (p0 & 0x08) != 0;
            let x4_bit = (p1 & 0x04) != 0;
            (nf_bit, nd_bit, b4_bit, x4_bit)
        } else {
            (false, false, false, false)
        };

        // Store EVEX prefix in context
        ctx.evex = Some(EvexPrefix {
            r,
            x,
            b,
            r_prime,
            mm,
            w,
            vvvv,
            pp,
            z,
            ll,
            broadcast,
            v_prime,
            aaa,
            // APX-specific fields
            b4,
            x4,
            nd,
            nf,
            apx_mode,
        });

        // Set operand size based on W bit
        ctx.op_size = if w { 8 } else { 4 };

        // Set implied prefix flags based on pp
        match pp {
            1 => ctx.operand_size_override = true, // 66
            2 => ctx.rep_prefix = Some(0xF3),      // F3
            3 => ctx.rep_prefix = Some(0xF2),      // F2
            _ => {}
        }

        // Dispatch to EVEX instruction handler
        return vcpu.execute_evex(ctx, mm);
    } else {
        // In 32-bit/compatibility mode, this is BOUND (bounds check)
        let modrm_start = ctx.cursor;
        let modrm = ctx.consume_u8()?;
        let reg = (modrm >> 3) & 7;

        // BOUND requires memory operand (mod != 11)
        if modrm >> 6 == 3 {
            return Err(Error::Emulator(format!(
                "BOUND requires memory operand at RIP={:#x}",
                vcpu.regs.rip
            )));
        }

        let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
        ctx.cursor = modrm_start + 1 + extra;

        // Determine operand size (16-bit or 32-bit)
        // CS.D (db flag) determines default: D=0 means 16-bit default, D=1 means 32-bit default
        let default_16bit = !vcpu.sregs.cs.db;
        let is_16bit = default_16bit ^ ctx.operand_size_override;

        // Read the index from the register
        // Read bounds from memory: [addr] = lower, [addr+size] = upper
        if is_16bit {
            let index = vcpu.get_reg(reg, 2) as i16;
            let lower = vcpu.read_mem16(addr)? as i16;
            let upper = vcpu.read_mem16(addr + 2)? as i16;

            // Check: lower <= index <= upper
            if index < lower || index > upper {
                // #BR exception - for now just return error
                return Err(Error::Emulator(format!(
                    "BOUND range exceeded: index {} not in [{}, {}] at RIP={:#x}",
                    index, lower, upper, vcpu.regs.rip
                )));
            }
        } else {
            let index = vcpu.get_reg(reg, 4) as i32;
            let lower = vcpu.read_mem32(addr)? as i32;
            let upper = vcpu.read_mem32(addr + 4)? as i32;

            // Check: lower <= index <= upper
            if index < lower || index > upper {
                // #BR exception - for now just return error
                return Err(Error::Emulator(format!(
                    "BOUND range exceeded: index {} not in [{}, {}] at RIP={:#x}",
                    index, lower, upper, vcpu.regs.rip
                )));
            }
        }

        vcpu.regs.rip += ctx.cursor as u64;
    }
    Ok(None)
}
