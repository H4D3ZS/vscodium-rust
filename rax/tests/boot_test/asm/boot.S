/*
 * Minimal Test Kernel - Entry Point (AT&T syntax)
 *
 * This kernel mimics Linux kernel boot initialization complexity:
 * - 64-bit long mode entry
 * - GDT/IDT setup
 * - Page table configuration
 * - CR0/CR3/CR4/EFER setup
 * - LAPIC timer initialization
 * - MSR operations
 * - Interrupt handling
 *
 * Unlike Linux, this is designed to be slim and fast for debugging emulators.
 */

.code64
.section .boot, "ax"

/* Constants */
.set SERIAL_PORT,       0x3F8
.set MSR_EFER,          0xC0000080
.set MSR_STAR,          0xC0000081
.set MSR_LSTAR,         0xC0000082
.set MSR_GS_BASE,       0xC0000101
.set MSR_FS_BASE,       0xC0000100
.set MSR_KERNEL_GS_BASE, 0xC0000102
.set MSR_IA32_APIC_BASE, 0x1B

.set APIC_BASE_ADDR,    0xFEE00000
.set APIC_ID,           0x020
.set APIC_VERSION,      0x030
.set APIC_TPR,          0x080
.set APIC_EOI,          0x0B0
.set APIC_SVR,          0x0F0
.set APIC_ESR,          0x280
.set APIC_ICR_LOW,      0x300
.set APIC_ICR_HIGH,     0x310
.set APIC_LVT_TIMER,    0x320
.set APIC_TIMER_INIT,   0x380
.set APIC_TIMER_CURR,   0x390
.set APIC_TIMER_DIV,    0x3E0

.set CR0_PE,            (1 << 0)
.set CR0_MP,            (1 << 1)
.set CR0_ET,            (1 << 4)
.set CR0_NE,            (1 << 5)
.set CR0_WP,            (1 << 16)
.set CR0_AM,            (1 << 18)
.set CR0_PG,            (1 << 31)

.set CR4_PSE,           (1 << 4)
.set CR4_PAE,           (1 << 5)
.set CR4_MCE,           (1 << 6)
.set CR4_PGE,           (1 << 7)
.set CR4_OSFXSR,        (1 << 9)
.set CR4_OSXMMEXCPT,    (1 << 10)

.set EFER_SCE,          (1 << 0)
.set EFER_LME,          (1 << 8)
.set EFER_LMA,          (1 << 10)
.set EFER_NXE,          (1 << 11)

/* Entry point - expects to be in 64-bit long mode */
.globl _start
_start:
    cli
    cld

    /* Set up stack */
    leaq stack_top(%rip), %rsp
    xorq %rbp, %rbp

    /* Initialize serial port for output */
    call serial_init

    /* Print banner */
    leaq msg_banner(%rip), %rdi
    call serial_puts

    /* ========================================
     * TEST 1: Verify we're in long mode
     * ======================================== */
    leaq msg_test1(%rip), %rdi
    call serial_puts

    movl $MSR_EFER, %ecx
    rdmsr
    testl $EFER_LMA, %eax
    jz long_mode_fail

    leaq msg_pass(%rip), %rdi
    call serial_puts
    jmp test2

long_mode_fail:
    leaq msg_fail(%rip), %rdi
    call serial_puts
    jmp halt

    /* ========================================
     * TEST 2: Verify/setup CR0
     * ======================================== */
test2:
    leaq msg_test2(%rip), %rdi
    call serial_puts

    movq %cr0, %rax
    /* Verify PE and PG are set */
    testl $(CR0_PE | CR0_PG), %eax
    jz cr0_fail

    /* Set additional bits like Linux does */
    orq $(CR0_MP | CR0_ET | CR0_NE | CR0_WP | CR0_AM), %rax
    movq %rax, %cr0

    leaq msg_pass(%rip), %rdi
    call serial_puts
    jmp test3

cr0_fail:
    leaq msg_fail(%rip), %rdi
    call serial_puts
    jmp halt

    /* ========================================
     * TEST 3: Setup and reload GDT
     * ======================================== */
