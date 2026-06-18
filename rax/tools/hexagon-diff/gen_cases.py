#!/usr/bin/env python3
"""Generate a Hexagon differential-test corpus from the spec semantics.

Reads the fully macro-expanded semantics dump produced by gen_semantics.c
(tools/hexagon/qemu/semantics_generated.pyinc) and, for every *register-only*
scalar instruction (no memory, no control flow, no HVX, no system/control
registers), synthesises a concrete assembly packet by substituting fixed
operands. Each candidate is assembled with llvm-mc; only the ones that assemble
cleanly are emitted.

Output (cases.txt), one instruction per line, tab-separated:
    TAG \\t ASM \\t w0,w1,...        (machine words, little-endian hex)

The Rust harness (tests/hexagon_diff.rs) consumes this corpus: it feeds the
words to both the qemu-hexagon oracle and the rax interpreter and reports which
instructions rax executes correctly, rejects, or diverges on.

Operand register assignment keeps destinations and sources disjoint so writes
are observable and source values are never clobbered before use:
    d  -> R0   (dd -> R1:0)        x  -> R0   (xx -> R1:0)   [read-modify dest]
    e  -> R2
    s  -> R4   (ss -> R5:4)
    t  -> R6   (tt -> R7:6)
    u  -> R8   (uu -> R9:8)
    v  -> R10  (vv -> R11:10)
    y  -> R12  (yy -> R13:12)
    Pd -> P0  Pe -> P1  Ps -> P1  Pt -> P2  Pu -> P3  Pv -> P1  Px -> P0
"""

import os
import re
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
SEM_PATH = os.path.join(HERE, "..", "hexagon", "qemu", "semantics_generated.pyinc")
OUT_PATH = os.path.join(HERE, "cases.txt")
MCPU = os.environ.get("HEX_MCPU", "hexagonv68")

# Attributes that make an instruction unsuitable for a register-only state diff.
SKIP_ATTRS = {
    "A_LOAD", "A_STORE", "A_MEMLIKE", "A_MEMLIKE_PACKET_RULES",
    "A_CALL", "A_INDIRECT", "A_RETURN", "A_COF", "A_HWLOOP0_SETUP",
    "A_HWLOOP1_SETUP", "A_BRANCHADDER", "A_DEALLOCRET", "A_DEALLOCFRAME",
    "A_EXTENSION", "A_CVI", "A_PRIV", "A_GUEST", "A_HVX_KIND",
    "A_RESTRICT_SLOT0ONLY_HVX",
}
# Whole syntactic kinds we don't yet drive (need V state / system regs / COF).
SKIP_SYNTAX_TOKENS = (
    "Cd", "Cs", "Cdd", "Css",        # control registers
    "Sd", "Ss", "Sdd", "Sss",        # supervisor registers
    "Gd", "Gs", "Gdd", "Gss",        # guest registers
    "Vd", "Vs", "Vt", "Vu", "Vv", "Vw", "Vx", "Vy",  # HVX vector
    "Qd", "Qs", "Qt", "Qu", "Qv", "Qx",              # HVX predicate
    "Nt8", "Ns8",                    # new-value (store-only)
    "mem", "dcfetch", "dczero", "trap", "pause", "brkpt",
    "jump", "call", "endloop", "loop", "deallocframe", "dealloc_return",
    "allocframe", "icinva", "isync", "barrier", "syncht", "tlb",
    "Mu",                            # modifier register (addressing)
    "Rd16", "Rs16", "Rx16", "Rt16",  # sub-insn (handled separately)
    "gp", "sp", "framelimit", "framekey", "pc",
)

REG_PAIR = {"dd": 0, "ss": 4, "tt": 6, "uu": 8, "vv": 10, "xx": 0, "yy": 12}
REG_SINGLE = {"d": 0, "e": 2, "s": 4, "t": 6, "u": 8, "v": 10, "x": 0, "y": 12}
PRED = {"d": 0, "e": 1, "s": 1, "t": 2, "u": 3, "v": 1, "x": 0}


