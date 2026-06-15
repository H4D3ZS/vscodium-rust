/*
 * AArch64 differential-test oracle.
 *
 * Built as a *static* AArch64 ELF and executed under `qemu-aarch64` (user mode)
 * on an x86 host. It is the hardware-semantics reference against which the rax
 * software interpreter (src/arm/aarch64/cpu.rs) is checked.
 *
 * Protocol (little-endian binary, over stdin -> stdout):
 *   stdin:  u32 magic 'A','R','M','1' (0x314d5241)
 *           u32 count
 *           count * struct InCase
 *   stdout: u32 magic (echoed)
 *           u32 count
 *           count * struct OutCase
 *
 * Each case loads the full architectural register file (X0..X30, SP, NZCV via
 * PSTATE, FPSR, FPCR, V0..V31) from an input block, executes up to three
 * instruction words, and captures the resulting register file.
 *
 * Mechanism: qemu-user restores GPRs from a signal frame but NOT the SIMD/FP
 * registers, so inputs cannot be installed purely through the frame. Instead a
 * fixed assembly *prologue* (placed in an executable page) loads every register
 * from the input block, then runs the patched test instruction, then `BRK #0`.
 * A SIGTRAP handler captures the post-instruction frame (GPRs, SP, PSTATE and
 * the FPSIMD record all read back correctly) and restores the harness context.
 */
#define _GNU_SOURCE
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <signal.h>
#include <ucontext.h>
#include <sys/mman.h>
#include <sys/prctl.h>
#include <unistd.h>

/* ------------------------------------------------------------------ */
/* Wire format -- must match tests/arm_diff.rs ArmState exactly.       */
/* All-u64 layout (8-byte aligned, no padding). V-registers are stored */
/* as lo/hi u64 pairs: v[2*r], v[2*r+1].                               */
/* ------------------------------------------------------------------ */

typedef struct {
    uint64_t x[31];   /* X0..X30                          */
    uint64_t sp;      /* SP                               */
    uint64_t pc;      /* set by harness; output = post-pc */
    uint64_t pstate;  /* NZCV in bits [31:28]             */
    uint64_t fpsr;
    uint64_t fpcr;
    uint64_t v[64];   /* V0..V31 as lo/hi u64 pairs       */
    uint64_t scratch[32]; /* contents of the shared scratch window (256 bytes) */
    uint64_t preds[4]; /* SVE P0..P15 packed: 16 x 16-bit (VL=128), 2 bytes each */
} ArmState;

/* SVE signal-frame record. At VL=128 (vq=1) the Z registers alias V, and the
 * 16 predicate registers are 2 bytes each. Record layout: 16-byte header, then
 * Z[32]*16B (=512), then P[16]*2B (=32) at +528, then FFR at +560. */
#ifndef SVE_MAGIC
#define SVE_MAGIC 0x53564501u
#endif
#define SVE_PREGS_OFFSET 528
#define SVE_RECORD_FULL  560

/* Shared scratch memory for load/store tests. The window is MAP_FIXED so the
 * same numeric address is valid in both qemu-user and the rax FlatMemory.
 * Tests point a base register at SCRATCH_BASE (offsets in [-64, +191] stay in
 * the exchanged window). */
#define SCRATCH_ADDR 0x200000ull
#define SCRATCH_SIZE 4096
#define SCRATCH_BASE (SCRATCH_ADDR + 64)

/* Optional test-only relocation mode for control-flow instructions. If an
 * input case sets ArmState.pc to PCREL_MAGIC, any X register whose high 24 bits
 * match PCREL_TOKEN is rewritten to harness_testslot + signed_low32 before the
 * test runs. Captured code-page addresses are then normalized back to offsets
 * from harness_testslot. */
#define PCREL_MAGIC 0x524158504352454cull /* "RAXPCREL" */
#define PCREL_MASK  0xffffff0000000000ull
#define PCREL_TOKEN 0x5241580000000000ull /* high 24 bits: "RAX" */

