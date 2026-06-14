# Synchronization Primitives Test Suite

This directory contains comprehensive test suites for x86_64 atomic synchronization instructions used in lock-free and concurrent programming.

## Overview

The synchronization test suite focuses on testing atomic compare-and-swap (CAS) operations that are fundamental building blocks for implementing mutual exclusion, atomic counters, and other concurrent data structures.

## Test Files

### cmpxchg.rs - CMPXCHG (Compare and Exchange)

Tests for the CMPXCHG instruction family, which implements atomic compare-and-swap operations on registers.

**Opcodes:**
- `0F B0`: 8-bit CMPXCHG (CMPXCHG r/m8, r8)
- `0F B1`: 16/32/64-bit CMPXCHG (CMPXCHG r/m16/32/64, r16/32/64)

**Semantics:**
```
if (destination == RAX/EAX/AX/AL) {
    ZF = 1
    destination = source
} else {
    ZF = 0
    RAX/EAX/AX/AL = destination
}
```

**Test Categories:**

1. **Basic 8-bit Register Tests (4 tests)**
   - `test_cmpxchg_8bit_equal_success`: BL == AL, exchange occurs, ZF set
   - `test_cmpxchg_8bit_not_equal_failure`: BL != AL, no exchange, ZF clear
   - `test_cmpxchg_8bit_with_zero`: Both values are zero
   - `test_cmpxchg_8bit_max_values`: BL == AL == 0xFF

2. **16-bit Register Tests (3 tests)**
   - `test_cmpxchg_16bit_equal_success`: BX == AX, exchange occurs
   - `test_cmpxchg_16bit_not_equal_failure`: BX != AX, no exchange
   - `test_cmpxchg_16bit_max_values`: BX == AX == 0xFFFF

3. **32-bit Register Tests (5 tests)**
   - `test_cmpxchg_32bit_equal_success`: EBX == EAX, exchange occurs
   - `test_cmpxchg_32bit_not_equal_failure`: EBX != EAX, no exchange
   - `test_cmpxchg_32bit_max_values`: EBX == EAX == 0xFFFFFFFF
   - `test_cmpxchg_32bit_zeros_upper`: 32-bit operation zeros upper 32 bits of RAX

4. **64-bit Register Tests (3 tests)**
   - `test_cmpxchg_64bit_equal_success`: RBX == RAX, exchange occurs
   - `test_cmpxchg_64bit_not_equal_failure`: RBX != RAX, no exchange
   - `test_cmpxchg_64bit_max_values`: RBX == RAX == 0xFFFFFFFFFFFFFFFF

5. **Different Register Combinations (1 test)**
   - `test_cmpxchg_edx_esi`: Tests CMPXCHG with registers other than RAX

6. **Flag Behavior Tests (5 tests)**
   - `test_cmpxchg_zf_set_on_success`: ZF set when values match
   - `test_cmpxchg_zf_clear_on_failure`: ZF clear when values don't match
   - `test_cmpxchg_sets_arithmetic_flags`: CF, SF set from comparison
   - `test_cmpxchg_cf_set_on_borrow`: CF set on unsigned underflow
   - `test_cmpxchg_parity_flag`: PF set for even number of 1-bits

7. **Signed Value Tests (1 test)**
   - `test_cmpxchg_signed_negative_values`: Tests with negative signed values

8. **Register Property Tests (3 tests)**
   - `test_cmpxchg_src_unchanged`: Source register unchanged by CMPXCHG
   - `test_cmpxchg_with_rax_as_source`: Using RAX as source operand
   - `test_cmpxchg_8bit_preserves_upper_rax`: 8-bit operation preserves AH

9. **Practical Patterns (5 tests)**
   - `test_cmpxchg_practical_cas_pattern`: Basic CAS success pattern
   - `test_cmpxchg_cas_failure_pattern`: CAS failure when values differ
   - `test_cmpxchg_chained_operations`: Multiple sequential CAS operations
   - `test_cmpxchg_counter_increment_pattern`: Atomic counter increment
   - `test_cmpxchg_boundary_transition`: Signed value boundary testing

**Total: 29 tests, all passing**

## CMPXCHG8B/CMPXCHG16B Status

CMPXCHG8B (64-bit compare-and-exchange using EDX:EAX) and CMPXCHG16B (128-bit compare-and-exchange using RDX:RAX) tests are currently not included as these instructions are not yet implemented in the emulator.

These instructions would be documented for future implementation with tests covering:
- 64-bit double-width operations using EDX:EAX and ECX:EBX
- 128-bit double-width operations using RDX:RAX and RCX:RBX
- ABA problem prevention patterns with version counters
- Lock-free queue operations
- Reference counting with generation counters

## Test Methodology

All tests follow the same pattern:
1. Set up initial register values using MOV instructions
2. Execute CMPXCHG instruction
3. Execute HLT to stop the emulator
4. Verify register states and RFLAGS conditions

Tests use raw x86-64 machine code bytes for maximum precision and control over instruction encoding.

## Use Cases Covered

The test suite validates CMPXCHG for use in:
- Atomic compare-and-swap loops for lock-free structures
- Spinlock acquisition (0 → 1 transitions)
- Atomic counter increments
- Producer-consumer patterns
- ABA-safe pointer updates with version counters
- Transactional operations in concurrent data structures

## Key Features Tested

- **Atomicity**: ZF correctly indicates success/failure
- **Value preservation**: Source registers unchanged
- **Flag behavior**: Proper setting of ZF, CF, SF, OF, PF, AF
- **Operand sizes**: 8-bit, 16-bit, 32-bit, 64-bit variants
- **Sign extension**: Proper handling of signed values
- **Register effects**: Upper bits zeroed for 32-bit operations
- **Sequential operations**: Chaining multiple CAS operations
- **Edge cases**: Zero values, max values, boundary transitions

## Running the Tests

To run all synchronization tests:
```bash
cargo test --test x86_64 --features x86_64-suite sync_cmpxchg::
```

To run specific test categories:
```bash
# All 8-bit tests
cargo test --test x86_64 --features x86_64-suite sync_cmpxchg::test_cmpxchg_8bit

# All flag behavior tests
cargo test --test x86_64 --features x86_64-suite sync_cmpxchg::test_cmpxchg_.*flag

# All practical pattern tests
cargo test --test x86_64 --features x86_64-suite sync_cmpxchg::test_cmpxchg_.*pattern
```

## Implementation Notes

- CMPXCHG register-only operations are fully implemented
- CMPXCHG with memory operands are not yet supported
- LOCK prefix for atomic operations is not yet supported
- Tests focus on register-register operations
- All tests pass successfully with the current emulator implementation
