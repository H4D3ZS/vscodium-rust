/*
 * 32-bit to 64-bit transition stub
 *
 * This code is responsible for:
 * 1. Setting up identity-mapped page tables
 * 2. Enabling PAE and long mode
 * 3. Jumping to the 64-bit entry point
 *
 * This mimics what the Linux kernel's decompressor does.
 */

.code32
.section .boot32, "ax"

/* Constants */
.set KERNEL_START,      0x100000
.set PAGE_TABLES,       0x1000          /* Put page tables at 4KB */

.set CR0_PE,            (1 << 0)
.set CR0_PG,            (1 << 31)

.set CR4_PAE,           (1 << 5)
.set CR4_PGE,           (1 << 7)

.set MSR_EFER,          0xC0000080
.set EFER_LME,          (1 << 8)

.set PTE_PRESENT,       (1 << 0)
.set PTE_WRITABLE,      (1 << 1)
.set PTE_USER,          (1 << 2)
.set PTE_HUGE,          (1 << 7)

.globl _start32
_start32:
    cli
    cld

    /* Set up a stack */
    mov esp, 0x7C00

    /* Clear screen (optional, helps debugging) */
    mov edi, 0xB8000
    mov ecx, 80 * 25
    mov ax, 0x0720          /* Space with light gray on black */
    rep stosw

    /* Print "32" to show we're in 32-bit mode */
    mov dword ptr [0xB8000], 0x0F330F32     /* "32" in white */

    /* Set up identity-mapped page tables for first 4GB
     * We use 2MB pages (huge pages) for simplicity
     *
     * Memory layout:
     * 0x1000: PML4 (1 entry pointing to 0x2000)
     * 0x2000: PDPT (4 entries pointing to 0x3000, 0x4000, 0x5000, 0x6000)
     * 0x3000: PD for 0-1GB (512 x 2MB pages)
     * 0x4000: PD for 1-2GB
     * 0x5000: PD for 2-3GB
     * 0x6000: PD for 3-4GB
     */

    /* Clear page table area */
    mov edi, PAGE_TABLES
    xor eax, eax
    mov ecx, 6 * 1024       /* 6 pages x 4KB = 24KB, 6K dwords */
    rep stosd

    /* PML4[0] -> PDPT at 0x2000 */
    mov dword ptr [PAGE_TABLES], 0x2000 | PTE_PRESENT | PTE_WRITABLE

    /* PDPT entries pointing to PD tables */
    mov dword ptr [0x2000 + 0*8], 0x3000 | PTE_PRESENT | PTE_WRITABLE
    mov dword ptr [0x2000 + 1*8], 0x4000 | PTE_PRESENT | PTE_WRITABLE
    mov dword ptr [0x2000 + 2*8], 0x5000 | PTE_PRESENT | PTE_WRITABLE
    mov dword ptr [0x2000 + 3*8], 0x6000 | PTE_PRESENT | PTE_WRITABLE

    /* Fill PD tables with 2MB huge pages */
    /* PD for 0-1GB at 0x3000 */
    mov edi, 0x3000
    mov eax, PTE_PRESENT | PTE_WRITABLE | PTE_HUGE
    mov ecx, 512
.fill_pd0:
    mov [edi], eax
    add eax, 0x200000       /* Next 2MB page */
    add edi, 8
    loop .fill_pd0

    /* PD for 1-2GB at 0x4000 */
    mov edi, 0x4000
    mov eax, 0x40000000 | PTE_PRESENT | PTE_WRITABLE | PTE_HUGE
    mov ecx, 512
.fill_pd1:
    mov [edi], eax
    add eax, 0x200000
    add edi, 8
    loop .fill_pd1

    /* PD for 2-3GB at 0x5000 */
    mov edi, 0x5000
    mov eax, 0x80000000 | PTE_PRESENT | PTE_WRITABLE | PTE_HUGE
    mov ecx, 512
.fill_pd2:
    mov [edi], eax
    add eax, 0x200000
    add edi, 8
    loop .fill_pd2

    /* PD for 3-4GB at 0x6000 */
    mov edi, 0x6000
    mov eax, 0xC0000000 | PTE_PRESENT | PTE_WRITABLE | PTE_HUGE
    mov ecx, 512
.fill_pd3:
    mov [edi], eax
    add eax, 0x200000
    add edi, 8
    loop .fill_pd3

    /* Print "PT" to show page tables are set up */
    mov dword ptr [0xB8004], 0x0F540F50     /* "PT" in white */

    /* Load CR3 with PML4 address */
    mov eax, PAGE_TABLES
    mov cr3, eax

    /* Enable PAE in CR4 */
    mov eax, cr4
    or eax, CR4_PAE
    mov cr4, eax

    /* Enable Long Mode in EFER MSR */
    mov ecx, MSR_EFER
    rdmsr
    or eax, EFER_LME
    wrmsr

    /* Enable paging (and thus activate long mode) */
    mov eax, cr0
    or eax, CR0_PG
    mov cr0, eax

    /* Print "LM" to show long mode is active */
    mov dword ptr [0xB8008], 0x0F4D0F4C     /* "LM" in white */

    /* Now in compatibility mode, need to load 64-bit GDT and far jump */
    lgdt gdt32_ptr

    /* Far jump to 64-bit code */
    jmp 0x08:trampoline64

/* 64-bit trampoline - still at low address */
.code64
trampoline64:
    /* Set up 64-bit segments */
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov ss, ax
    xor ax, ax
    mov fs, ax
    mov gs, ax

    /* Print "64" to show we're in 64-bit mode */
    mov dword ptr [0xB800C], 0x0F340F36     /* "64" in white */

    /* Jump to the 64-bit kernel at KERNEL_START */
    mov rax, KERNEL_START
    jmp rax

/* GDT for 64-bit mode */
.align 16
gdt32:
    .quad 0                             /* Null descriptor */
    .quad 0x00AF9A000000FFFF            /* 64-bit code */
    .quad 0x00CF92000000FFFF            /* 64-bit data */
gdt32_end:

gdt32_ptr:
    .word gdt32_end - gdt32 - 1
    .long gdt32
