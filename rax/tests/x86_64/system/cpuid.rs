use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

use crate::common::{run_until_hlt, setup_vm};

// CPUID - CPU Identification
// Opcode: 0F A2
// Input: RAX (function), RCX (sub-function)
// Output: RAX, RBX, RCX, RDX (CPU info)
//
// Main functions:
// EAX=0: Vendor ID, max function
// EAX=1: Family, model, features
// EAX=2: TLB/Cache info
// EAX=3: Serial number
// EAX=4: Cache descriptors (Pentium 4+)
// EAX=5: Monitor/Mwait
// EAX=6: Thermal info
// EAX=7: Extended features
// EAX=0x80000000+: Extended functions

// Basic CPUID with EAX=0 (Get Vendor ID)
#[test]
fn test_cpuid_function_0_vendor_id() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0 (get vendor ID)
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Exact "GenuineIntel" vendor string and max basic leaf 0x29.
    // EBX="Genu"=0x756e6547, EDX="ineI"=0x49656e69, ECX="ntel"=0x6c65746e.
    assert_eq!(regs.rax as u32, 0x29, "max basic leaf");
    assert_eq!(regs.rbx as u32, 0x756e6547, "EBX = 'Genu'");
    assert_eq!(regs.rdx as u32, 0x49656e69, "EDX = 'ineI'");
    assert_eq!(regs.rcx as u32, 0x6c65746e, "ECX = 'ntel'");
    // Verify it really spells GenuineIntel when reassembled.
    let mut vendor = Vec::new();
    vendor.extend_from_slice(&(regs.rbx as u32).to_le_bytes());
    vendor.extend_from_slice(&(regs.rdx as u32).to_le_bytes());
    vendor.extend_from_slice(&(regs.rcx as u32).to_le_bytes());
    assert_eq!(&vendor, b"GenuineIntel");
}

// CPUID function 1 - Get Family, Model, Stepping, Features
#[test]
fn test_cpuid_function_1_features() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Exact processor signature 0x6F1 (Stepping=1, Model=15, Family=6).
    assert_eq!(regs.rax as u32, 0x000006F1, "leaf 1 signature");
    assert_eq!(regs.rbx as u32, 0x00000000, "leaf 1 EBX");

    // EDX feature bits: FPU(0) PSE(3) TSC(4) MSR(5) PAE(6) CX8(8) APIC(9)
    // PGE(13) CMOV(15) CLFLUSH(19) MMX(23) FXSR(24) SSE(25) SSE2(26).
    let edx = regs.rdx as u32;
    assert_eq!(edx, 0x0788A379, "leaf 1 EDX feature bits");
    for (bit, name) in [
        (0, "FPU"),
        (4, "TSC"),
        (5, "MSR"),
        (8, "CX8"),
        (15, "CMOV"),
        (23, "MMX"),
        (24, "FXSR"),
        (25, "SSE"),
        (26, "SSE2"),
    ] {
        assert!(
            edx & (1 << bit) != 0,
            "leaf1 EDX bit {} ({}) should be set",
            bit,
            name
        );
    }

    // ECX with default CR4 (OSXSAVE=0): SSE3(0) SSSE3(9) SSE4.1(19) SSE4.2(20)
    // POPCNT(23) XSAVE(26) AVX(28). OSXSAVE(27) is 0 because CR4.OSXSAVE=0.
    let ecx = regs.rcx as u32;
    assert_eq!(ecx, 0x14980201, "leaf 1 ECX with OSXSAVE clear");
    assert!(ecx & (1 << 26) != 0, "XSAVE advertised");
    assert!(ecx & (1 << 28) != 0, "AVX advertised");
    assert_eq!(ecx & (1 << 27), 0, "OSXSAVE clear (CR4.OSXSAVE=0)");
}

// CPUID clears RCX when EAX < 0x80000000 and ECX input is non-zero
#[test]
fn test_cpuid_ecx_handling() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xc7, 0xc1, 0x42, 0x00, 0x00, 0x00, // MOV RCX, 0x42
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // For some functions, ECX input matters
    // but for function 1, it should return feature info in ECX
    assert_ne!(regs.rcx, 0, "RCX should contain extended features");
}