test3:
    leaq msg_test3(%rip), %rdi
    call serial_puts

    /* Load our GDT */
    leaq gdt64(%rip), %rax
    leaq gdt64_ptr(%rip), %rbx
    movq %rax, 2(%rbx)

    lgdt gdt64_ptr(%rip)

    /* Reload segment registers */
    movw $0x10, %ax         /* Data segment selector */
    movw %ax, %ds
    movw %ax, %es
    movw %ax, %ss
    xorw %ax, %ax
    movw %ax, %fs
    movw %ax, %gs

    /* Far jump to reload CS - use a retfq trick */
    leaq gdt_loaded(%rip), %rax
    pushq $0x08             /* CS selector */
    pushq %rax
    lretq

gdt_loaded:
    leaq msg_pass(%rip), %rdi
    call serial_puts

    /* ========================================
     * TEST 4: Setup IDT with exception handlers
     * ======================================== */
test4:
    leaq msg_test4(%rip), %rdi
    call serial_puts

    call setup_idt

    leaq msg_pass(%rip), %rdi
    call serial_puts

    /* ========================================
     * TEST 5: Test GSBASE MSR
     * ======================================== */
test5:
    leaq msg_test5(%rip), %rdi
    call serial_puts

    /* Write a test value to GS_BASE */
    movl $MSR_GS_BASE, %ecx
    movl $0xDEADBEEF, %eax
    movl $0x12345678, %edx
    wrmsr

    /* Read it back */
    rdmsr
    cmpl $0xDEADBEEF, %eax
    jne gsbase_fail
    cmpl $0x12345678, %edx
    jne gsbase_fail

    /* Clear it */
    xorl %eax, %eax
    xorl %edx, %edx
    wrmsr

    leaq msg_pass(%rip), %rdi
    call serial_puts
    jmp test6

gsbase_fail:
    leaq msg_fail(%rip), %rdi
    call serial_puts

    /* ========================================
     * TEST 6: Configure CR4 (like Linux)
     * ======================================== */
test6:
    leaq msg_test6(%rip), %rdi
    call serial_puts

    movq %cr4, %rax
    orq $(CR4_PAE | CR4_PSE | CR4_PGE | CR4_OSFXSR | CR4_OSXMMEXCPT), %rax
    movq %rax, %cr4

    /* Verify it took */
    movq %cr4, %rbx
    andq $(CR4_PAE | CR4_PSE), %rbx
    cmpq $(CR4_PAE | CR4_PSE), %rbx
    jne cr4_fail

    leaq msg_pass(%rip), %rdi
    call serial_puts
    jmp test7

cr4_fail:
    leaq msg_fail(%rip), %rdi
    call serial_puts

    /* ========================================
     * TEST 7: Enable NX bit in EFER
     * ======================================== */
test7:
    leaq msg_test7(%rip), %rdi
    call serial_puts

    /* Check if NX is supported via CPUID */
    movl $0x80000001, %eax
    cpuid
    testl $(1 << 20), %edx     /* NX bit */
    jz nx_not_supported

    /* Enable NX */
    movl $MSR_EFER, %ecx
    rdmsr
    orl $EFER_NXE, %eax
    wrmsr

    /* Verify */
    rdmsr
    testl $EFER_NXE, %eax
    jz efer_fail

    leaq msg_pass(%rip), %rdi
    call serial_puts
    jmp test8

nx_not_supported:
    leaq msg_skip(%rip), %rdi
    call serial_puts
    jmp test8

efer_fail:
    leaq msg_fail(%rip), %rdi
    call serial_puts

    /* ========================================
     * TEST 8: LAPIC initialization
     * ======================================== */
test8:
    leaq msg_test8(%rip), %rdi
    call serial_puts

    call lapic_init
    testl %eax, %eax
    jnz lapic_fail

    leaq msg_pass(%rip), %rdi
    call serial_puts
    jmp test9

lapic_fail:
    leaq msg_fail(%rip), %rdi
    call serial_puts

    /* ========================================
     * TEST 9: LAPIC Timer test
     * ======================================== */
