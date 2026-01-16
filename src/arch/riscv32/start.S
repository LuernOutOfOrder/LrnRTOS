.section .text.entry
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
  la sp, stack_top    
  # We use mret here so that the mstatus register
  # is properly updated.
  # Set mstatus to M-mode
  li t0, (0b11 << 11)
  csrw	mstatus, t0
  # load address of stack_top
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
