use crate::common::*;
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// REP STOS/STOSB/STOSW/STOSD/STOSQ - Repeat Store String
//
// Opcodes:
//   F3 AA        - REP STOSB (repeat store AL)
//   F3 66 AB     - REP STOSW (repeat store AX)
//   F3 AB        - REP STOSD (repeat store EAX)
//   F3 REX.W AB  - REP STOSQ (repeat store RAX)
//
// Operation:
// WHILE RCX != 0 DO
//   [RDI] := AL/AX/EAX/RAX
//   IF DF = 0 THEN RDI += size
//   ELSE RDI -= size
//   RCX -= 1
// END
//
// Based on: /Users/int/dev/rax/docs/rep:repe:repz:repne:repnz.txt

// ============================================================================
// REP STOSB - Repeat Store Byte
// ============================================================================

#[test]
fn test_rep_stosb_basic() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8
        0xb0, 0xAA, // MOV AL, 0xAA
        0xfc, // CLD
        0xf3, 0xaa, // REP STOSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Verify memory filled with 0xAA
    for i in 0..8 {
        assert_eq!(read_mem_at_u8(&mem, 0x4000 + i), 0xAA);
    }

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x4008);
}

#[test]
fn test_rep_stosb_zero_count() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xb0, 0xFF, // MOV AL, 0xFF
        0xfc, // CLD
        0xf3, 0xaa, // REP STOSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // No memory should be modified
    assert_eq!(read_mem_at_u8(&mem, 0x4000), 0);

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x4000);
}

#[test]
fn test_rep_stosb_single() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xb0, 0x42, // MOV AL, 0x42
        0xfc, // CLD
        0xf3, 0xaa, // REP STOSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u8(&mem, 0x4000), 0x42);
    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x4001);
}

#[test]
fn test_rep_stosb_backward() {
    let code = [
        0x48, 0xc7, 0xc7, 0x07, 0x40, 0x00, 0x00, // MOV RDI, 0x4007 (end)
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8
        0xb0, 0x55, // MOV AL, 0x55
        0xfd, // STD
        0xf3, 0xaa, // REP STOSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..8 {
        assert_eq!(read_mem_at_u8(&mem, 0x4000 + i), 0x55);
    }

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x3FFF, "RDI decrements by 8");
}

#[test]
fn test_rep_stosb_large() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x00, 0x01, 0x00, 0x00, // MOV RCX, 256
        0xb0, 0x99, // MOV AL, 0x99
        0xfc, // CLD
        0xf3, 0xaa, // REP STOSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..256 {
        assert_eq!(read_mem_at_u8(&mem, 0x4000 + i), 0x99);
    }

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x4100);
}

#[test]
fn test_rep_stosb_zero_fill() {
    // Common use case: zero-fill memory
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x10, 0x00, 0x00, 0x00, // MOV RCX, 16
        0xb0, 0x00, // MOV AL, 0
        0xfc, // CLD
        0xf3, 0xaa, // REP STOSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Pre-fill with garbage
    for i in 0..16 {
        write_mem_at_u8(&mem, 0x4000 + i, 0xFF);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..16 {
        assert_eq!(read_mem_at_u8(&mem, 0x4000 + i), 0);
    }

    assert_eq!(regs.rcx, 0);
}

#[test]
fn test_rep_stosb_all_ones() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x20, 0x00, 0x00, 0x00, // MOV RCX, 32
        0xb0, 0xFF, // MOV AL, 0xFF
        0xfc, // CLD
        0xf3, 0xaa, // REP STOSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();

    for i in 0..32 {
        assert_eq!(read_mem_at_u8(&mem, 0x4000 + i), 0xFF);
    }
}

#[test]
fn test_rep_stosb_preserves_other_regs() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0x48, 0xc7, 0xc3, 0x99, 0x00, 0x00, 0x00, // MOV RBX, 0x99
        0x48, 0xc7, 0xc6, 0x42, 0x00, 0x00, 0x00, // MOV RSI, 0x42
        0xb0, 0xAA, // MOV AL, 0xAA
        0xfc, // CLD
        0xf3, 0xaa, // REP STOSB
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x99);
    assert_eq!(regs.rsi, 0x42);
}