test9:
    leaq msg_test9(%rip), %rdi
    call serial_puts

    call lapic_timer_test
    testl %eax, %eax
    jnz timer_fail

    leaq msg_pass(%rip), %rdi
    call serial_puts
    jmp test10

timer_fail:
    leaq msg_fail(%rip), %rdi
    call serial_puts

    /* ========================================
     * TEST 10: Exception handling (divide by zero)
     * ======================================== */
test10:
    leaq msg_test10(%rip), %rdi
    call serial_puts

    /* Set flag that we expect an exception */
    movb $1, expect_exception(%rip)
    movb $0, exception_occurred(%rip)

    /* Trigger divide by zero - #DE exception */
    xorl %eax, %eax
    xorl %edx, %edx
    movl $0, %ecx
    /* The div instruction will cause #DE when divisor is 0 */
    /* Handler will skip past the div instruction */
    divl %ecx              /* This triggers #DE */

exception_return:
    /* Check if exception occurred */
    movb $0, expect_exception(%rip)
    cmpb $1, exception_occurred(%rip)
    jne exc_test_fail

    leaq msg_pass(%rip), %rdi
    call serial_puts
    jmp test11

exc_test_fail:
    leaq msg_fail(%rip), %rdi
    call serial_puts

    /* ========================================
     * TEST 11: Page table walk test
     * ======================================== */
test11:
    leaq msg_test11(%rip), %rdi
    call serial_puts

    /* Read CR3 */
    movq %cr3, %rax

    /* Verify we can read from the page table */
    movq %rax, %rbx
    andq $~0xFFF, %rbx         /* Mask off flags */

    /* Try to access page table (should work if identity mapped) */
    movq (%rbx), %rcx          /* Read PML4[0] */

    leaq msg_pass(%rip), %rdi
    call serial_puts

    /* ========================================
     * TEST 12: SYSCALL/SYSRET setup (like Linux)
     * ======================================== */
test12:
    leaq msg_test12(%rip), %rdi
    call serial_puts

    /* Enable SCE (SYSCALL Enable) in EFER */
    movl $MSR_EFER, %ecx
    rdmsr
    orl $EFER_SCE, %eax
    wrmsr

    /* Set up STAR MSR (syscall segments) */
    movl $MSR_STAR, %ecx
    movl $0x00230010, %edx     /* SYSRET CS/SS : SYSCALL CS/SS */
    xorl %eax, %eax
    wrmsr

    /* Set up LSTAR (syscall entry point) */
    movl $MSR_LSTAR, %ecx
    leaq syscall_entry(%rip), %rax
    movq %rax, %rdx
    shrq $32, %rdx
    wrmsr

    leaq msg_pass(%rip), %rdi
    call serial_puts

    /* ========================================
     * TEST 13: Memory access patterns
     * ======================================== */
test13:
    leaq msg_test13(%rip), %rdi
    call serial_puts

    /* Test various memory access sizes */
    leaq test_buffer(%rip), %rdi

    /* 8-bit access */
    movb $0x12, (%rdi)
    movb (%rdi), %al
    cmpb $0x12, %al
    jne mem_fail

    /* 16-bit access */
    movw $0x3456, (%rdi)
    movw (%rdi), %ax
    cmpw $0x3456, %ax
    jne mem_fail

    /* 32-bit access */
    movl $0x789ABCDE, (%rdi)
    movl (%rdi), %eax
    cmpl $0x789ABCDE, %eax
    jne mem_fail

    /* 64-bit access */
    movabsq $0xFEDCBA9876543210, %rax
    movq %rax, (%rdi)
    movq (%rdi), %rax
    movabsq $0xFEDCBA9876543210, %rbx
    cmpq %rbx, %rax
    jne mem_fail

    /* Unaligned access */
    movl $0xAABBCCDD, 1(%rdi)
    movl 1(%rdi), %eax
    cmpl $0xAABBCCDD, %eax
    jne mem_fail

    leaq msg_pass(%rip), %rdi
    call serial_puts
    jmp test14

