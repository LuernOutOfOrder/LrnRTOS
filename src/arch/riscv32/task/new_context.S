# Create a new  task context. 
# Used when the task has just been created and has no context yet.
.global new_task_context
.type new_task_context, @function
new_task_context: 
  # Move current task context struct save in caller in s0 reg to t6
  mv t6, s0
  # Update sp
  # Load sp from struct in t0
  lw t0, OFFSET_ADDR_SPACE_HI(t6)
  mv sp, t0
  # Load address of task entry point to t0
  mv t0, s1
  # Set mepc to the address of task entry point
  csrw mepc, t0
  # Restore current task context using GNU macro
  # GNU macros from `src/arch/riscv32/asm/gnu_macro.S`
  .set	i, 1
  .rept	31
    load_gp %i
    .set	i, i+1
  .endr
  mret
