# Create a new  task context. 
# Used when the task has just been created and has no context yet.
.global new_task_context
.type new_task_context, @function
new_task_context: 
  # Move current task context struct save in caller in a0 reg to t6
  mv t6, a0
  # Update sp
  # Load sp from struct in t0
  lw t0, OFFSET_ADDR_SPACE_HI(t6)
  mv sp, t0
  # Load address of task entry point to t0
  mv t0, a1
  # Set mepc to the address of task entry point
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
  mret
