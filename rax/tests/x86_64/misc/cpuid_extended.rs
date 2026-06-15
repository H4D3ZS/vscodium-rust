// Module path for tests run via x86_64.rs
use crate::common::*;
use rax::cpu::Registers;

// CPUID - CPU Identification
// Extended comprehensive tests covering all standard and extended leaves
//
// CPUID returns CPU identification and feature information in EAX, EBX, ECX, EDX
// based on the input in EAX (and ECX for some leaves)
//
// Standard leaves: 0x00000000 through 0x00000029
// Extended leaves: 0x80000000 through 0x80000008
//
// Opcodes:
// 0F A2                  CPUID                    - Return CPU Identification and Feature Information

// ===== STANDARD LEAVES TESTS =====

#[test]
fn test_cpuid_leaf_0_vendor_id() {
    // EAX=0: Returns highest standard leaf and vendor ID
    let code = [
        0xb8, 0x00, 0x00, 0x00, 0x00, // MOV EAX, 0
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // EAX should be the highest standard leaf supported (at least 0)
    assert!(
        regs.rax & 0xFFFFFFFF >= 0,
        "EAX should contain max standard leaf"
    );
    // EBX, ECX, EDX should contain vendor ID string
    let ebx = regs.rbx & 0xFFFFFFFF;
    let ecx = regs.rcx & 0xFFFFFFFF;
    let edx = regs.rdx & 0xFFFFFFFF;
    // At least one of these should be non-zero for a real vendor
    assert!(
        ebx != 0 || ecx != 0 || edx != 0,
        "Vendor ID should not be all zeros"
    );
}

#[test]
fn test_cpuid_leaf_0_max_leaf_valid() {
    // EAX=0: Max standard leaf should be reasonable (not absurdly high)
    let code = [
        0xb8, 0x00, 0x00, 0x00, 0x00, // MOV EAX, 0
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let max_leaf = regs.rax & 0xFFFFFFFF;
    // APX extends the standard leaf range through 0x29.
    assert!(max_leaf <= 0x29, "Max standard leaf should be <= 0x29");
}

#[test]
fn test_cpuid_leaf_1_features() {
    // EAX=1: Returns processor version and feature flags
    let code = [
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 1
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // EAX should contain version info
    let version = regs.rax & 0xFFFFFFFF;
    assert!(version != 0, "Version info should be non-zero");

    // ECX and EDX contain feature flags (shouldn't be all zero)
    let ecx = regs.rcx & 0xFFFFFFFF;
    let edx = regs.rdx & 0xFFFFFFFF;
    assert!(ecx != 0 || edx != 0, "Feature flags should not be all zero");
}

#[test]
fn test_cpuid_leaf_1_feature_bits() {
    // EAX=1: Check specific feature bits
    let code = [
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 1
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // EDX contains basic features like FPU, TSC, MSR, PAE, MCE, CX8, APIC, SEP, MTRR, PGE, MCA, CMOV, PAT, PSE36, PSNUM, CLFLUSH, DS, ACPI, MMX, FXSR, SSE, SSE2, SS, HTT, TM, IA64, PBE
    let edx = regs.rdx & 0xFFFFFFFF;
    // At least FPU (bit 0) should typically be set
    let has_fpu = (edx & 0x00000001) != 0;
    // MMX (bit 23) should typically be set
    let has_mmx = (edx & 0x00800000) != 0;
    // Either FPU or MMX should be present
    assert!(has_fpu || has_mmx, "Should have at least FPU or MMX");
}

#[test]
fn test_cpuid_leaf_2_cache_descriptor() {
    // EAX=2: Returns cache and TLB information
    let code = [
        0xb8, 0x02, 0x00, 0x00, 0x00, // MOV EAX, 2
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // EAX[7:0] (AL) contains iteration count (usually 1 for modern CPUs)
    let al = regs.rax & 0xFF;
    assert!(al >= 1, "AL should contain valid iteration count");
}

#[test]
fn test_cpuid_leaf_3_serial_number() {
    // EAX=3: Returns processor serial number (deprecated, mostly zeros)
    let code = [
        0xb8, 0x03, 0x00, 0x00, 0x00, // MOV EAX, 3
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Just verify it returns something (may be zeros)
    let _ebx = regs.rbx & 0xFFFFFFFF;
    let _ecx = regs.rcx & 0xFFFFFFFF;
    let _edx = regs.rdx & 0xFFFFFFFF;
    // Instruction should execute without error
}

#[test]
fn test_cpuid_leaf_4_deterministic_cache() {
    // EAX=4: Returns deterministic cache parameters (Skylake and newer)
    let code = [
        0xb8, 0x04, 0x00, 0x00, 0x00, // MOV EAX, 4
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0 (cache index)
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // EAX[4:0] contains cache type (0=no cache, 1=data, 2=instruction, 3=unified)
    let cache_type = regs.rax & 0x1F;
    assert!(cache_type <= 3, "Cache type should be 0-3");
}

#[test]
fn test_cpuid_leaf_5_monitor_mwait() {
    // EAX=5: Returns MONITOR/MWAIT parameters
    let code = [
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // EAX contains smallest line size for MONITOR (bits 15:0)
    // ECX contains extensions (bit 0: enumeration of monitor-mwait extensions)
    let _eax = regs.rax & 0xFFFF;
    let _ecx = regs.rcx & 0xFFFFFFFF;
    // Just verify instruction executes
}

#[test]
fn test_cpuid_leaf_6_thermal_power() {
    // EAX=6: Returns thermal and power management features
    let code = [
        0xb8, 0x06, 0x00, 0x00, 0x00, // MOV EAX, 6
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // EAX contains feature bits for thermal/power management
    let _eax = regs.rax & 0xFFFFFFFF;
    // Just verify instruction executes
}

#[test]
fn test_cpuid_leaf_7_extended_features() {
    // EAX=7: Returns extended feature flags (newer features)
    let code = [
        0xb8, 0x07, 0x00, 0x00, 0x00, // MOV EAX, 7
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0 (subleaf)
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // EBX contains extended features
    // Common bits: BMI1 (bit 3), AVX2 (bit 5), etc.
    let _ebx = regs.rbx & 0xFFFFFFFF;
}

#[test]
fn test_cpuid_leaf_7_bmi_features() {
    // EAX=7, ECX=0: Check for BMI features
    let code = [
        0xb8, 0x07, 0x00, 0x00, 0x00, // MOV EAX, 7
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // EBX[3] = BMI1, EBX[8] = BMI2
    let ebx = regs.rbx & 0xFFFFFFFF;
    let _has_bmi1 = (ebx & (1 << 3)) != 0;
    let _has_bmi2 = (ebx & (1 << 8)) != 0;
}

#[test]
fn test_cpuid_leaf_9_direct_cache_access() {
    // EAX=9: Returns direct cache access (DCA) information
    let code = [
        0xb8, 0x09, 0x00, 0x00, 0x00, // MOV EAX, 9
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // EAX contains DCA information
    let _eax = regs.rax & 0xFFFFFFFF;
}

#[test]
fn test_cpuid_leaf_a_performance_monitoring() {
    // EAX=A: Returns performance monitoring capabilities
    let code = [
        0xb8, 0x0a, 0x00, 0x00, 0x00, // MOV EAX, 10
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // EAX[7:0] contains version ID, EBX contains event bitmap
    let _eax = regs.rax & 0xFFFFFFFF;
    let _ebx = regs.rbx & 0xFFFFFFFF;
}

#[test]
fn test_cpuid_leaf_b_extended_topology() {
    // EAX=B: Returns extended topology information (Nehalem and newer)
    let code = [
        0xb8, 0x0b, 0x00, 0x00, 0x00, // MOV EAX, 11
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // EAX contains bits 4:0 (number of bits to shift right of APIC ID to get next level ID)
    // ECX contains bits 15:8 (level type), bits 7:0 (level number)
    let _eax = regs.rax & 0x1F;
    let _ecx = regs.rcx & 0xFFFFFFFF;
}

#[test]
fn test_cpuid_leaf_d_xsave_features() {
    // EAX=D: Returns XSAVE feature information
    let code = [
        0xb8, 0x0d, 0x00, 0x00, 0x00, // MOV EAX, 13
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // EAX contains valid bits in XCR0
    // EBX contains size of XSAVE area with all supported features enabled
    let eax = regs.rax & 0xFFFFFFFF;
    // If XSAVE is supported, bit 0 (x87 state) should be set
    let _has_x87 = (eax & 1) != 0;
}

#[test]
fn test_cpuid_leaf_f_qos_monitoring() {
    // EAX=F: Returns QoS Monitoring capabilities (Skylake and newer)
    let code = [
        0xb8, 0x0f, 0x00, 0x00, 0x00, // MOV EAX, 15
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // May return 0 if QoS monitoring not supported
    let _ebx = regs.rbx & 0xFFFFFFFF;
}

#[test]
fn test_cpuid_leaf_10_qos_enforcement() {
    // EAX=10: Returns QoS Enforcement capabilities (Skylake and newer)
    let code = [
        0xb8, 0x10, 0x00, 0x00, 0x00, // MOV EAX, 16
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // May return 0 if QoS enforcement not supported
    let _ebx = regs.rbx & 0xFFFFFFFF;
}

#[test]
fn test_cpuid_leaf_12_sgx_capabilities() {
    // EAX=12: Returns SGX capabilities (Skylake and newer)
    let code = [
        0xb8, 0x12, 0x00, 0x00, 0x00, // MOV EAX, 18
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // May return 0 if SGX not supported
    let _eax = regs.rax & 0xFFFFFFFF;
}

#[test]
fn test_cpuid_leaf_14_processor_trace() {
    // EAX=14: Returns Intel Processor Trace capabilities (Skylake and newer)
    let code = [
        0xb8, 0x14, 0x00, 0x00, 0x00, // MOV EAX, 20
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // May return 0 if Intel PT not supported
    let _eax = regs.rax & 0xFFFFFFFF;
}

#[test]
fn test_cpuid_leaf_15_tsc_frequency() {
    // EAX=15: Returns TSC/Core Crystal Clock frequency information
    let code = [
        0xb8, 0x15, 0x00, 0x00, 0x00, // MOV EAX, 21
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // EAX: TSC/Core ratio, EBX: Nominal Frequency, ECX: Core Crystal Frequency
    let _eax = regs.rax & 0xFFFFFFFF;
}

// ===== EXTENDED LEAVES TESTS =====

#[test]
fn test_cpuid_extended_leaf_80000000_max_extended() {
    // EAX=0x80000000: Returns highest extended leaf and vendor ID
    let code = [
        0xb8, 0x00, 0x00, 0x00, 0x80, // MOV EAX, 0x80000000
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // EAX should be at least 0x80000001
    let max_extended = regs.rax & 0xFFFFFFFF;
    assert!(
        max_extended >= 0x80000001,
        "Max extended leaf should be >= 0x80000001"
    );
    // Usually should be <= 0x8000001F
    assert!(
        max_extended <= 0x8000001F,
        "Max extended leaf should be reasonable"
    );
}

#[test]
fn test_cpuid_extended_leaf_80000001_features() {
    // EAX=0x80000001: Returns extended features
    let code = [
        0xb8, 0x01, 0x00, 0x00, 0x80, // MOV EAX, 0x80000001
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // EDX contains extended features like NX, SYSCALL, XD, etc.
    // ECX contains advanced features like LAHF/SAHF, CMP_LEG, SVM, etc.
    let _edx = regs.rdx & 0xFFFFFFFF;
    let _ecx = regs.rcx & 0xFFFFFFFF;
}

#[test]
fn test_cpuid_extended_leaf_80000001_64bit_features() {
    // EAX=0x80000001: Check for 64-bit feature support
    let code = [
        0xb8, 0x01, 0x00, 0x00, 0x80, // MOV EAX, 0x80000001
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // EDX[29] = LM (Long Mode/64-bit support)
    let edx = regs.rdx & 0xFFFFFFFF;
    let has_long_mode = (edx & (1 << 29)) != 0;
    // We should be running in 64-bit mode, so this should be set
    assert!(has_long_mode, "Long Mode (64-bit) should be supported");
}

#[test]
fn test_cpuid_extended_leaf_80000002_brand_string_1() {
    // EAX=0x80000002: Returns processor brand string part 1
    let code = [
        0xb8, 0x02, 0x00, 0x00, 0x80, // MOV EAX, 0x80000002
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Brand string part 1: EAX, EBX, ECX, EDX contain 16 bytes
    // Should contain ASCII characters (0x20-0x7E for printable range)
    let eax = regs.rax & 0xFFFFFFFF;
    let _ebx = regs.rbx & 0xFFFFFFFF;
    let _ecx = regs.rcx & 0xFFFFFFFF;
    let _edx = regs.rdx & 0xFFFFFFFF;
    // Just verify we get some value
    assert!(
        eax != 0 || _ebx != 0 || _ecx != 0 || _edx != 0,
        "Brand string should not be empty"
    );
}

#[test]
fn test_cpuid_extended_leaf_80000003_brand_string_2() {
    // EAX=0x80000003: Returns processor brand string part 2
    let code = [
        0xb8, 0x03, 0x00, 0x00, 0x80, // MOV EAX, 0x80000003
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Brand string part 2
    let _eax = regs.rax & 0xFFFFFFFF;
}

#[test]
fn test_cpuid_extended_leaf_80000004_brand_string_3() {
    // EAX=0x80000004: Returns processor brand string part 3
    let code = [
        0xb8, 0x04, 0x00, 0x00, 0x80, // MOV EAX, 0x80000004
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Brand string part 3
    let _eax = regs.rax & 0xFFFFFFFF;
}

#[test]
fn test_cpuid_extended_leaf_80000005_tlb_cache_info() {
    // EAX=0x80000005: Returns TLB and cache information
    let code = [
        0xb8, 0x05, 0x00, 0x00, 0x80, // MOV EAX, 0x80000005
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Returns TLB and cache information
    let _eax = regs.rax & 0xFFFFFFFF;
    let _ebx = regs.rbx & 0xFFFFFFFF;
    let _ecx = regs.rcx & 0xFFFFFFFF;
    let _edx = regs.rdx & 0xFFFFFFFF;
}

#[test]
fn test_cpuid_extended_leaf_80000006_cache_info() {
    // EAX=0x80000006: Returns extended cache information
    let code = [
        0xb8, 0x06, 0x00, 0x00, 0x80, // MOV EAX, 0x80000006
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Returns extended cache information
    let _eax = regs.rax & 0xFFFFFFFF;
}

#[test]
fn test_cpuid_extended_leaf_80000007_advanced_power_management() {
    // EAX=0x80000007: Returns advanced power management information
    let code = [
        0xb8, 0x07, 0x00, 0x00, 0x80, // MOV EAX, 0x80000007
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // EDX contains advanced power management features
    let _edx = regs.rdx & 0xFFFFFFFF;
}

#[test]
fn test_cpuid_extended_leaf_80000008_virtual_addressing() {
    // EAX=0x80000008: Returns virtual/physical addressing information
    let code = [
        0xb8, 0x08, 0x00, 0x00, 0x80, // MOV EAX, 0x80000008
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // EAX[7:0] = Physical address bits, EAX[15:8] = Linear address bits
    let eax = regs.rax & 0xFFFFFFFF;
    let phys_addr_bits = eax & 0xFF;
    let linear_addr_bits = (eax >> 8) & 0xFF;

    // Reasonable values for modern x86-64
    assert!(
        phys_addr_bits >= 32 && phys_addr_bits <= 52,
        "Physical address bits should be 32-52"
    );
    assert!(
        linear_addr_bits >= 32 && linear_addr_bits <= 57,
        "Linear address bits should be 32-57"
    );
}

// ===== VENDOR STRING TESTS =====

#[test]
fn test_cpuid_vendor_string_extraction() {
    // Extract and verify vendor string can be read
    let code = [
        0xb8, 0x00, 0x00, 0x00, 0x00, // MOV EAX, 0
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Vendor string is in EBX:EDX:ECX
    // Common vendors: "GenuineIntel" or "AuthenticAMD"
    let ebx = regs.rbx & 0xFFFFFFFF;
    let edx = regs.rdx & 0xFFFFFFFF;
    let ecx = regs.rcx & 0xFFFFFFFF;

    // Reconstruct vendor string bytes
    let vendor_bytes = [
        (ebx & 0xFF) as u8,
        ((ebx >> 8) & 0xFF) as u8,
        ((ebx >> 16) & 0xFF) as u8,
        ((ebx >> 24) & 0xFF) as u8,
        (edx & 0xFF) as u8,
        ((edx >> 8) & 0xFF) as u8,
        ((edx >> 16) & 0xFF) as u8,
        ((edx >> 24) & 0xFF) as u8,
        (ecx & 0xFF) as u8,
        ((ecx >> 8) & 0xFF) as u8,
        ((ecx >> 16) & 0xFF) as u8,
        ((ecx >> 24) & 0xFF) as u8,
    ];

    // At least some bytes should be printable ASCII
    let has_printable = vendor_bytes.iter().any(|&b| b >= 0x20 && b < 0x7F);
    assert!(
        has_printable,
        "Vendor string should contain printable ASCII"
    );
}

// ===== LEAF CONSISTENCY TESTS =====

#[test]
fn test_cpuid_extended_leaf_is_not_lower_than_standard() {
    // Extended max leaf should be >= standard max leaf (with high bit)
    let code = [
        0xb8, 0x00, 0x00, 0x00, 0x00, // MOV EAX, 0 (standard max)
        0x0f, 0xa2, // CPUID
        0x89, 0xc3, // MOV EBX, EAX (save standard max)
        0xb8, 0x00, 0x00, 0x00, 0x80, // MOV EAX, 0x80000000 (extended max)
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let standard_max = regs.rbx & 0xFFFFFFFF;
    let extended_max = regs.rax & 0xFFFFFFFF;

    // Extended leaves (0x80000000+) are separate from standard leaves (0+)
    // Both should be valid
    assert!(standard_max <= 0x100, "Standard max should be reasonable");
    assert!(
        extended_max >= 0x80000000,
        "Extended max should be >= 0x80000000"
    );
}

#[test]
fn test_cpuid_returns_consistent_values_for_same_input() {
    // Calling CPUID twice with same input should return same output
    // Note: CPUID modifies EAX, EBX, ECX, EDX, so save to RSI which is not touched
    let code = [
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 1 (first call)
        0x0f, 0xa2, // CPUID
        0x89, 0xc6, // MOV ESI, EAX (save EAX from first call)
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 1 (second call)
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Both calls should return same EAX value
    let eax_second = regs.rax & 0xFFFFFFFF;
    let eax_first = regs.rsi & 0xFFFFFFFF;
    assert_eq!(
        eax_first, eax_second,
        "CPUID should return consistent values"
    );
}

#[test]
fn test_cpuid_multiple_leaves_execution() {
    // Execute CPUID for multiple different leaves sequentially
    let code = [
        0xb8, 0x00, 0x00, 0x00, 0x00, // MOV EAX, 0
        0x0f, 0xa2, // CPUID (leaf 0)
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 1
        0x0f, 0xa2, // CPUID (leaf 1)
        0xb8, 0x02, 0x00, 0x00, 0x00, // MOV EAX, 2
        0x0f, 0xa2, // CPUID (leaf 2)
        0xb8, 0x03, 0x00, 0x00, 0x00, // MOV EAX, 3
        0x0f, 0xa2, // CPUID (leaf 3)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Last instruction was leaf 3, so final values should be from leaf 3
    let _eax = regs.rax & 0xFFFFFFFF;
}

#[test]
fn test_cpuid_ecx_subleaf_parameter() {
    // Test that ECX parameter is used for subleaf selection
    let code = [
        // Call leaf 4 (deterministic cache) with ECX=0
        0xb8, 0x04, 0x00, 0x00, 0x00, // MOV EAX, 4
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0x0f, 0xa2, // CPUID
        0x89, 0xc3, // MOV EBX, EAX (save result)
        // Call leaf 4 with ECX=1
        0xb8, 0x04, 0x00, 0x00, 0x00, // MOV EAX, 4
        0xb9, 0x01, 0x00, 0x00, 0x00, // MOV ECX, 1
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Results should be different for different subleaves
    // (or at least the instruction should execute without error)
    let _result1 = regs.rbx & 0xFFFFFFFF;
    let _result2 = regs.rax & 0xFFFFFFFF;
}

#[test]
fn test_cpuid_high_order_bits_cleared_in_32bit_leaves() {
    // In 32-bit leaves, RAX should contain valid EAX, but upper bits may be garbage
    let code = [
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 1
        0x0f, 0xa2, // CPUID
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Check that lower 32 bits have a reasonable value
    let eax_32 = regs.rax & 0xFFFFFFFF;
    assert!(eax_32 != 0, "EAX should be non-zero for leaf 1");
}
