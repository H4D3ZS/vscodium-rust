//! AVX10.1 and AVX10.2 Instruction Tests
//!
//! AVX10 is Intel's unified vector instruction set that converges AVX-512 features
//! into a single, consistent feature set available across different vector widths
//! (128-bit, 256-bit, and optionally 512-bit).
//!
//! AVX10.1 includes:
//! - VNNI (Vector Neural Network Instructions) - VPDPBUSD, VPDPBUSDS, VPDPWSSD, VPDPWSSDS
//! - IFMA (Integer Fused Multiply-Add) - VPMADD52LUQ, VPMADD52HUQ
//! - VBMI/VBMI2 (Vector Byte Manipulation) - VPERMB, VPERMI2B, VPCOMPRESSB, etc.
//! - VPOPCNTDQ (Population Count) - VPOPCNTD, VPOPCNTQ
//! - BITALG (Bit Algorithms) - VPSHUFBITQMB, VPOPCNTB, VPOPCNTW
//! - BF16 (Brain Float 16) - VCVTNEPS2BF16, VCVTNE2PS2BF16, VDPBF16PS
//! - FP16 operations with embedded rounding on all vector widths
//! - YMM embedded rounding (new capability)
//!
//! AVX10.2 adds:
//! - VMINMAXPS/PD/PH - Min/max with comparison predicate
//! - VMINMAXSS/SD/SH - Scalar min/max variants
//! - Saturation conversions (VCVT* with saturation)
//! - VMPSADBW - Multiple packed sum of absolute differences
//! - VCOMSBF16/VUCOMSBF16 - BF16 scalar comparisons
//! - Copy-sign and zero-based instructions
//! - Additional media acceleration instructions

// AVX10.1 modules
mod vnni;
mod ifma;
mod vbmi;
mod vpopcntdq;
mod bitalg;
mod bf16;
mod ymm_embedded_rounding;

// AVX10.2 modules
mod minmax;
mod saturation_convert;
mod media_accel;
mod vmpsadbw;
mod compare_bf16;
mod copy_sign;
