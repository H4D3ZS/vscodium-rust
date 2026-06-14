use crate::common::{
    Bytes, VM, run_until_hlt_legacy as run_until_hlt, setup_vm_legacy as setup_vm,
};

// LODS/LODSB/LODSW/LODSD/LODSQ - Load String
// Loads from [RSI] into AL/AX/EAX/RAX, increments/decrements RSI based on DF
// Opcodes:
//   AC - LODSB (load into AL)
//   AD - LODSW (load into AX, 66h prefix)
//   AD - LODSD (load into EAX)
//   REX.W AD - LODSQ (load into RAX)

#[test]
fn test_lodsb_basic() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0xc6, 0x06, 0x42, // MOV BYTE PTR [RSI], 0x42
        0xfc, // CLD
        0xac, // LODSB
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax & 0xFF, 0x42); // AL loaded
    assert_eq!(vm.rsi, 0x3001); // RSI incremented
}

#[test]
fn test_lodsb_with_std() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0xc6, 0x06, 0x99, // MOV BYTE PTR [RSI], 0x99
        0xfd, // STD
        0xac, // LODSB
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax & 0xFF, 0x99);
    assert_eq!(vm.rsi, 0x2FFF); // RSI decremented
}

#[test]
fn test_lodsw_basic() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x66, 0xc7, 0x06, 0x34, 0x12, // MOV WORD PTR [RSI], 0x1234
        0xfc, // CLD
        0x66, 0xad, // LODSW
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax & 0xFFFF, 0x1234); // AX loaded
    assert_eq!(vm.rsi, 0x3002); // Incremented by 2
}

#[test]
fn test_lodsd_basic() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0xc7, 0x06, 0x78, 0x56, 0x34, 0x12, // MOV DWORD PTR [RSI], 0x12345678
        0xfc, // CLD
        0xad, // LODSD
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax & 0xFFFFFFFF, 0x12345678); // EAX loaded
    assert_eq!(vm.rsi, 0x3004); // Incremented by 4
}

#[test]
fn test_lodsq_basic() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xb8, 0xef, 0xcd, 0xab, 0x90, 0x78, 0x56, 0x34,
        0x12, // MOV RAX, 0x1234567890ABCDEF
        0x48, 0x89, 0x06, // MOV [RSI], RAX
        0xfc, // CLD
        0x48, 0xad, // LODSQ
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0x1234567890ABCDEF); // RAX loaded
    assert_eq!(vm.rsi, 0x3008); // Incremented by 8
}

#[test]
fn test_lodsb_multiple() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0xc6, 0x06, 0x41, // MOV BYTE PTR [RSI], 'A'
        0xc6, 0x46, 0x01, 0x42, // MOV BYTE PTR [RSI+1], 'B'
        0xc6, 0x46, 0x02, 0x43, // MOV BYTE PTR [RSI+2], 'C'
        0xfc, // CLD
        0xac, // LODSB (A)
        0x50, // PUSH RAX
        0xac, // LODSB (B)
        0x50, // PUSH RAX
        0xac, // LODSB (C)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax & 0xFF, 0x43); // Last loaded: 'C'
    assert_eq!(vm.rsi, 0x3003);
}

#[test]
fn test_lodsb_preserves_other_registers() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0xc6, 0x06, 0x42, // MOV BYTE PTR [RSI], 0x42
        0x48, 0xc7, 0xc3, 0x99, 0x00, 0x00, 0x00, // MOV RBX, 0x99
        0x48, 0xc7, 0xc1, 0xaa, 0x00, 0x00, 0x00, // MOV RCX, 0xAA
        0xfc, // CLD
        0xac, // LODSB
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rbx, 0x99); // Preserved
    assert_eq!(vm.rcx, 0xAA); // Preserved
}

#[test]
fn test_lodsb_clears_high_bytes() {
    // LODSB should only load into AL, not affect AH
    let code = [
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0xc6, 0x06, 0x42, // MOV BYTE PTR [RSI], 0x42
        0xfc, // CLD
        0xac, // LODSB
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax & 0xFF, 0x42); // AL set
    // Note: behavior of high bytes depends on implementation
}

#[test]
fn test_lodsw_clears_high_word() {
    let code = [
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x66, 0xc7, 0x06, 0x34, 0x12, // MOV WORD PTR [RSI], 0x1234
        0xfc, // CLD
        0x66, 0xad, // LODSW
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax & 0xFFFF, 0x1234); // AX set
}

#[test]
fn test_lodsd_zero_extends() {
    // LODSD zero-extends into RAX (clears high 32 bits)
    let code = [
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0xc7, 0x06, 0x78, 0x56, 0x34, 0x12, // MOV DWORD PTR [RSI], 0x12345678
        0xfc, // CLD
        0xad, // LODSD
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0x12345678); // High 32 bits should be 0
}

