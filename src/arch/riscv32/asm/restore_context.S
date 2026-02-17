# Restore context function
# We don't want any remaining data from previous task in the registers, even some CSRs
.global restore_context
.type restore_context, @function
restore_context:
  # Move current task context struct save in caller in a0 reg to t6
  mv t6, a0
  # Update ra
  lw t0, OFFSET_RA(t6)
  mv ra, t0
  # Update sp
  # Restore sp from current task context structure
  lw t0, OFFSET_SP(t6)
  mv sp, t0
  # Update mstatus
  lw t0, OFFSET_MSTATUS(t6)
  csrw mstatus, t0
  # Restore current task context using GNU macro
  # GNU macros from `src/arch/riscv32/asm/gnu_macro.S`
  .set	i, 1
  .rept	31
    load_gp_context %i
    .set	i, i+1
  .endr
  ret

.global trap_restore_context
.type trap_restore_context, @function
trap_restore_context:
  # Move current task context struct save in caller in a0 reg to t6
  mv t6, a0
  # Update sp
  # Restore sp from current task context structure
  lw t0, OFFSET_SP(t6)
  mv sp, t0
  # Update mstatus
  lw t0, OFFSET_MSTATUS(t6)
  csrw mstatus, t0
  # Update mepc
  lw t0, OFFSET_PC(t6)
  csrw mepc, t0
  # Restore current task context using GNU macro
  # GNU macros from `src/arch/riscv32/asm/gnu_macro.S`
  .set	i, 1
  .rept	31
    load_gp_context %i
    .set	i, i+1
  .endr
  mret
