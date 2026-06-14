# Fixmap Address Corruption Bug - Investigation Notes

## Summary

The emulator crashes during Linux kernel boot when the kernel's `text_poke` mechanism tries to use a fixmap virtual address. The fixmap address is corrupted, causing a page table fault.

## Symptoms

### 1. Crash Location
- **Function:** `text_poke_memcpy+0xa/0x20`
- **Instruction:** `REP MOVSB` (f3 a4)
- **Call stack:**
  ```
  text_poke_memcpy
  __text_poke+0x192/0x3e0
  alternatives_smp_module_add+0x125/0x130
  alternative_instructions+0xf4/0x140
  arch_cpu_finalize_init+0x11b/0x170
  start_kernel+0x6fc/0x780
  ```

### 2. Corrupted Address Pattern
- **Expected:** `0xffffffffff7fcXXX` (kernel fixmap virtual address)
- **Actual:** `0x0000NNNNNNNNx209` where NNNN is garbage (varies per run)

Examples observed:
- `0x00002ebc737a7209`
- `0x0000773d5589c209`
- `0x0000751c6a2c6209`
- `0x0000664bced2e209`

### 3. Pattern Analysis
| Bits | Expected | Actual | Status |
|------|----------|--------|--------|
| 63-48 | 0xFFFF | 0x0000 | **WRONG** |
| 47-12 | fixmap base bits | garbage (TSC-like) | **WRONG** |
| 11-0 | page offset (0x209) | 0x209 | CORRECT |

The page offset (bottom 12 bits) is always correct, but the upper 52 bits are garbage.

### 4. Corruption Trace Path

Traced backwards from crash:

1. **Crash:** `text_poke_memcpy` uses RDI=corrupted address
2. **RIP=0xffffffff812ac789:** `MOV RDI, R13` - copies corrupted R13 to RDI
3. **RIP=0xffffffff812ac776:** `ADD R13, [RIP+0x017fcc33]` - R13 gets corrupted value from MEMORY
4. Memory at `~0xffffffff82aa93b0` contains the corrupted value
5. **RIP=0xffffffff81001916:** `MOV [RSP+8], RDI` - earlier code wrote corrupted RDI to stack/memory

### 5. Register State at Corruption Detection

```
WRITE_CORRUPT: [0xffffffff82c03dd0] = 0x0000664bced2e209 at RIP=0xffffffff81001916
  INSN: 48 89 7c 24 08 (MOV [RSP+8], RDI)
  RAX=0xffffffff82d75180
  RBX=0xffffffff81200209  <- Correct kernel text address being patched
  RCX=0x1
  RDX=0x1
  RSI=0xffffffff8100128d
  RDI=0x664bced2e209      <- CORRUPTED fixmap address
  R8=0xffffffff82c0e980
  R9=0x8000000000000063   <- Looks like a PTE value
```

### 6. Key Observations

1. **RBX is correct:** Contains `0xffffffff81200209` (the kernel text address being patched)
2. **RDI is corrupted:** Should contain the fixmap virtual address for writing
3. **R9 looks like a PTE:** `0x8000000000000063` (NX bit set, Present|Writable|Accessed)
4. **Garbage resembles TSC:** The upper bits change each run, suggesting RDTSC influence

## Hypothesis

The fixmap virtual address computation is going wrong. The kernel computes:
```c
fixmap_addr = FIXADDR_TOP - (idx << PAGE_SHIFT) + (phys_addr & 0xFFF);
```

- `FIXADDR_TOP` is ~0xffffffffff7ff000
- The page offset (`phys_addr & 0xFFF`) is preserved correctly (0x209)
- The base computation is producing garbage

Possible causes:
1. **Memory corruption:** The fixmap base stored in memory gets overwritten with garbage
2. **Computation bug:** SUB/SHL instruction producing wrong result
3. **Read bug:** Memory read returning wrong value

## Linux Kernel Context

### set_fixmap_offset Implementation

