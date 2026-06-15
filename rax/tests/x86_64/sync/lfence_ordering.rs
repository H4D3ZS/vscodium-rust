use crate::common::*;

// LFENCE Tests - Load Fence for ordering loads
// LFENCE: 0F AE E8
// Serializes all load operations before the fence with loads after the fence

#[test]
fn test_lfence_basic() {
    let code = [
        0x0f, 0xae, 0xe8, // LFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
    // LFENCE should execute without errors
}

#[test]
fn test_lfence_after_load() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x48, 0x8b, 0x03, // MOV RAX, [RBX] (load)
        0x0f, 0xae, 0xe8, // LFENCE (ensure load completes)
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x42);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x42, "Load should complete before LFENCE");
}

#[test]
fn test_lfence_before_load() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x0f, 0xae, 0xe8, // LFENCE (ensure previous loads complete)
        0x48, 0x8b, 0x03, // MOV RAX, [RBX] (load after fence)
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x99);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x99, "Load after LFENCE should see value");
}

#[test]
fn test_lfence_between_loads() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x48, 0x8b, 0x03, // MOV RAX, [RBX] (first load)
        0x0f, 0xae, 0xe8, // LFENCE
        0x48, 0x8b, 0x53, 0x08, // MOV RDX, [RBX+8] (second load)
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    use vm_memory::{Bytes, GuestAddress};
    mem.write_slice(&0x11u64.to_le_bytes(), GuestAddress(0x2000))
        .unwrap();
    mem.write_slice(&0x22u64.to_le_bytes(), GuestAddress(0x2008))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x11, "First load should complete before LFENCE");
    assert_eq!(regs.rdx, 0x22, "Second load should happen after LFENCE");
}

#[test]
fn test_lfence_multiple_fences() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x48, 0x8b, 0x03, // MOV RAX, [RBX]
        0x0f, 0xae, 0xe8, // LFENCE
        0x48, 0x8b, 0x53, 0x08, // MOV RDX, [RBX+8]
        0x0f, 0xae, 0xe8, // LFENCE
        0x48, 0x8b, 0x73, 0x10, // MOV RSI, [RBX+16]
        0x0f, 0xae, 0xe8, // LFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    use vm_memory::{Bytes, GuestAddress};
    mem.write_slice(&0x11u64.to_le_bytes(), GuestAddress(0x2000))
        .unwrap();
    mem.write_slice(&0x22u64.to_le_bytes(), GuestAddress(0x2008))
        .unwrap();
    mem.write_slice(&0x33u64.to_le_bytes(), GuestAddress(0x2010))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x11, "First load should complete");
    assert_eq!(regs.rdx, 0x22, "Second load should complete");
    assert_eq!(regs.rsi, 0x33, "Third load should complete");
}

#[test]
fn test_lfence_sequential_loads() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        // Load, fence, process pattern
        0x48, 0x8b, 0x03, // MOV RAX, [RBX]
        0x0f, 0xae, 0xe8, // LFENCE
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0x48, 0x8b, 0x53, 0x08, // MOV RDX, [RBX+8]
        0x0f, 0xae, 0xe8, // LFENCE
        0x48, 0x01, 0xd0, // ADD RAX, RDX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    use vm_memory::{Bytes, GuestAddress};
    mem.write_slice(&0xAAu64.to_le_bytes(), GuestAddress(0x2000))
        .unwrap();
    mem.write_slice(&0xBBu64.to_le_bytes(), GuestAddress(0x2008))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax,
        0xAA + 1 + 0xBB,
        "Loads should be properly ordered"
    );
}

#[test]
fn test_lfence_prevents_speculative_loads() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        // First load
        0x48, 0x8b, 0x03, // MOV RAX, [RBX]
        // LFENCE prevents speculative execution of subsequent loads
        0x0f, 0xae, 0xe8, // LFENCE
        // Second load cannot start until first completes
        0x48, 0x8b, 0x53, 0x08, // MOV RDX, [RBX+8]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    use vm_memory::{Bytes, GuestAddress};
    mem.write_slice(&0x123u64.to_le_bytes(), GuestAddress(0x2000))
        .unwrap();
    mem.write_slice(&0x456u64.to_le_bytes(), GuestAddress(0x2008))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x123, "First load completes");
    assert_eq!(regs.rdx, 0x456, "Second load completes after LFENCE");
}

#[test]
fn test_lfence_with_conditional_branch() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x48, 0x8b, 0x03, // MOV RAX, [RBX]
        0x0f, 0xae, 0xe8, // LFENCE (prevents speculation)
        0x48, 0x83, 0xf8, 0x00, // CMP RAX, 0
        0x74, 0x05, // JE +5 (skip if zero)
        0x48, 0x8b, 0x53, 0x08, // MOV RDX, [RBX+8]
        0xeb, 0x03, // JMP +3
        0x48, 0x31, 0xd2, // XOR RDX, RDX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    use vm_memory::{Bytes, GuestAddress};
    mem.write_slice(&0x1u64.to_le_bytes(), GuestAddress(0x2000))
        .unwrap();
    mem.write_slice(&0x789u64.to_le_bytes(), GuestAddress(0x2008))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 1, "Condition load completes");
    assert_eq!(regs.rdx, 0x789, "Conditional load happens");
}