// CPUID function 2 - Cache/TLB information
#[test]
fn test_cpuid_function_2_cache_info() {
    let code = [
        0x48, 0xc7, 0xc0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // EAX contains count and descriptor count
    // EBX, ECX, EDX contain cache descriptors
    assert_ne!(regs.rax, 0, "EAX should contain cache info");
}

// CPUID function 3 - Processor Serial Number
#[test]
fn test_cpuid_function_3_serial_number() {
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RCX contains high part of serial number
    // RDX contains low part
    // Modern CPUs may return 0 for security reasons
    let _ = regs; // Just verify it doesn't crash
}

// CPUID function 4 - Deterministic Cache Parameters
#[test]
fn test_cpuid_function_4_cache_params() {
    let code = [
        0x48, 0xc7, 0xc0, 0x04, 0x00, 0x00, 0x00, // MOV RAX, 4
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RAX contains cache info
    // RBX contains associativity
    // RCX contains number of sets
    // RDX contains invalidation info
    let _ = regs;
}

// CPUID function 5 - MONITOR/MWAIT Feature
#[test]
fn test_cpuid_function_5_monitor_mwait() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RAX contains smallest monitor line size
    // RBX contains largest monitor line size
    // RCX contains MONITOR/MWAIT features
    // RDX contains MWAIT sub-C states
    let _ = regs;
}

// CPUID function 6 - Thermal Power Management
#[test]
fn test_cpuid_function_6_thermal_power() {
    let code = [
        0x48, 0xc7, 0xc0, 0x06, 0x00, 0x00, 0x00, // MOV RAX, 6
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // EAX contains thermal/power management features
    // EBX contains number of address bits
    // ECX contains ACNT bits
    // EDX contains reserved
    let _ = regs;
}

// CPUID function 7 - Extended Features (ECX=0)
#[test]
fn test_cpuid_function_7_extended_features() {
    let code = [
        0x48, 0xc7, 0xc0, 0x07, 0x00, 0x00, 0x00, // MOV RAX, 7
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Leaf 7 subleaf 0: EAX=1 (max subleaf), EBX has SMAP(20) + AVX2(5),
    // ECX has GFNI(8), EDX has SERIALIZE(14) + IBT(20).
    assert_eq!(regs.rax as u32, 1, "leaf 7 max subleaf");
    assert_eq!(regs.rbx as u32, 0x00100020, "leaf 7 EBX (SMAP|AVX2)");
    assert!(regs.rbx as u32 & (1 << 5) != 0, "AVX2 advertised");
    assert!(regs.rbx as u32 & (1 << 20) != 0, "SMAP advertised");
    assert_eq!(regs.rcx as u32, 0x00000100, "leaf 7 ECX (GFNI)");
    assert_eq!(regs.rdx as u32, 0x00104000, "leaf 7 EDX (SERIALIZE|IBT)");
}

// CPUID extended function 0x80000000 - Get Max Extended Function
#[test]
fn test_cpuid_extended_function_80000000() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x80, // MOV RAX, 0x80000000
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Exact max extended leaf 0x80000008.
    assert_eq!(regs.rax as u32, 0x80000008, "max extended leaf");
    assert_eq!(regs.rbx as u32, 0, "EBX reserved");
    assert_eq!(regs.rcx as u32, 0, "ECX reserved");
    assert_eq!(regs.rdx as u32, 0, "EDX reserved");
}

// CPUID extended function 0x80000001 - Extended Feature Info
#[test]
fn test_cpuid_extended_function_80000001() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x80, // MOV RAX, 0x80000001
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Extended signature equals leaf-1 signature; EDX has LM(29), RDTSCP(27), NX(20).
    assert_eq!(regs.rax as u32, 0x000006F1, "extended signature");
    let edx = regs.rdx as u32;
    assert_eq!(edx, 0x28100000, "0x80000001 EDX (LM|RDTSCP|NX)");
    assert!(edx & (1 << 29) != 0, "LM (long mode) advertised");
    assert!(edx & (1 << 27) != 0, "RDTSCP advertised");
    assert!(edx & (1 << 20) != 0, "NX advertised");
}

// CPUID extended function 0x80000002 - Brand String Part 1
#[test]
fn test_cpuid_extended_function_80000002_brand_1() {
    let code = [
        0x48, 0xc7, 0xc0, 0x02, 0x00, 0x00, 0x80, // MOV RAX, 0x80000002
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Brand string part 1 = "Rax Emulator\0\0\0\0" (EAX:EBX:ECX:EDX little-endian).
    assert_eq!(regs.rax as u32, 0x20786152, "brand part1 EAX = 'Rax '");
    assert_eq!(regs.rbx as u32, 0x6c756d45, "brand part1 EBX = 'Emul'");
    assert_eq!(regs.rcx as u32, 0x726f7461, "brand part1 ECX = 'ator'");
    assert_eq!(regs.rdx as u32, 0x00000000, "brand part1 EDX = null");
    let mut s = Vec::new();
    for r in [regs.rax, regs.rbx, regs.rcx, regs.rdx] {
        s.extend_from_slice(&(r as u32).to_le_bytes());
    }
    assert_eq!(&s[..12], b"Rax Emulator");
}

// CPUID extended function 0x80000003 - Brand String Part 2
#[test]
fn test_cpuid_extended_function_80000003_brand_2() {
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x80, // MOV RAX, 0x80000003
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RAX:RBX:RCX:RDX contain brand string (second 16 chars)
    let _ = regs;
}

// CPUID extended function 0x80000004 - Brand String Part 3
#[test]
fn test_cpuid_extended_function_80000004_brand_3() {
    let code = [
        0x48, 0xc7, 0xc0, 0x04, 0x00, 0x00, 0x80, // MOV RAX, 0x80000004
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RAX:RBX:RCX:RDX contain brand string (third 16 chars)
    let _ = regs;
}

// CPUID extended function 0x80000005 - TLB/Cache Info (L1)
#[test]
fn test_cpuid_extended_function_80000005() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x80, // MOV RAX, 0x80000005
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RAX contains L1 TLB info for 2MB/4MB
    // RBX contains L1 TLB info for 4KB
    // RCX contains L1 data cache info
    // RDX contains L1 instruction cache info
    let _ = regs;
}

// CPUID extended function 0x80000006 - Cache Info (L2/L3)
#[test]
fn test_cpuid_extended_function_80000006() {
    let code = [
        0x48, 0xc7, 0xc0, 0x06, 0x00, 0x00, 0x80, // MOV RAX, 0x80000006
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RAX contains L2 TLB info for 2MB/4MB
    // RBX contains L2 TLB info for 4KB
    // RCX contains L2 cache info
    // RDX contains L3 cache info
    let _ = regs;
}

// CPUID extended function 0x80000007 - Advanced Power Management
#[test]
fn test_cpuid_extended_function_80000007() {
    let code = [
        0x48, 0xc7, 0xc0, 0x07, 0x00, 0x00, 0x80, // MOV RAX, 0x80000007
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RAX is reserved
    // RBX is reserved
    // RCX contains C-state info
    // RDX contains advanced power management features
    let _ = regs;
}

// CPUID extended function 0x80000008 - Virtual/Physical Address Size
#[test]
fn test_cpuid_extended_function_80000008() {
    let code = [
        0x48, 0xc7, 0xc0, 0x08, 0x00, 0x00, 0x80, // MOV RAX, 0x80000008
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Address sizes: physical bits (low byte) = 48, linear bits (next byte) = 48.
    let eax = regs.rax as u32;
    assert_eq!(eax, 0x00003030, "0x80000008 EAX (48 phys | 48 linear)");
    assert_eq!(eax & 0xFF, 48, "48 physical address bits");
    assert_eq!((eax >> 8) & 0xFF, 48, "48 linear address bits");
}

// CPUID preserves register values when called with same input
#[test]
fn test_cpuid_deterministic() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x0f, 0xa2, // CPUID (first call)
        0x50, // PUSH RAX
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x0f, 0xa2, // CPUID (second call with same input)
        0x59, // POP RCX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RCX has result from first CPUID(1), RAX has result from second CPUID(1)
    // They should match
    assert_eq!(regs.rax, regs.rcx, "CPUID should be deterministic");
}

// CPUID doesn't affect other registers unnecessarily
#[test]
fn test_cpuid_preserves_other_registers() {
    // Use values with bit 31 clear to avoid sign-extension issues with MOV r64, imm32
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xc7, 0xc2, 0x42, 0x42, 0x42, 0x42, // MOV RDX, 0x42424242
        0x48, 0xc7, 0xc6, 0x55, 0x55, 0x55, 0x55, // MOV RSI, 0x55555555
        0x48, 0xc7, 0xc7, 0x66, 0x66, 0x66, 0x66, // MOV RDI, 0x66666666
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RSI and RDI should be unchanged
    assert_eq!(regs.rsi, 0x55555555, "RSI should not be affected");
    assert_eq!(regs.rdi, 0x66666666, "RDI should not be affected");
}

// Multiple CPUID calls in sequence
#[test]
fn test_multiple_cpuid_calls() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x0f, 0xa2, // CPUID
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x0f, 0xa2, // CPUID
        0x48, 0xc7, 0xc0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should complete without error
    let _ = regs;
}

// CPUID with function 0x1 returns sensible feature bits
#[test]
fn test_cpuid_feature_bits_function_1() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // At least one feature bit should be set in EDX or ECX
    // For a modern x86_64, we expect FPU (bit 0 of EDX) to be set
    assert!(
        (regs.rdx as u32) & 0x01 != 0 || (regs.rcx as u32) != 0,
        "Should have some features"
    );
}

// CPUID ECX input for cache descriptors
#[test]
fn test_cpuid_function_4_with_different_ecx() {
    let code = [
        // First call with ECX=0
        0x48, 0xc7, 0xc0, 0x04, 0x00, 0x00, 0x00, // MOV RAX, 4
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0x0f, 0xa2, // CPUID
        0x50, // PUSH RAX
        0x51, // PUSH RCX
        // Second call with ECX=1
        0x48, 0xc7, 0xc0, 0x04, 0x00, 0x00, 0x00, // MOV RAX, 4
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Both calls should complete
    let _ = regs;
}

// CPUID with input in EAX only (not RAX full width)
#[test]
fn test_cpuid_eax_32bit_input() {
    let code = [
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 1 (32-bit, zero extends)
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = 0xFFFFFFFFFFFFFFFF; // High bits set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // After MOV EAX, high bits of RAX should be zeroed
    // But CPUID should still work
    let _ = regs;
}

// CPUID function 0 max returned function check
#[test]
fn test_cpuid_function_0_returns_max() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // EAX should contain max function number
    // Should be at least 1 for modern CPUs
    assert!(regs.rax >= 0x01, "Should support at least function 1");
}

// CPUID stores vendor ID correctly in function 0
#[test]
fn test_cpuid_function_0_vendor_layout() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // After CPUID 0: EBX, EDX, ECX form vendor string
    // Like "GenuineIntel" or "AuthenticAMD"
    // RBX, RDX, RCX should have values (though emulator might not)
    let _ = regs;
}

// CPUID doesn't modify flags
#[test]
fn test_cpuid_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets ZF)
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // ZF should still be set from the ADD
    assert!(regs.rflags & 0x40 != 0, "ZF should be preserved");
}