The kernel uses fixmap for temporary mappings:
```c
// arch/x86/include/asm/fixmap.h
#define __fix_to_virt(x)    (FIXADDR_TOP - ((x) << PAGE_SHIFT))
#define set_fixmap_offset(idx, phys) \
    __set_fixmap_offset(idx, phys, PAGE_KERNEL)

static inline void *__set_fixmap_offset(enum fixed_addresses idx,
                                        phys_addr_t phys, pgprot_t flags)
{
    __set_fixmap(idx, phys & PAGE_MASK, flags);
    return (void *)(__fix_to_virt(idx) + (phys & ~PAGE_MASK));
}
```

### text_poke Flow

1. `text_poke` wants to modify kernel code at `phys_addr`
2. Calls `set_fixmap_offset(FIX_TEXT_POKE0, phys_addr)`
3. Gets back fixmap virtual address
4. Uses that address to write via `text_poke_memcpy`

## ROOT CAUSE IDENTIFIED (2025-12-26)

### The Corruption Source

The corruption happens in the kernel's KASLR initialization code. At RIP `0xffffffff8347a86f`:

```asm
MOV RAX, 0x00002aaaaaaab000   ; 48 b8 00 b0 aa aa aa 2a 00 00
```

This loads a placeholder/magic value into RAX, which is then stored to the fixmap base address at `0xffffffff82aa93b0`.

### The Value Pattern

The value `0x00002aaaaaaab000` appears to be:
- A page-aligned placeholder (`0xAAAAAAAAB << 12`)
- Used by KASLR infrastructure for address randomization
- Should be replaced with actual kernel addresses during KASLR patching

### Why This Happens

Even with `nokaslr` on the command line, the kernel's KASLR code appears to:
1. Load placeholder addresses as immediate values
2. Store them to kernel data structures (including fixmap base)
3. These should be patched to real addresses, but something prevents this

### Writes to Fixmap Base

Two corrupted writes were observed:
1. RIP=`0xffffffff8347a8a0`: `MOV [RIP+disp], RAX` stores `0x00002aaaaaaab000`
2. RIP=`0xffffffff8347a94f`: `MOV [RIP+disp], RDX` stores `0x000036cff7b88000`

Both values are non-canonical (upper 16 bits = 0x0000) and look like KASLR-related computations.

## Investigation Update (2025-12-26) - Root Cause Clarified

### The `text_poke_mm_addr` Values Are CORRECT

Initial investigation incorrectly identified `0x00002aaaaaaab000` as "corrupted". This is actually the correct value:

- `TASK_UNMAPPED_BASE = PAGE_ALIGN(TASK_SIZE_LOW / 3) ≈ 0x00002aaaaaaab000`
- This is a **user-space virtual address** for the `text_poke_mm` mm_struct
- The kernel creates a separate mm for patching kernel code safely

### Actual Crash Analysis

The kernel crashes at `text_poke_memcpy` with:
```
Corrupted page table at address 60380e0a3209
PGD 8000000003c58067 P4D 8000000003c58067 PUD 3c59067 PMD 3c5a067 PTE 800580000000a063
```

Key observations:
1. CR3 is `0x3c52000` - the kernel DID switch to `text_poke_mm`'s page tables
2. Upper-level page table entries (PGD, PUD, PMD) look valid (~60MB physical addresses)
3. The PTE `0x800580000000a063` has physical address ~87TB - **THIS IS GARBAGE**
4. Guest RAM is only 512MB, so this physical address is impossible

### Root Cause Hypothesis

The PTE for `text_poke_mm_addr` contains garbage. This happens during:
1. `mk_pte(pages[0], pgprot)` - Creates PTE from struct page
2. `set_pte_at(text_poke_mm, text_poke_mm_addr, ptep, pte)` - Writes PTE to page table

Either:
- The `struct page` pointer is bogus (from `virt_to_page(addr)`)
- The PTE write goes to the wrong location
- Something overwrites the PTE after it's set

### Next Investigation Steps

