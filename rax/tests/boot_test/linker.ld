/*
 * Linker script for Rax Boot Test Kernel
 *
 * The kernel is loaded at 1MB (0x100000) which is the standard
 * location for Linux-style direct boot.
 */

ENTRY(_start)

KERNEL_VMA = 0x100000;

SECTIONS
{
    . = KERNEL_VMA;

    /* Boot section first - contains entry point */
    .boot : AT(KERNEL_VMA)
    {
        _kernel_start = .;
        *(.boot)
    }

    . = ALIGN(4096);

    /* Code section */
    .text :
    {
        _text_start = .;
        *(.text)
        *(.text.*)
        _text_end = .;
    }

    . = ALIGN(4096);

    /* Read-only data */
    .rodata :
    {
        _rodata_start = .;
        *(.rodata)
        *(.rodata.*)
        _rodata_end = .;
    }

    . = ALIGN(4096);

    /* Initialized data */
    .data :
    {
        _data_start = .;
        *(.data)
        *(.data.*)
        _data_end = .;
    }

    . = ALIGN(4096);

    /* Uninitialized data */
    .bss :
    {
        _bss_start = .;
        *(.bss)
        *(.bss.*)
        *(COMMON)
        _bss_end = .;
    }

    . = ALIGN(4096);
    _kernel_end = .;

    /* Discard unwanted sections */
    /DISCARD/ :
    {
        *(.comment)
        *(.note.*)
        *(.eh_frame*)
    }
}
