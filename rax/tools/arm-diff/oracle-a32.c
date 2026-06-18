/*
 * AArch32 (A32 + T16/T32 Thumb) differential-test oracle.
 *
 * Built as a *static* 32-bit ARM ELF and executed under `qemu-arm` (user mode)
 * on an x86 host. It is the hardware-semantics reference against which the rax
 * AArch32 interpreter (src/arm/execution.rs + src/arm/decoder/{aarch32,thumb}.rs)
 * is checked. Mirrors tools/arm-diff/oracle.c (the AArch64 oracle).
 *
 * Protocol (little-endian binary, over stdin -> stdout):
 *   stdin:  u32 magic 'A','3','2','1' (0x31323341)
 *           u32 count
 *           count * struct InCase32
 *   stdout: u32 magic (echoed)
 *           u32 count
 *           count * struct OutCase32
 *
 * Each case loads the full architectural register file (R0..R14, CPSR flags
 * NZCVQ+GE, FPSCR, D0..D31) from an input block, executes ONE instruction in
 * either ARM or Thumb state, and captures the resulting register file.
 * Instructions are register-only or touch only the shared scratch window.
 *
 * Mechanism: identical two-phase signal dance as the AArch64 oracle. A fixed
 * ARM *prologue* (in an executable page) loads every register from the input
 * block, then an interworking `ldr pc` branch enters the patched test slot in
 * ARM or Thumb state (selected by bit0 of the branch literal), runs the test
 * instruction, then `BKPT`. A SIGTRAP handler captures the post-instruction
 * signal frame (GPRs, CPSR from sigcontext; D-regs/FPSCR from the VFP record)
 * and restores the harness context.
 */
#define _GNU_SOURCE
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <signal.h>
#include <ucontext.h>
#include <sys/mman.h>
#include <unistd.h>

/* ------------------------------------------------------------------ */
/* Wire format -- must match tests/arm_diff32.rs ArmState32 exactly.    */
/* ------------------------------------------------------------------ */

typedef struct {
    uint32_t r[15];      /* R0..R14                          */
    uint32_t pc;         /* set by harness; output = post-pc */
    uint32_t cpsr;       /* NZCVQ (31:27) + GE (19:16) + T   */
    uint32_t fpscr;      /* VFP status/control               */
    uint64_t d[32];      /* D0..D31 (NEON Q0..Q15)           */
    uint32_t scratch[64];/* shared scratch window (256 bytes)*/
} ArmState32;

/* Shared scratch memory for load/store tests. MAP_FIXED so the same numeric
 * address is valid in both qemu-user and the rax FlatMemory. */
#define SCRATCH_ADDR 0x200000u
#define SCRATCH_SIZE 4096
#define SCRATCH_BASE (SCRATCH_ADDR + 128)

typedef struct {
    uint32_t insn;   /* ARM: word; Thumb16: low halfword; Thumb32: hw1<<16|hw2 */
    uint32_t mode;   /* 0 = ARM, 1 = Thumb16, 2 = Thumb32                      */
    ArmState32 st;   /* input architectural state                             */
} InCase32;

typedef struct {
    ArmState32 st;   /* output architectural state                */
    uint32_t trapped;/* signal number if the insn faulted, else 0 */
    uint32_t valid;  /* 1 = executed and captured                 */
} OutCase32;

/* VFP signal-frame record (asm/ucontext.h: VFP_MAGIC). */
#define VFP_MAGIC 0x56465001u
struct vfp_sigframe_hdr { uint32_t magic; uint32_t size; };

/* ------------------------------------------------------------------ */
/* ARM register-loading prologue (assembled into .text, copied to an  */
/* executable page). On entry r0 -> input ArmState32 block.            */
/* ------------------------------------------------------------------ */
extern const uint32_t a32_prologue[];
extern const uint32_t a32_litslot[];
extern const uint32_t a32_testslot[];
extern const uint32_t a32_end[];