typedef struct {
    uint32_t insn;    /* instruction word under test      */
    uint32_t flags;   /* optional second instruction      */
    uint32_t insn3;   /* optional third instruction       */
    uint32_t reserved;
    ArmState st;      /* input architectural state        */
} InCase;

typedef struct {
    ArmState st;      /* output architectural state       */
    uint32_t trapped; /* signal number if the insn faulted, else 0 */
    uint32_t valid;   /* 1 = executed and captured        */
} OutCase;

/* FPSIMD record in the signal frame. */
struct fpsimd_ctx_hdr { uint32_t magic; uint32_t size; };
#ifndef FPSIMD_MAGIC
#define FPSIMD_MAGIC 0x46508001u
#endif

/* ------------------------------------------------------------------ */
/* Register-loading prologue template (assembled into .text, copied to */
/* an executable page). On entry x0 points at the input ArmState block.*/
/* The instruction after the prologue is patched per test case.        */
/* ------------------------------------------------------------------ */
extern const uint32_t harness_prologue[];
extern const uint32_t harness_testslot[];
extern const uint32_t harness_end[];

__asm__(
    ".pushsection .text\n"
    ".arch armv8-a+sve\n"
    ".balign 4\n"
    ".global harness_prologue\n"
    ".global harness_testslot\n"
    ".global harness_end\n"
    "harness_prologue:\n"
    "    ldr q0, [x0, #288]\n"
    "    ldr q1, [x0, #304]\n"
    "    ldr q2, [x0, #320]\n"
    "    ldr q3, [x0, #336]\n"
    "    ldr q4, [x0, #352]\n"
    "    ldr q5, [x0, #368]\n"
    "    ldr q6, [x0, #384]\n"
    "    ldr q7, [x0, #400]\n"
    "    ldr q8, [x0, #416]\n"
    "    ldr q9, [x0, #432]\n"
    "    ldr q10, [x0, #448]\n"
    "    ldr q11, [x0, #464]\n"
    "    ldr q12, [x0, #480]\n"
    "    ldr q13, [x0, #496]\n"
    "    ldr q14, [x0, #512]\n"
    "    ldr q15, [x0, #528]\n"
    "    ldr q16, [x0, #544]\n"
    "    ldr q17, [x0, #560]\n"
    "    ldr q18, [x0, #576]\n"
    "    ldr q19, [x0, #592]\n"
    "    ldr q20, [x0, #608]\n"
    "    ldr q21, [x0, #624]\n"
    "    ldr q22, [x0, #640]\n"
    "    ldr q23, [x0, #656]\n"
    "    ldr q24, [x0, #672]\n"
    "    ldr q25, [x0, #688]\n"
    "    ldr q26, [x0, #704]\n"
    "    ldr q27, [x0, #720]\n"
    "    ldr q28, [x0, #736]\n"
    "    ldr q29, [x0, #752]\n"
    "    ldr q30, [x0, #768]\n"
    "    ldr q31, [x0, #784]\n"
    /* Load SVE predicate registers P0..P15 from preds[] at offset 1056. The
     * `mul vl` multiplier is the predicate size in bytes (VL/64 = 2 at VL=128),
     * so successive imm offsets address the contiguous 2-byte predicates. This
     * also marks the predicates live so the captured sigframe is full-size. */
    "    add x1, x0, #1056\n"
    "    ldr p0,  [x1, #0,  mul vl]\n"
    "    ldr p1,  [x1, #1,  mul vl]\n"
    "    ldr p2,  [x1, #2,  mul vl]\n"
    "    ldr p3,  [x1, #3,  mul vl]\n"
    "    ldr p4,  [x1, #4,  mul vl]\n"
    "    ldr p5,  [x1, #5,  mul vl]\n"
    "    ldr p6,  [x1, #6,  mul vl]\n"
    "    ldr p7,  [x1, #7,  mul vl]\n"
    "    ldr p8,  [x1, #8,  mul vl]\n"
    "    ldr p9,  [x1, #9,  mul vl]\n"
    "    ldr p10, [x1, #10, mul vl]\n"
    "    ldr p11, [x1, #11, mul vl]\n"
    "    ldr p12, [x1, #12, mul vl]\n"
    "    ldr p13, [x1, #13, mul vl]\n"
    "    ldr p14, [x1, #14, mul vl]\n"
    "    ldr p15, [x1, #15, mul vl]\n"
    "    ldr w1, [x0, #280]\n"
    "    msr fpcr, x1\n"
    "    ldr w1, [x0, #272]\n"
    "    msr fpsr, x1\n"
    "    ldr x1, [x0, #264]\n"
    "    msr nzcv, x1\n"
    "    ldr x1, [x0, #248]\n"
    "    mov sp, x1\n"
    "    ldr x30, [x0, #240]\n"
    "    ldr x29, [x0, #232]\n"
    "    ldr x28, [x0, #224]\n"
    "    ldr x27, [x0, #216]\n"
    "    ldr x26, [x0, #208]\n"
    "    ldr x25, [x0, #200]\n"
    "    ldr x24, [x0, #192]\n"
    "    ldr x23, [x0, #184]\n"
    "    ldr x22, [x0, #176]\n"
    "    ldr x21, [x0, #168]\n"
    "    ldr x20, [x0, #160]\n"
    "    ldr x19, [x0, #152]\n"
    "    ldr x18, [x0, #144]\n"
    "    ldr x17, [x0, #136]\n"
    "    ldr x16, [x0, #128]\n"
    "    ldr x15, [x0, #120]\n"
    "    ldr x14, [x0, #112]\n"
    "    ldr x13, [x0, #104]\n"
    "    ldr x12, [x0, #96]\n"
    "    ldr x11, [x0, #88]\n"
    "    ldr x10, [x0, #80]\n"
    "    ldr x9, [x0, #72]\n"
    "    ldr x8, [x0, #64]\n"
    "    ldr x7, [x0, #56]\n"
    "    ldr x6, [x0, #48]\n"
    "    ldr x5, [x0, #40]\n"
    "    ldr x4, [x0, #32]\n"
    "    ldr x3, [x0, #24]\n"
    "    ldr x2, [x0, #16]\n"
    "    ldr x1, [x0, #8]\n"
    "    ldr x0, [x0, #0]\n"
    "harness_testslot:\n"
    "    nop\n"                /* patched with the test instruction  */
    "    nop\n"                /* patched with a second instruction  */
    "    nop\n"                /* patched with a third instruction   */
    "    brk #0\n"
    "harness_end:\n"
    ".popsection\n"
);

