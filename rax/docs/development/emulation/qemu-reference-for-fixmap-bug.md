# QEMU x86-64 Emulation Reference for Fixmap Corruption Bug

This document provides an exhaustive technical reference of QEMU's x86-64 emulation implementation, specifically focused on the instruction execution path involved in the fixmap corruption bug. Use this as the authoritative reference when debugging the rax emulator.

**QEMU Source Location:** `/models/dev/qemu`
**Bug Location:** `ADD R13, [RIP+0x017fcc33]` at `RIP=0xffffffff812ac776`

---

## Table of Contents

1. [Bug Summary and Symptoms](#1-bug-summary-and-symptoms)
2. [Instruction Execution Pipeline Overview](#2-instruction-execution-pipeline-overview)
3. [REX Prefix Handling](#3-rex-prefix-handling)
4. [ModRM and SIB Byte Decoding](#4-modrm-and-sib-byte-decoding)
5. [RIP-Relative Addressing](#5-rip-relative-addressing)
6. [64-bit Memory Read Implementation](#6-64-bit-memory-read-implementation)
7. [ADD Instruction Implementation](#7-add-instruction-implementation)
8. [Register Writeback](#8-register-writeback)
9. [Condition Code Handling](#9-condition-code-handling)
10. [Debugging Strategy and Checklists](#10-debugging-strategy-and-checklists)
11. [File Reference Index](#11-file-reference-index)

---

## 1. Bug Summary and Symptoms

### 1.1 Crash Location

The emulator crashes during Linux kernel boot when `text_poke` uses a fixmap virtual address:

```
Function: text_poke_memcpy+0xa/0x20
Instruction: REP MOVSB (f3 a4)
Call stack:
  text_poke_memcpy
  __text_poke+0x192/0x3e0
  alternatives_smp_module_add+0x125/0x130
  alternative_instructions+0xf4/0x140
  arch_cpu_finalize_init+0x11b/0x170
  start_kernel+0x6fc/0x780
```

### 1.2 Corrupted Address Pattern

| Component | Expected | Actual | Status |
|-----------|----------|--------|--------|
| Bits 63-48 | `0xFFFF` | `0x0000` | **WRONG** |
| Bits 47-12 | fixmap base | garbage (TSC-like) | **WRONG** |
| Bits 11-0 | `0x209` (page offset) | `0x209` | CORRECT |

**Examples of corrupted addresses:**
- `0x00002ebc737a7209`
- `0x0000773d5589c209`
- `0x0000751c6a2c6209`
- `0x0000664bced2e209`

### 1.3 Corruption Trace

The corruption was traced backwards from the crash:

1. **Crash:** `text_poke_memcpy` uses `RDI=corrupted_address`
2. **RIP=0xffffffff812ac789:** `MOV RDI, R13` copies corrupted R13 to RDI
3. **RIP=0xffffffff812ac776:** `ADD R13, [RIP+0x017fcc33]` produces garbage in R13
4. Memory at `~0xffffffff82aa93b0` contains the corrupted value

### 1.4 Key Observations

1. **Page offset preserved:** Lower 12 bits (`0x209`) are always correct
2. **Upper bits zeroed:** Bits 48-63 are `0x0000` instead of `0xFFFF`
3. **Middle bits garbage:** Bits 12-47 contain random values resembling TSC
4. **Pattern suggests:** 32-bit truncation or incorrect value source

---

## 2. Instruction Execution Pipeline Overview

QEMU uses a multi-stage translation pipeline for x86 instructions:

### 2.1 Pipeline Stages

```
┌─────────────────────────────────────────────────────────────────┐
│                    QEMU TCG Translation Pipeline                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  1. PREFIX COLLECTION                                            │
│     └─> Accumulate REX, VEX, segment overrides, etc.            │
│                                                                  │
│  2. OPCODE DECODE                                                │
│     └─> Lookup instruction in opcode tables                      │
│     └─> Determine operand types and sizes                        │
│                                                                  │
│  3. MODRM/SIB DECODE                                             │
│     └─> Extract mod, reg, rm fields                              │
│     └─> Parse SIB if present                                     │
│     └─> Compute effective address (AddressParts)                 │
│                                                                  │
│  4. OPERAND LOADING                                              │
│     └─> gen_load() for each source operand                       │
│     └─> Memory operands: gen_op_ld_v()                           │
│     └─> Register operands: gen_op_mov_v_reg()                    │
│                                                                  │
│  5. INSTRUCTION EXECUTION                                        │
│     └─> Call gen_XXX() function (e.g., gen_ADD)                  │
│     └─> Perform computation on TCG temporaries                   │
│                                                                  │
│  6. RESULT WRITEBACK                                             │
│     └─> gen_writeback() to destination                           │
│     └─> Memory: gen_op_st_v()                                    │
│     └─> Register: gen_op_mov_reg_v()                             │
│                                                                  │
│  7. FLAGS UPDATE                                                 │
│     └─> Store operands for lazy evaluation                       │
│     └─> set_cc_op() with operation type                          │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### 2.2 Key Data Structures

#### DisasContext (translate.c:85-144)

```c
typedef struct DisasContext {
    DisasContextBase base;

    // Prefix state
    int prefix;              // Accumulated prefix bits (PREFIX_REX, etc.)
    int override;            // Segment override (-1 if none)

    // REX prefix fields (x86-64 only)
    uint8_t rex_r;           // REX.R bit (extends ModRM reg)
    uint8_t rex_x;           // REX.X bit (extends SIB index)
    uint8_t rex_b;           // REX.B bit (extends ModRM r/m or SIB base)
    bool vex_w;              // REX.W or VEX.W (64-bit operand size)

    // Operand and address size
    MemOp dflag;             // Default operand size (MO_16/32/64)
    MemOp aflag;             // Address size (MO_16/32/64)

    // Temporaries for computation
    TCGv T0, T1;             // General-purpose temporaries
    TCGv A0;                 // Address temporary
    TCGv cc_srcT;            // Condition code source

    // Instruction tracking
    target_ulong pc;         // Current program counter
    int rip_offset;          // Offset for RIP-relative calculations
    target_ulong pc_save;    // Saved PC for translation block

    // CPU state
    uint32_t flags;          // TB flags (from hflags)
    int cpl;                 // Current privilege level
    int mem_index;           // MMU index for memory accesses
} DisasContext;
```

#### X86DecodedInsn (decode-new.h:305-345)

```c
typedef struct X86DecodedInsn {
    X86OpEntry e;              // Opcode entry with metadata
    X86DecodedOp op[3];        // Up to 3 operands
    target_ulong immediate;    // Rightmost immediate value
    AddressParts mem;          // Memory addressing components

    // Condition code tracking
    TCGv cc_dst;               // Destination for CC
    TCGv cc_src;               // Source for CC
    TCGv cc_src2;              // Second source for CC
    TCGv_i32 cc_op_dynamic;    // Dynamic CC operation
    int8_t cc_op;              // Static CC operation

    uint8_t b;                 // Main opcode byte
} X86DecodedInsn;
```

#### AddressParts (decode-new.h:322-328)

```c
typedef struct AddressParts {
    int def_seg;            // Default segment register (R_DS or R_SS)
    int base;               // Base register (0-15) or special: -1=none, -2=RIP
    int index;              // Index register (0-15) or -1 if none
    int scale;              // Scale factor (0=1x, 1=2x, 2=4x, 3=8x)
    target_long disp;       // Displacement (sign-extended)
} AddressParts;
```

---

## 3. REX Prefix Handling

### 3.1 REX Byte Structure

The REX prefix (0x40-0x4F) extends register addressing in x86-64:

```
REX Byte: 0100 WRXB
          ││││ ││││
          ││││ │││└─ B: Extension of ModRM r/m, SIB base, or opcode reg
          ││││ ││└── X: Extension of SIB index
          ││││ │└─── R: Extension of ModRM reg
          ││││ └──── W: 64-bit operand size
          │││└────── Fixed: 0
          ││└─────── Fixed: 1
          │└──────── Fixed: 0
          └───────── Fixed: 0
```

**Example: REX prefix 0x49**
```
0x49 = 0100 1001
       ││││ ││││
       ││││ │││└─ B = 1 (extends r/m to R8-R15)
       ││││ ││└── X = 0 (no SIB index extension)
       ││││ │└─── R = 0 (no reg extension)
       ││││ └──── W = 1 (64-bit operand)
```

### 3.2 QEMU REX Parsing

**Location:** `/models/dev/qemu/target/i386/tcg/decode-new.c.inc:2599-2608`

```c
case 0x40 ... 0x4f:
    if (CODE64(s)) {
        /*
         * REX prefix; ignored unless it is the last prefix, so
         * for now just stash it
         */
        rex = b;
        goto next_byte_rex;
    }
    break;
```

**REX Bit Extraction:** `decode-new.c.inc:2680-2686`

```c
if (rex != -1) {
    s->prefix |= PREFIX_REX;
    s->vex_w = (rex >> 3) & 1;      // REX.W -> bit 3
    s->rex_r = (rex & 0x4) << 1;    // REX.R (bit 2) -> stored shifted to bit 3
    s->rex_x = (rex & 0x2) << 2;    // REX.X (bit 1) -> stored shifted to bit 3
    s->rex_b = (rex & 0x1) << 3;    // REX.B (bit 0) -> stored shifted to bit 3
}
```

**Critical Detail:** REX.R, REX.X, REX.B are pre-shifted to bit position 3 (value 0x8 when set). This allows direct OR-ing with the 3-bit register field.

### 3.3 REX Macros

**Location:** `/models/dev/qemu/target/i386/tcg/translate.c:214-226`

```c
#ifdef TARGET_X86_64
#define REX_PREFIX(S)  (((S)->prefix & PREFIX_REX) != 0)
#define REX_W(S)       ((S)->vex_w)
#define REX_R(S)       ((S)->rex_r + 0)   // Returns 0 or 0x8
#define REX_X(S)       ((S)->rex_x + 0)   // Returns 0 or 0x8
#define REX_B(S)       ((S)->rex_b + 0)   // Returns 0 or 0x8
#else
#define REX_PREFIX(S)  false
#define REX_W(S)       false
#define REX_R(S)       0
#define REX_X(S)       0
#define REX_B(S)       0
#endif
```

### 3.4 Register Selection with REX.B

For R13 access in `ADD R13, [mem]`:

```c
// ModRM r/m field provides bits [2:0]
int rm = modrm & 7;  // = 5 for R13

// REX.B provides bit [3]
int reg = rm | REX_B(s);  // = 5 | 0x8 = 13

// Access register
TCGv value = cpu_regs[reg];  // cpu_regs[13] = R13
```

### 3.5 REX.W and Operand Size

**Location:** `decode-new.c.inc:2689-2694`

```c
/*
 * In 64-bit mode, the default data size is 32-bit.  Select 64-bit
 * data with rex_w, and 16-bit data with 0x66; rex_w takes precedence
 * over 0x66 if both are present.
 */
s->dflag = (REX_W(s) ? MO_64 : s->prefix & PREFIX_DATA ? MO_16 : MO_32);
```

**Operand Size Priority:**
1. REX.W = 1 → **64-bit** (MO_64)
2. 0x66 prefix → **16-bit** (MO_16)
3. Default → **32-bit** (MO_32)

---

## 4. ModRM and SIB Byte Decoding

### 4.1 ModRM Byte Structure

```
ModRM Byte: [mod:2] [reg:3] [r/m:3]
            [7:6]   [5:3]   [2:0]

mod field (bits 7-6):
  00 = Memory operand, no displacement (or special cases)
  01 = Memory operand, 8-bit signed displacement
  10 = Memory operand, 32-bit displacement
  11 = Register operand (no memory access)

reg field (bits 5-3):
  Register number or opcode extension (depends on instruction)
  Extended by REX.R in 64-bit mode

r/m field (bits 2-0):
  Register number or memory addressing mode
  Extended by REX.B in 64-bit mode
```

### 4.2 SIB Byte Structure

When ModRM r/m = 4 (RSP encoding), a SIB byte follows:

```
SIB Byte: [scale:2] [index:3] [base:3]
          [7:6]     [5:3]     [2:0]

scale field (bits 7-6):
  00 = ×1
  01 = ×2
  10 = ×4
  11 = ×8

index field (bits 5-3):
  Index register (extended by REX.X)
  4 (RSP) = no index register

base field (bits 2-0):
  Base register (extended by REX.B)
  5 with mod=00 = no base, disp32 only
```

### 4.3 gen_lea_modrm_0() Implementation

**Location:** `/models/dev/qemu/target/i386/tcg/translate.c:1714-1835`

This function parses ModRM and SIB bytes into an `AddressParts` structure:

```c
static AddressParts gen_lea_modrm_0(CPUX86State *env, DisasContext *s,
                                    int modrm, bool is_vsib)
{
    int def_seg, base, index, scale, mod, rm;
    target_long disp;
    bool havesib;

    def_seg = R_DS;     // Default to data segment
    index = -1;         // No index by default
    scale = 0;
    disp = 0;

    // Extract ModRM fields
    mod = (modrm >> 6) & 3;    // Bits 7-6
    rm = modrm & 7;             // Bits 2-0
    base = rm | REX_B(s);       // Apply REX.B extension

    // mod=11 means register operand, not memory
    if (mod == 3) {
        goto done;
    }

    switch (s->aflag) {
    case MO_64:  // 64-bit addressing
    case MO_32:  // 32-bit addressing
        havesib = 0;

        // Check for SIB byte (rm=4 in 32/64-bit mode)
        if (rm == 4) {
            int code = x86_ldub_code(env, s);  // Load SIB byte
            scale = (code >> 6) & 3;           // SIB bits 7-6
            index = ((code >> 3) & 7) | REX_X(s);  // SIB bits 5-3 + REX.X

            // RSP (4) as index means no index register
            if (index == 4 && !is_vsib) {
                index = -1;
            }
            base = (code & 7) | REX_B(s);      // SIB bits 2-0 + REX.B
            havesib = 1;
        }

        // Handle displacement based on mod field
        switch (mod) {
        case 0:  // No displacement (with exceptions)
            if ((base & 7) == 5) {
                // Special case: base=5 with mod=0 means disp32-only
                base = -1;
                disp = (int32_t)x86_ldl_code(env, s);  // Sign-extended 32-bit

                // In 64-bit mode without SIB: RIP-relative addressing
                if (CODE64(s) && !havesib) {
                    base = -2;  // Special marker for RIP-relative
                    disp += s->pc + s->rip_offset;  // Pre-compute effective addr
                }
            }
            break;

        case 1:  // 8-bit signed displacement
            disp = (int8_t)x86_ldub_code(env, s);
            break;

        case 2:  // 32-bit displacement
            disp = (int32_t)x86_ldl_code(env, s);
            break;
        }

        // Stack segment default for EBP/ESP base
        if (base == R_EBP || base == R_ESP) {
            def_seg = R_SS;
        }
        break;

    case MO_16:  // 16-bit addressing (legacy, different encoding)
        // ... (omitted for brevity)
        break;
    }

 done:
    return (AddressParts){ def_seg, base, index, scale, disp };
}
```

### 4.4 Special Case: mod=00, rm=5

This encoding has different meanings depending on mode:

| Mode | Condition | Meaning |
|------|-----------|---------|
| 32-bit | mod=00, rm=5 | `[disp32]` - absolute address |
| 64-bit | mod=00, rm=5, no SIB | `[RIP+disp32]` - RIP-relative |
| 64-bit | mod=00, rm=5, with SIB | `[disp32]` - absolute (rare) |

**Detection in QEMU:**
```c
if ((base & 7) == 5) {
    base = -1;  // No base register
    disp = (int32_t)x86_ldl_code(env, s);

    if (CODE64(s) && !havesib) {
        base = -2;  // Mark as RIP-relative
        disp += s->pc + s->rip_offset;  // Compute full address
    }
}
```

### 4.5 Effective Address Generation

**Location:** `/models/dev/qemu/target/i386/tcg/translate.c:1838-1870`

```c
static TCGv gen_lea_modrm_1(DisasContext *s, AddressParts a, bool is_vsib)
{
    TCGv ea = NULL;

    // Handle index register with scaling
    if (a.index >= 0 && !is_vsib) {
        if (a.scale == 0) {
            ea = cpu_regs[a.index];
        } else {
            tcg_gen_shli_tl(s->A0, cpu_regs[a.index], a.scale);
            ea = s->A0;
        }
        if (a.base >= 0) {
            tcg_gen_add_tl(s->A0, ea, cpu_regs[a.base]);
            ea = s->A0;
        }
    }
    // Handle base register only
    else if (a.base >= 0) {
        ea = cpu_regs[a.base];
    }

    // Handle displacement only or RIP-relative
    if (!ea) {
        if (tb_cflags(s->base.tb) & CF_PCREL && a.base == -2) {
            // PC-relative translation block
            tcg_gen_addi_tl(s->A0, cpu_eip, a.disp - s->pc_save);
        } else {
            // Absolute displacement (or pre-computed RIP-relative)
            tcg_gen_movi_tl(s->A0, a.disp);
        }
        ea = s->A0;
    }
    // Handle base/index + displacement
    else if (a.disp != 0) {
        tcg_gen_addi_tl(s->A0, ea, a.disp);
        ea = s->A0;
    }

    return ea;
}
```

---

## 5. RIP-Relative Addressing

### 5.1 Concept

In x86-64, the encoding `mod=00, rm=5` (without SIB) represents RIP-relative addressing:

```
Effective Address = RIP_next + sign_extend(disp32)

Where:
  RIP_next = address of the instruction AFTER the current one
  disp32   = 32-bit signed displacement from instruction stream
```

### 5.2 QEMU Implementation

**Detection:** `translate.c:1752-1760`

```c
if ((base & 7) == 5) {
    base = -1;
    disp = (int32_t)x86_ldl_code(env, s);  // Sign-extend 32→64 bits

    if (CODE64(s) && !havesib) {
        base = -2;  // Sentinel value for RIP-relative
        disp += s->pc + s->rip_offset;  // Pre-compute effective address
    }
}
```

**Key Points:**

1. **Sign Extension:** The displacement is cast to `(int32_t)` which automatically sign-extends to 64 bits when assigned to `target_long disp`.

2. **Next RIP Calculation:**
   - `s->pc` = current program counter (updated as bytes are consumed)
   - `s->rip_offset` = additional offset for bytes not yet consumed
   - After consuming the displacement, `s->pc + s->rip_offset` points to next instruction

3. **Pre-computation:** The effective address is computed during decode and stored in `disp`. The `base = -2` marker indicates this pre-computed RIP-relative address.

### 5.3 PC Tracking

**Location:** `translate.c:1663-1687`

```c
static uint64_t advance_pc(CPUX86State *env, DisasContext *s, int num_bytes)
{
    uint64_t pc = s->pc;

    s->pc += num_bytes;

    // Check for page boundary crossing
    if (s->pc - s->page_start > TARGET_PAGE_SIZE) {
        // ... boundary handling
    }

    return pc;
}
```

The `x86_ldub_code()`, `x86_ldl_code()` functions call `advance_pc()` to track instruction position:

```c
static inline uint32_t x86_ldl_code(CPUX86State *env, DisasContext *s)
{
    return translator_ldl(env, &s->base, advance_pc(env, s, 4));
}
```

### 5.4 Example: ADD R13, [RIP+0x017fcc33]

For the instruction at `RIP=0xffffffff812ac776`:

```
Instruction bytes: 4C 03 2D 33 CC 7F 01
                   │  │  │  └──────────── disp32 = 0x017fcc33
                   │  │  └─────────────── ModRM = 0x2D (mod=00, reg=5, rm=5)
                   │  └────────────────── Opcode = 0x03 (ADD Gv, Ev)
                   └───────────────────── REX.W + REX.B = 0x4C

Instruction length: 7 bytes
Next RIP = 0xffffffff812ac776 + 7 = 0xffffffff812ac77d

Effective address = Next_RIP + sign_extend(0x017fcc33)
                  = 0xffffffff812ac77d + 0x00000000017fcc33
                  = 0xffffffff82aa93b0
```

### 5.5 Comparison with rax Implementation

**rax decoder.rs:207-230:**

```rust
} else if rm_field == 5 && mod_bits == 0 {
    // RIP-relative addressing
    let disp = i32::from_le_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]) as i64;
    extra += 4;

    if self.sregs.cs.l {
        // 64-bit mode: RIP-relative
        let rip_after = self.regs.rip as i64
            + modrm_offset as i64
            + 1                           // ModRM byte
            + 4                           // disp32
            + ctx.rip_relative_offset as i64;  // Additional bytes (immediate, etc.)
        addr = rip_after.wrapping_add(disp) as u64;
    } else {
        // Compatibility mode: absolute
        addr = disp as u64;
    }
}
```

**Critical Check:** Ensure `ctx.rip_relative_offset` is set correctly by the instruction handler before calling `decode_modrm()`. This value must account for any immediate bytes that follow the memory operand.

---

## 6. 64-bit Memory Read Implementation

### 6.1 Memory Read Entry Point

**Location:** `/models/dev/qemu/target/i386/tcg/translate.c:526-529`

```c
static inline void gen_op_ld_v(DisasContext *s, int idx, TCGv t0, TCGv a0)
{
    tcg_gen_qemu_ld_tl(t0, a0, s->mem_index, idx | MO_LE);
}
```

**Parameters:**
- `s` - Disassembly context
- `idx` - Operand size (MO_8, MO_16, MO_32, MO_64)
- `t0` - Destination TCG variable
- `a0` - Source address TCG variable

### 6.2 MemOp Flags

**Location:** `/models/dev/qemu/include/exec/memop.h:17-163`

```c
typedef enum MemOp {
    // Size encoding
    MO_8     = 0,    // 1 byte
    MO_16    = 1,    // 2 bytes
    MO_32    = 2,    // 4 bytes
    MO_64    = 3,    // 8 bytes
    MO_128   = 4,    // 16 bytes
    MO_SIZE  = 0x07, // Mask for size bits

    // Sign extension
    MO_SIGN  = 0x08, // Sign-extend smaller values

    // Byte ordering
    MO_BSWAP = 0x10, // Byte-swap flag
#if HOST_BIG_ENDIAN
    MO_LE    = MO_BSWAP,  // Little-endian requires swap on BE host
    MO_BE    = 0,
#else
    MO_LE    = 0,         // Little-endian is native on LE host
    MO_BE    = MO_BSWAP,
#endif

    // Atomicity requirements
    MO_ATOM_IFALIGN       = 0 << MO_ATOM_SHIFT,
    MO_ATOM_IFALIGN_PAIR  = 1 << MO_ATOM_SHIFT,
    MO_ATOM_WITHIN16      = 2 << MO_ATOM_SHIFT,
    MO_ATOM_WITHIN16_PAIR = 3 << MO_ATOM_SHIFT,
    MO_ATOM_SUBALIGN      = 4 << MO_ATOM_SHIFT,
    MO_ATOM_NONE          = 5 << MO_ATOM_SHIFT,
} MemOp;

// Size calculation
static inline unsigned memop_size(MemOp op)
{
    return 1 << (op & MO_SIZE);  // MO_64 = 3 → 1 << 3 = 8 bytes
}
```

### 6.3 TCG Load Generation

**Location:** `/models/dev/qemu/tcg/tcg-op-ldst.c:351-415`

```c
static void tcg_gen_qemu_ld_i64_int(TCGv_i64 val, TCGTemp *addr,
                                    TCGArg idx, MemOp memop)
{
    MemOp orig_memop;
    MemOpIdx orig_oi, oi;

    // Handle 32-bit host loading values < 64 bits
    if (TCG_TARGET_REG_BITS == 32 && (memop & MO_SIZE) < MO_64) {
        tcg_gen_qemu_ld_i32_int(TCGV_LOW(val), addr, idx, memop);
        if (memop & MO_SIGN) {
            tcg_gen_sari_i32(TCGV_HIGH(val), TCGV_LOW(val), 31);
        } else {
            tcg_gen_movi_i32(TCGV_HIGH(val), 0);
        }
        return;
    }

    // Request memory ordering
    tcg_gen_req_mo(TCG_MO_LD_LD | TCG_MO_ST_LD);

    // Canonicalize memop
    orig_memop = memop = tcg_canonicalize_memop(memop, 1, 0);
    orig_oi = oi = make_memop_idx(memop, idx);

    // Handle byte-swap if target doesn't support it in hardware
    if ((memop & MO_BSWAP) && !tcg_target_has_memory_bswap(memop)) {
        memop &= ~MO_BSWAP;
        oi = make_memop_idx(memop, idx);
    }

    // Generate the load
    gen_ld_i64(val, addr_new, oi);

    // Apply byte-swap if needed
    if ((orig_memop ^ memop) & MO_BSWAP) {
        switch (orig_memop & MO_SIZE) {
        case MO_64:
            tcg_gen_bswap64_i64(val, val);
            break;
        // ... other sizes
        }
    }
}
```

### 6.4 TLB Lookup

**Location:** `/models/dev/qemu/accel/tcg/cputlb.c:1730-1794`

```c
static bool mmu_lookup(CPUState *cpu, vaddr addr, MemOpIdx oi,
                       uintptr_t ra, MMUAccessType type, MMULookupLocals *l)
{
    bool crosspage;
    vaddr last;
    int flags;

    l->memop = get_memop(oi);
    l->mmu_idx = get_mmuidx(oi);

    // Initialize page lookup
    l->page[0].addr = addr;
    l->page[0].size = memop_size(l->memop);  // For MO_64: 8 bytes
    l->page[1].addr = 0;
    l->page[1].size = 0;

    // Lookup first page
    mmu_lookup1(cpu, &l->page[0], l->memop, l->mmu_idx, type, ra);

    // Check for page boundary crossing
    last = addr + l->page[0].size - 1;  // For 64-bit: addr + 7
    crosspage = (addr ^ last) & TARGET_PAGE_MASK;

    if (likely(!crosspage)) {
        // Fast path: access within single page
        flags = l->page[0].flags;
        if (unlikely(flags & (TLB_WATCHPOINT | TLB_NOTDIRTY))) {
            mmu_watch_or_dirty(cpu, &l->page[0], type, ra);
        }
        if (unlikely(flags & TLB_BSWAP)) {
            l->memop ^= MO_BSWAP;
        }
    } else {
        // Slow path: access crosses page boundary
        vaddr addr1 = last & TARGET_PAGE_MASK;
        int size0 = addr1 - addr;
        l->page[1].size = l->page[0].size - size0;
        l->page[0].size = size0;

        // Lookup second page
        mmu_lookup1(cpu, &l->page[1], 0, l->mmu_idx, type, ra);

        // Merge flags
        flags = l->page[0].flags | l->page[1].flags;
    }

    return crosspage;
}
```

### 6.5 64-bit Load Execution

**Location:** `/models/dev/qemu/accel/tcg/cputlb.c:2377-2396`

```c
static uint64_t do_ld8_mmu(CPUState *cpu, vaddr addr, MemOpIdx oi,
                           uintptr_t ra, MMUAccessType access_type)
{
    MMULookupLocals l;
    bool crosspage;
    uint64_t ret;

    // Request memory ordering
    cpu_req_mo(cpu, TCG_MO_LD_LD | TCG_MO_ST_LD);

    // Perform TLB lookup
    crosspage = mmu_lookup(cpu, addr, oi, ra, access_type, &l);

    if (likely(!crosspage)) {
        // Single page load - fast path
        return do_ld_8(cpu, &l.page[0], l.mmu_idx, access_type, l.memop, ra);
    }

    // Cross-page load - slow path
    ret = do_ld_beN(cpu, &l.page[0], 0, l.mmu_idx, access_type, l.memop, ra);
    ret = do_ld_beN(cpu, &l.page[1], ret, l.mmu_idx, access_type, l.memop, ra);

    // Apply byte-swap for little-endian
    if ((l.memop & MO_BSWAP) == MO_LE) {
        ret = bswap64(ret);
    }
    return ret;
}
```

### 6.6 Single-Page 64-bit Load

```c
static uint64_t do_ld_8(CPUState *cpu, MMULookupPageData *p, int mmu_idx,
                        MMUAccessType type, MemOp memop, uintptr_t ra)
{
    uint64_t ret;

    if (unlikely(p->flags & TLB_MMIO)) {
        // MMIO access
        ret = do_ld_mmio_beN(cpu, p->full, 0, p->addr, 8, mmu_idx, type, ra);
        if ((memop & MO_BSWAP) == MO_LE) {
            ret = bswap64(ret);
        }
    } else {
        // RAM access - atomic load from host address
        ret = load_atom_8(cpu, ra, p->haddr, memop);
        if (memop & MO_BSWAP) {
            ret = bswap64(ret);
        }
    }
    return ret;
}
```

### 6.7 Potential Issues

1. **Cross-page loads are non-atomic:** Two separate reads are combined, which could expose partial data on exceptions.

2. **Byte order accumulation:** Cross-page loads accumulate in big-endian order, then swap at the end for little-endian.

3. **TLB resizing race:** If TLB is resized during multi-page lookup, stale entries may be used.

---

## 7. ADD Instruction Implementation

### 7.1 Instruction Entry

**Location:** `/models/dev/qemu/target/i386/tcg/decode-new.c.inc`

ADD instructions are defined in the opcode table:

```c
[0x01] = X86_OP_ENTRY2(ADD, E,v, G,v, lock),  // ADD r/m, reg
[0x03] = X86_OP_ENTRY2(ADD, G,v, E,v),        // ADD reg, r/m
```

### 7.2 gen_ADD() Function

**Location:** `/models/dev/qemu/target/i386/tcg/emit.c.inc:1259-1270`

```c
static void gen_ADD(DisasContext *s, X86DecodedInsn *decode)
{
    MemOp ot = decode->op[1].ot;  // Operand size from second operand

    if (s->prefix & PREFIX_LOCK) {
        // Atomic ADD for LOCK prefix
        tcg_gen_atomic_add_fetch_tl(s->T0, s->A0, s->T1,
                                    s->mem_index, ot | MO_LE);
    } else {
        // Normal ADD: T0 = T0 + T1
        tcg_gen_add_tl(s->T0, s->T0, s->T1);
    }

    // Set condition codes for ADD operation
    prepare_update2_cc(decode, s, CC_OP_ADDB + ot);
}
```

**Key Points:**
- Operand size is extracted from `decode->op[1].ot`
- For 64-bit: `ot = MO_64`
- Addition uses `tcg_gen_add_tl()` which performs full 64-bit addition
- No truncation occurs in the add operation itself

### 7.3 Operand Loading Pipeline

**Location:** `decode-new.c.inc:2867-2870`

```c
gen_load(s, &decode, 1, s->T0);      // Load destination (register) into T0
gen_load(s, &decode, 2, s->T1);      // Load source (memory) into T1
decode.e.gen(s, &decode);             // Execute gen_ADD()
gen_writeback(s, &decode, 0, s->T0); // Write result to destination
```

### 7.4 gen_load() Function

**Location:** `/models/dev/qemu/target/i386/tcg/emit.c.inc:242-316`

```c
static void gen_load(DisasContext *s, X86DecodedInsn *decode, int opn, TCGv v)
{
    X86DecodedOp *op = &decode->op[opn];

    switch (op->unit) {
    case X86_OP_INT:
        if (op->has_ea) {
            // Memory operand: Load from effective address
            if (v == s->T0 && decode->e.special == X86_SPECIAL_SExtT0) {
                gen_op_ld_v(s, op->ot | MO_SIGN, v, s->A0);
            } else {
                gen_op_ld_v(s, op->ot, v, s->A0);
            }
        } else {
            // Register operand: Load from register file
            gen_op_mov_v_reg(s, op->ot, v, op->n);
        }
        break;

    case X86_OP_IMM:
        tcg_gen_movi_tl(v, op->imm);
        break;

    // ... other cases (SSE, MMX, etc.)
    }
}
```

### 7.5 Register Load

**Location:** `/models/dev/qemu/target/i386/tcg/translate.c:483-490`

```c
void gen_op_mov_v_reg(DisasContext *s, MemOp ot, TCGv t0, int reg)
{
    if (ot == MO_8 && byte_reg_is_xH(s, reg)) {
        // High byte register (AH, BH, CH, DH)
        tcg_gen_shri_tl(t0, cpu_regs[reg - 4], 8);
    } else {
        // Full register access
        tcg_gen_mov_tl(t0, cpu_regs[reg]);
    }
}
```

For 64-bit register access:
- `tcg_gen_mov_tl(t0, cpu_regs[reg])` copies the full 64-bit value
- No truncation or extension occurs

---

## 8. Register Writeback

### 8.1 gen_writeback() Function

**Location:** `/models/dev/qemu/target/i386/tcg/emit.c.inc:337-376`

```c
static void gen_writeback(DisasContext *s, X86DecodedInsn *decode, int opn, TCGv v)
{
    X86DecodedOp *op = &decode->op[opn];

    switch (op->unit) {
    case X86_OP_INT:
        if (op->has_ea) {
            // Memory operand: Store to memory
            gen_op_st_v(s, op->ot, v, s->A0);
        } else {
            // Register operand: Store to register
            gen_op_mov_reg_v(s, op->ot, op->n, v);
        }
        break;

    // ... other cases
    }
}
```

### 8.2 gen_op_mov_reg_v() Function

**Location:** `/models/dev/qemu/target/i386/tcg/translate.c:477-480`

```c
static void gen_op_mov_reg_v(DisasContext *s, MemOp ot, int reg, TCGv t0)
{
    gen_op_deposit_reg_v(s, ot, reg, NULL, t0);
}
```

### 8.3 Core Writeback Logic: gen_op_deposit_reg_v()

**Location:** `/models/dev/qemu/target/i386/tcg/translate.c:443-475`

```c
static TCGv gen_op_deposit_reg_v(DisasContext *s, MemOp ot, int reg,
                                  TCGv dest, TCGv t0)
{
    switch(ot) {
    case MO_8:
        if (byte_reg_is_xH(s, reg)) {
            // High byte register (AH, BH, CH, DH)
            dest = dest ? dest : cpu_regs[reg - 4];
            tcg_gen_deposit_tl(dest, cpu_regs[reg - 4], t0, 8, 8);
            return cpu_regs[reg - 4];
        }
        // Low byte register
        dest = dest ? dest : cpu_regs[reg];
        tcg_gen_deposit_tl(dest, cpu_regs[reg], t0, 0, 8);
        break;

    case MO_16:
        // 16-bit: preserve upper 48 bits
        dest = dest ? dest : cpu_regs[reg];
        tcg_gen_deposit_tl(dest, cpu_regs[reg], t0, 0, 16);
        break;

    case MO_32:
        // 32-bit: ZERO-EXTEND to 64 bits (x86-64 behavior)
        dest = dest ? dest : cpu_regs[reg];
        tcg_gen_ext32u_tl(dest, t0);
        break;

#ifdef TARGET_X86_64
    case MO_64:
        // 64-bit: FULL REPLACEMENT
        dest = dest ? dest : cpu_regs[reg];
        tcg_gen_mov_tl(dest, t0);
        break;
#endif

    default:
        g_assert_not_reached();
    }
    return cpu_regs[reg];
}
```

### 8.4 Writeback Behavior Summary

| Operand Size | Operation | Upper Bits |
|--------------|-----------|------------|
| MO_8 | `tcg_gen_deposit_tl(..., 0, 8)` | Preserved |
| MO_16 | `tcg_gen_deposit_tl(..., 0, 16)` | Preserved |
| MO_32 | `tcg_gen_ext32u_tl()` | **Zeroed** |
| MO_64 | `tcg_gen_mov_tl()` | Replaced |

**Critical for Bug Analysis:**
- For 64-bit operations (MO_64), the writeback uses `tcg_gen_mov_tl()` which performs a complete 64-bit replacement
- If `t0` contains garbage, that garbage is written to the register
- The writeback itself is correct; corruption must occur earlier in the pipeline

### 8.5 Register Array

**Location:** `/models/dev/qemu/target/i386/tcg/translate.c:80`

```c
static TCGv cpu_regs[CPU_NB_REGS];  // 16 registers for x86-64
```

**Register Indices (from cpu.h:47-65):**

| Index | Register | Index | Register |
|-------|----------|-------|----------|
| 0 | RAX | 8 | R8 |
| 1 | RCX | 9 | R9 |
| 2 | RDX | 10 | R10 |
| 3 | RBX | 11 | R11 |
| 4 | RSP | 12 | R12 |
| 5 | RBP | 13 | **R13** |
| 6 | RSI | 14 | R14 |
| 7 | RDI | 15 | R15 |

---

## 9. Condition Code Handling

### 9.1 Lazy Evaluation

QEMU uses lazy evaluation for condition codes. The actual flags (CF, PF, AF, ZF, SF, OF) are not computed immediately; instead, the operands and operation type are stored for later computation.

### 9.2 prepare_update2_cc()

**Location:** `/models/dev/qemu/target/i386/tcg/emit.c.inc:398-403`

```c
static void prepare_update2_cc(X86DecodedInsn *decode, DisasContext *s, CCOp op)
{
    decode->cc_src = s->T1;    // Save source operand
    decode->cc_dst = s->T0;    // Save result
    decode->cc_op = op;        // Save operation type
}
```

For ADD with 64-bit operands:
- `op = CC_OP_ADDB + MO_64 = CC_OP_ADDQ`

### 9.3 Condition Code Operations

**Location:** `/models/dev/qemu/target/i386/cpu.h:1263-1310`

```c
typedef enum {
    CC_OP_DYNAMIC,    // Must compute from cc_op variable
    CC_OP_EFLAGS,     // All flags in cc_src
    CC_OP_MULB, CC_OP_MULW, CC_OP_MULL, CC_OP_MULQ,
    CC_OP_ADDB, CC_OP_ADDW, CC_OP_ADDL, CC_OP_ADDQ,  // ADD
    CC_OP_ADCB, CC_OP_ADCW, CC_OP_ADCL, CC_OP_ADCQ,  // ADC
    CC_OP_SUBB, CC_OP_SUBW, CC_OP_SUBL, CC_OP_SUBQ,  // SUB
    CC_OP_SBBB, CC_OP_SBBW, CC_OP_SBBL, CC_OP_SBBQ,  // SBB
    CC_OP_LOGICB, CC_OP_LOGICW, CC_OP_LOGICL, CC_OP_LOGICQ,
    CC_OP_INCB, CC_OP_INCW, CC_OP_INCL, CC_OP_INCQ,
    CC_OP_DECB, CC_OP_DECW, CC_OP_DECL, CC_OP_DECQ,
    // ... more operations
    CC_OP_NB,
} CCOp;
```

### 9.4 Flag Computation (When Needed)

**Location:** `/models/dev/qemu/target/i386/tcg/cc_helper_template.h.inc:68-73`

```c
static uint32_t glue(compute_all_add, SUFFIX)(DATA_TYPE dst, DATA_TYPE src1)
{
    DATA_TYPE src2 = dst - src1;  // Recover src2 from dst
    DATA_TYPE carries = ADD_COUT_VEC(src1, src2, dst);
    return glue(compute_all_cout, SUFFIX)(dst, carries);
}
```

Flags are computed from:
- `dst` = result of addition (stored in `cc_dst`)
- `src1` = source operand (stored in `cc_src`)
- `src2` = recovered as `dst - src1`

---

## 10. Debugging Strategy and Checklists

### 10.1 Root Cause Hypothesis

The corruption pattern (upper bits zeroed/garbage, lower 12 bits correct) suggests:

| Hypothesis | Likelihood | Evidence |
|------------|------------|----------|
| 32-bit truncation in address computation | **HIGH** | Upper 32 bits zeroed |
| Incorrect `rip_relative_offset` | **HIGH** | Wrong effective address |
| Memory read returning wrong data | MEDIUM | Garbage in middle bits |
| Wrong operand size (MO_32 vs MO_64) | MEDIUM | Would zero upper 32 bits |
| TSC value leaking into computation | LOW | Garbage resembles TSC |

### 10.2 Verification Checklist

#### REX Prefix Handling
- [ ] REX byte (0x40-0x4F) correctly identified in 64-bit mode
- [ ] REX.W extracted from bit 3 → forces 64-bit operand size
- [ ] REX.R extracted from bit 2 → extends ModRM reg field
- [ ] REX.X extracted from bit 1 → extends SIB index field
- [ ] REX.B extracted from bit 0 → extends ModRM r/m / SIB base
- [ ] REX fields stored as 0 or 0x8 (pre-shifted for OR-ing)

#### ModRM Decoding
- [ ] mod field extracted from bits 7-6
- [ ] reg field extracted from bits 5-3 (+ REX.R)
- [ ] r/m field extracted from bits 2-0 (+ REX.B)
- [ ] mod=00, rm=5 detected as RIP-relative in 64-bit mode (without SIB)
- [ ] SIB byte parsed when rm=4

#### RIP-Relative Addressing
- [ ] 32-bit displacement read as signed integer
- [ ] Displacement sign-extended to 64 bits
- [ ] Next RIP computed correctly: `instruction_start + instruction_length`
- [ ] `rip_relative_offset` accounts for all bytes after ModRM (immediates, etc.)
- [ ] Effective address = next_RIP + sign_extend(disp32)

#### 64-bit Memory Read
- [ ] Address is 64-bit (not truncated to 32 bits)
- [ ] All 8 bytes are read
- [ ] Byte order is little-endian
- [ ] TLB/page table lookup returns correct physical address
- [ ] Cross-page reads handled correctly

#### ADD Operation
- [ ] Operand size is MO_64 (not MO_32)
- [ ] Addition is 64-bit: `result = op1 + op2` (no truncation)
- [ ] No accidental 32-bit arithmetic

#### Register Writeback
- [ ] Operand size is MO_64
- [ ] Full 64-bit write (not deposit into lower bits)
- [ ] Correct register index (13 for R13)

### 10.3 Trace Points to Add

Add logging at these points in the rax emulator:

```rust
// 1. Before ADD execution
trace!("ADD R13,[mem]: RIP={:#x}, R13_before={:#x}", rip, regs.r13);
trace!("  Effective addr={:#x}", effective_addr);
trace!("  Memory value={:#x}", memory_value);

// 2. During RIP-relative computation
trace!("RIP-relative: RIP_start={:#x}, cursor={:#x}, rip_offset={:#x}",
       rip, cursor, rip_relative_offset);
trace!("  disp32={:#x} (signed: {})", disp32, disp32 as i32);
trace!("  next_RIP={:#x}", next_rip);
trace!("  effective_addr={:#x}", effective_addr);

// 3. After memory read
trace!("Memory read: addr={:#x}, value={:#x}, size={}", addr, value, size);

// 4. After ADD execution
trace!("ADD result: {:#x} + {:#x} = {:#x}", r13_before, mem_value, result);

// 5. After writeback
trace!("R13 after writeback: {:#x}", regs.r13);
```

### 10.4 Expected Values for the Bug Case

For `ADD R13, [RIP+0x017fcc33]` at `RIP=0xffffffff812ac776`:

| Value | Expected |
|-------|----------|
| Instruction bytes | `4C 03 2D 33 CC 7F 01` |
| REX prefix | `0x4C` (W=1, R=0, X=0, B=1) |
| Opcode | `0x03` (ADD Gv, Ev) |
| ModRM | `0x2D` (mod=00, reg=5, rm=5) |
| Displacement | `0x017fcc33` |
| Instruction length | 7 bytes |
| Next RIP | `0xffffffff812ac77d` |
| Effective address | `0xffffffff82aa93b0` |
| Register | R13 (index 13) |
| Operand size | 64-bit (MO_64) |

---

## 11. File Reference Index

### QEMU Source Files

| File | Lines | Key Contents |
|------|-------|--------------|
| `/models/dev/qemu/target/i386/tcg/translate.c` | 3932 | DisasContext, ModRM decode, register operations |
| `/models/dev/qemu/target/i386/tcg/decode-new.c.inc` | 2913 | Prefix parsing, opcode tables, instruction dispatch |
| `/models/dev/qemu/target/i386/tcg/decode-new.h` | 347 | X86DecodedInsn, AddressParts, operand types |
| `/models/dev/qemu/target/i386/tcg/emit.c.inc` | 4824 | Instruction implementations (gen_ADD, etc.) |
| `/models/dev/qemu/target/i386/tcg/cc_helper.c` | ~200 | Condition code helpers |
| `/models/dev/qemu/target/i386/tcg/cc_helper_template.h.inc` | ~150 | Flag computation templates |
| `/models/dev/qemu/target/i386/cpu.h` | 2900+ | CPU state, register indices, CCOp enum |
| `/models/dev/qemu/accel/tcg/cputlb.c` | 2900+ | TLB lookup, memory load/store |
| `/models/dev/qemu/include/exec/memop.h` | 163 | MemOp flags (MO_64, MO_LE, etc.) |

### Key Function Locations

| Function | File | Line |
|----------|------|------|
| `gen_lea_modrm_0()` | translate.c | 1714-1835 |
| `gen_lea_modrm_1()` | translate.c | 1838-1870 |
| `gen_op_ld_v()` | translate.c | 526-529 |
| `gen_op_mov_v_reg()` | translate.c | 483-490 |
| `gen_op_mov_reg_v()` | translate.c | 477-480 |
| `gen_op_deposit_reg_v()` | translate.c | 443-475 |
| `gen_ADD()` | emit.c.inc | 1259-1270 |
| `gen_load()` | emit.c.inc | 242-316 |
| `gen_writeback()` | emit.c.inc | 337-376 |
| `prepare_update2_cc()` | emit.c.inc | 398-403 |
| `do_ld8_mmu()` | cputlb.c | 2377-2396 |
| `mmu_lookup()` | cputlb.c | 1730-1794 |

### rax Emulator Files to Check

| File | Key Areas |
|------|-----------|
| `src/backend/emulator/x86_64/decoder.rs` | RIP-relative detection (lines 207-230), ModRM parsing |
| `src/backend/emulator/x86_64/cpu.rs` | `rip_relative_offset` field, register state |
| `src/backend/emulator/x86_64/insn/arith/add.rs` | ADD instruction implementation |
| `src/backend/emulator/x86_64/insn/data/mov.rs` | Memory operand handling, `rip_relative_offset` usage |

---

## Appendix A: Instruction Encoding Reference

### A.1 ADD R13, [RIP+disp32] Encoding

```
4C 03 2D 33 CC 7F 01
│  │  │  └──────────── disp32 = 0x017fcc33 (little-endian)
│  │  └─────────────── ModRM = 0x2D
│  │                   ├── mod = 00 (memory, no base displacement)
│  │                   ├── reg = 101 (5 → with REX.R=0: R13)
│  │                   └── r/m = 101 (5 → RIP-relative in 64-bit)
│  └────────────────── Opcode = 0x03 (ADD Gv, Ev)
└───────────────────── REX = 0x4C
                       ├── W = 1 (64-bit operand)
                       ├── R = 0 (reg field not extended)
                       ├── X = 0 (no SIB index extension)
                       └── B = 1 (r/m extended → selects R8-R15)
```

### A.2 REX Prefix Quick Reference

| REX Byte | W | R | X | B | Effect |
|----------|---|---|---|---|--------|
| 0x40 | 0 | 0 | 0 | 0 | Access R8-R15 in some encodings |
| 0x41 | 0 | 0 | 0 | 1 | r/m or base → R8-R15 |
| 0x42 | 0 | 0 | 1 | 0 | SIB index → R8-R15 |
| 0x44 | 0 | 1 | 0 | 0 | reg → R8-R15 |
| 0x48 | 1 | 0 | 0 | 0 | 64-bit operand |
| 0x49 | 1 | 0 | 0 | 1 | 64-bit + r/m R8-R15 |
| 0x4C | 1 | 1 | 0 | 0 | 64-bit + reg R8-R15 |
| 0x4D | 1 | 1 | 0 | 1 | 64-bit + both R8-R15 |

---

*Document generated from QEMU source analysis for debugging the rax emulator fixmap corruption bug.*
