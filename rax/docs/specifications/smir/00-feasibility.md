# SMIR Feasibility Analysis

**Date**: 2026-01-24  
**Status**: Feasible  
**Codename**: SMIR (Sigma Machine IR)

## Executive Summary

This document analyzes the feasibility of implementing a custom intermediate representation (IR) for RAX that can:

1. Lift x86_64 instructions to a unified IR
2. Support multiple source architectures (x86_64, ARM/AArch64, Hexagon, future RISC-V)
3. Execute the IR through interpretation or compilation

**Verdict**: Highly feasible. RAX already contains substantial infrastructure that can be extended for an IR layer.

---

## Current Architecture Analysis

### x86_64 Backend (`src/backend/emulator/x86_64/`)

| Component | Current State | Notes |
|-----------|---------------|-------|
| Decoder | Prefix parsing only (`InsnContext`) | Returns raw bytes + prefix state, no instruction enum |
| Execution | Giant opcode match dispatch | Direct interpretation, no intermediate form |
| Operands | On-demand via `decode_modrm()` | Extracted inline during execution |
| Flags | Lazy evaluation (`LazyFlagOp`) | Good abstraction, reusable |
| SIMD | Cross-platform dispatch (`simd_native.rs`) | Proven pattern for multi-arch |

**Key Observation**: No IR exists. Each instruction is decoded and executed atomically with operands extracted on-demand.

```rust
// Current x86 InsnContext - prefix state only, no semantic IR
pub struct InsnContext {
    pub bytes: [u8; MAX_INSN_LEN],
    pub bytes_len: usize,
    pub cursor: usize,
    pub rex: Option<u8>,
    pub operand_size_override: bool,
    pub address_size_override: bool,
    pub rep_prefix: Option<u8>,
    pub segment_override: Option<u8>,
    pub evex: Option<EvexPrefix>,
}
```

### ARM/AArch64 Backend (`src/arm/`)

| Component | Current State | Notes |
|-----------|---------------|-------|
| Decoder | Full `DecodedInsn` struct | ~300 mnemonics, explicit operands |
| Execution | Re-decodes raw u32 in `execute()` | Doesn't use decoder output during execution |
| Operands | `Vec<Operand>` with rich types | Registers, immediates, memory, shifts |
| Flags | PSTATE inline updates | Per-instruction |
| CPU Trait | `ArmCpu` trait | Cross-profile abstraction |

**Key Observation**: Has structured decoder output but execution path doesn't use it - re-decodes from raw instruction.

```rust
// ARM DecodedInsn - structured but not used for execution
pub struct DecodedInsn {
    pub mnemonic: Mnemonic,
    pub cond: Option<Condition>,
    pub operands: Vec<Operand>,
    pub sets_flags: bool,
    pub raw: u32,
    pub size: u8,
    pub state: ExecutionState,
}
```

### Hexagon Backend (`src/backend/emulator/hexagon/`)

| Component | Current State | Notes |
|-----------|---------------|-------|
| Decoder | `DecodedInsn` enum | **IR-like structure** |
| Execution | Interprets decoded enum | Pattern we want for SMIR |
| Operands | Explicit in enum variants | Clean, no re-parsing |

**Key Observation**: Hexagon `DecodedInsn` is effectively a working prototype of SMIR.

```rust
// Hexagon DecodedInsn - the SMIR prototype
pub enum DecodedInsn {
    Add { dst: u8, src1: u8, src2: u8 },
    Sub { dst: u8, src1: u8, src2: u8 },
    Load { dst: u8, addr: AddrMode, width: MemWidth, sign: MemSign, pred: Option<PredCond> },
    Store { src: u8, addr: AddrMode, width: MemWidth, pred: Option<PredCond>, src_new: bool },
    ShiftImm { dst: u8, src: u8, kind: ShiftKind, amount: u8 },
    Cmp { pred: u8, src1: u8, src2: u8, kind: CmpKind },
    Jump { offset: i32 },
    // ... ~50 variants
}
```

---

## Existing Infrastructure to Leverage

### 1. simd_native.rs - Cross-Platform Dispatch Pattern

The `simd_native` module demonstrates how to abstract operations with platform-specific optimized implementations:

```rust
pub fn addps_xmm(dst: &mut Xmm, src: &Xmm) {
    #[cfg(target_arch = "x86_64")]
    if SimdCapabilities::get().sse2 {
        return unsafe { addps_native_sse(dst, src) };
    }
    #[cfg(target_arch = "aarch64")]
    if SimdCapabilities::get().neon {
        return unsafe { addps_native_neon(dst, src) };
    }
    addps_scalar(dst, src); // Universal fallback
}
```

**Application to SMIR**: This pattern extends naturally to IR operation execution with host-optimized backends.

### 2. Shared Types Across Backends

Several types are already defined similarly across backends:

```rust
// Memory width (Hexagon + ARM)
pub enum MemWidth { Byte, Half, Word, Double }

// Shift kinds (Hexagon + ARM)
pub enum ShiftKind { Lsl, Lsr, Asr, Ror }

// Comparison kinds (Hexagon)
pub enum CmpKind { Eq, Gt, Gtu, Ne, Lte, Lteu }

// SIMD vector types (x86)
pub type Xmm = [u64; 2];   // 128-bit
pub type Ymm = [Xmm; 2];   // 256-bit
pub type Zmm = [Xmm; 4];   // 512-bit
```

### 3. ArmCpu Trait - CPU Abstraction

