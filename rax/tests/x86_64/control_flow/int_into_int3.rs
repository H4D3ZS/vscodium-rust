use crate::common::{run_until_hlt, setup_vm, setup_vm_compat, setup_vm_no_idt};
use rax::cpu::{Registers, VCpu, VcpuExit};

// Comprehensive tests for INT, INTO, INT3 instructions (software interrupts)
// INT imm8 (CD), INTO (CE), INT3 (CC)

// ============================================================================
// INT3 - Breakpoint Interrupt (0xCC)
// ============================================================================

#[test]
fn test_int3_basic() {
    // INT3 - breakpoint interrupt (interrupt 3)
    let code = [
        0xcc, // INT3
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1 (fallback)
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    // INT3 should trigger interrupt or trap
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // If no interrupt handler, execution continues
    assert_eq!(regs.rax, 1);
}

#[test]
fn test_int3_preserves_registers() {
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x48, 0xc7, 0xc3, 0x99, 0x00, 0x00, 0x00, // MOV RBX, 0x99
        0xcc, // INT3
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x42);
    assert_eq!(regs.rbx, 0x99);
}

#[test]
fn test_int3_one_byte_encoding() {
    // INT3 is a single byte (0xCC) - more compact than INT 3
    let code = [
        0xcc, // INT3 (1 byte)
        0xcd, 0x03, // INT 3 (2 bytes - equivalent but longer)
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Both INT3 (0xCC) and INT 3 (0xCD 03) trap to vector 3; the IRETQ stub
    // returns to the next instruction, so execution reaches the final HLT.
    // RIP is past the HLT at offset 3 => 0x1000 + 3 + 1.
    assert_eq!(
        regs.rip,
        0x1000 + code.len() as u64,
        "reached final HLT past both INT3 forms"
    );
}

#[test]
fn test_int3_multiple_consecutive() {
    // Multiple INT3 instructions in sequence
    let code = [
        0xcc, // INT3
        0xcc, // INT3
        0xcc, // INT3
        0x48, 0xc7, 0xc0, 0xaa, 0x00, 0x00, 0x00, // MOV RAX, 0xAA
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xaa);
}

// ============================================================================
// INT imm8 - General Software Interrupt
// ============================================================================

#[test]
fn test_int_imm8_vector_0() {
    // INT 0 - divide error interrupt
    let code = [
        0xcd, 0x00, // INT 0
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 1);
}

#[test]
fn test_int_imm8_vector_1() {
    // INT 1 - debug exception
    let code = [
        0xcd, 0x01, // INT 1
        0x48, 0xc7, 0xc0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 2);
}

#[test]
fn test_int_imm8_vector_3() {
    // INT 3 - equivalent to INT3 but 2 bytes
    let code = [
        0xcd, 0x03, // INT 3
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 3);
}

#[test]
fn test_int_imm8_vector_4() {
    // INT 4 - overflow interrupt (INTO uses this)
    let code = [
        0xcd, 0x04, // INT 4
        0x48, 0xc7, 0xc0, 0x04, 0x00, 0x00, 0x00, // MOV RAX, 4
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 4);
}

#[test]
fn test_int_imm8_vector_13() {
    // INT 0x0D - general protection fault
    let code = [
        0xcd, 0x0d, // INT 13
        0x48, 0xc7, 0xc0, 0x0d, 0x00, 0x00, 0x00, // MOV RAX, 0x0D
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x0d);
}

#[test]
fn test_int_imm8_vector_14() {
    // INT 0x0E - page fault
    let code = [
        0xcd, 0x0e, // INT 14
        0x48, 0xc7, 0xc0, 0x0e, 0x00, 0x00, 0x00, // MOV RAX, 0x0E
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x0e);
}

#[test]
fn test_int_imm8_vector_16() {
    // INT 0x10 - x87 FPU error
    let code = [
        0xcd, 0x10, // INT 16
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 0x10
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x10);
}

#[test]
fn test_int_imm8_vector_21h() {
    // INT 0x21 - DOS service interrupt
    let code = [
        0xcd, 0x21, // INT 0x21
        0x48, 0xc7, 0xc0, 0x21, 0x00, 0x00, 0x00, // MOV RAX, 0x21
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x21);
}

#[test]
fn test_int_imm8_vector_80h() {
    // INT 0x80 - Linux system call interrupt (32-bit)
    let code = [
        0xcd, 0x80, // INT 0x80
        0x48, 0xc7, 0xc0, 0x80, 0x00, 0x00, 0x00, // MOV RAX, 0x80
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x80);
}

#[test]
fn test_int_imm8_vector_255() {
    // INT 0xFF - maximum vector number
    let code = [
        0xcd, 0xff, // INT 255
        0x48, 0xc7, 0xc0, 0xff, 0x00, 0x00, 0x00, // MOV RAX, 0xFF
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xff);
}

// ============================================================================
// INT - Stack Behavior
// ============================================================================

#[test]
fn test_int_pushes_flags_cs_ip() {
    // INT should push FLAGS, CS, and IP onto stack
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x80, 0x00, 0x00, // MOV RSP, 0x8000
        0xcd, 0x20, // INT 0x20
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Stack should have been modified if interrupt executed
}

#[test]
fn test_int_stack_alignment() {
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x80, 0x00, 0x00, // MOV RSP, 0x8000
        0x48, 0x89, 0xe0, // MOV RAX, RSP (save initial)
        0xcd, 0x30, // INT 0x30
        0x48, 0x89, 0xe3, // MOV RBX, RSP (save after)
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x8000);
}