__asm__(
    ".pushsection .text\n"
    ".arch armv7-a\n"
    ".fpu neon-vfpv4\n"
    ".arm\n"
    ".balign 4\n"
    ".global a32_prologue\n"
    ".global a32_litslot\n"
    ".global a32_testslot\n"
    ".global a32_end\n"
    "a32_prologue:\n"
    /* FPSCR */
    "    ldr r1, [r0, #68]\n"
    "    vmsr fpscr, r1\n"
    /* CPSR flags: N,Z,C,V,Q and GE[3:0] */
    "    ldr r1, [r0, #64]\n"
    "    msr APSR_nzcvqg, r1\n"
    /* D0..D31 from offset 72 */
    "    vldr d0,  [r0, #72]\n"
    "    vldr d1,  [r0, #80]\n"
    "    vldr d2,  [r0, #88]\n"
    "    vldr d3,  [r0, #96]\n"
    "    vldr d4,  [r0, #104]\n"
    "    vldr d5,  [r0, #112]\n"
    "    vldr d6,  [r0, #120]\n"
    "    vldr d7,  [r0, #128]\n"
    "    vldr d8,  [r0, #136]\n"
    "    vldr d9,  [r0, #144]\n"
    "    vldr d10, [r0, #152]\n"
    "    vldr d11, [r0, #160]\n"
    "    vldr d12, [r0, #168]\n"
    "    vldr d13, [r0, #176]\n"
    "    vldr d14, [r0, #184]\n"
    "    vldr d15, [r0, #192]\n"
    "    vldr d16, [r0, #200]\n"
    "    vldr d17, [r0, #208]\n"
    "    vldr d18, [r0, #216]\n"
    "    vldr d19, [r0, #224]\n"
    "    vldr d20, [r0, #232]\n"
    "    vldr d21, [r0, #240]\n"
    "    vldr d22, [r0, #248]\n"
    "    vldr d23, [r0, #256]\n"
    "    vldr d24, [r0, #264]\n"
    "    vldr d25, [r0, #272]\n"
    "    vldr d26, [r0, #280]\n"
    "    vldr d27, [r0, #288]\n"
    "    vldr d28, [r0, #296]\n"
    "    vldr d29, [r0, #304]\n"
    "    vldr d30, [r0, #312]\n"
    "    vldr d31, [r0, #320]\n"
    /* GPRs r14..r1 (r0 last, it is the block base) */
    "    ldr r14, [r0, #56]\n"
    "    ldr r13, [r0, #52]\n"
    "    ldr r12, [r0, #48]\n"
    "    ldr r11, [r0, #44]\n"
    "    ldr r10, [r0, #40]\n"
    "    ldr r9,  [r0, #36]\n"
    "    ldr r8,  [r0, #32]\n"
    "    ldr r7,  [r0, #28]\n"
    "    ldr r6,  [r0, #24]\n"
    "    ldr r5,  [r0, #20]\n"
    "    ldr r4,  [r0, #16]\n"
    "    ldr r3,  [r0, #12]\n"
    "    ldr r2,  [r0, #8]\n"
    "    ldr r1,  [r0, #4]\n"
    "    ldr r0,  [r0, #0]\n"
    /* Interworking branch to (testslot | T); needs no GPR. */
    "    ldr pc, a32_litslot\n"
    "a32_litslot:\n"
    "    .word 0\n"           /* patched = testslot_addr | T          */
    "a32_testslot:\n"
    "    .word 0\n"           /* patched: test insn (ARM) / halfwords */
    "    .word 0\n"           /* patched: BKPT (ARM) / more + BKPT    */
    "    .word 0\n"
    "    .word 0\n"
    "a32_end:\n"
    ".popsection\n"
);

/* ------------------------------------------------------------------ */
/* Globals shared with the signal handler.                            */
/* ------------------------------------------------------------------ */

static volatile int      g_phase;     /* 0 = launch, 1 = capture        */
static ArmState32        g_block;     /* input register block           */
static ArmState32       *g_out;       /* captured outputs               */
static volatile uint32_t g_trapped;   /* non-zero = faulted             */
static uint32_t          g_code;      /* address of the test code page  */
static mcontext_t        g_saved_mc;  /* harness mcontext (to resume)   */
static uint8_t           g_saved_regspace[1024];

static void capture_vfp(const ucontext_t *uc, ArmState32 *st) {
    const uint8_t *p = (const uint8_t *)uc->uc_regspace;
    const uint8_t *end = (const uint8_t *)uc->uc_regspace + sizeof(uc->uc_regspace);
    while (p + sizeof(struct vfp_sigframe_hdr) <= end) {
        const struct vfp_sigframe_hdr *h = (const struct vfp_sigframe_hdr *)p;
        if (h->magic == VFP_MAGIC) {
            /* struct user_vfp { u64 fpregs[32]; u32 fpscr; } follows the 8B hdr */
            memcpy(st->d, p + 8, sizeof st->d);
            st->fpscr = *(const uint32_t *)(p + 8 + 256);
            return;
        }
        if (h->magic == 0 || h->size == 0) return;
        p += h->size;
    }
}