def quoted_segments(line):
    """Return the concatenation of all double-quoted substrings on a line."""
    return "".join(re.findall(r'"((?:[^"\\]|\\.)*)"', line))


def parse_semantics(path):
    """Yield (tag, syntax) from the SEMANTICS() blocks."""
    lines = open(path).read().splitlines()
    i = 0
    out = []
    while i < len(lines):
        if lines[i].lstrip().startswith("SEMANTICS("):
            tag = quoted_segments(lines[i + 1]) if i + 1 < len(lines) else ""
            syn = quoted_segments(lines[i + 2]) if i + 2 < len(lines) else ""
            out.append((tag, syn))
            i += 4
        else:
            i += 1
    return out


def parse_attributes(path):
    lines = open(path).read().splitlines()
    attrs = {}
    i = 0
    while i < len(lines):
        if lines[i].lstrip().startswith("ATTRIBUTES("):
            tag = quoted_segments(lines[i + 1]) if i + 1 < len(lines) else ""
            blob = quoted_segments(lines[i + 2]) if i + 2 < len(lines) else ""
            blob = blob.replace("ATTRIBS", "").replace("(", "").replace(")", "")
            attrs[tag] = [a.strip() for a in blob.split(",") if a.strip()]
            i += 4
        else:
            i += 1
    return attrs


# Number of immediate-value variants generated per instruction (covers small,
# zero, max, min and a mid value so immediate-width/offset bugs are exercised).
N_IMM_VARIANTS = 5


