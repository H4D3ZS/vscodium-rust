# x86 to ARM64 Translation in FEX

FEX is a user-space x86-64 emulator that runs x86 programs on ARM64 Linux systems. This document explains how FEX translates x86 instructions to ARM64 native code.

## Overview

FEX uses a four-stage translation pipeline:

```
x86 Binary → Frontend → OpcodeDispatcher → IR → Arm64JITCore → ARM64 Machine Code
```

This architecture separates concerns:
- The **Frontend** decodes x86 bytes and extracts metadata
- The **OpcodeDispatcher** translates x86 semantics to IR operations
- The **IR** provides an architecture-neutral representation for optimization
- The **JIT backend** generates native ARM64 code

## Stage 1: Frontend - x86 Decoding

**Location**: `FEXCore/Source/Interface/Core/Frontend.cpp`

The frontend's primary responsibility is to decode raw x86-64 instruction bytes and extract metadata for further processing.

### What the Frontend Does

1. **Instruction Decoding**: Reads x86-64 machine code bytes and interprets the encoding
2. **Metadata Extraction**: Extracts instruction length, operands, prefixes, and addressing modes
3. **Control Flow Analysis**: Identifies branch targets and block boundaries
4. **Multiblock Analysis**: Determines if multiple basic blocks can be compiled together (see `BranchTargetInMultiblockRange()`)

The Frontend does **not** perform semantic translation to IR - that is handled by the OpcodeDispatcher in Stage 2.

### Block Ending Analysis

The frontend identifies instructions that change control flow:
- Conditional/unconditional jumps (`jmp`, `je`, `jne`, etc.)
- Function returns (`ret`)
- System calls (`syscall`)
- Indirect branches (`jmp rax`, `call rax`)

This analysis enables **multiblock JIT compilation**, where FEX can compile multiple basic blocks in a single pass rather than compiling each block individually. This reduces dispatch overhead and improves performance.

### Multiblock Compilation Example

```asm
test eax, eax
jne .Continue
ret           ; Block ender - but we know it's unconditional
.Continue:
```

With multiblock analysis, FEX can compile past the `ret` instruction because the branch analysis determines that the code will only reach `ret` when `eax` is zero, and in that case the function returns.

## Stage 2: OpcodeDispatcher - x86 to IR Translation

**Location**: `FEXCore/Source/Interface/Core/OpcodeDispatcher.cpp`

The OpcodeDispatcher is the core translation layer that converts decoded x86 instructions into FEX's intermediate representation.

### What the OpcodeDispatcher Does

1. **Semantic Translation**: Converts x86 opcode semantics to equivalent IR operations
2. **Normalization**: Maps different encodings of the same operation to common IR ops
3. **Flag Handling**: Generates IR for x86 flag computations (ZF, CF, OF, SF, PF, AF)
4. **Addressing Modes**: Translates complex x86 addressing to IR memory operations

### Example: Normalization

x86 has multiple encodings for semantically identical operations:

```asm
00 C0    ; add al, al
04 01    ; add al, 0x01
```

Both instructions are simply "add", but they use different encoding schemes. The OpcodeDispatcher normalizes these to a common IR Add operation with appropriate source operands.

## Stage 3: Intermediate Representation (IR)

The IR is an SSA-based (Static Single Assignment) intermediate representation that serves as the bridge between x86 decoding and ARM64 code generation.

### Why SSA Form?

SSA form offers several advantages for emulation:
- **Def-use chains are explicit**: Each variable is assigned once, making data flow analysis straightforward
- **Optimization passes**: Common subexpression elimination, constant propagation, and dead code elimination are simpler
- **Architecture neutrality**: The IR doesn't encode host-specific details, making it easier to target different backends

### IR Features

#### Explicitly Sized Variables
The IR uses explicitly sized variables matching x86's operand sizes:
- 8-bit, 16-bit, 32-bit, 64-bit scalar types
- 128-bit and 256-bit vector types
- Floating-point and integer operations are distinct

#### Load/Store Operations
Memory access is handled through explicit IR ops:

```
LoadContext  $addr, size    ; Read from guest memory
StoreContext $value, $addr, size  ; Write to guest memory
```

These operations translate guest virtual addresses to host virtual addresses by adding the guest's memory base offset.

#### Control Flow
The IR supports:
- **Conditional branches**: `CondJump target, fallthrough`
- **Unconditional branches**: `Jump target`
- **Function returns**: `ExitFunction`

#### Special Operations
- **Syscall**: Emulates x86 syscalls with proper argument marshaling
- **CPUID**: Returns guest-visible CPU information
- **Atomic operations**: Handles x86's strong memory model requirements

### IR Example

A simple x86 function might translate to IR like this:

```
IRHeader <function_entry>
CodeBlock <block_start>, <block_end>, <next_block>
    BeginBlock
    %addr = Constant 0x4001000
    %value = LoadContext %addr, 8
    %result = Add %value, 1
    StoreContext %result, %addr, 8
    ExitFunction
    EndBlock
```

## Stage 4: Arm64JITCore - ARM64 Code Generation

**Location**: `FEXCore/Source/Interface/Core/JIT/`

The final stage walks the IR and emits native ARM64 machine code using FEX's custom `ARMEmitter` (the Vixl library is used only for simulation and disassembly, not code generation).

### What the JIT Does

1. **IR Interpretation**: Walks through the IR operations in order
2. **Register Allocation**: Maps IR variables to ARM64 registers (x0-x30) or stack slots
3. **Code Emission**: Emits ARM64 instructions that implement each IR operation
4. **Memory Model Handling**: Emulates x86's TSO (Total Store Order) memory model on ARM
5. **Code Patching**: Handles self-modifying code and JIT invalidation