// ============================================================================
// INTO - Interrupt on Overflow
// ============================================================================

#[test]
fn test_into_overflow_flag_clear() {
    // INTO when OF=0 should not interrupt
    // INTO is only valid in 32-bit/compatibility mode
    let code = [
        0x66, 0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 1
        0x66, 0x83, 0xc0, 0x01, // ADD EAX, 1 (no overflow, OF=0)
        0xce, // INTO (should not trigger)
        0x66, 0xbb, 0x42, 0x00, 0x00, 0x00, // MOV EBX, 0x42
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 2);
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x42); // Execution continued
}

#[test]
fn test_into_overflow_flag_set() {
    // INTO when OF=1 should trigger interrupt 4
    // INTO is only valid in 32-bit/compatibility mode
    let code = [
        0x66, 0xb8, 0xff, 0xff, 0xff, 0x7f, // MOV EAX, 0x7FFFFFFF (max positive 32-bit)
        0x66, 0x83, 0xc0, 0x01, // ADD EAX, 1 (overflow, OF=1)
        0xce, // INTO (should trigger INT 4)
        0x66, 0xbb, 0x99, 0x00, 0x00, 0x00, // MOV EBX, 0x99
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // INTO with OF=1 traps to vector 4; the IDT stub IRETQs back to the
    // instruction after INTO, so EBX=0x99 is reached and EAX wrapped to 0x80000000.
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x80000000,
        "0x7FFFFFFF + 1 overflowed"
    );
    assert_eq!(
        regs.rbx & 0xFFFFFFFF,
        0x99,
        "execution resumed after INTO trap"
    );
}

#[test]
fn test_into_after_addition_no_overflow() {
    // INTO is only valid in 32-bit/compatibility mode
    let code = [
        0x66, 0xb8, 0x10, 0x00, 0x00, 0x00, // MOV EAX, 16
        0x66, 0x83, 0xc0, 0x10, // ADD EAX, 16 (no overflow)
        0xce, // INTO
        0x66, 0xb9, 0xaa, 0x00, 0x00, 0x00, // MOV ECX, 0xAA
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 32);
    assert_eq!(regs.rcx & 0xFFFFFFFF, 0xaa);
}

#[test]
fn test_into_after_subtraction_no_overflow() {
    // INTO is only valid in 32-bit/compatibility mode
    let code = [
        0x66, 0xb8, 0x20, 0x00, 0x00, 0x00, // MOV EAX, 32
        0x66, 0x83, 0xe8, 0x10, // SUB EAX, 16 (no overflow)
        0xce, // INTO
        0x66, 0xba, 0xbb, 0x00, 0x00, 0x00, // MOV EDX, 0xBB
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 16);
    assert_eq!(regs.rdx & 0xFFFFFFFF, 0xbb);
}

