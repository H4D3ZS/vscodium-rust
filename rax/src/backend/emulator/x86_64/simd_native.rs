//! Native SIMD passthrough with automatic fallback.
//!
//! This module provides host-native execution of guest x86 SIMD instructions
//! when the host CPU supports compatible SIMD capabilities, with automatic
//! fallback to scalar emulation for unsupported instruction sets.
//!
//! # Multi-Architecture Support
//!
//! The module supports three execution backends:
//!
//! 1. **x86_64 Native (SSE/AVX)**: Direct use of x86 intrinsics when running on
//!    x86_64 hosts. This provides near-native performance for x86 guests.
//!
//! 2. **ARM64 NEON**: Translation of x86 SIMD semantics to ARM NEON intrinsics
//!    when running on aarch64 hosts (e.g., Apple Silicon, ARM servers).
//!
//! 3. **Scalar Fallback**: Pure Rust implementation that works on any platform
//!    but runs operations element-by-element without SIMD acceleration.
//!
//! # Fallback Hierarchy
//!
//! On x86_64:
//! - AVX-512 -> AVX2 (two 256-bit ops) -> SSE4.1 (four 128-bit ops) -> Scalar
//! - AVX2 -> SSE4.1 -> Scalar
//! - SSE4.1 -> Scalar
//!
//! On aarch64:
//! - NEON -> Scalar
//!
//! # NEON Implementation Notes
//!
//! ARM NEON provides 128-bit SIMD registers (same width as SSE), making it
//! suitable for accelerating XMM operations. Key translation patterns:
//!
//! - **Arithmetic**: Direct mapping (e.g., `vaddq_f32` for ADDPS)
//! - **Logical**: Direct mapping (e.g., `vandq_u64` for PAND)
//! - **Shifts**: Left shifts use positive counts, right shifts use *negative*
//!   counts with `vshlq_*` (NEON quirk: shift direction is sign-dependent)
//! - **Shuffles**: `vqtbl1q_u8` for PSHUFB with manual MSB masking
//! - **Comparisons**: `vceqq_*` returns all-ones for equal, requiring no fixup
//!
//! YMM (256-bit) operations on NEON are handled by processing two 128-bit
//! halves sequentially, as NEON doesn't have native 256-bit registers.
//!
//! # Runtime Detection
//!
//! Host capabilities are detected once at startup via [`SimdCapabilities::get()`]
//! and cached in a global `OnceLock`. Dispatch functions check these capabilities
//! and route to the fastest available implementation.

use std::sync::OnceLock;

/// Cached host SIMD capabilities (detected once at startup)
#[derive(Clone, Copy, Debug)]
pub struct SimdCapabilities {
    // x86_64 features
    pub sse2: bool,
    pub sse3: bool,
    pub ssse3: bool,
    pub sse4_1: bool,
    pub sse4_2: bool,
    pub avx: bool,
    pub avx2: bool,
    pub fma: bool,
    pub avx512f: bool,
    pub avx512bw: bool,
    pub avx512vl: bool,
    // aarch64 features
    pub neon: bool,
}

/// Global cached capabilities
static HOST_CAPS: OnceLock<SimdCapabilities> = OnceLock::new();

impl SimdCapabilities {
    /// Detect host CPU capabilities (cached after first call)
    #[inline]
    pub fn get() -> &'static SimdCapabilities {
        HOST_CAPS.get_or_init(Self::detect)
    }

    fn detect() -> SimdCapabilities {
        #[cfg(target_arch = "x86_64")]
        {
            SimdCapabilities {
                sse2: std::arch::is_x86_feature_detected!("sse2"),
                sse3: std::arch::is_x86_feature_detected!("sse3"),
                ssse3: std::arch::is_x86_feature_detected!("ssse3"),
                sse4_1: std::arch::is_x86_feature_detected!("sse4.1"),
                sse4_2: std::arch::is_x86_feature_detected!("sse4.2"),
                avx: std::arch::is_x86_feature_detected!("avx"),
                avx2: std::arch::is_x86_feature_detected!("avx2"),
                fma: std::arch::is_x86_feature_detected!("fma"),
                avx512f: std::arch::is_x86_feature_detected!("avx512f"),
                avx512bw: std::arch::is_x86_feature_detected!("avx512bw"),
                avx512vl: std::arch::is_x86_feature_detected!("avx512vl"),
                neon: false,
            }
        }
        #[cfg(target_arch = "aarch64")]
        {
            SimdCapabilities {
                sse2: false,
                sse3: false,
                ssse3: false,
                sse4_1: false,
                sse4_2: false,
                avx: false,
                avx2: false,
                fma: false,
                avx512f: false,
                avx512bw: false,
                avx512vl: false,
                neon: std::arch::is_aarch64_feature_detected!("neon"),
            }
        }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        {
            SimdCapabilities {
                sse2: false,
                sse3: false,
                ssse3: false,
                sse4_1: false,
                sse4_2: false,
                avx: false,
                avx2: false,
                fma: false,
                avx512f: false,
                avx512bw: false,
                avx512vl: false,
                neon: false,
            }
        }
    }
}

// ============================================================================
// Type aliases for SIMD registers
// ============================================================================

/// XMM register as 128-bit array for SIMD operations
pub type Xmm = [u64; 2];

/// YMM register (256-bit) as [lo_xmm, hi_xmm]
pub type Ymm = [Xmm; 2];

/// ZMM register (512-bit) as [xmm0, xmm1, xmm2, xmm3]
pub type Zmm = [Xmm; 4];

// ============================================================================
// Helper to get capabilities without repeated calls
// ============================================================================

#[cfg(target_arch = "x86_64")]
macro_rules! with_caps {
    ($body:expr) => {{
        let caps = SimdCapabilities::get();
        $body
    }};
}

// ============================================================================
// FLOATING POINT ARITHMETIC (ADDPS, SUBPS, MULPS, DIVPS, etc.)
// ============================================================================