def imm_value(letter, width, scale, variant):
    """Concrete, in-range, scale-aligned immediate for a #field and variant, or
    None if this variant collapses to one already covered."""
    step = 1 << scale
    if letter in ("u", "U", "m", "g"):  # unsigned-ish
        maxk = (1 << width) - 1
        ks = [min(3, maxk), 0, maxk, 1, maxk // 2]
    else:  # s, S, r -- signed
        maxk = (1 << (width - 1)) - 1
        mink = -(1 << (width - 1))
        ks = [-3 if maxk >= 3 else mink, 0, maxk, mink, maxk // 2]
    if variant >= len(ks):
        return None
    return ks[variant] * step


def substitute(syntax, variant=0):
    """Turn a spec syntax string into concrete assembly for the given immediate
    variant, or None if unsupported / the variant is out of range."""
    s = syntax.strip()
    # Reject anything referencing an unsupported operand kind / mnemonic.
    for tok in SKIP_SYNTAX_TOKENS:
        if tok in s:
            return None

    # Register pairs first (doubled lowercase letter + 32): Rdd32, Rss32, ...
    def repl_pair(m):
        letter = m.group(1) + m.group(1)
        if letter not in REG_PAIR:
            return m.group(0)
        return "R%d:%d" % (REG_PAIR[letter] + 1, REG_PAIR[letter])
    s = re.sub(r"R([dsteuvxy])\1(?:32)?", repl_pair, s)

    # Half-register operands: Rs.H32 / Rt.L32 -> R4.h / R6.l
    def repl_half(m):
        letter, hl = m.group(1), m.group(2)
        return "R%d.%s" % (REG_SINGLE[letter], hl.lower()) if letter in REG_SINGLE else m.group(0)
    s = re.sub(r"R([dsteuvxy])\.([HL])32", repl_half, s)

    # Single GPRs: Rd32, Rs32, ...
    def repl_single(m):
        letter = m.group(1)
        return "R%d" % REG_SINGLE[letter] if letter in REG_SINGLE else m.group(0)
    s = re.sub(r"R([dsteuvxy])32", repl_single, s)

    # Predicates: Pd4, Ps4, ... (also bare Pu4 used as a condition).
    def repl_pred(m):
        letter = m.group(1)
        return "P%d" % PRED[letter] if letter in PRED else m.group(0)
    s = re.sub(r"P([dsteuvx])4", repl_pred, s)

    # Immediates: #<letter><width>[:<scale>]
    collapsed = [False]

    def repl_imm(m):
        letter, width = m.group(1), int(m.group(2))
        scale = int(m.group(3)) if m.group(3) else 0
        v = imm_value(letter, width, scale, variant)
        if v is None:
            collapsed[0] = True
            return m.group(0)
        return "#%d" % v
    s = re.sub(r"#([usUSmgr])(\d+)(?::(\d+))?", repl_imm, s)
    if collapsed[0]:
        return None

    # If any spec placeholder survived, we couldn't fully concretise it.
    if re.search(r"R[dsteuvxy](32|[dsteuvxy])|P[dsteuvx]4|#[a-zA-Z]\d", s):
        return None
    return s


def assemble(asm):
    """Assemble one packet; return list of LE words, or None if it won't build."""
    src = "{ %s }\n" % asm
    p = subprocess.run(
        ["llvm-mc", "-triple=hexagon", "-mcpu=" + MCPU, "-show-encoding"],
        input=src.encode(), capture_output=True,
    )
    if p.returncode != 0:
        return None
    words = []
    for line in p.stdout.decode().splitlines():
        m = re.search(r"encoding: \[([0-9a-fx,\s]+)\]", line)
        if not m:
            continue
        bs = [int(x.strip(), 16) for x in m.group(1).split(",")]
        for i in range(0, len(bs), 4):
            if i + 3 < len(bs):
                words.append(bs[i] | bs[i + 1] << 8 | bs[i + 2] << 16 | bs[i + 3] << 24)
    return words or None


def ensure_semantics():
    """Build+run gen_semantics.c to produce the expanded semantics dump if it
    is missing (it is a generated artifact, not checked in)."""
    if os.path.exists(SEM_PATH):
        return
    qemu_dir = os.path.join(HERE, "..", "hexagon", "qemu")
    binp = os.path.join(qemu_dir, "gen_semantics")
    subprocess.run(
        ["cc", "-O2", "-I", qemu_dir, "-o", binp, os.path.join(qemu_dir, "gen_semantics.c")],
        check=True,
    )
    subprocess.run([binp, SEM_PATH], check=True)


def main():
    ensure_semantics()
    sem = parse_semantics(SEM_PATH)
    attrs = parse_attributes(SEM_PATH)

    emitted_tags, skipped_attr, skipped_syn, skipped_asm = 0, 0, 0, 0
    n_cases = 0
    lines = []
    for tag, syntax in sem:
        a = set(attrs.get(tag, []))
        if a & SKIP_ATTRS:
            skipped_attr += 1
            continue
        # Generate distinct immediate variants so immediate-dependent bugs
        # (extract/insert widths, shift amounts, ...) are exercised.
        seen_asm = set()
        any_asm, any_ok = False, False
        for variant in range(N_IMM_VARIANTS):
            asm = substitute(syntax, variant)
            if asm is None or asm in seen_asm:
                continue
            seen_asm.add(asm)
            any_asm = True
            words = assemble(asm)
            if words is None:
                continue
            any_ok = True
            words_hex = ",".join("%08x" % w for w in words)
            lines.append("%s\t%s\t%s" % (tag, asm, words_hex))
            n_cases += 1
        if not any_asm:
            skipped_syn += 1
        elif not any_ok:
            skipped_asm += 1
        else:
            emitted_tags += 1

    lines.sort()
    with open(OUT_PATH, "w") as f:
        f.write("# Generated by tools/hexagon-diff/gen_cases.py from the Hexagon spec.\n")
        f.write("# TAG<TAB>ASM<TAB>word0,word1,...   (mcpu=%s)\n" % MCPU)
        f.write("\n".join(lines) + "\n")

    sys.stderr.write(
        "tags=%d cases=%d  skipped(attr=%d syntax=%d asm=%d)  total=%d\n"
        % (emitted_tags, n_cases, skipped_attr, skipped_syn, skipped_asm, len(sem))
    )


if __name__ == "__main__":
    main()
