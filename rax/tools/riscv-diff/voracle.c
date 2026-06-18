/*
 * RISC-V vector (RVV 1.0) differential-test oracle.
 *
 * A sibling of oracle.c dedicated to the V extension. Built as a static RV64
 * ELF and run under qemu-riscv64 (default cpu, which implements RVV). The
 * generated prologue installs the integer/FP registers, loads v0..v31 from a
 * MAP_FIXED block, sets the test vtype/vl, runs ONE vector instruction, then
 * EBREAK. The SIGTRAP handler captures the integer/FP frame plus the vector
 * state (vl/vtype/vstart and the VLEN-bit register file) parsed from the
 * signal-frame V context.
 *
 * Vector instructions in the prologue are hand-encoded (the binary itself is
 * compiled rv64gc); qemu executes them. VLEN is 128 bits (16 bytes/register).
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

#define WIRE_MAGIC 0x56524332u /* 'V','R','C','2' */
#define RISCV_V_MAGIC 0x53465457u
#define VLENB 16 /* VLEN / 8 */

#define SCRATCH_ADDR 0x200000ull
#define SCRATCH_SIZE 4096
#define INPUT_ADDR 0x210000ull /* x/f/fcsr/vtype/vl block */
#define VIN_ADDR 0x220000ull   /* 512-byte v0..v31 input data */
#define BLOCK_SIZE 4096

#define IN_X_OFF 0
#define IN_F_OFF (32 * 8)
#define IN_FCSR_OFF (64 * 8)
#define IN_VTYPE_OFF (65 * 8)
#define IN_VL_OFF (66 * 8)
#define IN_VCSR_OFF (67 * 8)

typedef struct {
    uint64_t x[32];
    uint64_t f[32];
    uint64_t vtype;
    uint64_t vl;
    uint64_t vstart;
    uint64_t fcsr;
    uint64_t vcsr; /* {vxrm[2:1], vxsat[0]} */
    uint64_t v[64];       /* 32 regs * 16 bytes = 512 bytes (2 u64 each) */
    uint64_t scratch[32]; /* shared 256-byte window */
} VState;

typedef struct {
    uint32_t insn;
    uint32_t insn_len;
    VState st;
} VInCase;

typedef struct {
    VState st;
    uint32_t trapped;
    uint32_t valid;
} VOutCase;

struct ctxhdr {
    uint32_t magic;
    uint32_t size;
};
struct vstate_hdr {
    unsigned long vstart, vl, vtype, vcsr, vlenb;
    void *datap;
};

static sigjmp_buf g_harness;
static volatile VState g_out;
static volatile uint32_t g_trapped;

static void handler(int sig, siginfo_t *si, void *ucv) {
    (void)si;
    ucontext_t *uc = (ucontext_t *)ucv;
    for (int i = 1; i < 32; i++) {
        g_out.x[i] = uc->uc_mcontext.__gregs[i];
    }
    for (int i = 0; i < 32; i++) {
        g_out.f[i] = uc->uc_mcontext.__fpregs.__d.__f[i];
    }
    g_out.fcsr = uc->uc_mcontext.__fpregs.__d.__fcsr;
    /* Locate the V context in the signal frame and copy the vector state. */
    unsigned char *base = (unsigned char *)&uc->uc_mcontext;
    for (int off = 256; off < 1200; off += 4) {
        struct ctxhdr *h = (struct ctxhdr *)(base + off);
        if (h->magic == RISCV_V_MAGIC) {
            struct vstate_hdr *v = (struct vstate_hdr *)(h + 1);
            g_out.vl = v->vl;
            g_out.vtype = v->vtype;
            g_out.vstart = v->vstart;
            g_out.vcsr = v->vcsr;
            unsigned char *regs = v->datap ? (unsigned char *)v->datap
                                           : (unsigned char *)(v + 1);
            memcpy((void *)g_out.v, regs, 32 * VLENB);
            break;
        }
    }
    if (sig != SIGTRAP) {
        g_trapped = (uint32_t)sig;
    }
    siglongjmp(g_harness, 1);
}

