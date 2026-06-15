//! Sign extension instructions: CBW, CWDE, CDQE, CWD, CDQ, CQO.

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::super::cpu::{InsnContext, X86_64Vcpu};

/// CBW/CWDE/CDQE (0x98) - sign-extend AL to AX, AX to EAX, or EAX to RAX
pub fn cbw_cwde_cdqe(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    match ctx.op_size {
        2 => {
            // CBW: AL -> AX
            let val = vcpu.regs.rax as i8 as i16 as u16;
            vcpu.regs.rax = (vcpu.regs.rax & !0xFFFF) | val as u64;
        }
        4 => {
            // CWDE: AX -> EAX
            let val = vcpu.regs.rax as i16 as i32 as u32;
            vcpu.regs.rax = val as u64;
        }
        8 => {
            // CDQE: EAX -> RAX
            let val = vcpu.regs.rax as i32 as i64 as u64;
            vcpu.regs.rax = val;
        }
        _ => {}
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// CWD/CDQ/CQO (0x99) - sign-extend AX to DX:AX, EAX to EDX:EAX, or RAX to RDX:RAX
pub fn cwd_cdq_cqo(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    match ctx.op_size {
        2 => {
            // CWD: AX -> DX:AX
            let sign = if (vcpu.regs.rax as i16) < 0 {
                0xFFFF
            } else {
                0
            };
            vcpu.regs.rdx = (vcpu.regs.rdx & !0xFFFF) | sign;
        }
        4 => {
            // CDQ: EAX -> EDX:EAX
            let sign = if (vcpu.regs.rax as i32) < 0 {
                0xFFFFFFFF
            } else {
                0
            };
            vcpu.regs.rdx = sign;
        }
        8 => {
            // CQO: RAX -> RDX:RAX
            let sign = if (vcpu.regs.rax as i64) < 0 {
                u64::MAX
            } else {
                0
            };
            vcpu.regs.rdx = sign;
        }
        _ => {}
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
