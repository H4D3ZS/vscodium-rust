//! Decode-cache handler resolution for the function-pointer dispatch fast path.
//!
//! On a decode-cache MISS the big `execute` opcode match still runs once (to
//! produce the result for the current instruction); alongside it we resolve the
//! instruction's handler to a uniform-signature function pointer and stash it in
//! the [`DecodeCacheEntry`]. On a subsequent HIT the stored pointer is called
//! directly, skipping the `execute` match and the two-byte / escape call chain.
//!
//! The mapping here MIRRORS `dispatch/legacy.rs::execute` exactly (same opcode
//! ranges, same prefix-independent behaviour). Handlers that take an extra
//! argument in `execute` (`opcode`, condition code, or a literal segment index)
//! are wrapped in thin shims that recover that argument from `InsnContext`
//! (`ctx.opcode` is set by `step()` before dispatch).
//!
//! Correctness note: the resolved pointer must be a pure function of the same
//! inputs the cache key already covers (opcode + prefixes + mode). Multi-byte
//! escape opcodes (0x0F, VEX, EVEX, x87) resolve to their top-level dispatcher
//! (`execute_0f`, `execute_vex2/3`, escape_dX), which re-reads the trailing
//! bytes from `ctx` every call exactly as the match did — so they remain correct
//! even though several distinct instructions share one cache slot's handler.

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::cpu::{HandlerFn, InsnContext, X86_64Vcpu};
use super::super::insn;

// ===========================================================================
// Shims: recover the opcode-/cc-/sreg-derived argument from the context.
// These mirror the `self.handler(ctx, arg)` arms in `execute`.
// ===========================================================================

fn sh_jcc_rel8(v: &mut X86_64Vcpu, c: &mut InsnContext) -> Result<Option<VcpuExit>> {
    insn::control::jcc_rel8(v, c, c.opcode & 0x0F)
}
fn sh_mov_r8_imm8(v: &mut X86_64Vcpu, c: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op = c.opcode;
    insn::data::mov_r8_imm8(v, c, op)
}
fn sh_mov_r_imm(v: &mut X86_64Vcpu, c: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op = c.opcode;
    insn::data::mov_r_imm(v, c, op)
}
fn sh_push_r64(v: &mut X86_64Vcpu, c: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op = c.opcode;
    insn::data::push_r64(v, c, op)
}
fn sh_pop_r64(v: &mut X86_64Vcpu, c: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op = c.opcode;
    insn::data::pop_r64(v, c, op)
}
fn sh_xchg_rax_r(v: &mut X86_64Vcpu, c: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op = c.opcode;
    insn::data::xchg_rax_r(v, c, op)
}
// PUSH/POP segment register: literal segment index per opcode (matches `execute`).
fn sh_push_es(v: &mut X86_64Vcpu, c: &mut InsnContext) -> Result<Option<VcpuExit>> {
    insn::data::push_sreg(v, c, 0)
}
fn sh_push_cs(v: &mut X86_64Vcpu, c: &mut InsnContext) -> Result<Option<VcpuExit>> {
    insn::data::push_sreg(v, c, 1)
}
fn sh_push_ss(v: &mut X86_64Vcpu, c: &mut InsnContext) -> Result<Option<VcpuExit>> {
    insn::data::push_sreg(v, c, 2)
}
fn sh_push_ds(v: &mut X86_64Vcpu, c: &mut InsnContext) -> Result<Option<VcpuExit>> {
    insn::data::push_sreg(v, c, 3)
}
fn sh_pop_es(v: &mut X86_64Vcpu, c: &mut InsnContext) -> Result<Option<VcpuExit>> {
    insn::data::pop_sreg(v, c, 0)
}
fn sh_pop_ss(v: &mut X86_64Vcpu, c: &mut InsnContext) -> Result<Option<VcpuExit>> {
    insn::data::pop_sreg(v, c, 2)
}
fn sh_pop_ds(v: &mut X86_64Vcpu, c: &mut InsnContext) -> Result<Option<VcpuExit>> {
    insn::data::pop_sreg(v, c, 3)
}