#[test]
fn test_rep_stosb_pattern_boundary() {
    let code = [
        0x48, 0xc7, 0xc7, 0xF0, 0x0F, 0x00, 0x00, // MOV RDI, 0x0FF0 (near boundary)
        0x48, 0xc7, 0xc1, 0x20, 0x00, 0x00, 0x00, // MOV RCX, 32 (crosses page)
        0xb0, 0x77, // MOV AL, 0x77
        0xfc, // CLD
        0xf3, 0xaa, // REP STOSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();

    for i in 0..32 {
        assert_eq!(read_mem_at_u8(&mem, 0x0FF0 + i), 0x77);
    }
}

// ============================================================================
// REP STOSW - Repeat Store Word
// ============================================================================

#[test]
fn test_rep_stosw_basic() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0x66, 0xb8, 0x34, 0x12, // MOV AX, 0x1234
        0xfc, // CLD
        0xf3, 0x66, 0xab, // REP STOSW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..4 {
        assert_eq!(read_mem_at_u16(&mem, 0x4000 + i * 2), 0x1234);
    }

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x4008);
}

#[test]
fn test_rep_stosw_zero_count() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0x66, 0xb8, 0xFF, 0xFF, // MOV AX, 0xFFFF
        0xfc, // CLD
        0xf3, 0x66, 0xab, // REP STOSW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u16(&mem, 0x4000), 0);
    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x4000);
}

#[test]
fn test_rep_stosw_single() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x66, 0xb8, 0xCD, 0xAB, // MOV AX, 0xABCD
        0xfc, // CLD
        0xf3, 0x66, 0xab, // REP STOSW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u16(&mem, 0x4000), 0xABCD);
    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x4002);
}

#[test]
fn test_rep_stosw_backward() {
    let code = [
        0x48, 0xc7, 0xc7, 0x06, 0x40, 0x00, 0x00, // MOV RDI, 0x4006
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0x66, 0xb8, 0x88, 0x77, // MOV AX, 0x7788
        0xfd, // STD
        0xf3, 0x66, 0xab, // REP STOSW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..4 {
        assert_eq!(read_mem_at_u16(&mem, 0x4000 + i * 2), 0x7788);
    }

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x3FFE);
}

#[test]
fn test_rep_stosw_large() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x80, 0x00, 0x00, 0x00, // MOV RCX, 128
        0x66, 0xb8, 0x11, 0x22, // MOV AX, 0x2211
        0xfc, // CLD
        0xf3, 0x66, 0xab, // REP STOSW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..128 {
        assert_eq!(read_mem_at_u16(&mem, 0x4000 + i * 2), 0x2211);
    }

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x4100);
}

#[test]
fn test_rep_stosw_zero_fill() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x10, 0x00, 0x00, 0x00, // MOV RCX, 16
        0x66, 0xb8, 0x00, 0x00, // MOV AX, 0
        0xfc, // CLD
        0xf3, 0x66, 0xab, // REP STOSW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Pre-fill
    for i in 0..16 {
        write_mem_at_u16(&mem, 0x4000 + i * 2, 0xFFFF);
    }

    run_until_hlt(&mut vcpu).unwrap();

    for i in 0..16 {
        assert_eq!(read_mem_at_u16(&mem, 0x4000 + i * 2), 0);
    }
}

#[test]
fn test_rep_stosw_max_value() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8
        0x66, 0xb8, 0xFF, 0xFF, // MOV AX, 0xFFFF
        0xfc, // CLD
        0xf3, 0x66, 0xab, // REP STOSW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();

    for i in 0..8 {
        assert_eq!(read_mem_at_u16(&mem, 0x4000 + i * 2), 0xFFFF);
    }
}

#[test]
fn test_rep_stosw_unaligned() {
    let code = [
        0x48, 0xc7, 0xc7, 0x01, 0x40, 0x00, 0x00, // MOV RDI, 0x4001 (unaligned)
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0x66, 0xb8, 0xAA, 0xBB, // MOV AX, 0xBBAA
        0xfc, // CLD
        0xf3, 0x66, 0xab, // REP STOSW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..4 {
        assert_eq!(read_mem_at_u16(&mem, 0x4001 + i * 2), 0xBBAA);
    }

    assert_eq!(regs.rdi, 0x4009);
}

// ============================================================================
// REP STOSD - Repeat Store Doubleword
// ============================================================================

#[test]
fn test_rep_stosd_basic() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xb8, 0x78, 0x56, 0x34, 0x12, // MOV EAX, 0x12345678
        0xfc, // CLD
        0xf3, 0xab, // REP STOSD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..4 {
        assert_eq!(read_mem_at_u32(&mem, 0x4000 + i * 4), 0x12345678);
    }

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x4010);
}