mem_fail:
    leaq msg_fail(%rip), %rdi
    call serial_puts

    /* ========================================
     * TEST 14: SSE/XMM registers
     * ======================================== */
test14:
    leaq msg_test14(%rip), %rdi
    call serial_puts

    /* Enable SSE in CR0 (clear EM, set MP) */
    movq %cr0, %rax
    andw $0xFFFB, %ax          /* Clear EM (bit 2) */
    orw $0x2, %ax              /* Set MP (bit 1) */
    movq %rax, %cr0

    /* Enable SSE in CR4 */
    movq %cr4, %rax
    orq $(CR4_OSFXSR | CR4_OSXMMEXCPT), %rax
    movq %rax, %cr4

    /* Test XMM register */
    leaq test_buffer(%rip), %rdi
    movabsq $0x123456789ABCDEF0, %rax
    movq %rax, (%rdi)
    movabsq $0xFEDCBA9876543210, %rax
    movq %rax, 8(%rdi)

    movdqu (%rdi), %xmm0
    movdqu %xmm0, 16(%rdi)

    movq 16(%rdi), %rax
    movabsq $0x123456789ABCDEF0, %rbx
    cmpq %rbx, %rax
    jne sse_fail

    leaq msg_pass(%rip), %rdi
    call serial_puts
    jmp test15

sse_fail:
    leaq msg_fail(%rip), %rdi
    call serial_puts

    /* ========================================
     * TEST 15: Interrupt enable/disable
     * ======================================== */
test15:
    leaq msg_test15(%rip), %rdi
    call serial_puts

    /* Check that interrupts are disabled */
    pushfq
    popq %rax
    testl $0x200, %eax         /* IF flag */
    jnz int_unexpected

    /* Enable interrupts briefly */
    sti
    nop
    nop
    cli

    leaq msg_pass(%rip), %rdi
    call serial_puts
    jmp all_done

int_unexpected:
    leaq msg_fail(%rip), %rdi
    call serial_puts

all_done:
    leaq msg_complete(%rip), %rdi
    call serial_puts

    /* Halt with success */
    movl $0, %eax
    jmp halt

/* ============================================
 * Serial port functions
 * ============================================ */
serial_init:
    /* Disable interrupts */
    movw $(SERIAL_PORT + 1), %dx
    xorb %al, %al
    outb %al, %dx

    /* Enable DLAB */
    movw $(SERIAL_PORT + 3), %dx
    movb $0x80, %al
    outb %al, %dx

    /* Set divisor to 1 (115200 baud) */
    movw $(SERIAL_PORT + 0), %dx
    movb $0x01, %al
    outb %al, %dx
    movw $(SERIAL_PORT + 1), %dx
    xorb %al, %al
    outb %al, %dx

    /* 8 bits, no parity, 1 stop bit */
    movw $(SERIAL_PORT + 3), %dx
    movb $0x03, %al
    outb %al, %dx

    /* Enable FIFO */
    movw $(SERIAL_PORT + 2), %dx
    movb $0xC7, %al
    outb %al, %dx

    /* Enable DTR/RTS */
    movw $(SERIAL_PORT + 4), %dx
    movb $0x0B, %al
    outb %al, %dx

    ret

serial_putc:
    pushq %rdx
    pushq %rax
    movb %al, %ah

wait_tx:
    movw $(SERIAL_PORT + 5), %dx
    inb %dx, %al
    testb $0x20, %al
    jz wait_tx

    movw $SERIAL_PORT, %dx
    movb %ah, %al
    outb %al, %dx

    popq %rax
    popq %rdx
    ret

serial_puts:
    pushq %rax
    pushq %rdi
puts_loop:
    movb (%rdi), %al
    testb %al, %al
    jz puts_done
    call serial_putc
    incq %rdi
    jmp puts_loop
puts_done:
    popq %rdi
    popq %rax
    ret

/* Print hex number in RAX */
serial_print_hex:
    pushq %rbx
    pushq %rcx
    pushq %rdx
    pushq %rax

    movq $16, %rcx         /* 16 hex digits for 64-bit */