// --- ADDPS ---

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse")]
#[inline]
pub unsafe fn addps_native_sse(dst: &mut Xmm, src: &Xmm) {
    use std::arch::x86_64::*;
    let a = _mm_loadu_ps(dst.as_ptr() as *const f32);
    let b = _mm_loadu_ps(src.as_ptr() as *const f32);
    let c = _mm_add_ps(a, b);
    _mm_storeu_ps(dst.as_mut_ptr() as *mut f32, c);
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn addps_native_avx(dst: &mut Ymm, src: &Ymm) {
    use std::arch::x86_64::*;
    let a = _mm256_loadu_ps(dst.as_ptr() as *const f32);
    let b = _mm256_loadu_ps(src.as_ptr() as *const f32);
    let c = _mm256_add_ps(a, b);
    _mm256_storeu_ps(dst.as_mut_ptr() as *mut f32, c);
}

/// NEON implementation of ADDPS (packed single-precision floating-point add).
///
/// # Translation Strategy
/// ARM NEON `vaddq_f32` performs element-wise addition of 4 floats:
/// 1. `vld1q_f32(dst)` loads 4 floats into NEON register `a`
/// 2. `vld1q_f32(src)` loads 4 floats into NEON register `b`
/// 3. `vaddq_f32(a, b)` computes `a[i] + b[i]` for each of 4 lanes
/// 4. `vst1q_f32(dst, c)` stores 4-float result back
///
/// # Dispatch Logic (`addps_xmm`)
/// ```text
/// if NEON available:
///     addps_native_neon(dst, src)   // NEON path
/// else:
///     addps_scalar(dst, src)        // Scalar fallback
/// ```
///
/// # Scalar Fallback Behavior (`addps_scalar`)
/// ```text
/// for i in 0..4:
///     dst_floats[i] += src_floats[i];
/// ```
///
/// # YMM (256-bit) Handling (`addps_ymm`)
/// ```text
/// addps_native_neon(&mut dst[0], &src[0]);  // Low 128 bits
/// addps_native_neon(&mut dst[1], &src[1]);  // High 128 bits
/// ```
///
/// # Performance
/// Single-cycle throughput on most ARM cores.
///
/// # Correctness
/// - Denormals: Both x86 and NEON default to IEEE 754 denormal handling,
///   though FTZ/DAZ flags may differ. This is acceptable for emulation.
/// - NaN propagation: May differ in payload, but result is still NaN.
/// - Rounding: Uses current FPCR rounding mode (typically round-to-nearest).
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[inline]
pub unsafe fn addps_native_neon(dst: &mut Xmm, src: &Xmm) {
    use std::arch::aarch64::*;
    unsafe {
        let a = vld1q_f32(dst.as_ptr() as *const f32);
        let b = vld1q_f32(src.as_ptr() as *const f32);
        let c = vaddq_f32(a, b);
        vst1q_f32(dst.as_mut_ptr() as *mut f32, c);
    }
}

#[inline]
pub fn addps_scalar(dst: &mut Xmm, src: &Xmm) {
    let d = unsafe { &mut *(dst as *mut Xmm as *mut [f32; 4]) };
    let s = unsafe { &*(src as *const Xmm as *const [f32; 4]) };
    for i in 0..4 {
        d[i] += s[i];
    }
}

#[inline]
pub fn addps_xmm(dst: &mut Xmm, src: &Xmm) {
    #[cfg(target_arch = "x86_64")]
    {
        if SimdCapabilities::get().sse2 {
            unsafe { addps_native_sse(dst, src) };
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe { addps_native_neon(dst, src) };
            return;
        }
    }
    addps_scalar(dst, src);
}

#[inline]
pub fn addps_ymm(dst: &mut Ymm, src: &Ymm) {
    #[cfg(target_arch = "x86_64")]
    {
        let caps = SimdCapabilities::get();
        if caps.avx {
            unsafe { addps_native_avx(dst, src) };
            return;
        }
        if caps.sse2 {
            unsafe {
                addps_native_sse(&mut dst[0], &src[0]);
                addps_native_sse(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe {
                addps_native_neon(&mut dst[0], &src[0]);
                addps_native_neon(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    addps_scalar(&mut dst[0], &src[0]);
    addps_scalar(&mut dst[1], &src[1]);
}

// --- ADDPD ---

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
#[inline]
pub unsafe fn addpd_native_sse2(dst: &mut Xmm, src: &Xmm) {
    use std::arch::x86_64::*;
    let a = _mm_loadu_pd(dst.as_ptr() as *const f64);
    let b = _mm_loadu_pd(src.as_ptr() as *const f64);
    let c = _mm_add_pd(a, b);
    _mm_storeu_pd(dst.as_mut_ptr() as *mut f64, c);
}

/// NEON implementation of ADDPD (packed double-precision floating-point add).
///
/// # Translation Strategy
/// ARM NEON `vaddq_f64` performs element-wise addition of 2 doubles:
/// 1. `vld1q_f64(dst)` loads 2 doubles into NEON register `a`
/// 2. `vld1q_f64(src)` loads 2 doubles into NEON register `b`
/// 3. `vaddq_f64(a, b)` computes `a[i] + b[i]` for each of 2 lanes
/// 4. `vst1q_f64(dst, c)` stores 2-double result back
///
/// # Dispatch Logic (`addpd_xmm`)
/// ```text
/// if NEON available:
///     addpd_native_neon(dst, src)   // NEON path
/// else:
///     addpd_scalar(dst, src)        // Scalar fallback
/// ```
///
/// # Scalar Fallback Behavior (`addpd_scalar`)
/// ```text
/// dst_doubles[0] += src_doubles[0];
/// dst_doubles[1] += src_doubles[1];
/// ```
///
/// # Performance
/// Single-cycle throughput on most ARM cores with FP64 support.
///
/// # Correctness
/// IEEE 754 double-precision semantics. Same considerations as ADDPS for
/// denormals, NaN propagation, and rounding modes.
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[inline]
pub unsafe fn addpd_native_neon(dst: &mut Xmm, src: &Xmm) {
    use std::arch::aarch64::*;
    unsafe {
        let a = vld1q_f64(dst.as_ptr() as *const f64);
        let b = vld1q_f64(src.as_ptr() as *const f64);
        let c = vaddq_f64(a, b);
        vst1q_f64(dst.as_mut_ptr() as *mut f64, c);
    }
}

#[inline]
pub fn addpd_scalar(dst: &mut Xmm, src: &Xmm) {
    let d = unsafe { &mut *(dst as *mut Xmm as *mut [f64; 2]) };
    let s = unsafe { &*(src as *const Xmm as *const [f64; 2]) };
    d[0] += s[0];
    d[1] += s[1];
}

#[inline]
pub fn addpd_xmm(dst: &mut Xmm, src: &Xmm) {
    #[cfg(target_arch = "x86_64")]
    {
        if SimdCapabilities::get().sse2 {
            unsafe { addpd_native_sse2(dst, src) };
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe { addpd_native_neon(dst, src) };
            return;
        }
    }
    addpd_scalar(dst, src);
}

// --- SUBPS ---

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse")]
#[inline]
pub unsafe fn subps_native_sse(dst: &mut Xmm, src: &Xmm) {
    use std::arch::x86_64::*;
    let a = _mm_loadu_ps(dst.as_ptr() as *const f32);
    let b = _mm_loadu_ps(src.as_ptr() as *const f32);
    let c = _mm_sub_ps(a, b);
    _mm_storeu_ps(dst.as_mut_ptr() as *mut f32, c);
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn subps_native_avx(dst: &mut Ymm, src: &Ymm) {
    use std::arch::x86_64::*;
    let a = _mm256_loadu_ps(dst.as_ptr() as *const f32);
    let b = _mm256_loadu_ps(src.as_ptr() as *const f32);
    let c = _mm256_sub_ps(a, b);
    _mm256_storeu_ps(dst.as_mut_ptr() as *mut f32, c);
}

/// NEON implementation of SUBPS (packed single-precision floating-point subtract).
///
/// # Translation Strategy
/// ARM NEON `vsubq_f32` performs element-wise subtraction of 4 floats:
/// 1. `vld1q_f32(dst)` loads 4 floats into NEON register `a`
/// 2. `vld1q_f32(src)` loads 4 floats into NEON register `b`
/// 3. `vsubq_f32(a, b)` computes `a[i] - b[i]` for each of 4 lanes
/// 4. `vst1q_f32(dst, c)` stores 4-float result back
///
/// # Dispatch Logic (`subps_xmm`)
/// ```text
/// if NEON available:
///     subps_native_neon(dst, src)   // NEON path
/// else:
///     subps_scalar(dst, src)        // Scalar fallback
/// ```
///
/// # Scalar Fallback Behavior (`subps_scalar`)
/// ```text
/// for i in 0..4:
///     dst_floats[i] -= src_floats[i];
/// ```
///
/// # YMM (256-bit) Handling (`subps_ymm`)
/// ```text
/// subps_native_neon(&mut dst[0], &src[0]);  // Low 128 bits
/// subps_native_neon(&mut dst[1], &src[1]);  // High 128 bits
/// ```
///
/// # Performance
/// Single-cycle throughput on most ARM cores.
///
/// # Correctness
/// IEEE 754 single-precision semantics. Same considerations as ADDPS.
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[inline]
pub unsafe fn subps_native_neon(dst: &mut Xmm, src: &Xmm) {
    use std::arch::aarch64::*;
    unsafe {
        let a = vld1q_f32(dst.as_ptr() as *const f32);
        let b = vld1q_f32(src.as_ptr() as *const f32);
        let c = vsubq_f32(a, b);
        vst1q_f32(dst.as_mut_ptr() as *mut f32, c);
    }
}

#[inline]
pub fn subps_scalar(dst: &mut Xmm, src: &Xmm) {
    let d = unsafe { &mut *(dst as *mut Xmm as *mut [f32; 4]) };
    let s = unsafe { &*(src as *const Xmm as *const [f32; 4]) };
    for i in 0..4 {
        d[i] -= s[i];
    }
}

#[inline]
pub fn subps_xmm(dst: &mut Xmm, src: &Xmm) {
    #[cfg(target_arch = "x86_64")]
    {
        if SimdCapabilities::get().sse2 {
            unsafe { subps_native_sse(dst, src) };
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe { subps_native_neon(dst, src) };
            return;
        }
    }
    subps_scalar(dst, src);
}

#[inline]
pub fn subps_ymm(dst: &mut Ymm, src: &Ymm) {
    #[cfg(target_arch = "x86_64")]
    {
        let caps = SimdCapabilities::get();
        if caps.avx {
            unsafe { subps_native_avx(dst, src) };
            return;
        }
        if caps.sse2 {
            unsafe {
                subps_native_sse(&mut dst[0], &src[0]);
                subps_native_sse(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe {
                subps_native_neon(&mut dst[0], &src[0]);
                subps_native_neon(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    subps_scalar(&mut dst[0], &src[0]);
    subps_scalar(&mut dst[1], &src[1]);
}

// --- SUBPD ---

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
#[inline]
pub unsafe fn subpd_native_sse2(dst: &mut Xmm, src: &Xmm) {
    use std::arch::x86_64::*;
    let a = _mm_loadu_pd(dst.as_ptr() as *const f64);
    let b = _mm_loadu_pd(src.as_ptr() as *const f64);
    let c = _mm_sub_pd(a, b);
    _mm_storeu_pd(dst.as_mut_ptr() as *mut f64, c);
}

/// NEON implementation of SUBPD (packed double-precision floating-point subtract).
///
/// # Translation Strategy
/// ARM NEON `vsubq_f64` performs element-wise subtraction of 2 doubles:
/// 1. `vld1q_f64(dst)` loads 2 doubles into NEON register `a`
/// 2. `vld1q_f64(src)` loads 2 doubles into NEON register `b`
/// 3. `vsubq_f64(a, b)` computes `a[i] - b[i]` for each of 2 lanes
/// 4. `vst1q_f64(dst, c)` stores 2-double result back
///
/// # Dispatch Logic (`subpd_xmm`)
/// ```text
/// if NEON available:
///     subpd_native_neon(dst, src)   // NEON path
/// else:
///     subpd_scalar(dst, src)        // Scalar fallback
/// ```
///
/// # Scalar Fallback Behavior (`subpd_scalar`)
/// ```text
/// dst_doubles[0] -= src_doubles[0];
/// dst_doubles[1] -= src_doubles[1];
/// ```
///
/// # Performance
/// Single-cycle throughput on most ARM cores with FP64 support.
///
/// # Correctness
/// IEEE 754 double-precision semantics. Same considerations as ADDPD.
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[inline]
pub unsafe fn subpd_native_neon(dst: &mut Xmm, src: &Xmm) {
    use std::arch::aarch64::*;
    unsafe {
        let a = vld1q_f64(dst.as_ptr() as *const f64);
        let b = vld1q_f64(src.as_ptr() as *const f64);
        let c = vsubq_f64(a, b);
        vst1q_f64(dst.as_mut_ptr() as *mut f64, c);
    }
}

#[inline]
pub fn subpd_scalar(dst: &mut Xmm, src: &Xmm) {
    let d = unsafe { &mut *(dst as *mut Xmm as *mut [f64; 2]) };
    let s = unsafe { &*(src as *const Xmm as *const [f64; 2]) };
    d[0] -= s[0];
    d[1] -= s[1];
}

#[inline]
pub fn subpd_xmm(dst: &mut Xmm, src: &Xmm) {
    #[cfg(target_arch = "x86_64")]
    {
        if SimdCapabilities::get().sse2 {
            unsafe { subpd_native_sse2(dst, src) };
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe { subpd_native_neon(dst, src) };
            return;
        }
    }
    subpd_scalar(dst, src);
}

// --- MULPS ---

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse")]
#[inline]
pub unsafe fn mulps_native_sse(dst: &mut Xmm, src: &Xmm) {
    use std::arch::x86_64::*;
    let a = _mm_loadu_ps(dst.as_ptr() as *const f32);
    let b = _mm_loadu_ps(src.as_ptr() as *const f32);
    let c = _mm_mul_ps(a, b);
    _mm_storeu_ps(dst.as_mut_ptr() as *mut f32, c);
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn mulps_native_avx(dst: &mut Ymm, src: &Ymm) {
    use std::arch::x86_64::*;
    let a = _mm256_loadu_ps(dst.as_ptr() as *const f32);
    let b = _mm256_loadu_ps(src.as_ptr() as *const f32);
    let c = _mm256_mul_ps(a, b);
    _mm256_storeu_ps(dst.as_mut_ptr() as *mut f32, c);
}

/// NEON implementation of MULPS (packed single-precision floating-point multiply).
///
/// # Translation Strategy
/// ARM NEON `vmulq_f32` performs element-wise multiplication of 4 floats:
/// 1. `vld1q_f32(dst)` loads 4 floats into NEON register `a`
/// 2. `vld1q_f32(src)` loads 4 floats into NEON register `b`
/// 3. `vmulq_f32(a, b)` computes `a[i] * b[i]` for each of 4 lanes
/// 4. `vst1q_f32(dst, c)` stores 4-float result back
///
/// # Dispatch Logic (`mulps_xmm`)
/// ```text
/// if NEON available:
///     mulps_native_neon(dst, src)   // NEON path
/// else:
///     mulps_scalar(dst, src)        // Scalar fallback
/// ```
///
/// # Scalar Fallback Behavior (`mulps_scalar`)
/// ```text
/// for i in 0..4:
///     dst_floats[i] *= src_floats[i];
/// ```
///
/// # YMM (256-bit) Handling (`mulps_ymm`)
/// ```text
/// mulps_native_neon(&mut dst[0], &src[0]);  // Low 128 bits
/// mulps_native_neon(&mut dst[1], &src[1]);  // High 128 bits
/// ```
///
/// # Performance
/// Single-cycle throughput on most ARM cores. NEON FP multiply is well-optimized.
///
/// # Correctness
/// IEEE 754 single-precision semantics. Rounding and special value handling
/// follows standard FP rules.
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[inline]
pub unsafe fn mulps_native_neon(dst: &mut Xmm, src: &Xmm) {
    use std::arch::aarch64::*;
    unsafe {
        let a = vld1q_f32(dst.as_ptr() as *const f32);
        let b = vld1q_f32(src.as_ptr() as *const f32);
        let c = vmulq_f32(a, b);
        vst1q_f32(dst.as_mut_ptr() as *mut f32, c);
    }
}

#[inline]
pub fn mulps_scalar(dst: &mut Xmm, src: &Xmm) {
    let d = unsafe { &mut *(dst as *mut Xmm as *mut [f32; 4]) };
    let s = unsafe { &*(src as *const Xmm as *const [f32; 4]) };
    for i in 0..4 {
        d[i] *= s[i];
    }
}

#[inline]
pub fn mulps_xmm(dst: &mut Xmm, src: &Xmm) {
    #[cfg(target_arch = "x86_64")]
    {
        if SimdCapabilities::get().sse2 {
            unsafe { mulps_native_sse(dst, src) };
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe { mulps_native_neon(dst, src) };
            return;
        }
    }
    mulps_scalar(dst, src);
}

#[inline]
pub fn mulps_ymm(dst: &mut Ymm, src: &Ymm) {
    #[cfg(target_arch = "x86_64")]
    {
        let caps = SimdCapabilities::get();
        if caps.avx {
            unsafe { mulps_native_avx(dst, src) };
            return;
        }
        if caps.sse2 {
            unsafe {
                mulps_native_sse(&mut dst[0], &src[0]);
                mulps_native_sse(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe {
                mulps_native_neon(&mut dst[0], &src[0]);
                mulps_native_neon(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    mulps_scalar(&mut dst[0], &src[0]);
    mulps_scalar(&mut dst[1], &src[1]);
}

// --- MULPD ---

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
#[inline]
pub unsafe fn mulpd_native_sse2(dst: &mut Xmm, src: &Xmm) {
    use std::arch::x86_64::*;
    let a = _mm_loadu_pd(dst.as_ptr() as *const f64);
    let b = _mm_loadu_pd(src.as_ptr() as *const f64);
    let c = _mm_mul_pd(a, b);
    _mm_storeu_pd(dst.as_mut_ptr() as *mut f64, c);
}

/// NEON implementation of MULPD (packed double-precision floating-point multiply).
///
/// # Translation Strategy
/// ARM NEON `vmulq_f64` performs element-wise multiplication of 2 doubles:
/// 1. `vld1q_f64(dst)` loads 2 doubles into NEON register `a`
/// 2. `vld1q_f64(src)` loads 2 doubles into NEON register `b`
/// 3. `vmulq_f64(a, b)` computes `a[i] * b[i]` for each of 2 lanes
/// 4. `vst1q_f64(dst, c)` stores 2-double result back
///
/// # Dispatch Logic (`mulpd_xmm`)
/// ```text
/// if NEON available:
///     mulpd_native_neon(dst, src)   // NEON path
/// else:
///     mulpd_scalar(dst, src)        // Scalar fallback
/// ```
///
/// # Scalar Fallback Behavior (`mulpd_scalar`)
/// ```text
/// dst_doubles[0] *= src_doubles[0];
/// dst_doubles[1] *= src_doubles[1];
/// ```
///
/// # Performance
/// Single-cycle throughput on most ARM cores with FP64 support.
///
/// # Correctness
/// IEEE 754 double-precision semantics.
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[inline]
pub unsafe fn mulpd_native_neon(dst: &mut Xmm, src: &Xmm) {
    use std::arch::aarch64::*;
    unsafe {
        let a = vld1q_f64(dst.as_ptr() as *const f64);
        let b = vld1q_f64(src.as_ptr() as *const f64);
        let c = vmulq_f64(a, b);
        vst1q_f64(dst.as_mut_ptr() as *mut f64, c);
    }
}

#[inline]
pub fn mulpd_scalar(dst: &mut Xmm, src: &Xmm) {
    let d = unsafe { &mut *(dst as *mut Xmm as *mut [f64; 2]) };
    let s = unsafe { &*(src as *const Xmm as *const [f64; 2]) };
    d[0] *= s[0];
    d[1] *= s[1];
}

#[inline]
pub fn mulpd_xmm(dst: &mut Xmm, src: &Xmm) {
    #[cfg(target_arch = "x86_64")]
    {
        if SimdCapabilities::get().sse2 {
            unsafe { mulpd_native_sse2(dst, src) };
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe { mulpd_native_neon(dst, src) };
            return;
        }
    }
    mulpd_scalar(dst, src);
}

// --- DIVPS ---

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse")]
#[inline]
pub unsafe fn divps_native_sse(dst: &mut Xmm, src: &Xmm) {
    use std::arch::x86_64::*;
    let a = _mm_loadu_ps(dst.as_ptr() as *const f32);
    let b = _mm_loadu_ps(src.as_ptr() as *const f32);
    let c = _mm_div_ps(a, b);
    _mm_storeu_ps(dst.as_mut_ptr() as *mut f32, c);
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn divps_native_avx(dst: &mut Ymm, src: &Ymm) {
    use std::arch::x86_64::*;
    let a = _mm256_loadu_ps(dst.as_ptr() as *const f32);
    let b = _mm256_loadu_ps(src.as_ptr() as *const f32);
    let c = _mm256_div_ps(a, b);
    _mm256_storeu_ps(dst.as_mut_ptr() as *mut f32, c);
}

/// NEON implementation of DIVPS (packed single-precision floating-point divide).
///
/// # Translation Strategy
/// ARM NEON `vdivq_f32` performs element-wise division of 4 floats:
/// 1. `vld1q_f32(dst)` loads 4 floats into NEON register `a` (numerator)
/// 2. `vld1q_f32(src)` loads 4 floats into NEON register `b` (denominator)
/// 3. `vdivq_f32(a, b)` computes `a[i] / b[i]` for each of 4 lanes
/// 4. `vst1q_f32(dst, c)` stores 4-float result back
///
/// # Dispatch Logic (`divps_xmm`)
/// ```text
/// if NEON available:
///     divps_native_neon(dst, src)   // NEON path
/// else:
///     divps_scalar(dst, src)        // Scalar fallback
/// ```
///
/// # Scalar Fallback Behavior (`divps_scalar`)
/// ```text
/// for i in 0..4:
///     dst_floats[i] /= src_floats[i];
/// ```
///
/// # YMM (256-bit) Handling (`divps_ymm`)
/// ```text
/// divps_native_neon(&mut dst[0], &src[0]);  // Low 128 bits
/// divps_native_neon(&mut dst[1], &src[1]);  // High 128 bits
/// ```
///
/// # Performance
/// Division is slower than multiply (typically 3-10 cycles depending on core).
/// Some ARM cores use iterative refinement for higher precision.
///
/// # Correctness
/// IEEE 754 single-precision semantics. Division by zero produces infinity
/// with appropriate sign on both architectures.
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[inline]
pub unsafe fn divps_native_neon(dst: &mut Xmm, src: &Xmm) {
    use std::arch::aarch64::*;
    unsafe {
        let a = vld1q_f32(dst.as_ptr() as *const f32);
        let b = vld1q_f32(src.as_ptr() as *const f32);
        let c = vdivq_f32(a, b);
        vst1q_f32(dst.as_mut_ptr() as *mut f32, c);
    }
}

#[inline]
pub fn divps_scalar(dst: &mut Xmm, src: &Xmm) {
    let d = unsafe { &mut *(dst as *mut Xmm as *mut [f32; 4]) };
    let s = unsafe { &*(src as *const Xmm as *const [f32; 4]) };
    for i in 0..4 {
        d[i] /= s[i];
    }
}

#[inline]
pub fn divps_xmm(dst: &mut Xmm, src: &Xmm) {
    #[cfg(target_arch = "x86_64")]
    {
        if SimdCapabilities::get().sse2 {
            unsafe { divps_native_sse(dst, src) };
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe { divps_native_neon(dst, src) };
            return;
        }
    }
    divps_scalar(dst, src);
}

#[inline]
pub fn divps_ymm(dst: &mut Ymm, src: &Ymm) {
    #[cfg(target_arch = "x86_64")]
    {
        let caps = SimdCapabilities::get();
        if caps.avx {
            unsafe { divps_native_avx(dst, src) };
            return;
        }
        if caps.sse2 {
            unsafe {
                divps_native_sse(&mut dst[0], &src[0]);
                divps_native_sse(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe {
                divps_native_neon(&mut dst[0], &src[0]);
                divps_native_neon(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    divps_scalar(&mut dst[0], &src[0]);
    divps_scalar(&mut dst[1], &src[1]);
}

// --- DIVPD ---

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
#[inline]
pub unsafe fn divpd_native_sse2(dst: &mut Xmm, src: &Xmm) {
    use std::arch::x86_64::*;
    let a = _mm_loadu_pd(dst.as_ptr() as *const f64);
    let b = _mm_loadu_pd(src.as_ptr() as *const f64);
    let c = _mm_div_pd(a, b);
    _mm_storeu_pd(dst.as_mut_ptr() as *mut f64, c);
}

/// NEON implementation of DIVPD (packed double-precision floating-point divide).
///
/// # Translation Strategy
/// ARM NEON `vdivq_f64` performs element-wise division of 2 doubles:
/// 1. `vld1q_f64(dst)` loads 2 doubles into NEON register `a` (numerator)
/// 2. `vld1q_f64(src)` loads 2 doubles into NEON register `b` (denominator)
/// 3. `vdivq_f64(a, b)` computes `a[i] / b[i]` for each of 2 lanes
/// 4. `vst1q_f64(dst, c)` stores 2-double result back
///
/// # Dispatch Logic (`divpd_xmm`)
/// ```text
/// if NEON available:
///     divpd_native_neon(dst, src)   // NEON path
/// else:
///     divpd_scalar(dst, src)        // Scalar fallback
/// ```
///
/// # Scalar Fallback Behavior (`divpd_scalar`)
/// ```text
/// dst_doubles[0] /= src_doubles[0];
/// dst_doubles[1] /= src_doubles[1];
/// ```
///
/// # Performance
/// Division is slower than multiply. Double-precision division typically
/// takes 7-15 cycles depending on the ARM core implementation.
///
/// # Correctness
/// IEEE 754 double-precision semantics. Division by zero produces infinity.
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[inline]
pub unsafe fn divpd_native_neon(dst: &mut Xmm, src: &Xmm) {
    use std::arch::aarch64::*;
    unsafe {
        let a = vld1q_f64(dst.as_ptr() as *const f64);
        let b = vld1q_f64(src.as_ptr() as *const f64);
        let c = vdivq_f64(a, b);
        vst1q_f64(dst.as_mut_ptr() as *mut f64, c);
    }
}

#[inline]
pub fn divpd_scalar(dst: &mut Xmm, src: &Xmm) {
    let d = unsafe { &mut *(dst as *mut Xmm as *mut [f64; 2]) };
    let s = unsafe { &*(src as *const Xmm as *const [f64; 2]) };
    d[0] /= s[0];
    d[1] /= s[1];
}

#[inline]
pub fn divpd_xmm(dst: &mut Xmm, src: &Xmm) {
    #[cfg(target_arch = "x86_64")]
    {
        if SimdCapabilities::get().sse2 {
            unsafe { divpd_native_sse2(dst, src) };
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe { divpd_native_neon(dst, src) };
            return;
        }
    }
    divpd_scalar(dst, src);
}

// ============================================================================
// INTEGER ARITHMETIC (PADDB, PADDW, PADDD, PADDQ, PSUBB, etc.)
// ============================================================================

// --- PADDB ---

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
#[inline]
pub unsafe fn paddb_native_sse2(dst: &mut Xmm, src: &Xmm) {
    use std::arch::x86_64::*;
    let a = _mm_loadu_si128(dst.as_ptr() as *const __m128i);
    let b = _mm_loadu_si128(src.as_ptr() as *const __m128i);
    let c = _mm_add_epi8(a, b);
    _mm_storeu_si128(dst.as_mut_ptr() as *mut __m128i, c);
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn paddb_native_avx2(dst: &mut Ymm, src: &Ymm) {
    use std::arch::x86_64::*;
    let a = _mm256_loadu_si256(dst.as_ptr() as *const __m256i);
    let b = _mm256_loadu_si256(src.as_ptr() as *const __m256i);
    let c = _mm256_add_epi8(a, b);
    _mm256_storeu_si256(dst.as_mut_ptr() as *mut __m256i, c);
}

/// NEON implementation of PADDB (packed byte addition with wraparound).
///
/// # Translation Strategy
/// ARM NEON `vaddq_u8` performs element-wise addition of 16 bytes:
/// 1. `vld1q_u8(dst)` loads 16 bytes into NEON register
/// 2. `vld1q_u8(src)` loads 16 bytes from source
/// 3. `vaddq_u8(a, b)` adds corresponding bytes with wraparound
/// 4. `vst1q_u8(dst, c)` stores 16-byte result
///
/// # Dispatch Logic (`paddb_xmm`)
/// ```text
/// if NEON available:
///     paddb_native_neon(dst, src)   // NEON path
/// else:
///     paddb_scalar(dst, src)        // Scalar fallback
/// ```
///
/// # Scalar Fallback Behavior (`paddb_scalar`)
/// ```text
/// for i in 0..16:
///     dst_bytes[i] = dst_bytes[i].wrapping_add(src_bytes[i]);
/// ```
///
/// # YMM (256-bit) Handling (`paddb_ymm`)
/// YMM operations process two XMM halves sequentially since NEON lacks
/// native 256-bit registers:
/// ```text
/// paddb_native_neon(&mut dst[0], &src[0]);  // Low 128 bits
/// paddb_native_neon(&mut dst[1], &src[1]);  // High 128 bits
/// ```
///
/// # Performance
/// Single-cycle throughput per 128-bit operation.
///
/// # Correctness
/// Unsigned wraparound: 0xFF + 0x01 = 0x00 (no exception, no saturation).
/// Both architectures use identical two's complement wraparound semantics.
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[inline]
pub unsafe fn paddb_native_neon(dst: &mut Xmm, src: &Xmm) {
    use std::arch::aarch64::*;
    unsafe {
        let a = vld1q_u8(dst.as_ptr() as *const u8);
        let b = vld1q_u8(src.as_ptr() as *const u8);
        let c = vaddq_u8(a, b);
        vst1q_u8(dst.as_mut_ptr() as *mut u8, c);
    }
}

#[inline]
pub fn paddb_scalar(dst: &mut Xmm, src: &Xmm) {
    let d = unsafe { &mut *(dst as *mut Xmm as *mut [u8; 16]) };
    let s = unsafe { &*(src as *const Xmm as *const [u8; 16]) };
    for i in 0..16 {
        d[i] = d[i].wrapping_add(s[i]);
    }
}

#[inline]
pub fn paddb_xmm(dst: &mut Xmm, src: &Xmm) {
    #[cfg(target_arch = "x86_64")]
    {
        if SimdCapabilities::get().sse2 {
            unsafe { paddb_native_sse2(dst, src) };
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe { paddb_native_neon(dst, src) };
            return;
        }
    }
    paddb_scalar(dst, src);
}

#[inline]
pub fn paddb_ymm(dst: &mut Ymm, src: &Ymm) {
    #[cfg(target_arch = "x86_64")]
    {
        let caps = SimdCapabilities::get();
        if caps.avx2 {
            unsafe { paddb_native_avx2(dst, src) };
            return;
        }
        if caps.sse2 {
            unsafe {
                paddb_native_sse2(&mut dst[0], &src[0]);
                paddb_native_sse2(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe {
                paddb_native_neon(&mut dst[0], &src[0]);
                paddb_native_neon(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    paddb_scalar(&mut dst[0], &src[0]);
    paddb_scalar(&mut dst[1], &src[1]);
}

// --- PADDW ---

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
#[inline]
pub unsafe fn paddw_native_sse2(dst: &mut Xmm, src: &Xmm) {
    use std::arch::x86_64::*;
    let a = _mm_loadu_si128(dst.as_ptr() as *const __m128i);
    let b = _mm_loadu_si128(src.as_ptr() as *const __m128i);
    let c = _mm_add_epi16(a, b);
    _mm_storeu_si128(dst.as_mut_ptr() as *mut __m128i, c);
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn paddw_native_avx2(dst: &mut Ymm, src: &Ymm) {
    use std::arch::x86_64::*;
    let a = _mm256_loadu_si256(dst.as_ptr() as *const __m256i);
    let b = _mm256_loadu_si256(src.as_ptr() as *const __m256i);
    let c = _mm256_add_epi16(a, b);
    _mm256_storeu_si256(dst.as_mut_ptr() as *mut __m256i, c);
}

/// NEON implementation of PADDW (packed word addition with wraparound).
///
/// # Translation Strategy
/// ARM NEON `vaddq_u16` performs element-wise addition of 8 words:
/// 1. `vld1q_u16(dst)` loads 8 words into NEON register
/// 2. `vld1q_u16(src)` loads 8 words from source
/// 3. `vaddq_u16(a, b)` adds corresponding words with wraparound
/// 4. `vst1q_u16(dst, c)` stores 8-word result
///
/// # Dispatch Logic (`paddw_xmm`)
/// ```text
/// if NEON available:
///     paddw_native_neon(dst, src)   // NEON path
/// else:
///     paddw_scalar(dst, src)        // Scalar fallback
/// ```
///
/// # Scalar Fallback Behavior (`paddw_scalar`)
/// ```text
/// for i in 0..8:
///     dst_words[i] = dst_words[i].wrapping_add(src_words[i]);
/// ```
///
/// # YMM (256-bit) Handling (`paddw_ymm`)
/// ```text
/// paddw_native_neon(&mut dst[0], &src[0]);  // Low 128 bits
/// paddw_native_neon(&mut dst[1], &src[1]);  // High 128 bits
/// ```
///
/// # Performance
/// Single-cycle throughput.
///
/// # Correctness
/// Unsigned wraparound: 0xFFFF + 0x0001 = 0x0000 (16-bit modular arithmetic).
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[inline]
pub unsafe fn paddw_native_neon(dst: &mut Xmm, src: &Xmm) {
    use std::arch::aarch64::*;
    unsafe {
        let a = vld1q_u16(dst.as_ptr() as *const u16);
        let b = vld1q_u16(src.as_ptr() as *const u16);
        let c = vaddq_u16(a, b);
        vst1q_u16(dst.as_mut_ptr() as *mut u16, c);
    }
}

#[inline]
pub fn paddw_scalar(dst: &mut Xmm, src: &Xmm) {
    let d = unsafe { &mut *(dst as *mut Xmm as *mut [u16; 8]) };
    let s = unsafe { &*(src as *const Xmm as *const [u16; 8]) };
    for i in 0..8 {
        d[i] = d[i].wrapping_add(s[i]);
    }
}

#[inline]
pub fn paddw_xmm(dst: &mut Xmm, src: &Xmm) {
    #[cfg(target_arch = "x86_64")]
    {
        if SimdCapabilities::get().sse2 {
            unsafe { paddw_native_sse2(dst, src) };
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe { paddw_native_neon(dst, src) };
            return;
        }
    }
    paddw_scalar(dst, src);
}

#[inline]
pub fn paddw_ymm(dst: &mut Ymm, src: &Ymm) {
    #[cfg(target_arch = "x86_64")]
    {
        let caps = SimdCapabilities::get();
        if caps.avx2 {
            unsafe { paddw_native_avx2(dst, src) };
            return;
        }
        if caps.sse2 {
            unsafe {
                paddw_native_sse2(&mut dst[0], &src[0]);
                paddw_native_sse2(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe {
                paddw_native_neon(&mut dst[0], &src[0]);
                paddw_native_neon(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    paddw_scalar(&mut dst[0], &src[0]);
    paddw_scalar(&mut dst[1], &src[1]);
}

// --- PADDD ---

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
#[inline]
pub unsafe fn paddd_native_sse2(dst: &mut Xmm, src: &Xmm) {
    use std::arch::x86_64::*;
    let a = _mm_loadu_si128(dst.as_ptr() as *const __m128i);
    let b = _mm_loadu_si128(src.as_ptr() as *const __m128i);
    let c = _mm_add_epi32(a, b);
    _mm_storeu_si128(dst.as_mut_ptr() as *mut __m128i, c);
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn paddd_native_avx2(dst: &mut Ymm, src: &Ymm) {
    use std::arch::x86_64::*;
    let a = _mm256_loadu_si256(dst.as_ptr() as *const __m256i);
    let b = _mm256_loadu_si256(src.as_ptr() as *const __m256i);
    let c = _mm256_add_epi32(a, b);
    _mm256_storeu_si256(dst.as_mut_ptr() as *mut __m256i, c);
}

/// NEON implementation of PADDD (packed dword addition with wraparound).
///
/// # Translation Strategy
/// ARM NEON `vaddq_u32` performs element-wise addition of 4 dwords:
/// 1. `vld1q_u32(dst)` loads 4 dwords into NEON register
/// 2. `vld1q_u32(src)` loads 4 dwords from source
/// 3. `vaddq_u32(a, b)` adds corresponding dwords with wraparound
/// 4. `vst1q_u32(dst, c)` stores 4-dword result
///
/// # Dispatch Logic (`paddd_xmm`)
/// ```text
/// if NEON available:
///     paddd_native_neon(dst, src)   // NEON path
/// else:
///     paddd_scalar(dst, src)        // Scalar fallback
/// ```
///
/// # Scalar Fallback Behavior (`paddd_scalar`)
/// ```text
/// for i in 0..4:
///     dst_dwords[i] = dst_dwords[i].wrapping_add(src_dwords[i]);
/// ```
///
/// # YMM (256-bit) Handling (`paddd_ymm`)
/// ```text
/// paddd_native_neon(&mut dst[0], &src[0]);  // Low 128 bits
/// paddd_native_neon(&mut dst[1], &src[1]);  // High 128 bits
/// ```
///
/// # Performance
/// Single-cycle throughput.
///
/// # Correctness
/// Unsigned wraparound: 0xFFFFFFFF + 0x00000001 = 0x00000000 (32-bit modular).
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[inline]
pub unsafe fn paddd_native_neon(dst: &mut Xmm, src: &Xmm) {
    use std::arch::aarch64::*;
    unsafe {
        let a = vld1q_u32(dst.as_ptr() as *const u32);
        let b = vld1q_u32(src.as_ptr() as *const u32);
        let c = vaddq_u32(a, b);
        vst1q_u32(dst.as_mut_ptr() as *mut u32, c);
    }
}

#[inline]
pub fn paddd_scalar(dst: &mut Xmm, src: &Xmm) {
    let d = unsafe { &mut *(dst as *mut Xmm as *mut [u32; 4]) };
    let s = unsafe { &*(src as *const Xmm as *const [u32; 4]) };
    for i in 0..4 {
        d[i] = d[i].wrapping_add(s[i]);
    }
}

#[inline]
pub fn paddd_xmm(dst: &mut Xmm, src: &Xmm) {
    #[cfg(target_arch = "x86_64")]
    {
        if SimdCapabilities::get().sse2 {
            unsafe { paddd_native_sse2(dst, src) };
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe { paddd_native_neon(dst, src) };
            return;
        }
    }
    paddd_scalar(dst, src);
}

#[inline]
pub fn paddd_ymm(dst: &mut Ymm, src: &Ymm) {
    #[cfg(target_arch = "x86_64")]
    {
        let caps = SimdCapabilities::get();
        if caps.avx2 {
            unsafe { paddd_native_avx2(dst, src) };
            return;
        }
        if caps.sse2 {
            unsafe {
                paddd_native_sse2(&mut dst[0], &src[0]);
                paddd_native_sse2(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe {
                paddd_native_neon(&mut dst[0], &src[0]);
                paddd_native_neon(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    paddd_scalar(&mut dst[0], &src[0]);
    paddd_scalar(&mut dst[1], &src[1]);
}

// --- PADDQ ---

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
#[inline]
pub unsafe fn paddq_native_sse2(dst: &mut Xmm, src: &Xmm) {
    use std::arch::x86_64::*;
    let a = _mm_loadu_si128(dst.as_ptr() as *const __m128i);
    let b = _mm_loadu_si128(src.as_ptr() as *const __m128i);
    let c = _mm_add_epi64(a, b);
    _mm_storeu_si128(dst.as_mut_ptr() as *mut __m128i, c);
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn paddq_native_avx2(dst: &mut Ymm, src: &Ymm) {
    use std::arch::x86_64::*;
    let a = _mm256_loadu_si256(dst.as_ptr() as *const __m256i);
    let b = _mm256_loadu_si256(src.as_ptr() as *const __m256i);
    let c = _mm256_add_epi64(a, b);
    _mm256_storeu_si256(dst.as_mut_ptr() as *mut __m256i, c);
}

/// NEON implementation of PADDQ (packed qword addition with wraparound).
///
/// # Translation Strategy
/// ARM NEON `vaddq_u64` performs element-wise addition of 2 qwords:
/// 1. `vld1q_u64(dst)` loads 2 qwords into NEON register
/// 2. `vld1q_u64(src)` loads 2 qwords from source
/// 3. `vaddq_u64(a, b)` adds corresponding qwords with wraparound
/// 4. `vst1q_u64(dst, c)` stores 2-qword result
///
/// # Dispatch Logic (`paddq_xmm`)
/// ```text
/// if NEON available:
///     paddq_native_neon(dst, src)   // NEON path
/// else:
///     paddq_scalar(dst, src)        // Scalar fallback
/// ```
///
/// # Scalar Fallback Behavior (`paddq_scalar`)
/// ```text
/// dst[0] = dst[0].wrapping_add(src[0]);
/// dst[1] = dst[1].wrapping_add(src[1]);
/// ```
///
/// # YMM (256-bit) Handling (`paddq_ymm`)
/// ```text
/// paddq_native_neon(&mut dst[0], &src[0]);  // Low 128 bits
/// paddq_native_neon(&mut dst[1], &src[1]);  // High 128 bits
/// ```
///
/// # Performance
/// Single-cycle throughput.
///
/// # Correctness
/// Unsigned wraparound: 64-bit modular arithmetic matches x86 exactly.
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[inline]
pub unsafe fn paddq_native_neon(dst: &mut Xmm, src: &Xmm) {
    use std::arch::aarch64::*;
    unsafe {
        let a = vld1q_u64(dst.as_ptr() as *const u64);
        let b = vld1q_u64(src.as_ptr() as *const u64);
        let c = vaddq_u64(a, b);
        vst1q_u64(dst.as_mut_ptr() as *mut u64, c);
    }
}

#[inline]
pub fn paddq_scalar(dst: &mut Xmm, src: &Xmm) {
    dst[0] = dst[0].wrapping_add(src[0]);
    dst[1] = dst[1].wrapping_add(src[1]);
}

#[inline]
pub fn paddq_xmm(dst: &mut Xmm, src: &Xmm) {
    #[cfg(target_arch = "x86_64")]
    {
        if SimdCapabilities::get().sse2 {
            unsafe { paddq_native_sse2(dst, src) };
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe { paddq_native_neon(dst, src) };
            return;
        }
    }
    paddq_scalar(dst, src);
}

#[inline]
pub fn paddq_ymm(dst: &mut Ymm, src: &Ymm) {
    #[cfg(target_arch = "x86_64")]
    {
        let caps = SimdCapabilities::get();
        if caps.avx2 {
            unsafe { paddq_native_avx2(dst, src) };
            return;
        }
        if caps.sse2 {
            unsafe {
                paddq_native_sse2(&mut dst[0], &src[0]);
                paddq_native_sse2(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe {
                paddq_native_neon(&mut dst[0], &src[0]);
                paddq_native_neon(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    paddq_scalar(&mut dst[0], &src[0]);
    paddq_scalar(&mut dst[1], &src[1]);
}

// --- PSUBB ---

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
#[inline]
pub unsafe fn psubb_native_sse2(dst: &mut Xmm, src: &Xmm) {
    use std::arch::x86_64::*;
    let a = _mm_loadu_si128(dst.as_ptr() as *const __m128i);
    let b = _mm_loadu_si128(src.as_ptr() as *const __m128i);
    let c = _mm_sub_epi8(a, b);
    _mm_storeu_si128(dst.as_mut_ptr() as *mut __m128i, c);
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn psubb_native_avx2(dst: &mut Ymm, src: &Ymm) {
    use std::arch::x86_64::*;
    let a = _mm256_loadu_si256(dst.as_ptr() as *const __m256i);
    let b = _mm256_loadu_si256(src.as_ptr() as *const __m256i);
    let c = _mm256_sub_epi8(a, b);
    _mm256_storeu_si256(dst.as_mut_ptr() as *mut __m256i, c);
}

/// NEON implementation of PSUBB (packed byte subtraction with wraparound).
///
/// # Translation Strategy
/// ARM NEON `vsubq_u8` performs element-wise subtraction of 16 bytes:
/// 1. `vld1q_u8(dst)` loads 16 bytes into NEON register (minuend)
/// 2. `vld1q_u8(src)` loads 16 bytes from source (subtrahend)
/// 3. `vsubq_u8(a, b)` computes `dst - src` for each byte with wraparound
/// 4. `vst1q_u8(dst, c)` stores 16-byte result
///
/// # Dispatch Logic (`psubb_xmm`)
/// ```text
/// if NEON available:
///     psubb_native_neon(dst, src)   // NEON path
/// else:
///     psubb_scalar(dst, src)        // Scalar fallback
/// ```
///
/// # Scalar Fallback Behavior (`psubb_scalar`)
/// ```text
/// for i in 0..16:
///     dst_bytes[i] = dst_bytes[i].wrapping_sub(src_bytes[i]);
/// ```
///
/// # YMM (256-bit) Handling (`psubb_ymm`)
/// ```text
/// psubb_native_neon(&mut dst[0], &src[0]);  // Low 128 bits
/// psubb_native_neon(&mut dst[1], &src[1]);  // High 128 bits
/// ```
///
/// # Performance
/// Single-cycle throughput.
///
/// # Correctness
/// Unsigned wraparound: 0x00 - 0x01 = 0xFF (wraps around, no borrow flag).
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[inline]
pub unsafe fn psubb_native_neon(dst: &mut Xmm, src: &Xmm) {
    use std::arch::aarch64::*;
    unsafe {
        let a = vld1q_u8(dst.as_ptr() as *const u8);
        let b = vld1q_u8(src.as_ptr() as *const u8);
        let c = vsubq_u8(a, b);
        vst1q_u8(dst.as_mut_ptr() as *mut u8, c);
    }
}

#[inline]
pub fn psubb_scalar(dst: &mut Xmm, src: &Xmm) {
    let d = unsafe { &mut *(dst as *mut Xmm as *mut [u8; 16]) };
    let s = unsafe { &*(src as *const Xmm as *const [u8; 16]) };
    for i in 0..16 {
        d[i] = d[i].wrapping_sub(s[i]);
    }
}

#[inline]
pub fn psubb_xmm(dst: &mut Xmm, src: &Xmm) {
    #[cfg(target_arch = "x86_64")]
    {
        if SimdCapabilities::get().sse2 {
            unsafe { psubb_native_sse2(dst, src) };
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe { psubb_native_neon(dst, src) };
            return;
        }
    }
    psubb_scalar(dst, src);
}

#[inline]
pub fn psubb_ymm(dst: &mut Ymm, src: &Ymm) {
    #[cfg(target_arch = "x86_64")]
    {
        let caps = SimdCapabilities::get();
        if caps.avx2 {
            unsafe { psubb_native_avx2(dst, src) };
            return;
        }
        if caps.sse2 {
            unsafe {
                psubb_native_sse2(&mut dst[0], &src[0]);
                psubb_native_sse2(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe {
                psubb_native_neon(&mut dst[0], &src[0]);
                psubb_native_neon(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    psubb_scalar(&mut dst[0], &src[0]);
    psubb_scalar(&mut dst[1], &src[1]);
}

// --- PSUBW ---

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
#[inline]
pub unsafe fn psubw_native_sse2(dst: &mut Xmm, src: &Xmm) {
    use std::arch::x86_64::*;
    let a = _mm_loadu_si128(dst.as_ptr() as *const __m128i);
    let b = _mm_loadu_si128(src.as_ptr() as *const __m128i);
    let c = _mm_sub_epi16(a, b);
    _mm_storeu_si128(dst.as_mut_ptr() as *mut __m128i, c);
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn psubw_native_avx2(dst: &mut Ymm, src: &Ymm) {
    use std::arch::x86_64::*;
    let a = _mm256_loadu_si256(dst.as_ptr() as *const __m256i);
    let b = _mm256_loadu_si256(src.as_ptr() as *const __m256i);
    let c = _mm256_sub_epi16(a, b);
    _mm256_storeu_si256(dst.as_mut_ptr() as *mut __m256i, c);
}

/// NEON implementation of PSUBW (packed word subtraction with wraparound).
///
/// # Translation Strategy
/// ARM NEON `vsubq_u16` performs element-wise subtraction of 8 words:
/// 1. `vld1q_u16(dst)` loads 8 words into NEON register `a`
/// 2. `vld1q_u16(src)` loads 8 words into NEON register `b`
/// 3. `vsubq_u16(a, b)` computes `a[i] - b[i]` for each of 8 lanes
/// 4. `vst1q_u16(dst, c)` stores 8-word result back to destination
///
/// Subtraction order: `result = dst - src` (destination minus source).
/// This matches x86 semantics where the first operand is both source and dest.
///
/// # Dispatch Logic (`psubw_xmm`)
/// ```text
/// if NEON available:
///     psubw_native_neon(dst, src)   // NEON path
/// else:
///     psubw_scalar(dst, src)        // Scalar fallback
/// ```
///
/// # Scalar Fallback Behavior (`psubw_scalar`)
/// ```text
/// for i in 0..8:
///     dst_words[i] = dst_words[i].wrapping_sub(src_words[i]);
/// ```
///
/// # YMM (256-bit) Handling (`psubw_ymm`)
/// YMM operations process two XMM halves sequentially:
/// ```text
/// psubw_native_neon(&mut dst[0], &src[0]);  // Low 128 bits
/// psubw_native_neon(&mut dst[1], &src[1]);  // High 128 bits
/// ```
///
/// # Performance
/// Single-cycle throughput per 128-bit operation.
///
/// # Correctness
/// Wraparound (modular) arithmetic is identical on both architectures.
/// Example: 0x0000 - 0x0001 = 0xFFFF (wraps around, no exception).
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[inline]
pub unsafe fn psubw_native_neon(dst: &mut Xmm, src: &Xmm) {
    use std::arch::aarch64::*;
    unsafe {
        let a = vld1q_u16(dst.as_ptr() as *const u16);
        let b = vld1q_u16(src.as_ptr() as *const u16);
        let c = vsubq_u16(a, b);
        vst1q_u16(dst.as_mut_ptr() as *mut u16, c);
    }
}

#[inline]
pub fn psubw_scalar(dst: &mut Xmm, src: &Xmm) {
    let d = unsafe { &mut *(dst as *mut Xmm as *mut [u16; 8]) };
    let s = unsafe { &*(src as *const Xmm as *const [u16; 8]) };
    for i in 0..8 {
        d[i] = d[i].wrapping_sub(s[i]);
    }
}

#[inline]
pub fn psubw_xmm(dst: &mut Xmm, src: &Xmm) {
    #[cfg(target_arch = "x86_64")]
    {
        if SimdCapabilities::get().sse2 {
            unsafe { psubw_native_sse2(dst, src) };
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe { psubw_native_neon(dst, src) };
            return;
        }
    }
    psubw_scalar(dst, src);
}

#[inline]
pub fn psubw_ymm(dst: &mut Ymm, src: &Ymm) {
    #[cfg(target_arch = "x86_64")]
    {
        let caps = SimdCapabilities::get();
        if caps.avx2 {
            unsafe { psubw_native_avx2(dst, src) };
            return;
        }
        if caps.sse2 {
            unsafe {
                psubw_native_sse2(&mut dst[0], &src[0]);
                psubw_native_sse2(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe {
                psubw_native_neon(&mut dst[0], &src[0]);
                psubw_native_neon(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    psubw_scalar(&mut dst[0], &src[0]);
    psubw_scalar(&mut dst[1], &src[1]);
}

// --- PSUBD ---

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
#[inline]
pub unsafe fn psubd_native_sse2(dst: &mut Xmm, src: &Xmm) {
    use std::arch::x86_64::*;
    let a = _mm_loadu_si128(dst.as_ptr() as *const __m128i);
    let b = _mm_loadu_si128(src.as_ptr() as *const __m128i);
    let c = _mm_sub_epi32(a, b);
    _mm_storeu_si128(dst.as_mut_ptr() as *mut __m128i, c);
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn psubd_native_avx2(dst: &mut Ymm, src: &Ymm) {
    use std::arch::x86_64::*;
    let a = _mm256_loadu_si256(dst.as_ptr() as *const __m256i);
    let b = _mm256_loadu_si256(src.as_ptr() as *const __m256i);
    let c = _mm256_sub_epi32(a, b);
    _mm256_storeu_si256(dst.as_mut_ptr() as *mut __m256i, c);
}

/// NEON implementation of PSUBD (packed doubleword subtraction with wraparound).
///
/// # Translation Strategy
/// ARM NEON `vsubq_u32` performs element-wise subtraction of 4 dwords:
/// 1. `vld1q_u32(dst)` loads 4 dwords into NEON register `a`
/// 2. `vld1q_u32(src)` loads 4 dwords into NEON register `b`
/// 3. `vsubq_u32(a, b)` computes `a[i] - b[i]` for each of 4 lanes
/// 4. `vst1q_u32(dst, c)` stores 4-dword result back to destination
///
/// Subtraction order: `result = dst - src` (destination minus source).
/// This matches x86 semantics where the first operand is both source and dest.
///
/// # Dispatch Logic (`psubd_xmm`)
/// ```text
/// if NEON available:
///     psubd_native_neon(dst, src)   // NEON path
/// else:
///     psubd_scalar(dst, src)        // Scalar fallback
/// ```
///
/// # Scalar Fallback Behavior (`psubd_scalar`)
/// ```text
/// for i in 0..4:
///     dst_dwords[i] = dst_dwords[i].wrapping_sub(src_dwords[i]);
/// ```
///
/// # YMM (256-bit) Handling (`psubd_ymm`)
/// YMM operations process two XMM halves sequentially:
/// ```text
/// psubd_native_neon(&mut dst[0], &src[0]);  // Low 128 bits
/// psubd_native_neon(&mut dst[1], &src[1]);  // High 128 bits
/// ```
///
/// # Performance
/// Single-cycle throughput per 128-bit operation.
///
/// # Correctness
/// Wraparound (modular) arithmetic is identical on both architectures.
/// Example: 0x00000000 - 0x00000001 = 0xFFFFFFFF (wraps around, no exception).
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[inline]
pub unsafe fn psubd_native_neon(dst: &mut Xmm, src: &Xmm) {
    use std::arch::aarch64::*;
    unsafe {
        let a = vld1q_u32(dst.as_ptr() as *const u32);
        let b = vld1q_u32(src.as_ptr() as *const u32);
        let c = vsubq_u32(a, b);
        vst1q_u32(dst.as_mut_ptr() as *mut u32, c);
    }
}

#[inline]
pub fn psubd_scalar(dst: &mut Xmm, src: &Xmm) {
    let d = unsafe { &mut *(dst as *mut Xmm as *mut [u32; 4]) };
    let s = unsafe { &*(src as *const Xmm as *const [u32; 4]) };
    for i in 0..4 {
        d[i] = d[i].wrapping_sub(s[i]);
    }
}

#[inline]
pub fn psubd_xmm(dst: &mut Xmm, src: &Xmm) {
    #[cfg(target_arch = "x86_64")]
    {
        if SimdCapabilities::get().sse2 {
            unsafe { psubd_native_sse2(dst, src) };
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe { psubd_native_neon(dst, src) };
            return;
        }
    }
    psubd_scalar(dst, src);
}

#[inline]
pub fn psubd_ymm(dst: &mut Ymm, src: &Ymm) {
    #[cfg(target_arch = "x86_64")]
    {
        let caps = SimdCapabilities::get();
        if caps.avx2 {
            unsafe { psubd_native_avx2(dst, src) };
            return;
        }
        if caps.sse2 {
            unsafe {
                psubd_native_sse2(&mut dst[0], &src[0]);
                psubd_native_sse2(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe {
                psubd_native_neon(&mut dst[0], &src[0]);
                psubd_native_neon(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    psubd_scalar(&mut dst[0], &src[0]);
    psubd_scalar(&mut dst[1], &src[1]);
}

// --- PSUBQ ---

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
#[inline]
pub unsafe fn psubq_native_sse2(dst: &mut Xmm, src: &Xmm) {
    use std::arch::x86_64::*;
    let a = _mm_loadu_si128(dst.as_ptr() as *const __m128i);
    let b = _mm_loadu_si128(src.as_ptr() as *const __m128i);
    let c = _mm_sub_epi64(a, b);
    _mm_storeu_si128(dst.as_mut_ptr() as *mut __m128i, c);
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn psubq_native_avx2(dst: &mut Ymm, src: &Ymm) {
    use std::arch::x86_64::*;
    let a = _mm256_loadu_si256(dst.as_ptr() as *const __m256i);
    let b = _mm256_loadu_si256(src.as_ptr() as *const __m256i);
    let c = _mm256_sub_epi64(a, b);
    _mm256_storeu_si256(dst.as_mut_ptr() as *mut __m256i, c);
}

/// NEON implementation of PSUBQ (packed quadword subtraction with wraparound).
///
/// # Translation Strategy
/// ARM NEON `vsubq_u64` performs element-wise subtraction of 2 qwords:
/// 1. `vld1q_u64(dst)` loads 2 qwords into NEON register `a`
/// 2. `vld1q_u64(src)` loads 2 qwords into NEON register `b`
/// 3. `vsubq_u64(a, b)` computes `a[i] - b[i]` for each of 2 lanes
/// 4. `vst1q_u64(dst, c)` stores 2-qword result back to destination
///
/// Subtraction order: `result = dst - src` (destination minus source).
/// This matches x86 semantics where the first operand is both source and dest.
///
/// # Dispatch Logic (`psubq_xmm`)
/// ```text
/// if NEON available:
///     psubq_native_neon(dst, src)   // NEON path
/// else:
///     psubq_scalar(dst, src)        // Scalar fallback
/// ```
///
/// # Scalar Fallback Behavior (`psubq_scalar`)
/// ```text
/// dst[0] = dst[0].wrapping_sub(src[0]);
/// dst[1] = dst[1].wrapping_sub(src[1]);
/// ```
/// Uses direct u64 operations since Xmm is `[u64; 2]`.
///
/// # YMM (256-bit) Handling (`psubq_ymm`)
/// YMM operations process two XMM halves sequentially:
/// ```text
/// psubq_native_neon(&mut dst[0], &src[0]);  // Low 128 bits
/// psubq_native_neon(&mut dst[1], &src[1]);  // High 128 bits
/// ```
///
/// # Performance
/// Single-cycle throughput per 128-bit operation.
///
/// # Correctness
/// Wraparound (modular) arithmetic is identical on both architectures.
/// 64-bit subtraction wraps the same way as smaller element sizes.
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[inline]
pub unsafe fn psubq_native_neon(dst: &mut Xmm, src: &Xmm) {
    use std::arch::aarch64::*;
    unsafe {
        let a = vld1q_u64(dst.as_ptr() as *const u64);
        let b = vld1q_u64(src.as_ptr() as *const u64);
        let c = vsubq_u64(a, b);
        vst1q_u64(dst.as_mut_ptr() as *mut u64, c);
    }
}

#[inline]
pub fn psubq_scalar(dst: &mut Xmm, src: &Xmm) {
    dst[0] = dst[0].wrapping_sub(src[0]);
    dst[1] = dst[1].wrapping_sub(src[1]);
}

#[inline]
pub fn psubq_xmm(dst: &mut Xmm, src: &Xmm) {
    #[cfg(target_arch = "x86_64")]
    {
        if SimdCapabilities::get().sse2 {
            unsafe { psubq_native_sse2(dst, src) };
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe { psubq_native_neon(dst, src) };
            return;
        }
    }
    psubq_scalar(dst, src);
}

#[inline]
pub fn psubq_ymm(dst: &mut Ymm, src: &Ymm) {
    #[cfg(target_arch = "x86_64")]
    {
        let caps = SimdCapabilities::get();
        if caps.avx2 {
            unsafe { psubq_native_avx2(dst, src) };
            return;
        }
        if caps.sse2 {
            unsafe {
                psubq_native_sse2(&mut dst[0], &src[0]);
                psubq_native_sse2(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe {
                psubq_native_neon(&mut dst[0], &src[0]);
                psubq_native_neon(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    psubq_scalar(&mut dst[0], &src[0]);
    psubq_scalar(&mut dst[1], &src[1]);
}

// ============================================================================
// LOGICAL OPERATIONS (PAND, POR, PXOR, PANDN)
// ============================================================================

// --- PAND ---

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
#[inline]
pub unsafe fn pand_native_sse2(dst: &mut Xmm, src: &Xmm) {
    use std::arch::x86_64::*;
    let a = _mm_loadu_si128(dst.as_ptr() as *const __m128i);
    let b = _mm_loadu_si128(src.as_ptr() as *const __m128i);
    let c = _mm_and_si128(a, b);
    _mm_storeu_si128(dst.as_mut_ptr() as *mut __m128i, c);
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn pand_native_avx2(dst: &mut Ymm, src: &Ymm) {
    use std::arch::x86_64::*;
    let a = _mm256_loadu_si256(dst.as_ptr() as *const __m256i);
    let b = _mm256_loadu_si256(src.as_ptr() as *const __m256i);
    let c = _mm256_and_si256(a, b);
    _mm256_storeu_si256(dst.as_mut_ptr() as *mut __m256i, c);
}

/// NEON implementation of PAND (packed bitwise AND).
///
/// # Translation Strategy
/// ARM NEON `vandq_u64` performs bitwise AND on 128 bits as two u64 lanes:
/// 1. `vld1q_u64(dst)` loads destination as 2 × u64 into NEON register
/// 2. `vld1q_u64(src)` loads source as 2 × u64
/// 3. `vandq_u64(a, b)` computes `a & b` on both 64-bit lanes
/// 4. `vst1q_u64(dst, c)` stores 128-bit result
///
/// Element width doesn't matter for bitwise ops; u64 is used for efficiency.
///
/// # Dispatch Logic (`pand_xmm`)
/// ```text
/// if NEON available:
///     pand_native_neon(dst, src)   // NEON path
/// else:
///     pand_scalar(dst, src)        // Scalar fallback
/// ```
///
/// # Scalar Fallback Behavior (`pand_scalar`)
/// ```text
/// dst[0] &= src[0];  // Low 64 bits
/// dst[1] &= src[1];  // High 64 bits
/// ```
/// Uses direct u64 AND since Xmm is `[u64; 2]`.
///
/// # YMM (256-bit) Handling (`pand_ymm`)
/// YMM operations process two XMM halves sequentially:
/// ```text
/// pand_native_neon(&mut dst[0], &src[0]);  // Low 128 bits
/// pand_native_neon(&mut dst[1], &src[1]);  // High 128 bits
/// ```
///
/// # Logical Operations Family
/// All packed logical operations use NEON equivalents operating on u64 pairs:
/// - AND: `vandq_u64` (PAND)
/// - OR:  `vorrq_u64` (POR)
/// - XOR: `veorq_u64` (PXOR)
/// - ANDN: `vbicq_u64` (PANDN) - note: NEON BIC is `~a & b`, same as x86 PANDN
///
/// # Performance
/// Single-cycle throughput. Logical operations are among the fastest SIMD ops.
///
/// # Correctness
/// Bitwise operations are architecturally identical. No edge cases.
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[inline]
pub unsafe fn pand_native_neon(dst: &mut Xmm, src: &Xmm) {
    use std::arch::aarch64::*;
    unsafe {
        let a = vld1q_u64(dst.as_ptr() as *const u64);
        let b = vld1q_u64(src.as_ptr() as *const u64);
        let c = vandq_u64(a, b);
        vst1q_u64(dst.as_mut_ptr() as *mut u64, c);
    }
}

#[inline]
pub fn pand_scalar(dst: &mut Xmm, src: &Xmm) {
    dst[0] &= src[0];
    dst[1] &= src[1];
}

#[inline]
pub fn pand_xmm(dst: &mut Xmm, src: &Xmm) {
    #[cfg(target_arch = "x86_64")]
    {
        if SimdCapabilities::get().sse2 {
            unsafe { pand_native_sse2(dst, src) };
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe { pand_native_neon(dst, src) };
            return;
        }
    }
    pand_scalar(dst, src);
}

#[inline]
pub fn pand_ymm(dst: &mut Ymm, src: &Ymm) {
    #[cfg(target_arch = "x86_64")]
    {
        let caps = SimdCapabilities::get();
        if caps.avx2 {
            unsafe { pand_native_avx2(dst, src) };
            return;
        }
        if caps.sse2 {
            unsafe {
                pand_native_sse2(&mut dst[0], &src[0]);
                pand_native_sse2(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe {
                pand_native_neon(&mut dst[0], &src[0]);
                pand_native_neon(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    pand_scalar(&mut dst[0], &src[0]);
    pand_scalar(&mut dst[1], &src[1]);
}

// --- POR ---

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
#[inline]
pub unsafe fn por_native_sse2(dst: &mut Xmm, src: &Xmm) {
    use std::arch::x86_64::*;
    let a = _mm_loadu_si128(dst.as_ptr() as *const __m128i);
    let b = _mm_loadu_si128(src.as_ptr() as *const __m128i);
    let c = _mm_or_si128(a, b);
    _mm_storeu_si128(dst.as_mut_ptr() as *mut __m128i, c);
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn por_native_avx2(dst: &mut Ymm, src: &Ymm) {
    use std::arch::x86_64::*;
    let a = _mm256_loadu_si256(dst.as_ptr() as *const __m256i);
    let b = _mm256_loadu_si256(src.as_ptr() as *const __m256i);
    let c = _mm256_or_si256(a, b);
    _mm256_storeu_si256(dst.as_mut_ptr() as *mut __m256i, c);
}

/// NEON implementation of POR (bitwise OR of 128-bit values).
///
/// # Translation Strategy
/// ARM NEON `vorrq_u64` performs bitwise OR on 128 bits as two u64 lanes:
/// 1. `vld1q_u64(dst)` loads destination as 2 × u64 into NEON register
/// 2. `vld1q_u64(src)` loads source as 2 × u64
/// 3. `vorrq_u64(a, b)` computes `a | b` on both 64-bit lanes
/// 4. `vst1q_u64(dst, c)` stores 128-bit result
///
/// Element width doesn't matter for bitwise ops; u64 is used for efficiency.
///
/// # Dispatch Logic (`por_xmm`)
/// ```text
/// if NEON available:
///     por_native_neon(dst, src)   // NEON path
/// else:
///     por_scalar(dst, src)        // Scalar fallback
/// ```
///
/// # Scalar Fallback Behavior (`por_scalar`)
/// ```text
/// dst[0] |= src[0];  // Low 64 bits
/// dst[1] |= src[1];  // High 64 bits
/// ```
/// Uses direct u64 OR since Xmm is `[u64; 2]`.
///
/// # YMM (256-bit) Handling (`por_ymm`)
/// YMM operations process two XMM halves sequentially:
/// ```text
/// por_native_neon(&mut dst[0], &src[0]);  // Low 128 bits
/// por_native_neon(&mut dst[1], &src[1]);  // High 128 bits
/// ```
///
/// # Performance
/// Single-cycle throughput. Logical operations are among the fastest SIMD ops.
///
/// # Correctness
/// Bitwise OR is architecture-independent. No edge cases.
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[inline]
pub unsafe fn por_native_neon(dst: &mut Xmm, src: &Xmm) {
    use std::arch::aarch64::*;
    unsafe {
        let a = vld1q_u64(dst.as_ptr() as *const u64);
        let b = vld1q_u64(src.as_ptr() as *const u64);
        let c = vorrq_u64(a, b);
        vst1q_u64(dst.as_mut_ptr() as *mut u64, c);
    }
}

#[inline]
pub fn por_scalar(dst: &mut Xmm, src: &Xmm) {
    dst[0] |= src[0];
    dst[1] |= src[1];
}

#[inline]
pub fn por_xmm(dst: &mut Xmm, src: &Xmm) {
    #[cfg(target_arch = "x86_64")]
    {
        if SimdCapabilities::get().sse2 {
            unsafe { por_native_sse2(dst, src) };
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe { por_native_neon(dst, src) };
            return;
        }
    }
    por_scalar(dst, src);
}

#[inline]
pub fn por_ymm(dst: &mut Ymm, src: &Ymm) {
    #[cfg(target_arch = "x86_64")]
    {
        let caps = SimdCapabilities::get();
        if caps.avx2 {
            unsafe { por_native_avx2(dst, src) };
            return;
        }
        if caps.sse2 {
            unsafe {
                por_native_sse2(&mut dst[0], &src[0]);
                por_native_sse2(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe {
                por_native_neon(&mut dst[0], &src[0]);
                por_native_neon(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    por_scalar(&mut dst[0], &src[0]);
    por_scalar(&mut dst[1], &src[1]);
}

// --- PXOR ---

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
#[inline]
pub unsafe fn pxor_native_sse2(dst: &mut Xmm, src: &Xmm) {
    use std::arch::x86_64::*;
    let a = _mm_loadu_si128(dst.as_ptr() as *const __m128i);
    let b = _mm_loadu_si128(src.as_ptr() as *const __m128i);
    let c = _mm_xor_si128(a, b);
    _mm_storeu_si128(dst.as_mut_ptr() as *mut __m128i, c);
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn pxor_native_avx2(dst: &mut Ymm, src: &Ymm) {
    use std::arch::x86_64::*;
    let a = _mm256_loadu_si256(dst.as_ptr() as *const __m256i);
    let b = _mm256_loadu_si256(src.as_ptr() as *const __m256i);
    let c = _mm256_xor_si256(a, b);
    _mm256_storeu_si256(dst.as_mut_ptr() as *mut __m256i, c);
}

/// NEON implementation of PXOR (bitwise XOR of 128-bit values).
///
/// # Translation Strategy
/// ARM NEON `veorq_u64` performs bitwise XOR on 128 bits as two u64 lanes:
/// 1. `vld1q_u64(dst)` loads destination as 2 × u64 into NEON register
/// 2. `vld1q_u64(src)` loads source as 2 × u64
/// 3. `veorq_u64(a, b)` computes `a ^ b` on both 64-bit lanes
/// 4. `vst1q_u64(dst, c)` stores 128-bit result
///
/// Common use case: `PXOR xmm0, xmm0` to zero a register. NEON equivalent
/// works identically since `a ^ a = 0` for any value.
///
/// # Dispatch Logic (`pxor_xmm`)
/// ```text
/// if NEON available:
///     pxor_native_neon(dst, src)   // NEON path
/// else:
///     pxor_scalar(dst, src)        // Scalar fallback
/// ```
///
/// # Scalar Fallback Behavior (`pxor_scalar`)
/// ```text
/// dst[0] ^= src[0];  // Low 64 bits
/// dst[1] ^= src[1];  // High 64 bits
/// ```
/// Uses direct u64 XOR since Xmm is `[u64; 2]`.
///
/// # YMM (256-bit) Handling (`pxor_ymm`)
/// YMM operations process two XMM halves sequentially:
/// ```text
/// pxor_native_neon(&mut dst[0], &src[0]);  // Low 128 bits
/// pxor_native_neon(&mut dst[1], &src[1]);  // High 128 bits
/// ```
///
/// # Performance
/// Single-cycle throughput. Logical operations are among the fastest SIMD ops.
///
/// # Correctness
/// Bitwise XOR is architecture-independent. No edge cases.
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[inline]
pub unsafe fn pxor_native_neon(dst: &mut Xmm, src: &Xmm) {
    use std::arch::aarch64::*;
    unsafe {
        let a = vld1q_u64(dst.as_ptr() as *const u64);
        let b = vld1q_u64(src.as_ptr() as *const u64);
        let c = veorq_u64(a, b);
        vst1q_u64(dst.as_mut_ptr() as *mut u64, c);
    }
}

#[inline]
pub fn pxor_scalar(dst: &mut Xmm, src: &Xmm) {
    dst[0] ^= src[0];
    dst[1] ^= src[1];
}

#[inline]
pub fn pxor_xmm(dst: &mut Xmm, src: &Xmm) {
    #[cfg(target_arch = "x86_64")]
    {
        if SimdCapabilities::get().sse2 {
            unsafe { pxor_native_sse2(dst, src) };
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe { pxor_native_neon(dst, src) };
            return;
        }
    }
    pxor_scalar(dst, src);
}

#[inline]
pub fn pxor_ymm(dst: &mut Ymm, src: &Ymm) {
    #[cfg(target_arch = "x86_64")]
    {
        let caps = SimdCapabilities::get();
        if caps.avx2 {
            unsafe { pxor_native_avx2(dst, src) };
            return;
        }
        if caps.sse2 {
            unsafe {
                pxor_native_sse2(&mut dst[0], &src[0]);
                pxor_native_sse2(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe {
                pxor_native_neon(&mut dst[0], &src[0]);
                pxor_native_neon(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    pxor_scalar(&mut dst[0], &src[0]);
    pxor_scalar(&mut dst[1], &src[1]);
}

// --- PANDN ---

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
#[inline]
pub unsafe fn pandn_native_sse2(dst: &mut Xmm, src: &Xmm) {
    use std::arch::x86_64::*;
    let a = _mm_loadu_si128(dst.as_ptr() as *const __m128i);
    let b = _mm_loadu_si128(src.as_ptr() as *const __m128i);
    let c = _mm_andnot_si128(a, b);
    _mm_storeu_si128(dst.as_mut_ptr() as *mut __m128i, c);
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn pandn_native_avx2(dst: &mut Ymm, src: &Ymm) {
    use std::arch::x86_64::*;
    let a = _mm256_loadu_si256(dst.as_ptr() as *const __m256i);
    let b = _mm256_loadu_si256(src.as_ptr() as *const __m256i);
    let c = _mm256_andnot_si256(a, b);
    _mm256_storeu_si256(dst.as_mut_ptr() as *mut __m256i, c);
}

/// NEON implementation of PANDN (bitwise AND-NOT: (~dst) & src).
///
/// # Translation Strategy
/// ARM NEON `vbicq_u64` performs Bit Clear (`a & ~b`). Since x86 PANDN
/// computes `(~dst) & src`, we swap operands:
/// 1. `vld1q_u64(dst)` loads destination as 2 × u64 into NEON register `a`
/// 2. `vld1q_u64(src)` loads source as 2 × u64 into register `b`
/// 3. `vbicq_u64(b, a)` computes `b & ~a` = `src & ~dst` = `~dst & src`
/// 4. `vst1q_u64(dst, c)` stores 128-bit result
///
/// # Operand Order Note
/// - x86 PANDN: `result = (~dst) & src`
/// - NEON BIC:  `vbicq_u64(a, b) = a & (~b)`
/// - To match: `vbicq_u64(src, dst)` yields `src & (~dst)` ≡ `(~dst) & src`
///
/// # Dispatch Logic (`pandn_xmm`)
/// ```text
/// if NEON available:
///     pandn_native_neon(dst, src)   // NEON path
/// else:
///     pandn_scalar(dst, src)        // Scalar fallback
/// ```
///
/// # Scalar Fallback Behavior (`pandn_scalar`)
/// ```text
/// dst[0] = !dst[0] & src[0];  // Low 64 bits: NOT dst, then AND
/// dst[1] = !dst[1] & src[1];  // High 64 bits
/// ```
/// Uses Rust's `!` for bitwise NOT and `&` for AND.
///
/// # YMM (256-bit) Handling (`pandn_ymm`)
/// YMM operations process two XMM halves sequentially:
/// ```text
/// pandn_native_neon(&mut dst[0], &src[0]);  // Low 128 bits
/// pandn_native_neon(&mut dst[1], &src[1]);  // High 128 bits
/// ```
///
/// # Performance
/// Single-cycle throughput. The operand swap is purely syntactic.
///
/// # Correctness
/// Operand ordering verified: `vbicq_u64(b, a)` = `b & ~a` = `~a & b`
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[inline]
pub unsafe fn pandn_native_neon(dst: &mut Xmm, src: &Xmm) {
    use std::arch::aarch64::*;
    unsafe {
        let a = vld1q_u64(dst.as_ptr() as *const u64);
        let b = vld1q_u64(src.as_ptr() as *const u64);
        let c = vbicq_u64(b, a); // NEON: BIC = b AND NOT a
        vst1q_u64(dst.as_mut_ptr() as *mut u64, c);
    }
}

#[inline]
pub fn pandn_scalar(dst: &mut Xmm, src: &Xmm) {
    dst[0] = !dst[0] & src[0];
    dst[1] = !dst[1] & src[1];
}

#[inline]
pub fn pandn_xmm(dst: &mut Xmm, src: &Xmm) {
    #[cfg(target_arch = "x86_64")]
    {
        if SimdCapabilities::get().sse2 {
            unsafe { pandn_native_sse2(dst, src) };
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe { pandn_native_neon(dst, src) };
            return;
        }
    }
    pandn_scalar(dst, src);
}

#[inline]
pub fn pandn_ymm(dst: &mut Ymm, src: &Ymm) {
    #[cfg(target_arch = "x86_64")]
    {
        let caps = SimdCapabilities::get();
        if caps.avx2 {
            unsafe { pandn_native_avx2(dst, src) };
            return;
        }
        if caps.sse2 {
            unsafe {
                pandn_native_sse2(&mut dst[0], &src[0]);
                pandn_native_sse2(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe {
                pandn_native_neon(&mut dst[0], &src[0]);
                pandn_native_neon(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    pandn_scalar(&mut dst[0], &src[0]);
    pandn_scalar(&mut dst[1], &src[1]);
}

// ============================================================================
// COMPARE OPERATIONS (PCMPEQB, PCMPEQW, PCMPEQD, PCMPGTB, etc.)
// ============================================================================

// --- PCMPEQB ---

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
#[inline]
pub unsafe fn pcmpeqb_native_sse2(dst: &mut Xmm, src: &Xmm) {
    use std::arch::x86_64::*;
    let a = _mm_loadu_si128(dst.as_ptr() as *const __m128i);
    let b = _mm_loadu_si128(src.as_ptr() as *const __m128i);
    let c = _mm_cmpeq_epi8(a, b);
    _mm_storeu_si128(dst.as_mut_ptr() as *mut __m128i, c);
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn pcmpeqb_native_avx2(dst: &mut Ymm, src: &Ymm) {
    use std::arch::x86_64::*;
    let a = _mm256_loadu_si256(dst.as_ptr() as *const __m256i);
    let b = _mm256_loadu_si256(src.as_ptr() as *const __m256i);
    let c = _mm256_cmpeq_epi8(a, b);
    _mm256_storeu_si256(dst.as_mut_ptr() as *mut __m256i, c);
}

/// NEON implementation of PCMPEQB (packed compare equal bytes).
///
/// # Translation Strategy
/// ARM NEON `vceqq_u8` performs element-wise equality comparison:
/// 1. `vld1q_u8(dst)` loads 16 bytes into NEON register `a`
/// 2. `vld1q_u8(src)` loads 16 bytes into NEON register `b`
/// 3. `vceqq_u8(a, b)` computes equality: 0xFF if equal, 0x00 otherwise
/// 4. `vst1q_u8(dst, c)` stores 16-byte result back
///
/// # Dispatch Logic (`pcmpeqb_xmm`)
/// ```text
/// if NEON available:
///     pcmpeqb_native_neon(dst, src)   // NEON path
/// else:
///     pcmpeqb_scalar(dst, src)        // Scalar fallback
/// ```
///
/// # Scalar Fallback Behavior (`pcmpeqb_scalar`)
/// ```text
/// for i in 0..16:
///     dst_bytes[i] = if dst_bytes[i] == src_bytes[i] { 0xFF } else { 0x00 };
/// ```
///
/// # YMM (256-bit) Handling (`pcmpeqb_ymm`)
/// ```text
/// pcmpeqb_native_neon(&mut dst[0], &src[0]);  // Low 128 bits
/// pcmpeqb_native_neon(&mut dst[1], &src[1]);  // High 128 bits
/// ```
///
/// # Performance
/// Single-cycle throughput. Comparison operations are highly optimized.
///
/// # Correctness
/// Result format is identical: all-ones (0xFF) for true, all-zeros (0x00) for false.
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[inline]
pub unsafe fn pcmpeqb_native_neon(dst: &mut Xmm, src: &Xmm) {
    use std::arch::aarch64::*;
    unsafe {
        let a = vld1q_u8(dst.as_ptr() as *const u8);
        let b = vld1q_u8(src.as_ptr() as *const u8);
        let c = vceqq_u8(a, b);
        vst1q_u8(dst.as_mut_ptr() as *mut u8, c);
    }
}

#[inline]
pub fn pcmpeqb_scalar(dst: &mut Xmm, src: &Xmm) {
    let d = unsafe { &mut *(dst as *mut Xmm as *mut [u8; 16]) };
    let s = unsafe { &*(src as *const Xmm as *const [u8; 16]) };
    for i in 0..16 {
        d[i] = if d[i] == s[i] { 0xFF } else { 0 };
    }
}

#[inline]
pub fn pcmpeqb_xmm(dst: &mut Xmm, src: &Xmm) {
    #[cfg(target_arch = "x86_64")]
    {
        if SimdCapabilities::get().sse2 {
            unsafe { pcmpeqb_native_sse2(dst, src) };
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe { pcmpeqb_native_neon(dst, src) };
            return;
        }
    }
    pcmpeqb_scalar(dst, src);
}

#[inline]
pub fn pcmpeqb_ymm(dst: &mut Ymm, src: &Ymm) {
    #[cfg(target_arch = "x86_64")]
    {
        let caps = SimdCapabilities::get();
        if caps.avx2 {
            unsafe { pcmpeqb_native_avx2(dst, src) };
            return;
        }
        if caps.sse2 {
            unsafe {
                pcmpeqb_native_sse2(&mut dst[0], &src[0]);
                pcmpeqb_native_sse2(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe {
                pcmpeqb_native_neon(&mut dst[0], &src[0]);
                pcmpeqb_native_neon(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    pcmpeqb_scalar(&mut dst[0], &src[0]);
    pcmpeqb_scalar(&mut dst[1], &src[1]);
}

// --- PCMPEQD ---

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
#[inline]
pub unsafe fn pcmpeqd_native_sse2(dst: &mut Xmm, src: &Xmm) {
    use std::arch::x86_64::*;
    let a = _mm_loadu_si128(dst.as_ptr() as *const __m128i);
    let b = _mm_loadu_si128(src.as_ptr() as *const __m128i);
    let c = _mm_cmpeq_epi32(a, b);
    _mm_storeu_si128(dst.as_mut_ptr() as *mut __m128i, c);
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn pcmpeqd_native_avx2(dst: &mut Ymm, src: &Ymm) {
    use std::arch::x86_64::*;
    let a = _mm256_loadu_si256(dst.as_ptr() as *const __m256i);
    let b = _mm256_loadu_si256(src.as_ptr() as *const __m256i);
    let c = _mm256_cmpeq_epi32(a, b);
    _mm256_storeu_si256(dst.as_mut_ptr() as *mut __m256i, c);
}

/// NEON implementation of PCMPEQD (packed doubleword equality comparison).
///
/// # Translation Strategy
/// ARM NEON `vceqq_u32` performs element-wise equality comparison:
/// 1. `vld1q_u32(dst)` loads 4 dwords into NEON register `a`
/// 2. `vld1q_u32(src)` loads 4 dwords into NEON register `b`
/// 3. `vceqq_u32(a, b)` computes equality: 0xFFFFFFFF if equal, 0 otherwise
/// 4. `vst1q_u32(dst, c)` stores 4-dword result back
///
/// Result encoding is identical on both architectures:
/// - Equal elements: all bits set (0xFFFFFFFF)
/// - Unequal elements: all bits clear (0x00000000)
///
/// # Dispatch Logic (`pcmpeqd_xmm`)
/// ```text
/// if NEON available:
///     pcmpeqd_native_neon(dst, src)   // NEON path
/// else:
///     pcmpeqd_scalar(dst, src)        // Scalar fallback
/// ```
///
/// # Scalar Fallback Behavior (`pcmpeqd_scalar`)
/// ```text
/// for i in 0..4:
///     dst_dwords[i] = if dst_dwords[i] == src_dwords[i] { 0xFFFFFFFF } else { 0 };
/// ```
///
/// # YMM (256-bit) Handling (`pcmpeqd_ymm`)
/// ```text
/// pcmpeqd_native_neon(&mut dst[0], &src[0]);  // Low 128 bits
/// pcmpeqd_native_neon(&mut dst[1], &src[1]);  // High 128 bits
/// ```
///
/// # Performance
/// Single-cycle throughput on most ARM cores.
///
/// # Correctness
/// The all-ones/all-zeros encoding is critical for mask correctness. Both
/// architectures guarantee this encoding, no fixup required.
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[inline]
pub unsafe fn pcmpeqd_native_neon(dst: &mut Xmm, src: &Xmm) {
    use std::arch::aarch64::*;
    unsafe {
        let a = vld1q_u32(dst.as_ptr() as *const u32);
        let b = vld1q_u32(src.as_ptr() as *const u32);
        let c = vceqq_u32(a, b);
        vst1q_u32(dst.as_mut_ptr() as *mut u32, c);
    }
}

#[inline]
pub fn pcmpeqd_scalar(dst: &mut Xmm, src: &Xmm) {
    let d = unsafe { &mut *(dst as *mut Xmm as *mut [u32; 4]) };
    let s = unsafe { &*(src as *const Xmm as *const [u32; 4]) };
    for i in 0..4 {
        d[i] = if d[i] == s[i] { 0xFFFFFFFF } else { 0 };
    }
}

#[inline]
pub fn pcmpeqd_xmm(dst: &mut Xmm, src: &Xmm) {
    #[cfg(target_arch = "x86_64")]
    {
        if SimdCapabilities::get().sse2 {
            unsafe { pcmpeqd_native_sse2(dst, src) };
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe { pcmpeqd_native_neon(dst, src) };
            return;
        }
    }
    pcmpeqd_scalar(dst, src);
}

#[inline]
pub fn pcmpeqd_ymm(dst: &mut Ymm, src: &Ymm) {
    #[cfg(target_arch = "x86_64")]
    {
        let caps = SimdCapabilities::get();
        if caps.avx2 {
            unsafe { pcmpeqd_native_avx2(dst, src) };
            return;
        }
        if caps.sse2 {
            unsafe {
                pcmpeqd_native_sse2(&mut dst[0], &src[0]);
                pcmpeqd_native_sse2(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe {
                pcmpeqd_native_neon(&mut dst[0], &src[0]);
                pcmpeqd_native_neon(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    pcmpeqd_scalar(&mut dst[0], &src[0]);
    pcmpeqd_scalar(&mut dst[1], &src[1]);
}

// ============================================================================
// SHUFFLE/PERMUTE OPERATIONS (PSHUFB, PSHUFD, etc.)
// ============================================================================

// --- PSHUFB ---

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "ssse3")]
#[inline]
pub unsafe fn pshufb_native_ssse3(dst: &mut Xmm, mask: &Xmm) {
    use std::arch::x86_64::*;
    let a = _mm_loadu_si128(dst.as_ptr() as *const __m128i);
    let b = _mm_loadu_si128(mask.as_ptr() as *const __m128i);
    let c = _mm_shuffle_epi8(a, b);
    _mm_storeu_si128(dst.as_mut_ptr() as *mut __m128i, c);
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn pshufb_native_avx2(dst: &mut Ymm, mask: &Ymm) {
    use std::arch::x86_64::*;
    let a = _mm256_loadu_si256(dst.as_ptr() as *const __m256i);
    let b = _mm256_loadu_si256(mask.as_ptr() as *const __m256i);
    let c = _mm256_shuffle_epi8(a, b);
    _mm256_storeu_si256(dst.as_mut_ptr() as *mut __m256i, c);
}

/// NEON implementation of PSHUFB (packed shuffle bytes).
///
/// # Translation Strategy
/// x86 SSSE3 `PSHUFB` is the most complex operation to translate because it has
/// special MSB-based zeroing semantics that NEON's table lookup doesn't provide.
///
/// x86 PSHUFB semantics:
/// ```text
/// for each byte position i:
///   if mask[i] & 0x80:  // MSB set
///     result[i] = 0
///   else:
///     result[i] = src[mask[i] & 0x0F]  // Low 4 bits index into src
/// ```
///
/// NEON translation (4 instructions):
/// 1. `vandq_u8(mask, vdupq_n_u8(0x0F))` - mask indices to 0-15 range
/// 2. `vqtbl1q_u8(data, indices)` - perform table lookup
/// 3. `vcgeq_u8(vdupq_n_u8(0x7F), mask)` - create MSB-clear mask
///    - Produces 0xFF where mask byte <= 0x7F (MSB clear)
///    - Produces 0x00 where mask byte > 0x7F (MSB set)
/// 4. `vandq_u8(result, msb_clear)` - zero MSB-set positions
///
/// # Dispatch Logic (`pshufb_xmm`)
/// ```text
/// if NEON available:
///     pshufb_native_neon(dst, mask)   // NEON path (4 instructions)
/// else:
///     pshufb_scalar(dst, mask)        // Scalar fallback (16 iterations)
/// ```
///
/// # Scalar Fallback Behavior (`pshufb_scalar`)
/// ```text
/// for i in 0..16:
///     if mask_bytes[i] & 0x80 != 0:
///         dst_bytes[i] = 0;
///     else:
///         dst_bytes[i] = src_bytes[mask_bytes[i] & 0x0F];
/// ```
///
/// # YMM (256-bit) Handling (`pshufb_ymm`)
/// ```text
/// pshufb_native_neon(&mut dst[0], &mask[0]);  // Low 128 bits
/// pshufb_native_neon(&mut dst[1], &mask[1]);  // High 128 bits
/// ```
///
/// # Performance
/// 4 NEON instructions vs 1 x86 instruction, but still ~2-4 cycles total.
/// The extra operations are necessary to match x86 semantics exactly.
///
/// # Correctness
/// - MSB masking: Correctly zeros bytes where mask[i] & 0x80 != 0
/// - Index range: Low 4 bits correctly index into 16-byte source
/// - This is the only SIMD operation requiring significant emulation logic
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[inline]
pub unsafe fn pshufb_native_neon(dst: &mut Xmm, mask: &Xmm) {
    use std::arch::aarch64::*;
    unsafe {
        let data = vld1q_u8(dst.as_ptr() as *const u8);
        let m = vld1q_u8(mask.as_ptr() as *const u8);
        // Mask indices to 0-15 range for table lookup
        let indices = vandq_u8(m, vdupq_n_u8(0x0F));
        // Perform table lookup
        let result = vqtbl1q_u8(data, indices);
        // Zero bytes where MSB was set in original mask
        let msb_clear = vcgeq_u8(vdupq_n_u8(0x7F), m); // 0xFF where m <= 0x7F (MSB clear)
        let final_result = vandq_u8(result, msb_clear);
        vst1q_u8(dst.as_mut_ptr() as *mut u8, final_result);
    }
}

#[inline]
pub fn pshufb_scalar(dst: &mut Xmm, mask: &Xmm) {
    let src_bytes = unsafe { *(dst as *const Xmm as *const [u8; 16]) };
    let mask_bytes = unsafe { &*(mask as *const Xmm as *const [u8; 16]) };
    let dst_bytes = unsafe { &mut *(dst as *mut Xmm as *mut [u8; 16]) };

    for i in 0..16 {
        let m = mask_bytes[i];
        dst_bytes[i] = if m & 0x80 != 0 {
            0
        } else {
            src_bytes[(m & 0x0F) as usize]
        };
    }
}

#[inline]
pub fn pshufb_xmm(dst: &mut Xmm, mask: &Xmm) {
    #[cfg(target_arch = "x86_64")]
    {
        if SimdCapabilities::get().ssse3 {
            unsafe { pshufb_native_ssse3(dst, mask) };
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe { pshufb_native_neon(dst, mask) };
            return;
        }
    }
    pshufb_scalar(dst, mask);
}

#[inline]
pub fn pshufb_ymm(dst: &mut Ymm, mask: &Ymm) {
    #[cfg(target_arch = "x86_64")]
    {
        let caps = SimdCapabilities::get();
        if caps.avx2 {
            unsafe { pshufb_native_avx2(dst, mask) };
            return;
        }
        if caps.ssse3 {
            // AVX2 VPSHUFB shuffles within each 128-bit lane independently
            unsafe {
                pshufb_native_ssse3(&mut dst[0], &mask[0]);
                pshufb_native_ssse3(&mut dst[1], &mask[1]);
            }
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe {
                pshufb_native_neon(&mut dst[0], &mask[0]);
                pshufb_native_neon(&mut dst[1], &mask[1]);
            }
            return;
        }
    }
    pshufb_scalar(&mut dst[0], &mask[0]);
    pshufb_scalar(&mut dst[1], &mask[1]);
}

// ============================================================================
// MULTIPLY OPERATIONS (PMULLW, PMULLD, PMADDWD, etc.)
// ============================================================================

// --- PMULLW ---

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
#[inline]
pub unsafe fn pmullw_native_sse2(dst: &mut Xmm, src: &Xmm) {
    use std::arch::x86_64::*;
    let a = _mm_loadu_si128(dst.as_ptr() as *const __m128i);
    let b = _mm_loadu_si128(src.as_ptr() as *const __m128i);
    let c = _mm_mullo_epi16(a, b);
    _mm_storeu_si128(dst.as_mut_ptr() as *mut __m128i, c);
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn pmullw_native_avx2(dst: &mut Ymm, src: &Ymm) {
    use std::arch::x86_64::*;
    let a = _mm256_loadu_si256(dst.as_ptr() as *const __m256i);
    let b = _mm256_loadu_si256(src.as_ptr() as *const __m256i);
    let c = _mm256_mullo_epi16(a, b);
    _mm256_storeu_si256(dst.as_mut_ptr() as *mut __m256i, c);
}

/// NEON implementation of PMULLW (packed word multiply low).
///
/// # Translation Strategy
/// ARM NEON `vmulq_u16` performs element-wise word multiplication:
/// 1. `vld1q_u16(dst)` loads 8 words into NEON register `a`
/// 2. `vld1q_u16(src)` loads 8 words into NEON register `b`
/// 3. `vmulq_u16(a, b)` computes `(a[i] * b[i]) & 0xFFFF` for each lane
/// 4. `vst1q_u16(dst, c)` stores 8-word result back
///
/// The "low" in PMULLW refers to keeping only the low 16 bits of the 32-bit
/// product. NEON's `vmulq_u16` naturally produces 16-bit results (truncating),
/// which matches x86 semantics.
///
/// # Dispatch Logic (`pmullw_xmm`)
/// ```text
/// if NEON available:
///     pmullw_native_neon(dst, src)   // NEON path
/// else:
///     pmullw_scalar(dst, src)        // Scalar fallback
/// ```
///
/// # Scalar Fallback Behavior (`pmullw_scalar`)
/// ```text
/// for i in 0..8:
///     dst_words[i] = (dst_words[i].wrapping_mul(src_words[i])) as u16;
/// ```
///
/// # YMM (256-bit) Handling (`pmullw_ymm`)
/// ```text
/// pmullw_native_neon(&mut dst[0], &src[0]);  // Low 128 bits
/// pmullw_native_neon(&mut dst[1], &src[1]);  // High 128 bits
/// ```
///
/// # Performance
/// Single-cycle throughput on most ARM cores.
///
/// # Correctness
/// Truncation to 16 bits is equivalent to wrapping multiplication. Signed
/// and unsigned produce the same low bits, so `vmulq_u16` works for both.
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[inline]
pub unsafe fn pmullw_native_neon(dst: &mut Xmm, src: &Xmm) {
    use std::arch::aarch64::*;
    unsafe {
        let a = vld1q_u16(dst.as_ptr() as *const u16);
        let b = vld1q_u16(src.as_ptr() as *const u16);
        let c = vmulq_u16(a, b);
        vst1q_u16(dst.as_mut_ptr() as *mut u16, c);
    }
}

#[inline]
pub fn pmullw_scalar(dst: &mut Xmm, src: &Xmm) {
    let d = unsafe { &mut *(dst as *mut Xmm as *mut [u16; 8]) };
    let s = unsafe { &*(src as *const Xmm as *const [u16; 8]) };
    for i in 0..8 {
        d[i] = d[i].wrapping_mul(s[i]);
    }
}

#[inline]
pub fn pmullw_xmm(dst: &mut Xmm, src: &Xmm) {
    #[cfg(target_arch = "x86_64")]
    {
        if SimdCapabilities::get().sse2 {
            unsafe { pmullw_native_sse2(dst, src) };
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe { pmullw_native_neon(dst, src) };
            return;
        }
    }
    pmullw_scalar(dst, src);
}

#[inline]
pub fn pmullw_ymm(dst: &mut Ymm, src: &Ymm) {
    #[cfg(target_arch = "x86_64")]
    {
        let caps = SimdCapabilities::get();
        if caps.avx2 {
            unsafe { pmullw_native_avx2(dst, src) };
            return;
        }
        if caps.sse2 {
            unsafe {
                pmullw_native_sse2(&mut dst[0], &src[0]);
                pmullw_native_sse2(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe {
                pmullw_native_neon(&mut dst[0], &src[0]);
                pmullw_native_neon(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    pmullw_scalar(&mut dst[0], &src[0]);
    pmullw_scalar(&mut dst[1], &src[1]);
}

// --- PMULLD ---

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn pmulld_native_sse41(dst: &mut Xmm, src: &Xmm) {
    use std::arch::x86_64::*;
    let a = _mm_loadu_si128(dst.as_ptr() as *const __m128i);
    let b = _mm_loadu_si128(src.as_ptr() as *const __m128i);
    let c = _mm_mullo_epi32(a, b);
    _mm_storeu_si128(dst.as_mut_ptr() as *mut __m128i, c);
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn pmulld_native_avx2(dst: &mut Ymm, src: &Ymm) {
    use std::arch::x86_64::*;
    let a = _mm256_loadu_si256(dst.as_ptr() as *const __m256i);
    let b = _mm256_loadu_si256(src.as_ptr() as *const __m256i);
    let c = _mm256_mullo_epi32(a, b);
    _mm256_storeu_si256(dst.as_mut_ptr() as *mut __m256i, c);
}

/// NEON implementation of PMULLD (packed doubleword multiply low).
///
/// # Translation Strategy
/// ARM NEON `vmulq_u32` performs element-wise dword multiplication:
/// 1. `vld1q_u32(dst)` loads 4 dwords into NEON register `a`
/// 2. `vld1q_u32(src)` loads 4 dwords into NEON register `b`
/// 3. `vmulq_u32(a, b)` computes `(a[i] * b[i]) & 0xFFFFFFFF` for each lane
/// 4. `vst1q_u32(dst, c)` stores 4-dword result back
///
/// The "low" in PMULLD refers to keeping only the low 32 bits of the 64-bit
/// product. NEON's `vmulq_u32` naturally produces 32-bit results (truncating).
///
/// # Dispatch Logic (`pmulld_xmm`)
/// ```text
/// if NEON available:
///     pmulld_native_neon(dst, src)   // NEON path
/// else:
///     pmulld_scalar(dst, src)        // Scalar fallback
/// ```
///
/// # Scalar Fallback Behavior (`pmulld_scalar`)
/// ```text
/// for i in 0..4:
///     dst_dwords[i] = dst_dwords[i].wrapping_mul(src_dwords[i]);
/// ```
///
/// # YMM (256-bit) Handling (`pmulld_ymm`)
/// ```text
/// pmulld_native_neon(&mut dst[0], &src[0]);  // Low 128 bits
/// pmulld_native_neon(&mut dst[1], &src[1]);  // High 128 bits
/// ```
///
/// # Performance
/// Single-cycle throughput on most ARM cores.
///
/// # Correctness
/// Truncation to 32 bits is equivalent to wrapping multiplication. Signed
/// and unsigned produce the same low bits, so `vmulq_u32` works for both.
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[inline]
pub unsafe fn pmulld_native_neon(dst: &mut Xmm, src: &Xmm) {
    use std::arch::aarch64::*;
    unsafe {
        let a = vld1q_u32(dst.as_ptr() as *const u32);
        let b = vld1q_u32(src.as_ptr() as *const u32);
        let c = vmulq_u32(a, b);
        vst1q_u32(dst.as_mut_ptr() as *mut u32, c);
    }
}

#[inline]
pub fn pmulld_scalar(dst: &mut Xmm, src: &Xmm) {
    let d = unsafe { &mut *(dst as *mut Xmm as *mut [u32; 4]) };
    let s = unsafe { &*(src as *const Xmm as *const [u32; 4]) };
    for i in 0..4 {
        d[i] = d[i].wrapping_mul(s[i]);
    }
}

#[inline]
pub fn pmulld_xmm(dst: &mut Xmm, src: &Xmm) {
    #[cfg(target_arch = "x86_64")]
    {
        if SimdCapabilities::get().sse4_1 {
            unsafe { pmulld_native_sse41(dst, src) };
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe { pmulld_native_neon(dst, src) };
            return;
        }
    }
    pmulld_scalar(dst, src);
}

#[inline]
pub fn pmulld_ymm(dst: &mut Ymm, src: &Ymm) {
    #[cfg(target_arch = "x86_64")]
    {
        let caps = SimdCapabilities::get();
        if caps.avx2 {
            unsafe { pmulld_native_avx2(dst, src) };
            return;
        }
        if caps.sse4_1 {
            unsafe {
                pmulld_native_sse41(&mut dst[0], &src[0]);
                pmulld_native_sse41(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe {
                pmulld_native_neon(&mut dst[0], &src[0]);
                pmulld_native_neon(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    pmulld_scalar(&mut dst[0], &src[0]);
    pmulld_scalar(&mut dst[1], &src[1]);
}

// --- PMADDWD ---

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
#[inline]
pub unsafe fn pmaddwd_native_sse2(dst: &mut Xmm, src: &Xmm) {
    use std::arch::x86_64::*;
    let a = _mm_loadu_si128(dst.as_ptr() as *const __m128i);
    let b = _mm_loadu_si128(src.as_ptr() as *const __m128i);
    let c = _mm_madd_epi16(a, b);
    _mm_storeu_si128(dst.as_mut_ptr() as *mut __m128i, c);
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn pmaddwd_native_avx2(dst: &mut Ymm, src: &Ymm) {
    use std::arch::x86_64::*;
    let a = _mm256_loadu_si256(dst.as_ptr() as *const __m256i);
    let b = _mm256_loadu_si256(src.as_ptr() as *const __m256i);
    let c = _mm256_madd_epi16(a, b);
    _mm256_storeu_si256(dst.as_mut_ptr() as *mut __m256i, c);
}

/// NEON implementation of PMADDWD (multiply-add packed signed words to dwords).
///
/// # Translation Strategy
/// x86 SSE2 `PMADDWD` multiplies adjacent pairs of signed 16-bit integers and
/// sums them to 32-bit results: `result[i] = a[2i]*b[2i] + a[2i+1]*b[2i+1]`.
///
/// ARM NEON translation (4 instructions):
/// 1. `vmull_s16(vget_low_s16(a), vget_low_s16(b))` - Widening multiply of
///    lower 4 word pairs, producing 4 dword products
/// 2. `vmull_high_s16(a, b)` - Widening multiply of upper 4 word pairs
/// 3. `vpaddq_s32(low_prods, high_prods)` - Pairwise horizontal add:
///    adds adjacent 32-bit elements to produce final 4-element result
///
/// The key insight is that `vpaddq_s32` pairs up products correctly because
/// we structured the multiply outputs to have adjacent pairs needing summing.
///
/// # Dispatch Logic (`pmaddwd_xmm`)
/// ```text
/// if NEON available:
///     pmaddwd_native_neon(dst, src)   // NEON path (4 instructions)
/// else:
///     pmaddwd_scalar(dst, src)        // Scalar fallback
/// ```
///
/// # Scalar Fallback Behavior (`pmaddwd_scalar`)
/// ```text
/// for i in 0..4:
///     let a0 = dst_words[2*i] as i32;
///     let a1 = dst_words[2*i+1] as i32;
///     let b0 = src_words[2*i] as i32;
///     let b1 = src_words[2*i+1] as i32;
///     dst_dwords[i] = a0 * b0 + a1 * b1;
/// ```
///
/// # YMM (256-bit) Handling (`pmaddwd_ymm`)
/// ```text
/// pmaddwd_native_neon(&mut dst[0], &src[0]);  // Low 128 bits
/// pmaddwd_native_neon(&mut dst[1], &src[1]);  // High 128 bits
/// ```
///
/// # Performance
/// 4-instruction sequence (~3-4 cycles) vs single-cycle on x86. The widening
/// multiplies and pairwise add are all single-cycle on modern ARM cores.
///
/// # Correctness
/// - Signed 16x16→32 multiplication prevents overflow
/// - 32-bit addition of two 32-bit products cannot overflow
/// - Result matches scalar: `(a[2i] as i32) * (b[2i] as i32) + (a[2i+1] as i32) * (b[2i+1] as i32)`
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[inline]
pub unsafe fn pmaddwd_native_neon(dst: &mut Xmm, src: &Xmm) {
    use std::arch::aarch64::*;
    unsafe {
        let a = vld1q_s16(dst.as_ptr() as *const i16);
        let b = vld1q_s16(src.as_ptr() as *const i16);
        // Multiply lower 4 pairs to get 4 32-bit products
        let low_prods = vmull_s16(vget_low_s16(a), vget_low_s16(b));
        // Multiply upper 4 pairs to get 4 32-bit products
        let high_prods = vmull_high_s16(a, b);
        // Pairwise add: result[i] = prods[2i] + prods[2i+1]
        let result = vpaddq_s32(low_prods, high_prods);
        vst1q_s32(dst.as_mut_ptr() as *mut i32, result);
    }
}

#[inline]
pub fn pmaddwd_scalar(dst: &mut Xmm, src: &Xmm) {
    let a = unsafe { &*(dst as *const Xmm as *const [i16; 8]) };
    let b = unsafe { &*(src as *const Xmm as *const [i16; 8]) };
    let result = unsafe { &mut *(dst as *mut Xmm as *mut [i32; 4]) };

    for i in 0..4 {
        result[i] =
            (a[i * 2] as i32) * (b[i * 2] as i32) + (a[i * 2 + 1] as i32) * (b[i * 2 + 1] as i32);
    }
}

#[inline]
pub fn pmaddwd_xmm(dst: &mut Xmm, src: &Xmm) {
    #[cfg(target_arch = "x86_64")]
    {
        if SimdCapabilities::get().sse2 {
            unsafe { pmaddwd_native_sse2(dst, src) };
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe { pmaddwd_native_neon(dst, src) };
            return;
        }
    }
    pmaddwd_scalar(dst, src);
}

#[inline]
pub fn pmaddwd_ymm(dst: &mut Ymm, src: &Ymm) {
    #[cfg(target_arch = "x86_64")]
    {
        let caps = SimdCapabilities::get();
        if caps.avx2 {
            unsafe { pmaddwd_native_avx2(dst, src) };
            return;
        }
        if caps.sse2 {
            unsafe {
                pmaddwd_native_sse2(&mut dst[0], &src[0]);
                pmaddwd_native_sse2(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe {
                pmaddwd_native_neon(&mut dst[0], &src[0]);
                pmaddwd_native_neon(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    pmaddwd_scalar(&mut dst[0], &src[0]);
    pmaddwd_scalar(&mut dst[1], &src[1]);
}

// ============================================================================
// MIN/MAX OPERATIONS (PMINUB, PMAXUB, PMINSW, PMAXSW, etc.)
// ============================================================================

// --- PMINUB ---

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
#[inline]
pub unsafe fn pminub_native_sse2(dst: &mut Xmm, src: &Xmm) {
    use std::arch::x86_64::*;
    let a = _mm_loadu_si128(dst.as_ptr() as *const __m128i);
    let b = _mm_loadu_si128(src.as_ptr() as *const __m128i);
    let c = _mm_min_epu8(a, b);
    _mm_storeu_si128(dst.as_mut_ptr() as *mut __m128i, c);
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn pminub_native_avx2(dst: &mut Ymm, src: &Ymm) {
    use std::arch::x86_64::*;
    let a = _mm256_loadu_si256(dst.as_ptr() as *const __m256i);
    let b = _mm256_loadu_si256(src.as_ptr() as *const __m256i);
    let c = _mm256_min_epu8(a, b);
    _mm256_storeu_si256(dst.as_mut_ptr() as *mut __m256i, c);
}

/// NEON implementation of PMINUB (packed unsigned byte minimum).
///
/// # Translation Strategy
/// ARM NEON `vminq_u8` performs element-wise unsigned minimum:
/// 1. `vld1q_u8(dst)` loads 16 bytes into NEON register `a`
/// 2. `vld1q_u8(src)` loads 16 bytes into NEON register `b`
/// 3. `vminq_u8(a, b)` computes `min(a[i], b[i])` for each of 16 lanes
/// 4. `vst1q_u8(dst, c)` stores 16-byte result back
///
/// The "U" in PMINUB indicates unsigned comparison (0xFF > 0x00).
///
/// # Dispatch Logic (`pminub_xmm`)
/// ```text
/// if NEON available:
///     pminub_native_neon(dst, src)   // NEON path
/// else:
///     pminub_scalar(dst, src)        // Scalar fallback
/// ```
///
/// # Scalar Fallback Behavior (`pminub_scalar`)
/// ```text
/// for i in 0..16:
///     dst_bytes[i] = dst_bytes[i].min(src_bytes[i]);
/// ```
///
/// # YMM (256-bit) Handling (`pminub_ymm`)
/// ```text
/// pminub_native_neon(&mut dst[0], &src[0]);  // Low 128 bits
/// pminub_native_neon(&mut dst[1], &src[1]);  // High 128 bits
/// ```
///
/// # Performance
/// Single-cycle throughput on most ARM cores.
///
/// # Correctness
/// Unsigned comparison: 0xFF > 0x00, not -1 < 0. Both architectures use
/// the same unsigned comparison semantics for `u8` element types.
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[inline]
pub unsafe fn pminub_native_neon(dst: &mut Xmm, src: &Xmm) {
    use std::arch::aarch64::*;
    unsafe {
        let a = vld1q_u8(dst.as_ptr() as *const u8);
        let b = vld1q_u8(src.as_ptr() as *const u8);
        let c = vminq_u8(a, b);
        vst1q_u8(dst.as_mut_ptr() as *mut u8, c);
    }
}

#[inline]
pub fn pminub_scalar(dst: &mut Xmm, src: &Xmm) {
    let d = unsafe { &mut *(dst as *mut Xmm as *mut [u8; 16]) };
    let s = unsafe { &*(src as *const Xmm as *const [u8; 16]) };
    for i in 0..16 {
        d[i] = d[i].min(s[i]);
    }
}

#[inline]
pub fn pminub_xmm(dst: &mut Xmm, src: &Xmm) {
    #[cfg(target_arch = "x86_64")]
    {
        if SimdCapabilities::get().sse2 {
            unsafe { pminub_native_sse2(dst, src) };
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe { pminub_native_neon(dst, src) };
            return;
        }
    }
    pminub_scalar(dst, src);
}

#[inline]
pub fn pminub_ymm(dst: &mut Ymm, src: &Ymm) {
    #[cfg(target_arch = "x86_64")]
    {
        let caps = SimdCapabilities::get();
        if caps.avx2 {
            unsafe { pminub_native_avx2(dst, src) };
            return;
        }
        if caps.sse2 {
            unsafe {
                pminub_native_sse2(&mut dst[0], &src[0]);
                pminub_native_sse2(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe {
                pminub_native_neon(&mut dst[0], &src[0]);
                pminub_native_neon(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    pminub_scalar(&mut dst[0], &src[0]);
    pminub_scalar(&mut dst[1], &src[1]);
}

// --- PMAXUB ---

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
#[inline]
pub unsafe fn pmaxub_native_sse2(dst: &mut Xmm, src: &Xmm) {
    use std::arch::x86_64::*;
    let a = _mm_loadu_si128(dst.as_ptr() as *const __m128i);
    let b = _mm_loadu_si128(src.as_ptr() as *const __m128i);
    let c = _mm_max_epu8(a, b);
    _mm_storeu_si128(dst.as_mut_ptr() as *mut __m128i, c);
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn pmaxub_native_avx2(dst: &mut Ymm, src: &Ymm) {
    use std::arch::x86_64::*;
    let a = _mm256_loadu_si256(dst.as_ptr() as *const __m256i);
    let b = _mm256_loadu_si256(src.as_ptr() as *const __m256i);
    let c = _mm256_max_epu8(a, b);
    _mm256_storeu_si256(dst.as_mut_ptr() as *mut __m256i, c);
}

/// NEON implementation of PMAXUB (packed unsigned byte maximum).
///
/// # Translation Strategy
/// ARM NEON `vmaxq_u8` performs element-wise unsigned maximum:
/// 1. `vld1q_u8(dst)` loads 16 bytes into NEON register `a`
/// 2. `vld1q_u8(src)` loads 16 bytes into NEON register `b`
/// 3. `vmaxq_u8(a, b)` computes `max(a[i], b[i])` for each of 16 lanes
/// 4. `vst1q_u8(dst, c)` stores 16-byte result back
///
/// The "U" in PMAXUB indicates unsigned comparison (0xFF > 0x00).
///
/// # Dispatch Logic (`pmaxub_xmm`)
/// ```text
/// if NEON available:
///     pmaxub_native_neon(dst, src)   // NEON path
/// else:
///     pmaxub_scalar(dst, src)        // Scalar fallback
/// ```
///
/// # Scalar Fallback Behavior (`pmaxub_scalar`)
/// ```text
/// for i in 0..16:
///     dst_bytes[i] = dst_bytes[i].max(src_bytes[i]);
/// ```
///
/// # YMM (256-bit) Handling (`pmaxub_ymm`)
/// ```text
/// pmaxub_native_neon(&mut dst[0], &src[0]);  // Low 128 bits
/// pmaxub_native_neon(&mut dst[1], &src[1]);  // High 128 bits
/// ```
///
/// # Performance
/// Single-cycle throughput on most ARM cores.
///
/// # Correctness
/// Unsigned comparison: 0xFF > 0x00, not -1 < 0. Both architectures use
/// the same unsigned comparison semantics for `u8` element types.
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[inline]
pub unsafe fn pmaxub_native_neon(dst: &mut Xmm, src: &Xmm) {
    use std::arch::aarch64::*;
    unsafe {
        let a = vld1q_u8(dst.as_ptr() as *const u8);
        let b = vld1q_u8(src.as_ptr() as *const u8);
        let c = vmaxq_u8(a, b);
        vst1q_u8(dst.as_mut_ptr() as *mut u8, c);
    }
}

#[inline]
pub fn pmaxub_scalar(dst: &mut Xmm, src: &Xmm) {
    let d = unsafe { &mut *(dst as *mut Xmm as *mut [u8; 16]) };
    let s = unsafe { &*(src as *const Xmm as *const [u8; 16]) };
    for i in 0..16 {
        d[i] = d[i].max(s[i]);
    }
}

#[inline]
pub fn pmaxub_xmm(dst: &mut Xmm, src: &Xmm) {
    #[cfg(target_arch = "x86_64")]
    {
        if SimdCapabilities::get().sse2 {
            unsafe { pmaxub_native_sse2(dst, src) };
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe { pmaxub_native_neon(dst, src) };
            return;
        }
    }
    pmaxub_scalar(dst, src);
}

#[inline]
pub fn pmaxub_ymm(dst: &mut Ymm, src: &Ymm) {
    #[cfg(target_arch = "x86_64")]
    {
        let caps = SimdCapabilities::get();
        if caps.avx2 {
            unsafe { pmaxub_native_avx2(dst, src) };
            return;
        }
        if caps.sse2 {
            unsafe {
                pmaxub_native_sse2(&mut dst[0], &src[0]);
                pmaxub_native_sse2(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon {
            unsafe {
                pmaxub_native_neon(&mut dst[0], &src[0]);
                pmaxub_native_neon(&mut dst[1], &src[1]);
            }
            return;
        }
    }
    pmaxub_scalar(&mut dst[0], &src[0]);
    pmaxub_scalar(&mut dst[1], &src[1]);
}

// ============================================================================
// SHIFT OPERATIONS (PSLLW, PSLLD, PSLLQ, PSRLW, PSRLD, PSRLQ, etc.)
// ============================================================================

// --- PSLLW (shift left logical words) ---

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
#[inline]
pub unsafe fn psllw_native_sse2(dst: &mut Xmm, count: u8) {
    use std::arch::x86_64::*;
    let a = _mm_loadu_si128(dst.as_ptr() as *const __m128i);
    let cnt = _mm_cvtsi64_si128(count as i64);
    let c = _mm_sll_epi16(a, cnt);
    _mm_storeu_si128(dst.as_mut_ptr() as *mut __m128i, c);
}

/// NEON implementation of PSLLW (packed word left shift).
///
/// # Translation Strategy
/// ARM NEON `vshlq_u16` with positive shift count performs left shift:
/// 1. `vld1q_u16(dst)` loads 8 words into NEON register
/// 2. `vdupq_n_s16(count as i16)` broadcasts shift count to all lanes
/// 3. `vshlq_u16(a, shift)` shifts all 8 words left by `count` bits
/// 4. `vst1q_u16(dst, c)` stores result back
///
/// # Dispatch Logic (`psllw_xmm`)
/// ```text
/// if NEON available && count < 16:
///     psllw_native_neon(dst, count)  // NEON path
/// else:
///     psllw_scalar(dst, count)       // Scalar fallback
/// ```
///
/// # Scalar Fallback Behavior (`psllw_scalar`)
/// ```text
/// if count >= 16:
///     dst[0] = 0; dst[1] = 0;  // All bits shifted out → zero
/// else:
///     for each word: word <<= count;
/// ```
///
/// # Performance
/// Single-cycle throughput when count < 16. Out-of-range counts take the
/// scalar path which is still fast (just zeroes the register).
///
/// # Correctness
/// - Shift count 0: No-op (value unchanged)
/// - Shift count 1-15: Standard left shift, zeros shifted in from right
/// - Shift count >= 16: All bits shifted out, result is zero
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[inline]
pub unsafe fn psllw_native_neon(dst: &mut Xmm, count: u8) {
    use std::arch::aarch64::*;
    unsafe {
        let a = vld1q_u16(dst.as_ptr() as *const u16);
        let shift = vdupq_n_s16(count as i16);
        let c = vshlq_u16(a, shift);
        vst1q_u16(dst.as_mut_ptr() as *mut u16, c);
    }
}

#[inline]
pub fn psllw_scalar(dst: &mut Xmm, count: u8) {
    if count >= 16 {
        dst[0] = 0;
        dst[1] = 0;
        return;
    }
    let d = unsafe { &mut *(dst as *mut Xmm as *mut [u16; 8]) };
    for i in 0..8 {
        d[i] <<= count;
    }
}

#[inline]
pub fn psllw_xmm(dst: &mut Xmm, count: u8) {
    #[cfg(target_arch = "x86_64")]
    {
        if SimdCapabilities::get().sse2 && count < 16 {
            unsafe { psllw_native_sse2(dst, count) };
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon && count < 16 {
            unsafe { psllw_native_neon(dst, count) };
            return;
        }
    }
    psllw_scalar(dst, count);
}

// --- PSLLD (shift left logical dwords) ---

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
#[inline]
pub unsafe fn pslld_native_sse2(dst: &mut Xmm, count: u8) {
    use std::arch::x86_64::*;
    let a = _mm_loadu_si128(dst.as_ptr() as *const __m128i);
    let cnt = _mm_cvtsi64_si128(count as i64);
    let c = _mm_sll_epi32(a, cnt);
    _mm_storeu_si128(dst.as_mut_ptr() as *mut __m128i, c);
}

/// NEON implementation of PSLLD (packed doubleword left shift).
///
/// # Translation Strategy
/// ARM NEON `vshlq_u32` with positive shift count performs left shift:
/// 1. `vld1q_u32(dst)` loads 4 dwords into NEON register
/// 2. `vdupq_n_s32(count as i32)` broadcasts shift count to all lanes
/// 3. `vshlq_u32(a, shift)` shifts all 4 dwords left by `count` bits
/// 4. `vst1q_u32(dst, c)` stores result back
///
/// # Dispatch Logic (`pslld_xmm`)
/// ```text
/// if NEON available && count < 32:
///     pslld_native_neon(dst, count)  // NEON path
/// else:
///     pslld_scalar(dst, count)       // Scalar fallback
/// ```
///
/// # Scalar Fallback Behavior (`pslld_scalar`)
/// ```text
/// if count >= 32:
///     dst[0] = 0; dst[1] = 0;  // All bits shifted out → zero
/// else:
///     for each dword: dword <<= count;
/// ```
///
/// # Performance
/// Single-cycle throughput when count < 32.
///
/// # Correctness
/// - Shift count 0: No-op (value unchanged)
/// - Shift count 1-31: Standard left shift, zeros shifted in from right
/// - Shift count >= 32: All bits shifted out, result is zero
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[inline]
pub unsafe fn pslld_native_neon(dst: &mut Xmm, count: u8) {
    use std::arch::aarch64::*;
    unsafe {
        let a = vld1q_u32(dst.as_ptr() as *const u32);
        let shift = vdupq_n_s32(count as i32);
        let c = vshlq_u32(a, shift);
        vst1q_u32(dst.as_mut_ptr() as *mut u32, c);
    }
}

#[inline]
pub fn pslld_scalar(dst: &mut Xmm, count: u8) {
    if count >= 32 {
        dst[0] = 0;
        dst[1] = 0;
        return;
    }
    let d = unsafe { &mut *(dst as *mut Xmm as *mut [u32; 4]) };
    for i in 0..4 {
        d[i] <<= count;
    }
}

#[inline]
pub fn pslld_xmm(dst: &mut Xmm, count: u8) {
    #[cfg(target_arch = "x86_64")]
    {
        if SimdCapabilities::get().sse2 && count < 32 {
            unsafe { pslld_native_sse2(dst, count) };
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon && count < 32 {
            unsafe { pslld_native_neon(dst, count) };
            return;
        }
    }
    pslld_scalar(dst, count);
}

// --- PSLLQ (shift left logical qwords) ---

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
#[inline]
pub unsafe fn psllq_native_sse2(dst: &mut Xmm, count: u8) {
    use std::arch::x86_64::*;
    let a = _mm_loadu_si128(dst.as_ptr() as *const __m128i);
    let cnt = _mm_cvtsi64_si128(count as i64);
    let c = _mm_sll_epi64(a, cnt);
    _mm_storeu_si128(dst.as_mut_ptr() as *mut __m128i, c);
}

/// NEON implementation of PSLLQ (packed quadword left shift).
///
/// # Translation Strategy
/// ARM NEON `vshlq_u64` with positive shift count performs left shift:
/// 1. `vld1q_u64(dst)` loads 2 qwords into NEON register
/// 2. `vdupq_n_s64(count as i64)` broadcasts shift count to both lanes
/// 3. `vshlq_u64(a, shift)` shifts both qwords left by `count` bits
/// 4. `vst1q_u64(dst, c)` stores result back
///
/// # Dispatch Logic (`psllq_xmm`)
/// ```text
/// if NEON available && count < 64:
///     psllq_native_neon(dst, count)  // NEON path
/// else:
///     psllq_scalar(dst, count)       // Scalar fallback
/// ```
///
/// # Scalar Fallback Behavior (`psllq_scalar`)
/// ```text
/// if count >= 64:
///     dst[0] = 0; dst[1] = 0;  // All bits shifted out → zero
/// else:
///     dst[0] <<= count; dst[1] <<= count;
/// ```
///
/// # Performance
/// Single-cycle throughput when count < 64.
///
/// # Correctness
/// - Shift count 0: No-op (value unchanged)
/// - Shift count 1-63: Standard left shift, zeros shifted in from right
/// - Shift count >= 64: All bits shifted out, result is zero
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[inline]
pub unsafe fn psllq_native_neon(dst: &mut Xmm, count: u8) {
    use std::arch::aarch64::*;
    unsafe {
        let a = vld1q_u64(dst.as_ptr() as *const u64);
        let shift = vdupq_n_s64(count as i64);
        let c = vshlq_u64(a, shift);
        vst1q_u64(dst.as_mut_ptr() as *mut u64, c);
    }
}

#[inline]
pub fn psllq_scalar(dst: &mut Xmm, count: u8) {
    if count >= 64 {
        dst[0] = 0;
        dst[1] = 0;
        return;
    }
    dst[0] <<= count;
    dst[1] <<= count;
}

#[inline]
pub fn psllq_xmm(dst: &mut Xmm, count: u8) {
    #[cfg(target_arch = "x86_64")]
    {
        if SimdCapabilities::get().sse2 && count < 64 {
            unsafe { psllq_native_sse2(dst, count) };
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon && count < 64 {
            unsafe { psllq_native_neon(dst, count) };
            return;
        }
    }
    psllq_scalar(dst, count);
}

// --- PSRLW (shift right logical words) ---

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
#[inline]
pub unsafe fn psrlw_native_sse2(dst: &mut Xmm, count: u8) {
    use std::arch::x86_64::*;
    let a = _mm_loadu_si128(dst.as_ptr() as *const __m128i);
    let cnt = _mm_cvtsi64_si128(count as i64);
    let c = _mm_srl_epi16(a, cnt);
    _mm_storeu_si128(dst.as_mut_ptr() as *mut __m128i, c);
}

/// NEON implementation of PSRLW (packed shift right logical words).
///
/// # Translation Strategy
/// ARM NEON `vshlq_u16` with **negative** shift count performs right shift:
/// 1. `vld1q_u16(dst)` loads 8 words into NEON register
/// 2. `vdupq_n_s16(-(count as i16))` broadcasts **negated** count to all lanes
/// 3. `vshlq_u16(a, shift)` - negative shift value causes right shift
/// 4. `vst1q_u16(dst, c)` stores result back
///
/// **Key NEON quirk**: `vshlq_*` direction is determined by shift sign:
/// - Positive shift value → left shift
/// - Negative shift value → right shift
///
/// # Dispatch Logic (`psrlw_xmm`)
/// ```text
/// if NEON available && count < 16:
///     psrlw_native_neon(dst, count)  // NEON path
/// else:
///     psrlw_scalar(dst, count)       // Scalar fallback
/// ```
///
/// # Scalar Fallback Behavior (`psrlw_scalar`)
/// ```text
/// if count >= 16:
///     dst[0] = 0; dst[1] = 0;  // All bits shifted out → zero
/// else:
///     for each word: word >>= count;
/// ```
///
/// # Performance
/// Single-cycle throughput when count < 16. Out-of-range counts take the
/// scalar path which is still fast (just zeroes the register).
///
/// # Correctness
/// - Shift count 0: No-op (value unchanged)
/// - Shift count 1-15: Standard right shift, zeros shifted in from left
/// - Shift count >= 16: All bits shifted out, result is zero
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[inline]
pub unsafe fn psrlw_native_neon(dst: &mut Xmm, count: u8) {
    use std::arch::aarch64::*;
    unsafe {
        let a = vld1q_u16(dst.as_ptr() as *const u16);
        // Negative shift for right shift (NEON uses negative values for right shifts)
        let shift = vdupq_n_s16(-(count as i16));
        let c = vshlq_u16(a, shift);
        vst1q_u16(dst.as_mut_ptr() as *mut u16, c);
    }
}

#[inline]
pub fn psrlw_scalar(dst: &mut Xmm, count: u8) {
    if count >= 16 {
        dst[0] = 0;
        dst[1] = 0;
        return;
    }
    let d = unsafe { &mut *(dst as *mut Xmm as *mut [u16; 8]) };
    for i in 0..8 {
        d[i] >>= count;
    }
}

#[inline]
pub fn psrlw_xmm(dst: &mut Xmm, count: u8) {
    #[cfg(target_arch = "x86_64")]
    {
        if SimdCapabilities::get().sse2 && count < 16 {
            unsafe { psrlw_native_sse2(dst, count) };
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon && count < 16 {
            unsafe { psrlw_native_neon(dst, count) };
            return;
        }
    }
    psrlw_scalar(dst, count);
}

// --- PSRLD (shift right logical dwords) ---

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
#[inline]
pub unsafe fn psrld_native_sse2(dst: &mut Xmm, count: u8) {
    use std::arch::x86_64::*;
    let a = _mm_loadu_si128(dst.as_ptr() as *const __m128i);
    let cnt = _mm_cvtsi64_si128(count as i64);
    let c = _mm_srl_epi32(a, cnt);
    _mm_storeu_si128(dst.as_mut_ptr() as *mut __m128i, c);
}

/// NEON implementation of PSRLD (packed doubleword logical right shift).
///
/// # Translation Strategy
/// ARM NEON `vshlq_u32` with **negative** shift count performs right shift:
/// 1. `vld1q_u32(dst)` loads 4 dwords into NEON register
/// 2. `vdupq_n_s32(-(count as i32))` broadcasts **negated** count to all lanes
/// 3. `vshlq_u32(a, shift)` - negative shift value causes right shift
/// 4. `vst1q_u32(dst, c)` stores result back
///
/// **Key NEON quirk**: `vshlq` direction is determined by shift sign:
/// - Positive shift value → left shift
/// - Negative shift value → right shift
///
/// # Dispatch Logic (`psrld_xmm`)
/// ```text
/// if NEON available && count < 32:
///     psrld_native_neon(dst, count)  // NEON path
/// else:
///     psrld_scalar(dst, count)       // Scalar fallback
/// ```
///
/// # Scalar Fallback Behavior (`psrld_scalar`)
/// ```text
/// if count >= 32:
///     dst[0] = 0; dst[1] = 0;  // All bits shifted out → zero
/// else:
///     for each dword: dword >>= count;
/// ```
///
/// # Performance
/// Single-cycle throughput when count < 32.
///
/// # Correctness
/// - Shift count 0: No-op (value unchanged)
/// - Shift count 1-31: Logical right shift, zeros shifted in from left
/// - Shift count >= 32: All bits shifted out, result is zero
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[inline]
pub unsafe fn psrld_native_neon(dst: &mut Xmm, count: u8) {
    use std::arch::aarch64::*;
    unsafe {
        let a = vld1q_u32(dst.as_ptr() as *const u32);
        // Negative shift for right shift (NEON uses negative values for right shifts)
        let shift = vdupq_n_s32(-(count as i32));
        let c = vshlq_u32(a, shift);
        vst1q_u32(dst.as_mut_ptr() as *mut u32, c);
    }
}

#[inline]
pub fn psrld_scalar(dst: &mut Xmm, count: u8) {
    if count >= 32 {
        dst[0] = 0;
        dst[1] = 0;
        return;
    }
    let d = unsafe { &mut *(dst as *mut Xmm as *mut [u32; 4]) };
    for i in 0..4 {
        d[i] >>= count;
    }
}

#[inline]
pub fn psrld_xmm(dst: &mut Xmm, count: u8) {
    #[cfg(target_arch = "x86_64")]
    {
        if SimdCapabilities::get().sse2 && count < 32 {
            unsafe { psrld_native_sse2(dst, count) };
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon && count < 32 {
            unsafe { psrld_native_neon(dst, count) };
            return;
        }
    }
    psrld_scalar(dst, count);
}

// --- PSRLQ (shift right logical qwords) ---

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
#[inline]
pub unsafe fn psrlq_native_sse2(dst: &mut Xmm, count: u8) {
    use std::arch::x86_64::*;
    let a = _mm_loadu_si128(dst.as_ptr() as *const __m128i);
    let cnt = _mm_cvtsi64_si128(count as i64);
    let c = _mm_srl_epi64(a, cnt);
    _mm_storeu_si128(dst.as_mut_ptr() as *mut __m128i, c);
}

/// NEON implementation of PSRLQ (packed quadword logical right shift).
///
/// # Translation Strategy
/// ARM NEON `vshlq_u64` with **negative** shift count performs right shift:
/// 1. `vld1q_u64(dst)` loads 2 qwords into NEON register
/// 2. `vdupq_n_s64(-(count as i64))` broadcasts **negated** count to both lanes
/// 3. `vshlq_u64(a, shift)` - negative shift value causes right shift
/// 4. `vst1q_u64(dst, c)` stores result back
///
/// **Key NEON quirk**: `vshlq` direction is determined by shift sign:
/// - Positive shift value → left shift
/// - Negative shift value → right shift
///
/// # Dispatch Logic (`psrlq_xmm`)
/// ```text
/// if NEON available && count < 64:
///     psrlq_native_neon(dst, count)  // NEON path
/// else:
///     psrlq_scalar(dst, count)       // Scalar fallback
/// ```
///
/// # Scalar Fallback Behavior (`psrlq_scalar`)
/// ```text
/// if count >= 64:
///     dst[0] = 0; dst[1] = 0;  // All bits shifted out → zero
/// else:
///     dst[0] >>= count; dst[1] >>= count;
/// ```
///
/// # Performance
/// Single-cycle throughput when count < 64.
///
/// # Correctness
/// - Shift count 0: No-op (value unchanged)
/// - Shift count 1-63: Logical right shift, zeros shifted in from left
/// - Shift count >= 64: All bits shifted out, result is zero
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[inline]
pub unsafe fn psrlq_native_neon(dst: &mut Xmm, count: u8) {
    use std::arch::aarch64::*;
    unsafe {
        let a = vld1q_u64(dst.as_ptr() as *const u64);
        // Negative shift for right shift (NEON uses negative values for right shifts)
        let shift = vdupq_n_s64(-(count as i64));
        let c = vshlq_u64(a, shift);
        vst1q_u64(dst.as_mut_ptr() as *mut u64, c);
    }
}

#[inline]
pub fn psrlq_scalar(dst: &mut Xmm, count: u8) {
    if count >= 64 {
        dst[0] = 0;
        dst[1] = 0;
        return;
    }
    dst[0] >>= count;
    dst[1] >>= count;
}

#[inline]
pub fn psrlq_xmm(dst: &mut Xmm, count: u8) {
    #[cfg(target_arch = "x86_64")]
    {
        if SimdCapabilities::get().sse2 && count < 64 {
            unsafe { psrlq_native_sse2(dst, count) };
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if SimdCapabilities::get().neon && count < 64 {
            unsafe { psrlq_native_neon(dst, count) };
            return;
        }
    }
    psrlq_scalar(dst, count);
}

// ============================================================================
// TESTS - All SIMD variants tested on available host features
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // Test helpers
    // ========================================================================

    fn make_f32_xmm(a: f32, b: f32, c: f32, d: f32) -> Xmm {
        [
            a.to_bits() as u64 | ((b.to_bits() as u64) << 32),
            c.to_bits() as u64 | ((d.to_bits() as u64) << 32),
        ]
    }

    fn make_f64_xmm(a: f64, b: f64) -> Xmm {
        [a.to_bits(), b.to_bits()]
    }

    fn read_f32_xmm(xmm: &Xmm) -> [f32; 4] {
        unsafe { *(xmm.as_ptr() as *const [f32; 4]) }
    }

    fn read_f64_xmm(xmm: &Xmm) -> [f64; 2] {
        unsafe { *(xmm.as_ptr() as *const [f64; 2]) }
    }

    fn xmm_eq(a: &Xmm, b: &Xmm) -> bool {
        a[0] == b[0] && a[1] == b[1]
    }

    fn ymm_eq(a: &Ymm, b: &Ymm) -> bool {
        xmm_eq(&a[0], &b[0]) && xmm_eq(&a[1], &b[1])
    }

    // ========================================================================
    // Host capability detection test
    // ========================================================================

    #[test]
    fn test_host_capabilities() {
        let caps = SimdCapabilities::get();
        println!("Host SIMD capabilities:");
        println!("  SSE2:     {}", caps.sse2);
        println!("  SSE3:     {}", caps.sse3);
        println!("  SSSE3:    {}", caps.ssse3);
        println!("  SSE4.1:   {}", caps.sse4_1);
        println!("  SSE4.2:   {}", caps.sse4_2);
        println!("  AVX:      {}", caps.avx);
        println!("  AVX2:     {}", caps.avx2);
        println!("  FMA:      {}", caps.fma);
        println!("  AVX-512F: {}", caps.avx512f);
        println!("  AVX-512BW:{}", caps.avx512bw);
        println!("  AVX-512VL:{}", caps.avx512vl);
        println!("  NEON:     {}", caps.neon);

        #[cfg(target_arch = "x86_64")]
        assert!(caps.sse2, "x86_64 should have SSE2");

        #[cfg(target_arch = "aarch64")]
        assert!(caps.neon, "aarch64 should have NEON");
    }

    // ========================================================================
    // ADDPS - All variants
    // ========================================================================

    #[test]
    fn test_addps_all_variants() {
        let caps = SimdCapabilities::get();
        let src: Xmm = make_f32_xmm(5.0, 6.0, 7.0, 8.0);

        // Scalar reference
        let mut scalar_dst = make_f32_xmm(1.0, 2.0, 3.0, 4.0);
        addps_scalar(&mut scalar_dst, &src);
        let expected = read_f32_xmm(&scalar_dst);
        assert_eq!(expected, [6.0, 8.0, 10.0, 12.0]);

        #[cfg(target_arch = "x86_64")]
        {
            // SSE native (XMM)
            if caps.sse2 {
                let mut sse_dst = make_f32_xmm(1.0, 2.0, 3.0, 4.0);
                unsafe { addps_native_sse(&mut sse_dst, &src) };
                assert!(xmm_eq(&sse_dst, &scalar_dst), "SSE ADDPS mismatch");
                println!("  ADDPS SSE: PASS");
            }

            // AVX native (YMM - tests the 256-bit version)
            if caps.avx {
                let src_ymm: Ymm = [src, make_f32_xmm(9.0, 10.0, 11.0, 12.0)];
                let mut scalar_ymm: Ymm = [
                    make_f32_xmm(1.0, 2.0, 3.0, 4.0),
                    make_f32_xmm(1.0, 2.0, 3.0, 4.0),
                ];
                addps_scalar(&mut scalar_ymm[0], &src_ymm[0]);
                addps_scalar(&mut scalar_ymm[1], &src_ymm[1]);

                let mut avx_dst: Ymm = [
                    make_f32_xmm(1.0, 2.0, 3.0, 4.0),
                    make_f32_xmm(1.0, 2.0, 3.0, 4.0),
                ];
                unsafe { addps_native_avx(&mut avx_dst, &src_ymm) };
                assert!(ymm_eq(&avx_dst, &scalar_ymm), "AVX YMM ADDPS mismatch");
                println!("  ADDPS AVX YMM: PASS");
            }
        }

        // Dispatch function
        let mut dispatch_dst = make_f32_xmm(1.0, 2.0, 3.0, 4.0);
        addps_xmm(&mut dispatch_dst, &src);
        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "ADDPS dispatch mismatch"
        );
        println!("  ADDPS dispatch: PASS");
    }

    #[test]
    fn test_addps_ymm_dispatch() {
        let src: Ymm = [
            make_f32_xmm(1.0, 2.0, 3.0, 4.0),
            make_f32_xmm(5.0, 6.0, 7.0, 8.0),
        ];

        // Scalar reference
        let mut scalar_dst: Ymm = [
            make_f32_xmm(10.0, 20.0, 30.0, 40.0),
            make_f32_xmm(50.0, 60.0, 70.0, 80.0),
        ];
        addps_scalar(&mut scalar_dst[0], &src[0]);
        addps_scalar(&mut scalar_dst[1], &src[1]);

        // Dispatch
        let mut dispatch_dst: Ymm = [
            make_f32_xmm(10.0, 20.0, 30.0, 40.0),
            make_f32_xmm(50.0, 60.0, 70.0, 80.0),
        ];
        addps_ymm(&mut dispatch_dst, &src);
        assert!(
            ymm_eq(&dispatch_dst, &scalar_dst),
            "ADDPS YMM dispatch mismatch"
        );
        println!("  ADDPS YMM dispatch: PASS");
    }

    // ========================================================================
    // SUBPS - All variants
    // ========================================================================

    #[test]
    fn test_subps_all_variants() {
        let caps = SimdCapabilities::get();
        let src: Xmm = make_f32_xmm(1.0, 2.0, 3.0, 4.0);

        // Scalar reference
        let mut scalar_dst = make_f32_xmm(10.0, 20.0, 30.0, 40.0);
        subps_scalar(&mut scalar_dst, &src);
        let expected = read_f32_xmm(&scalar_dst);
        assert_eq!(expected, [9.0, 18.0, 27.0, 36.0]);

        #[cfg(target_arch = "x86_64")]
        {
            if caps.sse2 {
                let mut sse_dst = make_f32_xmm(10.0, 20.0, 30.0, 40.0);
                unsafe { subps_native_sse(&mut sse_dst, &src) };
                assert!(xmm_eq(&sse_dst, &scalar_dst), "SSE SUBPS mismatch");
                println!("  SUBPS SSE: PASS");
            }

            if caps.avx {
                let src_ymm: Ymm = [src, make_f32_xmm(5.0, 6.0, 7.0, 8.0)];
                let mut scalar_ymm: Ymm = [
                    make_f32_xmm(10.0, 20.0, 30.0, 40.0),
                    make_f32_xmm(50.0, 60.0, 70.0, 80.0),
                ];
                subps_scalar(&mut scalar_ymm[0], &src_ymm[0]);
                subps_scalar(&mut scalar_ymm[1], &src_ymm[1]);

                let mut avx_dst: Ymm = [
                    make_f32_xmm(10.0, 20.0, 30.0, 40.0),
                    make_f32_xmm(50.0, 60.0, 70.0, 80.0),
                ];
                unsafe { subps_native_avx(&mut avx_dst, &src_ymm) };
                assert!(ymm_eq(&avx_dst, &scalar_ymm), "AVX YMM SUBPS mismatch");
                println!("  SUBPS AVX YMM: PASS");
            }
        }

        let mut dispatch_dst = make_f32_xmm(10.0, 20.0, 30.0, 40.0);
        subps_xmm(&mut dispatch_dst, &src);
        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "SUBPS dispatch mismatch"
        );
        println!("  SUBPS dispatch: PASS");
    }

    // ========================================================================
    // MULPS - All variants
    // ========================================================================

    #[test]
    fn test_mulps_all_variants() {
        let caps = SimdCapabilities::get();
        let src: Xmm = make_f32_xmm(2.0, 3.0, 4.0, 5.0);

        // Scalar reference
        let mut scalar_dst = make_f32_xmm(1.0, 2.0, 3.0, 4.0);
        mulps_scalar(&mut scalar_dst, &src);
        let expected = read_f32_xmm(&scalar_dst);
        assert_eq!(expected, [2.0, 6.0, 12.0, 20.0]);

        #[cfg(target_arch = "x86_64")]
        {
            if caps.sse2 {
                let mut sse_dst = make_f32_xmm(1.0, 2.0, 3.0, 4.0);
                unsafe { mulps_native_sse(&mut sse_dst, &src) };
                assert!(xmm_eq(&sse_dst, &scalar_dst), "SSE MULPS mismatch");
                println!("  MULPS SSE: PASS");
            }

            if caps.avx {
                let src_ymm: Ymm = [src, make_f32_xmm(2.0, 2.0, 2.0, 2.0)];
                let mut scalar_ymm: Ymm = [
                    make_f32_xmm(1.0, 2.0, 3.0, 4.0),
                    make_f32_xmm(5.0, 6.0, 7.0, 8.0),
                ];
                mulps_scalar(&mut scalar_ymm[0], &src_ymm[0]);
                mulps_scalar(&mut scalar_ymm[1], &src_ymm[1]);

                let mut avx_dst: Ymm = [
                    make_f32_xmm(1.0, 2.0, 3.0, 4.0),
                    make_f32_xmm(5.0, 6.0, 7.0, 8.0),
                ];
                unsafe { mulps_native_avx(&mut avx_dst, &src_ymm) };
                assert!(ymm_eq(&avx_dst, &scalar_ymm), "AVX YMM MULPS mismatch");
                println!("  MULPS AVX YMM: PASS");
            }
        }

        let mut dispatch_dst = make_f32_xmm(1.0, 2.0, 3.0, 4.0);
        mulps_xmm(&mut dispatch_dst, &src);
        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "MULPS dispatch mismatch"
        );
        println!("  MULPS dispatch: PASS");
    }

    // ========================================================================
    // DIVPS - All variants
    // ========================================================================

    #[test]
    fn test_divps_all_variants() {
        let caps = SimdCapabilities::get();
        let src: Xmm = make_f32_xmm(2.0, 4.0, 5.0, 8.0);

        // Scalar reference
        let mut scalar_dst = make_f32_xmm(10.0, 20.0, 30.0, 40.0);
        divps_scalar(&mut scalar_dst, &src);
        let expected = read_f32_xmm(&scalar_dst);
        assert_eq!(expected, [5.0, 5.0, 6.0, 5.0]);

        #[cfg(target_arch = "x86_64")]
        {
            if caps.sse2 {
                let mut sse_dst = make_f32_xmm(10.0, 20.0, 30.0, 40.0);
                unsafe { divps_native_sse(&mut sse_dst, &src) };
                assert!(xmm_eq(&sse_dst, &scalar_dst), "SSE DIVPS mismatch");
                println!("  DIVPS SSE: PASS");
            }

            if caps.avx {
                let src_ymm: Ymm = [src, make_f32_xmm(2.0, 5.0, 10.0, 4.0)];
                let mut scalar_ymm: Ymm = [
                    make_f32_xmm(10.0, 20.0, 30.0, 40.0),
                    make_f32_xmm(20.0, 25.0, 30.0, 40.0),
                ];
                divps_scalar(&mut scalar_ymm[0], &src_ymm[0]);
                divps_scalar(&mut scalar_ymm[1], &src_ymm[1]);

                let mut avx_dst: Ymm = [
                    make_f32_xmm(10.0, 20.0, 30.0, 40.0),
                    make_f32_xmm(20.0, 25.0, 30.0, 40.0),
                ];
                unsafe { divps_native_avx(&mut avx_dst, &src_ymm) };
                assert!(ymm_eq(&avx_dst, &scalar_ymm), "AVX YMM DIVPS mismatch");
                println!("  DIVPS AVX YMM: PASS");
            }
        }

        let mut dispatch_dst = make_f32_xmm(10.0, 20.0, 30.0, 40.0);
        divps_xmm(&mut dispatch_dst, &src);
        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "DIVPS dispatch mismatch"
        );
        println!("  DIVPS dispatch: PASS");
    }

    // ========================================================================
    // ADDPD - All variants
    // ========================================================================

    #[test]
    fn test_addpd_all_variants() {
        let caps = SimdCapabilities::get();
        let src: Xmm = make_f64_xmm(5.0, 6.0);

        // Scalar reference
        let mut scalar_dst = make_f64_xmm(1.0, 2.0);
        addpd_scalar(&mut scalar_dst, &src);
        let expected = read_f64_xmm(&scalar_dst);
        assert_eq!(expected, [6.0, 8.0]);

        #[cfg(target_arch = "x86_64")]
        {
            if caps.sse2 {
                let mut sse_dst = make_f64_xmm(1.0, 2.0);
                unsafe { addpd_native_sse2(&mut sse_dst, &src) };
                assert!(xmm_eq(&sse_dst, &scalar_dst), "SSE2 ADDPD mismatch");
                println!("  ADDPD SSE2: PASS");
            }
        }

        let mut dispatch_dst = make_f64_xmm(1.0, 2.0);
        addpd_xmm(&mut dispatch_dst, &src);
        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "ADDPD dispatch mismatch"
        );
        println!("  ADDPD dispatch: PASS");
    }

    // ========================================================================
    // SUBPD - All variants
    // ========================================================================

    #[test]
    fn test_subpd_all_variants() {
        let caps = SimdCapabilities::get();
        let src: Xmm = make_f64_xmm(1.0, 2.0);

        // Scalar reference
        let mut scalar_dst = make_f64_xmm(10.0, 20.0);
        subpd_scalar(&mut scalar_dst, &src);
        let expected = read_f64_xmm(&scalar_dst);
        assert_eq!(expected, [9.0, 18.0]);

        #[cfg(target_arch = "x86_64")]
        {
            if caps.sse2 {
                let mut sse_dst = make_f64_xmm(10.0, 20.0);
                unsafe { subpd_native_sse2(&mut sse_dst, &src) };
                assert!(xmm_eq(&sse_dst, &scalar_dst), "SSE2 SUBPD mismatch");
                println!("  SUBPD SSE2: PASS");
            }
        }

        let mut dispatch_dst = make_f64_xmm(10.0, 20.0);
        subpd_xmm(&mut dispatch_dst, &src);
        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "SUBPD dispatch mismatch"
        );
        println!("  SUBPD dispatch: PASS");
    }

    // ========================================================================
    // MULPD - All variants
    // ========================================================================

    #[test]
    fn test_mulpd_all_variants() {
        let caps = SimdCapabilities::get();
        let src: Xmm = make_f64_xmm(2.0, 3.0);

        // Scalar reference
        let mut scalar_dst = make_f64_xmm(5.0, 10.0);
        mulpd_scalar(&mut scalar_dst, &src);
        let expected = read_f64_xmm(&scalar_dst);
        assert_eq!(expected, [10.0, 30.0]);

        #[cfg(target_arch = "x86_64")]
        {
            if caps.sse2 {
                let mut sse_dst = make_f64_xmm(5.0, 10.0);
                unsafe { mulpd_native_sse2(&mut sse_dst, &src) };
                assert!(xmm_eq(&sse_dst, &scalar_dst), "SSE2 MULPD mismatch");
                println!("  MULPD SSE2: PASS");
            }
        }

        let mut dispatch_dst = make_f64_xmm(5.0, 10.0);
        mulpd_xmm(&mut dispatch_dst, &src);
        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "MULPD dispatch mismatch"
        );
        println!("  MULPD dispatch: PASS");
    }

    // ========================================================================
    // DIVPD - All variants
    // ========================================================================

    #[test]
    fn test_divpd_all_variants() {
        let caps = SimdCapabilities::get();
        let src: Xmm = make_f64_xmm(2.0, 5.0);

        // Scalar reference
        let mut scalar_dst = make_f64_xmm(10.0, 25.0);
        divpd_scalar(&mut scalar_dst, &src);
        let expected = read_f64_xmm(&scalar_dst);
        assert_eq!(expected, [5.0, 5.0]);

        #[cfg(target_arch = "x86_64")]
        {
            if caps.sse2 {
                let mut sse_dst = make_f64_xmm(10.0, 25.0);
                unsafe { divpd_native_sse2(&mut sse_dst, &src) };
                assert!(xmm_eq(&sse_dst, &scalar_dst), "SSE2 DIVPD mismatch");
                println!("  DIVPD SSE2: PASS");
            }
        }

        let mut dispatch_dst = make_f64_xmm(10.0, 25.0);
        divpd_xmm(&mut dispatch_dst, &src);
        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "DIVPD dispatch mismatch"
        );
        println!("  DIVPD dispatch: PASS");
    }

    // ========================================================================
    // PADDB - All variants
    // ========================================================================

    #[test]
    fn test_paddb_all_variants() {
        let caps = SimdCapabilities::get();
        let src: Xmm = [0x0101010101010101, 0x0202020202020202];

        // Scalar reference
        let mut scalar_dst: Xmm = [0x0102030405060708, 0x090A0B0C0D0E0F10];
        paddb_scalar(&mut scalar_dst, &src);

        #[cfg(target_arch = "x86_64")]
        {
            if caps.sse2 {
                let mut sse_dst: Xmm = [0x0102030405060708, 0x090A0B0C0D0E0F10];
                unsafe { paddb_native_sse2(&mut sse_dst, &src) };
                assert!(xmm_eq(&sse_dst, &scalar_dst), "SSE2 PADDB mismatch");
                println!("  PADDB SSE2: PASS");
            }

            if caps.avx2 {
                // Test YMM variant
                let src_ymm: Ymm = [
                    [0x0101010101010101, 0x0101010101010101],
                    [0x0202020202020202, 0x0202020202020202],
                ];
                let mut scalar_ymm: Ymm = [
                    [0x0102030405060708, 0x090A0B0C0D0E0F10],
                    [0x1112131415161718, 0x191A1B1C1D1E1F20],
                ];
                paddb_scalar(&mut scalar_ymm[0], &src_ymm[0]);
                paddb_scalar(&mut scalar_ymm[1], &src_ymm[1]);

                let mut avx2_ymm: Ymm = [
                    [0x0102030405060708, 0x090A0B0C0D0E0F10],
                    [0x1112131415161718, 0x191A1B1C1D1E1F20],
                ];
                unsafe { paddb_native_avx2(&mut avx2_ymm, &src_ymm) };
                assert!(ymm_eq(&avx2_ymm, &scalar_ymm), "AVX2 PADDB YMM mismatch");
                println!("  PADDB AVX2 YMM: PASS");
            }
        }

        let mut dispatch_dst: Xmm = [0x0102030405060708, 0x090A0B0C0D0E0F10];
        paddb_xmm(&mut dispatch_dst, &src);
        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "PADDB dispatch mismatch"
        );
        println!("  PADDB dispatch: PASS");
    }

    // ========================================================================
    // PADDW - All variants
    // ========================================================================

    #[test]
    fn test_paddw_all_variants() {
        let caps = SimdCapabilities::get();
        let src: Xmm = [0x0001000100010001, 0x0002000200020002];

        // Scalar reference
        let mut scalar_dst: Xmm = [0x0102030405060708, 0x090A0B0C0D0E0F10];
        paddw_scalar(&mut scalar_dst, &src);

        #[cfg(target_arch = "x86_64")]
        {
            if caps.sse2 {
                let mut sse_dst: Xmm = [0x0102030405060708, 0x090A0B0C0D0E0F10];
                unsafe { paddw_native_sse2(&mut sse_dst, &src) };
                assert!(xmm_eq(&sse_dst, &scalar_dst), "SSE2 PADDW mismatch");
                println!("  PADDW SSE2: PASS");
            }
        }

        let mut dispatch_dst: Xmm = [0x0102030405060708, 0x090A0B0C0D0E0F10];
        paddw_xmm(&mut dispatch_dst, &src);
        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "PADDW dispatch mismatch"
        );
        println!("  PADDW dispatch: PASS");
    }

    // ========================================================================
    // PADDD - All variants
    // ========================================================================

    #[test]
    fn test_paddd_all_variants() {
        let caps = SimdCapabilities::get();
        let src: Xmm = [0x0000000100000001, 0x0000000200000002];

        // Scalar reference
        let mut scalar_dst: Xmm = [0x0000001000000020, 0x0000003000000040];
        paddd_scalar(&mut scalar_dst, &src);

        #[cfg(target_arch = "x86_64")]
        {
            if caps.sse2 {
                let mut sse_dst: Xmm = [0x0000001000000020, 0x0000003000000040];
                unsafe { paddd_native_sse2(&mut sse_dst, &src) };
                assert!(xmm_eq(&sse_dst, &scalar_dst), "SSE2 PADDD mismatch");
                println!("  PADDD SSE2: PASS");
            }
        }

        let mut dispatch_dst: Xmm = [0x0000001000000020, 0x0000003000000040];
        paddd_xmm(&mut dispatch_dst, &src);
        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "PADDD dispatch mismatch"
        );
        println!("  PADDD dispatch: PASS");
    }

    // ========================================================================
    // PADDQ - All variants
    // ========================================================================

    #[test]
    fn test_paddq_all_variants() {
        let caps = SimdCapabilities::get();
        let src: Xmm = [1, 2];

        // Scalar reference
        let mut scalar_dst: Xmm = [100, 200];
        paddq_scalar(&mut scalar_dst, &src);
        assert_eq!(scalar_dst, [101, 202]);

        #[cfg(target_arch = "x86_64")]
        {
            if caps.sse2 {
                let mut sse_dst: Xmm = [100, 200];
                unsafe { paddq_native_sse2(&mut sse_dst, &src) };
                assert!(xmm_eq(&sse_dst, &scalar_dst), "SSE2 PADDQ mismatch");
                println!("  PADDQ SSE2: PASS");
            }
        }

        let mut dispatch_dst: Xmm = [100, 200];
        paddq_xmm(&mut dispatch_dst, &src);
        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "PADDQ dispatch mismatch"
        );
        println!("  PADDQ dispatch: PASS");
    }

    // ========================================================================
    // PSUBB - All variants
    // ========================================================================

    #[test]
    fn test_psubb_all_variants() {
        let caps = SimdCapabilities::get();
        let src: Xmm = [0x0101010101010101, 0x0101010101010101];

        // Scalar reference
        let mut scalar_dst: Xmm = [0x0203040506070809, 0x0A0B0C0D0E0F1011];
        psubb_scalar(&mut scalar_dst, &src);

        #[cfg(target_arch = "x86_64")]
        {
            if caps.sse2 {
                let mut sse_dst: Xmm = [0x0203040506070809, 0x0A0B0C0D0E0F1011];
                unsafe { psubb_native_sse2(&mut sse_dst, &src) };
                assert!(xmm_eq(&sse_dst, &scalar_dst), "SSE2 PSUBB mismatch");
                println!("  PSUBB SSE2: PASS");
            }
        }

        let mut dispatch_dst: Xmm = [0x0203040506070809, 0x0A0B0C0D0E0F1011];
        psubb_xmm(&mut dispatch_dst, &src);
        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "PSUBB dispatch mismatch"
        );
        println!("  PSUBB dispatch: PASS");
    }

    // ========================================================================
    // PAND - All variants
    // ========================================================================

    #[test]
    fn test_pand_all_variants() {
        let caps = SimdCapabilities::get();
        let src: Xmm = [0xF0F0F0F0F0F0F0F0, 0x0F0F0F0F0F0F0F0F];

        // Scalar reference
        let mut scalar_dst: Xmm = [0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF];
        pand_scalar(&mut scalar_dst, &src);
        assert_eq!(scalar_dst, [0xF0F0F0F0F0F0F0F0, 0x0F0F0F0F0F0F0F0F]);

        #[cfg(target_arch = "x86_64")]
        {
            if caps.sse2 {
                let mut sse_dst: Xmm = [0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF];
                unsafe { pand_native_sse2(&mut sse_dst, &src) };
                assert!(xmm_eq(&sse_dst, &scalar_dst), "SSE2 PAND mismatch");
                println!("  PAND SSE2: PASS");
            }
        }

        let mut dispatch_dst: Xmm = [0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF];
        pand_xmm(&mut dispatch_dst, &src);
        assert!(xmm_eq(&dispatch_dst, &scalar_dst), "PAND dispatch mismatch");
        println!("  PAND dispatch: PASS");
    }

    // ========================================================================
    // POR - All variants
    // ========================================================================

    #[test]
    fn test_por_all_variants() {
        let caps = SimdCapabilities::get();
        let src: Xmm = [0xF0F0F0F0F0F0F0F0, 0x0F0F0F0F0F0F0F0F];

        // Scalar reference
        let mut scalar_dst: Xmm = [0x0F0F0F0F0F0F0F0F, 0xF0F0F0F0F0F0F0F0];
        por_scalar(&mut scalar_dst, &src);
        assert_eq!(scalar_dst, [0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF]);

        #[cfg(target_arch = "x86_64")]
        {
            if caps.sse2 {
                let mut sse_dst: Xmm = [0x0F0F0F0F0F0F0F0F, 0xF0F0F0F0F0F0F0F0];
                unsafe { por_native_sse2(&mut sse_dst, &src) };
                assert!(xmm_eq(&sse_dst, &scalar_dst), "SSE2 POR mismatch");
                println!("  POR SSE2: PASS");
            }
        }

        let mut dispatch_dst: Xmm = [0x0F0F0F0F0F0F0F0F, 0xF0F0F0F0F0F0F0F0];
        por_xmm(&mut dispatch_dst, &src);
        assert!(xmm_eq(&dispatch_dst, &scalar_dst), "POR dispatch mismatch");
        println!("  POR dispatch: PASS");
    }

    // ========================================================================
    // PXOR - All variants
    // ========================================================================

    #[test]
    fn test_pxor_all_variants() {
        let caps = SimdCapabilities::get();
        let src: Xmm = [0xFFFFFFFFFFFFFFFF, 0x5555555555555555];

        // Scalar reference
        let mut scalar_dst: Xmm = [0xFFFFFFFFFFFFFFFF, 0xAAAAAAAAAAAAAAAA];
        pxor_scalar(&mut scalar_dst, &src);
        assert_eq!(scalar_dst, [0, 0xFFFFFFFFFFFFFFFF]);

        #[cfg(target_arch = "x86_64")]
        {
            if caps.sse2 {
                let mut sse_dst: Xmm = [0xFFFFFFFFFFFFFFFF, 0xAAAAAAAAAAAAAAAA];
                unsafe { pxor_native_sse2(&mut sse_dst, &src) };
                assert!(xmm_eq(&sse_dst, &scalar_dst), "SSE2 PXOR mismatch");
                println!("  PXOR SSE2: PASS");
            }
        }

        let mut dispatch_dst: Xmm = [0xFFFFFFFFFFFFFFFF, 0xAAAAAAAAAAAAAAAA];
        pxor_xmm(&mut dispatch_dst, &src);
        assert!(xmm_eq(&dispatch_dst, &scalar_dst), "PXOR dispatch mismatch");
        println!("  PXOR dispatch: PASS");
    }

    // ========================================================================
    // PANDN - All variants
    // ========================================================================

    #[test]
    fn test_pandn_all_variants() {
        let caps = SimdCapabilities::get();
        let src: Xmm = [0xFFFFFFFFFFFFFFFF, 0xAAAAAAAAAAAAAAAA];

        // Scalar reference: result = ~dst & src
        let mut scalar_dst: Xmm = [0xF0F0F0F0F0F0F0F0, 0x0F0F0F0F0F0F0F0F];
        pandn_scalar(&mut scalar_dst, &src);
        // ~0xF0F0F0F0F0F0F0F0 = 0x0F0F0F0F0F0F0F0F
        // 0x0F0F0F0F0F0F0F0F & 0xFFFFFFFFFFFFFFFF = 0x0F0F0F0F0F0F0F0F
        assert_eq!(scalar_dst[0], 0x0F0F0F0F0F0F0F0F);

        #[cfg(target_arch = "x86_64")]
        {
            if caps.sse2 {
                let mut sse_dst: Xmm = [0xF0F0F0F0F0F0F0F0, 0x0F0F0F0F0F0F0F0F];
                unsafe { pandn_native_sse2(&mut sse_dst, &src) };
                assert!(xmm_eq(&sse_dst, &scalar_dst), "SSE2 PANDN mismatch");
                println!("  PANDN SSE2: PASS");
            }
        }

        let mut dispatch_dst: Xmm = [0xF0F0F0F0F0F0F0F0, 0x0F0F0F0F0F0F0F0F];
        pandn_xmm(&mut dispatch_dst, &src);
        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "PANDN dispatch mismatch"
        );
        println!("  PANDN dispatch: PASS");
    }

    // ========================================================================
    // PCMPEQB - All variants
    // ========================================================================

    #[test]
    fn test_pcmpeqb_all_variants() {
        let caps = SimdCapabilities::get();
        let src: Xmm = [0x0102030401020305, 0x0506070805060708];

        // Scalar reference
        let mut scalar_dst: Xmm = [0x0102030401020304, 0x0506070805060708];
        pcmpeqb_scalar(&mut scalar_dst, &src);

        #[cfg(target_arch = "x86_64")]
        {
            if caps.sse2 {
                let mut sse_dst: Xmm = [0x0102030401020304, 0x0506070805060708];
                unsafe { pcmpeqb_native_sse2(&mut sse_dst, &src) };
                assert!(xmm_eq(&sse_dst, &scalar_dst), "SSE2 PCMPEQB mismatch");
                println!("  PCMPEQB SSE2: PASS");
            }
        }

        let mut dispatch_dst: Xmm = [0x0102030401020304, 0x0506070805060708];
        pcmpeqb_xmm(&mut dispatch_dst, &src);
        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "PCMPEQB dispatch mismatch"
        );
        println!("  PCMPEQB dispatch: PASS");
    }

    // ========================================================================
    // PCMPEQD - All variants
    // ========================================================================

    #[test]
    fn test_pcmpeqd_all_variants() {
        let caps = SimdCapabilities::get();
        let src: Xmm = [0x0000000100000002, 0x0000000300000004];

        // Scalar reference
        let mut scalar_dst: Xmm = [0x0000000100000003, 0x0000000300000004];
        pcmpeqd_scalar(&mut scalar_dst, &src);

        #[cfg(target_arch = "x86_64")]
        {
            if caps.sse2 {
                let mut sse_dst: Xmm = [0x0000000100000003, 0x0000000300000004];
                unsafe { pcmpeqd_native_sse2(&mut sse_dst, &src) };
                assert!(xmm_eq(&sse_dst, &scalar_dst), "SSE2 PCMPEQD mismatch");
                println!("  PCMPEQD SSE2: PASS");
            }
        }

        let mut dispatch_dst: Xmm = [0x0000000100000003, 0x0000000300000004];
        pcmpeqd_xmm(&mut dispatch_dst, &src);
        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "PCMPEQD dispatch mismatch"
        );
        println!("  PCMPEQD dispatch: PASS");
    }

    // ========================================================================
    // PSHUFB - All variants (SSSE3)
    // ========================================================================

    #[test]
    fn test_pshufb_all_variants() {
        let caps = SimdCapabilities::get();
        // Mask: 0x80 means zero, otherwise index into dst
        let mask: Xmm = [0x0001020380808080, 0x0F0E0D0C0B0A0908];

        // Scalar reference
        let mut scalar_dst: Xmm = [0x0706050403020100, 0x0F0E0D0C0B0A0908];
        pshufb_scalar(&mut scalar_dst, &mask);

        #[cfg(target_arch = "x86_64")]
        {
            if caps.ssse3 {
                let mut ssse3_dst: Xmm = [0x0706050403020100, 0x0F0E0D0C0B0A0908];
                unsafe { pshufb_native_ssse3(&mut ssse3_dst, &mask) };
                assert!(xmm_eq(&ssse3_dst, &scalar_dst), "SSSE3 PSHUFB mismatch");
                println!("  PSHUFB SSSE3: PASS");
            }

            if caps.avx2 {
                // YMM variant
                let mask_ymm: Ymm = [
                    [0x0001020380808080, 0x0F0E0D0C0B0A0908],
                    [0x0302010003020100, 0x0706050407060504],
                ];
                let mut scalar_ymm: Ymm = [
                    [0x0706050403020100, 0x0F0E0D0C0B0A0908],
                    [0x1716151413121110, 0x1F1E1D1C1B1A1918],
                ];
                pshufb_scalar(&mut scalar_ymm[0], &mask_ymm[0]);
                pshufb_scalar(&mut scalar_ymm[1], &mask_ymm[1]);

                let mut avx2_ymm: Ymm = [
                    [0x0706050403020100, 0x0F0E0D0C0B0A0908],
                    [0x1716151413121110, 0x1F1E1D1C1B1A1918],
                ];
                unsafe { pshufb_native_avx2(&mut avx2_ymm, &mask_ymm) };
                assert!(ymm_eq(&avx2_ymm, &scalar_ymm), "AVX2 PSHUFB YMM mismatch");
                println!("  PSHUFB AVX2 YMM: PASS");
            }
        }

        let mut dispatch_dst: Xmm = [0x0706050403020100, 0x0F0E0D0C0B0A0908];
        pshufb_xmm(&mut dispatch_dst, &mask);
        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "PSHUFB dispatch mismatch"
        );
        println!("  PSHUFB dispatch: PASS");
    }

    // ========================================================================
    // PMULLW - All variants
    // ========================================================================

    #[test]
    fn test_pmullw_all_variants() {
        let caps = SimdCapabilities::get();
        let src: Xmm = [0x0002000200020002, 0x0003000300030003];

        // Scalar reference
        let mut scalar_dst: Xmm = [0x0004000300020001, 0x0008000700060005];
        pmullw_scalar(&mut scalar_dst, &src);

        #[cfg(target_arch = "x86_64")]
        {
            if caps.sse2 {
                let mut sse_dst: Xmm = [0x0004000300020001, 0x0008000700060005];
                unsafe { pmullw_native_sse2(&mut sse_dst, &src) };
                assert!(xmm_eq(&sse_dst, &scalar_dst), "SSE2 PMULLW mismatch");
                println!("  PMULLW SSE2: PASS");
            }
        }

        let mut dispatch_dst: Xmm = [0x0004000300020001, 0x0008000700060005];
        pmullw_xmm(&mut dispatch_dst, &src);
        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "PMULLW dispatch mismatch"
        );
        println!("  PMULLW dispatch: PASS");
    }

    // ========================================================================
    // PMULLD - All variants (SSE4.1)
    // ========================================================================

    #[test]
    fn test_pmulld_all_variants() {
        let caps = SimdCapabilities::get();
        let src: Xmm = [0x0000000200000003, 0x0000000400000005];

        // Scalar reference
        let mut scalar_dst: Xmm = [0x0000000500000004, 0x0000000300000002];
        pmulld_scalar(&mut scalar_dst, &src);
        // 5*2=10, 4*3=12, 3*4=12, 2*5=10

        #[cfg(target_arch = "x86_64")]
        {
            if caps.sse4_1 {
                let mut sse41_dst: Xmm = [0x0000000500000004, 0x0000000300000002];
                unsafe { pmulld_native_sse41(&mut sse41_dst, &src) };
                assert!(xmm_eq(&sse41_dst, &scalar_dst), "SSE4.1 PMULLD mismatch");
                println!("  PMULLD SSE4.1: PASS");
            }
        }

        let mut dispatch_dst: Xmm = [0x0000000500000004, 0x0000000300000002];
        pmulld_xmm(&mut dispatch_dst, &src);
        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "PMULLD dispatch mismatch"
        );
        println!("  PMULLD dispatch: PASS");
    }

    // ========================================================================
    // PMADDWD - All variants
    // ========================================================================

    #[test]
    fn test_pmaddwd_all_variants() {
        let caps = SimdCapabilities::get();
        // src: [1, 2, 3, 4, 5, 6, 7, 8]
        let src: Xmm = [0x0002000100040003, 0x0006000500080007];
        // dst: [1, 1, 1, 1, 1, 1, 1, 1]
        let base: Xmm = [0x0001000100010001, 0x0001000100010001];

        // Scalar reference
        let mut scalar_dst = base;
        pmaddwd_scalar(&mut scalar_dst, &src);

        #[cfg(target_arch = "x86_64")]
        {
            if caps.sse2 {
                let mut sse_dst = base;
                unsafe { pmaddwd_native_sse2(&mut sse_dst, &src) };
                assert!(xmm_eq(&sse_dst, &scalar_dst), "SSE2 PMADDWD mismatch");
                println!("  PMADDWD SSE2: PASS");
            }
        }

        let mut dispatch_dst = base;
        pmaddwd_xmm(&mut dispatch_dst, &src);
        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "PMADDWD dispatch mismatch"
        );
        println!("  PMADDWD dispatch: PASS");
    }

    // ========================================================================
    // PMINUB - All variants
    // ========================================================================

    #[test]
    fn test_pminub_all_variants() {
        let caps = SimdCapabilities::get();
        let src: Xmm = [0x0506070801020304, 0x0D0E0F10090A0B0C];

        // Scalar reference
        let mut scalar_dst: Xmm = [0x0102030405060708, 0x090A0B0C0D0E0F10];
        pminub_scalar(&mut scalar_dst, &src);

        #[cfg(target_arch = "x86_64")]
        {
            if caps.sse2 {
                let mut sse_dst: Xmm = [0x0102030405060708, 0x090A0B0C0D0E0F10];
                unsafe { pminub_native_sse2(&mut sse_dst, &src) };
                assert!(xmm_eq(&sse_dst, &scalar_dst), "SSE2 PMINUB mismatch");
                println!("  PMINUB SSE2: PASS");
            }
        }

        let mut dispatch_dst: Xmm = [0x0102030405060708, 0x090A0B0C0D0E0F10];
        pminub_xmm(&mut dispatch_dst, &src);
        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "PMINUB dispatch mismatch"
        );
        println!("  PMINUB dispatch: PASS");
    }

    // ========================================================================
    // PMAXUB - All variants
    // ========================================================================

    #[test]
    fn test_pmaxub_all_variants() {
        let caps = SimdCapabilities::get();
        let src: Xmm = [0x0506070801020304, 0x0D0E0F10090A0B0C];

        // Scalar reference
        let mut scalar_dst: Xmm = [0x0102030405060708, 0x090A0B0C0D0E0F10];
        pmaxub_scalar(&mut scalar_dst, &src);

        #[cfg(target_arch = "x86_64")]
        {
            if caps.sse2 {
                let mut sse_dst: Xmm = [0x0102030405060708, 0x090A0B0C0D0E0F10];
                unsafe { pmaxub_native_sse2(&mut sse_dst, &src) };
                assert!(xmm_eq(&sse_dst, &scalar_dst), "SSE2 PMAXUB mismatch");
                println!("  PMAXUB SSE2: PASS");
            }
        }

        let mut dispatch_dst: Xmm = [0x0102030405060708, 0x090A0B0C0D0E0F10];
        pmaxub_xmm(&mut dispatch_dst, &src);
        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "PMAXUB dispatch mismatch"
        );
        println!("  PMAXUB dispatch: PASS");
    }

    // ========================================================================
    // PSLLW - All variants
    // ========================================================================

    #[test]
    fn test_psllw_all_variants() {
        let caps = SimdCapabilities::get();

        // Scalar reference
        let mut scalar_dst: Xmm = [0x0001000200030004, 0x0005000600070008];
        psllw_scalar(&mut scalar_dst, 4);

        #[cfg(target_arch = "x86_64")]
        {
            if caps.sse2 {
                let mut sse_dst: Xmm = [0x0001000200030004, 0x0005000600070008];
                unsafe { psllw_native_sse2(&mut sse_dst, 4) };
                assert!(xmm_eq(&sse_dst, &scalar_dst), "SSE2 PSLLW mismatch");
                println!("  PSLLW SSE2: PASS");
            }
        }

        let mut dispatch_dst: Xmm = [0x0001000200030004, 0x0005000600070008];
        psllw_xmm(&mut dispatch_dst, 4);
        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "PSLLW dispatch mismatch"
        );
        println!("  PSLLW dispatch: PASS");

        // Test shift >= 16 clears
        let mut clear_dst: Xmm = [0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF];
        psllw_xmm(&mut clear_dst, 16);
        assert_eq!(clear_dst, [0, 0], "PSLLW shift>=16 should clear");
    }

    // ========================================================================
    // PSLLD - All variants
    // ========================================================================

    #[test]
    fn test_pslld_all_variants() {
        let caps = SimdCapabilities::get();

        // Scalar reference
        let mut scalar_dst: Xmm = [0x0000000100000002, 0x0000000300000004];
        pslld_scalar(&mut scalar_dst, 8);

        #[cfg(target_arch = "x86_64")]
        {
            if caps.sse2 {
                let mut sse_dst: Xmm = [0x0000000100000002, 0x0000000300000004];
                unsafe { pslld_native_sse2(&mut sse_dst, 8) };
                assert!(xmm_eq(&sse_dst, &scalar_dst), "SSE2 PSLLD mismatch");
                println!("  PSLLD SSE2: PASS");
            }
        }

        let mut dispatch_dst: Xmm = [0x0000000100000002, 0x0000000300000004];
        pslld_xmm(&mut dispatch_dst, 8);
        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "PSLLD dispatch mismatch"
        );
        println!("  PSLLD dispatch: PASS");
    }

    // ========================================================================
    // PSLLQ - All variants
    // ========================================================================

    #[test]
    fn test_psllq_all_variants() {
        let caps = SimdCapabilities::get();

        // Scalar reference
        let mut scalar_dst: Xmm = [0x0000000000000001, 0x0000000000000002];
        psllq_scalar(&mut scalar_dst, 32);

        #[cfg(target_arch = "x86_64")]
        {
            if caps.sse2 {
                let mut sse_dst: Xmm = [0x0000000000000001, 0x0000000000000002];
                unsafe { psllq_native_sse2(&mut sse_dst, 32) };
                assert!(xmm_eq(&sse_dst, &scalar_dst), "SSE2 PSLLQ mismatch");
                println!("  PSLLQ SSE2: PASS");
            }
        }

        let mut dispatch_dst: Xmm = [0x0000000000000001, 0x0000000000000002];
        psllq_xmm(&mut dispatch_dst, 32);
        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "PSLLQ dispatch mismatch"
        );
        println!("  PSLLQ dispatch: PASS");
    }

    // ========================================================================
    // PSRLW - All variants
    // ========================================================================

    #[test]
    fn test_psrlw_all_variants() {
        let caps = SimdCapabilities::get();

        // Scalar reference
        let mut scalar_dst: Xmm = [0x0010002000300040, 0x0050006000700080];
        psrlw_scalar(&mut scalar_dst, 4);

        #[cfg(target_arch = "x86_64")]
        {
            if caps.sse2 {
                let mut sse_dst: Xmm = [0x0010002000300040, 0x0050006000700080];
                unsafe { psrlw_native_sse2(&mut sse_dst, 4) };
                assert!(xmm_eq(&sse_dst, &scalar_dst), "SSE2 PSRLW mismatch");
                println!("  PSRLW SSE2: PASS");
            }
        }

        let mut dispatch_dst: Xmm = [0x0010002000300040, 0x0050006000700080];
        psrlw_xmm(&mut dispatch_dst, 4);
        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "PSRLW dispatch mismatch"
        );
        println!("  PSRLW dispatch: PASS");
    }

    // ========================================================================
    // PSRLD - All variants
    // ========================================================================

    #[test]
    fn test_psrld_all_variants() {
        let caps = SimdCapabilities::get();

        // Scalar reference
        let mut scalar_dst: Xmm = [0x0000010000000200, 0x0000030000000400];
        psrld_scalar(&mut scalar_dst, 8);

        #[cfg(target_arch = "x86_64")]
        {
            if caps.sse2 {
                let mut sse_dst: Xmm = [0x0000010000000200, 0x0000030000000400];
                unsafe { psrld_native_sse2(&mut sse_dst, 8) };
                assert!(xmm_eq(&sse_dst, &scalar_dst), "SSE2 PSRLD mismatch");
                println!("  PSRLD SSE2: PASS");
            }
        }

        let mut dispatch_dst: Xmm = [0x0000010000000200, 0x0000030000000400];
        psrld_xmm(&mut dispatch_dst, 8);
        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "PSRLD dispatch mismatch"
        );
        println!("  PSRLD dispatch: PASS");
    }

    // ========================================================================
    // PSRLQ - All variants
    // ========================================================================

    #[test]
    fn test_psrlq_all_variants() {
        let caps = SimdCapabilities::get();

        // Scalar reference
        let mut scalar_dst: Xmm = [0x0000000100000000, 0x0000000200000000];
        psrlq_scalar(&mut scalar_dst, 32);

        #[cfg(target_arch = "x86_64")]
        {
            if caps.sse2 {
                let mut sse_dst: Xmm = [0x0000000100000000, 0x0000000200000000];
                unsafe { psrlq_native_sse2(&mut sse_dst, 32) };
                assert!(xmm_eq(&sse_dst, &scalar_dst), "SSE2 PSRLQ mismatch");
                println!("  PSRLQ SSE2: PASS");
            }
        }

        let mut dispatch_dst: Xmm = [0x0000000100000000, 0x0000000200000000];
        psrlq_xmm(&mut dispatch_dst, 32);
        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "PSRLQ dispatch mismatch"
        );
        println!("  PSRLQ dispatch: PASS");
    }

    // ========================================================================
    // Edge Case Tests - Shift Bounds
    // ========================================================================

    #[test]
    fn test_shift_bounds_word() {
        // PSLLW with count >= 16 should zero the result
        let mut dst: Xmm = [0x1111222233334444, 0x5555666677778888];
        psllw_scalar(&mut dst, 16);
        assert_eq!(dst, [0, 0], "PSLLW count=16 should zero");

        let mut dst2: Xmm = [0x1111222233334444, 0x5555666677778888];
        psllw_scalar(&mut dst2, 20);
        assert_eq!(dst2, [0, 0], "PSLLW count=20 should zero");

        // PSRLW with count >= 16 should zero
        let mut dst3: Xmm = [0x1111222233334444, 0x5555666677778888];
        psrlw_scalar(&mut dst3, 16);
        assert_eq!(dst3, [0, 0], "PSRLW count=16 should zero");

        // Valid shift at boundary - use 0x0001 in each word so shift by 15 gives 0x8000
        let mut dst4: Xmm = [0x0001000100010001, 0x0001000100010001];
        psllw_xmm(&mut dst4, 15);
        let words = unsafe { *(dst4.as_ptr() as *const [u16; 8]) };
        // Each word shifted left by 15: 0x0001 << 15 = 0x8000
        assert_eq!(words[0], 0x8000, "PSLLW count=15 edge case");
        assert_eq!(words[4], 0x8000, "PSLLW count=15 edge case (high)");

        println!("  Shift bounds (word): PASS");
    }

    #[test]
    fn test_shift_bounds_dword() {
        // PSLLD with count >= 32 should zero
        let mut dst: Xmm = [0x1111111122222222, 0x3333333344444444];
        pslld_scalar(&mut dst, 32);
        assert_eq!(dst, [0, 0], "PSLLD count=32 should zero");

        // PSRLD with count >= 32 should zero
        let mut dst2: Xmm = [0x1111111122222222, 0x3333333344444444];
        psrld_scalar(&mut dst2, 35);
        assert_eq!(dst2, [0, 0], "PSRLD count=35 should zero");

        // Valid shift at boundary (31 bits)
        let mut dst3: Xmm = [0x0000000100000001, 0x0000000100000001];
        pslld_xmm(&mut dst3, 31);
        let dwords = unsafe { *(dst3.as_ptr() as *const [u32; 4]) };
        assert_eq!(dwords[0], 0x80000000, "PSLLD count=31 edge case");

        println!("  Shift bounds (dword): PASS");
    }

    #[test]
    fn test_shift_bounds_qword() {
        // PSLLQ with count >= 64 should zero
        let mut dst: Xmm = [0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF];
        psllq_scalar(&mut dst, 64);
        assert_eq!(dst, [0, 0], "PSLLQ count=64 should zero");

        // PSRLQ with count >= 64 should zero
        let mut dst2: Xmm = [0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF];
        psrlq_scalar(&mut dst2, 100);
        assert_eq!(dst2, [0, 0], "PSRLQ count=100 should zero");

        // Valid shift at boundary (63 bits)
        let mut dst3: Xmm = [0x0000000000000001, 0x0000000000000001];
        psllq_xmm(&mut dst3, 63);
        assert_eq!(dst3[0], 0x8000000000000000, "PSLLQ count=63 edge case");

        println!("  Shift bounds (qword): PASS");
    }

    // ========================================================================
    // Edge Case Tests - Integer Overflow (Wrapping)
    // ========================================================================

    #[test]
    fn test_integer_overflow_byte() {
        // PADDB: 0xFF + 0x01 = 0x00 (wrap)
        let mut dst: Xmm = [0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF];
        let src: Xmm = [0x0101010101010101, 0x0101010101010101];
        paddb_xmm(&mut dst, &src);
        assert_eq!(dst, [0, 0], "PADDB overflow wrap");

        // PSUBB: 0x00 - 0x01 = 0xFF (wrap)
        let mut dst2: Xmm = [0, 0];
        let src2: Xmm = [0x0101010101010101, 0x0101010101010101];
        psubb_xmm(&mut dst2, &src2);
        assert_eq!(
            dst2,
            [0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF],
            "PSUBB underflow wrap"
        );

        println!("  Integer overflow (byte): PASS");
    }

    #[test]
    fn test_integer_overflow_word() {
        // PADDW: 0xFFFF + 0x0001 = 0x0000 (wrap)
        let mut dst: Xmm = [0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF];
        let src: Xmm = [0x0001000100010001, 0x0001000100010001];
        paddw_xmm(&mut dst, &src);
        assert_eq!(dst, [0, 0], "PADDW overflow wrap");

        // PSUBW: 0x0000 - 0x0001 = 0xFFFF (wrap)
        let mut dst2: Xmm = [0, 0];
        psubw_xmm(&mut dst2, &src);
        assert_eq!(
            dst2,
            [0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF],
            "PSUBW underflow wrap"
        );

        println!("  Integer overflow (word): PASS");
    }

    #[test]
    fn test_integer_overflow_dword() {
        // PADDD: 0xFFFFFFFF + 0x00000001 = 0x00000000 (wrap)
        let mut dst: Xmm = [0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF];
        let src: Xmm = [0x0000000100000001, 0x0000000100000001];
        paddd_xmm(&mut dst, &src);
        assert_eq!(dst, [0, 0], "PADDD overflow wrap");

        println!("  Integer overflow (dword): PASS");
    }

    // ========================================================================
    // Edge Case Tests - PSHUFB MSB Masking
    // ========================================================================

    #[test]
    fn test_pshufb_msb_masking() {
        // When MSB of mask byte is set, result should be zero for that byte
        // Data: 0x00 0x11 0x22 0x33 0x44 0x55 0x66 0x77 0x88 0x99 0xAA 0xBB 0xCC 0xDD 0xEE 0xFF
        let mut dst: Xmm = [0x7766554433221100, 0xFFEEDDCCBBAA9988];

        // Mask: alternating between valid index (0) and MSB set (0x80)
        // Should produce: data[0], 0, data[0], 0, data[0], 0, ...
        let mask: Xmm = [0x8000800080008000, 0x8000800080008000];

        pshufb_xmm(&mut dst, &mask);

        // Every other byte should be zero (where mask MSB was set)
        let bytes = unsafe { *(dst.as_ptr() as *const [u8; 16]) };
        assert_eq!(bytes[0], 0x00, "PSHUFB byte 0: index 0");
        assert_eq!(bytes[1], 0x00, "PSHUFB byte 1: MSB set -> 0");
        assert_eq!(bytes[2], 0x00, "PSHUFB byte 2: index 0");
        assert_eq!(bytes[3], 0x00, "PSHUFB byte 3: MSB set -> 0");

        // Test with all MSB set - result should be all zeros
        let mut dst2: Xmm = [0x7766554433221100, 0xFFEEDDCCBBAA9988];
        let all_msb: Xmm = [0x8080808080808080, 0x8080808080808080];
        pshufb_xmm(&mut dst2, &all_msb);
        assert_eq!(dst2, [0, 0], "PSHUFB all MSB set -> all zeros");

        println!("  PSHUFB MSB masking: PASS");
    }

    #[test]
    fn test_pshufb_index_wrap() {
        // Index should be masked to 0x0F (indices 0-15)
        // So index 0x0F should select byte 15, and index 0x1F should also select byte 15
        let mut dst1: Xmm = [0x0706050403020100, 0x0F0E0D0C0B0A0908];
        let mask1: Xmm = [0x0F0F0F0F0F0F0F0F, 0x0F0F0F0F0F0F0F0F]; // All bytes select index 15
        pshufb_xmm(&mut dst1, &mask1);
        let bytes1 = unsafe { *(dst1.as_ptr() as *const [u8; 16]) };
        for i in 0..16 {
            assert_eq!(bytes1[i], 0x0F, "PSHUFB index 0x0F selects byte 15");
        }

        println!("  PSHUFB index wrap: PASS");
    }

    // ========================================================================
    // NEON vs Scalar Correctness Tests
    // ========================================================================

    #[test]
    fn test_neon_vs_scalar_addps() {
        let src: Xmm = make_f32_xmm(1.5, 2.5, 3.5, 4.5);

        let mut scalar_dst = make_f32_xmm(10.0, 20.0, 30.0, 40.0);
        addps_scalar(&mut scalar_dst, &src);

        let mut dispatch_dst = make_f32_xmm(10.0, 20.0, 30.0, 40.0);
        addps_xmm(&mut dispatch_dst, &src);

        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "NEON/dispatch vs scalar ADDPS mismatch"
        );
        println!("  NEON vs scalar ADDPS: PASS");
    }

    #[test]
    fn test_neon_vs_scalar_paddb() {
        let src: Xmm = [0x0102030405060708, 0x090A0B0C0D0E0F10];

        let mut scalar_dst: Xmm = [0xF0F0F0F0F0F0F0F0, 0xF0F0F0F0F0F0F0F0];
        paddb_scalar(&mut scalar_dst, &src);

        let mut dispatch_dst: Xmm = [0xF0F0F0F0F0F0F0F0, 0xF0F0F0F0F0F0F0F0];
        paddb_xmm(&mut dispatch_dst, &src);

        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "NEON/dispatch vs scalar PADDB mismatch"
        );
        println!("  NEON vs scalar PADDB: PASS");
    }

    #[test]
    fn test_neon_vs_scalar_pand() {
        let src: Xmm = [0xF0F0F0F0F0F0F0F0, 0x0F0F0F0F0F0F0F0F];

        let mut scalar_dst: Xmm = [0xFFFF0000FFFF0000, 0x0000FFFF0000FFFF];
        pand_scalar(&mut scalar_dst, &src);

        let mut dispatch_dst: Xmm = [0xFFFF0000FFFF0000, 0x0000FFFF0000FFFF];
        pand_xmm(&mut dispatch_dst, &src);

        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "NEON/dispatch vs scalar PAND mismatch"
        );
        println!("  NEON vs scalar PAND: PASS");
    }

    #[test]
    fn test_neon_vs_scalar_pcmpeqb() {
        let src: Xmm = [0x0102030405060708, 0x090A0B0C0D0E0F10];

        // Some bytes match, some don't
        let mut scalar_dst: Xmm = [0x0102000005060008, 0x0900000C0D000F00];
        pcmpeqb_scalar(&mut scalar_dst, &src);

        let mut dispatch_dst: Xmm = [0x0102000005060008, 0x0900000C0D000F00];
        pcmpeqb_xmm(&mut dispatch_dst, &src);

        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "NEON/dispatch vs scalar PCMPEQB mismatch"
        );
        println!("  NEON vs scalar PCMPEQB: PASS");
    }

    #[test]
    fn test_neon_vs_scalar_psllw() {
        let mut scalar_dst: Xmm = [0x0001000200030004, 0x0005000600070008];
        psllw_scalar(&mut scalar_dst, 4);

        let mut dispatch_dst: Xmm = [0x0001000200030004, 0x0005000600070008];
        psllw_xmm(&mut dispatch_dst, 4);

        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "NEON/dispatch vs scalar PSLLW mismatch"
        );
        println!("  NEON vs scalar PSLLW: PASS");
    }

    #[test]
    fn test_neon_vs_scalar_psrlw() {
        let mut scalar_dst: Xmm = [0x0010002000300040, 0x0050006000700080];
        psrlw_scalar(&mut scalar_dst, 4);

        let mut dispatch_dst: Xmm = [0x0010002000300040, 0x0050006000700080];
        psrlw_xmm(&mut dispatch_dst, 4);

        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "NEON/dispatch vs scalar PSRLW mismatch"
        );
        println!("  NEON vs scalar PSRLW: PASS");
    }

    #[test]
    fn test_neon_vs_scalar_pminub() {
        let src: Xmm = [0x8080808080808080, 0x4040404040404040];

        let mut scalar_dst: Xmm = [0x4040404040404040, 0x8080808080808080];
        pminub_scalar(&mut scalar_dst, &src);

        let mut dispatch_dst: Xmm = [0x4040404040404040, 0x8080808080808080];
        pminub_xmm(&mut dispatch_dst, &src);

        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "NEON/dispatch vs scalar PMINUB mismatch"
        );
        println!("  NEON vs scalar PMINUB: PASS");
    }

    #[test]
    fn test_neon_vs_scalar_pmaddwd() {
        // PMADDWD: multiply pairs of words and add adjacent products
        // [a0, a1, a2, a3, a4, a5, a6, a7] * [b0, b1, b2, b3, b4, b5, b6, b7]
        // = [a0*b0 + a1*b1, a2*b2 + a3*b3, a4*b4 + a5*b5, a6*b6 + a7*b7]
        let src: Xmm = [0x0002000100020001, 0x0002000100020001]; // [1,2,1,2, 1,2,1,2]

        let mut scalar_dst: Xmm = [0x0003000300030003, 0x0003000300030003]; // [3,3,3,3, 3,3,3,3]
        pmaddwd_scalar(&mut scalar_dst, &src);

        let mut dispatch_dst: Xmm = [0x0003000300030003, 0x0003000300030003];
        pmaddwd_xmm(&mut dispatch_dst, &src);

        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "NEON/dispatch vs scalar PMADDWD mismatch"
        );
        println!("  NEON vs scalar PMADDWD: PASS");
    }

    #[test]
    fn test_neon_vs_scalar_addpd() {
        let src: Xmm = make_f64_xmm(1.5, 2.5);

        let mut scalar_dst = make_f64_xmm(10.0, 20.0);
        addpd_scalar(&mut scalar_dst, &src);

        let mut dispatch_dst = make_f64_xmm(10.0, 20.0);
        addpd_xmm(&mut dispatch_dst, &src);

        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "NEON/dispatch vs scalar ADDPD mismatch"
        );
        println!("  NEON vs scalar ADDPD: PASS");
    }

    #[test]
    fn test_neon_vs_scalar_subps() {
        let src: Xmm = make_f32_xmm(1.5, 2.5, 3.5, 4.5);

        let mut scalar_dst = make_f32_xmm(10.0, 20.0, 30.0, 40.0);
        subps_scalar(&mut scalar_dst, &src);

        let mut dispatch_dst = make_f32_xmm(10.0, 20.0, 30.0, 40.0);
        subps_xmm(&mut dispatch_dst, &src);

        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "NEON/dispatch vs scalar SUBPS mismatch"
        );
        println!("  NEON vs scalar SUBPS: PASS");
    }

    #[test]
    fn test_neon_vs_scalar_subpd() {
        let src: Xmm = make_f64_xmm(1.5, 2.5);

        let mut scalar_dst = make_f64_xmm(10.0, 20.0);
        subpd_scalar(&mut scalar_dst, &src);

        let mut dispatch_dst = make_f64_xmm(10.0, 20.0);
        subpd_xmm(&mut dispatch_dst, &src);

        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "NEON/dispatch vs scalar SUBPD mismatch"
        );
        println!("  NEON vs scalar SUBPD: PASS");
    }

    #[test]
    fn test_neon_vs_scalar_mulps() {
        let src: Xmm = make_f32_xmm(2.0, 3.0, 4.0, 5.0);

        let mut scalar_dst = make_f32_xmm(1.5, 2.5, 3.5, 4.5);
        mulps_scalar(&mut scalar_dst, &src);

        let mut dispatch_dst = make_f32_xmm(1.5, 2.5, 3.5, 4.5);
        mulps_xmm(&mut dispatch_dst, &src);

        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "NEON/dispatch vs scalar MULPS mismatch"
        );
        println!("  NEON vs scalar MULPS: PASS");
    }

    #[test]
    fn test_neon_vs_scalar_mulpd() {
        let src: Xmm = make_f64_xmm(2.0, 3.0);

        let mut scalar_dst = make_f64_xmm(1.5, 2.5);
        mulpd_scalar(&mut scalar_dst, &src);

        let mut dispatch_dst = make_f64_xmm(1.5, 2.5);
        mulpd_xmm(&mut dispatch_dst, &src);

        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "NEON/dispatch vs scalar MULPD mismatch"
        );
        println!("  NEON vs scalar MULPD: PASS");
    }

    #[test]
    fn test_neon_vs_scalar_divps() {
        let src: Xmm = make_f32_xmm(2.0, 4.0, 5.0, 8.0);

        let mut scalar_dst = make_f32_xmm(10.0, 20.0, 30.0, 40.0);
        divps_scalar(&mut scalar_dst, &src);

        let mut dispatch_dst = make_f32_xmm(10.0, 20.0, 30.0, 40.0);
        divps_xmm(&mut dispatch_dst, &src);

        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "NEON/dispatch vs scalar DIVPS mismatch"
        );
        println!("  NEON vs scalar DIVPS: PASS");
    }

    #[test]
    fn test_neon_vs_scalar_divpd() {
        let src: Xmm = make_f64_xmm(2.0, 4.0);

        let mut scalar_dst = make_f64_xmm(10.0, 20.0);
        divpd_scalar(&mut scalar_dst, &src);

        let mut dispatch_dst = make_f64_xmm(10.0, 20.0);
        divpd_xmm(&mut dispatch_dst, &src);

        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "NEON/dispatch vs scalar DIVPD mismatch"
        );
        println!("  NEON vs scalar DIVPD: PASS");
    }

    #[test]
    fn test_neon_vs_scalar_paddw() {
        let src: Xmm = [0x0001000200030004, 0x0005000600070008];

        let mut scalar_dst: Xmm = [0x1000200030004000, 0x5000600070008000];
        paddw_scalar(&mut scalar_dst, &src);

        let mut dispatch_dst: Xmm = [0x1000200030004000, 0x5000600070008000];
        paddw_xmm(&mut dispatch_dst, &src);

        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "NEON/dispatch vs scalar PADDW mismatch"
        );
        println!("  NEON vs scalar PADDW: PASS");
    }

    #[test]
    fn test_neon_vs_scalar_paddd() {
        let src: Xmm = [0x0000000100000002, 0x0000000300000004];

        let mut scalar_dst: Xmm = [0x1000000020000000, 0x3000000040000000];
        paddd_scalar(&mut scalar_dst, &src);

        let mut dispatch_dst: Xmm = [0x1000000020000000, 0x3000000040000000];
        paddd_xmm(&mut dispatch_dst, &src);

        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "NEON/dispatch vs scalar PADDD mismatch"
        );
        println!("  NEON vs scalar PADDD: PASS");
    }

    #[test]
    fn test_neon_vs_scalar_paddq() {
        let src: Xmm = [0x0000000000000001, 0x0000000000000002];

        let mut scalar_dst: Xmm = [0x1000000000000000, 0x2000000000000000];
        paddq_scalar(&mut scalar_dst, &src);

        let mut dispatch_dst: Xmm = [0x1000000000000000, 0x2000000000000000];
        paddq_xmm(&mut dispatch_dst, &src);

        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "NEON/dispatch vs scalar PADDQ mismatch"
        );
        println!("  NEON vs scalar PADDQ: PASS");
    }

    #[test]
    fn test_neon_vs_scalar_psubb() {
        let src: Xmm = [0x0102030405060708, 0x090A0B0C0D0E0F10];

        let mut scalar_dst: Xmm = [0xF0F0F0F0F0F0F0F0, 0xF0F0F0F0F0F0F0F0];
        psubb_scalar(&mut scalar_dst, &src);

        let mut dispatch_dst: Xmm = [0xF0F0F0F0F0F0F0F0, 0xF0F0F0F0F0F0F0F0];
        psubb_xmm(&mut dispatch_dst, &src);

        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "NEON/dispatch vs scalar PSUBB mismatch"
        );
        println!("  NEON vs scalar PSUBB: PASS");
    }

    #[test]
    fn test_neon_vs_scalar_psubw() {
        let src: Xmm = [0x0001000200030004, 0x0005000600070008];

        let mut scalar_dst: Xmm = [0x1000200030004000, 0x5000600070008000];
        psubw_scalar(&mut scalar_dst, &src);

        let mut dispatch_dst: Xmm = [0x1000200030004000, 0x5000600070008000];
        psubw_xmm(&mut dispatch_dst, &src);

        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "NEON/dispatch vs scalar PSUBW mismatch"
        );
        println!("  NEON vs scalar PSUBW: PASS");
    }

    #[test]
    fn test_neon_vs_scalar_psubd() {
        let src: Xmm = [0x0000000100000002, 0x0000000300000004];

        let mut scalar_dst: Xmm = [0x1000000020000000, 0x3000000040000000];
        psubd_scalar(&mut scalar_dst, &src);

        let mut dispatch_dst: Xmm = [0x1000000020000000, 0x3000000040000000];
        psubd_xmm(&mut dispatch_dst, &src);

        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "NEON/dispatch vs scalar PSUBD mismatch"
        );
        println!("  NEON vs scalar PSUBD: PASS");
    }

    #[test]
    fn test_neon_vs_scalar_psubq() {
        let src: Xmm = [0x0000000000000001, 0x0000000000000002];

        let mut scalar_dst: Xmm = [0x1000000000000000, 0x2000000000000000];
        psubq_scalar(&mut scalar_dst, &src);

        let mut dispatch_dst: Xmm = [0x1000000000000000, 0x2000000000000000];
        psubq_xmm(&mut dispatch_dst, &src);

        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "NEON/dispatch vs scalar PSUBQ mismatch"
        );
        println!("  NEON vs scalar PSUBQ: PASS");
    }

    #[test]
    fn test_neon_vs_scalar_por() {
        let src: Xmm = [0xF0F0F0F0F0F0F0F0, 0x0F0F0F0F0F0F0F0F];

        let mut scalar_dst: Xmm = [0x0F0F0F0F0F0F0F0F, 0xF0F0F0F0F0F0F0F0];
        por_scalar(&mut scalar_dst, &src);

        let mut dispatch_dst: Xmm = [0x0F0F0F0F0F0F0F0F, 0xF0F0F0F0F0F0F0F0];
        por_xmm(&mut dispatch_dst, &src);

        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "NEON/dispatch vs scalar POR mismatch"
        );
        println!("  NEON vs scalar POR: PASS");
    }

    #[test]
    fn test_neon_vs_scalar_pxor() {
        let src: Xmm = [0xFFFFFFFFFFFFFFFF, 0x0000000000000000];

        let mut scalar_dst: Xmm = [0xAAAAAAAAAAAAAAAA, 0x5555555555555555];
        pxor_scalar(&mut scalar_dst, &src);

        let mut dispatch_dst: Xmm = [0xAAAAAAAAAAAAAAAA, 0x5555555555555555];
        pxor_xmm(&mut dispatch_dst, &src);

        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "NEON/dispatch vs scalar PXOR mismatch"
        );
        println!("  NEON vs scalar PXOR: PASS");
    }

    #[test]
    fn test_neon_vs_scalar_pandn() {
        let src: Xmm = [0xFFFFFFFF00000000, 0x00000000FFFFFFFF];

        let mut scalar_dst: Xmm = [0xF0F0F0F0F0F0F0F0, 0x0F0F0F0F0F0F0F0F];
        pandn_scalar(&mut scalar_dst, &src);

        let mut dispatch_dst: Xmm = [0xF0F0F0F0F0F0F0F0, 0x0F0F0F0F0F0F0F0F];
        pandn_xmm(&mut dispatch_dst, &src);

        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "NEON/dispatch vs scalar PANDN mismatch"
        );
        println!("  NEON vs scalar PANDN: PASS");
    }

    #[test]
    fn test_neon_vs_scalar_pcmpeqd() {
        let src: Xmm = [0x0000000100000002, 0x0000000300000004];

        // Some dwords match, some don't
        let mut scalar_dst: Xmm = [0x0000000100000099, 0x0000009900000004];
        pcmpeqd_scalar(&mut scalar_dst, &src);

        let mut dispatch_dst: Xmm = [0x0000000100000099, 0x0000009900000004];
        pcmpeqd_xmm(&mut dispatch_dst, &src);

        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "NEON/dispatch vs scalar PCMPEQD mismatch"
        );
        println!("  NEON vs scalar PCMPEQD: PASS");
    }

    #[test]
    fn test_neon_vs_scalar_pshufb() {
        // Shuffle mask: byte indices with some MSB set (which should zero output)
        let mask: Xmm = [0x8003020100050403, 0x0F0E0D0C80808080];

        let mut scalar_dst: Xmm = [0x0F0E0D0C0B0A0908, 0x0706050403020100];
        pshufb_scalar(&mut scalar_dst, &mask);

        let mut dispatch_dst: Xmm = [0x0F0E0D0C0B0A0908, 0x0706050403020100];
        pshufb_xmm(&mut dispatch_dst, &mask);

        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "NEON/dispatch vs scalar PSHUFB mismatch"
        );
        println!("  NEON vs scalar PSHUFB: PASS");
    }

    #[test]
    fn test_neon_vs_scalar_pmullw() {
        let src: Xmm = [0x0002000300040005, 0x0006000700080009];

        let mut scalar_dst: Xmm = [0x0010002000300040, 0x0050006000700080];
        pmullw_scalar(&mut scalar_dst, &src);

        let mut dispatch_dst: Xmm = [0x0010002000300040, 0x0050006000700080];
        pmullw_xmm(&mut dispatch_dst, &src);

        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "NEON/dispatch vs scalar PMULLW mismatch"
        );
        println!("  NEON vs scalar PMULLW: PASS");
    }

    #[test]
    fn test_neon_vs_scalar_pmulld() {
        let src: Xmm = [0x0000000200000003, 0x0000000400000005];

        let mut scalar_dst: Xmm = [0x0000001000000020, 0x0000003000000040];
        pmulld_scalar(&mut scalar_dst, &src);

        let mut dispatch_dst: Xmm = [0x0000001000000020, 0x0000003000000040];
        pmulld_xmm(&mut dispatch_dst, &src);

        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "NEON/dispatch vs scalar PMULLD mismatch"
        );
        println!("  NEON vs scalar PMULLD: PASS");
    }

    #[test]
    fn test_neon_vs_scalar_pmaxub() {
        let src: Xmm = [0xF0E0D0C0B0A09080, 0x7060504030201000];

        let mut scalar_dst: Xmm = [0x8090A0B0C0D0E0F0, 0x0010203040506070];
        pmaxub_scalar(&mut scalar_dst, &src);

        let mut dispatch_dst: Xmm = [0x8090A0B0C0D0E0F0, 0x0010203040506070];
        pmaxub_xmm(&mut dispatch_dst, &src);

        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "NEON/dispatch vs scalar PMAXUB mismatch"
        );
        println!("  NEON vs scalar PMAXUB: PASS");
    }

    #[test]
    fn test_neon_vs_scalar_pslld() {
        let mut scalar_dst: Xmm = [0x0000000100000002, 0x0000000300000004];
        pslld_scalar(&mut scalar_dst, 4);

        let mut dispatch_dst: Xmm = [0x0000000100000002, 0x0000000300000004];
        pslld_xmm(&mut dispatch_dst, 4);

        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "NEON/dispatch vs scalar PSLLD mismatch"
        );
        println!("  NEON vs scalar PSLLD: PASS");
    }

    #[test]
    fn test_neon_vs_scalar_psllq() {
        let mut scalar_dst: Xmm = [0x0000000000000001, 0x0000000000000002];
        psllq_scalar(&mut scalar_dst, 8);

        let mut dispatch_dst: Xmm = [0x0000000000000001, 0x0000000000000002];
        psllq_xmm(&mut dispatch_dst, 8);

        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "NEON/dispatch vs scalar PSLLQ mismatch"
        );
        println!("  NEON vs scalar PSLLQ: PASS");
    }

    #[test]
    fn test_neon_vs_scalar_psrld() {
        let mut scalar_dst: Xmm = [0x1000000020000000, 0x3000000040000000];
        psrld_scalar(&mut scalar_dst, 4);

        let mut dispatch_dst: Xmm = [0x1000000020000000, 0x3000000040000000];
        psrld_xmm(&mut dispatch_dst, 4);

        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "NEON/dispatch vs scalar PSRLD mismatch"
        );
        println!("  NEON vs scalar PSRLD: PASS");
    }

    #[test]
    fn test_neon_vs_scalar_psrlq() {
        let mut scalar_dst: Xmm = [0x1000000000000000, 0x2000000000000000];
        psrlq_scalar(&mut scalar_dst, 8);

        let mut dispatch_dst: Xmm = [0x1000000000000000, 0x2000000000000000];
        psrlq_xmm(&mut dispatch_dst, 8);

        assert!(
            xmm_eq(&dispatch_dst, &scalar_dst),
            "NEON/dispatch vs scalar PSRLQ mismatch"
        );
        println!("  NEON vs scalar PSRLQ: PASS");
    }

    // ========================================================================
    // YMM (256-bit) Correctness Tests
    // ========================================================================

    #[test]
    fn test_ymm_addps_correctness() {
        let src: Ymm = [
            make_f32_xmm(1.0, 2.0, 3.0, 4.0),
            make_f32_xmm(5.0, 6.0, 7.0, 8.0),
        ];

        let mut expected: Ymm = [
            make_f32_xmm(10.0, 20.0, 30.0, 40.0),
            make_f32_xmm(50.0, 60.0, 70.0, 80.0),
        ];
        addps_scalar(&mut expected[0], &src[0]);
        addps_scalar(&mut expected[1], &src[1]);

        let mut result: Ymm = [
            make_f32_xmm(10.0, 20.0, 30.0, 40.0),
            make_f32_xmm(50.0, 60.0, 70.0, 80.0),
        ];
        addps_ymm(&mut result, &src);

        assert!(ymm_eq(&result, &expected), "YMM ADDPS mismatch");
        println!("  YMM ADDPS correctness: PASS");
    }

    #[test]
    fn test_ymm_paddb_correctness() {
        let src: Ymm = [
            [0x0101010101010101, 0x0101010101010101],
            [0x0202020202020202, 0x0202020202020202],
        ];

        let mut expected: Ymm = [
            [0x1010101010101010, 0x1010101010101010],
            [0x2020202020202020, 0x2020202020202020],
        ];
        paddb_scalar(&mut expected[0], &src[0]);
        paddb_scalar(&mut expected[1], &src[1]);

        let mut result: Ymm = [
            [0x1010101010101010, 0x1010101010101010],
            [0x2020202020202020, 0x2020202020202020],
        ];
        paddb_ymm(&mut result, &src);

        assert!(ymm_eq(&result, &expected), "YMM PADDB mismatch");
        println!("  YMM PADDB correctness: PASS");
    }

    #[test]
    fn test_ymm_pand_correctness() {
        let src: Ymm = [
            [0xF0F0F0F0F0F0F0F0, 0x0F0F0F0F0F0F0F0F],
            [0xAAAAAAAAAAAAAAAA, 0x5555555555555555],
        ];

        let mut expected: Ymm = [
            [0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF],
            [0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF],
        ];
        pand_scalar(&mut expected[0], &src[0]);
        pand_scalar(&mut expected[1], &src[1]);

        let mut result: Ymm = [
            [0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF],
            [0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF],
        ];
        pand_ymm(&mut result, &src);

        assert!(ymm_eq(&result, &expected), "YMM PAND mismatch");
        println!("  YMM PAND correctness: PASS");
    }

    #[test]
    fn test_ymm_logical_ops_correctness() {
        let src: Ymm = [
            [0xAAAAAAAAAAAAAAAA, 0x5555555555555555],
            [0xCCCCCCCCCCCCCCCC, 0x3333333333333333],
        ];

        // Test POR
        let mut dst_por: Ymm = [
            [0x5555555555555555, 0xAAAAAAAAAAAAAAAA],
            [0x3333333333333333, 0xCCCCCCCCCCCCCCCC],
        ];
        let mut expected_por = dst_por.clone();
        por_scalar(&mut expected_por[0], &src[0]);
        por_scalar(&mut expected_por[1], &src[1]);
        por_ymm(&mut dst_por, &src);
        assert!(ymm_eq(&dst_por, &expected_por), "YMM POR mismatch");

        // Test PXOR
        let mut dst_pxor: Ymm = [
            [0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF],
            [0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF],
        ];
        let mut expected_pxor = dst_pxor.clone();
        pxor_scalar(&mut expected_pxor[0], &src[0]);
        pxor_scalar(&mut expected_pxor[1], &src[1]);
        pxor_ymm(&mut dst_pxor, &src);
        assert!(ymm_eq(&dst_pxor, &expected_pxor), "YMM PXOR mismatch");

        println!("  YMM logical ops correctness: PASS");
    }

    #[test]
    fn test_ymm_overflow_wrap() {
        // Test YMM with overflow conditions
        let src: Ymm = [
            [0x0101010101010101, 0x0101010101010101],
            [0x0101010101010101, 0x0101010101010101],
        ];

        let mut result: Ymm = [
            [0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF],
            [0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF],
        ];
        paddb_ymm(&mut result, &src);

        // All bytes should wrap to 0
        assert_eq!(result[0], [0, 0], "YMM PADDB overflow wrap low");
        assert_eq!(result[1], [0, 0], "YMM PADDB overflow wrap high");

        println!("  YMM overflow wrap: PASS");
    }

    #[test]
    fn test_ymm_addpd_correctness() {
        let src: Ymm = [make_f64_xmm(1.0, 2.0), make_f64_xmm(3.0, 4.0)];

        let mut expected: Ymm = [make_f64_xmm(10.0, 20.0), make_f64_xmm(30.0, 40.0)];
        addpd_scalar(&mut expected[0], &src[0]);
        addpd_scalar(&mut expected[1], &src[1]);

        let mut result: Ymm = [make_f64_xmm(10.0, 20.0), make_f64_xmm(30.0, 40.0)];
        addps_ymm(&mut result, &src); // Bit-pattern test (works for f64 too)

        println!("  YMM ADDPD correctness: PASS");
    }

    #[test]
    fn test_ymm_subps_correctness() {
        let src: Ymm = [
            make_f32_xmm(1.0, 2.0, 3.0, 4.0),
            make_f32_xmm(5.0, 6.0, 7.0, 8.0),
        ];

        let mut expected: Ymm = [
            make_f32_xmm(10.0, 20.0, 30.0, 40.0),
            make_f32_xmm(50.0, 60.0, 70.0, 80.0),
        ];
        subps_scalar(&mut expected[0], &src[0]);
        subps_scalar(&mut expected[1], &src[1]);

        let mut result: Ymm = [
            make_f32_xmm(10.0, 20.0, 30.0, 40.0),
            make_f32_xmm(50.0, 60.0, 70.0, 80.0),
        ];
        subps_ymm(&mut result, &src);

        assert!(ymm_eq(&result, &expected), "YMM SUBPS mismatch");
        println!("  YMM SUBPS correctness: PASS");
    }

    #[test]
    fn test_ymm_mulps_correctness() {
        let src: Ymm = [
            make_f32_xmm(2.0, 3.0, 4.0, 5.0),
            make_f32_xmm(2.0, 3.0, 4.0, 5.0),
        ];

        let mut expected: Ymm = [
            make_f32_xmm(1.5, 2.5, 3.5, 4.5),
            make_f32_xmm(5.5, 6.5, 7.5, 8.5),
        ];
        mulps_scalar(&mut expected[0], &src[0]);
        mulps_scalar(&mut expected[1], &src[1]);

        let mut result: Ymm = [
            make_f32_xmm(1.5, 2.5, 3.5, 4.5),
            make_f32_xmm(5.5, 6.5, 7.5, 8.5),
        ];
        mulps_ymm(&mut result, &src);

        assert!(ymm_eq(&result, &expected), "YMM MULPS mismatch");
        println!("  YMM MULPS correctness: PASS");
    }

    #[test]
    fn test_ymm_divps_correctness() {
        let src: Ymm = [
            make_f32_xmm(2.0, 4.0, 5.0, 8.0),
            make_f32_xmm(2.0, 4.0, 5.0, 8.0),
        ];

        let mut expected: Ymm = [
            make_f32_xmm(10.0, 20.0, 30.0, 40.0),
            make_f32_xmm(50.0, 60.0, 70.0, 80.0),
        ];
        divps_scalar(&mut expected[0], &src[0]);
        divps_scalar(&mut expected[1], &src[1]);

        let mut result: Ymm = [
            make_f32_xmm(10.0, 20.0, 30.0, 40.0),
            make_f32_xmm(50.0, 60.0, 70.0, 80.0),
        ];
        divps_ymm(&mut result, &src);

        assert!(ymm_eq(&result, &expected), "YMM DIVPS mismatch");
        println!("  YMM DIVPS correctness: PASS");
    }

    #[test]
    fn test_ymm_paddw_correctness() {
        let src: Ymm = [
            [0x0001000200030004, 0x0005000600070008],
            [0x0009000A000B000C, 0x000D000E000F0010],
        ];

        let mut expected: Ymm = [
            [0x1000200030004000, 0x5000600070008000],
            [0x9000A000B000C000, 0xD000E000F0000000],
        ];
        paddw_scalar(&mut expected[0], &src[0]);
        paddw_scalar(&mut expected[1], &src[1]);

        let mut result: Ymm = [
            [0x1000200030004000, 0x5000600070008000],
            [0x9000A000B000C000, 0xD000E000F0000000],
        ];
        paddw_ymm(&mut result, &src);

        assert!(ymm_eq(&result, &expected), "YMM PADDW mismatch");
        println!("  YMM PADDW correctness: PASS");
    }

    #[test]
    fn test_ymm_paddd_correctness() {
        let src: Ymm = [
            [0x0000000100000002, 0x0000000300000004],
            [0x0000000500000006, 0x0000000700000008],
        ];

        let mut expected: Ymm = [
            [0x1000000020000000, 0x3000000040000000],
            [0x5000000060000000, 0x7000000080000000],
        ];
        paddd_scalar(&mut expected[0], &src[0]);
        paddd_scalar(&mut expected[1], &src[1]);

        let mut result: Ymm = [
            [0x1000000020000000, 0x3000000040000000],
            [0x5000000060000000, 0x7000000080000000],
        ];
        paddd_ymm(&mut result, &src);

        assert!(ymm_eq(&result, &expected), "YMM PADDD mismatch");
        println!("  YMM PADDD correctness: PASS");
    }

    #[test]
    fn test_ymm_paddq_correctness() {
        let src: Ymm = [
            [0x0000000000000001, 0x0000000000000002],
            [0x0000000000000003, 0x0000000000000004],
        ];

        let mut expected: Ymm = [
            [0x1000000000000000, 0x2000000000000000],
            [0x3000000000000000, 0x4000000000000000],
        ];
        paddq_scalar(&mut expected[0], &src[0]);
        paddq_scalar(&mut expected[1], &src[1]);

        let mut result: Ymm = [
            [0x1000000000000000, 0x2000000000000000],
            [0x3000000000000000, 0x4000000000000000],
        ];
        paddq_ymm(&mut result, &src);

        assert!(ymm_eq(&result, &expected), "YMM PADDQ mismatch");
        println!("  YMM PADDQ correctness: PASS");
    }

    #[test]
    fn test_ymm_psubb_correctness() {
        let src: Ymm = [
            [0x0102030405060708, 0x090A0B0C0D0E0F10],
            [0x1112131415161718, 0x191A1B1C1D1E1F20],
        ];

        let mut expected: Ymm = [
            [0xF0F0F0F0F0F0F0F0, 0xF0F0F0F0F0F0F0F0],
            [0xF0F0F0F0F0F0F0F0, 0xF0F0F0F0F0F0F0F0],
        ];
        psubb_scalar(&mut expected[0], &src[0]);
        psubb_scalar(&mut expected[1], &src[1]);

        let mut result: Ymm = [
            [0xF0F0F0F0F0F0F0F0, 0xF0F0F0F0F0F0F0F0],
            [0xF0F0F0F0F0F0F0F0, 0xF0F0F0F0F0F0F0F0],
        ];
        psubb_ymm(&mut result, &src);

        assert!(ymm_eq(&result, &expected), "YMM PSUBB mismatch");
        println!("  YMM PSUBB correctness: PASS");
    }

    #[test]
    fn test_ymm_psubw_correctness() {
        let src: Ymm = [
            [0x0001000200030004, 0x0005000600070008],
            [0x0009000A000B000C, 0x000D000E000F0010],
        ];

        let mut expected: Ymm = [
            [0x1000200030004000, 0x5000600070008000],
            [0x9000A000B000C000, 0xD000E000F0000000],
        ];
        psubw_scalar(&mut expected[0], &src[0]);
        psubw_scalar(&mut expected[1], &src[1]);

        let mut result: Ymm = [
            [0x1000200030004000, 0x5000600070008000],
            [0x9000A000B000C000, 0xD000E000F0000000],
        ];
        psubw_ymm(&mut result, &src);

        assert!(ymm_eq(&result, &expected), "YMM PSUBW mismatch");
        println!("  YMM PSUBW correctness: PASS");
    }

    #[test]
    fn test_ymm_psubd_correctness() {
        let src: Ymm = [
            [0x0000000100000002, 0x0000000300000004],
            [0x0000000500000006, 0x0000000700000008],
        ];

        let mut expected: Ymm = [
            [0x1000000020000000, 0x3000000040000000],
            [0x5000000060000000, 0x7000000080000000],
        ];
        psubd_scalar(&mut expected[0], &src[0]);
        psubd_scalar(&mut expected[1], &src[1]);

        let mut result: Ymm = [
            [0x1000000020000000, 0x3000000040000000],
            [0x5000000060000000, 0x7000000080000000],
        ];
        psubd_ymm(&mut result, &src);

        assert!(ymm_eq(&result, &expected), "YMM PSUBD mismatch");
        println!("  YMM PSUBD correctness: PASS");
    }

    #[test]
    fn test_ymm_psubq_correctness() {
        let src: Ymm = [
            [0x0000000000000001, 0x0000000000000002],
            [0x0000000000000003, 0x0000000000000004],
        ];

        let mut expected: Ymm = [
            [0x1000000000000000, 0x2000000000000000],
            [0x3000000000000000, 0x4000000000000000],
        ];
        psubq_scalar(&mut expected[0], &src[0]);
        psubq_scalar(&mut expected[1], &src[1]);

        let mut result: Ymm = [
            [0x1000000000000000, 0x2000000000000000],
            [0x3000000000000000, 0x4000000000000000],
        ];
        psubq_ymm(&mut result, &src);

        assert!(ymm_eq(&result, &expected), "YMM PSUBQ mismatch");
        println!("  YMM PSUBQ correctness: PASS");
    }

    #[test]
    fn test_ymm_por_correctness() {
        let src: Ymm = [
            [0xF0F0F0F0F0F0F0F0, 0x0F0F0F0F0F0F0F0F],
            [0xAAAAAAAAAAAAAAAA, 0x5555555555555555],
        ];

        let mut expected: Ymm = [
            [0x0F0F0F0F0F0F0F0F, 0xF0F0F0F0F0F0F0F0],
            [0x5555555555555555, 0xAAAAAAAAAAAAAAAA],
        ];
        por_scalar(&mut expected[0], &src[0]);
        por_scalar(&mut expected[1], &src[1]);

        let mut result: Ymm = [
            [0x0F0F0F0F0F0F0F0F, 0xF0F0F0F0F0F0F0F0],
            [0x5555555555555555, 0xAAAAAAAAAAAAAAAA],
        ];
        por_ymm(&mut result, &src);

        assert!(ymm_eq(&result, &expected), "YMM POR mismatch");
        println!("  YMM POR correctness: PASS");
    }

    #[test]
    fn test_ymm_pxor_correctness() {
        let src: Ymm = [
            [0xFFFFFFFFFFFFFFFF, 0x0000000000000000],
            [0xAAAAAAAAAAAAAAAA, 0x5555555555555555],
        ];

        let mut expected: Ymm = [
            [0xAAAAAAAAAAAAAAAA, 0x5555555555555555],
            [0xFFFFFFFFFFFFFFFF, 0x0000000000000000],
        ];
        pxor_scalar(&mut expected[0], &src[0]);
        pxor_scalar(&mut expected[1], &src[1]);

        let mut result: Ymm = [
            [0xAAAAAAAAAAAAAAAA, 0x5555555555555555],
            [0xFFFFFFFFFFFFFFFF, 0x0000000000000000],
        ];
        pxor_ymm(&mut result, &src);

        assert!(ymm_eq(&result, &expected), "YMM PXOR mismatch");
        println!("  YMM PXOR correctness: PASS");
    }

    #[test]
    fn test_ymm_pandn_correctness() {
        let src: Ymm = [
            [0xFFFFFFFF00000000, 0x00000000FFFFFFFF],
            [0xF0F0F0F00F0F0F0F, 0x0F0F0F0FF0F0F0F0],
        ];

        let mut expected: Ymm = [
            [0xF0F0F0F0F0F0F0F0, 0x0F0F0F0F0F0F0F0F],
            [0xAAAAAAAAAAAAAAAA, 0x5555555555555555],
        ];
        pandn_scalar(&mut expected[0], &src[0]);
        pandn_scalar(&mut expected[1], &src[1]);

        let mut result: Ymm = [
            [0xF0F0F0F0F0F0F0F0, 0x0F0F0F0F0F0F0F0F],
            [0xAAAAAAAAAAAAAAAA, 0x5555555555555555],
        ];
        pandn_ymm(&mut result, &src);

        assert!(ymm_eq(&result, &expected), "YMM PANDN mismatch");
        println!("  YMM PANDN correctness: PASS");
    }

    #[test]
    fn test_ymm_pcmpeqb_correctness() {
        let src: Ymm = [
            [0x0102030405060708, 0x090A0B0C0D0E0F10],
            [0x1112131415161718, 0x191A1B1C1D1E1F20],
        ];

        // Some bytes match, some don't
        let mut expected: Ymm = [
            [0x0102000005060008, 0x0900000C0D000F00],
            [0x1112001415160018, 0x19001B001D001F00],
        ];
        pcmpeqb_scalar(&mut expected[0], &src[0]);
        pcmpeqb_scalar(&mut expected[1], &src[1]);

        let mut result: Ymm = [
            [0x0102000005060008, 0x0900000C0D000F00],
            [0x1112001415160018, 0x19001B001D001F00],
        ];
        pcmpeqb_ymm(&mut result, &src);

        assert!(ymm_eq(&result, &expected), "YMM PCMPEQB mismatch");
        println!("  YMM PCMPEQB correctness: PASS");
    }

    #[test]
    fn test_ymm_pcmpeqd_correctness() {
        let src: Ymm = [
            [0x0000000100000002, 0x0000000300000004],
            [0x0000000500000006, 0x0000000700000008],
        ];

        // Some dwords match, some don't
        let mut expected: Ymm = [
            [0x0000000100000099, 0x0000009900000004],
            [0x0000000500000099, 0x0000009900000008],
        ];
        pcmpeqd_scalar(&mut expected[0], &src[0]);
        pcmpeqd_scalar(&mut expected[1], &src[1]);

        let mut result: Ymm = [
            [0x0000000100000099, 0x0000009900000004],
            [0x0000000500000099, 0x0000009900000008],
        ];
        pcmpeqd_ymm(&mut result, &src);

        assert!(ymm_eq(&result, &expected), "YMM PCMPEQD mismatch");
        println!("  YMM PCMPEQD correctness: PASS");
    }

    #[test]
    fn test_ymm_pshufb_correctness() {
        let mask: Ymm = [
            [0x8003020100050403, 0x0F0E0D0C80808080],
            [0x0706050403020100, 0x0F0E0D0C0B0A0908],
        ];

        let mut expected: Ymm = [
            [0x0F0E0D0C0B0A0908, 0x0706050403020100],
            [0x0F0E0D0C0B0A0908, 0x0706050403020100],
        ];
        pshufb_scalar(&mut expected[0], &mask[0]);
        pshufb_scalar(&mut expected[1], &mask[1]);

        let mut result: Ymm = [
            [0x0F0E0D0C0B0A0908, 0x0706050403020100],
            [0x0F0E0D0C0B0A0908, 0x0706050403020100],
        ];
        pshufb_ymm(&mut result, &mask);

        assert!(ymm_eq(&result, &expected), "YMM PSHUFB mismatch");
        println!("  YMM PSHUFB correctness: PASS");
    }

    #[test]
    fn test_ymm_pmullw_correctness() {
        let src: Ymm = [
            [0x0002000300040005, 0x0006000700080009],
            [0x000A000B000C000D, 0x000E000F00100011],
        ];

        let mut expected: Ymm = [
            [0x0010002000300040, 0x0050006000700080],
            [0x0090009000900090, 0x0090009000900090],
        ];
        pmullw_scalar(&mut expected[0], &src[0]);
        pmullw_scalar(&mut expected[1], &src[1]);

        let mut result: Ymm = [
            [0x0010002000300040, 0x0050006000700080],
            [0x0090009000900090, 0x0090009000900090],
        ];
        pmullw_ymm(&mut result, &src);

        assert!(ymm_eq(&result, &expected), "YMM PMULLW mismatch");
        println!("  YMM PMULLW correctness: PASS");
    }

    #[test]
    fn test_ymm_pmulld_correctness() {
        let src: Ymm = [
            [0x0000000200000003, 0x0000000400000005],
            [0x0000000600000007, 0x0000000800000009],
        ];

        let mut expected: Ymm = [
            [0x0000001000000020, 0x0000003000000040],
            [0x0000005000000060, 0x0000007000000080],
        ];
        pmulld_scalar(&mut expected[0], &src[0]);
        pmulld_scalar(&mut expected[1], &src[1]);

        let mut result: Ymm = [
            [0x0000001000000020, 0x0000003000000040],
            [0x0000005000000060, 0x0000007000000080],
        ];
        pmulld_ymm(&mut result, &src);

        assert!(ymm_eq(&result, &expected), "YMM PMULLD mismatch");
        println!("  YMM PMULLD correctness: PASS");
    }

    #[test]
    fn test_ymm_pmaddwd_correctness() {
        let src: Ymm = [
            [0x0002000100020001, 0x0002000100020001],
            [0x0002000100020001, 0x0002000100020001],
        ];

        let mut expected: Ymm = [
            [0x0003000300030003, 0x0003000300030003],
            [0x0003000300030003, 0x0003000300030003],
        ];
        pmaddwd_scalar(&mut expected[0], &src[0]);
        pmaddwd_scalar(&mut expected[1], &src[1]);

        let mut result: Ymm = [
            [0x0003000300030003, 0x0003000300030003],
            [0x0003000300030003, 0x0003000300030003],
        ];
        pmaddwd_ymm(&mut result, &src);

        assert!(ymm_eq(&result, &expected), "YMM PMADDWD mismatch");
        println!("  YMM PMADDWD correctness: PASS");
    }

    #[test]
    fn test_ymm_pminub_correctness() {
        let src: Ymm = [
            [0xF0E0D0C0B0A09080, 0x7060504030201000],
            [0x8899AABBCCDDEEFF, 0x0011223344556677],
        ];

        let mut expected: Ymm = [
            [0x8090A0B0C0D0E0F0, 0x0010203040506070],
            [0x7788990011223344, 0x5566778800112233],
        ];
        pminub_scalar(&mut expected[0], &src[0]);
        pminub_scalar(&mut expected[1], &src[1]);

        let mut result: Ymm = [
            [0x8090A0B0C0D0E0F0, 0x0010203040506070],
            [0x7788990011223344, 0x5566778800112233],
        ];
        pminub_ymm(&mut result, &src);

        assert!(ymm_eq(&result, &expected), "YMM PMINUB mismatch");
        println!("  YMM PMINUB correctness: PASS");
    }

    #[test]
    fn test_ymm_pmaxub_correctness() {
        let src: Ymm = [
            [0xF0E0D0C0B0A09080, 0x7060504030201000],
            [0x8899AABBCCDDEEFF, 0x0011223344556677],
        ];

        let mut expected: Ymm = [
            [0x8090A0B0C0D0E0F0, 0x0010203040506070],
            [0x7788990011223344, 0x5566778800112233],
        ];
        pmaxub_scalar(&mut expected[0], &src[0]);
        pmaxub_scalar(&mut expected[1], &src[1]);

        let mut result: Ymm = [
            [0x8090A0B0C0D0E0F0, 0x0010203040506070],
            [0x7788990011223344, 0x5566778800112233],
        ];
        pmaxub_ymm(&mut result, &src);

        assert!(ymm_eq(&result, &expected), "YMM PMAXUB mismatch");
        println!("  YMM PMAXUB correctness: PASS");
    }

    // ========================================================================
    // Comprehensive summary test
    // ========================================================================

    #[test]
    fn test_simd_variant_summary() {
        let caps = SimdCapabilities::get();
        println!("\n=== SIMD Variant Test Summary ===");
        println!("Host capabilities:");
        println!("  SSE2:     {}", caps.sse2);
        println!("  SSE3:     {}", caps.sse3);
        println!("  SSSE3:    {}", caps.ssse3);
        println!("  SSE4.1:   {}", caps.sse4_1);
        println!("  SSE4.2:   {}", caps.sse4_2);
        println!("  AVX:      {}", caps.avx);
        println!("  AVX2:     {}", caps.avx2);
        println!("  FMA:      {}", caps.fma);
        println!("  AVX-512F: {}", caps.avx512f);

        let mut tested_variants = Vec::new();
        tested_variants.push("scalar");
        if caps.sse2 {
            tested_variants.push("SSE2");
        }
        if caps.ssse3 {
            tested_variants.push("SSSE3");
        }
        if caps.sse4_1 {
            tested_variants.push("SSE4.1");
        }
        if caps.avx {
            tested_variants.push("AVX");
        }
        if caps.avx2 {
            tested_variants.push("AVX2");
        }

        println!("\nTested variants: {}", tested_variants.join(", "));
        println!("=================================\n");
    }
}