#[test]
fn test_rep_stosd_zero_count() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xb8, 0xFF, 0xFF, 0xFF, 0xFF, // MOV EAX, 0xFFFFFFFF
        0xfc, // CLD
        0xf3, 0xab, // REP STOSD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u32(&mem, 0x4000), 0);
    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x4000);
}

#[test]
fn test_rep_stosd_single() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xb8, 0xEF, 0xBE, 0xAD, 0xDE, // MOV EAX, 0xDEADBEEF
        0xfc, // CLD
        0xf3, 0xab, // REP STOSD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u32(&mem, 0x4000), 0xDEADBEEF);
    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x4004);
}

#[test]
fn test_rep_stosd_backward() {
    let code = [
        0x48, 0xc7, 0xc7, 0x0C, 0x40, 0x00, 0x00, // MOV RDI, 0x400C
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xb8, 0x11, 0x22, 0x33, 0x44, // MOV EAX, 0x44332211
        0xfd, // STD
        0xf3, 0xab, // REP STOSD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..4 {
        assert_eq!(read_mem_at_u32(&mem, 0x4000 + i * 4), 0x44332211);
    }

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x3FFC);
}

#[test]
fn test_rep_stosd_large() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x40, 0x00, 0x00, 0x00, // MOV RCX, 64
        0xb8, 0xAA, 0xBB, 0xCC, 0xDD, // MOV EAX, 0xDDCCBBAA
        0xfc, // CLD
        0xf3, 0xab, // REP STOSD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..64 {
        assert_eq!(read_mem_at_u32(&mem, 0x4000 + i * 4), 0xDDCCBBAA);
    }

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x4100);
}

#[test]
fn test_rep_stosd_zero_fill() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x10, 0x00, 0x00, 0x00, // MOV RCX, 16
        0xb8, 0x00, 0x00, 0x00, 0x00, // MOV EAX, 0
        0xfc, // CLD
        0xf3, 0xab, // REP STOSD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..16 {
        write_mem_at_u32(&mem, 0x4000 + i * 4, 0xFFFFFFFF);
    }

    run_until_hlt(&mut vcpu).unwrap();

    for i in 0..16 {
        assert_eq!(read_mem_at_u32(&mem, 0x4000 + i * 4), 0);
    }
}

#[test]
fn test_rep_stosd_buffer_init() {
    // Common pattern: initialize buffer with value
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x50, 0x00, 0x00, // MOV RDI, 0x5000
        0x48, 0xc7, 0xc1, 0x00, 0x01, 0x00, 0x00, // MOV RCX, 256
        0xb8, 0xCC, 0xCC, 0xCC, 0xCC, // MOV EAX, 0xCCCCCCCC
        0xfc, // CLD
        0xf3, 0xab, // REP STOSD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();

    for i in 0..256 {
        assert_eq!(read_mem_at_u32(&mem, 0x5000 + i * 4), 0xCCCCCCCC);
    }
}

#[test]
fn test_rep_stosd_incremental_addresses() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8
        0xb8, 0x00, 0x10, 0x00, 0x00, // MOV EAX, 0x1000
        0xfc, // CLD
        0xf3, 0xab, // REP STOSD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..8 {
        assert_eq!(read_mem_at_u32(&mem, 0x4000 + i * 4), 0x1000);
    }

    assert_eq!(regs.rdi, 0x4020, "8 dwords = 32 bytes");
}

// ============================================================================
// REP STOSQ - Repeat Store Quadword
// ============================================================================

#[test]
fn test_rep_stosq_basic() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0x48, 0xb8, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34,
        0x12, // MOV RAX, 0x1234567890ABCDEF
        0xfc, // CLD
        0xf3, 0x48, 0xab, // REP STOSQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..4 {
        assert_eq!(read_mem_at_u64(&mem, 0x4000 + i * 8), 0x1234567890ABCDEF);
    }

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x4020);
}

#[test]
fn test_rep_stosq_zero_count() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0x48, 0xb8, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1
        0xfc, // CLD
        0xf3, 0x48, 0xab, // REP STOSQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u64(&mem, 0x4000), 0);
    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x4000);
}