/* Encoders. */
static uint32_t enc_lui(int rd, uint32_t imm20) {
    return (imm20 << 12) | ((uint32_t)rd << 7) | 0x37u;
}
static uint32_t enc_ld(int rd, int rs1, int off) {
    return (((uint32_t)(off & 0xfff)) << 20) | ((uint32_t)rs1 << 15) | (3u << 12) |
           ((uint32_t)rd << 7) | 0x03u;
}
static uint32_t enc_fld(int rd, int rs1, int off) {
    return (((uint32_t)(off & 0xfff)) << 20) | ((uint32_t)rs1 << 15) | (3u << 12) |
           ((uint32_t)rd << 7) | 0x07u;
}
static uint32_t enc_addi(int rd, int rs1, int imm) {
    return (((uint32_t)(imm & 0xfff)) << 20) | ((uint32_t)rs1 << 15) | (0u << 12) |
           ((uint32_t)rd << 7) | 0x13u;
}
static uint32_t enc_fscsr(int rs1) {
    return (0x003u << 20) | ((uint32_t)rs1 << 15) | (1u << 12) | (0u << 7) | 0x73u;
}
/* csrrw x0, csr, rs1 */
static uint32_t enc_csrw(uint32_t csr, int rs1) {
    return (csr << 20) | ((uint32_t)rs1 << 15) | (1u << 12) | (0u << 7) | 0x73u;
}
/* vsetvli rd, rs1, vtypei */
static uint32_t enc_vsetvli(int rd, int rs1, uint32_t vtypei) {
    return (vtypei << 20) | ((uint32_t)rs1 << 15) | (7u << 12) | ((uint32_t)rd << 7) | 0x57u;
}
/* vsetvl rd, rs1, rs2 */
static uint32_t enc_vsetvl(int rd, int rs1, int rs2) {
    return (1u << 31) | ((uint32_t)rs2 << 20) | ((uint32_t)rs1 << 15) | (7u << 12) |
           ((uint32_t)rd << 7) | 0x57u;
}
/* vle8.v vd, (rs1)  (unit-stride, unmasked, width=8) */
static uint32_t enc_vle8(int vd, int rs1) {
    return (1u << 25) | ((uint32_t)rs1 << 15) | (0u << 12) | ((uint32_t)vd << 7) | 0x07u;
}
#define EBREAK 0x00100073u

