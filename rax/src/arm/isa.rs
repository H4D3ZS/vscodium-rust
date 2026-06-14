//! ARM ISA version definitions.
//!
//! This module provides hierarchical ISA selection following ARM's architecture:
//! Profile -> Version -> Extensions

use clap::ValueEnum;
use serde::Deserialize;

// =============================================================================
// ARM Profiles
// =============================================================================

/// ARM architecture profile.
///
/// ARM processors are divided into three main profiles, each optimized
/// for different use cases with different exception models and features.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum ArmProfile {
    /// Application profile (Cortex-A series).
    /// - Full virtual memory support (MMU)
    /// - Multiple exception levels (EL0-EL3)
    /// - Designed for rich OS support (Linux, Android, iOS)
    A,
    /// Real-time profile (Cortex-R series).
    /// - Memory Protection Unit (MPU) instead of MMU
    /// - Deterministic interrupt latency
    /// - Designed for safety-critical systems (automotive, industrial)
    R,
    /// Microcontroller profile (Cortex-M series).
    /// - Simplified exception model (NVIC)
    /// - Thumb-only instruction set
    /// - Designed for embedded systems with low power/cost
    M,
}

impl ArmProfile {
    /// Returns true if this profile supports virtual memory (MMU).
    pub fn has_mmu(&self) -> bool {
        matches!(self, ArmProfile::A)
    }

    /// Returns true if this profile has an MPU.
    pub fn has_mpu(&self) -> bool {
        matches!(self, ArmProfile::R | ArmProfile::M)
    }

    /// Returns true if this profile uses the NVIC exception model.
    pub fn has_nvic(&self) -> bool {
        matches!(self, ArmProfile::M)
    }

    /// Returns true if this profile supports multiple exception levels.
    pub fn has_exception_levels(&self) -> bool {
        matches!(self, ArmProfile::A)
    }
}

// =============================================================================
// ARM Architecture Version (unified)
// =============================================================================

/// ARM architecture version.
///
/// This enum covers all ARM architecture versions across all profiles.
/// Not all versions are valid for all profiles - use `ArmCpuConfig` to
/// ensure valid combinations.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum ArmVersion {
    // ARMv6 family (A/R profiles only)
    /// ARMv6: ARM1136, ARM1176, ARM11MPCore
    V6,
    /// ARMv6T2: Adds Thumb-2 technology
    V6T2,
    /// ARMv6K: Kernel extensions (CLREX, barriers)
    V6K,
    /// ARMv6-M: Microcontroller (Cortex-M0, M0+, M1)
    V6M,

    // ARMv7 family
    /// ARMv7-A: Application (Cortex-A5, A7, A8, A9, A15, A17)
    V7A,
    /// ARMv7-R: Real-time (Cortex-R4, R5, R7, R8)
    V7R,
    /// ARMv7-M: Microcontroller (Cortex-M3)
    V7M,
    /// ARMv7E-M: Enhanced Microcontroller with DSP (Cortex-M4, M7)
    V7EM,

    // ARMv8 family - AArch64 capable
    /// ARMv8.0-A: First 64-bit ARM (Cortex-A53, A57, A72, A73)
    V8_0A,
    /// ARMv8.1-A: LSE atomics, VHE, PAN (Cortex-A75, A76)
    V8_1A,
    /// ARMv8.2-A: SVE, FP16, RAS (Cortex-A55, A75, A76, A77)
    V8_2A,
    /// ARMv8.3-A: PAC, FCMA, NV (Cortex-A77, A78)
    V8_3A,
    /// ARMv8.4-A: Flag manipulation, RCPC2 (Cortex-A78)
    V8_4A,
    /// ARMv8.5-A: BTI, MTE, RNG (Cortex-X1, A78C)
    V8_5A,
    /// ARMv8.6-A: BFloat16, I8MM (Cortex-X2, A710)
    V8_6A,
    /// ARMv8.7-A: WFIT, HBC (Cortex-X3)
    V8_7A,
    /// ARMv8.8-A: NMI, MOPS (Cortex-X4)
    V8_8A,
    /// ARMv8.9-A: Latest 8-series
    V8_9A,

    // ARMv8 R-profile
    /// ARMv8-R: Real-time with optional AArch64 (Cortex-R52, R52+)
    V8R,

    // ARMv8 M-profile
    /// ARMv8-M Baseline: TrustZone for M (Cortex-M23)
    V8MBaseline,
    /// ARMv8-M Mainline: Full features (Cortex-M33, M35P)
    V8MMainline,
    /// ARMv8.1-M: MVE/Helium (Cortex-M55, M85)
    V8_1M,

    // ARMv9 family
    /// ARMv9.0-A: Mandatory SVE2, RME (Cortex-A510, A710, X2)
    V9_0A,
    /// ARMv9.1-A: Enhanced BTI (Cortex-A715, X3)
    V9_1A,
    /// ARMv9.2-A: SME (Cortex-A720, X4)
    V9_2A,
    /// ARMv9.3-A: GCS, enhanced RME
    V9_3A,
    /// ARMv9.4-A: SME2
    V9_4A,
    /// ARMv9.5-A: Latest
    V9_5A,
}

