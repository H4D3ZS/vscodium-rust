/*
 * RISC-V (RV64GC) differential-test oracle.
 *
 * Built as a *static* RV64 ELF and executed under `qemu-riscv64` (user mode) on
 * an x86 host. It is the hardware-semantics reference against which the rax
 * software interpreter (src/riscv/cpu.rs) is checked.
 *
 * Protocol (little-endian binary, over stdin -> stdout):
 *   stdin:  u32 magic 'R','V','6','4' (0x34365652)
 *           u32 count
 *           count * struct InCase
 *   stdout: u32 magic (echoed)
 *           u32 count
 *           count * struct OutCase
 *
 * Each case installs a full architectural register file (x1..x31, f0..f31,
 * fcsr) plus a shared 256-byte scratch window, executes ONE instruction word,
 * and captures the resulting register file. Instructions must be register-only
 * or touch only the scratch window (no PC-relative control flow).
 *
 * Mechanism: a generated *prologue* (in an RWX page) loads every register from
 * a MAP_FIXED input page, runs the patched test instruction, then `EBREAK`.
 * qemu-user populates both the integer (`__gregs`) and floating-point
 * (`__fpregs`) parts of the signal frame on delivery, so the post-instruction
 * state is read straight out of the SIGTRAP frame; the handler then
 * `siglongjmp`s back to the harness (the test SP is garbage, so the signal is
 * taken on an alternate stack). Faulting encodings raise SIGILL/SIGSEGV/SIGBUS/
 * SIGFPE, captured as `trapped`.
 */
#define _GNU_SOURCE
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>
#include <signal.h>
#include <setjmp.h>
#include <ucontext.h>
#include <sys/mman.h>
#include <unistd.h>

/* ------------------------------------------------------------------ */
/* Wire format -- must match tests/riscv_diff.rs RvState exactly.      */
/* All-u64 layout (8-byte aligned, no padding).                        */
/* ------------------------------------------------------------------ */

typedef struct {
    uint64_t x[32];       /* x0..x31 (x0 ignored on input/output)       */
    uint64_t f[32];       /* f0..f31 raw bits                           */
    uint64_t pc;          /* output: post-instruction PC (prologue rel) */
    uint64_t fcsr;        /* fcsr (frm<<5 | fflags)                     */
    uint64_t scratch[32]; /* shared 256-byte window contents            */
} RvState;

typedef struct {
    uint32_t insn;     /* instruction word under test (low 16b if compressed) */
    uint32_t insn_len; /* 2 or 4                                               */
    RvState  st;       /* input architectural state                           */
} InCase;

typedef struct {
    RvState  st;      /* output architectural state                 */
    uint32_t trapped; /* signal number if the insn faulted, else 0  */
    uint32_t valid;   /* 1 = executed and captured                  */
} OutCase;

#define WIRE_MAGIC 0x34365652u

/* Fixed guest addresses (all lui-materializable: low 12 bits zero). */
#define SCRATCH_ADDR 0x200000ull
#define SCRATCH_SIZE 4096
#define SCRATCH_BASE (SCRATCH_ADDR + 64)
#define INPUT_ADDR   0x210000ull   /* input GPR/FP block */
#define BLOCK_SIZE   4096

/* Offsets within the input block. */
#define IN_X_OFF    0           /* x[0..32] : 256 bytes */
#define IN_F_OFF    (32 * 8)    /* f[0..32] : 256 bytes */
#define IN_FCSR_OFF (64 * 8)    /* fcsr : 8 bytes        */

/* ------------------------------------------------------------------ */

static sigjmp_buf g_harness;
static volatile RvState g_out;
static volatile uint32_t g_trapped;

static void handler(int sig, siginfo_t *si, void *ucv) {
    (void)si;
    ucontext_t *uc = (ucontext_t *)ucv;
    /* __gregs[0] = pc, __gregs[i] = x[i] for i in 1..31. */
    g_out.pc = uc->uc_mcontext.__gregs[0];
    for (int i = 1; i < 32; i++) {
        g_out.x[i] = uc->uc_mcontext.__gregs[i];
    }
    for (int i = 0; i < 32; i++) {
        g_out.f[i] = uc->uc_mcontext.__fpregs.__d.__f[i];
    }
    g_out.fcsr = uc->uc_mcontext.__fpregs.__d.__fcsr;
    if (sig != SIGTRAP) {
        g_trapped = (uint32_t)sig;
    }
    siglongjmp(g_harness, 1);
}

