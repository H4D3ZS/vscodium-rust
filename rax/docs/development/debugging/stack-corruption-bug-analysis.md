# Linux Kernel Stack Corruption Bug Analysis

## Executive Summary

The rax x86_64 emulator successfully boots the Linux kernel through early initialization but panics approximately 8 seconds into boot with a **stack-protector corruption error**. The kernel's stack canary mechanism detects that a function's stack guard value has been overwritten, indicating memory corruption caused by the emulator.

**Final Diagnosis**: The emulator has a bug that corrupts the kernel stack during execution of `local_clock_noinstr`, likely related to incorrect handling of GS-segment-relative RIP-relative memory addressing or a stack manipulation instruction bug.

---

## 1. Problem Description

### 1.1 Symptoms

When running the Linux kernel (v6.19.0-rc2) in the rax x86_64 emulator:

1. **Successful early boot**: The kernel boots normally through memory initialization, CPU topology detection, RCU setup, and interrupt controller configuration.

2. **Last successful output**:
   ```
   [    8.010885] clocksource: tsc-early: mask: 0xffffffffffffffff max_cycles: 0x2b3e459bf4c, max_idle_ns: 440795289890 ns
   ```

3. **Apparent hang**: After this message, no further kernel output appears. The system appears frozen but is actually executing in an infinite loop.

4. **High CPU usage**: The emulator consumes 100% CPU, indicating active execution rather than a true halt.

### 1.2 Environment

- **Emulator**: rax x86_64 software emulator (not KVM)
- **Kernel**: Linux 6.19.0-rc2 (vmlinux ELF format, uncompressed)
- **Memory**: 2GB guest RAM
- **Boot parameters**: `console=ttyS0 earlycon=uart,io,0x3f8 root=/dev/ram0`

---

## 2. Investigation Timeline

### 2.1 Initial Hypothesis: Timing/TSC Issue (REJECTED)

**Theory**: The kernel might be stuck in a timing loop waiting for TSC values that never arrive properly.

**Investigation**:
- Added debug tracing to RDTSC instruction implementation
- Found RDTSC was being called millions of times per second
- The `delay_tsc` function was executing repeatedly

**Finding**: RDTSC was working correctly. The elapsed time calculations were proper, and delay loops were completing. However, something kept re-entering the delay loop.

### 2.2 Second Hypothesis: GS Segment Addressing Bug (PARTIALLY CONFIRMED)

**Theory**: The kernel uses GS-relative addressing for per-CPU variables. The `delay_tsc` function reads `cpu_number` via GS segment. Perhaps this was returning wrong values, causing infinite retry loops.

**Investigation**:
- Added tracing to GS-relative memory accesses
- Traced WRMSR for GS_BASE (MSR 0xC0000101)
- Verified GS.base was set correctly to the per-CPU offset
- Traced MOV instructions with GS prefix

**Finding**: GS-relative addressing appeared to work correctly. The `cpu_number` variable was being read as 0 (correct for BSP), and the cpu migration check in `delay_tsc` was passing (not triggering the restart path).

### 2.3 Third Hypothesis: Interrupt Delivery Problem (REJECTED)

**Theory**: The kernel might be waiting for an interrupt that never arrives, preventing forward progress.

**Investigation**:
- Added tracing to PIC interrupt injection
- Monitored `can_inject_interrupt()` status
- Checked if interrupts were disabled (RFLAGS.IF)

**Finding**: During the delay loops, interrupts were disabled (IF=0), which is expected during certain kernel operations. PIT timer interrupts were firing correctly (1000+ fires observed). The issue was not interrupt delivery.

### 2.4 Fourth Hypothesis: HLT Instruction Starvation (REJECTED)

**Theory**: Perhaps the kernel was executing HLT instructions expecting interrupts, but the emulator wasn't handling wakeup correctly.

**Investigation**:
- Added counter for HLT instruction execution
- Monitored HLT exit handling

**Finding**: Zero HLT instructions were executed during the hang period. The kernel was actively executing code, not waiting in HLT.

### 2.5 Fifth Hypothesis: Kernel Panic Detection (CONFIRMED)

**Theory**: The kernel might have panicked but the panic message wasn't being displayed.