impl Default for ArmVersion {
    fn default() -> Self {
        ArmVersion::V8_0A
    }
}

impl ArmVersion {
    /// Returns the profile this version belongs to.
    pub fn profile(&self) -> ArmProfile {
        match self {
            // A-profile versions
            ArmVersion::V6
            | ArmVersion::V6T2
            | ArmVersion::V6K
            | ArmVersion::V7A
            | ArmVersion::V8_0A
            | ArmVersion::V8_1A
            | ArmVersion::V8_2A
            | ArmVersion::V8_3A
            | ArmVersion::V8_4A
            | ArmVersion::V8_5A
            | ArmVersion::V8_6A
            | ArmVersion::V8_7A
            | ArmVersion::V8_8A
            | ArmVersion::V8_9A
            | ArmVersion::V9_0A
            | ArmVersion::V9_1A
            | ArmVersion::V9_2A
            | ArmVersion::V9_3A
            | ArmVersion::V9_4A
            | ArmVersion::V9_5A => ArmProfile::A,

            // R-profile versions
            ArmVersion::V7R | ArmVersion::V8R => ArmProfile::R,

            // M-profile versions
            ArmVersion::V6M
            | ArmVersion::V7M
            | ArmVersion::V7EM
            | ArmVersion::V8MBaseline
            | ArmVersion::V8MMainline
            | ArmVersion::V8_1M => ArmProfile::M,
        }
    }

    /// Returns true if this version supports AArch64 execution state.
    pub fn supports_aarch64(&self) -> bool {
        matches!(
            self,
            ArmVersion::V8_0A
                | ArmVersion::V8_1A
                | ArmVersion::V8_2A
                | ArmVersion::V8_3A
                | ArmVersion::V8_4A
                | ArmVersion::V8_5A
                | ArmVersion::V8_6A
                | ArmVersion::V8_7A
                | ArmVersion::V8_8A
                | ArmVersion::V8_9A
                | ArmVersion::V9_0A
                | ArmVersion::V9_1A
                | ArmVersion::V9_2A
                | ArmVersion::V9_3A
                | ArmVersion::V9_4A
                | ArmVersion::V9_5A
        )
    }

    /// Returns true if this version supports AArch32 execution state.
    pub fn supports_aarch32(&self) -> bool {
        // All A/R profile versions support 32-bit
        // Note: Some ARMv9 cores are AArch64-only
        match self.profile() {
            ArmProfile::A | ArmProfile::R => true,
            ArmProfile::M => false, // M-profile uses Thumb only
        }
    }

    /// Returns true if this version supports Thumb execution state.
    pub fn supports_thumb(&self) -> bool {
        // All versions except base ARMv6 support some form of Thumb
        !matches!(self, ArmVersion::V6)
    }

    /// Returns true if this version supports Thumb-2 (32-bit Thumb).
    pub fn supports_thumb2(&self) -> bool {
        match self {
            ArmVersion::V6 => false,
            ArmVersion::V6M => false,         // Only subset
            ArmVersion::V8MBaseline => false, // Only subset
            _ => true,
        }
    }

    /// Returns true if this is an ARMv9 version.
    pub fn is_v9(&self) -> bool {
        matches!(
            self,
            ArmVersion::V9_0A
                | ArmVersion::V9_1A
                | ArmVersion::V9_2A
                | ArmVersion::V9_3A
                | ArmVersion::V9_4A
                | ArmVersion::V9_5A
        )
    }

    /// Returns true if this version has mandatory SVE2.
    pub fn has_mandatory_sve2(&self) -> bool {
        self.is_v9()
    }

