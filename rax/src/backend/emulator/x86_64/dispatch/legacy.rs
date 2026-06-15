//! Single-byte opcode dispatch for the x86_64 CPU emulator.

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::cpu::{InsnContext, X86_64Vcpu};
use super::super::insn;

impl X86_64Vcpu {
    /// Main instruction dispatch for single-byte opcodes.
    pub(in crate::backend::emulator::x86_64) fn execute(
        &mut self,
        opcode: u8,
        ctx: &mut InsnContext,
    ) -> Result<Option<VcpuExit>> {
        match opcode {
            // INC r16/r32 (0x40-0x47) / DEC r16/r32 (0x48-0x4F). These encodings
            // exist only in 16/32-bit mode; in 64-bit mode the same bytes are REX
            // prefixes (consumed by the decoder). INC/DEC preserve CF.
            0x40..=0x4F => {
                let reg = (opcode & 0x07) | ctx.rex_b();
                let is_dec = opcode >= 0x48;
                let op_size = ctx.op_size;
                let a = self.get_reg(reg, op_size);
                let result = if is_dec {
                    a.wrapping_sub(1)
                } else {
                    a.wrapping_add(1)
                };
                self.set_reg(reg, result, op_size);
                if is_dec {
                    self.set_lazy_dec(a, result, op_size);
                } else {
                    self.set_lazy_inc(a, result, op_size);
                }
                self.regs.rip += ctx.cursor as u64;
                Ok(None)
            }
            // NOP / PAUSE (F3 90)
            0x90 => {
                if ctx.rep_prefix == Some(0xF3) {
                    insn::system::pause(self, ctx)
                } else {
                    self.regs.rip += ctx.cursor as u64;
                    Ok(None)
                }
            }

            // HLT - halt and exit to caller.
            // NOTE: architecturally HLT is privileged (#GP at CPL!=0), but the rax
            // test harness uses HLT as a universal terminator from any CPL (incl.
            // ring 3 after SYSEXIT/SYSRET), so it is intentionally NOT gated here.
            0xF4 => {
                self.regs.rip += ctx.cursor as u64;
                self.halted = true;
                Ok(Some(VcpuExit::Hlt))
            }

            // Two-byte opcode (0x0F prefix)
            0x0F => self.execute_0f(ctx),

            // Control flow
            0xEB => insn::control::jmp_rel8(self, ctx),
            0xE9 => insn::control::jmp_rel32(self, ctx),
            0xEA => insn::control::jmp_far_ptr(self, ctx),
            0xE8 => insn::control::call_rel32(self, ctx),
            0x9A => insn::control::call_far_ptr(self, ctx),
            0xC3 => insn::control::ret(self, ctx),
            0xC2 => insn::control::ret_imm16(self, ctx),
            0xCA => insn::control::retf_imm16(self, ctx),
            0xCB => insn::control::retf(self, ctx),
            0xCF => insn::control::iret(self, ctx),
            0x70..=0x7F => insn::control::jcc_rel8(self, ctx, opcode & 0x0F),

            // VEX-encoded instructions (partial support)
            0xC4 => self.execute_vex3(ctx),
            0xC5 => self.execute_vex2(ctx),

            // I/O
            0xE4 => insn::io::in_al_imm8(self, ctx),
            0xE5 => insn::io::in_ax_imm8(self, ctx),
            0xEC => insn::io::in_al_dx(self, ctx),
            0xED => insn::io::in_ax_dx(self, ctx),
            0xE6 => insn::io::out_imm8_al(self, ctx),
            0xE7 => insn::io::out_imm8_ax(self, ctx),
            0xEE => insn::io::out_dx_al(self, ctx),
            0xEF => insn::io::out_dx_ax(self, ctx),

            // String I/O
            0x6C => insn::io::insb(self, ctx),
            0x6D => insn::io::insw(self, ctx),
            0x6E => insn::io::outsb(self, ctx),
            0x6F => insn::io::outsw(self, ctx),

            // Data movement
            0xB0..=0xB7 => insn::data::mov_r8_imm8(self, ctx, opcode),
            0xB8..=0xBF => insn::data::mov_r_imm(self, ctx, opcode),
            0x88 => insn::data::mov_rm8_r8(self, ctx),
            0x89 => insn::data::mov_rm_r(self, ctx),
            0x8A => insn::data::mov_r8_rm8(self, ctx),
            0x8B => insn::data::mov_r_rm(self, ctx),
            0x8C => insn::data::mov_rm_sreg(self, ctx),
            0x8E => insn::data::mov_sreg_rm(self, ctx),
            0x8D => insn::data::lea(self, ctx),
            0x06 => insn::data::push_sreg(self, ctx, 0), // PUSH ES
            0x0E => insn::data::push_sreg(self, ctx, 1), // PUSH CS
            0x16 => insn::data::push_sreg(self, ctx, 2), // PUSH SS
            0x1E => insn::data::push_sreg(self, ctx, 3), // PUSH DS
            0x07 => insn::data::pop_sreg(self, ctx, 0),  // POP ES
            0x17 => insn::data::pop_sreg(self, ctx, 2),  // POP SS
            0x1F => insn::data::pop_sreg(self, ctx, 3),  // POP DS
            // MOV moffs instructions
            0xA0 => insn::data::mov_al_moffs(self, ctx),
            0xA1 if ctx.has_rex2() => insn::control::jmp_abs(self, ctx),
            0xA1 => insn::data::mov_rax_moffs(self, ctx),
            0xA2 => insn::data::mov_moffs_al(self, ctx),
            0xA3 => insn::data::mov_moffs_rax(self, ctx),
            0xC6 => insn::data::mov_rm8_imm8(self, ctx),
            0xC7 => insn::data::mov_rm_imm(self, ctx),
            0x50..=0x57 => insn::data::push_r64(self, ctx, opcode),
            0x58..=0x5F => insn::data::pop_r64(self, ctx, opcode),
            0x8F => insn::data::pop_rm(self, ctx),
            0x6A => insn::data::push_imm8(self, ctx),
            0x68 => insn::data::push_imm32(self, ctx),
            0x86 => insn::data::xchg_r8_rm8(self, ctx),
            0x87 => insn::data::xchg_r_rm(self, ctx),
            0x91..=0x97 => insn::data::xchg_rax_r(self, ctx, opcode),
            0x63 => insn::data::movsxd(self, ctx),

            // Arithmetic
            0x00 => insn::arith::add_rm8_r8(self, ctx),
            0x01 => insn::arith::add_rm_r(self, ctx),
            0x02 => insn::arith::add_r8_rm8(self, ctx),
            0x03 => insn::arith::add_r_rm(self, ctx),
            0x04 => insn::arith::add_al_imm8(self, ctx),
            0x05 => insn::arith::add_rax_imm(self, ctx),
            0x10 => insn::arith::adc_rm8_r8(self, ctx),
            0x11 => insn::arith::adc_rm_r(self, ctx),
            0x12 => insn::arith::adc_r8_rm8(self, ctx),
            0x13 => insn::arith::adc_r_rm(self, ctx),
            0x14 => insn::arith::adc_al_imm8(self, ctx),
            0x15 => insn::arith::adc_rax_imm(self, ctx),
            0x18 => insn::arith::sbb_rm8_r8(self, ctx),
            0x19 => insn::arith::sbb_rm_r(self, ctx),
            0x1A => insn::arith::sbb_r8_rm8(self, ctx),
            0x1B => insn::arith::sbb_r_rm(self, ctx),
            0x1C => insn::arith::sbb_al_imm8(self, ctx),
            0x1D => insn::arith::sbb_rax_imm(self, ctx),
            0x27 => insn::arith::daa(self, ctx),
            0x28 => insn::arith::sub_rm8_r8(self, ctx),
            0x29 => insn::arith::sub_rm_r(self, ctx),
            0x2A => insn::arith::sub_r8_rm8(self, ctx),
            0x2B => insn::arith::sub_r_rm(self, ctx),
            0x2C => insn::arith::sub_al_imm8(self, ctx),
            0x2D => insn::arith::sub_rax_imm(self, ctx),
            0x2F => insn::arith::das(self, ctx),
            0x38 => insn::arith::cmp_rm8_r8(self, ctx),
            0x39 => insn::arith::cmp_rm_r(self, ctx),
            0x3A => insn::arith::cmp_r8_rm8(self, ctx),
            0x3B => insn::arith::cmp_r_rm(self, ctx),
            0x3C => insn::arith::cmp_al_imm8(self, ctx),
            0x3D => insn::arith::cmp_rax_imm(self, ctx),
            0x3F => insn::arith::aas(self, ctx),
            0x80 | 0x82 => insn::arith::group1_rm8_imm8(self, ctx), // 0x82 is alias for 0x80
            0x81 => insn::arith::group1_rm_imm32(self, ctx),
            0x83 => insn::arith::group1_rm_imm8(self, ctx),
            0x69 => insn::arith::imul_r_rm_imm(self, ctx),
            0x6B => insn::arith::imul_r_rm_imm8(self, ctx),
            0x98 => insn::arith::cbw_cwde_cdqe(self, ctx),
            0x99 => insn::arith::cwd_cdq_cqo(self, ctx),

            // Logic
            0x08 => insn::logic::or_rm8_r8(self, ctx),
            0x09 => insn::logic::or_rm_r(self, ctx),
            0x0A => insn::logic::or_r8_rm8(self, ctx),
            0x0B => insn::logic::or_r_rm(self, ctx),
            0x0C => insn::logic::or_al_imm8(self, ctx),
            0x0D => insn::logic::or_rax_imm(self, ctx),
            0x20 => insn::logic::and_rm8_r8(self, ctx),
            0x21 => insn::logic::and_rm_r(self, ctx),
            0x22 => insn::logic::and_r8_rm8(self, ctx),
            0x23 => insn::logic::and_r_rm(self, ctx),
            0x24 => insn::logic::and_al_imm8(self, ctx),
            0x25 => insn::logic::and_rax_imm(self, ctx),
            0x30 => insn::logic::xor_rm8_r8(self, ctx),
            0x31 => insn::logic::xor_rm_r(self, ctx),
            0x32 => insn::logic::xor_r8_rm8(self, ctx),
            0x33 => insn::logic::xor_r_rm(self, ctx),
            0x34 => insn::logic::xor_al_imm8(self, ctx),
            0x35 => insn::logic::xor_rax_imm(self, ctx),
            0x37 => insn::arith::aaa(self, ctx),
            0x84 => insn::logic::test_rm8_r8(self, ctx),
            0x85 => insn::logic::test_rm_r(self, ctx),
            0xA8 => insn::logic::test_al_imm8(self, ctx),
            0xA9 => insn::logic::test_rax_imm(self, ctx),
            0xF6 => insn::logic::group3_rm8(self, ctx),
            0xF7 => insn::logic::group3_rm(self, ctx),

            // Shifts/Rotates
            0xC0 => insn::shift::group2_rm8_imm8(self, ctx),
            0xC1 => insn::shift::group2_rm_imm8(self, ctx),
            0xD0 => insn::shift::group2_rm8_1(self, ctx),
            0xD1 => insn::shift::group2_rm_1(self, ctx),
            0xD2 => insn::shift::group2_rm8_cl(self, ctx),
            0xD3 => insn::shift::group2_rm_cl(self, ctx),

            // BCD Adjust
            0xD4 => insn::arith::aam(self, ctx),
            0xD5 => insn::arith::aad(self, ctx),

            // System/Flags
            0xFA => insn::system::cli(self, ctx),
            0xFB => insn::system::sti(self, ctx),
            0xF8 => insn::system::clc(self, ctx),
            0xF9 => insn::system::stc(self, ctx),
            0xF5 => insn::system::cmc(self, ctx),
            0xFC => insn::system::cld(self, ctx),
            0xFD => insn::system::std(self, ctx),
            0x9C => insn::system::pushf(self, ctx),
            0x9D => insn::system::popf(self, ctx),
            0x9E => insn::system::sahf(self, ctx),
            0x9F => insn::system::lahf(self, ctx),

            // Loop instructions
            0xE0 => insn::control::loopnz(self, ctx),
            0xE1 => insn::control::loopz(self, ctx),
            0xE2 => insn::control::loop_rel8(self, ctx),
            0xE3 => insn::control::jrcxz(self, ctx),

            // Interrupts
            0xCC => insn::control::int3(self, ctx),
            0xCD => insn::control::int_imm8(self, ctx),
            0xCE => insn::control::into(self, ctx),

            // Misc
            0x60 => insn::data::pusha(self, ctx),
            0x61 => insn::data::popa(self, ctx),
            0x62 => insn::data::bound_or_evex(self, ctx),
            0xC8 => insn::data::enter(self, ctx),
            0xC9 => insn::data::leave(self, ctx),
            0xD7 => insn::control::xlat(self, ctx),
            0xFE => insn::control::group4(self, ctx),
            0xFF => insn::control::group5(self, ctx),

            // String operations (handled with REP prefix check)
            0xA4 => insn::string::movsb(self, ctx),
            0xA5 => insn::string::movs(self, ctx),
            0xAA => insn::string::stosb(self, ctx),
            0xAB => insn::string::stos(self, ctx),
            0xAC => insn::string::lodsb(self, ctx),
            0xAD => insn::string::lods(self, ctx),
            0xAE => insn::string::scasb(self, ctx),
            0xAF => insn::string::scas(self, ctx),
            0xA6 => insn::string::cmpsb(self, ctx),
            0xA7 => insn::string::cmps(self, ctx),

            // FWAIT/WAIT - check for pending FPU exceptions (NOP in emulator)
            0x9B => {
                self.regs.rip += ctx.cursor as u64;
                Ok(None)
            }

            // x87 FPU escape opcodes
            0xD8 => insn::fpu::escape_d8(self, ctx),
            0xD9 => insn::fpu::escape_d9(self, ctx),
            0xDA => insn::fpu::escape_da(self, ctx),
            0xDB => insn::fpu::escape_db(self, ctx),
            0xDC => insn::fpu::escape_dc(self, ctx),
            0xDD => insn::fpu::escape_dd(self, ctx),
            0xDE => insn::fpu::escape_de(self, ctx),
            0xDF => insn::fpu::escape_df(self, ctx),

            _ => self.inject_undefined_instruction(),
        }
    }
}
