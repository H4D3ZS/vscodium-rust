//! Re-export of crate-level timing module for x86_64 emulator.
//!
//! This allows the emulator modules to use `super::timing` instead of `crate::timing`.

pub use crate::timing::*;