#[test]
fn test_into_after_signed_overflow() {
    // Signed overflow: adding two large positive numbers
    // INTO is only valid in 32-bit/compatibility mode
    let code = [
        0x66, 0xb8, 0x00, 0x00, 0x00, 0x40, // MOV EAX, 0x40000000
        0x66, 0x05, 0x00, 0x00, 0x00, 0x40, // ADD EAX, 0x40000000 (overflow in signed)
        0xce, // INTO
        0x66, 0xbb, 0xcc, 0x00, 0x00, 0x00, // MOV EBX, 0xCC
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // May trigger interrupt 4 if OF set
}

#[test]
fn test_into_after_multiplication_overflow() {
    // Multiplication that causes overflow
    // INTO is only valid in 32-bit/compatibility mode
    let code = [
        0x66, 0xb8, 0x00, 0x00, 0x00, 0x80, // MOV EAX, 0x80000000
        0x66, 0xf7, 0xe8, // IMUL EAX (EAX * EAX, likely overflow)
        0xce, // INTO
        0x66, 0xbb, 0xdd, 0x00, 0x00, 0x00, // MOV EBX, 0xDD
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Behavior depends on OF flag state
}

// ============================================================================
// INT - Privilege Level Checks
// ============================================================================

#[test]
fn test_int_from_cpl0() {
    // INT from ring 0 (highest privilege)
    let code = [
        0xcd, 0x30, // INT 0x30
        0x48, 0xc7, 0xc0, 0x30, 0x00, 0x00, 0x00, // MOV RAX, 0x30
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x30);
}

#[test]
fn test_int_user_defined_vectors() {
    // User-defined interrupt vectors (32-255)
    let code = [
        0xcd, 0x40, // INT 0x40
        0xcd, 0x50, // INT 0x50
        0xcd, 0x60, // INT 0x60
        0x48, 0xc7, 0xc0, 0x60, 0x00, 0x00, 0x00, // MOV RAX, 0x60
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x60);
}

// ============================================================================
// INT - Reserved and Special Vectors
// ============================================================================

#[test]
fn test_int_divide_error_vector() {
    // INT 0 - divide error (normally triggered by DIV)
    let code = [
        0xcd, 0x00, // INT 0
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0);
}

#[test]
fn test_int_nmi_vector() {
    // INT 2 - NMI (non-maskable interrupt)
    let code = [
        0xcd, 0x02, // INT 2
        0x48, 0xc7, 0xc0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 2);
}

#[test]
fn test_int_bound_range_exceeded() {
    // INT 5 - BOUND range exceeded
    let code = [
        0xcd, 0x05, // INT 5
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 5);
}

#[test]
fn test_int_invalid_opcode() {
    // INT 6 - invalid opcode
    let code = [
        0xcd, 0x06, // INT 6
        0x48, 0xc7, 0xc0, 0x06, 0x00, 0x00, 0x00, // MOV RAX, 6
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 6);
}

#[test]
fn test_int_device_not_available() {
    // INT 7 - device not available (coprocessor)
    let code = [
        0xcd, 0x07, // INT 7
        0x48, 0xc7, 0xc0, 0x07, 0x00, 0x00, 0x00, // MOV RAX, 7
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 7);
}

#[test]
fn test_int_double_fault() {
    // INT 8 - double fault
    let code = [
        0xcd, 0x08, // INT 8
        0x48, 0xc7, 0xc0, 0x08, 0x00, 0x00, 0x00, // MOV RAX, 8
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 8);
}

#[test]
fn test_int_invalid_tss() {
    // INT 10 - invalid TSS
    let code = [
        0xcd, 0x0a, // INT 10
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 10);
}

#[test]
fn test_int_segment_not_present() {
    // INT 11 - segment not present
    let code = [
        0xcd, 0x0b, // INT 11
        0x48, 0xc7, 0xc0, 0x0b, 0x00, 0x00, 0x00, // MOV RAX, 11
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 11);
}

#[test]
fn test_int_stack_segment_fault() {
    // INT 12 - stack segment fault
    let code = [
        0xcd, 0x0c, // INT 12
        0x48, 0xc7, 0xc0, 0x0c, 0x00, 0x00, 0x00, // MOV RAX, 12
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 12);
}

#[test]
fn test_int_alignment_check() {
    // INT 17 - alignment check
    let code = [
        0xcd, 0x11, // INT 17
        0x48, 0xc7, 0xc0, 0x11, 0x00, 0x00, 0x00, // MOV RAX, 17
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 17);
}

#[test]
fn test_int_machine_check() {
    // INT 18 - machine check
    let code = [
        0xcd, 0x12, // INT 18
        0x48, 0xc7, 0xc0, 0x12, 0x00, 0x00, 0x00, // MOV RAX, 18
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 18);
}

#[test]
fn test_int_simd_floating_point() {
    // INT 19 - SIMD floating point exception
    let code = [
        0xcd, 0x13, // INT 19
        0x48, 0xc7, 0xc0, 0x13, 0x00, 0x00, 0x00, // MOV RAX, 19
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 19);
}

// ============================================================================
// INT - Sequential Interrupts
// ============================================================================

#[test]
fn test_int_multiple_different_vectors() {
    // Multiple different INT instructions
    let code = [
        0xcd, 0x30, // INT 0x30
        0xcd, 0x31, // INT 0x31
        0xcd, 0x32, // INT 0x32
        0x48, 0xc7, 0xc0, 0x32, 0x00, 0x00, 0x00, // MOV RAX, 0x32
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x32);
}

#[test]
fn test_int_same_vector_repeated() {
    // Same interrupt vector multiple times
    let code = [
        0xcd, 0x40, // INT 0x40
        0xcd, 0x40, // INT 0x40
        0xcd, 0x40, // INT 0x40
        0x48, 0xc7, 0xc0, 0x40, 0x00, 0x00, 0x00, // MOV RAX, 0x40
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x40);
}

// ============================================================================
// INTO - Edge Cases
// ============================================================================

#[test]
fn test_into_invalid_in_64bit_mode() {
    // INTO (0xCE) is INVALID in 64-bit mode and must raise #UD (vector 6).
    // It must NOT abort the emulator. We detect the injected fault using the
    // no-IDT harness (mirroring tests/x86_64/misc/ud.rs): with no IDT entries
    // populated, exception delivery fails fast instead of reaching HLT.
    let code = [
        0xce, // INTO (invalid in 64-bit)
        0x48, 0xc7, 0xc0, 0xff, 0x00, 0x00, 0x00, // MOV RAX, 0xFF (must not be reached)
        0xf4, // HLT (must not be reached)
    ];
    let (mut vcpu, _) = setup_vm_no_idt(&code, None);

    // The guest must not be able to kill the emulator: stepping the INTO must
    // not panic. It should inject #UD rather than reaching HLT.
    let result = vcpu.run();
    match result {
        Ok(VcpuExit::Hlt) => panic!("INTO in 64-bit mode must raise #UD, not reach HLT"),
        Ok(VcpuExit::Shutdown) => {} // #UD injected (no handler) -> shutdown
        Err(_) => {}                 // #UD injected, IDT entry not present -> Err (no abort)
        _ => {}                      // other non-HLT exit is acceptable
    }
}

// ============================================================================
// INT - Register Preservation
// ============================================================================

#[test]
fn test_int_preserves_all_registers() {
    let code = [
        0x48, 0xc7, 0xc0, 0x11, 0x11, 0x00, 0x00, // MOV RAX, 0x1111
        0x48, 0xc7, 0xc3, 0x22, 0x22, 0x00, 0x00, // MOV RBX, 0x2222
        0x48, 0xc7, 0xc1, 0x33, 0x33, 0x00, 0x00, // MOV RCX, 0x3333
        0x48, 0xc7, 0xc2, 0x44, 0x44, 0x00, 0x00, // MOV RDX, 0x4444
        0xcd, 0x50, // INT 0x50
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x1111);
    assert_eq!(regs.rbx, 0x2222);
    assert_eq!(regs.rcx, 0x3333);
    assert_eq!(regs.rdx, 0x4444);
}

// ============================================================================
// INT3 - Debugger Integration
// ============================================================================

#[test]
fn test_int3_debugger_breakpoint_pattern() {
    // Common pattern: INT3 for debugger breakpoints
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0xcc, // INT3 (breakpoint)
        0x48, 0xff, 0xc0, // INC RAX
        0xcc, // INT3 (another breakpoint)
        0x48, 0xff, 0xc0, // INC RAX
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 3);
}

#[test]
fn test_int3_code_patching() {
    // INT3 used for code patching (NOP replacement)
    let code = [
        0xcc, // INT3 (was NOP in original code)
        0x48, 0xc7, 0xc0, 0xab, 0xcd, 0x00, 0x00, // MOV RAX, 0xCDAB
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xcdab);
}