#[test]
fn test_rep_stosq_single() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x48, 0xb8, 0xBE, 0xBA, 0xFE, 0xCA, 0xEF, 0xBE, 0xAD,
        0xDE, // MOV RAX, 0xDEADBEEFCAFEBABE
        0xfc, // CLD
        0xf3, 0x48, 0xab, // REP STOSQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u64(&mem, 0x4000), 0xDEADBEEFCAFEBABE);
    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x4008);
}

#[test]
fn test_rep_stosq_backward() {
    let code = [
        0x48, 0xc7, 0xc7, 0x18, 0x40, 0x00, 0x00, // MOV RDI, 0x4018
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0x48, 0xb8, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, // MOV RAX, 0x1111...
        0xfd, // STD
        0xf3, 0x48, 0xab, // REP STOSQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..4 {
        assert_eq!(read_mem_at_u64(&mem, 0x4000 + i * 8), 0x1111111111111111);
    }

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x3FF8);
}

#[test]
fn test_rep_stosq_large() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x20, 0x00, 0x00, 0x00, // MOV RCX, 32
        0x48, 0xb8, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, // MOV RAX, 0x2222...
        0xfc, // CLD
        0xf3, 0x48, 0xab, // REP STOSQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..32 {
        assert_eq!(read_mem_at_u64(&mem, 0x4000 + i * 8), 0x2222222222222222);
    }

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x4100);
}

#[test]
fn test_rep_stosq_zero_fill() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x10, 0x00, 0x00, 0x00, // MOV RCX, 16
        0x48, 0x31, 0xc0, // XOR RAX, RAX (zero)
        0xfc, // CLD
        0xf3, 0x48, 0xab, // REP STOSQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..16 {
        write_mem_at_u64(&mem, 0x4000 + i * 8, 0xFFFFFFFFFFFFFFFF);
    }

    run_until_hlt(&mut vcpu).unwrap();

    for i in 0..16 {
        assert_eq!(read_mem_at_u64(&mem, 0x4000 + i * 8), 0);
    }
}

#[test]
fn test_rep_stosq_pointer_array_init() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8
        0x48, 0xb8, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x1000
        0xfc, // CLD
        0xf3, 0x48, 0xab, // REP STOSQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();

    for i in 0..8 {
        assert_eq!(read_mem_at_u64(&mem, 0x4000 + i * 8), 0x1000);
    }
}

#[test]
fn test_rep_stosq_max_value() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0x48, 0xb8, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1
        0xfc, // CLD
        0xf3, 0x48, 0xab, // REP STOSQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();

    for i in 0..4 {
        assert_eq!(read_mem_at_u64(&mem, 0x4000 + i * 8), 0xFFFFFFFFFFFFFFFF);
    }
}

#[test]
fn test_rep_stosq_preserves_other_regs() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x02, 0x00, 0x00, 0x00, // MOV RCX, 2
        0x48, 0xc7, 0xc3, 0x42, 0x00, 0x00, 0x00, // MOV RBX, 0x42
        0x48, 0xc7, 0xc6, 0x99, 0x00, 0x00, 0x00, // MOV RSI, 0x99
        0x48, 0xb8, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, // MOV RAX, pattern
        0xfc, // CLD
        0xf3, 0x48, 0xab, // REP STOSQ
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x42);
    assert_eq!(regs.rsi, 0x99);
}

// ============================================================================
// Mixed tests and edge cases
// ============================================================================

#[test]
fn test_rep_stos_direction_flag() {
    // Verify DF controls increment/decrement
    let code1 = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xb0, 0xAA, // MOV AL, 0xAA
        0xfc, // CLD (increment)
        0xf3, 0xaa, // REP STOSB
        0xf4, // HLT
    ];
    let (mut vcpu1, _) = setup_vm(&code1, None);
    let regs1 = run_until_hlt(&mut vcpu1).unwrap();

    let code2 = [
        0x48, 0xc7, 0xc7, 0x03, 0x40, 0x00, 0x00, // MOV RDI, 0x4003
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xb0, 0xAA, // MOV AL, 0xAA
        0xfd, // STD (decrement)
        0xf3, 0xaa, // REP STOSB
        0xf4, // HLT
    ];
    let (mut vcpu2, _) = setup_vm(&code2, None);
    let regs2 = run_until_hlt(&mut vcpu2).unwrap();

    assert_eq!(regs1.rdi, 0x4004, "DF=0 increments");
    assert_eq!(regs2.rdi, 0x3FFF, "DF=1 decrements");
}

