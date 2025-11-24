.section .text.entry
.global kstart
.type kstart, @function
# Do not allow interrupts while running kinit
kstart:
  la sp, stack_top    # load address of stack_top
  j _start
