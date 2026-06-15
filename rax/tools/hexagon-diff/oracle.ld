/* Single RWX load segment so the test-packet slot in .text is writable
 * (the oracle patches it per case) and executable. Static, non-PIE. */
PHDRS { all PT_LOAD FLAGS(7); }
SECTIONS {
  . = 0x100000;
  .text   : { *(.text*) }   :all
  .rodata : { *(.rodata*) } :all
  .data   : { *(.data*) }   :all
  .bss    : { *(.bss*) }    :all
}
