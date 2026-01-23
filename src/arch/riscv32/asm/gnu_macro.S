.altmacro
.set NUM_GP_REGS, 32  # Number of registers per context
.set NUM_FP_REGS, 32
.set REG_SIZE, 4   # Register size (in bytes)
.set MAX_CPUS, 8   # Maximum number of CPUs

# Use macros for saving and restoring multiple registers
.macro save_gp i, basereg=t6
	sw	x\i, ((\i)*REG_SIZE)(\basereg)
.endm
.macro load_gp i, basereg=t6
	lw	x\i, ((\i)*REG_SIZE)(\basereg)
.endm
.macro save_fp i, basereg=t6
	fsw	f\i, ((NUM_GP_REGS+(\i))*REG_SIZE)(\basereg)
.endm
.macro load_fp i, basereg=t6
	flw	f\i, ((NUM_GP_REGS+(\i))*REG_SIZE)(\basereg)
.endm
# Macro used to load context from all saved general purpose registers store in t6 during a context switch
# Avoid loading sp and t6 registers to avoid overriding both
.macro load_gp_context i, basereg= t6
  .if ((\i) != 2) && ((\i) != 31)
    lw x\i, ((\i)*REG_SIZE)(\basereg)
  .endif
.endm
