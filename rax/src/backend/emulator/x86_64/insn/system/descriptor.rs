//! Descriptor table instructions: LAR, LSL, Group 6.

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::cpu::{InsnContext, X86_64Vcpu};
use super::super::super::flags;
use super::control_regs::{is_cpl0, raise_gp0};

/// Group 6 - SLDT, STR, LLDT, LTR, VERR, VERW (0x0F 0x00)
pub fn group6(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let modrm_start = ctx.cursor;
    let modrm = ctx.consume_u8()?;
    let reg_op = (modrm >> 3) & 0x07;
    let rm = (modrm & 0x07) | ctx.rex_b();
    let is_memory = modrm >> 6 != 3;

    match reg_op {
        // SLDT - Store Local Descriptor Table (0x0F 0x00 /0)
        0 => {
            let selector = vcpu.sregs.ldt.selector;
            if is_memory {
                let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
                ctx.cursor = modrm_start + 1 + extra;
                vcpu.mmu.write_u16(addr, selector, &vcpu.sregs)?;
            } else {
                // Writing to register - zero-extends for 32/64-bit registers
                vcpu.set_reg(rm, selector as u64, ctx.op_size);
            }
        }
        // STR - Store Task Register (0x0F 0x00 /1)
        1 => {
            let selector = vcpu.sregs.tr.selector;
            if is_memory {
                let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
                ctx.cursor = modrm_start + 1 + extra;
                vcpu.mmu.write_u16(addr, selector, &vcpu.sregs)?;
            } else {
                vcpu.set_reg(rm, selector as u64, ctx.op_size);
            }
        }
        // LLDT - Load Local Descriptor Table (0x0F 0x00 /2)
        2 => {
            // Privileged: loading the LDTR requires CPL 0.
            if !is_cpl0(vcpu) {
                return raise_gp0(vcpu);
            }
            let selector = if is_memory {
                let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
                ctx.cursor = modrm_start + 1 + extra;
                vcpu.mmu.read_u16(addr, &vcpu.sregs)?
            } else {
                vcpu.get_reg(rm, 2) as u16
            };
            vcpu.sregs.ldt.selector = selector;
            // In a real implementation, we'd load the descriptor from the GDT
            // For emulation purposes, just store the selector
        }
        // LTR - Load Task Register (0x0F 0x00 /3)
        3 => {
            // Privileged: loading the task register requires CPL 0.
            if !is_cpl0(vcpu) {
                return raise_gp0(vcpu);
            }
            let selector = if is_memory {
                let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
                ctx.cursor = modrm_start + 1 + extra;
                vcpu.mmu.read_u16(addr, &vcpu.sregs)?
            } else {
                vcpu.get_reg(rm, 2) as u16
            };
            vcpu.sregs.tr.selector = selector;

            // Load the TSS descriptor from the GDT
            // In 64-bit mode, TSS descriptor is 16 bytes (system segment descriptor)
            let gdt_base = vcpu.sregs.gdt.base;
            let index = (selector >> 3) as u64;
            let desc_addr = gdt_base + index * 8;

            // Read the 16-byte system segment descriptor
            let mut desc_bytes = [0u8; 16];
            vcpu.mmu.read(desc_addr, &mut desc_bytes, &vcpu.sregs)?;

            // Parse the descriptor (64-bit TSS descriptor format)
            // Bytes 0-7: legacy descriptor format
            // Bytes 8-15: upper 32 bits of base address + reserved
            let limit_low = u16::from_le_bytes([desc_bytes[0], desc_bytes[1]]) as u32;
            let base_low = u16::from_le_bytes([desc_bytes[2], desc_bytes[3]]) as u64;
            let base_mid = desc_bytes[4] as u64;
            let _type_attr = desc_bytes[5];
            let limit_high = (desc_bytes[6] & 0x0F) as u32;
            let base_high_byte = desc_bytes[7] as u64;
            let base_upper =
                u32::from_le_bytes([desc_bytes[8], desc_bytes[9], desc_bytes[10], desc_bytes[11]])
                    as u64;

            let limit = limit_low | (limit_high << 16);
            let base = base_low | (base_mid << 16) | (base_high_byte << 24) | (base_upper << 32);

            vcpu.sregs.tr.base = base;
            vcpu.sregs.tr.limit = limit;
        }
        // VERR - Verify Read (0x0F 0x00 /4)
        4 => {
            let _selector = if is_memory {
                let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
                ctx.cursor = modrm_start + 1 + extra;
                vcpu.mmu.read_u16(addr, &vcpu.sregs)?
            } else {
                vcpu.get_reg(rm, 2) as u16
            };
            // In real hardware, this checks if the selector is readable
            // For emulation, we'll just set ZF=1 (readable) for non-null selectors
            if _selector != 0 {
                vcpu.regs.rflags |= flags::bits::ZF;
            } else {
                vcpu.regs.rflags &= !flags::bits::ZF;
            }
        }
        // VERW - Verify Write (0x0F 0x00 /5)
        5 => {
            let _selector = if is_memory {
                let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
                ctx.cursor = modrm_start + 1 + extra;
                vcpu.mmu.read_u16(addr, &vcpu.sregs)?
            } else {
                vcpu.get_reg(rm, 2) as u16
            };
            // For emulation, set ZF=1 (writable) for non-null selectors
            if _selector != 0 {
                vcpu.regs.rflags |= flags::bits::ZF;
            } else {
                vcpu.regs.rflags &= !flags::bits::ZF;
            }
        }
        _ => {
            return Err(Error::Emulator(format!(
                "unimplemented 0F 00 /{} at RIP={:#x}",
                reg_op, vcpu.regs.rip
            )));
        }
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// LAR - Load Access Rights (0x0F 0x02)
pub fn lar(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let modrm_start = ctx.cursor;
    let modrm = ctx.consume_u8()?;
    let reg = ((modrm >> 3) & 0x07) | ctx.rex_r();
    let rm = (modrm & 0x07) | ctx.rex_b();
    let is_memory = modrm >> 6 != 3;

    let selector = if is_memory {
        let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
        ctx.cursor = modrm_start + 1 + extra;
        vcpu.mmu.read_u16(addr, &vcpu.sregs)?
    } else {
        vcpu.get_reg(rm, 2) as u16
    };

    // In a real implementation, we'd read the descriptor from GDT/LDT
    // For emulation, return a standard code/data segment access rights
    if selector != 0 {
        // Return typical access rights: present, ring 0, code/data segment
        let access_rights: u64 = 0x00CF9300; // Standard access rights
        vcpu.set_reg(reg, access_rights, ctx.op_size);
        vcpu.regs.rflags |= flags::bits::ZF; // Valid selector
    } else {
        vcpu.regs.rflags &= !flags::bits::ZF; // Null selector
    }

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// LSL - Load Segment Limit (0x0F 0x03)
pub fn lsl(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let modrm_start = ctx.cursor;
    let modrm = ctx.consume_u8()?;
    let reg = ((modrm >> 3) & 0x07) | ctx.rex_r();
    let rm = (modrm & 0x07) | ctx.rex_b();
    let is_memory = modrm >> 6 != 3;

    let selector = if is_memory {
        let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
        ctx.cursor = modrm_start + 1 + extra;
        vcpu.mmu.read_u16(addr, &vcpu.sregs)?
    } else {
        vcpu.get_reg(rm, 2) as u16
    };

    // For emulation, return max limit for valid selectors
    if selector != 0 {
        let limit: u64 = 0xFFFFFFFF; // Max 4GB limit (granularity bit set)
        vcpu.set_reg(reg, limit, ctx.op_size);
        vcpu.regs.rflags |= flags::bits::ZF; // Valid selector
    } else {
        vcpu.regs.rflags &= !flags::bits::ZF; // Null selector
    }

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