1. Check `virt_to_page()` implementation - does it return correct page structs?
2. Check `set_pte_at()` - is the PTE write going to the correct address?
3. Add debug output to trace PTE values being written
4. Check if there's any memory corruption happening

## QEMU Investigation Findings (2025-12-26)

### Key Finding: QEMU Has No Special KASLR Handling

After investigating the QEMU source code at `/models/dev/qemu`, we found:

1. **No KASLR-specific code** - QEMU's TCG simply executes guest instructions as-is
2. **No placeholder patching** - QEMU doesn't intercept or modify kernel relocation code
3. **Standard address translation** - Uses normal page table walking for all addresses
4. **Canonical address validation** - Non-canonical addresses trigger #GP during paging

This means the issue is NOT that we're missing some special KASLR handling that QEMU has.

### Analysis: Why the Placeholder Isn't Patched

The value `0x00002aaaaaaab000` is loaded as an **immediate constant** in the instruction:
```asm
MOV RAX, 0x00002aaaaaaab000   ; 48 b8 00 b0 aa aa aa 2a 00 00
```

This is NOT a memory read - the placeholder is baked into the instruction encoding itself.

For this to be a valid kernel address after relocation, the kernel's early boot code (in the decompression stage) should have:
1. Applied relocations to patch this instruction
2. Replaced `0x00002aaaaaaab000` with the actual fixmap base address

Since the placeholder is still present after decompression, something is preventing the relocation from happening.

### Possible Causes

1. **Relocation code path not taken** - The kernel's relocation code may check for certain CPU features or conditions that our emulator doesn't satisfy
2. **RDRAND/RDSEED handling** - KASLR uses these for entropy; if they fail, the kernel may skip certain initialization
3. **Early boot code divergence** - Some code path that works differently due to missing/different CPU features

### The `0x00002aaaaaaab000` Pattern

This value has a distinctive pattern:
- Upper 16 bits: `0x0000` (non-canonical for kernel addresses)
- Middle: `0x2AAAAAAB` (looks like a marker pattern)
- Lower 12 bits: `0x000` (page-aligned)

The pattern `0xAAAB` repeated suggests this is an intentional placeholder marker, similar to how `0xDEADBEEF` is used for debugging.

### Next Steps for Investigation

1. Check if the kernel's `nokaslr` handling is being processed correctly
2. Investigate whether the emulator is missing some early boot KASLR patching
3. Look at how other emulators/hypervisors handle KASLR-enabled kernels with `nokaslr`
4. Consider if there's a timing issue where KASLR placeholders are used before being patched
5. **NEW:** Investigate RDRAND/RDSEED implementation - these may affect KASLR code paths
6. **NEW:** Check kernel's relocation code requirements (CPUID features, MSRs, etc.)

## Questions for Investigation

1. Where does the fixmap base address get computed/stored?
2. What instruction sequence computes `FIXADDR_TOP - (idx << PAGE_SHIFT)`?
3. Is the computation happening correctly but the result stored incorrectly?
4. Is there a memory read returning wrong data?
5. Why do the garbage bits look TSC-related?

## Files to Examine

- Emulator ADD instruction implementation
- Emulator SUB instruction implementation
- Emulator memory read/write (especially 64-bit)
- RIP-relative addressing calculation
- LEA instruction implementation

---

# QEMU x86 Emulation Reference

This section documents how QEMU implements the specific operations relevant to this bug. Use this as reference when investigating the rax emulator implementation.

## 1. Critical Instruction: ADD with RIP-Relative Memory Operand

The corruption was traced to this instruction:
```asm
ADD R13, [RIP+0x017fcc33]    ; at RIP=0xffffffff812ac776
```

This instruction requires:
1. Compute effective address: RIP (after fetch) + signed 32-bit displacement
2. Read 64-bit value from that address
3. Add the value to R13
4. Store result back to R13

### 1.1 QEMU ModRM Decoding for RIP-Relative

