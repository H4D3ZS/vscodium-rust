//! ARM feature flags for optional ISA extensions.
//!
//! This module defines all optional ARM architecture features that can be
//! enabled independently of the base architecture version.

use bitflags::bitflags;

bitflags! {
    /// ARM optional feature flags.
    ///
    /// These represent ISA extensions that may or may not be present on a
    /// specific implementation. Features are grouped by category.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub struct ArmFeatures: u64 {
        // =====================================================================
        // Cryptographic Extensions (bits 0-7)
        // =====================================================================

        /// AES instructions (AESE, AESD, AESMC, AESIMC).
        /// Part of the ARMv8 Cryptographic Extension.
        const CRYPTO_AES = 1 << 0;

        /// SHA-1 instructions (SHA1C, SHA1P, SHA1M, SHA1H, SHA1SU0, SHA1SU1).
        const CRYPTO_SHA1 = 1 << 1;

        /// SHA-256 instructions (SHA256H, SHA256H2, SHA256SU0, SHA256SU1).
        const CRYPTO_SHA256 = 1 << 2;

        /// SHA-512 instructions (FEAT_SHA512, ARMv8.2+).
        const CRYPTO_SHA512 = 1 << 3;

        /// SHA-3 instructions (FEAT_SHA3, ARMv8.2+).
        const CRYPTO_SHA3 = 1 << 4;

        /// SM3 Chinese cryptographic hash (FEAT_SM3, ARMv8.2+).
        const CRYPTO_SM3 = 1 << 5;

        /// SM4 Chinese block cipher (FEAT_SM4, ARMv8.2+).
        const CRYPTO_SM4 = 1 << 6;

        /// CRC32 instructions (CRC32B, CRC32H, CRC32W, CRC32X, CRC32C*).
        /// Mandatory from ARMv8.1-A.
        const CRC32 = 1 << 7;

        // =====================================================================
        // SIMD/Vector Extensions (bits 8-23)
        // =====================================================================

        /// NEON/Advanced SIMD.
        /// Optional in ARMv7, mandatory in ARMv8-A.
        const NEON = 1 << 8;

        /// Half-precision floating-point (FEAT_FP16).
        /// ARMv8.2+ optional, enables FP16 arithmetic.
        const FP16 = 1 << 9;

        /// BFloat16 format (FEAT_BF16, ARMv8.6+).
        /// ML-optimized 16-bit format.
        const BF16 = 1 << 10;

        /// Int8 matrix multiply (FEAT_I8MM, ARMv8.6+).
        /// SMMLA, UMMLA, USMMLA instructions.
        const I8MM = 1 << 11;

        /// Dot product instructions (FEAT_DotProd, ARMv8.2+).
        /// SDOT, UDOT for ML inference.
        const DOTPROD = 1 << 12;

        /// Scalable Vector Extension (FEAT_SVE).
        /// Optional in ARMv8.2+, scalable 128-2048 bit vectors.
        const SVE = 1 << 13;

        /// SVE2 (FEAT_SVE2, mandatory in ARMv9).
        /// Enhanced SVE with more operations.
        const SVE2 = 1 << 14;

        /// SVE2 AES instructions (FEAT_SVE_AES).
        const SVE2_AES = 1 << 15;

        /// SVE2 SHA3 instructions (FEAT_SVE_SHA3).
        const SVE2_SHA3 = 1 << 16;

        /// SVE2 SM4 instructions (FEAT_SVE_SM4).
        const SVE2_SM4 = 1 << 17;

        /// SVE2 bit permute instructions (FEAT_SVE_BitPerm).
        const SVE2_BITPERM = 1 << 18;

        /// Scalable Matrix Extension (FEAT_SME, ARMv9.2+).
        /// Matrix operations, streaming SVE mode.
        const SME = 1 << 19;

        /// SME2 (FEAT_SME2, ARMv9.4+).
        /// Enhanced SME with more tiles.
        const SME2 = 1 << 20;

        /// FRINTTS - floating-point round to int (FEAT_FRINTTS).
        const FRINTTS = 1 << 21;

        /// JavaScript conversion (FEAT_JSCVT, ARMv8.3+).
        /// FJCVTZS instruction for JS number conversion.
        const JSCVT = 1 << 22;

        /// Complex number instructions (FEAT_FCMA, ARMv8.3+).
        /// FCMLA, FCADD for complex multiply-add.
        const FCMA = 1 << 23;

        // =====================================================================
        // Atomics and Memory Ordering (bits 24-31)
        // =====================================================================

        /// Large System Extensions atomics (FEAT_LSE, ARMv8.1+).
        /// CAS, SWP, LDADD, STADD, etc.
        const LSE = 1 << 24;

        /// LSE2 - larger atomics (FEAT_LSE2, ARMv8.4+).
        /// 16-byte atomic operations.
        const LSE2 = 1 << 25;

        /// Release Consistent Processor Consistent (FEAT_LRCPC).
        /// LDAPR instructions for C++11 acquire semantics.
        const RCPC = 1 << 26;

        /// RCPC2 (FEAT_LRCPC2, ARMv8.4+).
        /// LDAPUR/STLUR with immediate offsets.
        const RCPC2 = 1 << 27;

        /// RCPC3 (FEAT_LRCPC3).
        const RCPC3 = 1 << 28;

        /// LSE128 - 128-bit atomics (FEAT_LSE128).
        const LSE128 = 1 << 29;

        // =====================================================================
        // Pointer/Control Flow Security (bits 32-39)
        // =====================================================================

        /// Pointer Authentication - address keys (FEAT_PAuth).
        /// PACIA, PACIB, AUTIA, AUTIB, etc.
        const PACA = 1 << 32;

        /// Pointer Authentication - generic key (FEAT_PAuth).
        /// PACGA instruction.
        const PACG = 1 << 33;

        /// Branch Target Identification (FEAT_BTI, ARMv8.5+).
        /// BTI instruction, PSTATE.BTYPE.
        const BTI = 1 << 34;

        /// Memory Tagging Extension (FEAT_MTE, ARMv8.5+).
        /// Hardware memory tagging for safety.
        const MTE = 1 << 35;

        /// MTE2 - asymmetric tag checking (FEAT_MTE2).
        const MTE2 = 1 << 36;

        /// MTE3 - enhanced MTE (FEAT_MTE3).
        const MTE3 = 1 << 37;

        /// Guarded Control Stack (FEAT_GCS, ARMv9.3+).
        /// Hardware shadow stack.
        const GCS = 1 << 38;

        // =====================================================================
        // Virtualization (bits 40-47)
        // =====================================================================

        /// Virtualization Host Extensions (FEAT_VHE, ARMv8.1+).
        /// Run host OS at EL2.
        const VHE = 1 << 40;

        /// Nested Virtualization (FEAT_NV, ARMv8.3+).
        /// Nested hypervisor support.
        const NV = 1 << 41;

        /// NV2 - enhanced nested virt (FEAT_NV2, ARMv8.4+).
        const NV2 = 1 << 42;

        /// Realm Management Extension (FEAT_RME, ARMv9+).
        /// Confidential computing.
        const RME = 1 << 43;

        /// Secure EL2 (FEAT_SEL2, ARMv8.4+).
        /// Hypervisor in secure world.
        const SEL2 = 1 << 44;

        // =====================================================================
        // Miscellaneous Features (bits 48-55)
        // =====================================================================

        /// Hardware random number generator (FEAT_RNG, ARMv8.5+).
        /// RNDR, RNDRRS instructions.
        const RNG = 1 << 48;

        /// Data Independent Timing (FEAT_DIT, ARMv8.4+).
        /// Constant-time execution mode.
        const DIT = 1 << 49;

        /// Speculative Store Bypass Safe (FEAT_SSBS, ARMv8.0+).
        /// PSTATE.SSBS for Spectre mitigation.
        const SSBS = 1 << 50;

        /// Speculation Barrier (FEAT_SB, ARMv8.0+).
        /// SB instruction.
        const SB = 1 << 51;

        /// Memory copy/set operations (FEAT_MOPS, ARMv8.8+).
        /// CPY*, SET* instructions.
        const MOPS = 1 << 52;

        /// Hinted Conditional Branches (FEAT_HBC, ARMv8.7+).
        /// BC.cond instruction.
        const HBC = 1 << 53;

        /// Non-Maskable Interrupts (FEAT_NMI, ARMv8.8+).
        const NMI = 1 << 54;

        /// Reliability, Availability, Serviceability (FEAT_RAS).
        const RAS = 1 << 55;

        // =====================================================================
        // VFP/FPU Variants - 32-bit (bits 56-62)
        // =====================================================================

        /// VFPv3-D16: 16 double-precision registers.
        const VFPV3_D16 = 1 << 56;

        /// VFPv3-D32: 32 double-precision registers.
        const VFPV3_D32 = 1 << 57;

        /// VFPv4: fused multiply-add.
        const VFPV4 = 1 << 58;

        /// Single-precision FPU only.
        const FP_SP = 1 << 59;

        /// Double-precision FPU.
        const FP_DP = 1 << 60;

        // =====================================================================
        // Cortex-M Specific (bit 61-62)
        // =====================================================================

        /// MVE (M-profile Vector Extension / Helium).
        /// Cortex-M55, M85.
        const MVE = 1 << 61;

        /// MVE with floating-point.
        const MVE_FP = 1 << 62;

        // =====================================================================
        // Basic Architecture Features (bit 63 and reusing lower bits)
        // Note: Using available bit positions
        // =====================================================================

        /// Thumb instruction set support.
        const THUMB = 1 << 63;
    }
}