```rust
pub trait ArmCpu: Send {
    fn step(&mut self) -> Result<CpuExit>;
    fn get_gpr(&self, reg: u8) -> u64;
    fn set_gpr(&mut self, reg: u8, val: u64);
    fn read_memory(&mut self, addr: u64, size: usize) -> Result<Vec<u8>>;
    fn write_memory(&mut self, addr: u64, data: &[u8]) -> Result<()>;
    // ...
}
```

This demonstrates how to abstract CPU state access across different implementations.

### 4. Lazy Flag Evaluation (x86)

```rust
pub struct LazyFlags {
    pub op: LazyFlagOp,   // None/Add/Sub/Logic/Inc/Dec/...
    pub result: u64,
    pub src: u64,
    pub dst: u64,
    pub size: u8,
}
```

Flags are materialized on-demand when read. This pattern is essential for efficient flag handling in SMIR.

### 5. CpuState Enum - Multi-Arch State

```rust
pub enum CpuState {
    X86_64(X86_64CpuState),
    Aarch64(Aarch64CpuState),
    Aarch32(Aarch32CpuState),
    CortexM(CortexMCpuState),
    Hexagon(HexagonCpuState),
}
```

Shows how heterogeneous register files can be handled in a unified interface.

---

## Key Design Decisions

### 1. Flag Representation

| Approach | Pros | Cons |
|----------|------|------|
| Explicit flag ops every instruction | Clean semantics | Slower, more ops |
| Lazy flags (current x86) | Fast, fewer ops | Complex, arch-specific |
| **Hybrid (recommended)** | Best of both | Moderate complexity |

The IR should track flag producers and consumers, materializing lazily only when needed.

### 2. Conditional Execution (ARM)

ARM instructions can be conditionally executed based on PSTATE flags:

```asm
ADDEQ x0, x1, x2   ; Only executes if Z=1
```

**Options**:
1. Predicated IR ops: `SmirOp::AddCond { cond, dst, src1, src2 }`
2. Lower to branches: Split into conditional branch + unconditional op

**Recommendation**: Option 2 - simpler IR, easier optimization, matches how x86 handles conditions.

### 3. Register Representation

```rust
pub enum VReg {
    Virtual(u32),          // SSA-style for optimization
    Arch(ArchReg),         // Pinned to specific arch register
}

pub enum ArchReg {
    X86(X86Reg),
    Arm(ArmReg),
    Hexagon(HexagonReg),
}
```

Virtual registers enable SSA-based optimization; arch registers for ABI constraints and I/O.

### 4. Addressing Modes

Must support both x86 complexity and ARM simplicity:

```rust
pub enum Address {
    // Simple (ARM, Hexagon)
    Direct(VReg),
    BasePlusOffset { base: VReg, offset: i64 },
    
    // Complex (x86 SIB)
    BaseIndexScale { base: VReg, index: VReg, scale: u8, offset: i32 },
    
    // PC-relative (both)
    PcRelative(i64),
}
```

---

## Implementation Phases

### Phase 1: Core IR + Interpreter (2-3 weeks)
- Define `SmirOp`, `VReg`, `Address`, `SmirBlock` types
- Implement SMIR interpreter
- Lift Hexagon first (already closest to IR)
- Validate with existing Hexagon tests

### Phase 2: x86 Lifting (4-6 weeks)
- Implement `X86Lifter` converting inline decode to SMIR
- Start with integer ops: ADD, SUB, MOV, CMP, Jcc
- Implement flag tracking
- Integrate with decode cache for block-level caching

### Phase 3: ARM Lifting (3-4 weeks)
- Implement `ArmLifter` from `DecodedInsn`
- Map ARM mnemonics to SMIR ops
- Handle conditional execution lowering

### Phase 4: Optimization + JIT (ongoing)
- Dead flag elimination
- Constant propagation
- Basic block fusion
- Optional: Template-based JIT tier

---

## Challenges & Mitigations

| Challenge | Impact | Mitigation |
|-----------|--------|------------|
| x86 flag complexity | High | Lazy flags in IR, selective materialization |
| x86 variable encoding | Medium | Lift at decode time, cache SMIR blocks |
| ARM conditionals | Medium | Lower to branches |
| Performance | Medium | Start interpreted, add JIT tier |
| SIMD differences | Medium | Abstract vector ops, use simd_native pattern |
| Self-modifying code | Low | Reuse existing SMC detection + cache invalidation |

---

## Benefits

1. **Multi-arch from one IR** - Add RISC-V, MIPS, etc. with just a lifter
2. **Optimization opportunities** - Dead code/flag elim, inlining, constant folding
3. **JIT-ready structure** - SMIR can lower to native code
4. **Better testing** - Can inspect IR for correctness, differential testing
5. **Formal reasoning** - Cleaner semantics than raw machine code
6. **Unified tooling** - One debugger/tracer/profiler for all arches

---

## Conclusion

SMIR is highly feasible with the current RAX architecture. Key enablers:

1. **Hexagon `DecodedInsn`** is a working IR prototype
2. **simd_native** proves cross-platform dispatch works
3. **Shared types** (MemWidth, ShiftKind, etc.) already exist
4. **ArmCpu trait** shows CPU abstraction pattern
5. **Lazy flags** provide efficient flag handling template

The main work is in x86 lifting (complexity) and flag semantics (correctness), both tractable.

---

## References

- `src/backend/emulator/x86_64/decoder.rs` - x86 prefix decoding
- `src/backend/emulator/x86_64/simd_native.rs` - Cross-platform SIMD
- `src/backend/emulator/hexagon/decode.rs` - IR-like decoder
- `src/arm/decoder/mod.rs` - ARM structured decoder
- `src/arm/cpu_trait.rs` - CPU abstraction trait
