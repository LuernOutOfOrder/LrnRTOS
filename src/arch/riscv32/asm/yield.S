.global yield
.type yield, @function
yield:
  # Get current task ptr
  la t0, TASK_HANDLER # Address in RAM
  lw t1, 0(t0) # Get the value behind the ref
  mv a0, t1
  # Save current ra
  mv a1, ra
  # Save current sp
  mv a2, sp
  # Call the save context function
  call save_context
  # Once save_context return, call the scheduler
  call dispatch