// Additional feature flags using a separate bitflags for extended features
// that don't fit in the 64-bit mask, or we define simple constants

impl ArmFeatures {
    /// Thumb-2 instruction set (32-bit Thumb encodings).
    pub const THUMB2: Self = Self::empty(); // Implied by VFPV4 or M3+

    /// VFP floating-point extension.
    pub const VFP: Self = Self::VFPV4;

    /// VFP with 32 double-precision registers.
    pub const VFP_D32: Self = Self::VFPV3_D32;

    /// DSP extension (saturating arithmetic, SIMD operations).
    pub const DSP: Self = Self::NEON; // Use NEON as proxy for DSP

    /// TrustZone security extensions.
    pub const TRUSTZONE: Self = Self::SEL2; // Use SEL2 as proxy
}

impl ArmFeatures {
    /// Create a feature set for a basic ARMv8.0-A implementation.
    pub fn armv8_0_base() -> Self {
        Self::NEON | Self::FP_DP
    }

    /// Create a feature set for ARMv8.1-A (adds LSE, CRC32).
    pub fn armv8_1_base() -> Self {
        Self::armv8_0_base() | Self::LSE | Self::CRC32
    }

    /// Create a feature set for ARMv8.2-A (adds FP16, DotProd optional).
    pub fn armv8_2_base() -> Self {
        Self::armv8_1_base() | Self::FP16 | Self::RAS
    }

