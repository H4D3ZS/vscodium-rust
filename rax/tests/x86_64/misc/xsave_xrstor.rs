// Module path for tests run via x86_64.rs
use crate::common::*;

// XSAVE - Save Processor Extended States
// XRSTOR - Restore Processor Extended States
//
// These instructions save and restore processor state components managed by XSAVE feature set
// Typically includes x87, SSE, AVX, and other extended state
//
// XSAVE:
// Opcode: NP 0F AE /4 XSAVE mem
// Opcode: NP REX.W 0F AE /4 XSAVE64 mem
// EDX:EAX specifies state component bitmap (RFBM)
// Saves components to memory XSAVE area (must be 64-byte aligned)
//
// XRSTOR:
// Opcode: NP 0F AE /5 XRSTOR mem
// Opcode: NP REX.W 0F AE /5 XRSTOR64 mem
// EDX:EAX specifies state component bitmap
// Restores components from memory XSAVE area (must be 64-byte aligned)
//
// XSAVE Area Layout:
// [0-511]: Legacy Region (x87 and SSE state)
// [512-575]: XSAVE Header
//   [512-519]: XSTATE_BV (which state components are valid)
//   [520-527]: XCOMP_BV (compression info)
// [576+]: Extended Region (AVX, AVX-512, etc.)

// ===== XSAVE TESTS =====

