//! x86_64 register mapping for GDB.
//!
//! GDB x86_64 register order:
//! 0-15: rax, rbx, rcx, rdx, rsi, rdi, rbp, rsp, r8-r15 (64-bit each)
//! 16: rip (64-bit)
//! 17: eflags (32-bit)
//! 18-23: cs, ss, ds, es, fs, gs (32-bit each)
//!
//! Total: 17 * 8 + 4 + 6 * 4 = 136 + 4 + 24 = 164 bytes

use super::protocol::{decode_hex, encode_hex};
use crate::cpu::state::X86_64CpuState;

/// Size of register data in bytes.
pub const REGISTER_DATA_SIZE: usize = 164;

/// Pack x86_64 CPU state into GDB hex-encoded register format.
pub fn pack_registers(state: &X86_64CpuState) -> String {
    let mut data = Vec::with_capacity(REGISTER_DATA_SIZE);
    let regs = &state.regs;
    let sregs = &state.sregs;

    // General purpose registers (64-bit each, little-endian)
    data.extend_from_slice(&regs.rax.to_le_bytes());
    data.extend_from_slice(&regs.rbx.to_le_bytes());
    data.extend_from_slice(&regs.rcx.to_le_bytes());
    data.extend_from_slice(&regs.rdx.to_le_bytes());
    data.extend_from_slice(&regs.rsi.to_le_bytes());
    data.extend_from_slice(&regs.rdi.to_le_bytes());
    data.extend_from_slice(&regs.rbp.to_le_bytes());
    data.extend_from_slice(&regs.rsp.to_le_bytes());
    data.extend_from_slice(&regs.r8.to_le_bytes());
    data.extend_from_slice(&regs.r9.to_le_bytes());
    data.extend_from_slice(&regs.r10.to_le_bytes());
    data.extend_from_slice(&regs.r11.to_le_bytes());
    data.extend_from_slice(&regs.r12.to_le_bytes());
    data.extend_from_slice(&regs.r13.to_le_bytes());
    data.extend_from_slice(&regs.r14.to_le_bytes());
    data.extend_from_slice(&regs.r15.to_le_bytes());

    // RIP (64-bit)
    data.extend_from_slice(&regs.rip.to_le_bytes());

    // EFLAGS (32-bit)
    data.extend_from_slice(&(regs.rflags as u32).to_le_bytes());

    // Segment registers (32-bit each - just the selector extended)
    data.extend_from_slice(&(sregs.cs.selector as u32).to_le_bytes());
    data.extend_from_slice(&(sregs.ss.selector as u32).to_le_bytes());
    data.extend_from_slice(&(sregs.ds.selector as u32).to_le_bytes());
    data.extend_from_slice(&(sregs.es.selector as u32).to_le_bytes());
    data.extend_from_slice(&(sregs.fs.selector as u32).to_le_bytes());
    data.extend_from_slice(&(sregs.gs.selector as u32).to_le_bytes());

    encode_hex(&data)
}

/// Unpack GDB hex-encoded register data into x86_64 CPU state.
pub fn unpack_registers(hex: &str, state: &mut X86_64CpuState) -> bool {
    let data = match decode_hex(hex) {
        Some(d) if d.len() >= REGISTER_DATA_SIZE => d,
        _ => return false,
    };

    let regs = &mut state.regs;
    let sregs = &mut state.sregs;
    let mut offset = 0;

    macro_rules! read_u64 {
        () => {{
            let val = u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());
            offset += 8;
            val
        }};
    }

    macro_rules! read_u32 {
        () => {{
            let val = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
            offset += 4;
            val
        }};
    }

    // General purpose registers
    regs.rax = read_u64!();
    regs.rbx = read_u64!();
    regs.rcx = read_u64!();
    regs.rdx = read_u64!();
    regs.rsi = read_u64!();
    regs.rdi = read_u64!();
    regs.rbp = read_u64!();
    regs.rsp = read_u64!();
    regs.r8 = read_u64!();
    regs.r9 = read_u64!();
    regs.r10 = read_u64!();
    regs.r11 = read_u64!();
    regs.r12 = read_u64!();
    regs.r13 = read_u64!();
    regs.r14 = read_u64!();
    regs.r15 = read_u64!();

    // RIP
    regs.rip = read_u64!();

    // EFLAGS
    regs.rflags = read_u32!() as u64;

    // Segment registers
    sregs.cs.selector = read_u32!() as u16;
    sregs.ss.selector = read_u32!() as u16;
    sregs.ds.selector = read_u32!() as u16;
    sregs.es.selector = read_u32!() as u16;
    sregs.fs.selector = read_u32!() as u16;
    sregs.gs.selector = read_u32!() as u16;

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pack_unpack_roundtrip() {
        let mut state = X86_64CpuState::default();
        state.regs.rax = 0x1234567890abcdef;
        state.regs.rbx = 0xfedcba0987654321;
        state.regs.rip = 0x00007fff12345678;
        state.regs.rflags = 0x202;
        state.sregs.cs.selector = 0x10;
        state.sregs.ss.selector = 0x18;

        let packed = pack_registers(&state);
        let mut unpacked = X86_64CpuState::default();
        assert!(unpack_registers(&packed, &mut unpacked));

        assert_eq!(unpacked.regs.rax, state.regs.rax);
        assert_eq!(unpacked.regs.rbx, state.regs.rbx);
        assert_eq!(unpacked.regs.rip, state.regs.rip);
        assert_eq!(unpacked.regs.rflags, state.regs.rflags);
        assert_eq!(unpacked.sregs.cs.selector, state.sregs.cs.selector);
        assert_eq!(unpacked.sregs.ss.selector, state.sregs.ss.selector);
    }
}
