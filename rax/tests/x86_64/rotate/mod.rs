//! Comprehensive tests for x86_64 rotate and double-precision shift instructions.
//!
//! This module contains exhaustive tests for:
//! - ROL (Rotate Left) - Basic rotation without carry
//! - ROR (Rotate Right) - Basic rotation without carry
//! - RCL (Rotate Through Carry Left) - 9/17/33/65-bit rotation
//! - RCR (Rotate Through Carry Right) - 9/17/33/65-bit rotation
//! - SHLD (Double Precision Shift Left) - Multi-precision left shift
//! - SHRD (Double Precision Shift Right) - Multi-precision right shift
//!
//! Each instruction is tested with:
//! - All operand sizes (8/16/32/64-bit)
//! - Immediate and CL register shift counts
//! - All general-purpose registers (including R8-R15)
//! - Memory operands
//! - Flag behavior (CF, OF, SF, ZF, PF)
//! - Boundary conditions and edge cases
//! - Count masking behavior
//!
//! Extended test files provide comprehensive coverage of:
//! - All valid shift counts for each operand size
//! - Boundary value testing
//! - Flag transition edge cases
//! - Pattern-based testing (alternating bits, powers of two, etc.)

#[path = "../common/mod.rs"]
mod common;

pub mod rol;
pub mod ror;
pub mod rcl;
pub mod rcr;
pub mod shld;
pub mod shrd;
pub mod rol_ror_extended;
pub mod shld_shrd_extended;
