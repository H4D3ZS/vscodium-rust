//! Platform detection and CPUID configuration for HVF backend.
//!
//! IMPORTANT: Apple's Hypervisor.framework on Apple Silicon only supports ARM64 guests.
//! There is NO support for running x86_64 kernels on Apple Silicon via Hypervisor.framework.
//!
//! Rosetta 2 only translates user-space binaries - it cannot translate kernel code.
//! To run x86_64 Linux on Apple Silicon, you would need:
//! - An ARM64 Linux VM (via Hypervisor.framework)  
//! - Rosetta exposed via VirtioFS (VZLinuxRosettaDirectoryShare)
//! - x86_64 userspace binaries translated by Rosetta
//!
//! This does NOT allow booting x86_64 kernels, which is what RAX does.
//! Therefore, on Apple Silicon, RAX must use the software emulator backend.
//!
//! This module is only compiled on Intel Macs where Hypervisor.framework
//! provides the VMX API for native x86_64 virtualization.

use crate::error::Result;

/// Check if we're running on Apple Silicon (ARM64).
/// Note: This module is only compiled on x86_64, so this always returns false.
#[allow(dead_code)]
pub fn is_apple_silicon() -> bool {
    cfg!(target_arch = "aarch64")
}

/// Check if we're running on Intel Mac.
/// Note: This module is only compiled on x86_64, so this always returns true.
pub fn is_intel_mac() -> bool {
    cfg!(target_arch = "x86_64")
}

/// Verify HVF is available on this Intel Mac.
/// On Intel Macs, we use native VMX virtualization via Hypervisor.framework.
pub fn check_hvf_available() -> Result<()> {
    // On Intel Macs, HVF uses VMX which is always available if the
    // Hypervisor.framework entitlement is present. The actual check
    // is done in bindings::hv_check_available().
    Ok(())
}

/// Get the VM creation flags for x86_64 virtualization.
/// On Intel Macs, we use the default flags for VMX-based virtualization.
pub fn get_vm_creation_flags() -> u64 {
    0 // HV_VM_DEFAULT
}

/// CPUID configuration for x86_64 guests.
///
/// When running under Rosetta translation, we need to provide appropriate
/// CPUID responses that reflect the emulated CPU capabilities.
#[derive(Clone)]
pub struct CpuidConfig {
    /// Maximum basic CPUID leaf
    pub max_basic_leaf: u32,
    /// Maximum extended CPUID leaf
    pub max_extended_leaf: u32,
    /// Vendor string (12 bytes: EBX, EDX, ECX)
    pub vendor: [u8; 12],
    /// Processor brand string (48 bytes)
    pub brand: [u8; 48],
}

impl Default for CpuidConfig {
    fn default() -> Self {
        // Provide a generic x86_64 compatible CPUID configuration
        CpuidConfig {
            max_basic_leaf: 0x16,
            max_extended_leaf: 0x80000008,
            // "GenuineIntel" - for compatibility
            vendor: *b"GenuineIntel",
            // Generic processor name (exactly 48 bytes)
            brand: *b"Intel(R) Core(TM) Processor (Rosetta)\0\0\0\0\0\0\0\0\0\0\0",
        }
    }
}

