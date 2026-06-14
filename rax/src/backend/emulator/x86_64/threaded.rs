//! Threaded interpretation for x86_64 emulator.
//!
//! This module provides an optimized execution path where instruction handlers
//! are inlined and directly dispatch to the next instruction without returning
//! to a central loop. This reduces function call overhead and improves branch
//! prediction.

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::cpu::{DECODE_CACHE_MASK, InsnContext, MAX_INSN_LEN, X86_64Vcpu};
use super::decoder::Decoder;

/// Batch size for threaded execution before checking for exits
const THREADED_BATCH_SIZE: usize = 64;

impl X86_64Vcpu {
    /// Run the CPU using threaded interpretation.
    ///
    /// This is an optimized execution path that keeps the hot loop tight
    /// and avoids function call overhead by inlining handlers and directly
    /// dispatching to the next instruction.
    #[inline(never)] // Prevent inlining to keep instruction cache locality
    pub fn run_threaded(&mut self) -> Result<VcpuExit> {
        // Main threaded execution loop
        loop {
            // Execute a batch of instructions before checking for exits
            match self.run_threaded_batch() {
                Ok(None) => continue,
                Ok(Some(exit)) => return Ok(exit),
                Err(e) => return Err(e),
            }
        }
    }

    /// Execute a batch of instructions using threaded interpretation.
    ///
    /// Returns None to continue, Some(exit) to exit to caller.
    #[inline(always)]
    fn run_threaded_batch(&mut self) -> Result<Option<VcpuExit>> {
        // Pre-check halt state
        if self.halted {
            return Ok(Some(VcpuExit::Hlt));
        }

        // Execute instructions in a tight loop
        for _ in 0..THREADED_BATCH_SIZE {
            // Inline the fetch-decode-execute cycle
            let exit = self.threaded_step()?;
            if exit.is_some() {
                return Ok(exit);
            }

            // Check halt after each instruction
            if self.halted {
                return Ok(Some(VcpuExit::Hlt));
            }
        }

        Ok(None)
    }

    /// Single instruction step optimized for threaded interpretation.
    ///
    /// This is similar to step() but optimized for being called in a tight loop.
    #[inline(always)]
    fn threaded_step(&mut self) -> Result<Option<VcpuExit>> {
        let rip = self.regs.rip;
        let cache_idx = (rip as usize) & DECODE_CACHE_MASK;
        let mode_tag =
            (self.sregs.cr3 & !0xFFF) | (self.sregs.cs.l as u64) | ((self.sregs.cs.db as u64) << 1);

        // Check decode cache
        let cached = self.decode_cache[cache_idx];
        if cached.rip == rip && cached.mode_tag == mode_tag {
            // Cache hit - fast path (reuse cached bytes, skip fetch)
            let mut ctx = InsnContext {
                bytes: cached.bytes,
                bytes_len: cached.bytes_len,
                cursor: if cached.rex2.map_or(false, |r| r.m) {
                    cached.cursor
                } else {
                    cached.cursor + 1
                },
                rex: cached.rex,
                rex2: cached.rex2,
                operand_size_override: cached.operand_size_override,
                address_size_override: cached.address_size_override,
                rep_prefix: cached.rep_prefix,
                op_size: cached.op_size,
                rip_relative_offset: 0,
                segment_override: cached.segment_override,
                evex: None,
                opcode: cached.opcode,
            };
            return self.dispatch_threaded(cached.opcode, &mut ctx);
        }

        // Cache miss - full decode
        let (bytes, bytes_len) = self.fetch()?;
        let mut ctx = Decoder::decode_prefixes(bytes, bytes_len, self.sregs.cs.l)?;

        // Determine operand size
        ctx.op_size = if self.sregs.cs.l {
            if ctx.any_rex_w() {
                8
            } else if ctx.operand_size_override {
                2
            } else {
                4
            }
        } else {
            let default_16bit = !self.sregs.cs.db;
            let is_16bit = default_16bit ^ ctx.operand_size_override;
            if is_16bit { 2 } else { 4 }
        };

        let opcode_cursor = ctx.cursor;
        let opcode = if ctx.rex2_m() {
            0x0F
        } else {
            ctx.consume_u8()?
        };
        ctx.opcode = opcode;

        // Resolve the fn-pointer handler so a later hit (via the main `step()`
        // path) can dispatch directly. Unmapped opcodes fall back to the match.
        let handler = Self::resolve_handler(opcode).unwrap_or(X86_64Vcpu::execute_via_match);

        // Cache the LOCK-present verdict so a later `step()` hit can skip the scan.
        let has_lock = ctx.bytes[..opcode_cursor.min(ctx.bytes_len)].contains(&0xF0);

        // Update cache
        self.decode_cache[cache_idx] = super::cpu::DecodeCacheEntry {
            rip,
            mode_tag,
            opcode,
            op_size: ctx.op_size,
            cursor: opcode_cursor,
            rex: ctx.rex,
            rex2: ctx.rex2,
            operand_size_override: ctx.operand_size_override,
            address_size_override: ctx.address_size_override,
            rep_prefix: ctx.rep_prefix,
            segment_override: ctx.segment_override,
            bytes: ctx.bytes,
            bytes_len: ctx.bytes_len,
            has_lock,
            handler,
        };

        self.dispatch_threaded(opcode, &mut ctx)
    }

