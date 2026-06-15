//! Interrupt instructions: INT, INT3, INTO.

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::cpu::{InsnContext, X86_64Vcpu};
use super::super::super::flags;
use super::call::validate_far_selector;

fn pop_by_size(vcpu: &mut X86_64Vcpu, size: u8) -> Result<u64> {
    match size {
        2 => Ok(vcpu.pop16()? as u64),
        4 => Ok(vcpu.pop32()? as u64),
        8 => vcpu.pop64(),
        _ => Err(Error::Emulator(format!(
            "invalid IRET stack pop size: {}",
            size
        ))),
    }
}

fn apply_iret_flags(vcpu: &mut X86_64Vcpu, size: u8, value: u64) -> Result<()> {
    match size {
        2 => {
            let mask = 0xFFFFu64;
            vcpu.regs.rflags = (vcpu.regs.rflags & !mask) | (value & mask) | 0x2;
        }
        4 | 8 => {
            let mask = 0x00000000_00257FD5u64;
            vcpu.regs.rflags = (value & mask) | 0x2;
        }
        _ => {
            return Err(Error::Emulator(format!(
                "invalid IRET flags size: {}",
                size
            )));
        }
    }
    // IRET restores RFLAGS wholesale into `regs.rflags`. Any pending lazy-flag op
    // left by the returning handler is now stale and MUST be invalidated, or the
    // resumed code would evaluate conditions from the HANDLER's CF/ZF/... instead
    // of the restored flags. That silently corrupts any flag-dependent loop an
    // interrupt landed in the middle of (e.g. memcpy_orig's `sub;jae`, vsnprintf's
    // digit `dec;jnz`), producing wrong copy lengths / digit counts and, over a
    // boot's worth of timer interrupts, scattered guest-memory corruption.
    vcpu.clear_lazy_flags();
    Ok(())
}

/// INT3 (0xCC) - Debug breakpoint interrupt
/// This instruction invokes exception vector 3 (breakpoint exception)
pub fn int3(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    // INT3 is a 1-byte instruction - RIP should point AFTER the INT3
    // (it's a trap, not a fault, so RIP points to next instruction)
    vcpu.regs.rip += ctx.cursor as u64;
    // Inject #BP exception (vector 3) into the guest via IDT
    vcpu.inject_exception(3, None)?;
    Ok(None)
}

/// INT imm8 (0xCD) - Software interrupt
pub fn int_imm8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let vector = ctx.consume_u8()?;
    vcpu.regs.rip += ctx.cursor as u64;
    // Real-mode BIOS interception: during a legacy boot (CR0.PE=0 with a boot CD
    // installed) the guest IDT is absent — service the well-known BIOS vectors
    // (INT 10h/13h/15h/16h) natively. Unhandled vectors fall through.
    if vcpu.sregs.cr0 & 1 == 0
        && super::super::super::bios::active()
        && super::super::super::bios::service(vcpu, vector)?
    {
        return Ok(None);
    }
    // Inject the software interrupt via IDT
    vcpu.inject_exception(vector, None)?;
    Ok(None)
}

/// INTO (0xCE) - Interrupt on Overflow
/// If OF=1, generates INT 4 (overflow exception)
/// If OF=0, does nothing and continues
/// Invalid in 64-bit mode (generates #UD)
pub fn into(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    // Check if we're in 64-bit mode
    let in_long_mode = (vcpu.sregs.efer & 0x400) != 0; // EFER.LMA = bit 10
    let in_64bit_mode = in_long_mode && vcpu.sregs.cs.l;

    if in_64bit_mode {
        // INTO is INVALID in 64-bit mode - inject #UD (vector 6, no error code)
        // instead of aborting the VM. Don't advance RIP: exception delivery
        // sets RIP to the handler, and the fault should point to this instruction.
        vcpu.inject_exception(6, None)?; // #UD = vector 6
        return Ok(None);
    }

    // Check overflow flag
    if vcpu.regs.rflags & flags::bits::OF != 0 {
        // OF=1: Generate INT 4 (overflow exception)
        vcpu.regs.rip += ctx.cursor as u64;
        vcpu.inject_exception(4, None)?;
        Ok(None)
    } else {
        // OF=0: No interrupt, continue execution
        vcpu.regs.rip += ctx.cursor as u64;
        Ok(None)
    }
}

/// IRET/IRETD/IRETQ (0xCF) - Interrupt return
pub fn iret(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    use super::super::super::cpu::log_if_transition;

    let op_size = ctx.op_size;
    let old_cpl = vcpu.sregs.cs.selector & 0x3;

    // Check if we're in 64-bit mode
    let in_long_mode = (vcpu.sregs.efer & 0x400) != 0; // EFER.LMA = bit 10
    let in_64bit_mode = in_long_mode && vcpu.sregs.cs.l;

    let ret_ip = pop_by_size(vcpu, op_size)?;
    let cs = pop_by_size(vcpu, op_size)? as u16;
    validate_far_selector(vcpu, cs)?;
    let flags = pop_by_size(vcpu, op_size)?;

    let new_cpl = cs & 0x3;
    if new_cpl < old_cpl {
        return Err(Error::Emulator(
            "IRET privilege increase not supported".to_string(),
        ));
    }

    // In 64-bit mode, IRETQ ALWAYS pops RSP and SS, regardless of privilege level change.
    // In 32-bit mode, RSP/SS are only popped on privilege level change.
    let (_new_rsp, _new_ss) = if in_64bit_mode || new_cpl > old_cpl {
        let new_rsp = pop_by_size(vcpu, op_size)?;
        let new_ss = pop_by_size(vcpu, op_size)? as u16;

        vcpu.set_sreg(2, new_ss); // SS is segment register 2
        vcpu.regs.rsp = new_rsp;
        (new_rsp, new_ss)
    } else {
        (vcpu.regs.rsp, vcpu.sregs.ss.selector)
    };

    let if_before = (vcpu.regs.rflags & 0x200) != 0;
    apply_iret_flags(vcpu, op_size, flags)?;
    let if_after = (vcpu.regs.rflags & 0x200) != 0;

    // Log when IRET should restore IF but doesn't
    let saved_if = (flags & 0x200) != 0;
    if saved_if && !if_after {
        eprintln!(
            "[IRET BUG] saved_IF=1 but IF not restored! rflags_before={:#x} flags_popped={:#x} rflags_after={:#x}",
            if if_before { 0x200u64 } else { 0u64 },
            flags,
            vcpu.regs.rflags
        );
    }

    log_if_transition(ret_ip, if_before, if_after, "IRET");
    vcpu.regs.rip = ret_ip;
    vcpu.set_sreg(1, cs); // CS is segment register 1
    Ok(None)
}
