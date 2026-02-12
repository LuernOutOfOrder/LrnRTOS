# Static variable representing offset in TrapFrame structure
# general purpose registers offset
.set OFFSET_GP_BASE, 0
# Supervisor Address Translation and Protection Register (satp register only exist when supervisor mode is enabled)
.set OFFSET_SATP, 128
# current hart id 
.set OFFSET_HARTID, 132
# static array to use as a stack inside the trap
.set OFFSET_TRAP_STACK, 136

.global trap_entry
.type trap_entry, @function
trap_entry:
  # Read mscratch into t6
  csrr	t6, mscratch
  # Save all GP registers using GNU macro
  # Just a loop 
  .set 	i, 1
  .rept	31
    save_gp %i, t6
    .set	i, i+1
  .endr
  # Read CSR and store word in correct offset of trap frame structure
  csrr    t0, satp
  sw      t0, OFFSET_SATP(t6)       # store satp
  csrr    t0, mhartid
  sw      t0, OFFSET_HARTID(t6)     # store hartid (optional, redundant)
  sw      sp, (OFFSET_TRAP_STACK - 4)(t6)   # store original sp just before trap_stack slot
  # Load content of trap stack offset into t1
  lw      t1, OFFSET_TRAP_STACK(t6)   # t1 = trap_stack pointer
  # Branch instruction to check if t1 = 0
  beqz    t1, trap_no_trapstack
  # Move trap stack ptr into sp
  mv      sp, t1
  # Read CSR into function argument registers
  csrr	a0, mepc
  csrr	a1, mtval
  csrr	a2, mcause
  csrr	a3, mhartid
  csrr	a4, mstatus
  # Move trap frame structure into a5 function argument register
  mv a5, t6
  # Call trap_handler rust function
  call trap_handler 
  # Load all register back to previous state
  csrr	t6, mscratch
  # Restore all GP registers
  .set	i, 1
  .rept	31
    load_gp %i
    .set	i, i+1
  .endr
  # Check if a re-schedule is needed or not.
  call read_need_reschedule
  # If a0 != 0, goto 1f, else mret
  bnez a0, 1f
  mret
1:
  call scheduler
  mret

# Function used when the trap strack = 0, just infinite loop for debuging purpose 
trap_no_trapstack:
  j .