**Investigation**:
- Used `addr2line` to identify function at return addresses
- Found that `delay_tsc` was being called from address `0xffffffff81206cc7`
- This address resolved to `vpanic` - the kernel panic handler!

**Finding**: The kernel WAS panicking. The "hang" was actually the panic handler's infinite blink loop, which calls `delay_tsc` repeatedly to create the panic LED blink pattern. The panic message wasn't appearing on serial console.

---

## 3. Root Cause Analysis

### 3.1 Capturing the Panic Message

Added instrumentation to detect entry to `vpanic` (at `0xffffffff81206a20`) and read the panic format string from RDI register:

```
[PANIC DETECTED] RIP=0xffffffff81206a20 fmt_ptr=0xffffffff8296f9b0
[PANIC MESSAGE] stack-protector: Kernel stack is corrupted in: %pB
```

### 3.2 Understanding Stack Protector

The Linux kernel's stack protector (CONFIG_STACKPROTECTOR) works by:
1. Placing a "canary" value on the stack at function entry
2. Checking the canary value before function return
3. If the canary is corrupted, calling `__stack_chk_fail` which triggers panic

The panic message indicates **stack memory corruption** - something overwrote the canary value (and likely other stack contents).

### 3.3 Stack Analysis at Crash Point

Instrumented `__stack_chk_fail` entry (at `0xffffffff82256ab0`) to dump stack state:

```
[__stack_chk_fail CALLED!]
RSP=0xffffffff82c03c88 RBP=0xffffffff82c03dd0
Stack dump:
  RSP+0x0: 0x0000000000000000    <-- Return address is ZERO (corrupted!)
  RSP+0x8: 0xffffffff82c03dd0    <-- Saved RBP
  RSP+0x10: 0xffffffff813600c0   <-- Return addr into local_clock
  RSP+0x18: 0xffffffff81375e31   <-- Return addr into vprintk_store
  RSP+0x20: 0x0000000000000000
  RSP+0x28: 0xffffffff82960988
  RSP+0x30: 0x0000000000000200
  RSP+0x38: 0x0000000000000000
```

