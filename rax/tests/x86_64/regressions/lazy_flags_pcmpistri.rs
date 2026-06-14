//! Regression tests for lazy flags bug with PCMPISTRI instruction.
//!
//! Bug: PCMPISTRI (and other SSE instructions that set flags directly) was not
//! clearing the lazy flags state before setting flags. When a subsequent Jcc
//! instruction called materialize_flags(), the lazy flags from a previous
//! instruction would overwrite the flags set by PCMPISTRI, causing incorrect
//! conditional jumps.
//!
//! This bug manifested in busybox ash shell where variable expansion inside
//! double quotes failed:
//!   x=ABC; echo "$x"  # Output was garbage instead of "ABC"
//!
//! The strpbrk() function uses PCMPISTRI with imm8=0x02 (equal any, signed bytes)
//! to find special characters. When a match was found (CF=1), the JB instruction
//! should have jumped, but the CF flag was being overwritten by lazy flag
//! materialization.
//!
//! Fix: Call clear_lazy_flags() before setting flags directly in pcmpxstrx().

use crate::common::*;

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// Core Regression Test: PCMPISTRI followed by JB
// ============================================================================

/// Test PCMPISTRI setting CF=1 followed by JB (Jump if Below/Carry)
/// This is the exact pattern that was broken in busybox strpbrk.
#[test]
fn test_pcmpistri_cf_followed_by_jb_match() {
    // Code sequence:
    //   MOV RAX, ALIGNED_ADDR
    //   MOVDQA XMM0, [RAX]      ; charset
    //   MOVDQA XMM1, [RAX+0x10] ; string to search
    //   PCMPISTRI XMM0, XMM1, 0x02  ; equal any, signed bytes
    //   JB .match               ; should jump if CF=1 (match found)
    //   MOV RBX, 0              ; no match path
    //   JMP .done
    // .match:
    //   MOV RBX, 1              ; match path
    // .done:
    //   HLT
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x48, 0x10, // MOVDQA XMM1, [RAX+0x10]
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x02, // PCMPISTRI XMM0, XMM1, 0x02
        0x72, 0x09, // JB +9 (.match) - skip MOV RBX,0 (7) + JMP (2)
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0
        0xeb, 0x07, // JMP +7 (.done) - skip MOV RBX,1 (7)
        // .match:
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        // .done:
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    // XMM0 (charset): contains characters to find, including 0x82
    // This simulates the CTLVAR charset from busybox: [0x87, 0x83, 0x81, 0x82, ...]
    let charset: [u8; 16] = [
        0x87, 0x83, 0x81, 0x82, 0x84, 0x88, 0x89, 0x85, 0x86, 0x00, 0, 0, 0, 0, 0, 0,
    ];

    // XMM1 (string): starts with 0x82 (should match at index 0)
    // This simulates the shell variable token: [CTLVAR, flags, 'x', '=', ...]
    let string: [u8; 16] = [
        0x82, 0x01, 0x78, 0x3d, 0x87, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    mem.write_slice(&charset, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&string, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();

    let regs = vcpu.get_regs().unwrap();

    // RBX should be 1 (match path taken)
    assert_eq!(
        regs.rbx, 1,
        "JB should have jumped because CF=1 (match found at index 0)"
    );

    // ECX should be 0 (index of first match)
    assert_eq!(
        regs.rcx, 0,
        "PCMPISTRI should return index 0 (first byte matches)"
    );
}

/// Test PCMPISTRI setting CF=0 followed by JB (should NOT jump)
#[test]
fn test_pcmpistri_cf_followed_by_jb_no_match() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x02,
        0x72, 0x09, // JB +9 (.match) - skip MOV RBX,0 (7) + JMP (2)
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0
        0xeb, 0x07, // JMP +7 (.done) - skip MOV RBX,1 (7)
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    // Charset: "abc\0"
    let charset: [u8; 16] = [0x61, 0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // String: "xyz\0" - no characters match
    let string: [u8; 16] = [0x78, 0x79, 0x7a, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    mem.write_slice(&charset, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&string, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();

    let regs = vcpu.get_regs().unwrap();

    // RBX should be 0 (no match path taken)
    assert_eq!(
        regs.rbx, 0,
        "JB should NOT have jumped because CF=0 (no match)"
    );

    // ECX should be 16 (no match found)
    assert_eq!(regs.rcx, 16, "PCMPISTRI should return 16 (no match)");
}

// ============================================================================
// Lazy Flags Interaction Tests
// ============================================================================

/// Test that an ADD before PCMPISTRI doesn't corrupt PCMPISTRI's flags
/// ADD sets lazy flags, PCMPISTRI should clear them before setting CF directly
#[test]
fn test_add_before_pcmpistri_jb() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10,
        // ADD that sets lazy flags (this would corrupt CF if not cleared)
        0x48, 0x83, 0xc2, 0x01, // ADD RDX, 1
        // PCMPISTRI should clear lazy flags before setting CF
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x02, 0x72,
        0x09, // JB +9 (.match) - skip MOV RBX,0 (7) + JMP (2)
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, 0xeb,
        0x07, // JMP +7 (.done) - skip MOV RBX,1 (7)
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    // Set RDX to a value that will NOT set CF on ADD
    let mut regs = vcpu.get_regs().unwrap();
    regs.rdx = 0;
    vcpu.set_regs(&regs).unwrap();

    // Match case: 'b' in charset, string starts with 'b'
    let charset: [u8; 16] = [0x61, 0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let string: [u8; 16] = [0x62, 0x78, 0x79, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    mem.write_slice(&charset, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&string, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(
        regs.rbx, 1,
        "JB should jump - ADD's lazy flags must not corrupt PCMPISTRI's CF"
    );
}

/// Test that a SUB before PCMPISTRI doesn't corrupt PCMPISTRI's flags
/// SUB sets lazy flags with CF based on borrow, PCMPISTRI should override
#[test]
fn test_sub_before_pcmpistri_jb() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10,
        // SUB that sets lazy flags with CF=1 (borrow)
        0x48, 0x83, 0xea, 0x01, // SUB RDX, 1
        // PCMPISTRI with NO match - should set CF=0
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x02, 0x72,
        0x09, // JB +9 (.match) - skip MOV RBX,0 (7) + JMP (2)
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, 0xeb,
        0x07, // JMP +7 (.done) - skip MOV RBX,1 (7)
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    // Set RDX to 0 so SUB 1 causes borrow (would set CF=1 in lazy flags)
    let mut regs = vcpu.get_regs().unwrap();
    regs.rdx = 0;
    vcpu.set_regs(&regs).unwrap();

    // No match case
    let charset: [u8; 16] = [0x61, 0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let string: [u8; 16] = [0x78, 0x79, 0x7a, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    mem.write_slice(&charset, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&string, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(
        regs.rbx, 0,
        "JB should NOT jump - PCMPISTRI's CF=0 must override SUB's lazy CF=1"
    );
}

/// Test that a CMP before PCMPISTRI doesn't corrupt PCMPISTRI's flags
#[test]
fn test_cmp_before_pcmpistri_jb() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10,
        // CMP that sets lazy flags with CF=1 (src > dst)
        0x48, 0x83, 0xfa, 0x10, // CMP RDX, 0x10
        // PCMPISTRI with match - should set CF=1
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x02, 0x72,
        0x09, // JB +9 (.match) - skip MOV RBX,0 (7) + JMP (2)
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, 0xeb,
        0x07, // JMP +7 (.done) - skip MOV RBX,1 (7)
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    // Set RDX to 0 so CMP 0x10 would set CF=1 in lazy flags
    let mut regs = vcpu.get_regs().unwrap();
    regs.rdx = 0;
    vcpu.set_regs(&regs).unwrap();

    // Match case
    let charset: [u8; 16] = [0x61, 0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let string: [u8; 16] = [0x62, 0x78, 0x79, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    mem.write_slice(&charset, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&string, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(
        regs.rbx, 1,
        "JB should jump - PCMPISTRI's CF=1 must be used, not CMP's lazy flags"
    );
}

// ============================================================================
// Other Jcc Instructions with PCMPISTRI
// ============================================================================

/// Test PCMPISTRI followed by JNB (Jump if Not Below) - opposite of JB
#[test]
fn test_pcmpistri_cf_followed_by_jnb_no_match() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x02,
        0x73, 0x09, // JNB +9 (jump if CF=0) - skip MOV RBX,0 (7) + JMP (2)
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, 0xeb, 0x07, // JMP +7 - skip MOV RBX,1 (7)
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    // No match case - CF=0
    let charset: [u8; 16] = [0x61, 0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let string: [u8; 16] = [0x78, 0x79, 0x7a, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    mem.write_slice(&charset, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&string, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rbx, 1, "JNB should jump because CF=0 (no match)");
}

/// Test PCMPISTRI followed by JZ (uses ZF, not CF)
#[test]
fn test_pcmpistri_zf_followed_by_jz() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x02,
        0x74, 0x09, // JZ +9 (jump if ZF=1) - skip MOV RBX,0 (7) + JMP (2)
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, 0xeb, 0x07, // JMP +7 - skip MOV RBX,1 (7)
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    // String with null terminator before end - ZF=1
    let charset: [u8; 16] = [0x61, 0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let string: [u8; 16] = [0x78, 0x79, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    mem.write_slice(&charset, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&string, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();

    let regs = vcpu.get_regs().unwrap();
    // ZF=1 when string (XMM1/src2) contains null before 16 bytes
    assert_eq!(regs.rbx, 1, "JZ should jump because ZF=1 (null in string)");
}

// ============================================================================
// COMISS/COMISD Lazy Flags Tests
// ============================================================================

/// Test COMISS followed by JB (uses CF from floating-point comparison)
#[test]
fn test_comiss_cf_followed_by_jb() {
    // COMISS sets CF=1 when src > dst (unordered or less than)
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX] (dst = 1.0)
        0x0f, 0x28, 0x48, 0x10, // MOVAPS XMM1, [RAX+0x10] (src = 2.0)
        // ADD to set lazy flags
        0x48, 0x83, 0xc2, 0x00, // ADD RDX, 0 (sets lazy flags, CF=0)
        0x0f, 0x2f, 0xc1, // COMISS XMM0, XMM1 (1.0 < 2.0, sets CF=1)
        0x72, 0x09, // JB +9 - skip MOV RBX,0 (7) + JMP (2)
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, 0xeb, 0x07, // JMP +7 - skip MOV RBX,1 (7)
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    // XMM0: 1.0f
    let val1: f32 = 1.0;
    let mut data1 = [0u8; 16];
    data1[0..4].copy_from_slice(&val1.to_le_bytes());

    // XMM1: 2.0f
    let val2: f32 = 2.0;
    let mut data2 = [0u8; 16];
    data2[0..4].copy_from_slice(&val2.to_le_bytes());

    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(
        regs.rbx, 1,
        "JB should jump because COMISS set CF=1 (1.0 < 2.0)"
    );
}

// ============================================================================
// Multiple PCMPISTRI in Sequence
// ============================================================================

/// Test multiple PCMPISTRI instructions in sequence (like a loop)
#[test]
fn test_multiple_pcmpistri_sequence() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX] (charset)
        0x66, 0x0f, 0x6f, 0x48, 0x10, // MOVDQA XMM1, [RAX+0x10] (string1 - no match)
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x02, // PCMPISTRI XMM0, XMM1, 0x02
        0x72, 0x04, // JB +4 (skip inc if match)
        0x48, 0xff, 0xc3, // INC RBX
        0x90, // NOP (alignment)
        0x66, 0x0f, 0x6f, 0x48, 0x20, // MOVDQA XMM1, [RAX+0x20] (string2 - match)
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x02, // PCMPISTRI XMM0, XMM1, 0x02
        0x72, 0x04, // JB +4 (skip inc if match)
        0x48, 0xff, 0xc3, // INC RBX
        0x90, // NOP
        0x66, 0x0f, 0x6f, 0x48, 0x30, // MOVDQA XMM1, [RAX+0x30] (string3 - no match)
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x02, // PCMPISTRI XMM0, XMM1, 0x02
        0x72, 0x04, // JB +4 (skip inc if match)
        0x48, 0xff, 0xc3, // INC RBX
        0x90, // NOP
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    // Initialize RBX to 0
    let mut regs = vcpu.get_regs().unwrap();
    regs.rbx = 0;
    vcpu.set_regs(&regs).unwrap();

    // Charset: "abc"
    let charset: [u8; 16] = [0x61, 0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // String1: "xyz" - no match (RBX++)
    let string1: [u8; 16] = [0x78, 0x79, 0x7a, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // String2: "xbz" - match at index 1 (RBX not incremented)
    let string2: [u8; 16] = [0x78, 0x62, 0x7a, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // String3: "123" - no match (RBX++)
    let string3: [u8; 16] = [0x31, 0x32, 0x33, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    mem.write_slice(&charset, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&string1, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    mem.write_slice(&string2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x20))
        .unwrap();
    mem.write_slice(&string3, vm_memory::GuestAddress(ALIGNED_ADDR + 0x30))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();

    let regs = vcpu.get_regs().unwrap();
    // RBX should be 2: incremented for string1 and string3, not for string2
    assert_eq!(regs.rbx, 2, "RBX should be 2 (no-match count)");
}

// ============================================================================
// Exact Busybox Scenario
// ============================================================================

/// Test the exact scenario from busybox strpbrk: imm8=0x02 with control chars
#[test]
fn test_busybox_strpbrk_scenario() {
    // This replicates the exact strpbrk pattern from busybox that was failing:
    // - Charset contains shell control characters (0x81-0x89)
    // - String starts with CTLVAR (0x82)
    // - PCMPISTRI should find match at index 0, set CF=1
    // - JB should jump
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX] (charset)
        0x66, 0x0f, 0x6f, 0x48, 0x10, // MOVDQA XMM1, [RAX+0x10] (string)
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x02, // PCMPISTRI XMM0, XMM1, 0x02
        0x72, 0x09, // JB +9 (.found) - skip MOV RBX,-1 (7) + JMP (2)
        0x48, 0xc7, 0xc3, 0xff, 0xff, 0xff, 0xff, // MOV RBX, -1 (not found)
        0xeb, 0x03, // JMP +3 (.done) - skip MOV RBX,RCX (3)
        // .found:
        0x48, 0x89, 0xcb, // MOV RBX, RCX (index)
        // .done:
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    // Exact charset from busybox: CTLQUOTEMARK, CTLESC, CTLVAR, etc.
    let charset: [u8; 16] = [
        0x87, 0x83, 0x81, 0x82, 0x84, 0x88, 0x89, 0x85, 0x86, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];

    // String from shell expansion: CTLVAR(0x82) + flags + "x" + "=" + CTLQUOTEMARK(0x87)
    let string: [u8; 16] = [
        0x82, 0x01, 0x78, 0x3d, 0x87, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];

    mem.write_slice(&charset, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&string, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();

    let regs = vcpu.get_regs().unwrap();

    // RBX should be 0 (found at index 0), not -1
    assert_eq!(regs.rbx, 0, "strpbrk should find CTLVAR (0x82) at index 0");
    assert_eq!(regs.rcx, 0, "PCMPISTRI should return index 0");
}
