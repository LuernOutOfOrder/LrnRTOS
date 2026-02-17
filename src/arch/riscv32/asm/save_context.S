# Save context function
# We don't want any remaining data from previous task in the registers, even some CSRs
.global save_context
.type save_context, @function
save_context:
  # Move current task context struct save in caller in a0 reg to t6
  mv t6, a0
  # Store ra in task context structure
  sw a1, OFFSET_RA(t6)
  # Store word from t0(sp) in context structure
  sw a2, OFFSET_SP(t6)
  # Save current task context using GNU macro
  # GNU macros from `src/arch/riscv32/asm/gnu_macro.S`
  .set	i, 1
  .rept	31
    save_gp_context %i
    .set	i, i+1
  .endr
  ret