**Source:** `/models/dev/qemu/target/i386/tcg/translate.c` (lines 1714-1837)

```c
static AddressParts gen_lea_modrm_0(CPUX86State *env, DisasContext *s, int modrm)
{
    int mod = (modrm >> 6) & 3;
    int rm = (modrm & 7) | REX_B(s);
    AddressParts result = { .def_seg = R_DS };

    if (s->aflag == MO_64) {
        // 64-bit addressing mode
        if (rm == 4) {
            // SIB byte follows (rm=100 in 64-bit)
            int sib = x86_ldub_code(env, s);
            result.base = (sib & 7) | REX_B(s);
            result.index = ((sib >> 3) & 7) | REX_X(s);
            result.scale = (sib >> 6) & 3;
            if (result.index == 4) {
                result.index = -1;  // RSP encoding means no index
            }
        } else if (rm == 5 && mod == 0) {
            // **RIP-RELATIVE ADDRESSING**
            // When mod=00 and rm=101, this is RIP-relative in 64-bit mode
            result.base = -2;  // Special marker for RIP-relative
            result.disp = (int32_t)x86_ldl_code(env, s);  // Sign-extended 32-bit
        } else {
            result.base = rm;
        }

        // Displacement based on mod field
        switch (mod) {
        case 0: break;  // No displacement (or RIP-relative above)
        case 1: result.disp = (int8_t)x86_ldub_code(env, s); break;
        case 2: result.disp = (int32_t)x86_ldl_code(env, s); break;
        }
    }

    return result;
}
```

### 1.2 Key Points for RIP-Relative