    /// Create a feature set for ARMv8.3-A (adds PAC, FCMA, JSCVT).
    pub fn armv8_3_base() -> Self {
        Self::armv8_2_base() | Self::PACA | Self::PACG | Self::FCMA | Self::JSCVT
    }

    /// Create a feature set for ARMv8.4-A (adds LSE2, DIT, NV2).
    pub fn armv8_4_base() -> Self {
        Self::armv8_3_base() | Self::LSE2 | Self::DIT | Self::RCPC2 | Self::NV2
    }

    /// Create a feature set for ARMv8.5-A (adds BTI, MTE, RNG, SB).
    pub fn armv8_5_base() -> Self {
        Self::armv8_4_base() | Self::BTI | Self::RNG | Self::SB | Self::SSBS
    }

    /// Create a feature set for ARMv8.6-A (adds BF16, I8MM).
    pub fn armv8_6_base() -> Self {
        Self::armv8_5_base() | Self::BF16 | Self::I8MM
    }

    /// Create a feature set for ARMv9.0-A (adds mandatory SVE2).
    pub fn armv9_0_base() -> Self {
        Self::armv8_5_base() | Self::SVE | Self::SVE2
    }

    /// Create a feature set for ARMv9.2-A (adds SME).
    pub fn armv9_2_base() -> Self {
        Self::armv9_0_base() | Self::SME | Self::BF16 | Self::I8MM
    }

    /// Create a feature set for a Cortex-M4/M7 with FPU.
    pub fn cortex_m4_fpu() -> Self {
        Self::VFPV4 | Self::FP_SP
    }

    /// Create a feature set for a Cortex-M7 with double-precision FPU.
    pub fn cortex_m7_dp_fpu() -> Self {
        Self::VFPV4 | Self::FP_SP | Self::FP_DP
    }

    /// Create a feature set for a Cortex-M55 with MVE.
    pub fn cortex_m55() -> Self {
        Self::MVE | Self::MVE_FP | Self::FP_SP | Self::FP_DP
    }

    /// Check if any crypto feature is enabled.
    pub fn has_crypto(&self) -> bool {
        self.intersects(
            Self::CRYPTO_AES
                | Self::CRYPTO_SHA1
                | Self::CRYPTO_SHA256
                | Self::CRYPTO_SHA512
                | Self::CRYPTO_SHA3
                | Self::CRYPTO_SM3
                | Self::CRYPTO_SM4,
        )
    }

    /// Check if any SVE feature is enabled.
    pub fn has_sve(&self) -> bool {
        self.intersects(Self::SVE | Self::SVE2)
    }

    /// Check if any SME feature is enabled.
    pub fn has_sme(&self) -> bool {
        self.intersects(Self::SME | Self::SME2)
    }

    /// Check if pointer authentication is enabled.
    pub fn has_pac(&self) -> bool {
        self.intersects(Self::PACA | Self::PACG)
    }

    /// Check if memory tagging is enabled.
    pub fn has_mte(&self) -> bool {
        self.intersects(Self::MTE | Self::MTE2 | Self::MTE3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn feature_sets() {
        let v8_0 = ArmFeatures::armv8_0_base();
        assert!(v8_0.contains(ArmFeatures::NEON));
        assert!(!v8_0.contains(ArmFeatures::LSE));

        let v8_1 = ArmFeatures::armv8_1_base();
        assert!(v8_1.contains(ArmFeatures::LSE));
        assert!(v8_1.contains(ArmFeatures::CRC32));
    }

    #[test]
    fn feature_queries() {
        let features = ArmFeatures::CRYPTO_AES | ArmFeatures::SVE2 | ArmFeatures::PACA;
        assert!(features.has_crypto());
        assert!(features.has_sve());
        assert!(features.has_pac());
        assert!(!features.has_mte());
    }
}