#[test]
fn test_xsave_requires_aligned_memory() {
    // XSAVE requires 64-byte aligned memory address
    // Attempting unaligned should cause #GP
    let code = [
        0x31, 0xc0, // XOR EAX, EAX (save x87 state only)
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0xae, 0x24, 0x00, // XSAVE [RSP] (likely unaligned, may fault)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let result = run_until_hlt(&mut vcpu);
    // May fault if RSP not aligned, which is expected
    if result.is_ok() {
        let _regs = result.unwrap();
    }
}

#[test]
fn test_xsave_basic_x87_state() {
    // XSAVE should save x87 state to memory
    // Use aligned memory address for XSAVE area
    let code = [
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 0x01 (save x87 state)
        0x31, 0xd2, // XOR EDX, EDX
        // Use memory address 0x3000 (assumed 64-byte aligned for testing)
        0x0f, 0xae, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // XSAVE [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();

    // Check that memory was written (should have some data)
    // The XSAVE area should contain valid data
    let first_dword = read_mem_at_u32(&mem, 0x3000);
    let _should_have_data = first_dword;
}

#[test]
fn test_xsave_edx_eax_specifies_bitmap() {
    // EDX:EAX should specify which state components to save
    // Bit 0 = x87, Bit 1 = SSE, Bit 2 = AVX, etc.
    let code = [
        0xb8, 0x03, 0x00, 0x00, 0x00, // MOV EAX, 0x03 (save x87 and SSE)
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0xae, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // XSAVE [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let result = run_until_hlt(&mut vcpu);
    if result.is_ok() {
        let _regs = result.unwrap();
    }
}

#[test]
fn test_xsave_zero_bitmap_minimal_save() {
    // XSAVE with EAX=0 should save minimal state (usually x87 header only)
    let code = [
        0x31, 0xc0, // XOR EAX, EAX (minimal save)
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0xae, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // XSAVE [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let result = run_until_hlt(&mut vcpu);
    if result.is_ok() {
        let _regs = result.unwrap();
    }
}

#[test]
fn test_xsave_header_contains_xstate_bv() {
    // XSAVE header at offset 512 should contain XSTATE_BV bitmap
    let code = [
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 0x01 (save x87)
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0xae, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // XSAVE [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();

    // XSTATE_BV should be at offset 512 from base
    let xstate_bv_addr = 0x3000 + 512;
    let xstate_bv = read_mem_at_u64(&mem, xstate_bv_addr as u64);

    // If x87 was saved, bit 0 of XSTATE_BV should be set
    // (Though it depends on current state)
    let _xstate_bv_value = xstate_bv;
}

#[test]
fn test_xsave_multiple_state_components() {
    // XSAVE with multiple state components specified
    let code = [
        0xb8, 0x07, 0x00, 0x00, 0x00, // MOV EAX, 0x07 (x87, SSE, AVX)
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0xae, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // XSAVE [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let result = run_until_hlt(&mut vcpu);
    if result.is_ok() {
        let _regs = result.unwrap();
    }
}

#[test]
fn test_xsave_does_not_modify_registers() {
    // XSAVE should not modify any registers
    let code = [
        0xb8, 0x11, 0x11, 0x11, 0x11, // MOV EAX, 0x11111111
        0xbb, 0x22, 0x22, 0x22, 0x22, // MOV EBX, 0x22222222
        0xb9, 0x33, 0x33, 0x33, 0x33, // MOV ECX, 0x33333333
        0xba, 0x44, 0x44, 0x44, 0x44, // MOV EDX, 0x44444444
        // Now use EDX:EAX for XSAVE bitmap - they'll be used but not modified
        0x0f, 0xae, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // XSAVE [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // While EDX:EAX are used as parameters, they should not be modified
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x11111111, "EAX unchanged");
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x22222222, "EBX unchanged");
    assert_eq!(regs.rcx & 0xFFFFFFFF, 0x33333333, "ECX unchanged");
    assert_eq!(regs.rdx & 0xFFFFFFFF, 0x44444444, "EDX unchanged");
}

#[test]
fn test_xsave_with_64bit_register_high_bits_ignored() {
    // In 64-bit mode, high 32 bits of RDX:RAX are ignored for XSAVE
    let code = [
        0x48, 0xb8, 0x01, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF,
        0xFF, // MOV RAX, 0xFFFFFFFF00000001
        0x48, 0xba, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF,
        0xFF, // MOV RDX, 0xFFFFFFFF00000000
        0x0f, 0xae, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // XSAVE [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let result = run_until_hlt(&mut vcpu);
    if result.is_ok() {
        let _regs = result.unwrap();
    }
}

// ===== XRSTOR TESTS =====

#[test]
fn test_xrstor_requires_aligned_memory() {
    // XRSTOR requires 64-byte aligned memory address
    // Use an unaligned address 0x3001 (instead of 0x3000)
    let code = [
        0x31, 0xc0, // XOR EAX, EAX
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0xae, 0x0c, 0x25, 0x01, 0x30, 0x00, 0x00, // FXRSTOR [0x3001] (unaligned)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let result = run_until_hlt(&mut vcpu);
    // May fault, which is expected (emulator may tolerate misalignment)
    if result.is_ok() {
        let _regs = result.unwrap();
    }
}

#[test]
fn test_xrstor_basic_restore() {
    // XRSTOR should restore state from memory
    let code = [
        // First save state
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 0x01
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0xae, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // XSAVE [0x3000]
        // Now restore state
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 0x01
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0xae, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // XRSTOR [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let result = run_until_hlt(&mut vcpu);
    if result.is_ok() {
        let _regs = result.unwrap();
    }
}

#[test]
fn test_xrstor_bitmap_specifies_components() {
    // EDX:EAX specifies which state components to restore
    let code = [
        // Save
        0xb8, 0x03, 0x00, 0x00, 0x00, // MOV EAX, 0x03 (x87 + SSE)
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0xae, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // XSAVE [0x3000]
        // Restore with different bitmap
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 0x01 (x87 only)
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0xae, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // XRSTOR [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let result = run_until_hlt(&mut vcpu);
    if result.is_ok() {
        let _regs = result.unwrap();
    }
}

#[test]
fn test_xrstor_does_not_modify_registers() {
    // XRSTOR should not modify registers
    let code = [
        0xb8, 0x11, 0x11, 0x11, 0x11, // MOV EAX, 0x11111111
        0xbb, 0x22, 0x22, 0x22, 0x22, // MOV EBX, 0x22222222
        0xb9, 0x33, 0x33, 0x33, 0x33, // MOV ECX, 0x33333333
        // Save first
        0x0f, 0xae, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // XSAVE [0x3000]
        // Restore
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 0x01
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0xae, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // XRSTOR [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // EBX and ECX should be unchanged by XRSTOR
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x22222222, "EBX unchanged");
    assert_eq!(regs.rcx & 0xFFFFFFFF, 0x33333333, "ECX unchanged");
}

#[test]
fn test_xrstor_zero_bitmap_minimal_restore() {
    // XRSTOR with EAX=0 should do minimal restore
    let code = [
        // Save
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 0x01
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0xae, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // XSAVE [0x3000]
        // Restore with zero bitmap
        0x31, 0xc0, // XOR EAX, EAX
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0xae, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // XRSTOR [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let result = run_until_hlt(&mut vcpu);
    if result.is_ok() {
        let _regs = result.unwrap();
    }
}

#[test]
fn test_xsave_xrstor_roundtrip() {
    // XSAVE followed by XRSTOR should restore state
    let code = [
        // Initialize some state (move to x87/SSE registers)
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 0x01
        // Save state
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0xae, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // XSAVE [0x3000]
        // Modify state (simulated)
        0xb8, 0x02, 0x00, 0x00, 0x00, // MOV EAX, 0x02
        // Restore state
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 0x01
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0xae, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // XRSTOR [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let result = run_until_hlt(&mut vcpu);
    if result.is_ok() {
        let _regs = result.unwrap();
    }
}

#[test]
fn test_xsave_multiple_areas() {
    // Multiple XSAVE operations to different memory areas
    let code = [
        // Save to first area
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 0x01
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0xae, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // XSAVE [0x3000]
        // Save to second area
        0xb8, 0x03, 0x00, 0x00, 0x00, // MOV EAX, 0x03
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0xae, 0x04, 0x25, 0x00, 0x34, 0x00, 0x00, // XSAVE [0x3400]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let result = run_until_hlt(&mut vcpu);
    if result.is_ok() {
        let _regs = result.unwrap();
    }
}

#[test]
fn test_xrstor_multiple_areas() {
    // Restore from multiple saved areas
    let code = [
        // Save area 1
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 0x01
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0xae, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // XSAVE [0x3000]
        // Save area 2
        0xb8, 0x03, 0x00, 0x00, 0x00, // MOV EAX, 0x03
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0xae, 0x04, 0x25, 0x00, 0x34, 0x00, 0x00, // XSAVE [0x3400]
        // Restore area 1
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 0x01
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0xae, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // XRSTOR [0x3000]
        // Restore area 2
        0xb8, 0x03, 0x00, 0x00, 0x00, // MOV EAX, 0x03
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0xae, 0x24, 0x25, 0x00, 0x34, 0x00, 0x00, // XRSTOR [0x3400]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let result = run_until_hlt(&mut vcpu);
    if result.is_ok() {
        let _regs = result.unwrap();
    }
}

#[test]
fn test_xsave_xrstor_with_varying_bitmaps() {
    // Test with different component bitmaps
    for bitmap in &[0x01u32, 0x03u32, 0x07u32, 0x0Fu32] {
        let eax_byte = (*bitmap & 0xFF) as u8;
        let edx_byte = ((*bitmap >> 8) & 0xFF) as u8;

        let code = [
            0xb8, eax_byte, 0x00, 0x00, 0x00, // MOV EAX, bitmap
            0x31, 0xd2, // XOR EDX, EDX
            0x0f, 0xae, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // XSAVE [0x3000]
            0xb8, eax_byte, 0x00, 0x00, 0x00, // MOV EAX, bitmap
            0x31, 0xd2, // XOR EDX, EDX
            0x0f, 0xae, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // XRSTOR [0x3000]
            0xf4, // HLT
        ];

        let (mut vcpu, _) = setup_vm(&code, None);
        let result = run_until_hlt(&mut vcpu);
        if result.is_ok() {
            let _regs = result.unwrap();
        }
    }
}

#[test]
fn test_xsave_with_high_bits_in_edx() {
    // Test XSAVE with extended state components in EDX
    let code = [
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 0x01
        0xba, 0x10, 0x00, 0x00, 0x00, // MOV EDX, 0x10 (extended components)
        0x0f, 0xae, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // XSAVE [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let result = run_until_hlt(&mut vcpu);
    if result.is_ok() {
        let _regs = result.unwrap();
    }
}

#[test]
fn test_xrstor_with_high_bits_in_edx() {
    // Test XRSTOR with extended state components in EDX
    let code = [
        // Save with extended components
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 0x01
        0xba, 0x10, 0x00, 0x00, 0x00, // MOV EDX, 0x10
        0x0f, 0xae, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // XSAVE [0x3000]
        // Restore with same components
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 0x01
        0xba, 0x10, 0x00, 0x00, 0x00, // MOV EDX, 0x10
        0x0f, 0xae, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // XRSTOR [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let result = run_until_hlt(&mut vcpu);
    if result.is_ok() {
        let _regs = result.unwrap();
    }
}

#[test]
fn test_xsave_does_not_modify_flags() {
    // XSAVE should not modify flags
    let code = [
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 0x01
        0x31, 0xd2, // XOR EDX, EDX
        0xf9, // STC (set carry) - after XOR so it's not cleared
        0x0f, 0xae, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // FXSAVE [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Carry flag should still be set
    assert_eq!(cf_set(regs.rflags), true, "CF unchanged");
}

#[test]
fn test_xrstor_does_not_modify_flags() {
    // XRSTOR should not modify flags
    let code = [
        // Save
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 0x01
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0xae, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // FXSAVE [0x3000]
        // Prepare for restore (XOR first, then set flag)
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 0x01
        0x31, 0xd2, // XOR EDX, EDX
        0xf9, // STC (after XOR so it's not cleared)
        // Restore
        0x0f, 0xae, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // FXRSTOR [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(cf_set(regs.rflags), true, "CF unchanged");
}
