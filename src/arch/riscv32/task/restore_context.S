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
  call restore_gpr
  # Update sp
  # Check if sp from current tack context structure is 0
  call update_sp
  # Restore pc from offset
  lw t0, OFFSET_PC(t6)
  csrw mepc, t0

# Restore general purpose register, always, even if they are empty.
# We don't want any remaining value from previous task in the registers.
restore_gpr:
  # GNU macros from `src/arch/riscv32/asm/gnu_macro.S`
  .set	i, 1
  .rept	31
    load_gp %i
    .set	i, i+1
  .endr
  ret

# Check value of sp in current task context structure
# If sp == 0, set sp to top of the task stack
# Else, set sp to the saved sp in current task context structure
update_sp:
  # Load sp from struct in t0
  lw t0, OFFSET_SP(t6) 
  # Check if t0 != 0
  # If t0 != 0, goto 1f
  bnez t0, 1f

  # If equal to zero
  lw t0, OFFSET_ADDR_SPACE_HI(t6)
  mv sp, t0
  # Jump directly to ret branch
  j 2f
1:
  # Restore sp from current task context structure
  lw t0, OFFSET_SP(t6)
  mv sp, t0
2:
  ret
