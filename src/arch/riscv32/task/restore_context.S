# Restore context function
# We don't want any remaining data from previous task in the registers, even some CSRs
.global restore_context
.type restore_context, @function
restore_context:
  # Move current task context struct save in caller in a0 reg to t6
  mv t6, a0
  # Update sp
  # Restore sp from current task context structure
  lw t0, OFFSET_SP(t6)
  mv sp, t0
  # Restore pc from offset
  lw t0, OFFSET_PC(t6)
  csrw mepc, t0
  # Restore current task context using GNU macro
  # GNU macros from `src/arch/riscv32/asm/gnu_macro.S`
  .set	i, 1
  .rept	31
    load_gp_context %i
    .set	i, i+1
  .endr
  # Read mstatus
  csrr  t0, mstatus
  # Clear mstatus.MPP (bits 12:11)
  li    t1, ~(0x1800)        # mask with MPP bits = 0
  and   t0, t0, t1
  # Set MPP = Machine (0b11 << 11 = 0x1800)
  li    t1, 0x1800
  or    t0, t0, t1
  # Update mstatus
  csrw  mstatus, t0
  # Using mret we will return and continue execution from mepc
  mret
