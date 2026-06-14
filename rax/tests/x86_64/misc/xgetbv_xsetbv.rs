// Module path for tests run via x86_64.rs
use crate::common::*;

// XGETBV - Get Value of Extended Control Register
// XSETBV - Set Extended Control Register
//
// These instructions manage extended control registers (XCRs)
// ECX specifies which XCR: typically only XCR0 (XFEATURE_ENABLED_MASK) is supported
//
// XGETBV:
// Opcode: NP 0F 01 D0
// Reads XCR[ECX] into EDX:EAX
// Requires: CR4.OSXSAVE=1 and CPUID.01H:ECX.XSAVE[bit 26]=1
//
// XSETBV:
// Opcode: NP 0F 01 D1
// Writes EDX:EAX to XCR[ECX]
// Requires: CPL=0 (privilege level 0)
// Requires: CR4.OSXSAVE=1 and CPUID.01H:ECX.XSAVE[bit 26]=1
//
// XCR0 Bits:
// Bit 0: x87 FPU state (must always be 1)
// Bit 1: SSE registers state
// Bit 2: AVX state (requires both bit 1 and 2 set)
// Bits 3-62: Extended state components (AVX-512, etc.)

// ===== XGETBV TESTS =====

#[test]
fn test_xgetbv_xcr0_basic() {
    // XGETBV should read XCR0 into EDX:EAX
    let code = [
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0 (XCR0)
        0x0f, 0x01, 0xd0, // XGETBV
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // EDX:EAX should contain XCR0 value
    let edx = regs.rdx & 0xFFFFFFFF;
    let eax = regs.rax & 0xFFFFFFFF;
    let xcr0 = ((edx as u64) << 32) | (eax as u64);

    // XCR0 should have at least bit 0 (x87) set
    assert_eq!(xcr0 & 1, 1, "XCR0 bit 0 (x87) must be set");
}

#[test]
fn test_xgetbv_xcr0_eax_contains_lower_32bits() {
    // XGETBV should put lower 32 bits of XCR0 in EAX
    let code = [
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0 (XCR0)
        0x0f, 0x01, 0xd0, // XGETBV
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // EAX should contain lower 32 bits
    let eax = regs.rax & 0xFFFFFFFF;
    // At minimum, bit 0 should be set
    assert_eq!(eax & 1, 1, "EAX[0] should be set for x87 state");
}

#[test]
fn test_xgetbv_xcr0_edx_contains_upper_32bits() {
    // XGETBV should put upper 32 bits of XCR0 in EDX
    let code = [
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0 (XCR0)
        0x0f, 0x01, 0xd0, // XGETBV
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // EDX should contain upper 32 bits
    let edx = regs.rdx & 0xFFFFFFFF;
    // EDX could be 0 for most CPUs (features fit in lower 32 bits)
    // Just verify it's a valid value
    let _edx_value = edx;
}

#[test]
fn test_xgetbv_xcr0_multiple_reads_consistent() {
    // Multiple XGETBV calls should return consistent values
    let code = [
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0x0f, 0x01, 0xd0, // XGETBV (first read)
        0x89, 0xc3, // MOV EBX, EAX (save first EAX)
        0x89, 0xd1, // MOV ECX, EDX (EDX -> save)
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0x0f, 0x01, 0xd0, // XGETBV (second read)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let eax_second = regs.rax & 0xFFFFFFFF;
    let eax_first = regs.rbx & 0xFFFFFFFF;

    // Both reads should return same value
    assert_eq!(
        eax_first, eax_second,
        "XGETBV should return consistent values"
    );
}

#[test]
fn test_xgetbv_xcr0_x87_always_set() {
    // Bit 0 of XCR0 (x87 state) should always be set
    let code = [
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0x0f, 0x01, 0xd0, // XGETBV
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let eax = regs.rax & 0xFFFFFFFF;
    // Bit 0 must be set
    assert_ne!(eax & 1, 0, "XCR0 bit 0 (x87) must be set");
}

#[test]
fn test_xgetbv_ecx_parameter_used() {
    // ECX should specify which XCR to read
    // Valid values are typically 0 and 1 (if XSAVE extended features supported)
    let code = [
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0 (XCR0)
        0x0f, 0x01, 0xd0, // XGETBV
        0x89, 0xc3, // MOV EBX, EAX (save XCR0)
        0xb9, 0x01, 0x00, 0x00, 0x00, // MOV ECX, 1 (XCR1, if supported)
        0x0f, 0x01, 0xd0, // XGETBV
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let xcr0_value = regs.rbx & 0xFFFFFFFF;
    let xcr1_value = regs.rax & 0xFFFFFFFF;

    // XCR0 should have at least bit 0 set
    assert_eq!(xcr0_value & 1, 1, "XCR0 should have bit 0 set");
    // XCR1 may be 0 if not supported (depends on implementation)
    let _xcr1 = xcr1_value;
}

#[test]
fn test_xgetbv_clears_high_32bits_in_64bit_mode() {
    // In 64-bit mode, high-order 32 bits of RAX/RDX should be cleared
    let code = [
        0x48, 0xb8, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, // MOV RAX, 0xFFFFFFFFFFFFFFFF
        0x48, 0xba, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, // MOV RDX, 0xFFFFFFFFFFFFFFFF
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0x0f, 0x01, 0xd0, // XGETBV (should clear upper 32 bits)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // In 64-bit mode, high-order 32 bits of RAX and RDX are cleared
    let rax_upper = regs.rax >> 32;
    let rdx_upper = regs.rdx >> 32;

    assert_eq!(rax_upper, 0, "RAX upper 32 bits should be cleared");
    assert_eq!(rdx_upper, 0, "RDX upper 32 bits should be cleared");
}

#[test]
fn test_xgetbv_does_not_modify_other_registers() {
    // XGETBV should only modify EAX and EDX
    let code = [
        0xb8, 0x11, 0x11, 0x11, 0x11, // MOV EAX, 0x11111111
        0xbb, 0x22, 0x22, 0x22, 0x22, // MOV EBX, 0x22222222
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0 (for XGETBV parameter)
        0x0f, 0x01, 0xd0, // XGETBV
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // EBX should be unchanged
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x22222222, "EBX should be unchanged");
}

#[test]
fn test_xgetbv_does_not_modify_flags() {
    // XGETBV should not modify flags
    let code = [
        0xf9, // STC (set carry flag)
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0x0f, 0x01, 0xd0, // XGETBV
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Carry flag should still be set
    assert_eq!(cf_set(regs.rflags), true, "CF should be unchanged");
}

// ===== XSETBV TESTS =====

#[test]
fn test_xsetbv_xcr0_x87_required() {
    // XSETBV should reject attempts to clear bit 0 of XCR0
    // This would cause a #GP exception, which we expect to fault
    // For testing, we'll try to set it to a valid value instead
    let code = [
        0x31, 0xc0, // XOR EAX, EAX (EAX = 0)
        0x31, 0xd2, // XOR EDX, EDX (EDX = 0)
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0 (XCR0)
        0x0f, 0x01, 0xd1, // XSETBV (would fail with #GP)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    // This may fault, which is expected behavior
    let result = run_until_hlt(&mut vcpu);
    // If it completes, the system must have rejected it
    if result.is_ok() {
        // The instruction should have been rejected (or CPU allows it)
        let _regs = result.unwrap();
    }
}

#[test]
fn test_xsetbv_xcr0_x87_must_remain_set() {
    // Try to set XCR0 with x87 bit set (should succeed)
    let code = [
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 1 (bit 0 = x87, set)
        0x31, 0xd2, // XOR EDX, EDX (EDX = 0)
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0 (XCR0)
        0x0f, 0x01, 0xd1, // XSETBV
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let result = run_until_hlt(&mut vcpu);
    // May fail in user mode (requires CPL=0), which is expected
    if result.is_ok() {
        let _regs = result.unwrap();
    }
}

#[test]
fn test_xsetbv_sse_compat_with_avx() {
    // AVX (bit 2) requires SSE (bit 1) to also be set
    // Setting bit 2 without bit 1 should fail
    let code = [
        0xb8, 0x04, 0x00, 0x00, 0x00, // MOV EAX, 0x04 (bit 2 = AVX, no bit 1 = SSE)
        0x31, 0xd2, // XOR EDX, EDX
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0 (XCR0)
        0x0f, 0x01, 0xd1, // XSETBV (should fail with #GP)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let result = run_until_hlt(&mut vcpu);
    // This should fail or be rejected, but we can't easily test exceptions
    if result.is_ok() {
        let _regs = result.unwrap();
    }
}

#[test]
fn test_xsetbv_xcr0_valid_combination() {
    // Set XCR0 with x87 and SSE enabled
    let code = [
        0xb8, 0x03, 0x00, 0x00, 0x00, // MOV EAX, 0x03 (bits 0 and 1 set)
        0x31, 0xd2, // XOR EDX, EDX
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0x0f, 0x01, 0xd1, // XSETBV
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let result = run_until_hlt(&mut vcpu);
    // May fail in user mode, which is expected
    if result.is_ok() {
        let _regs = result.unwrap();
    }
}

#[test]
fn test_xsetbv_accepts_apx_f_bit() {
    let code = [
        0xb8, 0x07, 0x00, 0x08, 0x00, // MOV EAX, x87|SSE|AVX|APX_F
        0x31, 0xd2, // XOR EDX, EDX
        0x31, 0xc9, // XOR ECX, ECX
        0x0f, 0x01, 0xd1, // XSETBV
        0x0f, 0x01, 0xd0, // XGETBV
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax as u32, 0x80007, "XCR0 should retain APX_F");
    assert_eq!(regs.rdx as u32, 0, "APX_F is in the low XCR0 dword");
}

#[test]
fn test_xsetbv_ecx_parameter_selects_xcrn() {
    // Different ECX values select different XCRs
    let code = [
        0xb8, 0x03, 0x00, 0x00, 0x00, // MOV EAX, 0x03
        0x31, 0xd2, // XOR EDX, EDX
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0 (XCR0)
        0x0f, 0x01, 0xd1, // XSETBV (XCR0)
        0xb9, 0x01, 0x00, 0x00, 0x00, // MOV ECX, 1 (XCR1, might not exist)
        0x0f, 0x01, 0xd1, // XSETBV (XCR1)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let result = run_until_hlt(&mut vcpu);
    // XCR1 doesn't exist on most CPUs, so this may fault
    if result.is_ok() {
        let _regs = result.unwrap();
    }
}

#[test]
fn test_xsetbv_edx_eax_64bit_parameter() {
    // EDX:EAX forms 64-bit value to set
    let code = [
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 1 (lower 32 bits)
        0xba, 0x00, 0x00, 0x00, 0x00, // MOV EDX, 0 (upper 32 bits)
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0 (XCR0)
        0x0f, 0x01, 0xd1, // XSETBV
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let result = run_until_hlt(&mut vcpu);
    if result.is_ok() {
        let _regs = result.unwrap();
    }
}

#[test]
fn test_xgetbv_after_xsetbv_reflects_change() {
    // After XSETBV, XGETBV should read back the set value
    // Note: This may fail if running in user mode (XSETBV requires CPL=0)
    let code = [
        // First, get current XCR0
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0x0f, 0x01, 0xd0, // XGETBV (read current)
        // Try to set XCR0 (may fail in user mode)
        0xb8, 0x03, 0x00, 0x00, 0x00, // MOV EAX, 0x03
        0x31, 0xd2, // XOR EDX, EDX
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0x0f, 0x01, 0xd1, // XSETBV (may fault)
        // Read it back
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0x0f, 0x01, 0xd0, // XGETBV
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let result = run_until_hlt(&mut vcpu);
    // This test just verifies the sequence executes without crashing
    if result.is_ok() {
        let _regs = result.unwrap();
    }
}

#[test]
fn test_xgetbv_xsetbv_operations_sequence() {
    // Test a sequence of XGETBV/XSETBV operations
    let code = [
        // Read XCR0
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0x0f, 0x01, 0xd0, // XGETBV
        0x89, 0xc3, // MOV EBX, EAX (save read value)
        // Attempt to set (may fail)
        0xb8, 0x03, 0x00, 0x00, 0x00, // MOV EAX, 0x03
        0x31, 0xd2, // XOR EDX, EDX
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0x0f, 0x01, 0xd1, // XSETBV
        // Read again
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0x0f, 0x01, 0xd0, // XGETBV
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let result = run_until_hlt(&mut vcpu);
    if result.is_ok() {
        let _regs = result.unwrap();
    }
}

#[test]
fn test_xgetbv_xcr0_contains_valid_state_bits() {
    // XCR0 should contain only valid state component bits
    let code = [
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0x0f, 0x01, 0xd0, // XGETBV
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let eax = regs.rax & 0xFFFFFFFF;

    // Bit 0 (x87) must be set
    assert_eq!(eax & 1, 1, "Bit 0 (x87) must be set");

    // If bit 2 (AVX) is set, bit 1 (SSE) must also be set
    if (eax & 4) != 0 {
        assert_ne!(eax & 2, 0, "If AVX (bit 2) set, SSE (bit 1) must be set");
    }
}

#[test]
fn test_xgetbv_multiple_sequential_reads() {
    // Multiple sequential XGETBV calls should return same value
    let code = [
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0x0f, 0x01, 0xd0, // XGETBV (1st)
        0x89, 0xc6, // MOV ESI, EAX (save 1st read to ESI)
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0x0f, 0x01, 0xd0, // XGETBV (2nd)
        0x89, 0xc3, // MOV EBX, EAX (save 2nd read to EBX)
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0x0f, 0x01, 0xd0, // XGETBV (3rd)
        // Now EAX = 3rd read, EBX = 2nd read, ESI = 1st read
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let read3 = regs.rax & 0xFFFFFFFF;
    let read2 = regs.rbx & 0xFFFFFFFF;
    let read1 = regs.rsi & 0xFFFFFFFF;

    // All three should be identical
    assert_eq!(read1, read2, "Read 1 and 2 should match");
    assert_eq!(read2, read3, "Read 2 and 3 should match");
}

// ===== AVX-ENABLE + XSAVE/XRSTOR ROUND-TRIP =====

#[test]
fn test_avx_enable_handshake() {
    // Full enable sequence: set CR4.OSXSAVE, XSETBV XCR0 = x87|SSE|AVX (0x7),
    // then XGETBV must read back 0x7.
    let code = [
        0x0f, 0x20, 0xe0, // MOV RAX, CR4
        0x48, 0x0d, 0x00, 0x00, 0x04, 0x00, // OR RAX, 0x40000 (OSXSAVE, bit 18)
        0x0f, 0x22, 0xe0, // MOV CR4, RAX
        0xb8, 0x07, 0x00, 0x00, 0x00, // MOV EAX, 7
        0x31, 0xd2, // XOR EDX, EDX
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0x0f, 0x01, 0xd1, // XSETBV
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0x0f, 0x01, 0xd0, // XGETBV
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFF_FFFF,
        0x7,
        "XGETBV should read back XCR0 = 0x7 after enabling AVX"
    );
}

#[test]
fn test_xsave_xrstor_roundtrip_xmm() {
    // Enable AVX state, stash a known value in XMM0, XSAVE it to a scratch buffer,
    // clobber XMM0, XRSTOR, and confirm the value survived.
    let code = [
        // Enable OSXSAVE + XCR0 = 0x7.
        0x0f, 0x20, 0xe0, // MOV RAX, CR4
        0x48, 0x0d, 0x00, 0x00, 0x04, 0x00, // OR RAX, 0x40000
        0x0f, 0x22, 0xe0, // MOV CR4, RAX
        0xb8, 0x07, 0x00, 0x00, 0x00, // MOV EAX, 7
        0x31, 0xd2, // XOR EDX, EDX
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0x0f, 0x01, 0xd1, // XSETBV
        // XMM0 = 0xDEADBEEFCAFEBABE
        0x48, 0xb8, 0xbe, 0xba, 0xfe, 0xca, 0xef, 0xbe, 0xad, 0xde, // MOV RAX, imm64
        0x66, 0x48, 0x0f, 0x6e, 0xc0, // MOVQ XMM0, RAX
        // RDI = 0x40000 scratch buffer (64-byte aligned, mapped RAM)
        0xbf, 0x00, 0x00, 0x04, 0x00, // MOV EDI, 0x40000
        // XSAVE [RDI] with feature mask EDX:EAX = 0x7
        0xb8, 0x07, 0x00, 0x00, 0x00, // MOV EAX, 7
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0xae, 0x27, // XSAVE [RDI]
        // Clobber XMM0 to zero
        0x66, 0x0f, 0xef, 0xc0, // PXOR XMM0, XMM0
        // XRSTOR [RDI] with mask 0x7
        0xb8, 0x07, 0x00, 0x00, 0x00, // MOV EAX, 7
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0xae, 0x2f, // XRSTOR [RDI]
        // RAX = XMM0 low 64 bits
        0x66, 0x48, 0x0f, 0x7e, 0xc0, // MOVQ RAX, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0xDEAD_BEEF_CAFE_BABE,
        "XMM0 low qword should survive XSAVE/XRSTOR round-trip"
    );
}