/* Instruction encoders for the generated prologue. The integer load width
 * follows the target XLEN (ld on RV64, lw on RV32). */
static uint32_t enc_lui(int rd, uint32_t imm20) {
    return (imm20 << 12) | ((uint32_t)rd << 7) | 0x37u;
}
#if __riscv_xlen == 32
#define XLOAD_FUNCT3 2u /* lw */
#else
#define XLOAD_FUNCT3 3u /* ld */
#endif
static uint32_t enc_ld(int rd, int rs1, int off) {
    return (((uint32_t)(off & 0xfff)) << 20) | ((uint32_t)rs1 << 15) | (XLOAD_FUNCT3 << 12) |
           ((uint32_t)rd << 7) | 0x03u;
}
static uint32_t enc_fld(int rd, int rs1, int off) {
    return (((uint32_t)(off & 0xfff)) << 20) | ((uint32_t)rs1 << 15) | (3u << 12) |
           ((uint32_t)rd << 7) | 0x07u;
}
/* csrrw x0, fcsr(0x003), rs1  (fscsr rs1) */
static uint32_t enc_fscsr(int rs1) {
    return (0x003u << 20) | ((uint32_t)rs1 << 15) | (1u << 12) | (0u << 7) | 0x73u;
}
#define EBREAK   0x00100073u
#define C_EBREAK 0x9002u

static ssize_t read_full(int fd, void *buf, size_t n) {
    size_t got = 0;
    char *p = (char *)buf;
    while (got < n) {
        ssize_t r = read(fd, p + got, n - got);
        if (r < 0) {
            if (errno == EINTR) continue;
            return -1;
        }
        if (r == 0) break;
        got += (size_t)r;
    }
    return (ssize_t)got;
}

static ssize_t write_full(int fd, const void *buf, size_t n) {
    size_t put = 0;
    const char *p = (const char *)buf;
    while (put < n) {
        ssize_t r = write(fd, p + put, n - put);
        if (r < 0) {
            if (errno == EINTR) continue;
            return -1;
        }
        put += (size_t)r;
    }
    return (ssize_t)put;
}

static char altstack[131072];