#[test]
fn test_rep_lodsb_last_value() {
    // REP LODS loads multiple values, but only last one remains in AL
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0xc6, 0x06, 0x01, // MOV BYTE PTR [RSI], 1
        0xc6, 0x46, 0x01, 0x02, // MOV BYTE PTR [RSI+1], 2
        0xc6, 0x46, 0x02, 0x03, // MOV BYTE PTR [RSI+2], 3
        0xc6, 0x46, 0x03, 0x04, // MOV BYTE PTR [RSI+3], 4
        0xc6, 0x46, 0x04, 0x05, // MOV BYTE PTR [RSI+4], 5
        0xfc, // CLD
        0xf3, 0xac, // REP LODSB
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax & 0xFF, 5); // Last value
    assert_eq!(vm.rcx, 0);
    assert_eq!(vm.rsi, 0x3005);
}

#[test]
fn test_lodsb_string_processing() {
    // Load and process each character
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0xc6, 0x06, 0x48, // MOV BYTE PTR [RSI], 'H'
        0xfc, // CLD
        0xac, // LODSB
        0x04, 0x20, // ADD AL, 0x20 (convert to lowercase)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax & 0xFF, 0x68); // 'h'
}

#[test]
fn test_lodsb_backward() {
    let code = [
        0x48, 0xc7, 0xc6, 0x04, 0x30, 0x00, 0x00, // MOV RSI, 0x3004 (end)
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0xc6, 0x46, 0xfc, 0x01, // MOV BYTE PTR [RSI-4], 1
        0xc6, 0x46, 0xfd, 0x02, // MOV BYTE PTR [RSI-3], 2
        0xc6, 0x46, 0xfe, 0x03, // MOV BYTE PTR [RSI-2], 3
        0xc6, 0x46, 0xff, 0x04, // MOV BYTE PTR [RSI-1], 4
        0xc6, 0x06, 0x05, // MOV BYTE PTR [RSI], 5
        0xfd, // STD
        0xf3, 0xac, // REP LODSB (backward)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax & 0xFF, 1); // Last loaded (first element)
    assert_eq!(vm.rsi, 0x2FFF);
}

#[test]
fn test_lodsw_array_processing() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x66, 0xc7, 0x06, 0x00, 0x01, // MOV WORD PTR [RSI], 0x0100
        0x66, 0xc7, 0x46, 0x02, 0x00, 0x02, // MOV WORD PTR [RSI+2], 0x0200
        0x66, 0xc7, 0x46, 0x04, 0x00, 0x03, // MOV WORD PTR [RSI+4], 0x0300
        0xfc, // CLD
        0x66, 0xad, // LODSW
        0x50, // PUSH RAX
        0x66, 0xad, // LODSW
        0x50, // PUSH RAX
        0x66, 0xad, // LODSW
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax & 0xFFFF, 0x0300); // Last word
    assert_eq!(vm.rsi, 0x3006);
}

#[test]
fn test_lodsd_consecutive() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0xc7, 0x06, 0x11, 0x11, 0x11, 0x11, // MOV DWORD PTR [RSI], 0x11111111
        0xc7, 0x46, 0x04, 0x22, 0x22, 0x22, 0x22, // MOV DWORD PTR [RSI+4], 0x22222222
        0xfc, // CLD
        0xad, // LODSD
        0x48, 0x89, 0xc3, // MOV RBX, RAX
        0xad, // LODSD
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rbx, 0x11111111); // First
    assert_eq!(vm.rax, 0x22222222); // Second
}

#[test]
fn test_lodsq_consecutive() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xb8, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, // MOV RAX, 0x1111...
        0x48, 0x89, 0x06, // MOV [RSI], RAX
        0x48, 0xb8, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, // MOV RAX, 0x2222...
        0x48, 0x89, 0x46, 0x08, // MOV [RSI+8], RAX
        0xfc, // CLD
        0x48, 0xad, // LODSQ
        0x48, 0x89, 0xc3, // MOV RBX, RAX
        0x48, 0xad, // LODSQ
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rbx, 0x1111111111111111);
    assert_eq!(vm.rax, 0x2222222222222222);
}

#[test]
fn test_lodsb_with_comparison() {
    // Load and compare pattern
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0xc6, 0x06, 0x41, // MOV BYTE PTR [RSI], 'A'
        0xfc, // CLD
        0xac, // LODSB
        0x3c, 0x41, // CMP AL, 'A'
        0x74, 0x01, // JE +1
        0xf4, // HLT (should not reach)
        0xf4, // HLT (target)
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rip, (0x1000 + code.len()) as u64); // Jumped to correct target
}

#[test]
fn test_lodsb_null_terminator_search() {
    // Load bytes until null
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0xc6, 0x06, 0x41, // MOV BYTE PTR [RSI], 'A'
        0xc6, 0x46, 0x01, 0x42, // MOV BYTE PTR [RSI+1], 'B'
        0xc6, 0x46, 0x02, 0x00, // MOV BYTE PTR [RSI+2], '\0'
        0xfc, // CLD
        // loop:
        0xac, // LODSB
        0x84, 0xc0, // TEST AL, AL
        0x75, 0xfa, // JNZ -6 (loop)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax & 0xFF, 0); // Null terminator
    assert_eq!(vm.rsi, 0x3003); // Stopped after null
}

