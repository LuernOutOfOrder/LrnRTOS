# Scheduler context switch

# Offsets
.set OFFSET_GPR, 0
.set OFFSET_RA, 128
.set OFFSET_SP, 132

.global sched_ctx_restore
.type sched_ctx_restore, @function
sched_ctx_restore:
  # Get the scheduler context from fn arg
  mv t6, a0 
  # Load ra
  lw t0, OFFSET_RA(t6)
  mv ra, t0
  # Load sp
  lw t0, OFFSET_SP(t6) 
  mv sp, t0
  # GNU macro to load all registers
  .set	i, 1
  .rept	31
    load_gp_context %i
    .set	i, i+1
  .endr
  ret 

.global sched_ctx_save
.type sched_ctx_save, @function
sched_ctx_save:
  # Get the scheduler context from fn arg
  mv t6, a0 
  # Sp
  mv t0, sp
  sw t0, OFFSET_SP(t6)
  # GNU macro to load all registers
  .set	i, 1
  .rept	31
    save_gp_context %i
    .set	i, i+1
  .endr
  ret
