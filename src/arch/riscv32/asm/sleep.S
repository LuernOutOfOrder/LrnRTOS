.global sleep
.type sleep, @function
sleep:
  # Move tick to another reg, a0 is used in save_context
  mv t5, a0
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
  # Once save_context return, call the task_set_wake_tick fn
  mv a0, t5
  call task_set_wake_tick