    /// Returns true if this version supports TrustZone.
    pub fn supports_trustzone(&self) -> bool {
        match self {
            // A-profile: v6Z and later (we treat v6K+ as having optional TZ)
            ArmVersion::V6K
            | ArmVersion::V7A
            | ArmVersion::V8_0A
            | ArmVersion::V8_1A
            | ArmVersion::V8_2A
            | ArmVersion::V8_3A
            | ArmVersion::V8_4A
            | ArmVersion::V8_5A
            | ArmVersion::V8_6A
            | ArmVersion::V8_7A
            | ArmVersion::V8_8A
            | ArmVersion::V8_9A
            | ArmVersion::V9_0A
            | ArmVersion::V9_1A
            | ArmVersion::V9_2A
            | ArmVersion::V9_3A
            | ArmVersion::V9_4A
            | ArmVersion::V9_5A => true,
            // M-profile: v8-M and later
            ArmVersion::V8MBaseline | ArmVersion::V8MMainline | ArmVersion::V8_1M => true,
            _ => false,
        }
    }

    /// Returns true if this version supports virtualization extensions.
    pub fn supports_virtualization(&self) -> bool {
        match self {
            // A-profile: v7-A with Virt and all v8+
            ArmVersion::V8_0A
            | ArmVersion::V8_1A
            | ArmVersion::V8_2A
            | ArmVersion::V8_3A
            | ArmVersion::V8_4A
            | ArmVersion::V8_5A
            | ArmVersion::V8_6A
            | ArmVersion::V8_7A
            | ArmVersion::V8_8A
            | ArmVersion::V8_9A
            | ArmVersion::V9_0A
            | ArmVersion::V9_1A
            | ArmVersion::V9_2A
            | ArmVersion::V9_3A
            | ArmVersion::V9_4A
            | ArmVersion::V9_5A => true,
            // R-profile: v8-R
            ArmVersion::V8R => true,
            _ => false,
        }
    }

    /// Returns the major version number (6, 7, 8, or 9).
    pub fn major_version(&self) -> u8 {
        match self {
            ArmVersion::V6 | ArmVersion::V6T2 | ArmVersion::V6K | ArmVersion::V6M => 6,
            ArmVersion::V7A | ArmVersion::V7R | ArmVersion::V7M | ArmVersion::V7EM => 7,
            ArmVersion::V8_0A
            | ArmVersion::V8_1A
            | ArmVersion::V8_2A
            | ArmVersion::V8_3A
            | ArmVersion::V8_4A
            | ArmVersion::V8_5A
            | ArmVersion::V8_6A
            | ArmVersion::V8_7A
            | ArmVersion::V8_8A
            | ArmVersion::V8_9A
            | ArmVersion::V8R
            | ArmVersion::V8MBaseline
            | ArmVersion::V8MMainline
            | ArmVersion::V8_1M => 8,
            ArmVersion::V9_0A
            | ArmVersion::V9_1A
            | ArmVersion::V9_2A
            | ArmVersion::V9_3A
            | ArmVersion::V9_4A
            | ArmVersion::V9_5A => 9,
        }
    }

    /// Returns the minor version number within the major version.
    pub fn minor_version(&self) -> u8 {
        match self {
            // ARMv8.x
            ArmVersion::V8_0A => 0,
            ArmVersion::V8_1A | ArmVersion::V8_1M => 1,
            ArmVersion::V8_2A => 2,
            ArmVersion::V8_3A => 3,
            ArmVersion::V8_4A => 4,
            ArmVersion::V8_5A => 5,
            ArmVersion::V8_6A => 6,
            ArmVersion::V8_7A => 7,
            ArmVersion::V8_8A => 8,
            ArmVersion::V8_9A => 9,
            // ARMv9.x
            ArmVersion::V9_0A => 0,
            ArmVersion::V9_1A => 1,
            ArmVersion::V9_2A => 2,
            ArmVersion::V9_3A => 3,
            ArmVersion::V9_4A => 4,
            ArmVersion::V9_5A => 5,
            // Others don't have minor versions
            _ => 0,
        }
    }
}

// =============================================================================
// ARM CPU Configuration
// =============================================================================

/// Endianness configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Deserialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum ArmEndianness {
    /// Little-endian (default for most ARM systems).
    #[default]
    Little,
    /// Big-endian.
    Big,
    /// Mixed-endian (BE-8 for data, LE for instructions).
    /// Used by some ARMv6+ systems.
    Mixed,
}

/// Exception level configuration for A-profile.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExceptionLevelConfig {
    /// Highest implemented exception level (1, 2, or 3).
    pub max_el: u8,
    /// EL2 is implemented (hypervisor support).
    pub has_el2: bool,
    /// EL3 is implemented (secure monitor).
    pub has_el3: bool,
    /// Secure state is implemented.
    pub has_secure_state: bool,
}

