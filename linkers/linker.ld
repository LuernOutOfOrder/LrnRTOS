ENTRY(kstart)

MEMORY {
  RAM (rwx) : ORIGIN = 0x80000000, LENGTH = 128K
}

SECTIONS {
    .text : {
    *(.text.entry)
    *(.text*)
  } > RAM

  .rodata : {
    *(.rodata*)
  } > RAM

  .data : {
    *(.data*)
  } > RAM

  .bss (NOLOAD) : {
    *(.bss*)
    *(COMMON)
  } > RAM

  /* Stack */
  .stack (NOLOAD) : ALIGN(4)
{
  stack_top = ORIGIN(RAM) + LENGTH(RAM);  /* stack starts at RAM top */
  . = stack_top - 0x2000;                 /* 0x4000 = 16 KiB */
  stack_start = .;
} > RAM
  
  /* make sure the symbols are provided to the symbol table */
  PROVIDE(__stack_start = stack_start);
  PROVIDE(__stack_top = stack_top);

  /* Debug info */
  .debug_info     0 : { *(.debug_info) }
  .debug_abbrev   0 : { *(.debug_abbrev) }
  .debug_aranges  0 : { *(.debug_aranges) }
  .debug_line     0 : { *(.debug_line) }
  .debug_str      0 : { *(.debug_str) }
  .debug_loc      0 : { *(.debug_loc) }
  .debug_ranges   0 : { *(.debug_ranges) }
  .debug_macinfo  0 : { *(.debug_macinfo) }
}