/* ------------------------------------------------------------------ */
/* Globals shared with the signal handler.                            */
/* ------------------------------------------------------------------ */

static volatile int      g_phase;        /* 0 = launch, 1 = capture        */
static ArmState          g_block;        /* input register block           */
static ArmState         *g_out;          /* captured outputs               */
static volatile uint32_t g_trapped;      /* non-zero = faulted             */
static uint64_t          g_code;         /* address of the test code page  */
static mcontext_t        g_saved_mc;     /* harness mcontext (to resume)   */
static uint8_t           g_saved_reserved[4096];

static int is_pcrel_marker(uint64_t value) {
    return (value & PCREL_MASK) == PCREL_TOKEN;
}

static uint64_t testslot_base(uint32_t *code, size_t slot) {
    return (uint64_t)(uintptr_t)(code + slot);
}

static void apply_pcrel_input(ArmState *st, uint32_t *code, size_t slot) {
    if (st->pc != PCREL_MAGIC) return;
    uint64_t base = testslot_base(code, slot);
    for (int i = 0; i < 31; i++) {
        if (is_pcrel_marker(st->x[i])) {
            int32_t off = (int32_t)(uint32_t)st->x[i];
            st->x[i] = base + (int64_t)off;
        }
    }
    st->pc = 0;
}

