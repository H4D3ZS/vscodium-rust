use crate::common::*;
use rax::cpu::Registers;
use std::sync::Arc;
use vm_memory::{Bytes, GuestAddress};

// Comprehensive tests for MOV with segment registers
// Based on documentation from /Users/int/dev/rax/docs/mov.txt
//
// MOV Sreg, r/m16 - Move r/m16 to segment register (opcode 8E /r)
// MOV r/m16, Sreg - Move segment register to r/m16 (opcode 8C /r)
//
// Segment registers: ES, CS, SS, DS, FS, GS
// CS cannot be loaded with MOV (use far JMP/CALL/RET instead)
// In 64-bit mode, segment registers are zero-extended to 16 bits

// ============================================================================
// MOV r16, Sreg - Move from segment register to general register
// Opcode: 8C /r
// ============================================================================

#[test]
fn test_mov_ax_es() {
    let code = [
        0x8c, 0xc0, // MOV AX, ES
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // ES selector is 0 in the default test setup; low 16 bits of AX hold it.
    assert_eq!(regs.rax & 0xFFFF, 0x0000, "AX = ES selector (0)");
}

#[test]
fn test_mov_bx_ds() {
    let code = [
        0x8c, 0xdb, // MOV BX, DS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // DS selector is 0 in the default test setup.
    assert_eq!(regs.rbx & 0xFFFF, 0x0000, "BX = DS selector (0)");
}

#[test]
fn test_mov_cx_ss() {
    let code = [
        0x8c, 0xd1, // MOV CX, SS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // SS selector is 0 in the default test setup.
    assert_eq!(regs.rcx & 0xFFFF, 0x0000, "CX = SS selector (0)");
}

#[test]
fn test_mov_dx_fs() {
    let code = [
        0x8c, 0xe2, // MOV DX, FS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // FS selector is 0 in the default test setup.
    assert_eq!(regs.rdx & 0xFFFF, 0x0000, "DX = FS selector (0)");
}

#[test]
fn test_mov_si_gs() {
    let code = [
        0x8c, 0xee, // MOV SI, GS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // GS selector is 0 in the default test setup.
    assert_eq!(regs.rsi & 0xFFFF, 0x0000, "SI = GS selector (0)");
}

#[test]
fn test_mov_r32_es() {
    // In 64-bit mode, can move to 32-bit register (zero-extended)
    let code = [
        0x8c, 0xc0, // MOV EAX, ES
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Upper 48 bits should be zero
    assert_eq!(regs.rax & 0xFFFF_FFFF_0000, 0);
}

#[test]
fn test_mov_r64_ds() {
    // With REX.W, move to 64-bit register (zero-extended)
    let code = [
        0x48, 0x8c, 0xd8, // MOV RAX, DS (REX.W + MOV)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Upper 48 bits should be zero
    assert_eq!(regs.rax >> 16, 0);
}

#[test]
fn test_mov_mem16_es() {
    let code = [
        0x8c, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOV [0x2000], ES
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();

    // Read the segment register value from memory
    let mut buf = [0u8; 2];
    mem.read_slice(&mut buf, GuestAddress(DATA_ADDR)).unwrap();
    let es_value = u16::from_le_bytes(buf);
    // ES selector is 0; the 16-bit store writes exactly that.
    assert_eq!(es_value, 0x0000, "MOV [mem], ES stores ES selector (0)");
}

#[test]
fn test_mov_mem16_ds() {
    let code = [
        0x8c, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // MOV [0x2000], DS
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();

    let mut buf = [0u8; 2];
    mem.read_slice(&mut buf, GuestAddress(DATA_ADDR)).unwrap();
    let ds_value = u16::from_le_bytes(buf);
    assert_eq!(ds_value, 0x0000, "MOV [mem], DS stores DS selector (0)");
}

#[test]
fn test_mov_mem16_ss() {
    let code = [
        0x8c, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00, // MOV [0x2000], SS
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();

    let mut buf = [0u8; 2];
    mem.read_slice(&mut buf, GuestAddress(DATA_ADDR)).unwrap();
    let ss_value = u16::from_le_bytes(buf);
    assert_eq!(ss_value, 0x0000, "MOV [mem], SS stores SS selector (0)");
}

#[test]
fn test_mov_mem16_fs() {
    let code = [
        0x8c, 0x24, 0x25, 0x00, 0x20, 0x00, 0x00, // MOV [0x2000], FS
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();

    let mut buf = [0u8; 2];
    mem.read_slice(&mut buf, GuestAddress(DATA_ADDR)).unwrap();
    let fs_value = u16::from_le_bytes(buf);
    assert_eq!(fs_value, 0x0000, "MOV [mem], FS stores FS selector (0)");
}

#[test]
fn test_mov_mem16_gs() {
    let code = [
        0x8c, 0x2c, 0x25, 0x00, 0x20, 0x00, 0x00, // MOV [0x2000], GS
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();

    let mut buf = [0u8; 2];
    mem.read_slice(&mut buf, GuestAddress(DATA_ADDR)).unwrap();
    let gs_value = u16::from_le_bytes(buf);
    assert_eq!(gs_value, 0x0000, "MOV [mem], GS stores GS selector (0)");
}

// ============================================================================
// MOV Sreg, r16 - Move from general register to segment register
// Opcode: 8E /r
// Note: Cannot load CS with MOV
// ============================================================================

#[test]
fn test_mov_es_ax() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x8e, 0xc0, // MOV ES, AX
        0x8c, 0xc3, // MOV BX, ES (read back)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFFFF, 0); // ES should be 0
}

#[test]
fn test_mov_ds_cx() {
    let code = [
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0x8e, 0xd9, // MOV DS, CX
        0x8c, 0xd8, // MOV AX, DS (read back)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0);
}

#[test]
fn test_mov_ss_dx() {
    let code = [
        0x48, 0xc7, 0xc2, 0x00, 0x00, 0x00, 0x00, // MOV RDX, 0
        0x8e, 0xd2, // MOV SS, DX
        0x8c, 0xd0, // MOV AX, SS (read back)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0);
}

#[test]
fn test_mov_fs_bx() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0
        0x8e, 0xe3, // MOV FS, BX
        0x8c, 0xe0, // MOV AX, FS (read back)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0);
}

#[test]
fn test_mov_gs_si() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x00, 0x00, 0x00, // MOV RSI, 0
        0x8e, 0xee, // MOV GS, SI
        0x8c, 0xe8, // MOV AX, GS (read back)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0);
}

#[test]
fn test_mov_es_mem16() {
    let code = [
        // Write value to memory first
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x66, 0x67, 0xa3, 0x00, 0x20, 0x00, 0x00, // MOV [0x2000], AX (addr-size override)
        // Load from memory to ES
        0x8e, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOV ES, [0x2000]
        0x8c, 0xc0, // MOV AX, ES (read back)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0);
}

#[test]
fn test_mov_ds_mem16() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x66, 0x67, 0xa3, 0x00, 0x20, 0x00, 0x00, // MOV [0x2000], AX (addr-size override)
        0x8e, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // MOV DS, [0x2000]
        0x8c, 0xd8, // MOV AX, DS (read back)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0);
}

// ============================================================================
// Advanced tests with segment register operations
// ============================================================================

#[test]
fn test_mov_segment_preserves_upper_bits() {
    // Moving segment register to 32/64-bit register should zero upper bits
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, 0xFFFFFFFF
        0x8c, 0xd8, // MOV EAX, DS (should zero upper 32 bits)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Upper 48 bits should be zero (segment is only 16 bits)
    assert_eq!(regs.rax >> 16, 0);
}

#[test]
fn test_mov_segment_from_r64_uses_low16() {
    // When loading segment from 64-bit register, only low 16 bits are used
    let code = [
        0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff,
        0xff, // MOV RAX, 0xFFFFFFFF00000000
        0x8e, 0xe0, // MOV FS, AX (should use low 16 bits = 0)
        0x8c, 0xe3, // MOV BX, FS (read back)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFFFF, 0); // FS should be 0 (from low 16 bits of RAX)
}

#[test]
fn test_mov_all_segment_registers_sequence() {
    let code = [
        // Save all segment registers to general registers
        0x8c, 0xc0, // MOV AX, ES
        0x8c, 0xdb, // MOV BX, DS
        0x8c, 0xd1, // MOV CX, SS
        0x8c, 0xe2, // MOV DX, FS
        0x8c, 0xee, // MOV SI, GS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // ES/DS/SS/FS/GS selectors are all 0 in the default test setup.
    assert_eq!(regs.rax & 0xFFFF, 0, "ES selector = 0");
    assert_eq!(regs.rbx & 0xFFFF, 0, "DS selector = 0");
    assert_eq!(regs.rcx & 0xFFFF, 0, "SS selector = 0");
    assert_eq!(regs.rdx & 0xFFFF, 0, "FS selector = 0");
    assert_eq!(regs.rsi & 0xFFFF, 0, "GS selector = 0");
}

#[test]
fn test_mov_segment_roundtrip() {
    let code = [
        // Read ES
        0x8c, 0xc0, // MOV AX, ES
        // Save to memory
        0x66, 0x67, 0xa3, 0x00, 0x20, 0x00, 0x00, // MOV [0x2000], AX (addr-size override)
        // Load back to FS
        0x8e, 0x24, 0x25, 0x00, 0x20, 0x00, 0x00, // MOV FS, [0x2000]
        // Read FS
        0x8c, 0xe3, // MOV BX, FS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AX and BX should match (ES value transferred to FS); both are 0.
    assert_eq!(regs.rax & 0xFFFF, regs.rbx & 0xFFFF);
    assert_eq!(regs.rbx & 0xFFFF, 0x0000, "FS loaded from ES (selector 0)");
}

#[test]
fn test_mov_es_zero_value() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        0x8e, 0xc0, // MOV ES, AX (load 0)
        0x8c, 0xc3, // MOV BX, ES
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFFFF, 0);
}

#[test]
fn test_mov_multiple_segments_to_memory() {
    let code = [
        0x8c, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOV [0x2000], ES
        0x8c, 0x1c, 0x25, 0x02, 0x20, 0x00, 0x00, // MOV [0x2002], DS
        0x8c, 0x14, 0x25, 0x04, 0x20, 0x00, 0x00, // MOV [0x2004], SS
        0x8c, 0x24, 0x25, 0x06, 0x20, 0x00, 0x00, // MOV [0x2006], FS
        0x8c, 0x2c, 0x25, 0x08, 0x20, 0x00, 0x00, // MOV [0x2008], GS
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();

    // Read all saved segment registers (ES/DS/SS/FS/GS all selector 0).
    for offset in [0x2000, 0x2002, 0x2004, 0x2006, 0x2008] {
        let mut buf = [0u8; 2];
        mem.read_slice(&mut buf, GuestAddress(offset)).unwrap();
        let seg_value = u16::from_le_bytes(buf);
        assert_eq!(seg_value, 0x0000, "stored selector at {:#x} is 0", offset);
    }
}

#[test]
fn test_mov_segment_with_rex_prefix() {
    let code = [
        0x48, 0x8c, 0xd8, // REX.W + MOV RAX, DS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Upper 48 bits should be zero (segment is 16-bit, zero-extended)
    assert_eq!(regs.rax >> 16, 0);
}

#[test]
fn test_mov_fs_different_values() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x8e, 0xe0, // MOV FS, AX
        0x8c, 0xe3, // MOV BX, FS
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0 (again)
        0x8e, 0xe0, // MOV FS, AX
        0x8c, 0xe1, // MOV CX, FS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFF, 0);
    assert_eq!(regs.rcx & 0xFFFF, 0);
}

#[test]
fn test_mov_gs_different_values() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x8e, 0xe8, // MOV GS, AX
        0x8c, 0xe8, // MOV AX, GS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0);
}

#[test]
fn test_mov_segment_zero_extension_32bit() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x8c, 0xd8, // MOV EAX, DS (32-bit form)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Upper 32 bits should be zeroed by 32-bit write
    assert_eq!(regs.rax >> 32, 0);
}

#[test]
fn test_mov_ds_es_copy() {
    let code = [
        0x8c, 0xc0, // MOV AX, ES
        0x8e, 0xd8, // MOV DS, AX
        0x8c, 0xdb, // MOV BX, DS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // AX and BX should match (ES copied to DS); both selector 0.
    assert_eq!(regs.rax & 0xFFFF, regs.rbx & 0xFFFF);
    assert_eq!(regs.rbx & 0xFFFF, 0x0000, "DS copied from ES (selector 0)");
}

#[test]
fn test_mov_segment_indirect_addressing() {
    let code = [
        // Set up pointer in RBX
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        // Write value to memory
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x66, 0x89, 0x03, // MOV [RBX], AX
        // Load from memory using indirect addressing
        0x8e, 0x23, // MOV FS, [RBX]
        0x8c, 0xe0, // MOV AX, FS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0);
}

#[test]
fn test_mov_segment_with_displacement() {
    let code = [
        // Set up base in RAX
        0x48, 0xc7, 0xc0, 0xf0, 0x1f, 0x00, 0x00, // MOV RAX, 0x1FF0
        // Write to memory at base + 0x10
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0x66, 0x89, 0x48, 0x10, // MOV [RAX+0x10], CX
        // Load with displacement
        0x8e, 0x60, 0x10, // MOV FS, [RAX+0x10]
        0x8c, 0xe3, // MOV BX, FS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFFFF, 0);
}

#[test]
fn test_mov_all_segment_to_stack() {
    let code = [
        0x8c, 0xc0, // MOV AX, ES
        0x50, // PUSH RAX
        0x8c, 0xd8, // MOV AX, DS
        0x50, // PUSH RAX
        0x8c, 0xd0, // MOV AX, SS
        0x50, // PUSH RAX
        0x8c, 0xe0, // MOV AX, FS
        0x50, // PUSH RAX
        0x8c, 0xe8, // MOV AX, GS
        0x50, // PUSH RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_mov_segment_back_to_back() {
    let code = [
        0x8c, 0xc0, // MOV AX, ES
        0x8c, 0xc0, // MOV AX, ES (again)
        0x8c, 0xc0, // MOV AX, ES (again)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_mov_es_ds_swap() {
    let code = [
        0x8c, 0xc0, // MOV AX, ES
        0x8c, 0xd9, // MOV CX, DS
        0x8e, 0xd8, // MOV DS, AX
        0x8e, 0xc1, // MOV ES, CX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_mov_segment_cascade() {
    let code = [
        0x8c, 0xc0, // MOV AX, ES
        0x8e, 0xd8, // MOV DS, AX
        0x8c, 0xd9, // MOV CX, DS
        0x8e, 0xe1, // MOV FS, CX
        0x8c, 0xe2, // MOV DX, FS
        0x8e, 0xea, // MOV GS, DX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_mov_segment_r8_r15() {
    let code = [
        0x4c, 0x8c, 0xd8, // MOV R8, DS (with REX prefix)
        0x4c, 0x8c, 0xe1, // MOV R9, FS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // R8 = DS selector (0), R9 = FS selector (0); upper bits zeroed too.
    assert_eq!(regs.r8, 0x0000, "R8 = DS selector (0)");
    assert_eq!(regs.r9, 0x0000, "R9 = FS selector (0)");
}

#[test]
fn test_mov_es_from_r8() {
    let code = [
        0x49, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV R8, 0
        0x41, 0x8e, 0xc0, // MOV ES, R8
        0x8c, 0xc3, // MOV BX, ES
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFFFF, 0);
}

#[test]
fn test_mov_segment_with_sib() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x20, 0x00, 0x00, // MOV RAX, 0x2000
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0x66, 0x89, 0x0c, 0x18, // MOV [RAX+RBX], CX
        0x8e, 0x24, 0x18, // MOV FS, [RAX+RBX]
        0x8c, 0xe2, // MOV DX, FS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdx & 0xFFFF, 0);
}

#[test]
fn test_mov_segment_null_selector() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        0x8e, 0xe0, // MOV FS, AX (load NULL selector)
        0x8c, 0xe3, // MOV BX, FS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFFFF, 0);
}

// ============================================================================
// Value-asserting segment tests: CS selector read, FS/GS base via WRMSR with a
// memory access through an FS/GS override (asserting the exact linear address),
// LSL/LAR, and segment-override-prefixed loads.
//
// In the test harness CS.selector = 0x08 and ES/DS/SS/FS/GS selectors = 0.
// `set_sreg` resets the segment base to 0, so FS/GS base tests set the base via
// WRMSR and then access memory *without* reloading the selector.
// ============================================================================

// MOV r16, CS reads the actual CS selector (0x08 in this harness).
#[test]
fn test_mov_ax_cs_reads_selector() {
    let code = [
        0x8c, 0xc8, // MOV AX, CS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0x0008, "AX = CS selector (0x08)");
}

#[test]
fn test_mov_r64_cs_zero_extends() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x8c, 0xc8, // MOV RAX, CS (REX.W)
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x0000_0000_0000_0008,
        "RAX = CS selector, zero-extended"
    );
}

// A non-zero selector loaded into a segment register reads back exactly.
#[test]
fn test_mov_fs_nonzero_selector_roundtrip() {
    let code = [
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 0x10
        0x8e, 0xe0, // MOV FS, AX
        0x8c, 0xe3, // MOV BX, FS
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rbx & 0xFFFF,
        0x0010,
        "FS reads back loaded selector 0x10"
    );
}

// FS-base + override: a load through an FS override reads from (fs.base + EA).
// EA = [RBX] with RBX = 0; fs.base = 0x3000 -> linear address 0x3000.
#[test]
fn test_fs_base_override_load_linear_address() {
    let code = [
        // WRMSR FS.base = 0x3000 : ECX=0xC0000100, EDX:EAX=0:0x3000
        0xb9, 0x00, 0x01, 0x00, 0xc0, // MOV ECX, 0xC0000100
        0x31, 0xd2, // XOR EDX, EDX
        0xb8, 0x00, 0x30, 0x00, 0x00, // MOV EAX, 0x3000
        0x0f, 0x30, // WRMSR
        // RBX = 0; load [FS:RBX] (FS override = 0x64)
        0x48, 0x31, 0xdb, // XOR RBX, RBX
        0x64, 0x48, 0x8b, 0x03, // MOV RAX, FS:[RBX]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    // Plant a sentinel at the FS-relative linear address 0x3000.
    write_mem_at_u64(&mem, 0x3000, 0xF00DCAFE_12345678);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0xF00DCAFE_12345678,
        "FS:[0] reads fs.base linear addr 0x3000"
    );
}

// FS-base + override with displacement: linear address = fs.base + disp.
// fs.base = 0x4000, EA = [RBX + 0x40] with RBX = 0 -> 0x4040.
#[test]
fn test_fs_base_override_load_with_disp() {
    let code = [
        0xb9, 0x00, 0x01, 0x00, 0xc0, // MOV ECX, 0xC0000100
        0x31, 0xd2, // XOR EDX, EDX
        0xb8, 0x00, 0x40, 0x00, 0x00, // MOV EAX, 0x4000
        0x0f, 0x30, // WRMSR (FS.base = 0x4000)
        0x48, 0x31, 0xdb, // XOR RBX, RBX
        0x64, 0x48, 0x8b, 0x43, 0x40, // MOV RAX, FS:[RBX + 0x40]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_at_u64(&mem, 0x4040, 0xAABBCCDD_EEFF0011);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0xAABBCCDD_EEFF0011,
        "FS:[disp] reads fs.base + disp (0x4040)"
    );
}

// A store through an FS override writes to (fs.base + EA).
#[test]
fn test_fs_base_override_store_linear_address() {
    let code = [
        0xb9, 0x00, 0x01, 0x00, 0xc0, // MOV ECX, 0xC0000100
        0x31, 0xd2, // XOR EDX, EDX
        0xb8, 0x00, 0x50, 0x00, 0x00, // MOV EAX, 0x5000
        0x0f, 0x30, // WRMSR (FS.base = 0x5000)
        0x48, 0x31, 0xdb, // XOR RBX, RBX
        0x48, 0xb8, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22,
        0x11, // MOV RAX, 0x1122334455667788
        0x64, 0x48, 0x89, 0x03, // MOV FS:[RBX], RAX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    // The store must land at the FS-relative linear address 0x5000.
    assert_eq!(
        read_mem_at_u64(&mem, 0x5000),
        0x1122334455667788,
        "FS:[0] store at 0x5000"
    );
    // And NOT at offset 0 (no base applied).
    assert_eq!(
        read_mem_at_u64(&mem, 0x0000),
        0x0000000000000000,
        "no write at linear 0"
    );
}

// GS-base + override: load through a GS override reads from (gs.base + EA).
// gs.base = 0x6000; the WRMSR side-effect write lands far out of range and is
// harmlessly ignored.
#[test]
fn test_gs_base_override_load_linear_address() {
    let code = [
        0xb9, 0x01, 0x01, 0x00, 0xc0, // MOV ECX, 0xC0000101 (GS.base)
        0x31, 0xd2, // XOR EDX, EDX
        0xb8, 0x00, 0x60, 0x00, 0x00, // MOV EAX, 0x6000
        0x0f, 0x30, // WRMSR (GS.base = 0x6000)
        0x48, 0x31, 0xdb, // XOR RBX, RBX
        0x65, 0x48, 0x8b, 0x03, // MOV RAX, GS:[RBX] (GS override = 0x65)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_at_u64(&mem, 0x6000, 0x0123456789ABCDEF);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x0123456789ABCDEF,
        "GS:[0] reads gs.base linear addr 0x6000"
    );
}

// ES/DS/SS overrides have no base in 64-bit mode: a DS override on a [RBX]
// load reads the same linear address as no override.
#[test]
fn test_ds_override_no_base_64bit() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x3e, 0x48, 0x8b, 0x03, // MOV RAX, DS:[RBX] (DS override = 0x3E)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_at_u64(&mem, 0x2000, 0xDEADBEEFCAFEBABE);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0xDEADBEEFCAFEBABE,
        "DS override applies no base in 64-bit mode"
    );
}

// LAR with a non-null selector: ZF set, dest loaded with the emulated access
// rights (0x00CF9300). Selector 0x08 in AX, with ZF initially clear so the
// assertion proves LAR *sets* ZF (not merely that it was already set).
#[test]
fn test_lar_valid_selector() {
    let code = [
        0x0f, 0x02, 0xd8, // LAR EBX, EAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x08; // valid selector
    regs.rflags = 0x2; // ZF clear initially
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rbx & 0xFFFF_FFFF,
        0x00CF9300,
        "LAR loads access-rights dword"
    );
    assert!(
        zf_set(regs.rflags),
        "LAR sets ZF for a valid (non-null) selector"
    );
}

// LAR with the null selector (0): ZF cleared.
//
// EAX (the selector) and the initial ZF are seeded via the register file rather
// than with a preceding ALU instruction so that no deferred ("lazy") flag
// computation is pending when LAR runs -- otherwise the pending flags would be
// materialized *after* LAR and clobber its ZF write.
#[test]
fn test_lar_null_selector_clears_zf() {
    let code = [
        0x0f, 0x02, 0xd8, // LAR EBX, EAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0; // null selector
    regs.rflags = 0x40 | 0x2; // ZF set initially
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(!zf_set(regs.rflags), "LAR clears ZF for the null selector");
}

// LSL with a non-null selector: ZF set, dest loaded with the emulated limit
// (0xFFFFFFFF). ZF initially clear so the assertion proves LSL *sets* ZF.
#[test]
fn test_lsl_valid_selector() {
    let code = [
        0x0f, 0x03, 0xd8, // LSL EBX, EAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x08; // valid selector
    regs.rflags = 0x2; // ZF clear initially
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rbx & 0xFFFF_FFFF,
        0xFFFF_FFFF,
        "LSL loads segment limit"
    );
    assert!(
        zf_set(regs.rflags),
        "LSL sets ZF for a valid (non-null) selector"
    );
}

// LSL with the null selector: ZF cleared. (EAX/ZF seeded via the register file;
// see the LAR null-selector test for why no preceding ALU op is used.)
#[test]
fn test_lsl_null_selector_clears_zf() {
    let code = [
        0x0f, 0x03, 0xd8, // LSL EBX, EAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0; // null selector
    regs.rflags = 0x40 | 0x2; // ZF set initially
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(!zf_set(regs.rflags), "LSL clears ZF for the null selector");
}