// Opcodes whose `execute` arm is a small inline body rather than an `insn::`
// call get their own shims so the fast path mirrors them exactly.
fn sh_nop_or_pause(v: &mut X86_64Vcpu, c: &mut InsnContext) -> Result<Option<VcpuExit>> {
    // 0x90: PAUSE under F3, otherwise NOP.
    if c.rep_prefix == Some(0xF3) {
        insn::system::pause(v, c)
    } else {
        v.regs.rip += c.cursor as u64;
        Ok(None)
    }
}
fn sh_hlt(v: &mut X86_64Vcpu, c: &mut InsnContext) -> Result<Option<VcpuExit>> {
    v.regs.rip += c.cursor as u64;
    v.halted = true;
    Ok(Some(VcpuExit::Hlt))
}
fn sh_fwait(v: &mut X86_64Vcpu, c: &mut InsnContext) -> Result<Option<VcpuExit>> {
    // 0x9B FWAIT/WAIT - NOP in this emulator.
    v.regs.rip += c.cursor as u64;
    Ok(None)
}

// Multi-byte escape dispatchers (re-read trailing bytes from ctx each call).
fn sh_0f(v: &mut X86_64Vcpu, c: &mut InsnContext) -> Result<Option<VcpuExit>> {
    v.execute_0f(c)
}
fn sh_vex3(v: &mut X86_64Vcpu, c: &mut InsnContext) -> Result<Option<VcpuExit>> {
    v.execute_vex3(c)
}
fn sh_vex2(v: &mut X86_64Vcpu, c: &mut InsnContext) -> Result<Option<VcpuExit>> {
    v.execute_vex2(c)
}
fn sh_mov_rax_moffs_or_jmp_abs(
    v: &mut X86_64Vcpu,
    c: &mut InsnContext,
) -> Result<Option<VcpuExit>> {
    if c.has_rex2() {
        insn::control::jmp_abs(v, c)
    } else {
        insn::data::mov_rax_moffs(v, c)
    }
}