static void normalize_pcrel_output(const ArmState *in, ArmState *out,
                                   uint32_t *code, size_t slot, size_t words) {
    if (in->pc != PCREL_MAGIC) return;
    uint64_t base = testslot_base(code, slot);
    uint64_t end = (uint64_t)(uintptr_t)(code + words);
    for (int i = 0; i < 31; i++) {
        if (out->x[i] >= base && out->x[i] < end) {
            out->x[i] -= base;
        }
    }
    if (out->pc >= base && out->pc < end) {
        out->pc -= base;
    }
}

static void capture_fpsimd(const mcontext_t *mc, ArmState *st) {
    const uint8_t *p = (const uint8_t *)mc->__reserved;
    for (int i = 0; i < 64; i++) {
        const struct fpsimd_ctx_hdr *h = (const struct fpsimd_ctx_hdr *)p;
        if (h->magic == FPSIMD_MAGIC) {
            st->fpsr = *(const uint32_t *)(p + 8);
            st->fpcr = *(const uint32_t *)(p + 12);
            memcpy(st->v, p + 16, sizeof st->v);
            /* keep walking: the SVE record (predicates) follows FPSIMD */
        } else if (h->magic == SVE_MAGIC) {
            /* Capture the 16 predicate registers if the record is full-size
             * (i.e. the SVE registers are live for this frame). */
            if (h->size >= SVE_RECORD_FULL) {
                memcpy(st->preds, p + SVE_PREGS_OFFSET, sizeof st->preds);
            }
        } else if (h->magic == 0 && h->size == 0) {
            return;
        }
        if (h->size == 0) return;
        p += h->size;
    }
}

static void handler(int sig, siginfo_t *si, void *uc_) {
    (void)si;
    ucontext_t *uc = (ucontext_t *)uc_;
    mcontext_t *mc = &uc->uc_mcontext;

    if (g_phase == 0) {
        /* Launch: remember how to resume the harness, then jump to the
         * prologue with x0 -> input block. The prologue loads all state. */
        memcpy(&g_saved_mc, mc, sizeof(*mc));
        memcpy(g_saved_reserved, mc->__reserved, sizeof(g_saved_reserved));
        g_saved_mc.pc += 4;          /* resume just past the launch BRK */

        mc->regs[0] = (uint64_t)(uintptr_t)&g_block;
        mc->pc = g_code;
        g_trapped = 0;
        g_phase = 1;
        return;
    }

    /* Capture phase. A non-SIGTRAP signal means the instruction faulted
     * (e.g. SIGILL from an undefined encoding); record it but still capture. */
    if (sig != SIGTRAP) g_trapped = (uint32_t)sig;

    for (int i = 0; i < 31; i++) g_out->x[i] = mc->regs[i];
    g_out->sp     = mc->sp;
    g_out->pc     = mc->pc;
    g_out->pstate = mc->pstate;
    capture_fpsimd(mc, g_out);

    /* Restore the harness context so the next case can run. */
    memcpy(mc, &g_saved_mc, sizeof(*mc));
    memcpy(mc->__reserved, g_saved_reserved, sizeof(g_saved_reserved));
    g_phase = 0;
}

static int read_exact(int fd, void *buf, size_t n) {
    uint8_t *p = (uint8_t *)buf;
    while (n) {
        ssize_t r = read(fd, p, n);
        if (r <= 0) return -1;
        p += r; n -= (size_t)r;
    }
    return 0;
}

static int write_exact(int fd, const void *buf, size_t n) {
    const uint8_t *p = (const uint8_t *)buf;
    while (n) {
        ssize_t r = write(fd, p, n);
        if (r <= 0) return -1;
        p += r; n -= (size_t)r;
    }
    return 0;
}