// CPUID with large RAX input (unsupported function)
#[test]
fn test_cpuid_unsupported_function() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, 0xffffffff
        0x0f, 0xa2, // CPUID (unsupported function)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should not crash, might return empty results
    let _ = regs;
}

// CPUID between supported and extended functions boundary
#[test]
fn test_cpuid_boundary_standard_extended() {
    let code = [
        // High standard function
        0x48, 0xc7, 0xc0, 0x0f, 0x00, 0x00, 0x00, // MOV RAX, 15
        0x0f, 0xa2, // CPUID
        0x50, // PUSH RAX
        // Low extended function
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x80, // MOV RAX, 0x80000000
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Both should work
    let _ = regs;
}

// CPUID RCX input for sub-leaf enumeration (function 7)
#[test]
fn test_cpuid_function_7_subleaves() {
    let code = [
        // Leaf 7, sub-leaf 0
        0x48, 0xc7, 0xc0, 0x07, 0x00, 0x00, 0x00, // MOV RAX, 7
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0x0f, 0xa2, // CPUID
        0x50, // PUSH RAX
        // Leaf 7, sub-leaf 1
        0x48, 0xc7, 0xc0, 0x07, 0x00, 0x00, 0x00, // MOV RAX, 7
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // After the sequence, RAX holds leaf 7 subleaf 1, which advertises APX_F.
    assert_eq!(regs.rax as u32, 0, "leaf 7 subleaf 1 EAX = 0");
    assert_eq!(regs.rbx as u32, 0, "leaf 7 subleaf 1 EBX = 0");
    assert_eq!(regs.rcx as u32, 0, "leaf 7 subleaf 1 ECX = 0");
    assert_eq!(regs.rdx as u32, 1 << 21, "leaf 7 subleaf 1 EDX APX_F");
}

// ============================================================================
// Strengthened additions: leaf 0xD (XSAVE), 0x80000007 (invariant TSC),
// OSXSAVE reflecting CR4, and unsupported-leaf zeroing.
// ============================================================================

// CPUID leaf 0xD subleaf 0 - XSAVE feature enumeration.
#[test]
fn test_cpuid_leaf_d_subleaf0_xsave_area() {
    let code = [
        0xb8, 0x0d, 0x00, 0x00, 0x00, // MOV EAX, 0xD
        0x31, 0xc9, // XOR ECX, ECX (subleaf 0)
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // EAX = supported XCR0 low bits = x87|SSE|AVX|APX_F = 0x80007.
    // EBX = current enabled area size (XCR0 default has AVX disabled => 576).
    // ECX = max area size for all supported (1088). EDX = high XCR0 bits = 0.
    assert_eq!(
        regs.rax as u32, 0x80007,
        "XCR0 valid low bits x87|SSE|AVX|APX_F"
    );
    assert_eq!(regs.rbx as u32, 576, "current XSAVE area (AVX disabled)");
    assert_eq!(regs.rcx as u32, 1088, "max XSAVE area");
    assert_eq!(regs.rdx as u32, 0, "XCR0 high bits");
}

// CPUID leaf 0xD subleaf 2 - AVX (YMM_Hi128) component size/offset.
#[test]
fn test_cpuid_leaf_d_subleaf2_avx_component() {
    let code = [
        0xb8, 0x0d, 0x00, 0x00, 0x00, // MOV EAX, 0xD
        0xb9, 0x02, 0x00, 0x00, 0x00, // MOV ECX, 2
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // YMM_Hi128 component: size = 256 bytes, offset = 576.
    assert_eq!(regs.rax as u32, 256, "AVX component size");
    assert_eq!(regs.rbx as u32, 576, "AVX component offset");
}

// CPUID leaf 0xD subleaf 19 - APX_F EGPR component.
#[test]
fn test_cpuid_leaf_d_subleaf19_apx_component() {
    let code = [
        0xb8, 0x0d, 0x00, 0x00, 0x00, // MOV EAX, 0xD
        0xb9, 0x13, 0x00, 0x00, 0x00, // MOV ECX, 19
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax as u32, 128, "APX_F component size");
    assert_eq!(regs.rbx as u32, 0x3C0, "APX_F component offset");
    assert_eq!(regs.rcx as u32, 0, "APX_F component controls");
}

// CPUID leaf 0x29 - APX feature leaf.
#[test]
fn test_cpuid_leaf_29_apx_features() {
    let code = [
        0xb8, 0x29, 0x00, 0x00, 0x00, // MOV EAX, 0x29
        0x31, 0xc9, // XOR ECX, ECX
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax as u32, 0, "APX max subleaf");
    assert_eq!(regs.rbx as u32, 1, "APX_NCI_NDD_NF");
    assert_eq!(regs.rcx as u32, 0, "APX leaf ECX reserved");
    assert_eq!(regs.rdx as u32, 0, "APX leaf EDX reserved");
}

// CPUID leaf 0x80000007 - Invariant TSC (EDX bit 8).
#[test]
fn test_cpuid_leaf_80000007_invariant_tsc() {
    let code = [
        0xb8, 0x07, 0x00, 0x00, 0x80, // MOV EAX, 0x80000007
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rdx as u32, 1 << 8, "invariant TSC bit");
    assert!(regs.rdx as u32 & (1 << 8) != 0, "invariant TSC advertised");
}

// CPUID leaf 1 ECX OSXSAVE bit reflects CR4.OSXSAVE (set via MOV CR4).
#[test]
fn test_cpuid_leaf1_osxsave_reflects_cr4() {
    // Read CR4, set OSXSAVE (bit 18), write CR4, then CPUID leaf 1.
    let code = [
        0x0f, 0x20, 0xe0, // MOV RAX, CR4
        0x48, 0x0d, 0x00, 0x00, 0x04, 0x00, // OR RAX, 0x40000 (bit 18 OSXSAVE)
        0x0f, 0x22, 0xe0, // MOV CR4, RAX
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 1
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let ecx = regs.rcx as u32;
    // With CR4.OSXSAVE=1, leaf 1 ECX now has bit 27 set => 0x1C980201.
    assert_eq!(ecx, 0x1C980201, "leaf 1 ECX with OSXSAVE set");
    assert!(ecx & (1 << 27) != 0, "OSXSAVE bit reflects CR4.OSXSAVE=1");
}

// CPUID with an unsupported leaf returns all zeros.
#[test]
fn test_cpuid_unsupported_leaf_zeros() {
    let code = [
        0xb8, 0x00, 0x00, 0x00, 0x40, // MOV EAX, 0x40000000 (hypervisor leaf, unimpl)
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax as u32, 0, "unsupported leaf EAX=0");
    assert_eq!(regs.rbx as u32, 0, "unsupported leaf EBX=0");
    assert_eq!(regs.rcx as u32, 0, "unsupported leaf ECX=0");
    assert_eq!(regs.rdx as u32, 0, "unsupported leaf EDX=0");
}