int main(void) {
    /* Map fixed input/scratch windows. */
    void *scratch = mmap((void *)SCRATCH_ADDR, SCRATCH_SIZE, PROT_READ | PROT_WRITE,
                         MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED, -1, 0);
    void *input = mmap((void *)INPUT_ADDR, BLOCK_SIZE, PROT_READ | PROT_WRITE,
                       MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED, -1, 0);
    if (scratch == MAP_FAILED || input == MAP_FAILED) {
        return 2;
    }

    /* Build an RWX code page: prologue (load all regs) + test slot + EBREAK. */
    uint32_t *code = mmap(NULL, 4096, PROT_READ | PROT_WRITE | PROT_EXEC,
                          MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    if (code == MAP_FAILED) return 2;

    int n = 0;
    code[n++] = enc_lui(31, (uint32_t)(INPUT_ADDR >> 12)); /* x31 <- INPUT_ADDR */
    for (int i = 0; i < 32; i++) {
        code[n++] = enc_fld(i, 31, IN_F_OFF + i * 8); /* f0..f31 */
    }
    code[n++] = enc_ld(30, 31, IN_FCSR_OFF); /* x30 <- fcsr value */
    code[n++] = enc_fscsr(30);               /* fcsr <- x30 */
    /* Load x1,x2,x5..x30. x3 (gp) and x4 (tp) are reserved: clobbering them
     * would break the signal handler's TLS / stack-canary access, so they keep
     * their C-runtime values and the harness never uses or compares them. */
    for (int i = 1; i <= 30; i++) {
        if (i == 3 || i == 4) continue;
        code[n++] = enc_ld(i, 31, IN_X_OFF + i * 8); /* x1,x2,x5..x30 */
    }
    code[n++] = enc_ld(31, 31, IN_X_OFF + 31 * 8); /* x31 last */

    int test_slot = n;
    code[n++] = EBREAK; /* test instruction (patched per case)        */
    code[n++] = EBREAK; /* +4: fall-through trap (not-taken branch)   */
    code[n++] = EBREAK; /* +8: target trap for taken branches (imm=8) */
    __builtin___clear_cache((char *)code, (char *)code + n * 4);

    /* Install signal handlers on an alternate stack. */
    stack_t ss = {.ss_sp = altstack, .ss_size = sizeof(altstack), .ss_flags = 0};
    sigaltstack(&ss, NULL);
    struct sigaction sa;
    memset(&sa, 0, sizeof(sa));
    sa.sa_sigaction = handler;
    sa.sa_flags = SA_SIGINFO | SA_ONSTACK;
    sigfillset(&sa.sa_mask);
    sigaction(SIGTRAP, &sa, NULL);
    sigaction(SIGILL, &sa, NULL);
    sigaction(SIGSEGV, &sa, NULL);
    sigaction(SIGBUS, &sa, NULL);
    sigaction(SIGFPE, &sa, NULL);

    uint32_t magic = 0, count = 0;
    if (read_full(0, &magic, 4) != 4 || magic != WIRE_MAGIC) return 3;
    if (read_full(0, &count, 4) != 4) return 3;

    InCase *cases = calloc(count ? count : 1, sizeof(InCase));
    OutCase *outs = calloc(count ? count : 1, sizeof(OutCase));
    if (!cases || !outs) return 2;
    if (read_full(0, cases, (size_t)count * sizeof(InCase)) !=
        (ssize_t)((size_t)count * sizeof(InCase))) {
        return 3;
    }

    void (*entry)(void) = (void (*)(void))code;

    for (uint32_t c = 0; c < count; c++) {
        InCase *ic = &cases[c];

        memcpy((void *)INPUT_ADDR, ic->st.x, 32 * 8);
        memcpy((char *)INPUT_ADDR + IN_F_OFF, ic->st.f, 32 * 8);
        memcpy((char *)INPUT_ADDR + IN_FCSR_OFF, &ic->st.fcsr, 8);
        memcpy((void *)SCRATCH_ADDR, ic->st.scratch, sizeof(ic->st.scratch));

        /* Patch the test slot: 2-byte insn -> insn||c.ebreak; 4-byte -> insn. */
        if (ic->insn_len == 2) {
            code[test_slot] = (ic->insn & 0xffff) | ((uint32_t)C_EBREAK << 16);
        } else {
            code[test_slot] = ic->insn;
        }
        __builtin___clear_cache((char *)&code[test_slot], (char *)&code[test_slot + 1]);

        g_trapped = 0;
        memset((void *)&g_out, 0, sizeof(g_out));

        if (sigsetjmp(g_harness, 1) == 0) {
            entry();
        }

        OutCase *oc = &outs[c];
        memset(oc, 0, sizeof(*oc));
        for (int i = 1; i < 32; i++) oc->st.x[i] = g_out.x[i];
        for (int i = 0; i < 32; i++) oc->st.f[i] = g_out.f[i];
        /* Report PC as a displacement from the test slot so PC-relative control
         * flow can be checked independent of the (runtime) code address. */
        oc->st.pc = g_out.pc - (uint64_t)(uintptr_t)&code[test_slot];
        oc->st.fcsr = g_out.fcsr;
        memcpy(oc->st.scratch, (void *)SCRATCH_ADDR, sizeof(oc->st.scratch));
        oc->trapped = g_trapped;
        oc->valid = 1;
    }

    if (write_full(1, &magic, 4) != 4) return 4;
    if (write_full(1, &count, 4) != 4) return 4;
    if (write_full(1, outs, (size_t)count * sizeof(OutCase)) !=
        (ssize_t)((size_t)count * sizeof(OutCase))) {
        return 4;
    }
    (void)SCRATCH_BASE;
    return 0;
}