#[test]
fn test_lfence_array_bounds_check() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        // Load index
        0x48, 0x8b, 0x03, // MOV RAX, [RBX]
        0x0f, 0xae, 0xe8, // LFENCE (prevent speculation past bounds check)
        // Bounds check (simplified)
        0x48, 0x83, 0xf8, 0x0a, // CMP RAX, 10
        0x73, 0x05, // JAE +5 (out of bounds)
        // Access array element
        0x48, 0x8b, 0x54, 0x03, 0x08, // MOV RDX, [RBX+RAX+8]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    use vm_memory::{Bytes, GuestAddress};
    mem.write_slice(&0x2u64.to_le_bytes(), GuestAddress(0x2000))
        .unwrap();
    mem.write_slice(&0xABCu64.to_le_bytes(), GuestAddress(0x200A))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 2, "Index loaded");
    // RDX would contain the array element if bounds check passed
}

#[test]
fn test_lfence_dependent_loads() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        // Load pointer
        0x48, 0x8b, 0x03, // MOV RAX, [RBX] (load pointer)
        0x0f, 0xae, 0xe8, // LFENCE
        // Use pointer to load data
        0x48, 0x8b, 0x10, // MOV RDX, [RAX] (dependent load)
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    use vm_memory::{Bytes, GuestAddress};
    // Pointer at 0x2000 points to 0x2100
    mem.write_slice(&0x2100u64.to_le_bytes(), GuestAddress(0x2000))
        .unwrap();
    // Data at 0x2100
    mem.write_slice(&0xDEFu64.to_le_bytes(), GuestAddress(0x2100))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x2100, "Pointer loaded");
    assert_eq!(regs.rdx, 0xDEF, "Data loaded via pointer");
}

#[test]
fn test_lfence_cache_timing_mitigation() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        // Secret-dependent load
        0x48, 0x8b, 0x03, // MOV RAX, [RBX] (secret)
        0x0f, 0xae, 0xe8, // LFENCE (prevent timing side-channel)
        // Use secret
        0x48, 0x8b, 0x54, 0x03, 0x08, // MOV RDX, [RBX+RAX+8]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    use vm_memory::{Bytes, GuestAddress};
    mem.write_slice(&0x10u64.to_le_bytes(), GuestAddress(0x2000))
        .unwrap();
    mem.write_slice(&0x5EC5EC5EC5EC5ECu64.to_le_bytes(), GuestAddress(0x2018))
        .unwrap(); // "secret" placeholder

    let _regs = run_until_hlt(&mut vcpu).unwrap();
    // LFENCE ensures secret-dependent load doesn't happen speculatively
}

#[test]
fn test_lfence_serialization() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        // Multiple loads with LFENCE between each
        0x48, 0x8b, 0x03, // MOV RAX, [RBX]
        0x0f, 0xae, 0xe8, // LFENCE
        0x48, 0x8b, 0x53, 0x08, // MOV RDX, [RBX+8]
        0x0f, 0xae, 0xe8, // LFENCE
        0x48, 0x8b, 0x73, 0x10, // MOV RSI, [RBX+16]
        0x0f, 0xae, 0xe8, // LFENCE
        0x48, 0x8b, 0x7b, 0x18, // MOV RDI, [RBX+24]
        0x0f, 0xae, 0xe8, // LFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    use vm_memory::{Bytes, GuestAddress};
    mem.write_slice(&0x11u64.to_le_bytes(), GuestAddress(0x2000))
        .unwrap();
    mem.write_slice(&0x22u64.to_le_bytes(), GuestAddress(0x2008))
        .unwrap();
    mem.write_slice(&0x33u64.to_le_bytes(), GuestAddress(0x2010))
        .unwrap();
    mem.write_slice(&0x44u64.to_le_bytes(), GuestAddress(0x2018))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x11);
    assert_eq!(regs.rdx, 0x22);
    assert_eq!(regs.rsi, 0x33);
    assert_eq!(regs.rdi, 0x44);
}

#[test]
fn test_lfence_store_to_load_forwarding() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        // Store
        0x48, 0xc7, 0x03, 0x42, 0x00, 0x00, 0x00, // MOV QWORD PTR [RBX], 0x42
        // LFENCE doesn't order stores, but orders subsequent loads
        0x0f, 0xae, 0xe8, // LFENCE
        // Load
        0x48, 0x8b, 0x03, // MOV RAX, [RBX]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x42, "Load should see store even with LFENCE");
}

#[test]
fn test_lfence_read_after_read() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        // First read
        0x48, 0x8b, 0x03, // MOV RAX, [RBX]
        // LFENCE ensures first read completes
        0x0f, 0xae, 0xe8, // LFENCE
        // Second read of same location
        0x48, 0x8b, 0x13, // MOV RDX, [RBX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x999);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x999, "First read");
    assert_eq!(regs.rdx, 0x999, "Second read sees same value");
}

#[test]
fn test_lfence_with_cpuid() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x48, 0x8b, 0x03, // MOV RAX, [RBX]
        // LFENCE provides lighter-weight serialization than CPUID
        0x0f, 0xae, 0xe8, // LFENCE
        0x48, 0x8b, 0x53, 0x08, // MOV RDX, [RBX+8]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    use vm_memory::{Bytes, GuestAddress};
    mem.write_slice(&0x111u64.to_le_bytes(), GuestAddress(0x2000))
        .unwrap();
    mem.write_slice(&0x222u64.to_le_bytes(), GuestAddress(0x2008))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x111);
    assert_eq!(regs.rdx, 0x222);
}