#[test]
fn test_rep_stos_different_sizes() {
    // Verify size affects RDI increment
    let code1 = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xb0, 0xAA, // MOV AL, 0xAA
        0xfc, // CLD
        0xf3, 0xaa, // REP STOSB
        0xf4, // HLT
    ];
    let (mut vcpu1, _) = setup_vm(&code1, None);
    let regs1 = run_until_hlt(&mut vcpu1).unwrap();

    let code2 = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xb8, 0xAA, 0xAA, 0xAA, 0xAA, // MOV EAX, 0xAAAAAAAA
        0xfc, // CLD
        0xf3, 0xab, // REP STOSD
        0xf4, // HLT
    ];
    let (mut vcpu2, _) = setup_vm(&code2, None);
    let regs2 = run_until_hlt(&mut vcpu2).unwrap();

    assert_eq!(regs1.rdi, 0x4004, "STOSB: 4 bytes");
    assert_eq!(regs2.rdi, 0x4010, "STOSD: 16 bytes (4*4)");
}

#[test]
fn test_rep_stosb_memset_pattern() {
    // Typical memset usage
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x50, 0x00, 0x00, // MOV RDI, 0x5000
        0x48, 0xc7, 0xc1, 0x00, 0x01, 0x00, 0x00, // MOV RCX, 256
        0xb0, 0x00, // MOV AL, 0 (clear)
        0xfc, // CLD
        0xf3, 0xaa, // REP STOSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Pre-fill with garbage
    for i in 0..256 {
        write_mem_at_u8(&mem, 0x5000 + i, 0xFF);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..256 {
        assert_eq!(read_mem_at_u8(&mem, 0x5000 + i), 0);
    }

    assert_eq!(regs.rcx, 0);
}

#[test]
fn test_rep_stosd_fast_clear() {
    // Using STOSD for fast memory clear
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x60, 0x00, 0x00, // MOV RDI, 0x6000
        0x48, 0xc7, 0xc1, 0x00, 0x04, 0x00, 0x00, // MOV RCX, 1024 (4KB)
        0x31, 0xc0, // XOR EAX, EAX
        0xfc, // CLD
        0xf3, 0xab, // REP STOSD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Spot check
    assert_eq!(read_mem_at_u32(&mem, 0x6000), 0);
    assert_eq!(read_mem_at_u32(&mem, 0x6100), 0);
    assert_eq!(read_mem_at_u32(&mem, 0x6FFC), 0);

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x7000);
}

#[test]
fn test_rep_stosq_struct_array_init() {
    // Initialize array of 64-bit values
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x10, 0x00, 0x00, 0x00, // MOV RCX, 16
        0x48, 0xb8, 0xDD, 0xDD, 0xDD, 0xDD, 0xDD, 0xDD, 0xDD, 0xDD, // MOV RAX, debug pattern
        0xfc, // CLD
        0xf3, 0x48, 0xab, // REP STOSQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();

    for i in 0..16 {
        assert_eq!(read_mem_at_u64(&mem, 0x4000 + i * 8), 0xDDDDDDDDDDDDDDDD);
    }
}

#[test]
fn test_rep_stos_rcx_countdown() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x20, 0x00, 0x00, 0x00, // MOV RCX, 32
        0xb0, 0x99, // MOV AL, 0x99
        0xfc, // CLD
        0xf3, 0xaa, // REP STOSB
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0, "RCX must count down to 0");
    assert_eq!(regs.rdi, 0x4020, "RDI advanced by RCX");
}

#[test]
fn test_rep_stosb_pattern_0x55() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x10, 0x00, 0x00, 0x00, // MOV RCX, 16
        0xb0, 0x55, // MOV AL, 0x55
        0xfc, // CLD
        0xf3, 0xaa, // REP STOSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
    for i in 0..16 {
        assert_eq!(read_mem_at_u8(&mem, 0x4000 + i), 0x55);
    }
}

#[test]
fn test_rep_stosw_pattern_0xAAAA() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8
        0x66, 0xb8, 0xAA, 0xAA, // MOV AX, 0xAAAA
        0xfc, // CLD
        0xf3, 0x66, 0xab, // REP STOSW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
    for i in 0..8 {
        assert_eq!(read_mem_at_u16(&mem, 0x4000 + i * 2), 0xAAAA);
    }
}

#[test]
fn test_rep_stosd_pattern_0x12345678() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8
        0xb8, 0x78, 0x56, 0x34, 0x12, // MOV EAX, 0x12345678
        0xfc, // CLD
        0xf3, 0xab, // REP STOSD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
    for i in 0..8 {
        assert_eq!(read_mem_at_u32(&mem, 0x4000 + i * 4), 0x12345678);
    }
}