impl X86_64Vcpu {
    /// Resolve a single-byte opcode to its uniform-signature handler.
    ///
    /// MUST stay in lockstep with `dispatch/legacy.rs::execute`. Returns `None`
    /// for opcodes `execute` would treat as unimplemented (the `_ =>` arm); in
    /// that case the fill path stores a fallback that simply re-enters `execute`
    /// (which produces the proper error), so behaviour is identical.
    pub(in crate::backend::emulator::x86_64) fn resolve_handler(opcode: u8) -> Option<HandlerFn> {
        let f: HandlerFn = match opcode {
            0x90 => sh_nop_or_pause,
            0xF4 => sh_hlt,
            0x0F => sh_0f,

            // Control flow
            0xEB => insn::control::jmp_rel8,
            0xE9 => insn::control::jmp_rel32,
            0xEA => insn::control::jmp_far_ptr,
            0xE8 => insn::control::call_rel32,
            0x9A => insn::control::call_far_ptr,
            0xC3 => insn::control::ret,
            0xC2 => insn::control::ret_imm16,
            0xCA => insn::control::retf_imm16,
            0xCB => insn::control::retf,
            0xCF => insn::control::iret,
            0x70..=0x7F => sh_jcc_rel8,

            // VEX
            0xC4 => sh_vex3,
            0xC5 => sh_vex2,

            // I/O
            0xE4 => insn::io::in_al_imm8,
            0xE5 => insn::io::in_ax_imm8,
            0xEC => insn::io::in_al_dx,
            0xED => insn::io::in_ax_dx,
            0xE6 => insn::io::out_imm8_al,
            0xE7 => insn::io::out_imm8_ax,
            0xEE => insn::io::out_dx_al,
            0xEF => insn::io::out_dx_ax,

            // String I/O
            0x6C => insn::io::insb,
            0x6D => insn::io::insw,
            0x6E => insn::io::outsb,
            0x6F => insn::io::outsw,

            // Data movement
            0xB0..=0xB7 => sh_mov_r8_imm8,
            0xB8..=0xBF => sh_mov_r_imm,
            0x88 => insn::data::mov_rm8_r8,
            0x89 => insn::data::mov_rm_r,
            0x8A => insn::data::mov_r8_rm8,
            0x8B => insn::data::mov_r_rm,
            0x8C => insn::data::mov_rm_sreg,
            0x8E => insn::data::mov_sreg_rm,
            0x8D => insn::data::lea,
            0x06 => sh_push_es,
            0x0E => sh_push_cs,
            0x16 => sh_push_ss,
            0x1E => sh_push_ds,
            0x07 => sh_pop_es,
            0x17 => sh_pop_ss,
            0x1F => sh_pop_ds,
            0xA0 => insn::data::mov_al_moffs,
            0xA1 => sh_mov_rax_moffs_or_jmp_abs,
            0xA2 => insn::data::mov_moffs_al,
            0xA3 => insn::data::mov_moffs_rax,
            0xC6 => insn::data::mov_rm8_imm8,
            0xC7 => insn::data::mov_rm_imm,
            0x50..=0x57 => sh_push_r64,
            0x58..=0x5F => sh_pop_r64,
            0x8F => insn::data::pop_rm,
            0x6A => insn::data::push_imm8,
            0x68 => insn::data::push_imm32,
            0x86 => insn::data::xchg_r8_rm8,
            0x87 => insn::data::xchg_r_rm,
            0x91..=0x97 => sh_xchg_rax_r,
            0x63 => insn::data::movsxd,

            // Arithmetic
            0x00 => insn::arith::add_rm8_r8,
            0x01 => insn::arith::add_rm_r,
            0x02 => insn::arith::add_r8_rm8,
            0x03 => insn::arith::add_r_rm,
            0x04 => insn::arith::add_al_imm8,
            0x05 => insn::arith::add_rax_imm,
            0x10 => insn::arith::adc_rm8_r8,
            0x11 => insn::arith::adc_rm_r,
            0x12 => insn::arith::adc_r8_rm8,
            0x13 => insn::arith::adc_r_rm,
            0x14 => insn::arith::adc_al_imm8,
            0x15 => insn::arith::adc_rax_imm,
            0x18 => insn::arith::sbb_rm8_r8,
            0x19 => insn::arith::sbb_rm_r,
            0x1A => insn::arith::sbb_r8_rm8,
            0x1B => insn::arith::sbb_r_rm,
            0x1C => insn::arith::sbb_al_imm8,
            0x1D => insn::arith::sbb_rax_imm,
            0x27 => insn::arith::daa,
            0x28 => insn::arith::sub_rm8_r8,
            0x29 => insn::arith::sub_rm_r,
            0x2A => insn::arith::sub_r8_rm8,
            0x2B => insn::arith::sub_r_rm,
            0x2C => insn::arith::sub_al_imm8,
            0x2D => insn::arith::sub_rax_imm,
            0x2F => insn::arith::das,
            0x38 => insn::arith::cmp_rm8_r8,
            0x39 => insn::arith::cmp_rm_r,
            0x3A => insn::arith::cmp_r8_rm8,
            0x3B => insn::arith::cmp_r_rm,
            0x3C => insn::arith::cmp_al_imm8,
            0x3D => insn::arith::cmp_rax_imm,
            0x3F => insn::arith::aas,
            0x80 | 0x82 => insn::arith::group1_rm8_imm8,
            0x81 => insn::arith::group1_rm_imm32,
            0x83 => insn::arith::group1_rm_imm8,
            0x69 => insn::arith::imul_r_rm_imm,
            0x6B => insn::arith::imul_r_rm_imm8,
            0x98 => insn::arith::cbw_cwde_cdqe,
            0x99 => insn::arith::cwd_cdq_cqo,

            // Logic
            0x08 => insn::logic::or_rm8_r8,
            0x09 => insn::logic::or_rm_r,
            0x0A => insn::logic::or_r8_rm8,
            0x0B => insn::logic::or_r_rm,
            0x0C => insn::logic::or_al_imm8,
            0x0D => insn::logic::or_rax_imm,
            0x20 => insn::logic::and_rm8_r8,
            0x21 => insn::logic::and_rm_r,
            0x22 => insn::logic::and_r8_rm8,
            0x23 => insn::logic::and_r_rm,
            0x24 => insn::logic::and_al_imm8,
            0x25 => insn::logic::and_rax_imm,
            0x30 => insn::logic::xor_rm8_r8,
            0x31 => insn::logic::xor_rm_r,
            0x32 => insn::logic::xor_r8_rm8,
            0x33 => insn::logic::xor_r_rm,
            0x34 => insn::logic::xor_al_imm8,
            0x35 => insn::logic::xor_rax_imm,
            0x37 => insn::arith::aaa,
            0x84 => insn::logic::test_rm8_r8,
            0x85 => insn::logic::test_rm_r,
            0xA8 => insn::logic::test_al_imm8,
            0xA9 => insn::logic::test_rax_imm,
            0xF6 => insn::logic::group3_rm8,
            0xF7 => insn::logic::group3_rm,

            // Shifts/Rotates
            0xC0 => insn::shift::group2_rm8_imm8,
            0xC1 => insn::shift::group2_rm_imm8,
            0xD0 => insn::shift::group2_rm8_1,
            0xD1 => insn::shift::group2_rm_1,
            0xD2 => insn::shift::group2_rm8_cl,
            0xD3 => insn::shift::group2_rm_cl,

            // BCD Adjust
            0xD4 => insn::arith::aam,
            0xD5 => insn::arith::aad,

            // System/Flags
            0xFA => insn::system::cli,
            0xFB => insn::system::sti,
            0xF8 => insn::system::clc,
            0xF9 => insn::system::stc,
            0xF5 => insn::system::cmc,
            0xFC => insn::system::cld,
            0xFD => insn::system::std,
            0x9C => insn::system::pushf,
            0x9D => insn::system::popf,
            0x9E => insn::system::sahf,
            0x9F => insn::system::lahf,

            // Loop instructions
            0xE0 => insn::control::loopnz,
            0xE1 => insn::control::loopz,
            0xE2 => insn::control::loop_rel8,
            0xE3 => insn::control::jrcxz,

            // Interrupts
            0xCC => insn::control::int3,
            0xCD => insn::control::int_imm8,
            0xCE => insn::control::into,

            // Misc
            0x60 => insn::data::pusha,
            0x61 => insn::data::popa,
            0x62 => insn::data::bound_or_evex,
            0xC8 => insn::data::enter,
            0xC9 => insn::data::leave,
            0xD7 => insn::control::xlat,
            0xFE => insn::control::group4,
            0xFF => insn::control::group5,

            // String operations
            0xA4 => insn::string::movsb,
            0xA5 => insn::string::movs,
            0xAA => insn::string::stosb,
            0xAB => insn::string::stos,
            0xAC => insn::string::lodsb,
            0xAD => insn::string::lods,
            0xAE => insn::string::scasb,
            0xAF => insn::string::scas,
            0xA6 => insn::string::cmpsb,
            0xA7 => insn::string::cmps,

            // FWAIT/WAIT
            0x9B => sh_fwait,

            // x87 FPU escape opcodes
            0xD8 => insn::fpu::escape_d8,
            0xD9 => insn::fpu::escape_d9,
            0xDA => insn::fpu::escape_da,
            0xDB => insn::fpu::escape_db,
            0xDC => insn::fpu::escape_dc,
            0xDD => insn::fpu::escape_dd,
            0xDE => insn::fpu::escape_de,
            0xDF => insn::fpu::escape_df,

            // Unimplemented in `execute` (the `_ =>` arm). Fall back to the match.
            _ => return None,
        };
        Some(f)
    }
}
