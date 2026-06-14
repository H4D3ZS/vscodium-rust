use crate::error::{Error, Result};
use clap::ValueEnum;
use serde::de::{self, Visitor};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::{Path, PathBuf};
use std::str::FromStr;

const DEFAULT_MEM_MIB: u64 = 512;
const MIN_MEM_MIB: u64 = 128;
const DEFAULT_VCPUS: u8 = 1;
/// Default kernel command line for emulator boot.
/// Includes timing options for stable emulation:
/// - tsc=reliable: Don't recalibrate TSC (we provide stable instruction-based TSC)
/// - nohz=off: Disable tickless mode (simplifies timer handling)
/// - clocksource=tsc: Use TSC as clock source (we emulate it based on instruction count)
const DEFAULT_CMDLINE: &str =
    "console=ttyS0 earlyprintk=serial,ttyS0,115200 nokaslr tsc=reliable nohz=off clocksource=tsc";
/// Default kernel command line for AArch64 guests: PL011 console at the virt
/// machine UART base.
const DEFAULT_CMDLINE_AARCH64: &str = "console=ttyAMA0 earlycon=pl011,mmio32,0x09000000";

#[derive(Clone, Copy, Debug, Deserialize, Serialize, ValueEnum, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ArchKind {
    X86_64,
    Hexagon,
    /// ARM 64-bit (AArch64/ARMv8-A 64-bit mode)
    Aarch64,
    /// ARM 32-bit ARMv7-A (Cortex-A series)
    Armv7a,
    /// ARM 32-bit ARMv8-A (AArch32 mode)
    Armv8a32,
    /// ARM Cortex-M (Thumb-2, ARMv6-M/ARMv7-M/ARMv8-M)
    CortexM,
    /// ARM Cortex-R (real-time processors)
    CortexR,
    /// RISC-V 64-bit (RV64GC).
    Riscv64,
}