impl Default for ExceptionLevelConfig {
    fn default() -> Self {
        ExceptionLevelConfig {
            max_el: 1,
            has_el2: false,
            has_el3: false,
            has_secure_state: false,
        }
    }
}

impl ExceptionLevelConfig {
    /// Full virtualization support (EL2 + EL3).
    pub fn full() -> Self {
        ExceptionLevelConfig {
            max_el: 3,
            has_el2: true,
            has_el3: true,
            has_secure_state: true,
        }
    }

    /// Hypervisor support only (EL2, no EL3).
    pub fn hypervisor() -> Self {
        ExceptionLevelConfig {
            max_el: 2,
            has_el2: true,
            has_el3: false,
            has_secure_state: false,
        }
    }
}

/// Complete ARM CPU configuration.
#[derive(Clone, Debug)]
pub struct ArmCpuConfig {
    /// Architecture version (determines profile implicitly).
    pub version: ArmVersion,
    /// Supported execution states.
    pub supported_states: SupportedExecutionStates,
    /// Enabled optional features.
    pub features: super::ArmFeatures,
    /// Endianness configuration.
    pub endianness: ArmEndianness,
    /// Exception level configuration (A-profile only).
    pub el_config: Option<ExceptionLevelConfig>,
    /// Number of MPU regions (R-profile and M-profile).
    pub mpu_regions: Option<u8>,
    /// NVIC priority bits (M-profile only, 2-8).
    pub nvic_priority_bits: Option<u8>,
    /// SVE vector length in bits (128-2048, must be multiple of 128).
    pub sve_vl: Option<u16>,
    /// SME vector length in bits.
    pub sme_vl: Option<u16>,
}

impl Default for ArmCpuConfig {
    fn default() -> Self {
        ArmCpuConfig {
            version: ArmVersion::V8_0A,
            supported_states: SupportedExecutionStates::AARCH64 | SupportedExecutionStates::AARCH32,
            features: super::ArmFeatures::empty(),
            endianness: ArmEndianness::Little,
            el_config: Some(ExceptionLevelConfig::default()),
            mpu_regions: None,
            nvic_priority_bits: None,
            sve_vl: None,
            sme_vl: None,
        }
    }
}

impl ArmCpuConfig {
    /// Create a new configuration for the given version with default settings.
    pub fn new(version: ArmVersion) -> Self {
        let mut config = ArmCpuConfig {
            version,
            ..Default::default()
        };

        // Set supported states based on version
        config.supported_states = match version.profile() {
            ArmProfile::A => {
                if version.supports_aarch64() {
                    SupportedExecutionStates::AARCH64 | SupportedExecutionStates::AARCH32
                } else {
                    SupportedExecutionStates::ARM | SupportedExecutionStates::THUMB
                }
            }
            ArmProfile::R => {
                if matches!(version, ArmVersion::V8R) {
                    // Cortex-R82 can be AArch64
                    SupportedExecutionStates::ARM | SupportedExecutionStates::THUMB
                } else {
                    SupportedExecutionStates::ARM | SupportedExecutionStates::THUMB
                }
            }
            ArmProfile::M => {
                if version.supports_thumb2() {
                    SupportedExecutionStates::THUMB | SupportedExecutionStates::THUMB2
                } else {
                    SupportedExecutionStates::THUMB
                }
            }
        };

        // Set profile-specific defaults
        match version.profile() {
            ArmProfile::A => {
                config.el_config = Some(ExceptionLevelConfig::default());
            }
            ArmProfile::R => {
                config.mpu_regions = Some(16); // Typical for Cortex-R
            }
            ArmProfile::M => {
                config.mpu_regions = Some(8); // Typical for Cortex-M
                config.nvic_priority_bits = Some(4); // Common default
            }
        }

        config
    }

    /// Get the profile for this configuration.
    pub fn profile(&self) -> ArmProfile {
        self.version.profile()
    }

    /// Check if a specific execution state is supported.
    pub fn supports_state(&self, state: ExecutionState) -> bool {
        match state {
            ExecutionState::Arm => self
                .supported_states
                .contains(SupportedExecutionStates::ARM),
            ExecutionState::Thumb => self
                .supported_states
                .contains(SupportedExecutionStates::THUMB),
            ExecutionState::Thumb2 => self
                .supported_states
                .contains(SupportedExecutionStates::THUMB2),
            ExecutionState::Aarch32 => self
                .supported_states
                .contains(SupportedExecutionStates::AARCH32),
            ExecutionState::Aarch64 => self
                .supported_states
                .contains(SupportedExecutionStates::AARCH64),
        }
    }

