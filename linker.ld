ENTRY(_start)

SECTIONS
{
  . = 0x80000000;       /* Adresse de boot de QEMU virt */

  .text : {
    *(.text.entry)      /* ton _start */
    *(.text*)           /* le reste du code */
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
}
