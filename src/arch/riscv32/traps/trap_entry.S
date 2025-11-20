.option norvc
.global trap_entry
trap_entry:
  # All registers are volatile here, we need to save them
  # before we do anything.
  csrrw	t6, mscratch, t6
  # We use t6 as the temporary register because it is the very
  # bottom register (x31)
  .set 	i, 1
  .rept	30
    save_gp	%i
    .set	i, i+1
  .endr
  # Save the actual t6 register, which we swapped into
  # mscratch
  mv		t5, t6
  csrr	t6, mscratch
  save_gp 31, t5

  # Restore the kernel trap frame into mscratch
  csrw	mscratch, t5
  # Get ready to go into trap_handler
  # We don't want to write into the user's stack or whomever
  # messed with us here.
  csrr	a0, mepc
  csrr	a1, mtval
  csrr	a2, mcause
  csrr	a3, mhartid
  csrr	a4, mstatus
  mv		a5, t5
  lw		sp, 388(a5) # Offset of sp in TrapFrame structure
  call trap_handler 
  # When we get here, we've returned from trap_handler, restore registers
  # and return.
  # trap_handler will return the return address via a0.

  csrw	mepc, a0

  # Now load the trap frame back into t6
  csrr	t6, mscratch

  # Restore all GP registers
  .set	i, 1
  .rept	31
    load_gp %i
    .set	i, i+1
  .endr

  # Since we ran this loop 31 times starting with i = 1,
  # the last one loaded t6 back to its original value.

  mret