    /// Validate the configuration for consistency.
    pub fn validate(&self) -> Result<(), &'static str> {
        // Check that supported states match version capabilities
        if self
            .supported_states
            .contains(SupportedExecutionStates::AARCH64)
            && !self.version.supports_aarch64()
        {
            return Err("AArch64 not supported by this version");
        }

        // Check SVE vector length
        if let Some(vl) = self.sve_vl {
            if vl < 128 || vl > 2048 || vl % 128 != 0 {
                return Err("SVE VL must be 128-2048 and multiple of 128");
            }
        }

        // Check NVIC priority bits (M-profile only)
        if let Some(bits) = self.nvic_priority_bits {
            if self.version.profile() != ArmProfile::M {
                return Err("NVIC priority bits only valid for M-profile");
            }
            if bits < 2 || bits > 8 {
                return Err("NVIC priority bits must be 2-8");
            }
        }

        Ok(())
    }
}

// =============================================================================
// Execution States
// =============================================================================

/// ARM execution state (runtime).
///
/// This represents the current instruction set state during execution.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ExecutionState {
    /// 32-bit ARM instruction set (A32).
    Arm,
    /// 16-bit Thumb instruction set (T16).
    Thumb,
    /// Mixed 16/32-bit Thumb-2 instruction set (T32).
    Thumb2,
    /// ARMv8+ 32-bit state (AArch32, similar to ARM but with v8 features).
    Aarch32,
    /// ARMv8+ 64-bit state (AArch64).
    Aarch64,
}

impl ExecutionState {
    /// Returns the instruction width in bytes (minimum for variable-length).
    pub fn min_instruction_size(&self) -> u8 {
        match self {
            ExecutionState::Arm | ExecutionState::Aarch32 => 4,
            ExecutionState::Thumb | ExecutionState::Thumb2 => 2,
            ExecutionState::Aarch64 => 4,
        }
    }

    /// Returns true if this is a 64-bit state.
    pub fn is_64bit(&self) -> bool {
        matches!(self, ExecutionState::Aarch64)
    }

    /// Returns true if this is a Thumb variant.
    pub fn is_thumb(&self) -> bool {
        matches!(self, ExecutionState::Thumb | ExecutionState::Thumb2)
    }
}

bitflags::bitflags! {
    /// Supported execution states (configuration).
    ///
    /// This represents which execution states a CPU implementation supports,
    /// not the current runtime state.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub struct SupportedExecutionStates: u8 {
        /// 32-bit ARM state (A32).
        const ARM = 1 << 0;
        /// 16-bit Thumb state (T16).
        const THUMB = 1 << 1;
        /// Thumb-2 state (T32, mixed 16/32-bit).
        const THUMB2 = 1 << 2;
        /// AArch32 state (ARMv8+ 32-bit).
        const AARCH32 = 1 << 3;
        /// AArch64 state (ARMv8+ 64-bit).
        const AARCH64 = 1 << 4;
    }
}

impl SupportedExecutionStates {
    /// Standard 32-bit ARM configuration.
    pub fn arm32() -> Self {
        Self::ARM | Self::THUMB | Self::THUMB2
    }

    /// Standard 64-bit ARM configuration with 32-bit support.
    pub fn arm64_with_aarch32() -> Self {
        Self::AARCH64 | Self::AARCH32
    }

    /// 64-bit only configuration (some ARMv9 cores).
    pub fn arm64_only() -> Self {
        Self::AARCH64
    }

    /// Cortex-M configuration (Thumb only).
    pub fn cortex_m() -> Self {
        Self::THUMB | Self::THUMB2
    }

    /// Cortex-M baseline configuration (subset Thumb only).
    pub fn cortex_m_baseline() -> Self {
        Self::THUMB
    }
}

// =============================================================================
// Processor Modes (AArch32)
// =============================================================================

