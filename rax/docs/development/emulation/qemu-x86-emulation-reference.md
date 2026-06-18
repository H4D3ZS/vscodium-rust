# QEMU x86/x86_64 Emulation Architecture Reference

This document provides an extremely detailed technical reference for QEMU's x86/x86_64 emulation implementation, based on analysis of the QEMU source code.

**QEMU Source Location:** `/models/dev/qemu`

---

## Table of Contents

1. [CPU Register Setup](#1-cpu-register-setup)
2. [Memory Paging Implementation](#2-memory-paging-implementation)
3. [Instruction Emulation Architecture](#3-instruction-emulation-architecture)
4. [Emulation Testing Infrastructure](#4-emulation-testing-infrastructure)
5. [Register and Memory Validation](#5-register-and-memory-validation)
6. [Vendor-Specific CPU Behavior](#6-vendor-specific-cpu-behavior)

---

## 1. CPU Register Setup

### 1.1 Primary CPU State Structure

**File:** `/models/dev/qemu/target/i386/cpu.h` (lines 1846-2164)

The main CPU state is held in the `CPUX86State` structure (typedef'd as `CPUArchState`):

#### 1.1.1 General-Purpose Registers

```c
// Lines 1847-1852
target_ulong regs[CPU_NB_REGS];  // 16 registers for x86-64, 8 for i386
target_ulong eip;                // Instruction pointer
target_ulong eflags;             // Flags register
```

Register indexing enumeration (lines 47-73):

| Index | 32-bit Name | 64-bit Name |
|-------|-------------|-------------|
| 0 | R_EAX | R_RAX |
| 1 | R_ECX | R_RCX |
| 2 | R_EDX | R_RDX |
| 3 | R_EBX | R_RBX |
| 4 | R_ESP | R_RSP |
| 5 | R_EBP | R_RBP |
| 6 | R_ESI | R_RSI |
| 7 | R_EDI | R_RDI |
| 8-15 | N/A | R_R8-R_R15 |

#### 1.1.2 Emulator Internal Flags

```c
// Lines 1854-1862
target_ulong cc_dst;    // Condition code destination operand
target_ulong cc_src;    // Condition code source operand
target_ulong cc_src2;   // Second condition code source (for 3-operand ops)
uint32_t cc_op;         // Condition code operation type
int32_t df;             // Direction flag: 1 if D=0, -1 if D=1
uint32_t hflags;        // Hidden flags for translation block caching
uint32_t hflags2;       // Additional hidden CPU state flags
```

### 1.2 Segment Registers and Descriptors

**File:** `/models/dev/qemu/target/i386/cpu.h` (lines 1537-1542, 1864-1869)

#### 1.2.1 Segment Cache Structure

```c
// Lines 1537-1542
typedef struct SegmentCache {
    uint32_t selector;    // Visible segment selector
    target_ulong base;    // Cached base address from descriptor
    uint32_t limit;       // Cached limit from descriptor
    uint32_t flags;       // Cached descriptor flags
} SegmentCache;
```

#### 1.2.2 Segment Register Array

```c
// Lines 1864-1869
SegmentCache segs[6];     // CS, DS, ES, SS, FS, GS
SegmentCache ldt;         // Local Descriptor Table register
SegmentCache tr;          // Task Register
SegmentCache gdt;         // Global Descriptor Table (base/limit only)
SegmentCache idt;         // Interrupt Descriptor Table (base/limit only)
```

#### 1.2.3 Segment Register Enumeration

```c
// Lines 75-84
typedef enum X86Seg {
    R_ES = 0,
    R_CS = 1,
    R_SS = 2,
    R_DS = 3,
    R_FS = 4,
    R_GS = 5,
    R_LDTR = 6,
    R_TR = 7
} X86Seg;
```

#### 1.2.4 Descriptor Flag Masks

**File:** `/models/dev/qemu/target/i386/cpu.h` (lines 87-112)

| Mask | Bit | Description |
|------|-----|-------------|
| `DESC_G_MASK` | 23 | Granularity (4KB pages when set) |
| `DESC_B_MASK` | 22 | Default operation size (32-bit when set) |
| `DESC_L_MASK` | 21 | Long mode (64-bit code segment) |
| `DESC_P_MASK` | 15 | Present bit |
| `DESC_DPL_MASK` | 13-14 | Descriptor Privilege Level |
| `DESC_S_MASK` | 12 | Descriptor type (1=code/data, 0=system) |
| `DESC_CS_MASK` | 11 | Code segment (1) vs Data segment (0) |
| `DESC_R_MASK` | 9 | Readable (code) / Writable (data) |
| `DESC_A_MASK` | 8 | Accessed bit |

### 1.3 Control Registers

**File:** `/models/dev/qemu/target/i386/cpu.h` (lines 1871, 223-277)

```c
// Line 1871
target_ulong cr[5];  // CR0, CR1 (reserved), CR2, CR3, CR4
```

#### 1.3.1 CR0 Bit Definitions (lines 226-236)

| Mask | Bit | Description |
|------|-----|-------------|
| `CR0_PE_MASK` | 0 | Protected Mode Enable |
| `CR0_MP_MASK` | 1 | Monitor Coprocessor |
| `CR0_EM_MASK` | 2 | Emulation (no FPU present) |
| `CR0_TS_MASK` | 3 | Task Switched |
| `CR0_ET_MASK` | 4 | Extension Type (always 1 on 486+) |
| `CR0_NE_MASK` | 5 | Numeric Error |
| `CR0_WP_MASK` | 16 | Write Protect |
| `CR0_AM_MASK` | 18 | Alignment Mask |
| `CR0_NW_MASK` | 29 | Not Write-through |
| `CR0_CD_MASK` | 30 | Cache Disable |
| `CR0_PG_MASK` | 31 | Paging Enable |

#### 1.3.2 CR4 Bit Definitions (lines 238-267)

| Mask | Bit | Description |
|------|-----|-------------|
| `CR4_VME_MASK` | 0 | Virtual-8086 Mode Extensions |
| `CR4_PVI_MASK` | 1 | Protected-Mode Virtual Interrupts |
| `CR4_TSD_MASK` | 2 | Time Stamp Disable |
| `CR4_DE_MASK` | 3 | Debugging Extensions |
| `CR4_PSE_MASK` | 4 | Page Size Extension (4MB pages) |
| `CR4_PAE_MASK` | 5 | Physical Address Extension |
| `CR4_MCE_MASK` | 6 | Machine-Check Enable |
| `CR4_PGE_MASK` | 7 | Page Global Enable |
| `CR4_PCE_MASK` | 8 | Performance Counter Enable |
| `CR4_OSFXSR_MASK` | 9 | OS Support for FXSAVE/FXRSTOR |
| `CR4_OSXMMEXCPT_MASK` | 10 | OS Support for Unmasked SIMD Exceptions |
| `CR4_UMIP_MASK` | 11 | User-Mode Instruction Prevention |
| `CR4_LA57_MASK` | 12 | 5-Level Paging (57-bit linear addresses) |
| `CR4_FSGSBASE_MASK` | 16 | FSGSBASE Instructions Enable |
| `CR4_PCIDE_MASK` | 17 | PCID Enable |
| `CR4_OSXSAVE_MASK` | 18 | OS Support for XSAVE |
| `CR4_SMEP_MASK` | 20 | Supervisor Mode Execution Prevention |
| `CR4_SMAP_MASK` | 21 | Supervisor Mode Access Prevention |
| `CR4_PKE_MASK` | 22 | Protection Key Enable (User) |
| `CR4_PKS_MASK` | 24 | Protection Key Enable (Supervisor) |
| `CR4_FRED_MASK` | 32 | Flexible Return and Exception Delivery |

### 1.4 Model-Specific Registers (MSRs)

**File:** `/models/dev/qemu/target/i386/cpu.h` (lines 384-529, 1880-2060)

#### 1.4.1 Core MSRs in CPUX86State

```c
// Lines 1880-1938
uint64_t efer;              // Extended Feature Enable Register (0xC0000080)

// x86-64 Syscall MSRs
target_ulong lstar;         // Long Mode SYSCALL Target (0xC0000082)
target_ulong cstar;         // Compatibility Mode SYSCALL Target (0xC0000083)
target_ulong fmask;         // SYSCALL Flag Mask (0xC0000084)
target_ulong kernelgsbase;  // Kernel GS Base for SWAPGS (0xC0000102)

// SYSENTER MSRs
uint32_t sysenter_cs;       // SYSENTER CS (0x174)
target_ulong sysenter_esp;  // SYSENTER ESP (0x175)
target_ulong sysenter_eip;  // SYSENTER EIP (0x176)
uint64_t star;              // SYSCALL/SYSRET Segments (0xC0000081)
```

#### 1.4.2 EFER MSR Bits (lines 514-524)

| Mask | Bit | Description |
|------|-----|-------------|
| `MSR_EFER_SCE` | 0 | SYSCALL/SYSRET Enable |
| `MSR_EFER_LME` | 8 | Long Mode Enable |
| `MSR_EFER_LMA` | 10 | Long Mode Active |
| `MSR_EFER_NXE` | 11 | No-Execute Enable |
| `MSR_EFER_SVME` | 12 | Secure Virtual Machine Enable |
| `MSR_EFER_LMSLE` | 13 | Long Mode Segment Limit Enable |
| `MSR_EFER_FFXSR` | 14 | Fast FXSAVE/FXRSTOR |
| `MSR_EFER_TCE` | 15 | Translation Cache Extension |

#### 1.4.3 Memory Type Range Registers (MTRRs)

```c
// Lines 1995-2010
uint64_t mtrr_fixed[11];                    // Fixed-range MTRRs
uint64_t mtrr_deftype;                      // MTRR Default Type
MTRRVar mtrr_var[MSR_MTRRcap_VCNT];        // Variable-range MTRRs (8 pairs)
```

#### 1.4.4 Other Important MSRs

```c
// Lines 2015-2060
uint64_t tsc_adjust;              // TSC Adjust (0x3B)
uint64_t tsc_deadline;            // TSC Deadline (0x6E0)
uint64_t tsc_aux;                 // TSC Auxiliary (0xC0000103)
uint64_t xcr0;                    // Extended Control Register 0
uint64_t pat;                     // Page Attribute Table (0x277)
uint64_t msr_ia32_misc_enable;    // Miscellaneous Enable (0x1A0)
uint64_t msr_ia32_feature_control; // Feature Control (0x3A)
```

### 1.5 FPU and SIMD Registers

**File:** `/models/dev/qemu/target/i386/cpu.h` (lines 1631-1634, 1885-1913)

#### 1.5.1 FPU Register Union

```c
// Lines 1631-1634
typedef union {
    floatx80 d __attribute__((aligned(16)));  // 80-bit extended precision
    MMXReg mmx;                               // MMX overlay
} FPReg;
```

#### 1.5.2 FPU State

```c
// Lines 1885-1900
unsigned int fpstt;        // FPU stack top pointer (0-7)
uint16_t fpus;             // FPU Status Word
uint16_t fpuc;             // FPU Control Word
uint8_t fptags[8];         // Tag bits: 0=valid, 1=empty
FPReg fpregs[8];           // 8 x87 FPU registers (ST0-ST7)
uint16_t fpop;             // Last FPU opcode
uint64_t fpip, fpdp;       // Last instruction/data pointers

float_status fp_status;    // SoftFloat status
floatx80 ft0;              // Temporary FP register
```

#### 1.5.3 SIMD Registers

```c
// Lines 1902-1913
float_status mmx_status;   // MMX floating-point status
float_status sse_status;   // SSE floating-point status
uint32_t mxcsr;            // SSE Control/Status Register

// XMM/YMM/ZMM registers
ZMMReg xmm_regs[CPU_NB_REGS == 8 ? 8 : 32] QEMU_ALIGNED(16);
ZMMReg xmm_t0 QEMU_ALIGNED(16);  // Temporary vector register

// AVX-512 Opmask registers
uint64_t opmask_regs[NB_OPMASK_REGS];  // 8 opmask registers (k0-k7)

// MPX Bounds registers
BNDReg bnd_regs[4];        // BND0-BND3
BNDCSReg bndcs_regs;       // Bounds configuration/status
```

#### 1.5.4 Vector Register Unions

```c
// Lines 1544-1572
typedef union ZMMReg {
    uint8_t  _b_ZMMReg[64];     // 64 bytes
    uint16_t _w_ZMMReg[32];     // 32 words
    uint32_t _l_ZMMReg[16];     // 16 doublewords
    uint64_t _q_ZMMReg[8];      // 8 quadwords
    float16  _h_ZMMReg[32];     // 32 half-precision floats
    float32  _s_ZMMReg[16];     // 16 single-precision floats
    float64  _d_ZMMReg[8];      // 8 double-precision floats
    XMMReg   _x_ZMMReg[4];      // 4 x 128-bit XMM
    YMMReg   _y_ZMMReg[2];      // 2 x 256-bit YMM
} ZMMReg;
```

### 1.6 Debug Registers

**File:** `/models/dev/qemu/target/i386/cpu.h` (lines 2031-2035, 279-290)

```c
// Lines 2031-2035
target_ulong dr[8];  // DR0-DR7 (DR4, DR5 aliased to DR6, DR7)

// Breakpoint/watchpoint associations
union {
    struct CPUBreakpoint *cpu_breakpoint[4];  // For DR0-DR3
    struct CPUWatchpoint *cpu_watchpoint[4];
};
```

#### 1.6.1 Debug Register Flags (lines 279-290)

| Mask | Description |
|------|-------------|
| `DR6_BD` | Debug register access detected |
| `DR6_BS` | Single-step trap |
| `DR6_BT` | Task switch trap |
| `DR6_FIXED_1` | Fixed bits that must be 1 |
| `DR7_GD` | General detect enable |
| `DR7_FIXED_1` | Fixed bits that must be 1 |
| `DR7_TYPE_*` | Breakpoint condition (exec, write, I/O, R/W) |
| `DR7_LEN_*` | Breakpoint length (1, 2, 4, 8 bytes) |

### 1.7 Hidden Flags (hflags)

**File:** `/models/dev/qemu/target/i386/cpu.h` (lines 142-221)

These flags cache critical CPU state for fast translation block lookup:

| Flag | Bits | Description |
|------|------|-------------|
| `HF_CPL_MASK` | 0-1 | Current Privilege Level |
| `HF_INHIBIT_IRQ_MASK` | 3 | Disable IRQ for next instruction |
| `HF_CS32_MASK` | 4 | 32-bit code segment |
| `HF_SS32_MASK` | 5 | 32-bit stack segment |
| `HF_ADDSEG_MASK` | 6 | Add segment base to addresses |
| `HF_PE_MASK` | 7 | Protected mode (copy of CR0.PE) |
| `HF_MP_MASK` | 9 | Monitor coprocessor (CR0.MP) |
| `HF_EM_MASK` | 10 | Emulation mode (CR0.EM) |
| `HF_TS_MASK` | 11 | Task switched (CR0.TS) |
| `HF_LMA_MASK` | 14 | Long Mode Active (EFER.LMA) |
| `HF_CS64_MASK` | 15 | 64-bit code segment |
| `HF_OSFXSR_MASK` | 22 | CR4.OSFXSR enabled |
| `HF_SMAP_MASK` | 23 | CR4.SMAP enabled |
| `HF_UMIP_MASK` | 25 | CR4.UMIP enabled |
| `HF_AVX_EN_MASK` | 27 | AVX enabled |

### 1.8 CPU Reset and Initialization

**File:** `/models/dev/qemu/target/i386/cpu.c` (lines 8617-8788)

The `x86_cpu_reset_hold()` function initializes CPU state to architectural reset values:

```c
// Key reset values (lines 8617-8720)
env->eip = 0xfff0;                        // Reset vector
env->regs[R_EDX] = env->cpuid_version;    // EDX = CPUID version
env->eflags = 0x2;                        // Reserved bit always 1

// CR0 initialization
cpu_x86_update_cr0(env, 0x60000010);      // CD=1, NW=1, ET=1

// Segment registers (via cpu_x86_load_seg_cache)
// CS: selector=0xF000, base=0xFFFF0000, limit=0xFFFF
// All others: selector=0, base=0, limit=0xFFFF

// Descriptor table limits
env->idt.limit = 0xffff;
env->gdt.limit = 0xffff;
env->ldt.limit = 0xffff;
env->tr.limit = 0xffff;

// FPU initialization
for (i = 0; i < 8; i++) {
    env->fptags[i] = 1;  // All tags = empty
}
env->mxcsr = 0x1f80;     // Default MXCSR
env->xcr0 = XSTATE_FP_MASK;

// PAT initialization
env->pat = 0x0007040600070406ULL;

// Debug registers
memset(env->dr, 0, sizeof(env->dr));
env->dr[6] = DR6_FIXED_1;
env->dr[7] = DR7_FIXED_1;

// SMM base
env->smbase = 0x30000;
```

### 1.9 Segment Loading Function

**File:** `/models/dev/qemu/target/i386/cpu.h` (lines 2454-2525)

```c
static inline void cpu_x86_load_seg_cache(CPUX86State *env,
                                          X86Seg seg_reg,
                                          unsigned int selector,
                                          target_ulong base,
                                          unsigned int limit,
                                          unsigned int flags)
```

This function:
1. Updates the `SegmentCache` with selector, base, limit, and flags
2. Recalculates `hflags` for CS segment changes (32/64-bit mode detection)
3. Updates CPL from SS descriptor privilege level
4. Handles ADDSEG flag logic for address calculation

### 1.10 Control Register Update Functions

**File:** `/models/dev/qemu/target/i386/helper.c` (lines 135-240)

#### `cpu_x86_update_cr0()` (lines 135-173)
- Flushes TLB if PE, WP, or PG bits change
- Handles long mode entry/exit transitions
- Updates PE, MP, EM, TS flags in hflags
- Forces CR0_ET_MASK always set

#### `cpu_x86_update_cr4()` (lines 187-240)
- Flushes TLB if PGE, PAE, PSE, SMEP, SMAP, or LA57 change
- Updates HF_OSFXSR_MASK, HF_SMAP_MASK, HF_UMIP_MASK
- Validates CR4 bits against CPU features

---

## 2. Memory Paging Implementation

### 2.1 Page Table Walking

**File:** `/models/dev/qemu/target/i386/tcg/system/excp_helper.c` (lines 30-654)

#### 2.1.1 Main Translation Function

```c
// Lines 142-522
static bool mmu_translate(CPUX86State *env, const TranslateParams *in,
                          TranslateResult *out, TranslateFault *err)
```

This function implements complete x86 page table walking for all paging modes.

#### 2.1.2 Page Table Entry Reading

```c
// Lines 88-94: Read 32-bit PTE
static uint32_t ptw_ldl(const PTETranslate *in, hwaddr addr)

// Lines 96-102: Read 64-bit PTE
static uint64_t ptw_ldq(const PTETranslate *in, hwaddr addr)

// Lines 128-140: Atomic PTE update (Accessed/Dirty bits)
static inline bool ptw_setl(const PTETranslate *in, uint32_t old, uint32_t set)
{
    if (set & ~old) {
        uint32_t new = old | set;
        if (likely(in->haddr)) {
            old = cpu_to_le32(old);
            new = cpu_to_le32(new);
            return qatomic_cmpxchg((uint32_t *)in->haddr, old, new) == old;
        }
        return ptw_setl_slow(in, old, new);
    }
    return true;
}
```

#### 2.1.3 Page Table Level Handling

| Level | Lines | Mode | Entry Size | Address Bits |
|-------|-------|------|------------|--------------|
| 5 (PML5) | 172-195 | LA57 | 64-bit | 48-56 |
| 4 (PML4) | 198-215 | Long Mode | 64-bit | 39-47 |
| 3 (PDPT) | 218-264 | PAE/Long | 64-bit | 30-38 |
| 2 (PD) | 267-289 | All modes | 32/64-bit | 21-29 |
| 1 (PT) | 292-308 | All modes | 32/64-bit | 12-20 |

### 2.2 Paging Mode Detection

**File:** `/models/dev/qemu/target/i386/tcg/seg_helper.c` (lines 97-131)

```c
static int get_pg_mode(CPUX86State *env)
{
    int pg_mode = 0;

    if (env->cr[0] & CR0_PG_MASK) {
        pg_mode |= PG_MODE_PG;
        if (env->cr[0] & CR0_WP_MASK) {
            pg_mode |= PG_MODE_WP;
        }
        if (env->cr[4] & CR4_PAE_MASK) {
            pg_mode |= PG_MODE_PAE;
            if (env->efer & MSR_EFER_NXE) {
                pg_mode |= PG_MODE_NXE;
            }
        } else {
            if (env->cr[4] & CR4_PSE_MASK) {
                pg_mode |= PG_MODE_PSE;
            }
        }
        if (env->cr[4] & CR4_SMEP_MASK) {
            pg_mode |= PG_MODE_SMEP;
        }
        if (env->hflags & HF_LMA_MASK) {
            pg_mode |= PG_MODE_LMA;
            if (env->cr[4] & CR4_PKE_MASK) {
                pg_mode |= PG_MODE_PKE;
            }
            if (env->cr[4] & CR4_PKS_MASK) {
                pg_mode |= PG_MODE_PKS;
            }
            if (env->cr[4] & CR4_LA57_MASK) {
                pg_mode |= PG_MODE_LA57;
            }
        }
    }
    return pg_mode;
}
```

#### 2.2.1 Paging Mode Flags

| Flag | Bit | Source | Description |
|------|-----|--------|-------------|
| `PG_MODE_PG` | 0 | CR0.PG | Paging enabled |
| `PG_MODE_WP` | 1 | CR0.WP | Write protect in kernel mode |
| `PG_MODE_PAE` | 2 | CR4.PAE | Physical Address Extension |
| `PG_MODE_NXE` | 3 | EFER.NXE | No-Execute enabled |
| `PG_MODE_PSE` | 4 | CR4.PSE | Page Size Extension (4MB pages) |
| `PG_MODE_SMEP` | 5 | CR4.SMEP | Supervisor Mode Execution Prevention |
| `PG_MODE_SMAP` | 6 | CR4.SMAP | Supervisor Mode Access Prevention |
| `PG_MODE_LMA` | 7 | EFER.LMA | Long Mode Active |
| `PG_MODE_PKE` | 8 | CR4.PKE | Protection Keys for User pages |
| `PG_MODE_PKS` | 9 | CR4.PKS | Protection Keys for Supervisor pages |
| `PG_MODE_LA57` | 10 | CR4.LA57 | 5-level paging |

### 2.3 TLB Implementation

**File:** `/models/dev/qemu/include/exec/tlb-common.h` (lines 22-56)

#### 2.3.1 Fast-Path TLB Entry

```c
// Lines 25-41
typedef union CPUTLBEntry {
    struct {
        uintptr_t addr_read;    // Virtual address for read access
        uintptr_t addr_write;   // Virtual address for write access
        uintptr_t addr_code;    // Virtual address for code fetch
        uintptr_t addend;       // vaddr + addend = host address
    };
    uintptr_t addr_idx[4];      // Indexed access to above
} CPUTLBEntry;
```

#### 2.3.2 Full TLB Entry

**File:** `/models/dev/qemu/include/hw/core/cpu.h` (lines 215-271)

```c
typedef struct CPUTLBEntryFull {
    hwaddr phys_addr;          // Physical address of page
    MemTxAttrs attrs;          // Memory transaction attributes
    int prot;                  // Protection bits (PAGE_READ|WRITE|EXEC)
    int8_t lg_page_size;       // log2(page_size)
    uint8_t slow_flags;        // TLB_FORCE_SLOW, TLB_NOTDIRTY, etc.
} CPUTLBEntryFull;
```

#### 2.3.3 TLB Descriptor

```c
// Lines 49-54
typedef struct CPUTLBDescFast {
    uintptr_t mask;            // Size mask for direct indexing
    CPUTLBEntry *table;        // Array of TLB entries
} CPUTLBDescFast;
```

### 2.4 MMU Index Modes

**File:** `/models/dev/qemu/target/i386/cpu.h` (lines 2650-2657)

```c
#define MMU_KSMAP64_IDX    0   // Kernel with SMAP, 64-bit
#define MMU_KSMAP32_IDX    1   // Kernel with SMAP, 32-bit
#define MMU_USER64_IDX     2   // User mode, 64-bit
#define MMU_USER32_IDX     3   // User mode, 32-bit
#define MMU_KNOSMAP64_IDX  4   // Kernel without SMAP, 64-bit
#define MMU_KNOSMAP32_IDX  5   // Kernel without SMAP, 32-bit
#define MMU_PHYS_IDX       6   // Physical address (no translation)
#define MMU_NESTED_IDX     7   // Nested paging (NPT/EPT)
```

#### 2.4.1 MMU Index Helper Functions (lines 2667-2681)

```c
static inline bool is_mmu_index_user(int mmu_idx) {
    return (mmu_idx & 2) != 0;  // Bit 1 set = user mode
}

static inline bool is_mmu_index_smap(int mmu_idx) {
    return (mmu_idx & 4) == 0 && (mmu_idx & 2) == 0;
    // Bits 2,1 = 00 = kernel with SMAP
}

static inline bool is_mmu_index_32(int mmu_idx) {
    return mmu_idx & 1;  // Bit 0 set = 32-bit
}
```

### 2.5 TLB Management Functions

**File:** `/models/dev/qemu/accel/tcg/cputlb.c`

| Function | Line | Description |
|----------|------|-------------|
| `tlb_index()` | 127-133 | Calculate TLB index from virtual address |
| `tlb_entry()` | 136-140 | Get TLB entry pointer |
| `tlb_hit()` | 1226-1229 | Check if address matches TLB entry |
| `tlb_flush()` | 418 | Flush entire TLB |
| `tlb_flush_page()` | 106 | Flush single page from TLB |
| `tlb_set_page_with_attrs()` | 1185-1198 | Insert entry with memory attributes |
| `tlb_set_page_full()` | 1023-1183 | Full TLB entry insertion |

### 2.6 Page Fault Handling

**File:** `/models/dev/qemu/target/i386/tcg/system/excp_helper.c` (lines 613-654)

#### 2.6.1 TLB Fill Entry Point

```c
bool x86_cpu_tlb_fill(CPUState *cs, vaddr addr, int size,
                      MMUAccessType access_type, int mmu_idx,
                      bool probe, uintptr_t retaddr)
{
    CPUX86State *env = cpu_env(cs);
    TranslateResult out;
    TranslateFault err;

    if (get_physical_address(env, addr, access_type, mmu_idx, &out, &err, retaddr)) {
        // Success: insert TLB entry
        tlb_set_page_with_attrs(cs, addr & TARGET_PAGE_MASK,
                                out.paddr & TARGET_PAGE_MASK,
                                cpu_get_mem_attrs(env),
                                out.prot, mmu_idx, out.page_size);
        return true;
    }

    // Failure: raise page fault
    env->cr[2] = err.cr2;
    raise_exception_err_ra(env, err.exception_index, err.error_code, retaddr);
}
```

#### 2.6.2 TranslateFault Structure (lines 51-56)

```c
typedef struct TranslateFault {
    int exception_index;        // EXCP0E_PAGE for page faults
    int error_code;             // Page fault error code bits
    target_ulong cr2;           // Faulting linear address
    TranslateFaultStage2 stage2; // NPT fault stage indicator
} TranslateFault;
```

#### 2.6.3 Page Fault Error Codes

**File:** `/models/dev/qemu/target/i386/cpu.h` (lines 326-333)

| Mask | Bit | Description |
|------|-----|-------------|
| `PG_ERROR_P_MASK` | 0 | Page was present (protection violation) |
| `PG_ERROR_W_MASK` | 1 | Write access caused fault |
| `PG_ERROR_U_MASK` | 2 | User mode access caused fault |
| `PG_ERROR_RSVD_MASK` | 3 | Reserved bit set in PTE |
| `PG_ERROR_I_D_MASK` | 4 | Instruction fetch caused fault |
| `PG_ERROR_PK_MASK` | 5 | Protection key violation |

#### 2.6.4 Fault Generation Code (lines 488-521)

```c
do_fault_rsvd:
    error_code = PG_ERROR_RSVD_MASK;
    goto do_fault_cont;

do_fault_protect:
    error_code = PG_ERROR_P_MASK;
    goto do_fault_cont;

do_fault_pk_protect:
    error_code = PG_ERROR_PK_MASK | PG_ERROR_P_MASK;
    goto do_fault_cont;

do_fault:
    error_code = 0;  // Page not present

do_fault_cont:
    if (is_user) {
        error_code |= PG_ERROR_U_MASK;
    }
    switch (access_type) {
    case MMU_DATA_STORE:
        error_code |= PG_ERROR_W_MASK;
        break;
    case MMU_INST_FETCH:
        if (pg_mode & (PG_MODE_NXE | PG_MODE_SMEP)) {
            error_code |= PG_ERROR_I_D_MASK;
        }
        break;
    }
```

### 2.7 Page Table Entry Bit Definitions

**File:** `/models/dev/qemu/target/i386/cpu.h` (lines 298-324)

| Bit | Mask | Description |
|-----|------|-------------|
| 0 | `PG_PRESENT_MASK` | Page present in memory |
| 1 | `PG_RW_MASK` | Read/Write (0=read-only, 1=read-write) |
| 2 | `PG_USER_MASK` | User/Supervisor (0=supervisor, 1=user) |
| 3 | `PG_PWT_MASK` | Page-level Write-Through |
| 4 | `PG_PCD_MASK` | Page-level Cache Disable |
| 5 | `PG_ACCESSED_MASK` | Page has been accessed |
| 6 | `PG_DIRTY_MASK` | Page has been written |
| 7 | `PG_PSE_MASK` | Page Size (1=large page) |
| 8 | `PG_GLOBAL_MASK` | Global page (not flushed on CR3 write) |
| 12 | `PG_PSE_PAT_MASK` | PAT bit for large pages |
| 59-62 | `PG_PKRU_MASK` | Protection key bits |
| 63 | `PG_NX_MASK` | No-Execute bit |

#### 2.7.1 Address Extraction Masks

```c
#define PG_ADDRESS_MASK    0x000ffffffffff000ULL  // Physical address bits [51:12]
#define PG_HI_USER_MASK    0x7ff0000000000000ULL  // High user bits for PAE
```

### 2.8 Permission Checking

**File:** `/models/dev/qemu/target/i386/tcg/system/excp_helper.c` (lines 365-415)

```c
// Build protection bits
prot = 0;

// Check SMAP: skip read permission if kernel accessing user page with SMAP
if (!is_mmu_index_smap(in->mmu_idx) || !(ptep & PG_USER_MASK)) {
    prot |= PAGE_READ;

    // Write permission: needs RW bit OR (kernel mode AND NOT WP mode)
    if ((ptep & PG_RW_MASK) || !(is_user || (pg_mode & PG_MODE_WP))) {
        prot |= PAGE_WRITE;
    }
}

// Execute permission: NX bit must be clear
if (!(ptep & PG_NX_MASK) &&
    (is_user || !((pg_mode & PG_MODE_SMEP) && (ptep & PG_USER_MASK)))) {
    prot |= PAGE_EXEC;
}
```

#### 2.8.1 Protection Key Enforcement (lines 391-411)

```c
if (pkr) {
    uint32_t pk = (pte & PG_PKRU_MASK) >> PG_PKRU_BIT;  // Extract key (0-15)
    uint32_t pkr_ad = (pkr >> (pk * 2)) & 1;  // Access Disable bit
    uint32_t pkr_wd = (pkr >> (pk * 2)) & 2;  // Write Disable bit

    if (pkr_ad) {
        pkr_prot &= ~(PAGE_READ | PAGE_WRITE);  // Deny all access
    } else if (pkr_wd && (is_user || (pg_mode & PG_MODE_WP))) {
        pkr_prot &= ~PAGE_WRITE;  // Deny write only
    }

    if ((pkr_prot & (1 << access_type)) == 0) {
        goto do_fault_pk_protect;
    }
}
```

---

## 3. Instruction Emulation Architecture

### 3.1 Decoder Architecture Overview

QEMU uses a two-stage translation system:
1. **Decode Stage:** Parse instruction bytes into structured representation
2. **Emit Stage:** Generate TCG intermediate representation

#### 3.1.1 Key Source Files

| File | Lines | Purpose |
|------|-------|---------|
| `/models/dev/qemu/target/i386/tcg/translate.c` | 3932 | Main translator loop |
| `/models/dev/qemu/target/i386/tcg/decode-new.c.inc` | 2913 | Modern table-driven decoder |
| `/models/dev/qemu/target/i386/tcg/decode-new.h` | 347 | Decoder types and structures |
| `/models/dev/qemu/target/i386/tcg/emit.c.inc` | 4824 | TCG code generation |

### 3.2 DisasContext Structure

**File:** `/models/dev/qemu/target/i386/tcg/translate.c` (lines 85-144)

```c
typedef struct DisasContext {
    DisasContextBase base;

    // Prefix state
    int prefix;              // Accumulated prefix bits
    int override;            // Segment override (-1 if none)

    // REX prefix fields (x86-64)
    uint8_t rex_r;           // REX.R bit (extends ModRM reg)
    uint8_t rex_x;           // REX.X bit (extends SIB index)
    uint8_t rex_b;           // REX.B bit (extends ModRM r/m or SIB base)
    bool rex_w;              // REX.W bit (64-bit operand size)

    // VEX prefix fields
    uint8_t vex_l;           // VEX.L (vector length: 0=128, 1=256)
    uint8_t vex_v;           // VEX.vvvv (additional register operand)
    bool vex_w;              // VEX.W (operand size extension)

    // Operand and address size
    MemOp dflag;             // Default operand size (MO_16/32/64)
    MemOp aflag;             // Address size (MO_16/32/64)

    // ModRM state
    int modrm;               // ModRM byte value
    bool has_modrm;          // Whether ModRM has been fetched

    // Current CPU state
    uint32_t flags;          // TB flags (from hflags)
    int cpl;                 // Current privilege level
    int mem_index;           // MMU index for memory accesses

    // Temporaries
    TCGv T0, T1;             // General-purpose temporaries
    TCGv A0;                 // Address temporary
    TCGv cc_srcT;            // Condition code source
} DisasContext;
```

### 3.3 Prefix Handling

**File:** `/models/dev/qemu/target/i386/tcg/decode-new.c.inc` (lines 2552-2675)

#### 3.3.1 Prefix Bit Definitions

**File:** `/models/dev/qemu/target/i386/tcg/translate.c` (lines 46-52)

```c
#define PREFIX_REPZ   0x01    // F3 prefix (REP/REPE)
#define PREFIX_REPNZ  0x02    // F2 prefix (REPNE)
#define PREFIX_LOCK   0x04    // F0 prefix (LOCK)
#define PREFIX_DATA   0x08    // 66 prefix (operand size override)
#define PREFIX_ADR    0x10    // 67 prefix (address size override)
#define PREFIX_VEX    0x20    // VEX prefix present
#define PREFIX_REX    0x40    // REX prefix present (x86-64)
```

#### 3.3.2 Prefix Processing Loop

```c
// Lines 2552-2675
next_byte:
    b = x86_ldub_code(env, s);
    switch (b) {
    case 0xf3:
        s->prefix |= PREFIX_REPZ;
        goto next_byte;
    case 0xf2:
        s->prefix |= PREFIX_REPNZ;
        goto next_byte;
    case 0xf0:
        s->prefix |= PREFIX_LOCK;
        goto next_byte;
    case 0x2e: case 0x36: case 0x3e: case 0x26:
    case 0x64: case 0x65:
        s->override = (b >> 3) & 7;  // Segment override
        goto next_byte;
    case 0x66:
        s->prefix |= PREFIX_DATA;
        goto next_byte;
    case 0x67:
        s->prefix |= PREFIX_ADR;
        goto next_byte;
    case 0x40 ... 0x4f:  // REX prefix (x86-64 only)
        if (CODE64(s)) {
            s->prefix |= PREFIX_REX;
            s->rex_w = (b >> 3) & 1;
            s->rex_r = (b >> 2) & 1;
            s->rex_x = (b >> 1) & 1;
            s->rex_b = b & 1;
            goto next_byte;
        }
        break;
    // VEX handling follows...
    }
```

#### 3.3.3 REX Prefix Macros

**File:** `/models/dev/qemu/target/i386/tcg/translate.c` (lines 215-226)

```c
#define REX_PREFIX(S)  (((S)->prefix & PREFIX_REX) != 0)
#define REX_W(S)       ((S)->rex_w)
#define REX_R(S)       ((S)->rex_r)
#define REX_X(S)       ((S)->rex_x)
#define REX_B(S)       ((S)->rex_b)
```

#### 3.3.4 VEX Prefix Handling (lines 2610-2671)

```c
case 0xc5:  // 2-byte VEX
    if (!CODE64(s) && (peek & 0xc0) != 0xc0) {
        break;  // LDS instruction in 32-bit mode
    }
    vex2 = x86_ldub_code(env, s);
    s->prefix |= PREFIX_VEX;
    s->rex_r = (~vex2 >> 7) & 1;
    s->vex_v = (~vex2 >> 3) & 0xf;
    s->vex_l = (vex2 >> 2) & 1;
    // Implied 0F prefix
    b = x86_ldub_code(env, s) | 0x100;
    break;

case 0xc4:  // 3-byte VEX
    if (!CODE64(s) && (peek & 0xc0) != 0xc0) {
        break;  // LES instruction in 32-bit mode
    }
    vex2 = x86_ldub_code(env, s);
    vex3 = x86_ldub_code(env, s);
    s->prefix |= PREFIX_VEX;
    s->rex_r = (~vex2 >> 7) & 1;
    s->rex_x = (~vex2 >> 6) & 1;
    s->rex_b = (~vex2 >> 5) & 1;
    s->vex_w = (vex3 >> 7) & 1;
    s->vex_v = (~vex3 >> 3) & 0xf;
    s->vex_l = (vex3 >> 2) & 1;
    // m-mmmm field selects implied prefix
    switch (vex2 & 0x1f) {
    case 1: b = 0x100; break;    // 0F
    case 2: b = 0x200; break;    // 0F 38
    case 3: b = 0x300; break;    // 0F 3A
    }
    b |= x86_ldub_code(env, s);
    break;
```

### 3.4 ModRM Decoding

**File:** `/models/dev/qemu/target/i386/tcg/decode-new.c.inc` (lines 1896-1912)

```c
static void decode_modrm(DisasContext *s, X86DecodedInsn *decode,
                         X86DecodedOp *op, X86OpType type)
{
    int modrm = get_modrm(s, decode);

    if ((modrm >> 6) == 3) {
        // Register operand: mod=11
        op->n = (modrm & 7) | (REX_B(s) << 3);
        op->has_ea = false;
    } else {
        // Memory operand: mod=00, 01, or 10
        decode->mem = gen_lea_modrm_0(env, s, modrm);
        op->has_ea = true;
    }
}
```

#### 3.4.1 AddressParts Structure

**File:** `/models/dev/qemu/target/i386/tcg/decode-new.h` (lines 322-328)

```c
typedef struct AddressParts {
    int def_seg;            // Default segment register
    int base;               // Base register index (-1 if none)
    int index;              // Index register index (-1 if none)
    int scale;              // Scale factor (1, 2, 4, 8)
    target_long disp;       // Displacement value
} AddressParts;
```

#### 3.4.2 Address Calculation

**File:** `/models/dev/qemu/target/i386/tcg/translate.c` (lines 1714-1837)

The `gen_lea_modrm_0()` function parses ModRM and optional SIB bytes:

```c
static AddressParts gen_lea_modrm_0(CPUX86State *env, DisasContext *s, int modrm)
{
    int mod = (modrm >> 6) & 3;
    int rm = (modrm & 7) | REX_B(s);
    AddressParts result = { .def_seg = R_DS };

    if (s->aflag == MO_64) {
        // 64-bit addressing
        if (rm == 4) {
            // SIB byte follows
            int sib = x86_ldub_code(env, s);
            result.base = (sib & 7) | REX_B(s);
            result.index = ((sib >> 3) & 7) | REX_X(s);
            result.scale = (sib >> 6) & 3;
            if (result.index == 4) {
                result.index = -1;  // No index (RSP encoding)
            }
        } else if (rm == 5 && mod == 0) {
            // RIP-relative addressing
            result.base = -2;  // Special marker
            result.disp = (int32_t)x86_ldl_code(env, s);
        } else {
            result.base = rm;
        }
        // Handle displacement based on mod
        switch (mod) {
        case 0: break;
        case 1: result.disp = (int8_t)x86_ldub_code(env, s); break;
        case 2: result.disp = (int32_t)x86_ldl_code(env, s); break;
        }
    }
    // Similar logic for 32-bit and 16-bit addressing...

    return result;
}
```

### 3.5 Operand Type Enumeration

**File:** `/models/dev/qemu/target/i386/tcg/decode-new.h` (lines 22-70)

```c
typedef enum X86OpType {
    X86_TYPE_None,

    // Register selections (from ModRM reg field)
    X86_TYPE_A,      // Direct address (no ModRM)
    X86_TYPE_C,      // Control register (CRn)
    X86_TYPE_D,      // Debug register (DRn)
    X86_TYPE_G,      // General register (ModRM reg)
    X86_TYPE_S,      // Segment register
    X86_TYPE_P,      // MMX register (ModRM reg)
    X86_TYPE_V,      // XMM/YMM register (ModRM reg)
    X86_TYPE_H,      // XMM/YMM register (VEX.vvvv)

    // Memory or register (from ModRM r/m field)
    X86_TYPE_E,      // GPR or memory
    X86_TYPE_M,      // Memory only (mod != 11)
    X86_TYPE_W,      // XMM/YMM or memory
    X86_TYPE_N,      // MMX register (ModRM r/m, mod=11)
    X86_TYPE_U,      // XMM register (ModRM r/m, mod=11)
    X86_TYPE_R,      // GPR (ModRM r/m, mod=11)

    // Immediate values
    X86_TYPE_I,      // Immediate
    X86_TYPE_J,      // Relative offset (for jumps)
    X86_TYPE_O,      // Absolute offset (moffs)
    X86_TYPE_L,      // 8-bit immediate (high 4 bits = register)

    // Fixed registers
    X86_TYPE_2,      // Constant 2 (for shifts)
    X86_TYPE_F,      // EFLAGS register
    X86_TYPE_X,      // DS:rSI string source
    X86_TYPE_Y,      // ES:rDI string destination

    // ... more types
} X86OpType;
```

### 3.6 Operand Size Enumeration

**File:** `/models/dev/qemu/target/i386/tcg/decode-new.h** (lines 72-101)

```c
typedef enum X86OpSize {
    X86_SIZE_None,

    // Fixed sizes
    X86_SIZE_b,      // Byte (8-bit)
    X86_SIZE_w,      // Word (16-bit)
    X86_SIZE_d,      // Doubleword (32-bit)
    X86_SIZE_q,      // Quadword (64-bit)
    X86_SIZE_dq,     // Double-quadword (128-bit, XMM)
    X86_SIZE_qq,     // Quad-quadword (256-bit, YMM)
    X86_SIZE_x,      // 128 or 256 based on VEX.L
    X86_SIZE_y,      // 256 or 512 based on EVEX.L'L

    // Operand-size dependent
    X86_SIZE_v,      // 16/32/64 based on operand size
    X86_SIZE_z,      // 16 or 32 based on operand size (max 32)
    X86_SIZE_p,      // Far pointer (32:16 or 48:16 or 80:16)
    X86_SIZE_s,      // 6-byte or 10-byte descriptor

    // Addressing-size dependent
    X86_SIZE_a,      // 16:16 or 32:32 based on address size
} X86OpSize;
```

### 3.7 Decoded Instruction Structure

**File:** `/models/dev/qemu/target/i386/tcg/decode-new.h** (lines 305-345)

#### 3.7.1 X86DecodedOp

```c
typedef struct X86DecodedOp {
    int8_t n;              // Register number or operand selector
    MemOp ot;              // Operand size (MO_8/16/32/64/128/256)
    X86OpUnit unit;        // Unit type (GPR, SSE, MMX, CR, DR, SEG, IMM)
    bool has_ea;           // True if memory operand
    int offset;            // Offset into register file

    union {
        target_ulong imm;  // Immediate value
        TCGv_ptr v_ptr;    // Pointer to register (internal)
    };
} X86DecodedOp;
```

#### 3.7.2 X86DecodedInsn

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

### 3.8 TCG Code Generation

**File:** `/models/dev/qemu/target/i386/tcg/emit.c.inc** (4824 lines)

#### 3.8.1 Global TCG Variables

**File:** `/models/dev/qemu/target/i386/tcg/translate.c** (lines 77-83)

```c
static TCGv cpu_cc_dst, cpu_cc_src, cpu_cc_src2;
static TCGv_i32 cpu_cc_op;
static TCGv cpu_eip;
static TCGv cpu_regs[CPU_NB_REGS];
static TCGv cpu_seg_base[6];
static TCGv cpu_bndl[4], cpu_bndu[4];
```

#### 3.8.2 Example: ADD Instruction (lines 1259-1270)

```c
static void gen_ADD(DisasContext *s, X86DecodedInsn *decode)
{
    MemOp ot = decode->op[1].ot;

    if (s->prefix & PREFIX_LOCK) {
        // Atomic ADD for LOCK prefix
        tcg_gen_atomic_add_fetch_tl(s->T0, s->A0, s->T1,
                                    s->mem_index, ot | MO_LE);
    } else {
        // Normal ADD
        tcg_gen_add_tl(s->T0, s->T0, s->T1);
    }

    // Set condition codes for ADD operation
    prepare_update2_cc(decode, s, CC_OP_ADDB + ot);
}
```

#### 3.8.3 Operand Loading

```c
// Line 242
static void gen_load(DisasContext *s, X86DecodedInsn *decode,
                     int opn, TCGv dest)
{
    X86DecodedOp *op = &decode->op[opn];

    switch (op->unit) {
    case X86_OP_INT:
        if (op->has_ea) {
            // Load from memory
            gen_op_ld_v(s, op->ot, dest, s->A0);
        } else {
            // Load from register
            gen_op_mov_v_reg(s, op->ot, dest, op->n);
        }
        break;
    case X86_OP_IMM:
        tcg_gen_movi_tl(dest, op->imm);
        break;
    case X86_OP_SSE:
        gen_load_sse(s, dest, op);
        break;
    // ... other cases
    }
}
```

#### 3.8.4 Operand Writeback

```c
// Line 337
static void gen_writeback(DisasContext *s, X86DecodedInsn *decode,
                          int opn, TCGv val)
{
    X86DecodedOp *op = &decode->op[opn];

    if (op->has_ea) {
        // Write to memory
        gen_op_st_v(s, op->ot, val, s->A0);
    } else {
        // Write to register
        gen_op_mov_reg_v(s, op->ot, op->n, val);
    }
}
```

### 3.9 Condition Code Handling

**File:** `/models/dev/qemu/target/i386/tcg/emit.c.inc** (lines 392-420)

```c
// For single-operand operations (INC, DEC, NOT, NEG)
static void prepare_update1_cc(X86DecodedInsn *decode, DisasContext *s, CCOp op)
{
    decode->cc_dst = s->T0;
    decode->cc_op = op;
}

// For two-operand ALU operations (ADD, SUB, AND, OR, XOR, CMP)
static void prepare_update2_cc(X86DecodedInsn *decode, DisasContext *s, CCOp op)
{
    decode->cc_dst = s->T0;
    decode->cc_src = s->T1;
    decode->cc_op = op;
}

// For three-operand operations (ADC, SBB, shift with carry)
static void prepare_update3_cc(X86DecodedInsn *decode, DisasContext *s, CCOp op)
{
    decode->cc_dst = s->T0;
    decode->cc_src = s->T1;
    decode->cc_src2 = s->cc_srcT;
    decode->cc_op = op;
}
```

#### 3.9.1 Condition Code Operations

**File:** `/models/dev/qemu/target/i386/cpu.h** (lines 1263-1310)

```c
typedef enum {
    CC_OP_DYNAMIC,    // Must compute from cc_op variable
    CC_OP_EFLAGS,     // All flags in cc_src
    CC_OP_MULB, CC_OP_MULW, CC_OP_MULL, CC_OP_MULQ,  // MUL flags
    CC_OP_ADDB, CC_OP_ADDW, CC_OP_ADDL, CC_OP_ADDQ,  // ADD flags
    CC_OP_ADCB, CC_OP_ADCW, CC_OP_ADCL, CC_OP_ADCQ,  // ADC flags
    CC_OP_SUBB, CC_OP_SUBW, CC_OP_SUBL, CC_OP_SUBQ,  // SUB flags
    CC_OP_SBBB, CC_OP_SBBW, CC_OP_SBBL, CC_OP_SBBQ,  // SBB flags
    CC_OP_LOGICB, CC_OP_LOGICW, CC_OP_LOGICL, CC_OP_LOGICQ,  // Logic flags
    CC_OP_INCB, CC_OP_INCW, CC_OP_INCL, CC_OP_INCQ,  // INC flags
    CC_OP_DECB, CC_OP_DECW, CC_OP_DECL, CC_OP_DECQ,  // DEC flags
    CC_OP_SHLB, CC_OP_SHLW, CC_OP_SHLL, CC_OP_SHLQ,  // SHL flags
    CC_OP_SARB, CC_OP_SARW, CC_OP_SARL, CC_OP_SARQ,  // SAR flags
    CC_OP_BMILGB, CC_OP_BMILGW, CC_OP_BMILGL, CC_OP_BMILGQ,  // BMI flags
    CC_OP_POPCNT,     // POPCNT flags
    CC_OP_NB,         // Number of CC operations
} CCOp;
```

### 3.10 Instruction Translation Flow

1. **Entry Point:** `i386_tr_translate_insn()` (translate.c:3820)
2. **Decode:** `disas_insn()` (decode-new.c.inc:2529)
   - Initialize DisasContext
   - Collect prefixes (0x40-0x4F for REX, 0xC4/C5 for VEX)
   - Fetch opcode byte(s)
   - Look up in opcode tables
3. **Operand Decode:** `decode_op()` (decode-new.c.inc:2007)
   - Parse ModRM/SIB if needed
   - Extract immediates
   - Build X86DecodedOp structures
4. **Validation:** (lines 2240-2290)
   - Check CPUID feature requirements
   - Validate operand sizes
   - Check mode compatibility
5. **Code Generation:** (line 2865+)
   - Call gen_*() function from X86OpEntry
   - Generate TCG operations
   - Update condition codes
6. **Instruction Boundary:**
   - Update EIP
   - Commit condition code operation
   - Handle exceptions/interrupts

---

## 4. Emulation Testing Infrastructure

### 4.1 Test Directory Structure

```
/models/dev/qemu/tests/
├── tcg/                  # TCG instruction tests
│   ├── i386/             # x86 32-bit tests
│   ├── x86_64/           # x86 64-bit tests
│   └── multiarch/        # Cross-platform tests
├── qtest/                # QTest system tests
├── unit/                 # Unit tests
├── functional/           # Python functional tests
│   ├── i386/
│   └── x86_64/
├── decode/               # Decoder tests
└── data/                 # Test data (ACPI tables, etc.)
```

### 4.2 Instruction Reference Database

**File:** `/models/dev/qemu/tests/tcg/i386/x86.csv` (697 KB)

Machine-readable instruction database containing:
- Intel manual mnemonic
- Go assembler mnemonic
- GNU binutils mnemonic
- Instruction encoding
- 32-bit and 64-bit validity
- CPUID feature requirements
- Operand specifications

### 4.3 Unit Tests

#### 4.3.1 Arithmetic/Logic Tests

**File:** `/models/dev/qemu/tests/tcg/i386/test-i386.c** (59 KB)

Tests via inline assembly:

```c
// Example test macro from test-i386.h (lines 8-150)
#define EXECOP2(size, op, src, dst, eflags_in) \
    asm volatile( \
        "push %4\n" \
        "popf\n" \
        #op size " %3, %0\n" \
        "pushf\n" \
        "pop %1\n" \
        : "=q" (dst), "=r" (eflags_out) \
        : "0" (dst), "q" (src), "r" (eflags_in) \
        : "cc")
```

**Tested Instructions:**
- ADD, ADC, SUB, SBB, CMP
- AND, OR, XOR, NOT, NEG
- INC, DEC
- SHL, SHR, SAR, ROL, ROR, RCL, RCR
- MUL, IMUL, DIV, IDIV

**Test Values (lines 122-147):**
```c
static const long test_values[] = {
    0x12345678,
    0x812FADA,
    0x12341,
    0xffffffff,
    0x7fffffff,
    0x80000000,
    0x12345678ABCDEF01ULL,  // 64-bit only
    // ... more boundary values
};
```

#### 4.3.2 Floating Point Tests

| File | Instruction | Test Cases |
|------|-------------|------------|
| `test-i386-fpatan.c` (135KB) | FPATAN | 200+ reference values |
| `test-i386-fyl2x.c` (135KB) | FYL2X | Logarithm tests |
| `test-i386-fyl2xp1.c` (135KB) | FYL2XP1 | Log(1+x) tests |
| `test-i386-fprem.c` | FPREM | Remainder |
| `test-i386-f2xm1.c` (96KB) | F2XM1 | 2^x - 1 |
| `test-i386-fscale.c` | FSCALE | Scale by power of 2 |
| `test-i386-fxam.c` | FXAM | Classification |
| `test-i386-fbstp.c` | FBSTP | Packed BCD |
| `test-i386-fisttp.c` | FISTTP | Integer truncation |

Each file contains:
- Edge cases: infinity, zero, NaN, denormalized numbers
- Randomly generated test cases
- Down/up rounding bounds for precision validation

#### 4.3.3 BMI2 Tests

**File:** `/models/dev/qemu/tests/tcg/i386/test-i386-bmi2.c** (80+ lines)

```c
// Example assertions
assert(pext64(0x202020204f4c4845, 0x000000ff000000ff) == 0x4f45);
assert(pdep64(0x4f45, 0x000000ff000000ff) == 0x0000004f00000045);
assert(andn32(0xbce8a75c, 0xd4698c17) == 0x23010003);
assert(bextr32(0x5cf8a75c, 0x1410) == 0x5cf8);
assert(bzhi32(0x5cf8a75c, 20) == 0x8a75c);
```

#### 4.3.4 SIMD Test Generation

**Files:**
- `/models/dev/qemu/tests/tcg/i386/test-avx.py`
- `/models/dev/qemu/tests/tcg/i386/test-mmx.py`

These scripts parse `x86.csv` and generate:
- Test cases for all instruction variants
- Multiple register combinations
- Immediate value coverage
- Memory operand tests

Generated files:
- `test-avx.h`
- `test-mmx.h`
- `test-3dnow.h`

### 4.4 Integration Tests

#### 4.4.1 Boot Tests

**Files:**
- `/models/dev/qemu/tests/tcg/i386/system/boot.S` (3.3 KB)
- `/models/dev/qemu/tests/tcg/x86_64/system/boot.S` (6.2 KB)

Minimal bootloaders that:
1. Enter protected/long mode
2. Set up page tables
3. Execute test code
4. Report results via serial port

#### 4.4.2 ACPI Table Validation

**File:** `/models/dev/qemu/tests/qtest/bios-tables-test.c** (100+ lines)

Process:
1. Boot QEMU guest
2. Dump ACPI tables (DSDT, FACP, APIC, MCFG, etc.)
3. Compare against golden masters in `/models/dev/qemu/tests/data/acpi/x86/`
4. Use IASL to disassemble for human review

**File:** `/models/dev/qemu/tests/functional/x86_64/test_acpi_bits.py** (14 KB)

Uses biosbits framework for comprehensive ACPI/SMBIOS validation.

#### 4.4.3 CPU Model Tests

**Files:**
- `/models/dev/qemu/tests/functional/x86_64/test_cpu_model_versions.py` (11 KB)
- `/models/dev/qemu/tests/functional/x86_64/test_cpu_queries.py`
- `/models/dev/qemu/tests/qtest/cpu-plug-test.c` (80 lines)

Tests:
- CPUID feature flags
- CPU model versioning
- Hotplug functionality
- QMP protocol interaction

### 4.5 Test Execution Infrastructure

**File:** `/models/dev/qemu/tests/tcg/Makefile.target** (272 lines)

#### 4.5.1 Test Macros

```makefile
# Lines 57-62: Run test with timeout
run-test = $(call quiet-command, \
    timeout $(TIMEOUT) $(QEMU) $(1) 2>/dev/null, \
    "TEST", "$(2)")

# Lines 64-69: Compare output with reference
diff-out = $(call quiet-command, \
    diff -q $(1) $(1).ref, \
    "DIFF", "$(1)")
```

#### 4.5.2 i386-Specific Configuration

**File:** `/models/dev/qemu/tests/tcg/i386/Makefile.target** (103 lines)

```makefile
# CPU requirements for specific tests
TESTS_BMI2 = test-i386-bmi2
run-test-i386-bmi2: QEMU_OPTS += -cpu max

TESTS_SSE4 = test-i386-pcmpistri test-i386-sse-exceptions
run-test-i386-pcmpistri run-test-i386-sse-exceptions: QEMU_OPTS += -cpu max

# Reference generation (for floating point)
SPEED ?= normal
ifeq ($(SPEED),slow)
test-i386-fprem.ref: test-i386-fprem
	$(call quiet-command, ./$< > $@, "GENREF", "generating $@")
endif
```

### 4.6 Reference-Based Validation

Floating point tests use reference outputs:
1. Tests run on native x86 hardware (or known-good implementation)
2. Outputs saved as `.ref` files
3. QEMU test output compared using `diff`

---

## 5. Register and Memory Validation

### 5.1 Control Register Validation

#### 5.1.1 CR4 Reserved Bits

**File:** `/models/dev/qemu/target/i386/cpu.h** (lines 269-277)

```c
#define CR4_RESERVED_MASK \
    (~(target_ulong)(CR4_VME_MASK | CR4_PVI_MASK | CR4_TSD_MASK \
                   | CR4_DE_MASK | CR4_PSE_MASK | CR4_PAE_MASK \
                   | CR4_MCE_MASK | CR4_PGE_MASK | CR4_PCE_MASK \
                   | CR4_OSFXSR_MASK | CR4_OSXMMEXCPT_MASK | CR4_UMIP_MASK \
                   | CR4_LA57_MASK | CR4_FSGSBASE_MASK | CR4_PCIDE_MASK \
                   | CR4_OSXSAVE_MASK | CR4_SMEP_MASK | CR4_SMAP_MASK \
                   | CR4_PKE_MASK | CR4_PKS_MASK | CR4_LAM_SUP_MASK \
                   | CR4_FRED_MASK))
```

**File:** `/models/dev/qemu/target/i386/cpu.h** (lines 2887-2921)

```c
static inline uint64_t cr4_reserved_bits(CPUX86State *env)
{
    uint64_t reserved_bits = CR4_RESERVED_MASK;

    // Dynamically add reserved bits based on missing features
    if (!env->features[FEAT_XSAVE]) {
        reserved_bits |= CR4_OSXSAVE_MASK;
    }
    if (!(env->features[FEAT_7_0_EBX] & CPUID_7_0_EBX_SMEP)) {
        reserved_bits |= CR4_SMEP_MASK;
    }
    if (!(env->features[FEAT_7_0_EBX] & CPUID_7_0_EBX_SMAP)) {
        reserved_bits |= CR4_SMAP_MASK;
    }
    if (!(env->features[FEAT_7_0_EBX] & CPUID_7_0_EBX_FSGSBASE)) {
        reserved_bits |= CR4_FSGSBASE_MASK;
    }
    if (!(env->features[FEAT_7_0_ECX] & CPUID_7_0_ECX_PKU)) {
        reserved_bits |= CR4_PKE_MASK;
    }
    if (!(env->features[FEAT_7_0_ECX] & CPUID_7_0_ECX_PKS)) {
        reserved_bits |= CR4_PKS_MASK;
    }
    if (!(env->features[FEAT_7_0_ECX] & CPUID_7_0_ECX_LA57)) {
        reserved_bits |= CR4_LA57_MASK;
    }
    // ... more feature checks

    return reserved_bits;
}
```

#### 5.1.2 CR Write Handler

**File:** `/models/dev/qemu/target/i386/tcg/system/misc_helper.c** (lines 76-130)

```c
void helper_write_crN(CPUX86State *env, int reg, target_ulong t0)
{
    switch (reg) {
    case 0:  // CR0
        // SVM intercept for selective CR0 writes
        if (cpu_svm_has_intercept(env, SVM_EXIT_CR0_SEL_WRITE) &&
            ((env->cr[0] ^ t0) & ~(CR0_TS_MASK | CR0_MP_MASK))) {
            cpu_vmexit(env, SVM_EXIT_CR0_SEL_WRITE, 0, GETPC());
        }
        cpu_x86_update_cr0(env, t0);
        break;

    case 3:  // CR3
        // Validate physical address doesn't exceed phys_bits
        if ((env->efer & MSR_EFER_LMA) &&
            (t0 & ((~0ULL) << env_archcpu(env)->phys_bits))) {
            cpu_vmexit(env, SVM_EXIT_ERR, 0, GETPC());
        }
        cpu_x86_update_cr3(env, t0);
        break;

    case 4:  // CR4
        // Check reserved bits
        if (t0 & cr4_reserved_bits(env)) {
            cpu_vmexit(env, SVM_EXIT_ERR, 0, GETPC());
        }
        // Cannot change LA57 in 64-bit mode
        if (((t0 ^ env->cr[4]) & CR4_LA57_MASK) &&
            (env->hflags & HF_CS64_MASK)) {
            raise_exception_ra(env, EXCP0D_GPF, GETPC());
        }
        cpu_x86_update_cr4(env, t0);
        break;

    case 8:  // CR8 (TPR)
        if (t0 & ~0xfULL) {
            raise_exception_ra(env, EXCP0D_GPF, GETPC());
        }
        cpu_set_apic_tpr(env_archcpu(env)->apic_state, t0);
        break;
    }
}
```

### 5.2 MSR Validation

**File:** `/models/dev/qemu/target/i386/tcg/system/misc_helper.c** (lines 132-327)

```c
void helper_wrmsr(CPUX86State *env)
{
    uint64_t val = ((uint64_t)env->regs[R_EDX] << 32) | (uint32_t)env->regs[R_EAX];
    uint32_t ecx = (uint32_t)env->regs[R_ECX];

    switch (ecx) {
    case MSR_IA32_APICBASE:
        // Reserved bits check
        if (val & MSR_IA32_APICBASE_RESERVED) {
            goto error;
        }
        cpu_set_apic_base(env_archcpu(env)->apic_state, val);
        break;

    case MSR_EFER:
        // Build valid update mask based on features
        uint64_t update_mask = 0;
        if (env->features[FEAT_8000_0001_EDX] & CPUID_EXT2_SYSCALL) {
            update_mask |= MSR_EFER_SCE;
        }
        if (env->features[FEAT_8000_0001_EDX] & CPUID_EXT2_LM) {
            update_mask |= MSR_EFER_LME;
        }
        if (env->features[FEAT_8000_0001_EDX] & CPUID_EXT2_NX) {
            update_mask |= MSR_EFER_NXE;
        }
        // ... more feature checks

        // Reject writes to unsupported bits
        if (val & ~update_mask) {
            goto error;
        }
        cpu_x86_update_efer(env, val);
        break;

    case MSR_IA32_PKRS:
        // PKR reserved bits (only low 32 bits valid)
        if (val & 0xFFFFFFFF00000000ULL) {
            goto error;
        }
        env->pkrs = val;
        break;

    // ... many more MSR cases
    }
    return;

error:
    raise_exception_err_ra(env, EXCP0D_GPF, 0, GETPC());
}
```

### 5.3 Page Table Reserved Bit Validation

**File:** `/models/dev/qemu/target/i386/tcg/system/excp_helper.c** (lines 150-370)

```c
// Compute reserved bits mask
rsvd_mask = ~MAKE_64BIT_MASK(0, env_archcpu(env)->phys_bits);
rsvd_mask &= PG_ADDRESS_MASK;
if (!(pg_mode & PG_MODE_NXE)) {
    rsvd_mask |= PG_NX_MASK;  // NX bit reserved if NXE disabled
}

// Level 5 (PML5) check
if (pte & (rsvd_mask | PG_PSE_MASK)) {  // PSE not valid at level 5
    goto do_fault_rsvd;
}

// Level 4 (PML4) check
if (pte & (rsvd_mask | PG_PSE_MASK)) {  // PSE not valid at level 4
    goto do_fault_rsvd;
}

// Level 3 (PDPT) check - 1GB pages allowed if PSE set
if (pte & rsvd_mask) {
    goto do_fault_rsvd;
}

// Level 2 (PD) and Level 1 (PT) checks
if (pte & rsvd_mask) {
    goto do_fault_rsvd;
}
```

### 5.4 Segment Descriptor Validation

**File:** `/models/dev/qemu/target/i386/tcg/seg_helper.c** (lines 150-177)

```c
static inline int load_segment_ra(CPUX86State *env, uint32_t *e1_ptr,
                                  uint32_t *e2_ptr, int selector,
                                  uintptr_t retaddr)
{
    SegmentCache *dt;
    int index;
    target_ulong ptr;

    // Select table (GDT or LDT)
    if (selector & 0x4) {
        dt = &env->ldt;
    } else {
        dt = &env->gdt;
    }

    // Check bounds
    index = selector & ~7;
    if ((index + 7) > dt->limit) {
        return -1;  // Selector out of bounds
    }

    // Load descriptor
    ptr = dt->base + index;
    *e1_ptr = cpu_ldl_kernel_ra(env, ptr, retaddr);
    *e2_ptr = cpu_ldl_kernel_ra(env, ptr + 4, retaddr);
    return 0;
}
```

#### 5.4.1 TSS Validation

**File:** `/models/dev/qemu/target/i386/tcg/seg_helper.c** (lines 255-299)

```c
static void tss_load_seg(CPUX86State *env, X86Seg seg_reg, int selector,
                         int cpl, uintptr_t retaddr)
{
    // Load and validate descriptor
    if (load_segment_ra(env, &e1, &e2, selector, retaddr) != 0) {
        raise_exception_err_ra(env, EXCP0A_TSS, selector & 0xfffc, retaddr);
    }

    // Check present bit
    if (!(e2 & DESC_P_MASK)) {
        raise_exception_err_ra(env, EXCP0B_NOSEG, selector & 0xfffc, retaddr);
    }

    dpl = (e2 >> DESC_DPL_SHIFT) & 3;
    rpl = selector & 3;

    // Code segment validation
    if (seg_reg == R_CS) {
        if (!(e2 & DESC_CS_MASK)) {
            raise_exception_err_ra(env, EXCP0A_TSS, selector & 0xfffc, retaddr);
        }
        if (dpl != rpl) {  // DPL must equal RPL for CS
            raise_exception_err_ra(env, EXCP0A_TSS, selector & 0xfffc, retaddr);
        }
    }
    // Stack segment validation
    else if (seg_reg == R_SS) {
        if ((e2 & DESC_CS_MASK) || !(e2 & DESC_W_MASK)) {
            raise_exception_err_ra(env, EXCP0A_TSS, selector & 0xfffc, retaddr);
        }
        if (dpl != cpl || dpl != rpl) {
            raise_exception_err_ra(env, EXCP0A_TSS, selector & 0xfffc, retaddr);
        }
    }
}
```

#### 5.4.2 LLDT/LTR Validation

**File:** `/models/dev/qemu/target/i386/tcg/seg_helper.c** (lines 1277-1356)

```c
void helper_lldt(CPUX86State *env, int selector)
{
    // Must be in protected mode
    if (!(env->cr[0] & CR0_PE_MASK)) {
        raise_exception_ra(env, EXCP0D_GPF, GETPC());
    }

    // LDT must come from GDT (not LDT)
    if (selector & 0x4) {
        raise_exception_err_ra(env, EXCP0D_GPF, selector & 0xfffc, GETPC());
    }

    // Descriptor type validation (must be LDT descriptor, type 2)
    if ((e2 & DESC_S_MASK) || ((e2 >> DESC_TYPE_SHIFT) & 0xf) != 2) {
        raise_exception_err_ra(env, EXCP0D_GPF, selector & 0xfffc, GETPC());
    }

    // Check present bit
    if (!(e2 & DESC_P_MASK)) {
        raise_exception_err_ra(env, EXCP0B_NOSEG, selector & 0xfffc, GETPC());
    }
}
```

### 5.5 Exception Numbers

**File:** `/models/dev/qemu/target/i386/cpu.h** (lines 1396-1417)

| Exception | Vector | Name |
|-----------|--------|------|
| `EXCP00_DIVZ` | 0 | Divide by Zero |
| `EXCP01_DB` | 1 | Debug |
| `EXCP02_NMI` | 2 | Non-Maskable Interrupt |
| `EXCP03_INT3` | 3 | Breakpoint |
| `EXCP04_INTO` | 4 | Overflow |
| `EXCP05_BOUND` | 5 | Bound Range Exceeded |
| `EXCP06_ILLOP` | 6 | Invalid Opcode |
| `EXCP07_PREX` | 7 | Device Not Available |
| `EXCP08_DBLE` | 8 | Double Fault |
| `EXCP09_XERR` | 9 | Coprocessor Segment Overrun |
| `EXCP0A_TSS` | 10 | Invalid TSS |
| `EXCP0B_NOSEG` | 11 | Segment Not Present |
| `EXCP0C_STACK` | 12 | Stack Segment Fault |
| `EXCP0D_GPF` | 13 | General Protection Fault |
| `EXCP0E_PAGE` | 14 | Page Fault |
| `EXCP10_COPR` | 16 | x87 FPU Error |
| `EXCP11_ALGN` | 17 | Alignment Check |
| `EXCP12_MCHK` | 18 | Machine Check |
| `EXCP13_SSE` | 19 | SIMD Floating-Point Exception |

### 5.6 Exception Nesting and Double Fault

**File:** `/models/dev/qemu/target/i386/tcg/excp_helper.c** (lines 44-82)

```c
static int check_exception(CPUX86State *env, int intno, int *error_code,
                           uintptr_t retaddr)
{
    int first_contributory = env->old_exception == 0 ||
                             (env->old_exception >= 10 &&
                              env->old_exception <= 13);
    int second_contributory = intno == 0 ||
                              (intno >= 10 && intno <= 13);

    // Already in double fault -> triple fault
    if (env->old_exception == EXCP08_DBLE) {
        if (env->hflags & HF_GUEST_MASK) {
            cpu_vmexit(env, SVM_EXIT_SHUTDOWN, 0, retaddr);
        }
        qemu_log_mask(CPU_LOG_RESET, "Triple fault\n");
        qemu_system_reset_request(SHUTDOWN_CAUSE_GUEST_RESET);
        return EXCP_HLT;
    }

    // Convert to double fault if:
    // - Two contributory exceptions, or
    // - Page fault during contributory or another page fault
    if ((first_contributory && second_contributory) ||
        (env->old_exception == EXCP0E_PAGE &&
         (second_contributory || (intno == EXCP0E_PAGE)))) {
        intno = EXCP08_DBLE;
        *error_code = 0;
    }

    return intno;
}
```

### 5.7 Debug Register Validation

**File:** `/models/dev/qemu/target/i386/tcg/system/svm_helper.c** (lines 378-383)

```c
// In VMRUN validation
if (new_dr7 & DR_RESERVED_MASK) {
    cpu_vmexit(env, SVM_EXIT_ERR, 0, GETPC());
}
if (new_dr6 & DR_RESERVED_MASK) {
    cpu_vmexit(env, SVM_EXIT_ERR, 0, GETPC());
}
```

### 5.8 SVM/EFER State Validation

**File:** `/models/dev/qemu/target/i386/tcg/system/svm_helper.c** (lines 87-120)

```c
static inline bool is_efer_invalid_state(CPUX86State *env)
{
    // SVM must be enabled
    if (!(env->efer & MSR_EFER_SVME)) {
        return true;
    }

    // Reserved bits check
    if (env->efer & MSR_EFER_RESERVED) {
        return true;
    }

    // Long mode requires CPU support
    if ((env->efer & (MSR_EFER_LMA | MSR_EFER_LME)) &&
        !(env->features[FEAT_8000_0001_EDX] & CPUID_EXT2_LM)) {
        return true;
    }

    // LME requires PAE
    if ((env->efer & MSR_EFER_LME) && (env->cr[0] & CR0_PG_MASK) &&
        !(env->cr[4] & CR4_PAE_MASK)) {
        return true;
    }

    // LME requires PE
    if ((env->efer & MSR_EFER_LME) && (env->cr[0] & CR0_PG_MASK) &&
        !(env->cr[0] & CR0_PE_MASK)) {
        return true;
    }

    // Cannot have both L and D bits in CS
    if ((env->efer & MSR_EFER_LME) && (env->cr[0] & CR0_PG_MASK) &&
        (env->cr[4] & CR4_PAE_MASK) &&
        (env->segs[R_CS].flags & DESC_L_MASK) &&
        (env->segs[R_CS].flags & DESC_B_MASK)) {
        return true;
    }

    return false;
}
```

---

## 6. Vendor-Specific CPU Behavior

### 6.1 CPU Model Definition Structure

**File:** `/models/dev/qemu/target/i386/cpu.c** (lines 2205-2225)

```c
typedef struct X86CPUDefinition {
    const char *name;                    // Model name (e.g., "Nehalem", "EPYC")
    uint32_t level;                      // Max CPUID basic level
    uint32_t xlevel;                     // Max CPUID extended level
    uint32_t xlevel2;                    // Max CPUID level for 0x80860000
    char vendor[CPUID_VENDOR_SZ + 1];    // Vendor ID string
    int family;                          // CPU family
    int model;                           // CPU model
    int stepping;                        // CPU stepping
    uint8_t avx10_version;               // AVX10 version
    FeatureWordArray features;           // Feature flags
    const char *model_id;                // Marketing name string
    const CPUCaches *const cache_info;   // Cache topology
    const X86CPUVersionDefinition *versions;  // Version variants
} X86CPUDefinition;
```

### 6.2 Vendor Identification

**File:** `/models/dev/qemu/target/i386/cpu.h** (lines 1176-1204)

```c
// Intel vendor string: "GenuineIntel"
#define CPUID_VENDOR_INTEL_1 0x756e6547    // "Genu"
#define CPUID_VENDOR_INTEL_2 0x49656e69    // "ineI"
#define CPUID_VENDOR_INTEL_3 0x6c65746e    // "ntel"
#define CPUID_VENDOR_INTEL   "GenuineIntel"

// AMD vendor string: "AuthenticAMD"
#define CPUID_VENDOR_AMD_1   0x68747541    // "Auth"
#define CPUID_VENDOR_AMD_2   0x69746e65    // "enti"
#define CPUID_VENDOR_AMD_3   0x444d4163    // "cAMD"
#define CPUID_VENDOR_AMD     "AuthenticAMD"

// VIA vendor string: "CentaurHauls"
#define CPUID_VENDOR_VIA     "CentaurHauls"

// Detection macros
#define IS_INTEL_CPU(env) \
    ((env)->cpuid_vendor1 == CPUID_VENDOR_INTEL_1 && \
     (env)->cpuid_vendor2 == CPUID_VENDOR_INTEL_2 && \
     (env)->cpuid_vendor3 == CPUID_VENDOR_INTEL_3)

#define IS_AMD_CPU(env) \
    ((env)->cpuid_vendor1 == CPUID_VENDOR_AMD_1 && \
     (env)->cpuid_vendor2 == CPUID_VENDOR_AMD_2 && \
     (env)->cpuid_vendor3 == CPUID_VENDOR_AMD_3)
```

### 6.3 Builtin CPU Definitions

**File:** `/models/dev/qemu/target/i386/cpu.c** (lines 3309+)

The `builtin_x86_defs[]` array contains 100+ predefined CPU models:

#### 6.3.1 AMD Models

| Model | Family | Features |
|-------|--------|----------|
| qemu64 | 15 | Basic 64-bit AMD |
| phenom | 16 | 3DNow!, SSE4A, SVM |
| athlon | 6 | 3DNow!, 3DNow!+ |
| EPYC | 23 | AVX2, CLZERO, SVM |
| EPYC-Rome | 23 | AVX2, RDPID, WBNOINVD |
| EPYC-Milan | 25 | AVX2, VAES, PKU |
| EPYC-Genoa | 25 | AVX-512, AMX |

#### 6.3.2 Intel Models

| Model | Family.Model | Features |
|-------|--------------|----------|
| core2duo | 6.15 | SSSE3 |
| Nehalem | 6.26 | SSE4.2, VT-x |
| Westmere | 6.44 | AES-NI |
| SandyBridge | 6.42 | AVX |
| IvyBridge | 6.58 | F16C, FSGSBASE |
| Haswell | 6.60 | AVX2, BMI1/2 |
| Broadwell | 6.61 | ADX, RDSEED |
| Skylake-Client | 6.94 | MPX, SGX |
| Skylake-Server | 6.85 | AVX-512 |
| Cascadelake-Server | 6.85 | VNNI |
| Icelake-Server | 6.106 | AVX-512 VBMI2 |
| SapphireRapids | 6.143 | AMX |

### 6.4 Feature Word Arrays

**File:** `/models/dev/qemu/target/i386/cpu.c** (lines 1057-1190)

Features organized by CPUID leaf:

| Feature Word | CPUID Leaf | Register | Description |
|--------------|------------|----------|-------------|
| FEAT_1_EDX | 0x1 | EDX | Basic features (FPU, MMX, SSE) |
| FEAT_1_ECX | 0x1 | ECX | Extended features (SSE3, SSSE3, SSE4) |
| FEAT_7_0_EBX | 0x7.0 | EBX | Structured features (AVX2, BMI) |
| FEAT_7_0_ECX | 0x7.0 | ECX | More features (AVX-512 VBMI) |
| FEAT_7_0_EDX | 0x7.0 | EDX | Even more (AVX-512 VP2INTERSECT) |
| FEAT_8000_0001_EDX | 0x80000001 | EDX | AMD extended (3DNow!, NX, LM) |
| FEAT_8000_0001_ECX | 0x80000001 | ECX | AMD extended (SVM, SSE4A) |
| FEAT_SVM | 0x8000000A | EDX | SVM features (NPT, NRIPS) |
| FEAT_VMX_* | 0x480-0x48F | - | VMX capabilities |

#### 6.4.1 Example: Phenom Definition (lines 3331-3358)

```c
{
    .name = "phenom",
    .level = 5,
    .vendor = CPUID_VENDOR_AMD,
    .family = 16,
    .model = 2,
    .stepping = 3,
    .features[FEAT_1_EDX] =
        CPUID_VME | CPUID_SSE2 | CPUID_SSE | CPUID_FXSR | CPUID_MMX |
        CPUID_CLFLUSH | CPUID_PSE36 | CPUID_PAT | CPUID_CMOV | CPUID_MCA |
        CPUID_PGE | CPUID_MTRR | CPUID_SEP | CPUID_APIC | CPUID_CX8 |
        CPUID_MCE | CPUID_PAE | CPUID_MSR | CPUID_TSC | CPUID_PSE |
        CPUID_DE | CPUID_FP87,
    .features[FEAT_1_ECX] =
        CPUID_EXT_SSE3 | CPUID_EXT_MONITOR | CPUID_EXT_CX16 |
        CPUID_EXT_POPCNT,
    .features[FEAT_8000_0001_EDX] =
        CPUID_EXT2_LM | CPUID_EXT2_SYSCALL | CPUID_EXT2_NX |
        CPUID_EXT2_3DNOW | CPUID_EXT2_3DNOWEXT | CPUID_EXT2_MMXEXT |
        CPUID_EXT2_FFXSR | CPUID_EXT2_PDPE1GB | CPUID_EXT2_RDTSCP,
    .features[FEAT_8000_0001_ECX] =
        CPUID_EXT3_LAHF_LM | CPUID_EXT3_SVM | CPUID_EXT3_ABM |
        CPUID_EXT3_SSE4A,
    .features[FEAT_SVM] =
        CPUID_SVM_NPT,
    .xlevel = 0x8000001A,
    .model_id = "AMD Phenom(tm) 9550 Quad-Core Processor",
}
```

### 6.5 CPUID Instruction Handling

**File:** `/models/dev/qemu/target/i386/cpu.c** (lines 7862+)

```c
void cpu_x86_cpuid(CPUX86State *env, uint32_t index, uint32_t count,
                   uint32_t *eax, uint32_t *ebx,
                   uint32_t *ecx, uint32_t *edx)
{
    switch (index) {
    case 0:  // Vendor and max level
        *eax = env->cpuid_level;
        *ebx = env->cpuid_vendor1;
        *edx = env->cpuid_vendor2;
        *ecx = env->cpuid_vendor3;
        break;

    case 1:  // Family/model/stepping and features
        *eax = env->cpuid_version;
        *ebx = /* APIC ID, CLFLUSH size, logical processors */;
        *ecx = env->features[FEAT_1_ECX];
        *edx = env->features[FEAT_1_EDX];
        break;

    case 4:  // Intel cache descriptors
        // Returns cache topology in Intel format
        break;

    case 0x80000000:  // Extended vendor and max level
        *eax = env->cpuid_xlevel;
        *ebx = env->cpuid_vendor1;
        *edx = env->cpuid_vendor2;
        *ecx = env->cpuid_vendor3;
        break;

    case 0x80000001:  // Extended features
        *eax = env->cpuid_version;  // Same as leaf 1
        *ecx = env->features[FEAT_8000_0001_ECX];
        *edx = env->features[FEAT_8000_0001_EDX];
        // Vendor-specific: Intel lacks SYSCALL in non-64-bit mode
        if (!IS_AMD_CPU(env) && !(env->hflags & HF_LMA_MASK)) {
            *edx &= ~CPUID_EXT2_SYSCALL;
        }
        break;

    case 0x80000005:  // AMD L1 cache info
        if (IS_AMD_CPU(env)) {
            // Return AMD-format cache information
        } else {
            *eax = *ebx = *ecx = *edx = 0;  // Reserved for Intel
        }
        break;

    case 0x80000006:  // AMD L2/L3 cache info
        // Similar vendor-specific handling
        break;

    case 0x8000000A:  // SVM features (AMD only)
        if (IS_AMD_CPU(env)) {
            *eax = 0x01;  // SVM revision
            *ebx = 0x40;  // Number of ASIDs
            *edx = env->features[FEAT_SVM];
        }
        break;
    }
}
```

### 6.6 Vendor-Specific MSR Handling

**File:** `/models/dev/qemu/target/i386/kvm/kvm.c** (lines 4409-4654)

#### 6.6.1 AMD-Specific MSRs

| MSR | Address | Purpose |
|-----|---------|---------|
| MSR_STAR | 0xC0000081 | SYSCALL target for 32-bit |
| MSR_LSTAR | 0xC0000082 | SYSCALL target for 64-bit |
| MSR_CSTAR | 0xC0000083 | SYSCALL target for compat mode |
| MSR_FMASK | 0xC0000084 | SYSCALL flag mask |
| MSR_VM_HSAVE_PA | 0xC0010117 | SVM host save area |
| MSR_TSC_AUX | 0xC0000103 | TSC auxiliary data |
| MSR_K7_HWCR | 0xC0010015 | Hardware configuration |

#### 6.6.2 Intel-Specific MSRs

| MSR | Address | Purpose |
|-----|---------|---------|
| MSR_IA32_SPEC_CTRL | 0x48 | Speculation control |
| MSR_IA32_TSX_CTRL | 0x122 | TSX control |
| MSR_IA32_RTIT_* | Various | Intel PT |
| MSR_IA32_SGXLEPUBKEYHASH* | 0x8C-0x8F | SGX launch enclave key |

### 6.7 Cache Information

**File:** `/models/dev/qemu/target/i386/cpu.c**

#### 6.7.1 Legacy Cache Structures

```c
// Lines 644-696: AMD legacy caches
static const CPUCaches legacy_amd_cache_info = {
    .l1d_cache = { .size = 64 * KiB, .line_size = 64, .assoc = 2 },
    .l1i_cache = { .size = 64 * KiB, .line_size = 64, .assoc = 2 },
    .l2_cache = { .size = 512 * KiB, .line_size = 64, .assoc = 16 },
    .l3_cache = { .size = 16 * MiB, .line_size = 64, .assoc = 16 },
};

// Lines 759-810: Intel legacy caches
static const CPUCaches legacy_intel_cache_info = {
    .l1d_cache = { .size = 32 * KiB, .line_size = 64, .assoc = 8 },
    .l1i_cache = { .size = 32 * KiB, .line_size = 64, .assoc = 8 },
    .l2_cache = { .size = 4 * MiB, .line_size = 64, .assoc = 16 },
    .l3_cache = { .size = 16 * MiB, .line_size = 64, .assoc = 16 },
};
```

#### 6.7.2 Modern EPYC Cache Structures

```c
// Lines 2261-2313: EPYC generation 1
static const CPUCaches epyc_cache_info = {
    .l1d_cache = {
        .type = DATA_CACHE,
        .level = 1,
        .size = 32 * KiB,
        .line_size = 64,
        .associativity = 8,
        .partitions = 1,
        .sets = 64,
        .lines_per_tag = 1,
        .self_init = true,
        .share_level = CPU_TOPO_LEVEL_CORE,
    },
    // ... L1I, L2, L3 definitions
};
```

### 6.8 CPU Model Loading

**File:** `/models/dev/qemu/target/i386/cpu.c** (lines 7713-7770)

```c
static void x86_cpu_load_model(X86CPU *cpu, const X86CPUModel *model)
{
    const X86CPUDefinition *def = model->cpudef;
    CPUX86State *env = &cpu->env;

    // Set CPUID levels
    object_property_set_uint(OBJECT(cpu), "min-level", def->level, &error_abort);
    object_property_set_uint(OBJECT(cpu), "min-xlevel", def->xlevel, &error_abort);

    // Set family/model/stepping
    object_property_set_int(OBJECT(cpu), "family", def->family, &error_abort);
    object_property_set_int(OBJECT(cpu), "model", def->model, &error_abort);
    object_property_set_int(OBJECT(cpu), "stepping", def->stepping, &error_abort);

    // Copy feature flags
    for (FeatureWord w = 0; w < FEATURE_WORDS; w++) {
        env->features[w] = def->features[w];
    }

    // Set vendor string
    object_property_set_str(OBJECT(cpu), "vendor", def->vendor, &error_abort);

    // Set model ID string
    object_property_set_str(OBJECT(cpu), "model-id", def->model_id, &error_abort);

    // Load cache information
    if (def->cache_info) {
        cpu->cache_info_passthrough = false;
        cpu->legacy_cache = false;
        // Copy cache structures
    }
}
```

---

## Quick Reference Tables

### Exception Summary

| Vector | Mnemonic | Type | Error Code | Description |
|--------|----------|------|------------|-------------|
| 0 | #DE | Fault | No | Divide Error |
| 1 | #DB | Fault/Trap | No | Debug |
| 2 | NMI | Interrupt | No | Non-Maskable Interrupt |
| 3 | #BP | Trap | No | Breakpoint |
| 4 | #OF | Trap | No | Overflow |
| 5 | #BR | Fault | No | BOUND Range Exceeded |
| 6 | #UD | Fault | No | Invalid Opcode |
| 7 | #NM | Fault | No | Device Not Available |
| 8 | #DF | Abort | Yes (0) | Double Fault |
| 10 | #TS | Fault | Yes | Invalid TSS |
| 11 | #NP | Fault | Yes | Segment Not Present |
| 12 | #SS | Fault | Yes | Stack Segment Fault |
| 13 | #GP | Fault | Yes | General Protection |
| 14 | #PF | Fault | Yes | Page Fault |
| 16 | #MF | Fault | No | x87 FPU Error |
| 17 | #AC | Fault | Yes (0) | Alignment Check |
| 18 | #MC | Abort | No | Machine Check |
| 19 | #XM | Fault | No | SIMD Floating-Point |

### Page Fault Error Code Bits

| Bit | Mask | Set Meaning | Clear Meaning |
|-----|------|-------------|---------------|
| 0 | P | Protection violation | Page not present |
| 1 | W/R | Write access | Read access |
| 2 | U/S | User mode | Supervisor mode |
| 3 | RSVD | Reserved bit set | - |
| 4 | I/D | Instruction fetch | Data access |
| 5 | PK | Protection key violation | - |
| 6 | SS | Shadow stack access | - |
| 15 | SGX | SGX violation | - |

### Paging Mode Summary

| Mode | CR0.PG | CR4.PAE | EFER.LME | CR4.LA57 | Levels | Address Bits |
|------|--------|---------|----------|----------|--------|--------------|
| None | 0 | - | - | - | 0 | Physical |
| 32-bit | 1 | 0 | - | - | 2 | 32 |
| 32-bit PSE | 1 | 0 | - | - | 2 | 32 (4MB pages) |
| PAE | 1 | 1 | 0 | - | 3 | 36 |
| Long Mode | 1 | 1 | 1 | 0 | 4 | 48 |
| LA57 | 1 | 1 | 1 | 1 | 5 | 57 |

---

## File Index

| Component | File Path | Lines |
|-----------|-----------|-------|
| CPU State | `target/i386/cpu.h` | 1846-2164 |
| CPU Reset | `target/i386/cpu.c` | 8617-8788 |
| CR Updates | `target/i386/helper.c` | 135-240 |
| Page Table Walk | `target/i386/tcg/system/excp_helper.c` | 142-654 |
| TLB Structure | `include/exec/tlb-common.h` | 22-56 |
| Instruction Decoder | `target/i386/tcg/decode-new.c.inc` | 1-2913 |
| Decoder Types | `target/i386/tcg/decode-new.h` | 1-347 |
| Code Emission | `target/i386/tcg/emit.c.inc` | 1-4824 |
| Translator | `target/i386/tcg/translate.c` | 1-3932 |
| Segment Helpers | `target/i386/tcg/seg_helper.c` | 1-2600 |
| MSR Helpers | `target/i386/tcg/system/misc_helper.c` | 1-500 |
| SVM Helpers | `target/i386/tcg/system/svm_helper.c` | 1-500 |
| CPU Models | `target/i386/cpu.c` | 2205-7000 |
| CPUID Handler | `target/i386/cpu.c` | 7862-8500 |
| Unit Tests | `tests/tcg/i386/test-i386.c` | 1-2000 |
| FPU Tests | `tests/tcg/i386/test-i386-fpatan.c` | 1-4000 |
| Test Makefile | `tests/tcg/i386/Makefile.target` | 1-103 |
