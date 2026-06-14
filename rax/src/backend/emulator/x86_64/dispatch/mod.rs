//! Instruction dispatch modules for the x86_64 CPU emulator.
//!
//! This module contains the opcode dispatch logic, split by encoding:
//! - `legacy`: Single-byte opcode dispatch
//! - `twobyte`: Two-byte (0x0F-prefixed) opcode dispatch
//! - `vex`: VEX-encoded (AVX) instruction dispatch
//! - `evex`: EVEX-encoded (AVX-512) instruction dispatch
//! - `resolver`: maps a decoded opcode to a fn-pointer handler for the
//!   decode-cache fast path

mod evex;
mod legacy;
mod resolver;
mod twobyte;
mod vex;
