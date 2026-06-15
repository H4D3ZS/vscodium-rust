//! Shift and rotate instructions: SHL, SHR, SAL, SAR, ROL, ROR, RCL, RCR, SHLD, SHRD.

mod core;
mod double;
mod groups;

// Re-export all instruction functions
pub use double::*;
pub use groups::*;

// Note: core module exports are internal (execute_shift, execute_shift8)