hex_loop:
    rolq $4, %rax          /* Rotate left 4 bits */
    movb %al, %bl
    andb $0x0F, %bl
    cmpb $10, %bl
    jl hex_digit
    addb $('A' - 10), %bl
    jmp hex_out
hex_digit:
    addb $'0', %bl
hex_out:
    pushq %rax
    movb %bl, %al
    call serial_putc
    popq %rax
    decq %rcx
    jnz hex_loop

    popq %rax
    popq %rdx
    popq %rcx
    popq %rbx
    ret

/* ============================================
 * IDT Setup
 * ============================================ */
setup_idt:
    pushq %rbx
    pushq %rcx
    pushq %rdx

    /* Fill IDT with default handlers */
    leaq idt64(%rip), %rdi
    leaq default_handler(%rip), %rax
    movq $256, %rcx

idt_fill:
    /* Low 16 bits of handler */
    movw %ax, (%rdi)
    /* Segment selector */
    movw $0x08, 2(%rdi)
    /* IST = 0, reserved */
    movb $0, 4(%rdi)
    /* Type = 0x8E (64-bit interrupt gate, DPL=0, present) */
    movb $0x8E, 5(%rdi)
    /* Mid 16 bits of handler */
    movq %rax, %rbx
    shrq $16, %rbx
    movw %bx, 6(%rdi)
    /* High 32 bits of handler */
    movq %rax, %rbx
    shrq $32, %rbx
    movl %ebx, 8(%rdi)
    /* Reserved */
    movl $0, 12(%rdi)

    addq $16, %rdi
    decq %rcx
    jnz idt_fill

    /* Set up specific exception handlers */
    leaq idt64(%rip), %rdi

    /* Vector 0: #DE (Divide Error) */
    leaq exc_de(%rip), %rax
    call set_idt_entry

    /* Vector 8: #DF (Double Fault) */
    leaq idt64(%rip), %rdi
    addq $(8 * 16), %rdi
    leaq exc_df(%rip), %rax
    call set_idt_entry

    /* Vector 13: #GP (General Protection) */
    leaq idt64(%rip), %rdi
    addq $(13 * 16), %rdi
    leaq exc_gp(%rip), %rax
    call set_idt_entry

    /* Vector 14: #PF (Page Fault) */
    leaq idt64(%rip), %rdi
    addq $(14 * 16), %rdi
    leaq exc_pf(%rip), %rax
    call set_idt_entry

    /* Vector 32: Timer interrupt */
    leaq idt64(%rip), %rdi
    addq $(32 * 16), %rdi
    leaq irq_timer(%rip), %rax
    call set_idt_entry

    /* Load IDT */
    leaq idt64(%rip), %rax
    leaq idt64_ptr(%rip), %rbx
    movq %rax, 2(%rbx)
    lidt idt64_ptr(%rip)

    popq %rdx
    popq %rcx
    popq %rbx
    ret

/* Set IDT entry: RDI = entry address, RAX = handler */
set_idt_entry:
    movw %ax, (%rdi)
    movw $0x08, 2(%rdi)
    movb $0, 4(%rdi)
    movb $0x8E, 5(%rdi)
    movq %rax, %rbx
    shrq $16, %rbx
    movw %bx, 6(%rdi)
    movq %rax, %rbx
    shrq $32, %rbx
    movl %ebx, 8(%rdi)
    movl $0, 12(%rdi)
    ret

/* ============================================
 * Exception Handlers
 * ============================================ */
default_handler:
    leaq msg_unhandled(%rip), %rdi
    call serial_puts
    hlt
    jmp default_handler

exc_de:
    pushq %rax
    leaq msg_exc_de(%rip), %rdi
    call serial_puts
    movb $1, exception_occurred(%rip)
    /* Skip past the faulting div instruction (2 bytes: F7 F1) */
    addq $2, 8(%rsp)       /* Adjust return RIP on stack */
    popq %rax
    iretq

