# Save context function
# We don't want any remaining data from previous task in the registers, even some CSRs
.global save_context
.type save_context, @function
save_context:
  # Clear mstatus.MPP and mstatus.mie to avoid interruption while saving and switching context
  li   t0, ((3 << 11) | (1 << 3))
  csrrc x0, mstatus, t0
  # Move current task context struct save in caller in a0 reg to t6
  mv t6, a0
  # Store ra in task context structure
  sw a1, OFFSET_RA(t6)
  # Store word from t0(sp) in context structure
  sw a2, OFFSET_SP(t6)
  sw a3, OFFSET_PC(t6)
  # Save current task context using GNU macro
  # GNU macros from `src/arch/riscv32/asm/gnu_macro.S`
  .set	i, 1
  .rept	31
    save_gp_context %i
    .set	i, i+1
  .endr
  ret

.global trap_save_context
.type trap_save_context, @function
trap_save_context:
  # Clear mstatus.MPP and mstatus.mie to avoid interruption while saving and switching context
  li   t0, ((3 << 11) | (1 << 3))
  csrrc x0, mstatus, t0
   # Move current task context struct save in caller in a0 reg to t6
  mv t6, a0
  # Store ra in task context structure
  sw a1, OFFSET_PC(t6)
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