impl CpuidConfig {
    /// Generate CPUID response for a given leaf and subleaf.
    pub fn cpuid(&self, leaf: u32, subleaf: u32) -> (u32, u32, u32, u32) {
        match leaf {
            // Basic CPUID information
            0x00 => {
                let vendor = &self.vendor;
                (
                    self.max_basic_leaf,
                    u32::from_le_bytes([vendor[0], vendor[1], vendor[2], vendor[3]]),
                    u32::from_le_bytes([vendor[8], vendor[9], vendor[10], vendor[11]]),
                    u32::from_le_bytes([vendor[4], vendor[5], vendor[6], vendor[7]]),
                )
            }
            // Processor info and feature bits
            0x01 => {
                // Family 6, Model 0x8E (Kaby Lake-ish), Stepping 0xA
                let eax = 0x000806EA;
                // EBX: Brand index 0, CLFLUSH size 8, max APIC IDs 1, APIC ID 0
                let ebx = 0x00010800;
                // ECX: Feature flags - SSE3, SSSE3, SSE4.1, SSE4.2, POPCNT, AES, XSAVE, AVX, etc.
                let ecx = 0x7FFAFBBF;
                // EDX: Feature flags - FPU, VME, DE, PSE, TSC, MSR, PAE, MCE, CX8, APIC, etc.
                let edx = 0xBFEBFBFF;
                (eax, ebx, ecx, edx)
            }
            // Cache and TLB info
            0x02 => (0x76036301, 0x00F0B5FF, 0x00000000, 0x00C10000),
            // Processor serial number (not supported)
            0x03 => (0, 0, 0, 0),
            // Deterministic cache parameters
            0x04 => match subleaf {
                0 => (0x1C004121, 0x01C0003F, 0x0000003F, 0x00000000), // L1 Data
                1 => (0x1C004122, 0x01C0003F, 0x0000003F, 0x00000000), // L1 Instruction
                2 => (0x1C004143, 0x01C0003F, 0x000001FF, 0x00000000), // L2
                3 => (0x1C03C163, 0x03C0003F, 0x00001FFF, 0x00000006), // L3
                _ => (0, 0, 0, 0),
            },
            // MONITOR/MWAIT
            0x05 => (0x00000040, 0x00000040, 0x00000003, 0x00001120),
            // Thermal and power management
            0x06 => (0x000027F7, 0x00000002, 0x00000009, 0x00000000),
            // Extended feature flags
            0x07 => match subleaf {
                0 => (
                    0,          // EBX: BMI1, AVX2, BMI2, ERMS, INVPCID, etc.
                    0x029C6FBB, // ECX: PKU
                    0x00000000, // EDX
                    0x00000000,
                ),
                _ => (0, 0, 0, 0),
            },
            // Extended topology enumeration
            0x0B => match subleaf {
                0 => (0x00000001, 0x00000001, 0x00000100, 0x00000000), // SMT
                1 => (0x00000004, 0x00000001, 0x00000201, 0x00000000), // Core
                _ => (0, 0, 0, 0),
            },
            // Processor extended state enumeration
            0x0D => match subleaf {
                0 => (0x00000007, 0x00000340, 0x00000340, 0x00000000),
                1 => (0x00000001, 0x00000000, 0x00000000, 0x00000000),
                _ => (0, 0, 0, 0),
            },
            // Extended CPUID info
            0x80000000 => (self.max_extended_leaf, 0, 0, 0),
            // Extended processor info
            0x80000001 => (0, 0, 0x00000121, 0x2C100800),
            // Processor brand string (part 1)
            0x80000002 => {
                let b = &self.brand[0..16];
                (
                    u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
                    u32::from_le_bytes([b[4], b[5], b[6], b[7]]),
                    u32::from_le_bytes([b[8], b[9], b[10], b[11]]),
                    u32::from_le_bytes([b[12], b[13], b[14], b[15]]),
                )
            }
            // Processor brand string (part 2)
            0x80000003 => {
                let b = &self.brand[16..32];
                (
                    u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
                    u32::from_le_bytes([b[4], b[5], b[6], b[7]]),
                    u32::from_le_bytes([b[8], b[9], b[10], b[11]]),
                    u32::from_le_bytes([b[12], b[13], b[14], b[15]]),
                )
            }
            // Processor brand string (part 3)
            0x80000004 => {
                let b = &self.brand[32..48];
                (
                    u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
                    u32::from_le_bytes([b[4], b[5], b[6], b[7]]),
                    u32::from_le_bytes([b[8], b[9], b[10], b[11]]),
                    u32::from_le_bytes([b[12], b[13], b[14], b[15]]),
                )
            }
            // L1 cache and TLB info
            0x80000005 => (0, 0, 0, 0),
            // L2 cache and TLB info
            0x80000006 => (0, 0, 0x01006040, 0),
            // Advanced power management
            0x80000007 => (0, 0, 0, 0x00000100),
            // Virtual and physical address sizes
            0x80000008 => (0x00003028, 0, 0, 0), // 48-bit virtual, 40-bit physical
            // Default: return zeros
            _ => (0, 0, 0, 0),
        }
    }
}

/// MSR values for x86_64 guest initialization.
pub struct MsrDefaults {
    /// IA32_MISC_ENABLE
    pub misc_enable: u64,
    /// IA32_PAT (Page Attribute Table)
    pub pat: u64,
}

impl Default for MsrDefaults {
    fn default() -> Self {
        MsrDefaults {
            // Enable fast string, TCC, etc.
            misc_enable: 0x00850089,
            // Default PAT value
            pat: 0x0007040600070406,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpuid_vendor() {
        let config = CpuidConfig::default();
        let (eax, ebx, ecx, edx) = config.cpuid(0, 0);

        // Verify max leaf
        assert!(eax >= 0x01);

        // Reconstruct vendor string
        let vendor = [
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
        assert_eq!(&vendor, b"GenuineIntel");
    }

    #[test]
    fn test_architecture_detection() {
        // These should not panic
        let _ = is_apple_silicon();
        let _ = is_intel_mac();
    }
}
