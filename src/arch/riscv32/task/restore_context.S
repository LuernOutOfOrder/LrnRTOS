# General purpose registers offset
.set OFFSET_GP, 0
# Task address space hi and lo addr
.set OFFSET_ADDR_SPACE_HI, 128
.set OFFSET_ADDR_SPACE_LO, 132
# Program counter offset
.set OFFSET_PC, 136
# Stack pointer offset (will be set to OFFSET_ADDR_SPACE_HI at first if sp == 0)
.set OFFSET_SP, 140
# Optionnal flags, will be use later maybe
.set OFFSET_FLAGS, 144
.set OFFSET_INSTRUCTION_REG, 147

# Restore context function
.global restore_context
.type restore_context, @function
restore_context:
  # Move current task context struct save in caller in s1 reg to t6
  mv t6, s1
  # Restore current task context using GNU macro
  bnez t6, 1f
1:
  .set	i, 1
  .rept	31
    load_gp %i
    .set	i, i+1
  .endr
    # Check if sp from current tack context structure is 0
  lw t0, OFFSET_SP(t6) 
  beqz t0, 2f
2:
  # Move hi address of task stack to sp
  lw t0, OFFSET_ADDR_SPACE_HI(t6)
  mv sp, t0
3:
  # Restore pc from offset
  lw t0, OFFSET_PC(t6)
  csrw mepc, t0