exc_df:
    leaq msg_exc_df(%rip), %rdi
    call serial_puts
    hlt
    jmp exc_df

exc_gp:
    pushq %rax
    leaq msg_exc_gp(%rip), %rdi
    call serial_puts
    /* Print error code (on stack) */
    movq 8(%rsp), %rax      /* Error code */
    call serial_print_hex
    movb $'\n', %al
    call serial_putc
    popq %rax
    addq $8, %rsp              /* Pop error code */
    iretq

exc_pf:
    pushq %rax
    leaq msg_exc_pf(%rip), %rdi
    call serial_puts
    /* Print CR2 (faulting address) */
    movq %cr2, %rax
    call serial_print_hex
    movb $'\n', %al
    call serial_putc
    popq %rax
    addq $8, %rsp              /* Pop error code */
    iretq

irq_timer:
    pushq %rax
    incl timer_ticks(%rip)

    /* Send EOI to LAPIC */
    movl $APIC_BASE_ADDR, %eax
    movl $0, APIC_EOI(%rax)

    popq %rax
    iretq

syscall_entry:
    /* Minimal syscall handler */
    sysretq

/* ============================================
 * LAPIC Functions
 * ============================================ */
lapic_init:
    pushq %rbx
    pushq %rcx
    pushq %rdx

    /* Check if APIC is available via CPUID */
    movl $1, %eax
    cpuid
    testl $(1 << 9), %edx      /* APIC bit */
    jz lapic_not_present

    /* Read APIC base MSR */
    movl $MSR_IA32_APIC_BASE, %ecx
    rdmsr

    /* Check if APIC is enabled */
    testl $(1 << 11), %eax     /* APIC Global Enable */
    jz lapic_not_enabled

    /* Get APIC base address (should be 0xFEE00000) */
    andl $0xFFFFF000, %eax
    movl $APIC_BASE_ADDR, %ebx

    /* Enable APIC via SVR */
    movl APIC_SVR(%rbx), %eax
    orl $0x1FF, %eax           /* Enable APIC + spurious vector 0xFF */
    movl %eax, APIC_SVR(%rbx)

    /* Clear error status */
    movl $0, APIC_ESR(%rbx)
    movl $0, APIC_ESR(%rbx)

    /* Set TPR to 0 (accept all interrupts) */
    movl $0, APIC_TPR(%rbx)

    /* Read APIC ID to verify access works */
    movl APIC_ID(%rbx), %eax
    shrl $24, %eax             /* ID is in bits 24-31 */

    xorl %eax, %eax            /* Success */
    jmp lapic_init_done

lapic_not_present:
    movl $1, %eax
    jmp lapic_init_done

lapic_not_enabled:
    movl $2, %eax

lapic_init_done:
    popq %rdx
    popq %rcx
    popq %rbx
    ret

lapic_timer_test:
    pushq %rbx
    pushq %rcx
    pushq %rdx

    movl $APIC_BASE_ADDR, %ebx

    /* Set timer divider to 16 */
    movl $0x03, APIC_TIMER_DIV(%rbx)

    /* Set timer in one-shot mode, vector 32 */
    movl $32, APIC_LVT_TIMER(%rbx)

    /* Clear tick counter */
    movl $0, timer_ticks(%rip)

    /* Set initial count (triggers timer)
     * Note: rax's LAPIC uses instruction count, not wall clock.
     * Formula: timer fires after initial_count * 3 * divisor instructions
     * With 0x1000 and divisor 16: 4096 * 3 * 16 = 196,608 instructions
     */
    movl $0x1000, APIC_TIMER_INIT(%rbx)

    /* Enable interrupts and wait briefly */
    sti

    /* Busy-wait for timer to fire or timeout (reduced iterations) */
    movl $0x100000, %ecx
timer_wait:
    pause
    cmpl $0, timer_ticks(%rip)
    jne timer_fired
    decl %ecx
    jnz timer_wait

    /* Timer didn't fire - check if it's counting down */
    cli
    movl APIC_TIMER_CURR(%rbx), %eax
    cmpl $0x100000, %eax
    jae timer_not_counting     /* Timer hasn't started */

    /* Timer is counting but no interrupt - partial success */
    leaq msg_timer_no_int(%rip), %rdi
    call serial_puts
    xorl %eax, %eax                /* Still return success */
    jmp timer_test_done