1. **Detection:** `mod == 0 && rm == 5` in 64-bit mode triggers RIP-relative
2. **Displacement:** Always 32-bit, sign-extended to 64-bit
3. **Base Address:** RIP value is the address **after** the current instruction (i.e., next instruction's RIP)
4. **Computation:** `effective_addr = next_RIP + sign_extend(disp32)`

### 1.3 QEMU ADD Instruction Implementation

**Source:** `/models/dev/qemu/target/i386/tcg/emit.c.inc` (lines 1259-1270)

```c
static void gen_ADD(DisasContext *s, X86DecodedInsn *decode)
{
    MemOp ot = decode->op[1].ot;  // Operand size (MO_64 for 64-bit)

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

### 1.4 Operand Loading Flow

For `ADD R13, [mem]`:
1. **Operand 0 (destination):** R13 loaded into T0
2. **Operand 1 (source):** Memory value loaded into T1
3. **Operation:** T0 = T0 + T1
4. **Writeback:** T0 stored back to R13

**Memory Load Code:**
```c
static void gen_load(DisasContext *s, X86DecodedInsn *decode, int opn, TCGv dest)
{
    X86DecodedOp *op = &decode->op[opn];

    switch (op->unit) {
    case X86_OP_INT:
        if (op->has_ea) {
            // **MEMORY OPERAND** - this is where the read happens
            gen_op_ld_v(s, op->ot, dest, s->A0);  // A0 = effective address
        } else {
            // Register operand
            gen_op_mov_v_reg(s, op->ot, dest, op->n);
        }
        break;
    // ... other cases
    }
}
```

## 2. Memory Read Implementation

### 2.1 QEMU Memory Load Helper

**Source:** `/models/dev/qemu/target/i386/tcg/translate.c` (lines 850-890)

```c
static inline void gen_op_ld_v(DisasContext *s, MemOp ot, TCGv dest, TCGv addr)
{
    // Generate TCG load operation with proper endianness and size
    tcg_gen_qemu_ld_tl(dest, addr, s->mem_index, ot | MO_LE);
}
```

### 2.2 TCG Load Generation

At runtime, this translates to:
1. **TLB Lookup:** Check if virtual address is in TLB
2. **Page Walk:** If TLB miss, walk page tables
3. **Physical Access:** Read from guest physical memory
4. **Size Extension:** For smaller sizes, zero or sign extend

### 2.3 64-bit Memory Read Checklist

For the rax emulator, verify:
- [ ] RIP-relative effective address computed correctly
- [ ] 64-bit read loads all 8 bytes
- [ ] Byte order is correct (little-endian)
- [ ] TLB/page table lookup returns correct physical address
- [ ] No register clobbering during memory access

## 3. REX Prefix Handling

The corrupted instruction uses R13, which requires the REX.B prefix.

### 3.1 REX Prefix Structure

```
REX prefix: 0100 WRXB (0x40-0x4F)
  W = 64-bit operand size
  R = extends ModRM.reg field (for destination in some instructions)
  X = extends SIB.index field
  B = extends ModRM.rm or SIB.base field
```

### 3.2 QEMU REX Processing

**Source:** `/models/dev/qemu/target/i386/tcg/decode-new.c.inc` (lines 2552-2675)

```c
case 0x40 ... 0x4f:  // REX prefix (x86-64 only)
    if (CODE64(s)) {
        s->prefix |= PREFIX_REX;
        s->rex_w = (b >> 3) & 1;  // Bit 3: 64-bit operand
        s->rex_r = (b >> 2) & 1;  // Bit 2: extends reg
        s->rex_x = (b >> 1) & 1;  // Bit 1: extends index
        s->rex_b = b & 1;         // Bit 0: extends r/m or base
        goto next_byte;           // Continue collecting prefixes
    }
    break;
```

### 3.3 Register Selection with REX.B

For `R13`:
- Base encoding: 5 (101 binary)
- REX.B = 1
- Final register: (REX.B << 3) | 5 = 8 + 5 = 13 = R13

**Code pattern:**
```c
int rm = (modrm & 7) | (REX_B(s) << 3);  // Extends r/m to 4 bits
```

## 4. Condition Codes (FLAGS)

The ADD instruction updates FLAGS based on the result.

### 4.1 QEMU Lazy Flags

QEMU uses "lazy evaluation" for condition codes:
```c
typedef enum {
    CC_OP_DYNAMIC,    // Must compute from cc_op variable
    CC_OP_EFLAGS,     // All flags in cc_src
    CC_OP_ADDB, CC_OP_ADDW, CC_OP_ADDL, CC_OP_ADDQ,  // ADD operation
    // ... more operations
} CCOp;
```

For ADD:
- `cc_dst` = result (T0 after addition)
- `cc_src` = one operand (T1, the source)
- `cc_op` = CC_OP_ADDQ (for 64-bit)

### 4.2 Flag Computation

When flags are actually needed:
```c
// Simplified from cpu.h helper functions
CF = (result < src1);  // Unsigned overflow
OF = ((src1 ^ result) & (src2 ^ result)) >> 63;  // Signed overflow
ZF = (result == 0);
SF = (result >> 63);   // Sign bit
```

## 5. Register Writeback

### 5.1 QEMU Register Write

```c
static void gen_op_mov_reg_v(DisasContext *s, MemOp ot, int reg, TCGv val)
{
    switch (ot) {
    case MO_8:
        // 8-bit: depends on REX prefix for high byte access
        tcg_gen_deposit_tl(cpu_regs[reg], cpu_regs[reg], val, 0, 8);
        break;
    case MO_16:
        // 16-bit: preserve high 48 bits
        tcg_gen_deposit_tl(cpu_regs[reg], cpu_regs[reg], val, 0, 16);
        break;
    case MO_32:
        // 32-bit: zero-extend to 64 bits
        tcg_gen_ext32u_tl(cpu_regs[reg], val);
        break;
    case MO_64:
        // 64-bit: full replacement
        tcg_gen_mov_tl(cpu_regs[reg], val);
        break;
    }
}
```

### 5.2 Key Behavior: 32-bit Writes Zero-Extend

In x86-64, writing to a 32-bit register **zero-extends** to 64 bits:
```asm
mov eax, 0x12345678   ; RAX becomes 0x0000000012345678
```

This is NOT the bug source since we're doing 64-bit operations, but worth verifying the emulator handles this correctly.

## 6. Investigation Debugging Strategy

### 6.1 Trace Points to Add

1. **Before ADD execution:**
   ```
   RIP, R13 (before), effective_address, memory_value_to_add
   ```

2. **After ADD execution:**
   ```
   R13 (after), expected_result, actual_result
   ```

3. **Memory read:**
   ```
   virtual_addr, physical_addr, value_read, size
   ```

### 6.2 Specific Values to Watch

| Location | Expected | Corruption Signature |
|----------|----------|----------------------|
| Fixmap addr | `0xFFFFFFFFFF7FC...` | `0x0000NNNNNNNN....` |
| Page offset | `0x209` | `0x209` (correct) |
| Upper bits | `0xFFFF` | `0x0000` |

### 6.3 Hypothesis Testing

**Test 1: Memory Read Bug**
- Add logging to 64-bit memory reads
- Verify returned value matches guest memory content
- Check for partial reads or byte swapping issues

**Test 2: ADD Computation Bug**
- Log operands and result of ADD instructions
- Compare against expected arithmetic
- Check for accidental truncation

**Test 3: Register Writeback Bug**
- Log register before and after writes
- Check for partial updates
- Verify 64-bit writes are full 64-bit

**Test 4: RIP-Relative Address Bug**
- Log RIP value used in address computation
- Verify displacement sign extension
- Check for off-by-one errors in "next RIP" calculation

### 6.4 Narrowing the Search

The corruption pattern (upper bits zeroed, lower 12 bits correct) suggests:
1. **Possible 32-bit truncation somewhere**
2. **Value from wrong source** (maybe TSC leaking into computation)
3. **Incorrect memory address computation** returning wrong data

### 6.5 RDTSC Contamination Check

The garbage bits change per run and look like TSC values. Check:
- Is RDTSC being executed near the corruption point?
- Could TSC value be leaking into RAX/RDX and then to other registers?
- Is there register allocation confusion?

## 7. Test Case Suggestion

Create a minimal test that exercises RIP-relative ADD:

```asm
; Test case: ADD with RIP-relative memory operand
.section .data
test_value: .quad 0xDEADBEEFCAFEBABE

.section .text
test_add_rip_rel:
    mov r13, 0x1111111111111111
    add r13, [rip + test_value]
    ; Expected R13 = 0x1111111111111111 + 0xDEADBEEFCAFEBABE
    ;             = 0xEFBF0000DBFFCCF
    ret
```

Run under both QEMU and rax emulator, compare R13 values.

## 8. QEMU Source File Reference

| Operation | File | Function |
|-----------|------|----------|
| ModRM decode | translate.c:1714 | `gen_lea_modrm_0()` |
| REX prefix | decode-new.c.inc:2552 | prefix loop |
| ADD emit | emit.c.inc:1259 | `gen_ADD()` |
| Memory load | translate.c:850 | `gen_op_ld_v()` |
| Register read | translate.c:815 | `gen_op_mov_v_reg()` |
| Register write | translate.c:836 | `gen_op_mov_reg_v()` |
| Effective address | translate.c:1850 | `gen_lea_modrm()` |
| Condition codes | emit.c.inc:392 | `prepare_update2_cc()`|

## 9. Summary Checklist

Before the bug can be fixed, verify the rax emulator correctly implements:

- [ ] **REX prefix parsing** (W, R, X, B bits)
- [ ] **ModRM decoding** for mod=00, rm=5 (RIP-relative)
- [ ] **Displacement sign extension** (32-bit to 64-bit)
- [ ] **RIP value for address calc** (address of NEXT instruction)
- [ ] **64-bit memory read** (all 8 bytes, correct byte order)
- [ ] **ADD operation** (64-bit addition with no truncation)
- [ ] **Register writeback** (full 64-bit to R8-R15)
- [ ] **No stray values** (TSC, temporary registers) leaking into result
