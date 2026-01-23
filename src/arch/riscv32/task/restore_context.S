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
# We don't want any remaining data from previous task in the registers, even some CSRs
.global restore_context
.type restore_context, @function
restore_context:
  # Move current task context struct save in caller in s0 reg to t6
  mv t6, s0
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
    load_gp %i
    .set	i, i+1
  .endr
  # Using mret we will return and continue execution from mepc
  mret