/// AArch32 processor modes.
///
/// These are the privilege/exception modes for 32-bit ARM, encoded in CPSR[4:0].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Aarch32Mode {
    /// User mode (unprivileged).
    User = 0b10000,
    /// FIQ mode (fast interrupt).
    Fiq = 0b10001,
    /// IRQ mode (normal interrupt).
    Irq = 0b10010,
    /// Supervisor mode (SVC/SWI).
    Supervisor = 0b10011,
    /// Monitor mode (secure monitor, TrustZone).
    Monitor = 0b10110,
    /// Abort mode (memory fault).
    Abort = 0b10111,
    /// Hypervisor mode (virtualization).
    Hyp = 0b11010,
    /// Undefined mode (undefined instruction).
    Undefined = 0b11011,
    /// System mode (privileged, shares regs with User).
    System = 0b11111,
}

impl Aarch32Mode {
    /// Try to decode a mode from the CPSR mode bits.
    pub fn from_bits(bits: u8) -> Option<Self> {
        match bits & 0x1F {
            0b10000 => Some(Aarch32Mode::User),
            0b10001 => Some(Aarch32Mode::Fiq),
            0b10010 => Some(Aarch32Mode::Irq),
            0b10011 => Some(Aarch32Mode::Supervisor),
            0b10110 => Some(Aarch32Mode::Monitor),
            0b10111 => Some(Aarch32Mode::Abort),
            0b11010 => Some(Aarch32Mode::Hyp),
            0b11011 => Some(Aarch32Mode::Undefined),
            0b11111 => Some(Aarch32Mode::System),
            _ => None,
        }
    }

    /// Get the mode bits for CPSR.
    pub fn bits(&self) -> u8 {
        *self as u8
    }

    /// Returns true if this is a privileged mode.
    pub fn is_privileged(&self) -> bool {
        !matches!(self, Aarch32Mode::User)
    }

    /// Returns true if this mode has banked SP and LR.
    pub fn has_banked_sp_lr(&self) -> bool {
        !matches!(self, Aarch32Mode::User | Aarch32Mode::System)
    }

    /// Returns true if this mode has a banked SPSR.
    pub fn has_spsr(&self) -> bool {
        !matches!(self, Aarch32Mode::User | Aarch32Mode::System)
    }
}

// =============================================================================
// Exception Levels (AArch64)
// =============================================================================

/// AArch64 exception levels.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ExceptionLevel {
    /// EL0: User/Application level (unprivileged).
    El0 = 0,
    /// EL1: OS/Kernel level.
    El1 = 1,
    /// EL2: Hypervisor level.
    El2 = 2,
    /// EL3: Secure Monitor level.
    El3 = 3,
}

impl ExceptionLevel {
    /// Create from raw value (0-3).
    pub fn from_raw(value: u8) -> Option<Self> {
        match value {
            0 => Some(ExceptionLevel::El0),
            1 => Some(ExceptionLevel::El1),
            2 => Some(ExceptionLevel::El2),
            3 => Some(ExceptionLevel::El3),
            _ => None,
        }
    }

    /// Get raw value (0-3).
    pub fn raw(&self) -> u8 {
        *self as u8
    }

    /// Returns true if this is a privileged level (EL1+).
    pub fn is_privileged(&self) -> bool {
        *self != ExceptionLevel::El0
    }

    /// Returns true if this is the hypervisor level.
    pub fn is_hypervisor(&self) -> bool {
        *self == ExceptionLevel::El2
    }

    /// Returns true if this is the secure monitor level.
    pub fn is_secure_monitor(&self) -> bool {
        *self == ExceptionLevel::El3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_profile() {
        assert_eq!(ArmVersion::V8_0A.profile(), ArmProfile::A);
        assert_eq!(ArmVersion::V7R.profile(), ArmProfile::R);
        assert_eq!(ArmVersion::V7M.profile(), ArmProfile::M);
    }

    #[test]
    fn version_aarch64_support() {
        assert!(ArmVersion::V8_0A.supports_aarch64());
        assert!(ArmVersion::V9_0A.supports_aarch64());
        assert!(!ArmVersion::V7A.supports_aarch64());
        assert!(!ArmVersion::V7M.supports_aarch64());
    }

    #[test]
    fn mode_decoding() {
        assert_eq!(Aarch32Mode::from_bits(0b10000), Some(Aarch32Mode::User));
        assert_eq!(
            Aarch32Mode::from_bits(0b10011),
            Some(Aarch32Mode::Supervisor)
        );
        assert_eq!(Aarch32Mode::from_bits(0b00000), None);
    }

    #[test]
    fn exception_level() {
        assert_eq!(ExceptionLevel::from_raw(0), Some(ExceptionLevel::El0));
        assert_eq!(ExceptionLevel::from_raw(3), Some(ExceptionLevel::El3));
        assert_eq!(ExceptionLevel::from_raw(4), None);
    }
}
