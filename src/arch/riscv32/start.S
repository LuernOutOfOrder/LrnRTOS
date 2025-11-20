.section .text.entry
.global kstart
.type kstart, @function
# Do not allow interrupts while running kinit
csrw mie, zero
la t1, kstart
csrw mepc, t1
kstart:
  la sp, stack_top    # load address of stack_top
  j _start
