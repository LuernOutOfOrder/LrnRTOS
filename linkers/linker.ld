ENTRY(kstart)

MEMORY {
  RAM (rwx) : ORIGIN = 0x80000000, LENGTH = 128K
}

SECTIONS {
  . = ORIGIN(RAM);
  /* Provide kernel image start */
  kernel_start = .;

  .text : {
    /* Custom section for asm init */
    *(.text.init)
    *(.text*)
  } > RAM

  global_ptr = .;

  .rodata : {
    *(.rodata*)
  } > RAM

  .data : {
    data_la_start = LOADADDR(.data);
    data_start = .;
    *(.data*)
    data_end = .;
  } > RAM

  /* Don't write .bss section in bin file */
  .bss (NOLOAD) : {
    bss_start = .;
    *(.bss*)
    /* Avoid any global uninitialized variable to be saved in another section */
    *(COMMON)
    bss_end = .;
  } > RAM

  /* Provide kernel image end */
  kernel_end = .;

  /* Stack */
  /* Don't write stack section in bin file, just reserve the memory in the RAM */
  .stack (NOLOAD) : ALIGN(4) {
    /* stack starts at RAM top */
    stack_top = ORIGIN(RAM) + LENGTH(RAM);
    /* 0x4000 = 16 KiB */
    stack_bottom = stack_top - 0x2000;
  } > RAM
  
  /* make sure the symbols are provided to the symbol table */
  PROVIDE(__global_ptr = global_ptr);
  PROVIDE(__stack_start = stack_bottom);
  PROVIDE(__stack_top = stack_top);
  PROVIDE(__bss_start = bss_start);
  PROVIDE(__bss_end = bss_end);
  PROVIDE(__data_la_start = data_la_start);
  PROVIDE(__data_start = data_start);
  PROVIDE(__data_end = data_end);
  PROVIDE(__kernel_start = kernel_start);
  PROVIDE(__kernel_end = kernel_end);

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