impl Default for ArchKind {
    fn default() -> Self {
        ArchKind::X86_64
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, ValueEnum, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BackendKind {
    Kvm,
    Emulator,
    /// Apple Hypervisor.framework with Rosetta for x86_64 emulation (macOS only)
    Hvf,
}

impl Default for BackendKind {
    fn default() -> Self {
        #[cfg(target_os = "linux")]
        {
            BackendKind::Kvm
        }
        // Intel Mac - use HVF for hardware virtualization
        #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
        {
            BackendKind::Hvf
        }
        // Apple Silicon - HVF can't run x86_64 guests, use emulator
        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        {
            BackendKind::Emulator
        }
        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
        {
            BackendKind::Emulator
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, ValueEnum, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Endianness {
    Little,
    Big,
}

impl Default for Endianness {
    fn default() -> Self {
        Endianness::Little
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, ValueEnum, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum HexagonIsa {
    V4,
    V5,
    V55,
    V60,
    V62,
    V65,
    V66,
    V67,
    V68,
    V69,
}

impl Default for HexagonIsa {
    fn default() -> Self {
        HexagonIsa::V68
    }
}

// =============================================================================
// ARM Architecture ISA Versions
// =============================================================================

/// ARM 64-bit (AArch64) architecture version.
/// Based on ARMv8-A and later with various extensions.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, ValueEnum, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Aarch64Isa {
    /// ARMv8.0-A: Base 64-bit ARM (Cortex-A53, A57, A72, A73)
    /// Features: AArch64 execution, AdvSIMD, optional crypto
    V8_0,
    /// ARMv8.1-A: LSE atomics, VHE, PAN, RDMA (Cortex-A75, A76)
    /// Features: Atomic ops (CAS, SWP), Virtualization Host Extensions
    V8_1,
    /// ARMv8.2-A: SVE, FP16, DotProd, RAS (Cortex-A55, A75, A76, A77)
    /// Features: Scalable Vector Extension (optional), half-precision FP
    V8_2,
    /// ARMv8.3-A: PAC, FCMA, NV (Cortex-A77, A78)
    /// Features: Pointer Authentication, complex number multiply
    V8_3,
    /// ARMv8.4-A: Flag manipulation, RCPC2, secure EL2 (Cortex-A78)
    /// Features: CFINV, RMIF, nested virtualization
    V8_4,
    /// ARMv8.5-A: BTI, MTE, SSBS, RNG (Cortex-X1, A78C)
    /// Features: Branch Target ID, Memory Tagging, speculation barrier
    V8_5,
    /// ARMv8.6-A: BFloat16, I8MM, WFET (Cortex-X2, A710)
    /// Features: ML-optimized formats, WFE with timeout
    V8_6,
    /// ARMv8.7-A: WFI with timeout, HBC, enhanced PAC (Cortex-X3)
    /// Features: WFIT, hardware capabilities
    V8_7,
    /// ARMv8.8-A: NMI, MOPS (memory copy/set) (Cortex-X4)
    /// Features: Non-maskable interrupts, memory operations
    V8_8,
    /// ARMv9.0-A: Mandatory SVE2, RME, FEAT_CSV2 (Cortex-A510, A710, X2)
    /// Features: SVE2, Realm Management Extension
    V9_0,
    /// ARMv9.1-A: Enhanced BTI (Cortex-A715, X3)
    V9_1,
    /// ARMv9.2-A: SME (Scalable Matrix Extension) (Cortex-A720, X4)
    /// Features: Matrix operations, streaming SVE mode
    V9_2,
    /// ARMv9.3-A: Enhanced RME, GCS
    /// Features: Guarded Control Stack
    V9_3,
    /// ARMv9.4-A: SME2, multi-vector ops
    /// Features: 16 ZA tiles
    V9_4,
}

impl Default for Aarch64Isa {
    fn default() -> Self {
        Aarch64Isa::V8_0
    }
}

impl Aarch64Isa {
    /// Returns true if this version supports Large System Extensions (atomics)
    pub fn has_lse(&self) -> bool {
        !matches!(self, Aarch64Isa::V8_0)
    }

    /// Returns true if this version supports Pointer Authentication
    pub fn has_pac(&self) -> bool {
        matches!(
            self,
            Aarch64Isa::V8_3
                | Aarch64Isa::V8_4
                | Aarch64Isa::V8_5
                | Aarch64Isa::V8_6
                | Aarch64Isa::V8_7
                | Aarch64Isa::V8_8
                | Aarch64Isa::V9_0
                | Aarch64Isa::V9_1
                | Aarch64Isa::V9_2
                | Aarch64Isa::V9_3
                | Aarch64Isa::V9_4
        )
    }

    /// Returns true if this version supports Branch Target Identification
    pub fn has_bti(&self) -> bool {
        matches!(
            self,
            Aarch64Isa::V8_5
                | Aarch64Isa::V8_6
                | Aarch64Isa::V8_7
                | Aarch64Isa::V8_8
                | Aarch64Isa::V9_0
                | Aarch64Isa::V9_1
                | Aarch64Isa::V9_2
                | Aarch64Isa::V9_3
                | Aarch64Isa::V9_4
        )
    }

    /// Returns true if this is an ARMv9 version (mandatory SVE2)
    pub fn is_v9(&self) -> bool {
        matches!(
            self,
            Aarch64Isa::V9_0
                | Aarch64Isa::V9_1
                | Aarch64Isa::V9_2
                | Aarch64Isa::V9_3
                | Aarch64Isa::V9_4
        )
    }
}

/// ARM 32-bit (AArch32) architecture version.
/// Covers ARMv6 through ARMv8-A in AArch32 mode.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, ValueEnum, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Aarch32Isa {
    /// ARMv6: ARM1136, ARM1176, ARM11MPCore
    /// Features: SIMD in GPRs, exclusive access, TrustZone (v6Z)
    V6,
    /// ARMv6T2: Thumb-2 technology
    /// Features: 32-bit Thumb, IT blocks, bit field ops
    V6T2,
    /// ARMv6K: Kernel extensions
    /// Features: CLREX, memory barriers, multiprocessing
    V6K,
    /// ARMv7-A: Cortex-A5, A7, A8, A9, A15, A17
    /// Features: VFP, NEON, virtualization extensions (optional)
    V7A,
    /// ARMv7-A with virtualization: Cortex-A15, A17
    /// Features: HYP mode, stage-2 translation
    V7AVirt,
    /// ARMv7-A with LPAE: Large Physical Address Extension
    /// Features: 40-bit physical addresses, long descriptors
    V7ALpae,
    /// ARMv8-A AArch32: Cortex-A32, A35 (32-bit only cores)
    /// Features: Crypto extensions, CRC32, all v8 mandatory
    V8A32,
}

impl Default for Aarch32Isa {
    fn default() -> Self {
        Aarch32Isa::V7A
    }
}

impl Aarch32Isa {
    /// Returns true if this version supports Thumb-2
    pub fn has_thumb2(&self) -> bool {
        !matches!(self, Aarch32Isa::V6)
    }

    /// Returns true if this version supports NEON
    pub fn has_neon(&self) -> bool {
        matches!(
            self,
            Aarch32Isa::V7A | Aarch32Isa::V7AVirt | Aarch32Isa::V7ALpae | Aarch32Isa::V8A32
        )
    }

    /// Returns true if this version supports virtualization
    pub fn has_virtualization(&self) -> bool {
        matches!(
            self,
            Aarch32Isa::V7AVirt | Aarch32Isa::V7ALpae | Aarch32Isa::V8A32
        )
    }

    /// Returns true if this version supports 40-bit physical addresses
    pub fn has_lpae(&self) -> bool {
        matches!(self, Aarch32Isa::V7ALpae | Aarch32Isa::V8A32)
    }
}

/// ARM Cortex-M architecture version.
/// Microcontroller profile with different exception model.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, ValueEnum, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CortexMIsa {
    /// ARMv6-M: Cortex-M0, M0+, M1
    /// Features: Subset Thumb, NVIC, no DIV, optional SysTick
    V6M,
    /// ARMv7-M: Cortex-M3
    /// Features: Full Thumb-2, DIV, bit-banding, MPU
    V7M,
    /// ARMv7E-M: Cortex-M4, M7
    /// Features: DSP extensions, optional FPU (VFPv4-D16)
    V7EM,
    /// ARMv8-M Baseline: Cortex-M23
    /// Features: TrustZone, stack limit checking
    V8MBaseline,
    /// ARMv8-M Mainline: Cortex-M33, M35P
    /// Features: TrustZone, DSP, optional FPU/MVE
    V8MMainline,
    /// ARMv8.1-M: Cortex-M55, M85
    /// Features: MVE (Helium), low-overhead loops, half-precision FP
    V8_1M,
}

impl Default for CortexMIsa {
    fn default() -> Self {
        CortexMIsa::V7M
    }
}

impl CortexMIsa {
    /// Returns true if this version has full Thumb-2
    pub fn has_full_thumb2(&self) -> bool {
        !matches!(self, CortexMIsa::V6M | CortexMIsa::V8MBaseline)
    }

    /// Returns true if this version supports TrustZone
    pub fn has_trustzone(&self) -> bool {
        matches!(
            self,
            CortexMIsa::V8MBaseline | CortexMIsa::V8MMainline | CortexMIsa::V8_1M
        )
    }

    /// Returns true if this version supports DSP extensions
    pub fn has_dsp(&self) -> bool {
        matches!(
            self,
            CortexMIsa::V7EM | CortexMIsa::V8MMainline | CortexMIsa::V8_1M
        )
    }

    /// Returns true if this version can have an FPU
    pub fn can_have_fpu(&self) -> bool {
        !matches!(self, CortexMIsa::V6M | CortexMIsa::V8MBaseline)
    }

    /// Returns true if this version supports MVE (Helium)
    pub fn has_mve(&self) -> bool {
        matches!(self, CortexMIsa::V8_1M)
    }
}

/// ARM Cortex-R architecture version.
/// Real-time profile with deterministic interrupt latency.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, ValueEnum, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CortexRIsa {
    /// ARMv7-R: Cortex-R4, R5, R7, R8
    /// Features: MPU, TCM, optional dual-core lockstep
    V7R,
    /// ARMv8-R AArch32: Cortex-R52, R52+
    /// Features: Virtualization, optional EL2, PMSAv8
    V8R,
    /// ARMv8-R AArch64: Cortex-R82
    /// Features: First 64-bit R-profile, optional MMU for Linux
    V8R64,
}

impl Default for CortexRIsa {
    fn default() -> Self {
        CortexRIsa::V7R
    }
}

impl CortexRIsa {
    /// Returns true if this is a 64-bit capable version
    pub fn is_64bit(&self) -> bool {
        matches!(self, CortexRIsa::V8R64)
    }

    /// Returns true if this version supports virtualization
    pub fn has_virtualization(&self) -> bool {
        matches!(self, CortexRIsa::V8R | CortexRIsa::V8R64)
    }
}

// =============================================================================
// ARM Feature Flags (optional extensions)
// =============================================================================

bitflags::bitflags! {
    /// ARM optional feature flags.
    /// These represent ISA extensions that may or may not be present.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub struct ArmFeatures: u64 {
        // Crypto and security
        /// AES instructions (AESE, AESD, AESMC, AESIMC)
        const CRYPTO_AES = 1 << 0;
        /// SHA-1 instructions
        const CRYPTO_SHA1 = 1 << 1;
        /// SHA-256 instructions
        const CRYPTO_SHA256 = 1 << 2;
        /// SHA-512 instructions (v8.2+)
        const CRYPTO_SHA512 = 1 << 3;
        /// SHA-3 instructions (v8.2+)
        const CRYPTO_SHA3 = 1 << 4;
        /// SM3/SM4 Chinese crypto (v8.2+)
        const CRYPTO_SM = 1 << 5;
        /// CRC32 instructions
        const CRC32 = 1 << 6;

        // SIMD/Vector extensions
        /// NEON/AdvSIMD
        const NEON = 1 << 8;
        /// Half-precision FP (FP16)
        const FP16 = 1 << 9;
        /// BFloat16 (v8.6+)
        const BF16 = 1 << 10;
        /// Int8 matrix multiply (v8.6+)
        const I8MM = 1 << 11;
        /// Dot product instructions (v8.2+)
        const DOTPROD = 1 << 12;
        /// SVE (Scalable Vector Extension)
        const SVE = 1 << 13;
        /// SVE2
        const SVE2 = 1 << 14;
        /// SVE2 + AES
        const SVE2_AES = 1 << 15;
        /// SVE2 + SHA3
        const SVE2_SHA3 = 1 << 16;
        /// SVE2 + SM4
        const SVE2_SM4 = 1 << 17;
        /// SVE2 + bit permute
        const SVE2_BITPERM = 1 << 18;
        /// SME (Scalable Matrix Extension)
        const SME = 1 << 19;
        /// SME2
        const SME2 = 1 << 20;

        // Atomics and memory
        /// LSE atomics (v8.1+)
        const LSE = 1 << 24;
        /// LSE2 - larger atomics (v8.4+)
        const LSE2 = 1 << 25;
        /// RCPC (Release Consistent Processor Consistent)
        const RCPC = 1 << 26;
        /// RCPC2 (v8.4+)
        const RCPC2 = 1 << 27;

        // Pointer/Control flow
        /// PAC (Pointer Authentication) - address
        const PACA = 1 << 32;
        /// PAC - generic
        const PACG = 1 << 33;
        /// BTI (Branch Target Identification)
        const BTI = 1 << 34;
        /// MTE (Memory Tagging Extension)
        const MTE = 1 << 35;
        /// MTE2
        const MTE2 = 1 << 36;
        /// GCS (Guarded Control Stack)
        const GCS = 1 << 37;

        // Virtualization
        /// VHE (Virtualization Host Extensions)
        const VHE = 1 << 40;
        /// Nested virtualization
        const NV = 1 << 41;
        /// RME (Realm Management Extension)
        const RME = 1 << 42;

        // Misc
        /// RNG (hardware random number)
        const RNG = 1 << 48;
        /// DIT (Data Independent Timing)
        const DIT = 1 << 49;
        /// SSBS (Speculative Store Bypass Safe)
        const SSBS = 1 << 50;
        /// SB (Speculation Barrier)
        const SB = 1 << 51;
        /// MOPS (Memory copy/set operations)
        const MOPS = 1 << 52;
        /// HBC (Hinted Conditional Branches)
        const HBC = 1 << 53;
        /// NMI (Non-Maskable Interrupts)
        const NMI = 1 << 54;

        // VFP/FPU variants (for 32-bit)
        /// VFPv3-D16 (16 double-precision registers)
        const VFPV3_D16 = 1 << 56;
        /// VFPv3-D32 (32 double-precision registers)
        const VFPV3_D32 = 1 << 57;
        /// VFPv4 (fused multiply-add)
        const VFPV4 = 1 << 58;
        /// FPU single-precision only
        const FP_SP = 1 << 59;
        /// FPU double-precision
        const FP_DP = 1 << 60;

        // Cortex-M specific
        /// MVE (M-profile Vector Extension / Helium)
        const MVE = 1 << 61;
        /// MVE with floating-point
        const MVE_FP = 1 << 62;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MemorySize(pub u64);

impl MemorySize {
    pub fn bytes(self) -> u64 {
        self.0
    }
}

impl Default for MemorySize {
    fn default() -> Self {
        MemorySize(DEFAULT_MEM_MIB << 20)
    }
}

impl fmt::Display for MemorySize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for MemorySize {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let s = input.trim();
        if s.is_empty() {
            return Err(Error::InvalidConfig("memory size is empty".to_string()));
        }

        let mut num_end = s.len();
        for (i, ch) in s.char_indices() {
            if !ch.is_ascii_digit() {
                num_end = i;
                break;
            }
        }

        let (num_part, suffix_part) = s.split_at(num_end);
        if num_part.is_empty() {
            return Err(Error::InvalidConfig(format!(
                "invalid memory size: {input}"
            )));
        }

        let value = num_part
            .parse::<u64>()
            .map_err(|_| Error::InvalidConfig(format!("invalid memory size: {input}")))?;

        let suffix = suffix_part.trim();
        let multiplier = if suffix.is_empty() {
            1u64
        } else {
            let mut suffix = suffix.to_ascii_uppercase();
            if let Some(stripped) = suffix.strip_suffix('B') {
                suffix = stripped.to_string();
            }
            if let Some(stripped) = suffix.strip_suffix('I') {
                suffix = stripped.to_string();
            }
            match suffix.as_str() {
                "K" => 1u64 << 10,
                "M" => 1u64 << 20,
                "G" => 1u64 << 30,
                "T" => 1u64 << 40,
                "P" => 1u64 << 50,
                "E" => 1u64 << 60,
                _ => {
                    return Err(Error::InvalidConfig(format!(
                        "invalid memory size suffix: {suffix_part}"
                    )));
                }
            }
        };

        let bytes = value
            .checked_mul(multiplier)
            .ok_or_else(|| Error::InvalidConfig("memory size overflow".to_string()))?;

        Ok(MemorySize(bytes))
    }
}

impl<'de> Deserialize<'de> for MemorySize {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct MemorySizeVisitor;

        impl<'de> Visitor<'de> for MemorySizeVisitor {
            type Value = MemorySize;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("memory size as string or integer bytes")
            }

            fn visit_u64<E>(self, value: u64) -> std::result::Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(MemorySize(value))
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: de::Error,
            {
                MemorySize::from_str(value).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_any(MemorySizeVisitor)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Address(pub u64);

impl Address {
    pub fn raw(self) -> u64 {
        self.0
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Address {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let s = input.trim();
        if s.is_empty() {
            return Err(Error::InvalidConfig("address is empty".to_string()));
        }

        let value = if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
            u64::from_str_radix(hex, 16)
                .map_err(|_| Error::InvalidConfig(format!("invalid address: {input}")))?
        } else {
            s.parse::<u64>()
                .map_err(|_| Error::InvalidConfig(format!("invalid address: {input}")))?
        };

        Ok(Address(value))
    }
}

impl<'de> Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct AddressVisitor;

        impl<'de> Visitor<'de> for AddressVisitor {
            type Value = Address;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("address as string or integer")
            }

            fn visit_u64<E>(self, value: u64) -> std::result::Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Address(value))
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: de::Error,
            {
                Address::from_str(value).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_any(AddressVisitor)
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct FileConfig {
    pub arch: Option<ArchKind>,
    pub backend: Option<BackendKind>,
    pub memory: Option<MemorySize>,
    pub vcpus: Option<u8>,
    pub kernel: Option<PathBuf>,
    pub initrd: Option<PathBuf>,
    pub cmdline: Option<String>,
    // Hexagon options
    pub hexagon_isa: Option<HexagonIsa>,
    pub hexagon_endian: Option<Endianness>,
    pub hexagon_entry: Option<Address>,
    pub hexagon_load_addr: Option<Address>,
    // ARM options
    pub aarch64_isa: Option<Aarch64Isa>,
    pub aarch32_isa: Option<Aarch32Isa>,
    pub cortexm_isa: Option<CortexMIsa>,
    pub cortexr_isa: Option<CortexRIsa>,
    pub arm_entry: Option<Address>,
    pub arm_load_addr: Option<Address>,
    pub arm_dtb: Option<PathBuf>,
    /// Attach the optional PCI device models (e1000/nvme/ahci/ac97/uhci).
    pub pci_devices: Option<bool>,
}

impl FileConfig {
    pub fn load(path: &Path) -> Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let config = toml::from_str::<FileConfig>(&contents)
            .map_err(|e| Error::InvalidConfig(format!("toml error: {e}")))?;
        Ok(config)
    }
}

#[derive(Clone, Debug, Default)]
pub struct CliConfig {
    pub arch: Option<ArchKind>,
    pub backend: Option<BackendKind>,
    pub memory: Option<MemorySize>,
    pub vcpus: Option<u8>,
    pub kernel: Option<PathBuf>,
    pub initrd: Option<PathBuf>,
    pub cmdline: Option<String>,
    // Hexagon options
    pub hexagon_isa: Option<HexagonIsa>,
    pub hexagon_endian: Option<Endianness>,
    pub hexagon_entry: Option<Address>,
    pub hexagon_load_addr: Option<Address>,
    // ARM options
    pub aarch64_isa: Option<Aarch64Isa>,
    pub aarch32_isa: Option<Aarch32Isa>,
    pub cortexm_isa: Option<CortexMIsa>,
    pub cortexr_isa: Option<CortexRIsa>,
    pub arm_entry: Option<Address>,
    pub arm_load_addr: Option<Address>,
    pub arm_dtb: Option<PathBuf>,
    // Debug/profiling options
    pub trace: Option<PathBuf>,
    /// GDB server port (enables GDB server when set).
    pub gdb_port: Option<u16>,
    /// Wait for GDB connection before starting.
    pub wait_gdb: bool,
    /// Snapshot interval (take snapshot every N instructions, 0 = disabled)
    pub snapshot_interval: u64,
    /// Take snapshot at specific instruction counts (comma-separated)
    pub snapshot_at: Vec<u64>,
    /// Directory to save snapshots
    pub snapshot_dir: Option<PathBuf>,
    /// Snapshot file to resume from
    pub resume: Option<PathBuf>,
    /// Checkpoint (.rxc) file to resume the whole machine from.
    pub checkpoint: Option<PathBuf>,
    /// Output path for checkpoints triggered by hotkey/signal (default
    /// `./checkpoint.rxc` relative to the working directory).
    pub snapshot_out: Option<PathBuf>,
    /// Enable instruction profiling
    pub profile: bool,
    /// JSON output path for profiling results
    pub profile_output: Option<PathBuf>,
    /// Live profiling stats interval (instructions)
    pub profile_interval: Option<u64>,
    /// Attach the optional PCI device models behind the host bridge.
    pub pci_devices: bool,
}

#[derive(Clone, Debug)]
pub struct VmConfig {
    pub arch: ArchKind,
    pub backend: BackendKind,
    pub memory: MemorySize,
    pub vcpus: u8,
    pub kernel: PathBuf,
    pub initrd: Option<PathBuf>,
    pub cmdline: String,
    // Hexagon options
    pub hexagon_isa: HexagonIsa,
    pub hexagon_endian: Endianness,
    pub hexagon_entry: Option<Address>,
    pub hexagon_load_addr: Option<Address>,
    // ARM options
    pub aarch64_isa: Aarch64Isa,
    pub aarch32_isa: Aarch32Isa,
    pub cortexm_isa: CortexMIsa,
    pub cortexr_isa: CortexRIsa,
    pub arm_entry: Option<Address>,
    pub arm_load_addr: Option<Address>,
    pub arm_dtb: Option<PathBuf>,
    // Debug/profiling options
    pub trace: Option<PathBuf>,
    /// GDB server port (enables GDB server when set).
    pub gdb_port: Option<u16>,
    /// Wait for GDB connection before starting.
    pub wait_gdb: bool,
    /// Snapshot interval (take snapshot every N instructions, 0 = disabled)
    pub snapshot_interval: u64,
    /// Take snapshot at specific instruction counts
    pub snapshot_at: Vec<u64>,
    /// Directory to save snapshots
    pub snapshot_dir: Option<PathBuf>,
    /// Snapshot file to resume from
    pub resume: Option<PathBuf>,
    /// Checkpoint (.rxc) file to resume the whole machine from.
    pub checkpoint: Option<PathBuf>,
    /// Output path for checkpoints triggered by hotkey/signal.
    pub snapshot_out: Option<PathBuf>,
    /// Enable instruction profiling
    pub profile: bool,
    /// JSON output path for profiling results
    pub profile_output: Option<PathBuf>,
    /// Live profiling stats interval (instructions)
    pub profile_interval: Option<u64>,
    /// Attach the optional PCI device models behind the host bridge. Off by
    /// default so the default machine (and its verified boot) is unchanged.
    pub pci_devices: bool,
}

/// Sniff the guest architecture from the kernel image: the ARM64 `Image`
/// magic, or an ELF `e_machine`. Returns None when the format is unknown
/// (e.g. an x86 bzImage), letting the caller fall back to the default.
fn detect_arch_from_kernel(kernel: &Path) -> Option<ArchKind> {
    use std::io::Read;

    let mut head = [0u8; 64];
    let mut f = std::fs::File::open(kernel).ok()?;
    let n = f.read(&mut head).ok()?;
    let head = &head[..n];

    // ARM64 Linux Image: "ARM\x64" magic at offset 56.
    if n >= 60 && u32::from_le_bytes(head[56..60].try_into().unwrap()) == 0x644D_5241 {
        return Some(ArchKind::Aarch64);
    }
    if head.starts_with(b"\x7fELF") && n >= 20 {
        return match u16::from_le_bytes(head[18..20].try_into().unwrap()) {
            62 => Some(ArchKind::X86_64),
            183 => Some(ArchKind::Aarch64),
            243 => Some(ArchKind::Riscv64),
            164 => Some(ArchKind::Hexagon),
            40 => Some(ArchKind::Armv7a),
            _ => None,
        };
    }
    None
}

/// Pick a backend suited to the guest architecture when none was requested.
/// AArch64 defaults to the software emulator everywhere; on Apple Silicon,
/// `--backend hvf` selects near-native Hypervisor.framework instead.
fn default_backend_for(arch: ArchKind) -> BackendKind {
    match arch {
        ArchKind::X86_64 => BackendKind::default(),
        _ => BackendKind::Emulator,
    }
}

impl VmConfig {
    pub fn from_sources(cli: CliConfig, file: Option<FileConfig>) -> Result<Self> {
        let file = file.unwrap_or_default();
        let kernel = cli
            .kernel
            .or(file.kernel)
            .ok_or_else(|| Error::InvalidConfig("kernel path is required".to_string()))?;
        let arch = cli
            .arch
            .or(file.arch)
            .or_else(|| {
                let detected = detect_arch_from_kernel(&kernel);
                if let Some(a) = detected {
                    tracing::info!(arch = ?a, "auto-detected guest architecture from kernel image");
                }
                detected
            })
            .unwrap_or_default();
        let backend = cli
            .backend
            .or(file.backend)
            .unwrap_or_else(|| default_backend_for(arch));
        let memory = cli.memory.or(file.memory).unwrap_or_default();
        let vcpus = cli.vcpus.or(file.vcpus).unwrap_or(DEFAULT_VCPUS);
        let initrd = cli.initrd.or(file.initrd);
        let cmdline = cli.cmdline.or(file.cmdline).unwrap_or_else(|| {
            match arch {
                ArchKind::Aarch64 => DEFAULT_CMDLINE_AARCH64,
                _ => DEFAULT_CMDLINE,
            }
            .to_string()
        });
        // Hexagon options
        let hexagon_isa = cli.hexagon_isa.or(file.hexagon_isa).unwrap_or_default();
        let hexagon_endian = cli
            .hexagon_endian
            .or(file.hexagon_endian)
            .unwrap_or_default();
        let hexagon_entry = cli.hexagon_entry.or(file.hexagon_entry);
        let hexagon_load_addr = cli.hexagon_load_addr.or(file.hexagon_load_addr);
        // ARM options
        let aarch64_isa = cli.aarch64_isa.or(file.aarch64_isa).unwrap_or_default();
        let aarch32_isa = cli.aarch32_isa.or(file.aarch32_isa).unwrap_or_default();
        let cortexm_isa = cli.cortexm_isa.or(file.cortexm_isa).unwrap_or_default();
        let cortexr_isa = cli.cortexr_isa.or(file.cortexr_isa).unwrap_or_default();
        let arm_entry = cli.arm_entry.or(file.arm_entry);
        let arm_load_addr = cli.arm_load_addr.or(file.arm_load_addr);
        let arm_dtb = cli.arm_dtb.or(file.arm_dtb);

        let config = VmConfig {
            arch,
            backend,
            memory,
            vcpus,
            kernel,
            initrd,
            cmdline,
            hexagon_isa,
            hexagon_endian,
            hexagon_entry,
            hexagon_load_addr,
            aarch64_isa,
            aarch32_isa,
            cortexm_isa,
            cortexr_isa,
            arm_entry,
            arm_load_addr,
            arm_dtb,
            trace: cli.trace,
            gdb_port: cli.gdb_port,
            wait_gdb: cli.wait_gdb,
            snapshot_interval: cli.snapshot_interval,
            snapshot_at: cli.snapshot_at,
            snapshot_dir: cli.snapshot_dir,
            resume: cli.resume,
            checkpoint: cli.checkpoint,
            snapshot_out: cli.snapshot_out,
            profile: cli.profile,
            profile_output: cli.profile_output,
            profile_interval: cli.profile_interval,
            pci_devices: cli.pci_devices || file.pci_devices.unwrap_or(false),
        };

        config.validate()?;
        Ok(config)
    }

    /// Build a config to resume a machine from a checkpoint's embedded config,
    /// applying CLI overrides (CLI takes precedence over the embedded values).
    ///
    /// Unlike [`from_sources`](Self::from_sources) this does NOT require the
    /// kernel/initrd to exist on disk: the entire machine image is restored from
    /// the checkpoint, so the embedded kernel path is informational only. The
    /// user may still override any field (memory size, cmdline, even the arch)
    /// — including in ways that will not work — which is intentional.
    pub fn from_checkpoint(cp: CheckpointConfig, cli: CliConfig) -> Result<Self> {
        let config = VmConfig {
            arch: cli.arch.unwrap_or(cp.arch),
            backend: cli.backend.unwrap_or(cp.backend),
            memory: cli.memory.unwrap_or(MemorySize(cp.memory_bytes)),
            vcpus: cli.vcpus.unwrap_or(cp.vcpus),
            kernel: cli.kernel.unwrap_or(cp.kernel),
            initrd: cli.initrd.or(cp.initrd),
            cmdline: cli.cmdline.unwrap_or(cp.cmdline),
            hexagon_isa: cli.hexagon_isa.unwrap_or(cp.hexagon_isa),
            hexagon_endian: cli.hexagon_endian.unwrap_or(cp.hexagon_endian),
            hexagon_entry: cli.hexagon_entry.or(cp.hexagon_entry.map(Address)),
            hexagon_load_addr: cli.hexagon_load_addr.or(cp.hexagon_load_addr.map(Address)),
            aarch64_isa: cli.aarch64_isa.unwrap_or(cp.aarch64_isa),
            aarch32_isa: cli.aarch32_isa.unwrap_or(cp.aarch32_isa),
            cortexm_isa: cli.cortexm_isa.unwrap_or(cp.cortexm_isa),
            cortexr_isa: cli.cortexr_isa.unwrap_or(cp.cortexr_isa),
            arm_entry: cli.arm_entry.or(cp.arm_entry.map(Address)),
            arm_load_addr: cli.arm_load_addr.or(cp.arm_load_addr.map(Address)),
            arm_dtb: cli.arm_dtb.or(cp.arm_dtb),
            trace: cli.trace,
            gdb_port: cli.gdb_port,
            wait_gdb: cli.wait_gdb,
            snapshot_interval: cli.snapshot_interval,
            snapshot_at: cli.snapshot_at,
            snapshot_dir: cli.snapshot_dir,
            resume: cli.resume,
            checkpoint: cli.checkpoint,
            snapshot_out: cli.snapshot_out,
            profile: cli.profile,
            profile_output: cli.profile_output,
            profile_interval: cli.profile_interval,
            pci_devices: cli.pci_devices,
        };
        config.validate_resume()?;
        Ok(config)
    }

    /// Capture the machine-defining portion of this config for embedding in a
    /// checkpoint.
    pub fn to_checkpoint(&self) -> CheckpointConfig {
        CheckpointConfig {
            arch: self.arch,
            backend: self.backend,
            memory_bytes: self.memory.bytes(),
            vcpus: self.vcpus,
            kernel: self.kernel.clone(),
            initrd: self.initrd.clone(),
            cmdline: self.cmdline.clone(),
            hexagon_isa: self.hexagon_isa,
            hexagon_endian: self.hexagon_endian,
            hexagon_entry: self.hexagon_entry.map(|a| a.raw()),
            hexagon_load_addr: self.hexagon_load_addr.map(|a| a.raw()),
            aarch64_isa: self.aarch64_isa,
            aarch32_isa: self.aarch32_isa,
            cortexm_isa: self.cortexm_isa,
            cortexr_isa: self.cortexr_isa,
            arm_entry: self.arm_entry.map(|a| a.raw()),
            arm_load_addr: self.arm_load_addr.map(|a| a.raw()),
            arm_dtb: self.arm_dtb.clone(),
        }
    }

    pub fn validate(&self) -> Result<()> {
        self.validate_inner(true)
    }

    /// Validate a config used to resume from a checkpoint. Identical to
    /// [`validate`](Self::validate) except it does NOT require the kernel/initrd
    /// files to exist on disk — when resuming, the machine image (including the
    /// kernel that was loaded into RAM) comes from the checkpoint itself.
    pub fn validate_resume(&self) -> Result<()> {
        self.validate_inner(false)
    }

    fn validate_inner(&self, check_files: bool) -> Result<()> {
        if self.vcpus == 0 {
            return Err(Error::InvalidConfig("vcpus must be at least 1".to_string()));
        }
        let min_mem_bytes = MIN_MEM_MIB << 20;
        if self.memory.bytes() < min_mem_bytes {
            return Err(Error::InvalidConfig(format!(
                "memory must be at least {MIN_MEM_MIB} MiB"
            )));
        }
        if check_files && !self.kernel.exists() {
            return Err(Error::InvalidConfig(format!(
                "kernel not found: {}",
                self.kernel.display()
            )));
        }
        if check_files {
            if let Some(initrd) = &self.initrd {
                if !initrd.exists() {
                    return Err(Error::InvalidConfig(format!(
                        "initrd not found: {}",
                        initrd.display()
                    )));
                }
            }
        }
        if self.arch == ArchKind::Hexagon && self.backend == BackendKind::Kvm {
            return Err(Error::InvalidConfig(
                "hexagon is only supported with the emulator backend".to_string(),
            ));
        }
        if self.arch == ArchKind::Hexagon && self.backend == BackendKind::Hvf {
            return Err(Error::InvalidConfig(
                "hexagon is only supported with the emulator backend".to_string(),
            ));
        }
        // ARM architecture validation
        match self.arch {
            ArchKind::Aarch64 => {
                // Aarch64 can use HVF on Apple Silicon, or emulator everywhere
                if self.backend == BackendKind::Kvm {
                    return Err(Error::InvalidConfig(
                        "aarch64 with KVM is not yet implemented".to_string(),
                    ));
                }
                #[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
                if self.backend == BackendKind::Hvf {
                    return Err(Error::InvalidConfig(
                        "HVF for aarch64 guests requires Apple Silicon (ARM64 Mac)".to_string(),
                    ));
                }
            }
            ArchKind::Armv7a | ArchKind::Armv8a32 | ArchKind::CortexM | ArchKind::CortexR => {
                // 32-bit ARM variants only support emulator for now
                if self.backend != BackendKind::Emulator {
                    return Err(Error::InvalidConfig(format!(
                        "{:?} is only supported with the emulator backend",
                        self.arch
                    )));
                }
            }
            ArchKind::Riscv64 => {
                if self.backend != BackendKind::Emulator {
                    return Err(Error::InvalidConfig(
                        "riscv64 is only supported with the emulator backend".to_string(),
                    ));
                }
            }
            _ => {}
        }
        // HVF backend architecture validation
        if self.backend == BackendKind::Hvf {
            #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
            if self.arch != ArchKind::X86_64 {
                return Err(Error::InvalidConfig(
                    "HVF on Intel Mac only supports x86_64 guests".to_string(),
                ));
            }
            #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
            if self.arch != ArchKind::Aarch64 {
                return Err(Error::InvalidConfig(
                    "HVF on Apple Silicon only supports aarch64 guests".to_string(),
                ));
            }
        }
        if self.arch == ArchKind::Hexagon {
            let mem_bytes = self.memory.bytes();
            if mem_bytes > (u32::MAX as u64 + 1) {
                return Err(Error::InvalidConfig(
                    "hexagon guest memory must not exceed 4 GiB".to_string(),
                ));
            }
            if let Some(addr) = self.hexagon_load_addr {
                if addr.raw() >= mem_bytes {
                    return Err(Error::InvalidConfig(format!(
                        "hexagon load address {:#x} outside guest memory",
                        addr.raw()
                    )));
                }
            }
            if let Some(entry) = self.hexagon_entry {
                if entry.raw() >= mem_bytes {
                    return Err(Error::InvalidConfig(format!(
                        "hexagon entry address {:#x} outside guest memory",
                        entry.raw()
                    )));
                }
            }
        }
        Ok(())
    }
}

/// The machine-defining configuration embedded in a checkpoint so it can be
/// resumed self-contained (`rax --checkpoint file.rxc`). Uses primitive field
/// types (plain `u64` rather than [`MemorySize`]/[`Address`]) so it serializes
/// cleanly through bincode, which cannot handle the `deserialize_any`-based
/// human-friendly parsers those types use for TOML/CLI input.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CheckpointConfig {
    pub arch: ArchKind,
    pub backend: BackendKind,
    pub memory_bytes: u64,
    pub vcpus: u8,
    pub kernel: PathBuf,
    pub initrd: Option<PathBuf>,
    pub cmdline: String,
    pub hexagon_isa: HexagonIsa,
    pub hexagon_endian: Endianness,
    pub hexagon_entry: Option<u64>,
    pub hexagon_load_addr: Option<u64>,
    pub aarch64_isa: Aarch64Isa,
    pub aarch32_isa: Aarch32Isa,
    pub cortexm_isa: CortexMIsa,
    pub cortexr_isa: CortexRIsa,
    pub arm_entry: Option<u64>,
    pub arm_load_addr: Option<u64>,
    pub arm_dtb: Option<PathBuf>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_size_parses_units() {
        assert_eq!(MemorySize::from_str("1024").unwrap().bytes(), 1024);
        assert_eq!(MemorySize::from_str("1K").unwrap().bytes(), 1024);
        assert_eq!(MemorySize::from_str("1KiB").unwrap().bytes(), 1024);
        assert_eq!(MemorySize::from_str("2M").unwrap().bytes(), 2 * 1024 * 1024);
        assert_eq!(
            MemorySize::from_str("3g").unwrap().bytes(),
            3 * 1024 * 1024 * 1024
        );
    }

    #[test]
    fn memory_size_rejects_bad_values() {
        assert!(MemorySize::from_str("").is_err());
        assert!(MemorySize::from_str("abc").is_err());
        assert!(MemorySize::from_str("1Z").is_err());
    }

    #[test]
    fn address_parses_hex_and_decimal() {
        assert_eq!(Address::from_str("0x10").unwrap().raw(), 16);
        assert_eq!(Address::from_str("32").unwrap().raw(), 32);
    }
}
