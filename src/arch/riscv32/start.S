# Disable generation of compressed instructions.
# Only use 32 bits instructions
.option norvc

# Define a .data section.
# Symbol in .data section
.section .data

# Define a .text.init section.
# Code in .text.init section
.section .text.init

.global kstart
.type kstart, @function
# Do not allow interrupts while running kinit
kstart:
  # Any hardware threads (hart) that are not bootstrapping
	# need to wait for an IPI
	csrr	t0, mhartid
	bnez	t0, 3f
	# SATP should be zero, but let's make sure
	csrw	satp, zero
  # Disable linker instruction relaxation for the `la` instruction below.
	# This disallows the assembler from assuming that `gp` is already initialized.
	# This causes the value stored in `gp` to be calculated from `pc`.
  # It basically avoid that gp being garbage and use the PC-relative option (safest)
.option push
# Don't use the relax option
.option norelax
  # Load address of global ptr
  la		gp, __global_ptr
.option pop
  # load address of stack_top
  la sp, stack_top    

  # Load .data from binary to RAM
  # .data destination
  la a0, __data_start
  la a1, __data_end
  # .data src
  la a2, __data_la_start

  bgeu a0, a1, 1f
1:
  # if a0 >= a1, jump to 2f (unsigned)
  bgeu a0, a1, 2f  
  lw t0, 0(a2)
  sw t0, 0(a0)
  # dst += 4
  addi    a0, a0, 4
  # src += 4
  addi    a2, a2, 4      
  j       1b
2:
  
  # The BSS section is expected to be zero
  # Load bss region 
	la a0, __bss_start
	la a1, __bss_end

  # if a0 >= a1, jump to 2forward (unsigned)
	bgeu a0, a1, 2f
  # Zero to .bss with basic recursive loop
1:
  # Store 4 bytes to a0
  sw zero, (a0)
  # Add 4 bytes to a0 to jump to next word
  addi a0, a0, 4
  # if a0 < a1, jump to 1backward (unsigned)
  bltu a0, a1, 1b
# .bss is zeroed, continue boot
2:
  # We use mret here so that the mstatus register
  # is properly updated.
  # Set mstatus to M-mode
  li t0, (0b11 << 11)
  csrw	mstatus, t0
  # load address of _start function
  la t1, _start
  # Set mepc to t1
  csrw	mepc, t1
  # Set hartid to a0, some machine do it directly
  # I prefer to do it like that to be sure
  csrr a0, mhartid
  # mret will return directly to the kernel entry point in rust
  la		ra, 4f
  mret

3:
	# Parked harts go here. We need to set these
	# to only awaken if it receives a software interrupt,
	# which we're going to call the SIPI (Software Intra-Processor Interrupt).
	# We only use these to run user-space programs, although this may
  # change.

4:
	wfi
  j		4b