static void handler(int sig, siginfo_t *si, void *uc_) {
    (void)si;
    ucontext_t *uc = (ucontext_t *)uc_;
    mcontext_t *mc = &uc->uc_mcontext;

    if (g_phase == 0) {
        /* Launch: remember how to resume the harness, then jump to the
         * prologue (ARM state) with r0 -> input block. */
        memcpy(&g_saved_mc, mc, sizeof(*mc));
        memcpy(g_saved_regspace, uc->uc_regspace, sizeof g_saved_regspace);
        g_saved_mc.arm_pc += 4;       /* resume just past the launch BKPT */

        mc->arm_r0 = (uint32_t)(uintptr_t)&g_block;
        mc->arm_pc = g_code;
        /* Run the prologue in ARM state: clear T (bit5) and IT bits. */
        mc->arm_cpsr &= ~((1u << 5) | (0x3Fu << 10) | (3u << 25));
        g_trapped = 0;
        g_phase = 1;
        return;
    }

    /* Capture phase. A non-SIGTRAP signal means the instruction faulted. */
    if (sig != SIGTRAP) g_trapped = (uint32_t)sig;

    g_out->r[0]  = mc->arm_r0;
    g_out->r[1]  = mc->arm_r1;
    g_out->r[2]  = mc->arm_r2;
    g_out->r[3]  = mc->arm_r3;
    g_out->r[4]  = mc->arm_r4;
    g_out->r[5]  = mc->arm_r5;
    g_out->r[6]  = mc->arm_r6;
    g_out->r[7]  = mc->arm_r7;
    g_out->r[8]  = mc->arm_r8;
    g_out->r[9]  = mc->arm_r9;
    g_out->r[10] = mc->arm_r10;
    g_out->r[11] = mc->arm_fp;
    g_out->r[12] = mc->arm_ip;
    g_out->r[13] = mc->arm_sp;
    g_out->r[14] = mc->arm_lr;
    g_out->pc    = mc->arm_pc;
    g_out->cpsr  = mc->arm_cpsr;
    capture_vfp(uc, g_out);

    /* Restore the harness context so the next case can run. */
    memcpy(mc, &g_saved_mc, sizeof(*mc));
    memcpy(uc->uc_regspace, g_saved_regspace, sizeof g_saved_regspace);
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

/* ARM BKPT #0 / Thumb BKPT #0 used to terminate the test slot. */
#define ARM_BKPT   0xE1200070u
#define THUMB_BKPT 0xBE00u

int main(void) {
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

    size_t words = (size_t)(a32_end - a32_prologue);
    size_t lit   = (size_t)(a32_litslot - a32_prologue);
    size_t slot  = (size_t)(a32_testslot - a32_prologue);
    uint32_t *code = (uint32_t *)mmap(NULL, 4096, PROT_READ | PROT_WRITE | PROT_EXEC,
                                      MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    if (code == MAP_FAILED) { perror("mmap"); return 2; }
    memcpy(code, a32_prologue, words * 4);
    __builtin___clear_cache((char *)code, (char *)(code + words));
    g_code = (uint32_t)(uintptr_t)code;
    uint32_t slot_addr = g_code + (uint32_t)(slot * 4);

    void *scratch = mmap((void *)SCRATCH_ADDR, SCRATCH_SIZE, PROT_READ | PROT_WRITE,
                         MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED, -1, 0);
    if (scratch == MAP_FAILED) { perror("mmap scratch"); return 8; }

    uint32_t magic = 0, count = 0;
    if (read_exact(0, &magic, 4) || read_exact(0, &count, 4)) return 3;
    if (magic != 0x31323341u) return 4;
    if (write_exact(1, &magic, 4) || write_exact(1, &count, 4)) return 5;

    for (uint32_t c = 0; c < count; c++) {
        InCase32 in;
        if (read_exact(0, &in, sizeof in)) return 6;

        g_block = in.st;

        /* Patch the interworking literal and the test slot. */
        uint8_t *sb = (uint8_t *)(code + slot);
        memset(sb, 0, 16);
        if (in.mode == 0) {
            /* ARM: word + ARM BKPT */
            code[lit] = slot_addr;            /* T = 0 */
            code[slot] = in.insn;
            code[slot + 1] = ARM_BKPT;
        } else if (in.mode == 1) {
            /* Thumb 16-bit: halfword + Thumb BKPT */
            code[lit] = slot_addr | 1u;       /* T = 1 */
            sb[0] = (uint8_t)(in.insn & 0xFF);
            sb[1] = (uint8_t)((in.insn >> 8) & 0xFF);
            sb[2] = (uint8_t)(THUMB_BKPT & 0xFF);
            sb[3] = (uint8_t)((THUMB_BKPT >> 8) & 0xFF);
        } else {
            /* Thumb 32-bit: hw1, hw2 (each LE) + Thumb BKPT */
            uint32_t hw1 = (in.insn >> 16) & 0xFFFF;
            uint32_t hw2 = in.insn & 0xFFFF;
            code[lit] = slot_addr | 1u;       /* T = 1 */
            sb[0] = (uint8_t)(hw1 & 0xFF);
            sb[1] = (uint8_t)((hw1 >> 8) & 0xFF);
            sb[2] = (uint8_t)(hw2 & 0xFF);
            sb[3] = (uint8_t)((hw2 >> 8) & 0xFF);
            sb[4] = (uint8_t)(THUMB_BKPT & 0xFF);
            sb[5] = (uint8_t)((THUMB_BKPT >> 8) & 0xFF);
        }
        __builtin___clear_cache((char *)(code + lit), (char *)(code + slot + 4));

        memcpy((void *)SCRATCH_ADDR, in.st.scratch, sizeof in.st.scratch);

        ArmState32 out;
        memset(&out, 0, sizeof out);
        g_out = &out;
        g_phase = 0;
        g_trapped = 0;

        __asm__ __volatile__("bkpt #0" ::: "memory");

        memcpy(out.scratch, (void *)SCRATCH_ADDR, sizeof out.scratch);

        OutCase32 oc;
        oc.st = out;
        oc.trapped = g_trapped;
        oc.valid = 1;
        if (write_exact(1, &oc, sizeof oc)) return 7;
    }
    return 0;
}
