.set OFFSET_GP_BASE, 0                 # gp_regs base
.set OFFSET_SATP, 128
.set OFFSET_TRAP_STACK, 132
.set OFFSET_HARTID, 136

.global trap_entry
.type trap_entry, @function
trap_entry:
  csrr	t6, mscratch
  # Save all GP
  .set 	i, 1
  .rept	31
    save_gp %i, t6
    .set	i, i+1
  .endr
  csrr    t0, satp
  sw      t0, OFFSET_SATP(t6)       # store satp
  csrr    t0, mhartid
  sw      t0, OFFSET_HARTID(t6)     # store hartid (optional, redundant)
  sw      sp, (OFFSET_TRAP_STACK - 4)(t6)   # store original sp just before trap_stack slot
  # Switch to save trap stack
  lw      t1, OFFSET_TRAP_STACK(t6)   # t1 = trap_stack pointer
  beqz    t1, trap_no_trapstack
  mv      sp, t1
  csrr a0, mcause
  csrr a1, mepc
  csrr a2, mhartid
  mv a3, t6
  call trap_handler 
  mret

trap_no_trapstack:
  j .
