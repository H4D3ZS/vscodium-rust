#!/usr/bin/env python3
"""Generate an exhaustive A32 + T16/T32 instruction encoding table for the
AArch32 differential sweep (tests/arm_diff32.rs).

Mirrors the NEON sweep generator: enumerate every register-data-processing
mnemonic across the operand variations that affect *semantics* (opcode, S-bit,
shift type/amount, immediate, sign, element size), with register fields fixed to
a safe low-register set, assemble each with llvm-mc, and emit a Rust table:

    pub static A32_SWEEP: &[(&str, u32, u8)] = &[ (label, insn, mode), ... ];

mode: 0 = ARM (A32), 1 = Thumb 16-bit, 2 = Thumb 32-bit.

Memory and VFP/NEON families are emitted into separate tables so the integer
core can be driven to zero-divergence first.
"""
import subprocess, sys, os, re

LLVM_MC = os.environ.get("LLVM_MC", "llvm-mc")

# ---------------------------------------------------------------------------
# Instruction template enumeration.  Each entry is an assembly string; register
# operands are fixed (r0=dest, r1/r2/r3 = sources) so only the semantic fields
# vary.  Shift amounts, immediates, sizes and sign variants are swept.
# ---------------------------------------------------------------------------

def dp_reg():
    out = []
    ops3 = ["and", "eor", "sub", "rsb", "add", "adc", "sbc", "rsc",
            "orr", "bic"]
    ops2 = ["mov", "mvn"]          # single source
    cmp_ops = ["tst", "teq", "cmp", "cmn"]
    shifts = [None, ("lsl", [0, 1, 7, 15, 31]), ("lsr", [1, 7, 31, 32]),
              ("asr", [1, 7, 31, 32]), ("ror", [1, 7, 31]), ("rrx", [None])]
    for s in ("", "s"):
        for op in ops3:
            for sh in shifts:
                if sh is None:
                    out.append(f"{op}{s} r0, r1, r2")
                else:
                    name, amts = sh
                    for a in amts:
                        if name == "rrx":
                            out.append(f"{op}{s} r0, r1, r2, rrx")
                        else:
                            out.append(f"{op}{s} r0, r1, r2, {name} #{a}")
            # register-controlled shift
            for name in ("lsl", "lsr", "asr", "ror"):
                out.append(f"{op}{s} r0, r1, r2, {name} r3")
        for op in ops2:
            out.append(f"{op}{s} r0, r2")
            for name in ("lsl", "lsr", "asr", "ror"):
                for a in (1, 7, 31):
                    out.append(f"{op}{s} r0, r2, {name} #{a}")
            out.append(f"{op}{s} r0, r2, rrx")
            for name in ("lsl", "lsr", "asr", "ror"):
                out.append(f"{op}{s} r0, r2, {name} r3")
    for op in cmp_ops:
        out.append(f"{op} r1, r2")
        out.append(f"{op} r1, r2, lsl #3")
        out.append(f"{op} r1, r2, asr #3")
    return out

def dp_imm():
    out = []
    ops3 = ["and", "eor", "sub", "rsb", "add", "adc", "sbc", "rsc",
            "orr", "bic"]
    imms = [0, 1, 0xff, 0x100, 0xf000000f, 0x80000000, 0x3fc, 0xcafe00]
    # only encodable modified-immediates survive llvm-mc; the rest are dropped
    for s in ("", "s"):
        for op in ops3:
            for i in imms:
                out.append(f"{op}{s} r0, r1, #{i}")
        for op in ("mov", "mvn"):
            for i in imms:
                out.append(f"{op}{s} r0, #{i}")
    for op in ("tst", "teq", "cmp", "cmn"):
        for i in imms:
            out.append(f"{op} r1, #{i}")
    return out

def movw_movt():
    out = []
    for i in (0, 1, 0xffff, 0x1234, 0x8000):
        out.append(f"movw r0, #{i}")
        out.append(f"movt r0, #{i}")
    return out

def multiply():
    out = ["mul r0, r1, r2", "muls r0, r1, r2",
           "mla r0, r1, r2, r3", "mlas r0, r1, r2, r3",
           "mls r0, r1, r2, r3",
           "umull r0, r1, r2, r3", "umulls r0, r1, r2, r3",
           "umlal r0, r1, r2, r3", "umlals r0, r1, r2, r3",
           "smull r0, r1, r2, r3", "smulls r0, r1, r2, r3",
           "smlal r0, r1, r2, r3", "smlals r0, r1, r2, r3",
           "umaal r0, r1, r2, r3"]
    # DSP halfword multiplies
    for x in ("b", "t"):
        for y in ("b", "t"):
            out.append(f"smul{x}{y} r0, r1, r2")
            out.append(f"smla{x}{y} r0, r1, r2, r3")
    for y in ("b", "t"):
        out.append(f"smulw{y} r0, r1, r2")
        out.append(f"smlaw{y} r0, r1, r2, r3")
    for x in ("", "x"):
        out.append(f"smuad{x} r0, r1, r2")
        out.append(f"smusd{x} r0, r1, r2")
        out.append(f"smlad{x} r0, r1, r2, r3")
        out.append(f"smlsd{x} r0, r1, r2, r3")
        out.append(f"smlald{x} r0, r1, r2, r3")
        out.append(f"smlsld{x} r0, r1, r2, r3")
    for r in ("", "r"):
        out.append(f"smmul{r} r0, r1, r2")
        out.append(f"smmla{r} r0, r1, r2, r3")
        out.append(f"smmls{r} r0, r1, r2, r3")
    return out

