# SMIR Specification v0.1

**SMIR** (Sigma Machine IR) is a multi-architecture intermediate representation for RAX.

## 1. Design Goals

### 1.1 Primary Goals

1. **Architecture Independence**: Single IR represents semantics from x86_64, ARM/AArch64, Hexagon, and future architectures (RISC-V, MIPS).

2. **Efficient Interpretation**: IR must be interpretable with minimal overhead. Target: <2x slowdown vs. current direct interpretation.

3. **JIT-Ready**: IR structure must support lowering to native code without redesign.

4. **Analyzable**: Enable static analysis, optimization passes, and formal verification.

5. **Debuggable**: IR must preserve enough information for source-level debugging and tracing.

### 1.2 Non-Goals (v0.1)

- Full SSA form (may add in v0.2)
- Aggressive optimization passes
- Multi-threaded JIT compilation
- GPU/accelerator backends

## 2. Architecture Overview

```
                        Source Architectures
                               │
        ┌──────────────────────┼──────────────────────┐
        │                      │                      │
        ▼                      ▼                      ▼
   ┌─────────┐           ┌───────────┐          ┌───────────┐
   │ x86_64  │           │ AArch64   │          │ Hexagon   │
   │ Lifter  │           │ Lifter    │          │ Lifter    │
   └────┬────┘           └─────┬─────┘          └─────┬─────┘
        │                      │                      │
        └──────────────────────┼──────────────────────┘
                               │
                               ▼
                    ┌─────────────────────┐
                    │     SMIR Module     │
                    │                     │
                    │  ┌───────────────┐  │
                    │  │ SmirFunction  │  │
                    │  │               │  │
                    │  │ ┌───────────┐ │  │
                    │  │ │ SmirBlock │ │  │
                    │  │ │           │ │  │
                    │  │ │ SmirOp[]  │ │  │
                    │  │ └───────────┘ │  │
                    │  └───────────────┘  │
                    └──────────┬──────────┘
                               │
        ┌──────────────────────┼──────────────────────┐
        │                      │                      │
        ▼                      ▼                      ▼
   ┌──────────┐         ┌────────────┐         ┌───────────┐
   │Interpreter│         │ JIT Tier 1 │         │ JIT Tier 2│
   │          │         │ (template) │         │ (optimize)│
   └──────────┘         └────────────┘         └───────────┘
```

## 3. Core Concepts

### 3.1 Module (`SmirModule`)

A module is the top-level container for lifted code. It contains:
- Functions (named and anonymous)
- Global symbols
- External references
- Architecture metadata

### 3.2 Function (`SmirFunction`)

A function represents a contiguous region of guest code:
- Entry block
- One or more basic blocks
- Local variable slots
- Stack frame layout

### 3.3 Block (`SmirBlock`)

A basic block is a straight-line sequence of operations with:
- Unique block ID
- Zero or more `SmirOp` instructions
- Exactly one terminator (branch, return, trap)
- Phi nodes at entry (for SSA, optional in v0.1)

### 3.4 Operation (`SmirOp`)

An individual IR instruction with:
- Opcode (operation type)
- Destination operand(s)
- Source operand(s)
- Optional side effects (flags, memory)

### 3.5 Virtual Register (`VReg`)

A value holder that may be:
- Virtual (SSA-style, unlimited)
- Architecture-specific (pinned to guest register)

## 4. Type System

SMIR uses a simple, fixed type system:

| Type | Size | Description |
|------|------|-------------|
| `i8` | 1 byte | 8-bit integer |
| `i16` | 2 bytes | 16-bit integer |
| `i32` | 4 bytes | 32-bit integer |
| `i64` | 8 bytes | 64-bit integer |
| `i128` | 16 bytes | 128-bit integer (for SIMD lanes) |
| `f32` | 4 bytes | IEEE 754 single precision |
| `f64` | 8 bytes | IEEE 754 double precision |
| `v128` | 16 bytes | 128-bit vector |
| `v256` | 32 bytes | 256-bit vector |
| `v512` | 64 bytes | 512-bit vector |
| `ptr` | 8 bytes | Guest virtual address |
| `flags` | 8 bytes | Flag register (lazy or materialized) |

Integers are signless; signedness is an operation property, not a type property.

## 5. Execution Model

### 5.1 Sequential Semantics

Within a block, operations execute sequentially. Side effects are visible to subsequent operations.

### 5.2 Memory Model

SMIR assumes sequential consistency for the interpreter. Weaker models (TSO for x86, weakly-ordered for ARM) are handled by explicit barrier operations.

### 5.3 Exception Model

Exceptions (page faults, undefined instructions, division by zero) are precise. The interpreter can always recover the guest PC at fault point.

### 5.4 Self-Modifying Code

Guest writes to executable pages invalidate cached SMIR. Detection uses page protection (current RAX mechanism).

## 6. Relation to Guest Architectures

### 6.1 x86_64 Mapping

| x86 Concept | SMIR Representation |
|-------------|---------------------|
| RAX, RBX, ... | `ArchReg::X86(X86Reg::Rax)` |
| RFLAGS | `FlagState` with lazy evaluation |
| XMM0-XMM31 | `VReg::Arch(X86Reg::Xmm(n))` |
| RIP-relative | `Address::PcRel(offset)` |
| REP prefix | Loop construct or unrolled ops |
| LOCK prefix | `SmirOp::AtomicRmw` |

### 6.2 AArch64 Mapping

| ARM Concept | SMIR Representation |
|-------------|---------------------|
| X0-X30, SP | `ArchReg::Arm(ArmReg::X(n))` |
| PSTATE.NZCV | `FlagState` |
| V0-V31 | `VReg::Arch(ArmReg::V(n))` |
| Conditional exec | Lowered to branches |
| CSEL | `SmirOp::Select` |
| LDR/STR | `SmirOp::Load/Store` with appropriate width |

### 6.3 Hexagon Mapping

| Hexagon Concept | SMIR Representation |
|-----------------|---------------------|
| R0-R31 | `ArchReg::Hexagon(HexagonReg::R(n))` |
| P0-P3 | Predicate as `i1` or separate `PredState` |
| GP-relative | `Address::GpRel(offset)` |
| Packets | Unrolled to sequential ops |
| Hardware loops | Loop constructs |

## 7. Document Organization

| Document | Contents |
|----------|----------|
| `02-types.md` | Complete Rust type definitions |
| `03-opcodes.md` | Full opcode catalog with semantics |
| `04-operands.md` | Operand and addressing modes |
| `05-flags.md` | Flag handling and conditions |
| `06-memory.md` | Memory model and atomics |
| `07-lifting.md` | Per-architecture lifters |
| `08-execution.md` | Interpreter specification |
| `09-optimization.md` | Optimization passes |

## 8. Version History

| Version | Date | Changes |
|---------|------|---------|
| 0.1 | 2026-01-24 | Initial specification |