    /// Threaded dispatch - delegates to standard execute for now.
    /// TODO: Re-enable inlined handlers after fixing mode-aware issues.
    #[inline(always)]
    fn dispatch_threaded(&mut self, opcode: u8, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        // For now, just use the standard execute to isolate issues
        self.execute(opcode, ctx)
    }

    // =========================================================================
    // Inlined instruction handlers for threaded interpretation
    // =========================================================================

    /// MOV r, imm (0xB8-0xBF)
    #[inline(always)]
    fn threaded_mov_r_imm(
        &mut self,
        ctx: &mut InsnContext,
        opcode: u8,
    ) -> Result<Option<VcpuExit>> {
        let reg = (opcode - 0xB8) | ctx.rex_b();
        let imm = ctx.consume_imm(ctx.op_size)?;
        self.set_reg(reg, imm, ctx.op_size);
        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// MOV r/m, r (0x89)
    #[inline(always)]
    fn threaded_mov_rm_r(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let value = self.get_reg(reg, ctx.op_size);
        if is_memory {
            self.write_mem(addr, value, ctx.op_size)?;
        } else {
            self.set_reg(rm, value, ctx.op_size);
        }
        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// MOV r, r/m (0x8B)
    #[inline(always)]
    fn threaded_mov_r_rm(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let value = if is_memory {
            self.read_mem(addr, ctx.op_size)?
        } else {
            self.get_reg(rm, ctx.op_size)
        };
        self.set_reg(reg, value, ctx.op_size);
        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// PUSH r64 (0x50-0x57)
    #[inline(always)]
    fn threaded_push_r64(&mut self, ctx: &mut InsnContext, opcode: u8) -> Result<Option<VcpuExit>> {
        let reg = (opcode & 0x07) | ctx.rex_b();
        let value = self.get_reg(reg, 8);
        self.regs.rsp = self.regs.rsp.wrapping_sub(8);
        self.write_mem(self.regs.rsp, value, 8)?;
        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// POP r64 (0x58-0x5F)
    #[inline(always)]
    fn threaded_pop_r64(&mut self, ctx: &mut InsnContext, opcode: u8) -> Result<Option<VcpuExit>> {
        let reg = (opcode & 0x07) | ctx.rex_b();
        let value = self.read_mem(self.regs.rsp, 8)?;
        self.regs.rsp = self.regs.rsp.wrapping_add(8);
        self.set_reg(reg, value, 8);
        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// ADD r/m, r (0x01)
    #[inline(always)]
    fn threaded_add_rm_r(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let src = self.get_reg(reg, ctx.op_size);
        let dst = if is_memory {
            self.read_mem(addr, ctx.op_size)?
        } else {
            self.get_reg(rm, ctx.op_size)
        };
        let result = dst.wrapping_add(src);
        if is_memory {
            self.write_mem(addr, result, ctx.op_size)?;
        } else {
            self.set_reg(rm, result, ctx.op_size);
        }
        self.set_lazy_add(dst, src, result, ctx.op_size);
        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// ADD r, r/m (0x03)
    #[inline(always)]
    fn threaded_add_r_rm(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let dst = self.get_reg(reg, ctx.op_size);
        let src = if is_memory {
            self.read_mem(addr, ctx.op_size)?
        } else {
            self.get_reg(rm, ctx.op_size)
        };
        let result = dst.wrapping_add(src);
        self.set_reg(reg, result, ctx.op_size);
        self.set_lazy_add(dst, src, result, ctx.op_size);
        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// SUB r/m, r (0x29)
    #[inline(always)]
    fn threaded_sub_rm_r(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let src = self.get_reg(reg, ctx.op_size);
        let dst = if is_memory {
            self.read_mem(addr, ctx.op_size)?
        } else {
            self.get_reg(rm, ctx.op_size)
        };
        let result = dst.wrapping_sub(src);
        if is_memory {
            self.write_mem(addr, result, ctx.op_size)?;
        } else {
            self.set_reg(rm, result, ctx.op_size);
        }
        self.set_lazy_sub(dst, src, result, ctx.op_size);
        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// SUB r, r/m (0x2B)
    #[inline(always)]
    fn threaded_sub_r_rm(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let dst = self.get_reg(reg, ctx.op_size);
        let src = if is_memory {
            self.read_mem(addr, ctx.op_size)?
        } else {
            self.get_reg(rm, ctx.op_size)
        };
        let result = dst.wrapping_sub(src);
        self.set_reg(reg, result, ctx.op_size);
        self.set_lazy_sub(dst, src, result, ctx.op_size);
        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// CMP r/m, r (0x39) - like SUB but doesn't store result
    #[inline(always)]
    fn threaded_cmp_rm_r(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let src = self.get_reg(reg, ctx.op_size);
        let dst = if is_memory {
            self.read_mem(addr, ctx.op_size)?
        } else {
            self.get_reg(rm, ctx.op_size)
        };
        let result = dst.wrapping_sub(src);
        self.set_lazy_sub(dst, src, result, ctx.op_size);
        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// CMP r, r/m (0x3B)
    #[inline(always)]
    fn threaded_cmp_r_rm(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let dst = self.get_reg(reg, ctx.op_size);
        let src = if is_memory {
            self.read_mem(addr, ctx.op_size)?
        } else {
            self.get_reg(rm, ctx.op_size)
        };
        let result = dst.wrapping_sub(src);
        self.set_lazy_sub(dst, src, result, ctx.op_size);
        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// XOR r/m, r (0x31)
    #[inline(always)]
    fn threaded_xor_rm_r(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let src = self.get_reg(reg, ctx.op_size);
        let dst = if is_memory {
            self.read_mem(addr, ctx.op_size)?
        } else {
            self.get_reg(rm, ctx.op_size)
        };
        let result = dst ^ src;
        if is_memory {
            self.write_mem(addr, result, ctx.op_size)?;
        } else {
            self.set_reg(rm, result, ctx.op_size);
        }
        self.set_lazy_logic(result, ctx.op_size);
        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// XOR r, r/m (0x33)
    #[inline(always)]
    fn threaded_xor_r_rm(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let dst = self.get_reg(reg, ctx.op_size);
        let src = if is_memory {
            self.read_mem(addr, ctx.op_size)?
        } else {
            self.get_reg(rm, ctx.op_size)
        };
        let result = dst ^ src;
        self.set_reg(reg, result, ctx.op_size);
        self.set_lazy_logic(result, ctx.op_size);
        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// TEST r/m, r (0x85)
    #[inline(always)]
    fn threaded_test_rm_r(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let src = self.get_reg(reg, ctx.op_size);
        let dst = if is_memory {
            self.read_mem(addr, ctx.op_size)?
        } else {
            self.get_reg(rm, ctx.op_size)
        };
        let result = dst & src;
        self.set_lazy_logic(result, ctx.op_size);
        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// LEA r, m (0x8D)
    #[inline(always)]
    fn threaded_lea(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let op_size = ctx.op_size;
        let modrm_start = ctx.cursor;
        let modrm = ctx.consume_u8()?;
        let reg = ((modrm >> 3) & 0x07) | ctx.rex_r();
        if modrm >> 6 == 3 {
            // LEA with register operand is undefined, but we handle it
            self.regs.rip += ctx.cursor as u64;
            return Ok(None);
        }
        // LEA yields the segment OFFSET and must ignore any FS/GS override.
        let (addr, extra) = self.decode_lea_addr(ctx, modrm_start)?;
        ctx.cursor = modrm_start + 1 + extra;
        self.set_reg(reg, addr, op_size);
        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    /// JMP rel8 (0xEB)
    #[inline(always)]
    fn threaded_jmp_rel8(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let rel = ctx.consume_u8()? as i8 as i64;
        self.regs.rip = self
            .regs
            .rip
            .wrapping_add(ctx.cursor as u64)
            .wrapping_add(rel as u64);
        Ok(None)
    }

    /// JMP rel32 (0xE9)
    #[inline(always)]
    fn threaded_jmp_rel32(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let rel = ctx.consume_u32()? as i32 as i64;
        self.regs.rip = self
            .regs
            .rip
            .wrapping_add(ctx.cursor as u64)
            .wrapping_add(rel as u64);
        Ok(None)
    }

    /// Jcc rel8 (0x70-0x7F)
    #[inline(always)]
    fn threaded_jcc_rel8(&mut self, ctx: &mut InsnContext, cc: u8) -> Result<Option<VcpuExit>> {
        let rel = ctx.consume_u8()? as i8 as i64;
        let next_rip = self.regs.rip.wrapping_add(ctx.cursor as u64);
        if self.check_condition(cc) {
            self.regs.rip = next_rip.wrapping_add(rel as u64);
        } else {
            self.regs.rip = next_rip;
        }
        Ok(None)
    }

    /// CALL rel32 (0xE8)
    #[inline(always)]
    fn threaded_call_rel32(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let rel = ctx.consume_u32()? as i32 as i64;
        let next_rip = self.regs.rip.wrapping_add(ctx.cursor as u64);
        // Push return address
        self.regs.rsp = self.regs.rsp.wrapping_sub(8);
        self.write_mem(self.regs.rsp, next_rip, 8)?;
        // Jump to target
        self.regs.rip = next_rip.wrapping_add(rel as u64);
        Ok(None)
    }

    /// RET (0xC3)
    #[inline(always)]
    fn threaded_ret(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let _ = ctx; // Unused, but keep for consistency
        let ret_addr = self.read_mem(self.regs.rsp, 8)?;
        self.regs.rsp = self.regs.rsp.wrapping_add(8);
        self.regs.rip = ret_addr;
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_threaded_batch_size() {
        // Ensure batch size is reasonable
        assert!(THREADED_BATCH_SIZE >= 16);
        assert!(THREADED_BATCH_SIZE <= 256);
    }
}