**Key Observations**:
1. **Return address corrupted to 0x0**: The top of stack should contain a valid return address, but it's zero
2. **Stack location**: RSP is in the init_thread_union area (kernel's initial boot stack)
3. **Call chain reconstruction**:
   - `vprintk_store` (at `0xffffffff81375e31`)
   - → `local_clock` (at `0xffffffff813600c0`)
   - → `local_clock_noinstr`
   - → `__stack_chk_fail`

### 3.4 The Corrupted Function

The function with stack corruption is `local_clock_noinstr`:

```asm
ffffffff82256ae0 <local_clock_noinstr>:
ffffffff82256ae0:  endbr64
ffffffff82256ae4:  push   %rbp
ffffffff82256ae5:  push   %rbx
ffffffff82256ae6:  jmp    ffffffff82256b01
...
ffffffff82256b03:  mov    %gs:0x14f651d(%rip),%rax   # CRITICAL INSTRUCTION
ffffffff82256b0b:  lea    -0x7c89d140(%rax),%rbp
ffffffff82256b12:  lea    -0x7c89d130(%rax),%rbx
ffffffff82256b19:  call   sched_clock_noinstr
...
```

**Critical instruction at `0xffffffff82256b03`**:
```
65 48 8b 05 1d 65 4f 01    mov %gs:0x14f651d(%rip),%rax
```

This instruction uses **GS-segment-relative RIP-relative addressing** - a complex addressing mode that:
1. Calculates RIP-relative address: `next_RIP + 0x14f651d`
2. Adds GS.base to get the final linear address
3. Loads the 64-bit value at that address into RAX

---

## 4. Diagnosis

### 4.1 Primary Hypothesis: GS+RIP-Relative Addressing Bug

The most likely cause is incorrect handling of the **GS segment override combined with RIP-relative addressing**.

In x86-64, when both a segment override (GS) and RIP-relative addressing are used:
- The RIP-relative offset is calculated first
- Then the segment base is added

If the emulator incorrectly:
- Adds GS.base before calculating RIP-relative offset, OR
- Calculates the wrong effective address, OR
- Uses the wrong memory location for the read/write

...it could corrupt adjacent memory, including the stack.

### 4.2 Alternative Hypothesis: Stack Manipulation Bug

A bug in PUSH, POP, CALL, or RET instruction implementation could cause:
- Writing to wrong stack location
- Incorrect RSP adjustment
- Corrupting return addresses

### 4.3 Alternative Hypothesis: Memory Write Misdirection

A bug in any memory write instruction could cause writes to go to wrong addresses, potentially corrupting the stack even from unrelated code.

---

## 5. Evidence Summary

| Finding | Implication |
|---------|-------------|
| Panic message: "stack-protector: Kernel stack is corrupted" | Stack memory corruption detected |
| Return address on stack is 0x0 | Something wrote zero to stack |
| Crash in `local_clock_noinstr` | Function using complex addressing modes |
| Uses `mov %gs:disp(%rip),%rax` instruction | GS+RIP-relative addressing involved |
| Crash happens during printk path | Reproducible, happens every boot |
| GS.base appears correctly set | Basic GS handling works |
| delay_tsc loops work correctly | Basic instruction execution works |

---

## 6. Recommended Next Steps

### 6.1 Immediate Debugging

1. **Add memory write watchpoint**:
   ```rust
   // In write_mem or mmu.write_*:
   if addr >= 0xffffffff82c03c80 && addr <= 0xffffffff82c03c90 {
       eprintln!("[STACK WRITE] addr={:#x} value={:#x} RIP={:#x}",
                 addr, value, self.regs.rip);
   }
   ```
   This will catch the exact instruction that corrupts the return address.

2. **Verify GS+RIP-relative addressing**:
   Add specific tracing for instructions that use both GS prefix and RIP-relative addressing (ModRM with mod=00, r/m=5, and segment override 0x65).

3. **Test with stack protector disabled**:
   Rebuild kernel with `CONFIG_STACKPROTECTOR=n` to see if execution continues past the corruption point (won't fix the bug, but may reveal more symptoms).

### 6.2 Code Review Areas

1. **`decode_modrm_addr` in decoder.rs** (lines 150-272):
   - Verify RIP-relative address calculation when segment override is present
   - Check order of operations: RIP-relative first, then segment base

2. **Stack manipulation instructions**:
   - `insn/data/push.rs` - PUSH implementations
   - `insn/data/pop.rs` - POP implementations
   - `insn/control/call.rs` - CALL implementations
   - `insn/control/ret.rs` - RET implementations

3. **Memory write path**:
   - `mmu.rs` - All write_* functions
   - Verify address translation is correct

### 6.3 Test Case Development

Create a minimal test case that exercises GS+RIP-relative addressing:
```asm
; Test: GS-relative RIP-relative MOV
mov $0x1000, %rax
wrmsr  ; Set GS.base to known value

mov %gs:some_offset(%rip), %rax  ; This addressing mode
; Verify RAX contains expected value
```

---

## 7. Files Modified During Investigation

Debug instrumentation was added to these files (should be cleaned up):

| File | Debug Added |
|------|-------------|
| `src/backend/emulator/x86_64/cpu.rs` | vpanic/stack_chk_fail detection, RIP history |
| `src/backend/emulator/x86_64/decoder.rs` | GS prefix tracing (cleaned) |
| `src/backend/emulator/x86_64/insn/system/msr.rs` | WRMSR GS_BASE tracing (cleaned) |
| `src/backend/emulator/x86_64/insn/system/timing.rs` | RDTSC tracing (cleaned) |

---

## 8. Conclusion

The Linux kernel panic is caused by **stack memory corruption** originating in the emulator's instruction execution. The corruption occurs during execution of `local_clock_noinstr`, a function that uses GS-segment-relative RIP-relative memory addressing.

The most probable root cause is a bug in how the emulator handles this complex addressing mode, causing either:
1. Incorrect memory read/write addresses
2. Corruption of adjacent memory (the stack)

The fix requires careful review of the segment override + RIP-relative addressing code path in the x86_64 decoder and memory subsystem.

---

*Document generated: 2025-12-26*
*Emulator: rax x86_64*
*Kernel: Linux 6.19.0-rc2*