static ssize_t read_full(int fd, void *buf, size_t n) {
    size_t got = 0;
    char *p = buf;
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
    const char *p = buf;
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

static char altstack[262144];

int main(void) {
    void *s1 = mmap((void *)SCRATCH_ADDR, SCRATCH_SIZE, PROT_READ | PROT_WRITE,
                    MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED, -1, 0);
    void *s2 = mmap((void *)INPUT_ADDR, BLOCK_SIZE, PROT_READ | PROT_WRITE,
                    MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED, -1, 0);
    void *s3 = mmap((void *)VIN_ADDR, BLOCK_SIZE, PROT_READ | PROT_WRITE,
                    MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED, -1, 0);
    if (s1 == MAP_FAILED || s2 == MAP_FAILED || s3 == MAP_FAILED) return 2;

    uint32_t *code = mmap(NULL, 8192, PROT_READ | PROT_WRITE | PROT_EXEC,
                          MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    if (code == MAP_FAILED) return 2;

    int n = 0;
    /* x31 <- INPUT_ADDR */
    code[n++] = enc_lui(31, (uint32_t)(INPUT_ADDR >> 12));
    /* fp regs + fcsr */
    for (int i = 0; i < 32; i++) code[n++] = enc_fld(i, 31, IN_F_OFF + i * 8);
    code[n++] = enc_ld(30, 31, IN_FCSR_OFF);
    code[n++] = enc_fscsr(30);
    /* vcsr (vxrm/vxsat) */
    code[n++] = enc_ld(29, 31, IN_VCSR_OFF);
    code[n++] = enc_csrw(0x00Fu, 29);
    /* vector setup: x5 <- VIN_ADDR; vsetvli x6,x0,e8m1 (vl=16); load v0..v31 */
    code[n++] = enc_lui(5, (uint32_t)(VIN_ADDR >> 12));
    code[n++] = enc_vsetvli(6, 0, 0); /* e8,m1 -> vl = VLEN/8 = 16 */
    for (int i = 0; i < 32; i++) {
        code[n++] = enc_vle8(i, 5);
        code[n++] = enc_addi(5, 5, VLENB);
    }
    /* test vtype/vl: x6 <- vtype, x7 <- avl(vl); vsetvl x0, x7, x6 */
    code[n++] = enc_ld(6, 31, IN_VTYPE_OFF);
    code[n++] = enc_ld(7, 31, IN_VL_OFF);
    code[n++] = enc_vsetvl(0, 7, 6);
    /* int regs x1,x2,x5..x30 (skip gp/tp), x31 last */
    for (int i = 1; i <= 30; i++) {
        if (i == 3 || i == 4) continue;
        code[n++] = enc_ld(i, 31, IN_X_OFF + i * 8);
    }
    code[n++] = enc_ld(31, 31, IN_X_OFF + 31 * 8);

    int test_slot = n;
    code[n++] = EBREAK;
    code[n++] = EBREAK;
    __builtin___clear_cache((char *)code, (char *)code + n * 4);

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
    VInCase *cases = calloc(count ? count : 1, sizeof(VInCase));
    VOutCase *outs = calloc(count ? count : 1, sizeof(VOutCase));
    if (!cases || !outs) return 2;
    if (read_full(0, cases, (size_t)count * sizeof(VInCase)) !=
        (ssize_t)((size_t)count * sizeof(VInCase)))
        return 3;

    void (*entry)(void) = (void (*)(void))code;

    for (uint32_t c = 0; c < count; c++) {
        VInCase *ic = &cases[c];
        memcpy((void *)INPUT_ADDR, ic->st.x, 32 * 8);
        memcpy((char *)INPUT_ADDR + IN_F_OFF, ic->st.f, 32 * 8);
        memcpy((char *)INPUT_ADDR + IN_FCSR_OFF, &ic->st.fcsr, 8);
        memcpy((char *)INPUT_ADDR + IN_VTYPE_OFF, &ic->st.vtype, 8);
        memcpy((char *)INPUT_ADDR + IN_VL_OFF, &ic->st.vl, 8);
        memcpy((char *)INPUT_ADDR + IN_VCSR_OFF, &ic->st.vcsr, 8);
        memcpy((void *)VIN_ADDR, ic->st.v, 32 * VLENB);
        memcpy((void *)SCRATCH_ADDR, ic->st.scratch, sizeof(ic->st.scratch));

        if (ic->insn_len == 2) {
            code[test_slot] = (ic->insn & 0xffff) | (0x9002u << 16);
        } else {
            code[test_slot] = ic->insn;
        }
        __builtin___clear_cache((char *)&code[test_slot], (char *)&code[test_slot + 1]);

        g_trapped = 0;
        memset((void *)&g_out, 0, sizeof(g_out));
        if (sigsetjmp(g_harness, 1) == 0) {
            entry();
        }
        VOutCase *oc = &outs[c];
        memset(oc, 0, sizeof(*oc));
        for (int i = 1; i < 32; i++) oc->st.x[i] = g_out.x[i];
        for (int i = 0; i < 32; i++) oc->st.f[i] = g_out.f[i];
        oc->st.fcsr = g_out.fcsr;
        oc->st.vcsr = g_out.vcsr;
        oc->st.vl = g_out.vl;
        oc->st.vtype = g_out.vtype;
        oc->st.vstart = g_out.vstart;
        memcpy(oc->st.v, (const void *)g_out.v, 32 * VLENB);
        memcpy(oc->st.scratch, (void *)SCRATCH_ADDR, sizeof(oc->st.scratch));
        oc->trapped = g_trapped;
        oc->valid = 1;
    }

    if (write_full(1, &magic, 4) != 4) return 4;
    if (write_full(1, &count, 4) != 4) return 4;
    if (write_full(1, outs, (size_t)count * sizeof(VOutCase)) !=
        (ssize_t)((size_t)count * sizeof(VOutCase)))
        return 4;
    return 0;
}
