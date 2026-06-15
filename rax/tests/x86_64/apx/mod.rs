//! Intel APX (Advanced Performance Extensions) Tests
//!
//! APX is Intel's major x86-64 ISA extension that adds:
//!
//! 1. **EGPR (Extended General Purpose Registers)** - 16 new 64-bit registers (R16-R31)
//! 2. **REX2 Prefix** - New prefix encoding (0xD5) for accessing EGPR in legacy instructions
//! 3. **Extended EVEX** - EVEX prefix extended to encode EGPR
//! 4. **NDD (New Data Destination)** - 3-operand forms of legacy 2-operand instructions
//! 5. **NF (No Flags)** - Arithmetic instructions that don't modify RFLAGS
//! 6. **CCMP/CTEST** - Conditional compare and test instructions
//! 7. **PUSH2/POP2** - Push/pop pairs of registers atomically
//! 8. **Zero-Upper (ZU)** - 32-bit operations that zero upper 32 bits in 64-bit mode
//!
//! Encoding details:
//! - REX2 prefix: 0xD5 followed by payload byte
//!   - Payload: M R3 X3 B3 W R4 X4 B4
//!   - M=1 enables extended opcode map
//!   - R3/X3/B3 are standard REX bits
//!   - R4/X4/B4 are extended bits for R16-R31
//!
//! - Extended EVEX:
//!   - EVEX.R4 (bit 4 of byte 1) for reg field
//!   - EVEX.X4 (bit 4 of byte 2) for index field
//!   - EVEX.B4 (bit 3 of byte 2) for r/m field
//!   - EVEX.V4 (bit 3 of byte 3) for vvvv field

// Extended GPR tests (R16-R31 with REX2)
mod egpr;

// REX2 prefix encoding tests
mod rex2;

// NDD (New Data Destination) 3-operand instruction tests
mod ndd;

// No-Flags (NF) instruction tests
mod nf;

// Conditional compare/test (CCMP/CTEST) tests
mod ccmp_ctest;

// PUSH2/POP2 instruction tests
mod push2_pop2;

// Zero-Upper semantics tests
mod zu;

// Combined/complex APX feature tests
mod combined;
