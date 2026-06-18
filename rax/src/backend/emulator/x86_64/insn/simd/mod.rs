//! SSE/AVX/AVX-512 SIMD instruction implementations.
//!
//! This module contains all SIMD-related instructions organized into submodules:
//! - `mov`: Data movement (MOVD, MOVQ, MOVDQA, MOVDQU)
//! - `sse`: Packed SSE operations (MOVUPS, MOVAPS, ANDPS, ORPS, XORPS)
//! - `convert`: Type conversion (CVT* instructions)
//! - `arith`: Arithmetic (ADD, SUB, MUL, DIV, SQRT)
//! - `compare`: Comparisons (CMPPS, CMPPD, CMPSS, CMPSD)
//! - `shuffle`: Shuffle and unpack (PSHUFD, UNPCKLPS, UNPCKHPS)
//! - `minmax`: Min/max operations (MINPS, MAXPS, MINPD, MAXPD)
//! - `avx512`: AVX-512 instructions (EVEX-encoded)

mod arith;
mod avx;
mod avx512;
mod compare;
mod convert;
mod gfni;
mod minmax;
mod mov;
mod shuffle;
mod sse;
mod sse4;
mod ssse3;

// Re-export all instruction functions
pub use arith::*;
pub use avx::*;
pub use avx512::*;
pub use compare::*;
pub use convert::*;
pub use gfni::*;
pub use minmax::*;
pub use mov::*;
pub use shuffle::*;
pub use sse::*;
pub use sse4::*;
pub use ssse3::*;