def saturating():
    out = []
    for op in ("qadd", "qsub", "qdadd", "qdsub"):
        out.append(f"{op} r0, r1, r2")
    for n in (0, 1, 7, 15, 31):
        out.append(f"ssat r0, #{n+1}, r1")
        out.append(f"ssat r0, #{n+1}, r1, lsl #3")
        out.append(f"ssat r0, #{n+1}, r1, asr #3")
    for n in (0, 7, 15, 31):
        out.append(f"usat r0, #{n}, r1")
        out.append(f"usat r0, #{n}, r1, lsl #3")
    for n in (1, 8, 16):
        out.append(f"ssat16 r0, #{n}, r1")
    for n in (0, 8, 15):
        out.append(f"usat16 r0, #{n}, r1")
    return out

def parallel():
    out = []
    pfx = ["s", "q", "sh", "u", "uq", "uh"]
    base = ["add8", "add16", "sub8", "sub16", "asx", "sax"]
    for p in pfx:
        for b in base:
            out.append(f"{p}{b} r0, r1, r2")
    return out

def pack_extend():
    out = ["pkhbt r0, r1, r2", "pkhbt r0, r1, r2, lsl #4",
           "pkhtb r0, r1, r2, asr #4", "pkhtb r0, r1, r2, asr #16"]
    for op in ("sxtb", "sxth", "uxtb", "uxth", "sxtb16", "uxtb16"):
        out.append(f"{op} r0, r1")
        for rot in (8, 16, 24):
            out.append(f"{op} r0, r1, ror #{rot}")
    for op in ("sxtab", "sxtah", "uxtab", "uxtah", "sxtab16", "uxtab16"):
        out.append(f"{op} r0, r1, r2")
        for rot in (8, 16, 24):
            out.append(f"{op} r0, r1, r2, ror #{rot}")
    return out

def memory_a32():
    """A32 load/store with base register r1 (pointed at the scratch window by the
    harness) and offset register r2 (held small). Rt low regs; LDRD/STRD use r4."""
    out = []
    # Single load/store, immediate offset (offset / pre-index / post-index).
    for ld in ("ldr", "ldrb", "ldrh", "ldrsb", "ldrsh", "str", "strb", "strh"):
        for off in (0, 4, 8, 64, -4, -64):
            out.append(f"{ld} r0, [r1, #{off}]")          # offset
            out.append(f"{ld} r0, [r1, #{off}]!")         # pre-index
            out.append(f"{ld} r0, [r1], #{off}")          # post-index
        # register offset (r2 held small by the harness)
        out.append(f"{ld} r0, [r1, r2]")
        out.append(f"{ld} r0, [r1, r2, lsl #2]")
        out.append(f"{ld} r0, [r1, -r2]")
    # Load/store dual (even Rt, base r1).
    for off in (0, 8, 64, -8):
        out.append(f"ldrd r4, r5, [r1, #{off}]")
        out.append(f"strd r4, r5, [r1, #{off}]")
        out.append(f"ldrd r4, r5, [r1, #{off}]!")
        out.append(f"strd r4, r5, [r1, #{off}]!")
        out.append(f"ldrd r4, r5, [r1], #{off}")
        out.append(f"strd r4, r5, [r1], #{off}")
    # Load/store multiple (base r1, low-register lists; with/without writeback).
    for mode in ("ia", "ib", "da", "db"):
        out.append(f"ldm{mode} r1, {{r0, r3, r5}}")
        out.append(f"stm{mode} r1, {{r0, r3, r5}}")
        out.append(f"ldm{mode} r1!, {{r0, r3, r5}}")
        out.append(f"stm{mode} r1!, {{r0, r3, r5}}")
    return out


def bitops():
    out = ["clz r0, r1", "rbit r0, r1", "rev r0, r1", "rev16 r0, r1",
           "revsh r0, r1", "sel r0, r1, r2", "usad8 r0, r1, r2",
           "usada8 r0, r1, r2, r3"]
    for lsb in (0, 3, 16, 31):
        for w in (1, 8, 16):
            if lsb + w <= 32:
                out.append(f"sbfx r0, r1, #{lsb}, #{w}")
                out.append(f"ubfx r0, r1, #{lsb}, #{w}")
    for lsb in (0, 4, 16):
        for w in (1, 8, 15):
            if lsb + w <= 32:
                out.append(f"bfi r0, r1, #{lsb}, #{w}")
    for lsb in (0, 8):
        for w in (1, 16):
            if lsb + w <= 32:
                out.append(f"bfc r0, #{lsb}, #{w}")
    return out

