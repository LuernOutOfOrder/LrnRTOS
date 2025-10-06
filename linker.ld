ENTRY(_start)

SECTIONS
{
  . = 0x80200000;

  .text : {
    *(.text.entry)      /* entry point */
    *(.text*)
  }

  .rodata : {
    *(.rodata*)
  }

  .data : {
    *(.data*)
  }

  .bss : {
    *(.bss*)
    *(COMMON)
  }

  . = ALIGN(16);
  _stack_start = .;
  . += 64K;
  _stack_top = .;
}