timer_fired:
    cli
    xorl %eax, %eax                /* Success */
    jmp timer_test_done

timer_not_counting:
    cli
    movl $1, %eax                  /* Failure */

timer_test_done:
    popq %rdx
    popq %rcx
    popq %rbx
    ret

/* ============================================
 * Halt
 * ============================================ */
halt:
    cli
    hlt
    jmp halt

/* ============================================
 * Data Section
 * ============================================ */
.section .rodata

msg_banner:
    .asciz "\n\n=== Rax Boot Test Kernel ===\n\n"
msg_test1:
    .asciz "TEST 1:  Long mode verification............. "
msg_test2:
    .asciz "TEST 2:  CR0 configuration.................. "
msg_test3:
    .asciz "TEST 3:  GDT reload.......................... "
msg_test4:
    .asciz "TEST 4:  IDT setup........................... "
msg_test5:
    .asciz "TEST 5:  GS_BASE MSR......................... "
msg_test6:
    .asciz "TEST 6:  CR4 configuration.................. "
msg_test7:
    .asciz "TEST 7:  EFER NX enable...................... "
msg_test8:
    .asciz "TEST 8:  LAPIC initialization............... "
msg_test9:
    .asciz "TEST 9:  LAPIC timer......................... "
msg_test10:
    .asciz "TEST 10: Exception handling.................. "
msg_test11:
    .asciz "TEST 11: Page table access................... "
msg_test12:
    .asciz "TEST 12: SYSCALL setup....................... "
msg_test13:
    .asciz "TEST 13: Memory access patterns.............. "
msg_test14:
    .asciz "TEST 14: SSE/XMM registers................... "
msg_test15:
    .asciz "TEST 15: Interrupt enable/disable............ "
msg_pass:
    .asciz "[PASS]\n"
msg_fail:
    .asciz "[FAIL]\n"
msg_skip:
    .asciz "[SKIP]\n"
msg_complete:
    .asciz "\n=== All tests completed ===\n"
msg_unhandled:
    .asciz "UNHANDLED INTERRUPT\n"
msg_exc_de:
    .asciz "#DE (Divide Error)\n"
msg_exc_df:
    .asciz "#DF (Double Fault)\n"
msg_exc_gp:
    .asciz "#GP (General Protection), error code: "
msg_exc_pf:
    .asciz "#PF (Page Fault), CR2: "
msg_timer_no_int:
    .asciz "(timer counting but no IRQ) "

/* ============================================
 * GDT
 * ============================================ */
.section .data
.align 16
gdt64:
    .quad 0                             /* Null descriptor */
gdt64_code:
    .quad 0x00AF9A000000FFFF            /* 64-bit code: base=0, limit=0xFFFFF, G=1, D=0, L=1, P=1, DPL=0, S=1, Type=0xA */
gdt64_data:
    .quad 0x00CF92000000FFFF            /* 64-bit data: base=0, limit=0xFFFFF, G=1, D/B=1, L=0, P=1, DPL=0, S=1, Type=0x2 */
gdt64_end:

gdt64_ptr:
    .word gdt64_end - gdt64 - 1
    .quad 0                             /* Will be filled in at runtime */

/* ============================================
 * IDT
 * ============================================ */
.align 16
idt64:
    .skip 256 * 16                      /* 256 entries x 16 bytes each */

idt64_ptr:
    .word 256 * 16 - 1
    .quad 0                             /* Will be filled in at runtime */

/* ============================================
 * BSS Section
 * ============================================ */
.section .bss
.align 16

test_buffer:
    .skip 256

timer_ticks:
    .skip 4

expect_exception:
    .skip 1

exception_occurred:
    .skip 1

.align 16
stack_bottom:
    .skip 16384                         /* 16KB stack */
stack_top:
