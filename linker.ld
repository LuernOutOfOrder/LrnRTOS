ENTRY(_start)

MEMORY {
  RAM (rwx) : ORIGIN = 0x80200000, LENGTH = 128K
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
    .stack (NOLOAD) : { stack_start = .; . += 0x2000; stack_top = .; } > RAM

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
