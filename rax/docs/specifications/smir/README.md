# SMIR Specification

**SMIR** (Sigma Machine IR) is a multi-architecture intermediate representation for the RAX emulator.

## Document Index

### Core Specification

| Document | Description |
|----------|-------------|
| [00-feasibility.md](00-feasibility.md) | Feasibility analysis and existing infrastructure assessment |
| [01-overview.md](01-overview.md) | Architecture overview, design goals, and core concepts |
| [02-types.md](02-types.md) | Complete Rust type definitions (Module, Function, Block, VReg, etc.) |
| [03-opcodes.md](03-opcodes.md) | Full opcode catalog with ~100 operations and semantics |
| [04-flags.md](04-flags.md) | Flag handling, lazy evaluation, and condition mapping |
| [05-memory.md](05-memory.md) | Memory model, addressing, atomics, and SMC detection |
| [06-lifting.md](06-lifting.md) | Per-architecture lifters (x86_64, AArch64, Hexagon) |
| [07-execution.md](07-execution.md) | Interpreter specification and execution context |
| [08-optimization.md](08-optimization.md) | Optimization passes (dead flags, constant prop, etc.) |

### Architecture Extensions

| Document | Description |
|----------|-------------|
| [09-vliw-extensions.md](09-vliw-extensions.md) | VLIW bundles, DSP ops, delay slots, register windows |
| [10-risc-extensions.md](10-risc-extensions.md) | RISC-V ISA, RVV vectors, crypto, bit manipulation |

## Quick Start

```rust
use smir::{SmirInterpreter, SmirContext, SourceArch};

// Create interpreter for x86_64
let mut interp = SmirInterpreter::new(SourceArch::X86_64);

// Create execution context
let mut ctx = SmirContext::new_x86_64(memory);
ctx.pc = entry_point;

// Run until exit
let exit = interp.run(&mut ctx);
```

## Design Principles

1. **Architecture Independence** - Same IR for CISC, RISC, VLIW, and DSP
2. **Efficient Interpretation** - Lazy flags, parallel bundle semantics
3. **JIT Ready** - Structure supports future native code generation
4. **Analyzable** - Clean semantics enable optimization passes
5. **Extensible** - Architecture-specific escapes for complex semantics

## Key Features

- **~150+ IR operations** covering integer, FP, SIMD, DSP, memory, control flow
- **Lazy flag evaluation** - Flags computed on-demand, not every instruction
- **VLIW bundle support** - Parallel execution, `.new` forwarding, slot constraints
- **DSP extensions** - Saturating arithmetic, fixed-point, circular addressing
- **Three-tier execution** - Interpreter → Template JIT → Optimizing JIT
- **Dead flag elimination** - Critical optimization for x86 performance

## Supported Architectures

| Architecture | Type | Status |
|--------------|------|--------|
| x86_64 | CISC | Specified |
| AArch64/ARM | RISC | Specified |
| Hexagon | VLIW/DSP | Specified |
| RISC-V | RISC | Specified |
| SPARC | RISC + Windows | Specified |
| MIPS | RISC + Delay | Specified |
| Itanium | VLIW | Partial |
| TI C6000 | VLIW/DSP | Partial |

## Implementation Status

| Component | Status |
|-----------|--------|
| Core types | Specified |
| x86_64 lifter | Specified |
| AArch64 lifter | Specified |
| Hexagon lifter | Specified |
| RISC-V lifter | Specified |
| VLIW bundles | Specified |
| DSP operations | Specified |
| Interpreter | Specified |
| Optimizations | Specified |
| JIT Tier 1 | Future |
| JIT Tier 2 | Future |

## Version

- **Specification Version**: 0.1
- **Date**: 2026-01-24