### Register Mapping

FEX uses static register allocation for x86 general-purpose registers. The mapping is defined in `FEXCore/Source/Interface/Core/JIT/Arm64/Arm64Emitter.cpp`:

**x86 to ARM64 Register Mapping (x64 mode):**

| x86 Register | ARM64 Register | Notes |
|-------------|----------------|-------|
| RAX | x4 | Accumulator |
| RCX | x7 | Counter |
| RDX | x5 | Data |
| RBX | x6 | Base |
| RSP | x8 | Stack pointer |
| RBP | x9 | Base pointer |
| RSI | x10 | Source index |
| RDI | x11 | Destination index |
| R8 | x12 | Extended |
| R9 | x13 | Extended |
| R10 | x14 | Extended |
| R11 | x15 | Extended |
| R12 | x16 | Extended |
| R13 | x17 | Extended |
| R14 | x19 | Extended |
| R15 | x29 | Extended |

**Special Registers:**

| Purpose | ARM64 Register |
|---------|----------------|
| CPU State pointer | x28 |
| Parity Flag (PF) | x26 |
| Auxiliary Flag (AF) | x27 |
| Temporaries | x0-x3 |

Note: x86 syscall arguments are marshaled separately - the register mapping above is for general computation, not ABI compliance.

### Memory Model Emulation

x86 has a stronger memory model (Total Store Order) than ARM64. This means:

- **x86**: Memory operations have a defined order visible to all threads
- **ARM64**: Weaker ordering, allowing more reordering optimizations

FEX handles this through (documented in `FEXCore/docs/MemoryModelEmulation.md`):
- **Atomic load instructions**: Uses `ldapr`/`ldar` for TSO-compliant loads
- **Atomic store instructions**: Uses `stlr` for TSO-compliant stores
- **Half-barrier backpatching**: Initially uses atomic instructions, patches to barriers if unaligned access detected
- **FEAT_LRCPC/LRCPC2/LRCPC3**: Leverages newer ARM features when available for better performance
- **Vector barriers**: Uses `dmb` half-barriers for vector load/store operations

### Atomic Operations

x86 provides atomic instructions like `lock cmpxchg`. On ARM64, FEX generates equivalent code:

```asm
; x86: lock cmpxchg [rax], rcx
; ARM64 equivalent using LSE (Large System Extensions)
    CMP     x0, x1        ; Compare expected value
    B.NE    .fail
    CASP    x0, x1, x2, x3, [x4]  ; Atomic compare-and-swap
    CBZ     x0, .success
.fail:
    ; Handle failure
.success:
    ; Handle success
```

For older ARM64 without LSE, FEX uses LL/SC (load-linked/store-conditional) loops.

### Code Cache

FEX maintains a code cache that:
- Stores compiled IR → ARM64 translations
- Invalidates stale translations when code is modified
- Enables hot-path optimization (frequently executed code stays cached)

## Translation Example

Let's trace a complete example:

### Input x86 Code
```asm
add eax, ebx
ret
```

### Stage 1: Frontend Decoding
- Decode `01 D8` as ADD r32, r32
- Identify `ret` as a block-ending instruction

### Stage 2: IR Generation
```
%1 = GetRegister EAX
%2 = GetRegister EBX
%3 = Add %1, %2
%4 = SetRegister EAX, %3
ExitFunction
```

### Stage 4: ARM64 Code Generation
```asm
; FEX uses static register allocation: EAX→x4, EBX→x6
ADD     w4, w4, w6      ; x86 EAX = EAX + EBX (32-bit operation)
; Result stays in x4, state pointer in x28
; ... continue execution or exit to dispatcher
```

## Performance Considerations

### Why This Approach Works

1. **Decoding complexity handled once**: Frontend normalization means complex x86 decoding happens only once per translation
2. **IR enables optimizations**: Common subexpression elimination, constant folding, and dead code removal reduce code size
3. **Multiblock reduces overhead**: Fewer exits to the dispatcher means more code executes in native ARM
4. **Host-native code**: Generated ARM64 uses the full register set and pipeline capabilities of the CPU

### Overhead Sources

- **Guest state management**: Loading/storing x86 register state adds overhead
- **Memory translation**: Guest virtual addresses must be translated to host addresses
- **Memory barriers**: Ensuring x86 memory ordering on ARM adds latency
- **Exceptions/interrupts**: Handling signals and exceptions requires extra checks

### Configuration Options

FEX provides per-app configuration to tune emulation:
- **Multiblock**: Enable/disable multiblock JIT compilation
- **Memory model emulation**: Skip TSO emulation for faster single-threaded code
- **Code cache size**: Tune cache for application memory footprint

## Summary

FEX's translation pipeline follows a classic JIT emulator architecture:

1. **Frontend** decodes x86 bytes and extracts metadata
2. **OpcodeDispatcher** translates x86 semantics to IR operations
3. **IR** provides an optimizable, architecture-neutral representation
4. **JIT backend** generates efficient native ARM64 code

This separation allows FEX to focus on correctness and optimization at each stage while maintaining a modular architecture.

## Key Source Files

| Component | Location |
|-----------|----------|
| Frontend (Decoder) | `FEXCore/Source/Interface/Core/Frontend.cpp` |
| OpcodeDispatcher | `FEXCore/Source/Interface/Core/OpcodeDispatcher.cpp` |
| IR Definition | `FEXCore/Source/Interface/IR/IR.json` |
| Arm64 JIT | `FEXCore/Source/Interface/Core/JIT/` |
| ARM Emitter | `FEXCore/Source/Interface/Core/JIT/Arm64/Arm64Emitter.cpp` |
| Memory Model | `FEXCore/docs/MemoryModelEmulation.md` |