int main(void) {
    /* Pin the SVE vector length to 128 bits (16 bytes) so that the Z registers
     * exactly alias the captured V registers and match rax's VL=128 model. This
     * lets unpredicated SVE ops be differentially tested with the existing
     * V-register prologue/capture path. PR_SVE_SET_VL == 50. Best-effort: a
     * kernel/qemu without SVE simply leaves VL unchanged. */
    prctl(50 /* PR_SVE_SET_VL */, (unsigned long)16, 0UL, 0UL, 0UL);

    /* Deliver signals on a dedicated stack: the test instruction runs with an
     * arbitrary (possibly zero) guest SP, so the signal frame must not depend
     * on it. */
    static uint8_t altstack[256 * 1024];
    stack_t ss = { .ss_sp = altstack, .ss_size = sizeof altstack, .ss_flags = 0 };
    sigaltstack(&ss, NULL);

    struct sigaction sa;
    memset(&sa, 0, sizeof sa);
    sa.sa_sigaction = handler;
    sa.sa_flags = SA_SIGINFO | SA_NODEFER | SA_ONSTACK;
    sigaction(SIGTRAP, &sa, NULL);
    sigaction(SIGILL,  &sa, NULL);
    sigaction(SIGSEGV, &sa, NULL);
    sigaction(SIGBUS,  &sa, NULL);
    sigaction(SIGFPE,  &sa, NULL);

    /* Executable copy of the prologue + [ test-slot ; BRK ]. */
    size_t words = (size_t)(harness_end - harness_prologue);
    size_t slot  = (size_t)(harness_testslot - harness_prologue);
    uint32_t *code = (uint32_t *)mmap(NULL, 4096, PROT_READ | PROT_WRITE | PROT_EXEC,
                                      MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    if (code == MAP_FAILED) { perror("mmap"); return 2; }
    memcpy(code, harness_prologue, words * 4);
    __builtin___clear_cache((char *)code, (char *)(code + words));
    g_code = (uint64_t)(uintptr_t)code;

    /* Shared scratch memory window at a fixed address. */
    void *scratch = mmap((void *)SCRATCH_ADDR, SCRATCH_SIZE, PROT_READ | PROT_WRITE,
                         MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED, -1, 0);
    if (scratch == MAP_FAILED) { perror("mmap scratch"); return 8; }

    uint32_t magic = 0, count = 0;
    if (read_exact(0, &magic, 4) || read_exact(0, &count, 4)) return 3;
    if (magic != 0x314d5241u) return 4;
    if (write_exact(1, &magic, 4) || write_exact(1, &count, 4)) return 5;

    for (uint32_t c = 0; c < count; c++) {
        InCase in;
        if (read_exact(0, &in, sizeof in)) return 6;

        g_block = in.st;
        apply_pcrel_input(&g_block, code, slot);
        code[slot] = in.insn;
        code[slot + 1] = in.flags; /* second instruction (NOP for single tests) */
        code[slot + 2] = in.insn3; /* third instruction (NOP for one/two-slot tests) */
        __builtin___clear_cache((char *)(code + slot), (char *)(code + slot + 3));

        /* Install the scratch window contents before the test runs. */
        memcpy((void *)SCRATCH_ADDR, in.st.scratch, sizeof in.st.scratch);

        ArmState out;
        memset(&out, 0, sizeof out);
        g_out = &out;
        g_phase = 0;
        g_trapped = 0;

        __asm__ __volatile__("brk #0" ::: "memory");

        /* Capture any modifications the test made to the scratch window. */
        memcpy(out.scratch, (void *)SCRATCH_ADDR, sizeof out.scratch);
        normalize_pcrel_output(&in.st, &out, code, slot, words);

        OutCase oc;
        oc.st = out;
        oc.trapped = g_trapped;
        oc.valid = 1;
        if (write_exact(1, &oc, sizeof oc)) return 7;
    }
    return 0;
}