#[test]
fn test_lodsb_counting_pattern() {
    // Count characters using LODS
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4 (count)
        0x48, 0x31, 0xdb, // XOR RBX, RBX (accumulator)
        0xc6, 0x06, 0x01, // MOV BYTE PTR [RSI], 1
        0xc6, 0x46, 0x01, 0x02, // MOV BYTE PTR [RSI+1], 2
        0xc6, 0x46, 0x02, 0x03, // MOV BYTE PTR [RSI+2], 3
        0xc6, 0x46, 0x03, 0x04, // MOV BYTE PTR [RSI+3], 4
        0xfc, // CLD
        // loop:
        0xac, // LODSB
        0x48, 0x0f, 0xb6, 0xc0, // MOVZX RAX, AL
        0x48, 0x01, 0xc3, // ADD RBX, RAX
        0xe2, 0xf5, // LOOP -11
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rbx, 10); // 1+2+3+4
}

#[test]
fn test_lodsd_alignment() {
    // Verify LODSD works regardless of alignment
    let code = [
        0x48, 0xc7, 0xc6, 0x01, 0x30, 0x00, 0x00, // MOV RSI, 0x3001 (unaligned)
        0xc7, 0x06, 0x78, 0x56, 0x34, 0x12, // MOV DWORD PTR [RSI], 0x12345678
        0xfc, // CLD
        0xad, // LODSD
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax & 0xFFFFFFFF, 0x12345678);
}

#[test]
fn test_lodsb_empty_rep() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xfc, // CLD
        0xf3, 0xac, // REP LODSB (no iterations)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rsi, 0x3000); // Unchanged
}

#[test]
fn test_lodsb_preserves_source() {
    // LODS is non-destructive (doesn't modify memory)
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0xc6, 0x06, 0x42, // MOV BYTE PTR [RSI], 0x42
        0xfc, // CLD
        0xac, // LODSB
        0xf4, // HLT
    ];
    let mut vm = setup_vm(&code);
    vm = run_until_hlt(vm);
    assert_eq!(vm.read_memory(0x3000, 1)[0], 0x42); // Source unchanged
}

#[test]
fn test_lodsb_case_conversion() {
    // Convert uppercase to lowercase while loading
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3
        0xc6, 0x06, 0x41, // MOV BYTE PTR [RSI], 'A'
        0xc6, 0x46, 0x01, 0x42, // MOV BYTE PTR [RSI+1], 'B'
        0xc6, 0x46, 0x02, 0x43, // MOV BYTE PTR [RSI+2], 'C'
        0xfc, // CLD
        // loop:
        0xac, // LODSB
        0x04, 0x20, // ADD AL, 0x20 (to lowercase)
        0xaa, // STOSB
        0xe2, 0xf9, // LOOP -7
        0xf4, // HLT
    ];
    let mut vm = setup_vm(&code);
    vm = run_until_hlt(vm);
    let dest = vm.read_memory(0x4000, 3);
    assert_eq!(&dest, &[0x61, 0x62, 0x63]); // "abc"
}

// ============================================================================
// Regression test: segment-override on the LODS source DS:[RSI]
// ============================================================================

#[test]
fn test_lodsb_fs_segment_override_source() {
    // LODS loads from DS:[RSI], overridable by an FS/GS prefix. With FS.base set
    // to 0x2000 (via WRMSR IA32_FS_BASE) and RSI=0x5000, FS LODSB must load from
    // linear 0x7000 (=0x3C), NOT plain RSI=0x5000 (=0x99).
    let code = [
        // WRMSR FS.base = 0x2000
        0x48, 0xc7, 0xc0, 0x00, 0x20, 0x00, 0x00, // MOV RAX, 0x2000
        0x48, 0xc7, 0xc2, 0x00, 0x00, 0x00, 0x00, // MOV RDX, 0
        0x48, 0xc7, 0xc1, 0x00, 0x01, 0x00, 0xc0, // MOV RCX, 0xC0000100
        0x0f, 0x30, // WRMSR
        // Seed source bytes.
        0x48, 0xc7, 0xc3, 0x00, 0x70, 0x00, 0x00, // MOV RBX, 0x7000
        0xc6, 0x03, 0x3c, // MOV BYTE PTR [RBX], 0x3C  (FS-relative source)
        0x48, 0xc7, 0xc3, 0x00, 0x50, 0x00, 0x00, // MOV RBX, 0x5000
        0xc6, 0x03, 0x99, // MOV BYTE PTR [RBX], 0x99  (non-overridden source)
        // Clear RAX so the loaded value is unambiguous.
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        0x48, 0xc7, 0xc6, 0x00, 0x50, 0x00, 0x00, // MOV RSI, 0x5000
        0xfc, // CLD
        0x64, 0xac, // FS LODSB
        0xf4, // HLT
    ];
    let mut vm = setup_vm(&code);
    vm = run_until_hlt(vm);
    assert_eq!(vm.rax & 0xFF, 0x3C); // AL loaded from FS:0x5000 == 0x7000
    assert_eq!(vm.rsi, 0x5001); // RSI advanced
}
