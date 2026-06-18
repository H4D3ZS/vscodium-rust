/* Linker script for bare-metal x86_64 microkernel */
/* Binary loaded at 0x1000000 (16MB) by the emulator */

ENTRY(_start)

SECTIONS
{
    . = 0x1000000;

    .text : {
        *(.text._start)
        *(.text .text.*)
    }

    .rodata : {
        *(.rodata .rodata.*)
    }

    .data : {
        *(.data .data.*)
    }

    .bss : {
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        __bss_end = .;
    }

    /* Stack grows down from 0x1010000 */
    . = 0x1010000;
    __stack_top = .;

    /* Heap starts at 0x1100000 */
    . = 0x1100000;
    __heap_start = .;

    /* Heap ends at 0x1700000 (6MB heap) */
    . = 0x1700000;
    __heap_end = .;

    /DISCARD/ : {
        *(.comment)
        *(.note*)
        *(.eh_frame*)
    }
}