#[test]
fn test_rep_stosq_sentinel_pattern() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0x48, 0xb8, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1
        0xfc, // CLD
        0xf3, 0x48, 0xab, // REP STOSQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
    for i in 0..4 {
        assert_eq!(read_mem_at_u64(&mem, 0x4000 + i * 8), 0xFFFFFFFFFFFFFFFF);
    }
}

#[test]
fn test_rep_stosb_count_1() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xb0, 0x88, // MOV AL, 0x88
        0xfc, // CLD
        0xf3, 0xaa, // REP STOSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_at_u8(&mem, 0x4000), 0x88);
    assert_eq!(regs.rdi, 0x4001);
}

// ============================================================================
// Bulk page-wise fast path regression tests
//
// These exercise the O(pages) fast path for forward REP STOS with multi-byte
// elements, including a fill that crosses a page boundary, verifying byte-exact
// memory and exact end registers.
// ============================================================================

#[test]
fn test_rep_stosd_multibyte_crosses_page_boundary() {
    // STOSD whose run spans a page boundary with one element straddling it.
    // Dest: 0x4FF8 .. 0x501F (10 dwords = 40 bytes), crossing 0x5000.
    let code = [
        0x48, 0xc7, 0xc7, 0xF8, 0x4F, 0x00, 0x00, // MOV RDI, 0x4FF8
        0x48, 0xc7, 0xc1, 0x0A, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0xc7, 0xc0, 0xEF, 0xBE, 0xAD, 0xDE, // MOV RAX, 0xDEADBEEF
        0xfc, // CLD
        0xf3, 0xab, // REP STOSD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..10u64 {
        assert_eq!(
            read_mem_at_u32(&mem, 0x4FF8 + i * 4),
            0xDEADBEEF,
            "dword {} not filled across page boundary",
            i
        );
    }
    assert_eq!(regs.rcx, 0, "RCX must be exactly 0");
    assert_eq!(regs.rdi, 0x4FF8 + 40, "RDI end value");
}

#[test]
fn test_rep_stosq_multibyte_multi_page_exact_end_regs() {
    // STOSQ filling across multiple pages, verifying byte-exact fill and the
    // exact final RDI/RCX after the bulk fast path.
    // 0x2008 .. 0x2008 + 0x600 * 8 spans several pages.
    let code = [
        0x48, 0xc7, 0xc7, 0x08, 0x20, 0x00, 0x00, // MOV RDI, 0x2008
        0x48, 0xc7, 0xc1, 0x00, 0x06, 0x00, 0x00, // MOV RCX, 0x600
        0x48, 0xb8, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02,
        0x01, // MOVABS RAX, 0x0102030405060708
        0xfc, // CLD
        0xf3, 0x48, 0xab, // REP STOSQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let n = 0x600u64;
    for i in 0..n {
        assert_eq!(
            read_mem_at_u64(&mem, 0x2008 + i * 8),
            0x0102030405060708,
            "qword {} not filled",
            i
        );
    }
    // Byte just past the fill must remain zero (no over-write).
    assert_eq!(
        read_mem_at_u64(&mem, 0x2008 + n * 8),
        0,
        "over-write past fill"
    );
    assert_eq!(regs.rcx, 0, "RCX must be exactly 0");
    assert_eq!(regs.rdi, 0x2008 + n * 8, "RDI end value");
}

#[test]
fn test_rep_stosw_multibyte_basic_exact_end_regs() {
    // STOSW (2-byte element) fully within a page; checks pattern + end regs.
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x09, 0x00, 0x00, 0x00, // MOV RCX, 9
        0x48, 0xc7, 0xc0, 0xCD, 0xAB, 0x00, 0x00, // MOV RAX, 0xABCD
        0xfc, // CLD
        0xf3, 0x66, 0xab, // REP STOSW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..9u64 {
        assert_eq!(read_mem_at_u16(&mem, 0x4000 + i * 2), 0xABCD, "word {}", i);
    }
    assert_eq!(
        read_mem_at_u16(&mem, 0x4000 + 9 * 2),
        0,
        "over-write past fill"
    );
    assert_eq!(regs.rcx, 0, "RCX must be exactly 0");
    assert_eq!(regs.rdi, 0x4000 + 18, "RDI end value");
}