# ---------------------------------------------------------------------------
# Assemble a list of mnemonics with llvm-mc for the given triple.  Returns
# [(asm, byte_list)], dropping any that fail to encode.
# ---------------------------------------------------------------------------
def assemble(lines, triple, mattr=""):
    args = [LLVM_MC, f"--triple={triple}", "--show-encoding"]
    if mattr:
        args.append(f"--mattr={mattr}")
    res = {}
    # Run one at a time so a single bad line doesn't abort the batch.
    # Batch in groups for speed, falling back to per-line on error.
    def run_batch(batch):
        p = subprocess.run(args, input="\n".join(batch) + "\n",
                           capture_output=True, text=True)
        if p.returncode != 0:
            return None
        return p.stdout
    def parse(stdout, batch):
        # Map encoding lines back to inputs in order.
        encs = re.findall(r"encoding:\s*\[([^\]]*)\]", stdout)
        if len(encs) != len(batch):
            return None
        for asm, e in zip(batch, encs):
            bs = [int(x, 16) for x in e.split(",")]
            res[asm] = bs
        return True
    # Try the whole thing in chunks; on failure, bisect down to per-line.
    def process(batch):
        if not batch:
            return
        out = run_batch(batch)
        if out is not None and parse(out, batch):
            return
        if len(batch) == 1:
            return  # drop the un-encodable line
        mid = len(batch) // 2
        process(batch[:mid])
        process(batch[mid:])
    process(lines)
    return res

def to_insn(bs, mode):
    if mode == 0:  # ARM word
        assert len(bs) == 4
        return bs[0] | (bs[1] << 8) | (bs[2] << 16) | (bs[3] << 24)
    if len(bs) == 2:  # T16
        return (bs[0] | (bs[1] << 8), 1)
    # T32: hw1 = bytes[0:2], hw2 = bytes[2:4]
    hw1 = bs[0] | (bs[1] << 8)
    hw2 = bs[2] | (bs[3] << 8)
    return ((hw1 << 16) | hw2, 2)

def main():
    families = {
        "dp_reg": dp_reg(), "dp_imm": dp_imm(), "movw_movt": movw_movt(),
        "multiply": multiply(), "saturating": saturating(),
        "parallel": parallel(), "pack_extend": pack_extend(),
        "bitops": bitops(),
    }
    all_lines = []
    seen = set()
    for fam, lines in families.items():
        for ln in lines:
            if ln not in seen:
                seen.add(ln)
                all_lines.append(ln)

    def build(lines):
        arm = assemble(lines, "armv7a", "+dsp")
        thumb = assemble(lines, "thumbv7a", "+dsp")
        entries = []
        dedup = set()
        for asm in lines:
            if asm in arm:
                v = to_insn(arm[asm], 0)
                if (v, 0) not in dedup:
                    dedup.add((v, 0))
                    entries.append((f"A32 {asm}", v, 0))
            if asm in thumb:
                insn, mode = to_insn(thumb[asm], 99)
                if (insn, mode) not in dedup:
                    dedup.add((insn, mode))
                    tag = "T16" if mode == 1 else "T32"
                    entries.append((f"{tag} {asm}", insn, mode))
        return entries

    entries = build(all_lines)

    # Memory family (separate table; harness installs the scratch base/offset).
    mem_lines = []
    mem_seen = set()
    for ln in memory_a32():
        if ln not in mem_seen:
            mem_seen.add(ln)
            mem_lines.append(ln)
    mem_entries = build(mem_lines)

    out = os.path.join(os.path.dirname(__file__), "..", "..", "tests", "arm32_gen.rs")
    out = os.path.normpath(out)
    with open(out, "w") as f:
        f.write("// AUTO-GENERATED by tools/arm-diff/gen_a32.py -- DO NOT EDIT.\n")
        f.write("// Exhaustive A32 + T16/T32 encoding tables for the AArch32 sweep.\n")
        f.write("// Tuple: (label, instruction-word, mode) where mode 0=ARM 1=T16 2=T32.\n\n")
        for name, ents in (("A32_SWEEP", entries), ("A32_MEM_SWEEP", mem_entries)):
            f.write(f"pub static {name}: &[(&str, u32, u8)] = &[\n")
            for label, insn, mode in ents:
                esc = label.replace("\\", "\\\\").replace('"', '\\"')
                f.write(f'    ("{esc}", {insn:#010x}, {mode}),\n')
            f.write("];\n\n")

    def stats(ents):
        return (sum(1 for e in ents if e[2] == 0),
                sum(1 for e in ents if e[2] == 1),
                sum(1 for e in ents if e[2] == 2))
    print(f"wrote {len(entries)} integer + {len(mem_entries)} memory entries to {out}")
    print(f"  integer A32/T16/T32 = {stats(entries)}")
    print(f"  memory  A32/T16/T32 = {stats(mem_entries)}")

if __name__ == "__main__":
    main()
