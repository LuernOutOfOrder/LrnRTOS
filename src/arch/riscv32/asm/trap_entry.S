# Static variable representing offset in TrapFrame structure
# general purpose registers offset
.set OFFSET_GP_BASE, 0
# Supervisor Address Translation and Protection Register (satp register only exist when supervisor mode is enabled)
.set OFFSET_SATP, 128
# current hart id 
.set OFFSET_HARTID, 132
# static array to use as a stack inside the trap
.set OFFSET_TRAP_STACK, 136

.global trap_entry
.type trap_entry, @function
trap_entry:
  # Save task context
  # Get current task ptr
  la t0, TASK_HANDLER # Address in RAM
  lw t1, 0(t0) # Get the value behind the ref
  mv a0, t1
  # Save current pc
  csrr a1, mepc
  # Save current sp
  mv a2, sp
  # Call the save context function
  call trap_save_context
  # Read mscratch into t7
  csrr	t6, mscratch
  lw      t1, OFFSET_TRAP_STACK(t6)   # t1 = trap_stack pointer
  # Branch instruction to check if t1 = 0
  beqz    t1, trap_no_trapstack
  # Move trap stack ptr into sp
  mv      sp, t1
  # Read CSR into function argument registers
  csrr	a0, mepc
  csrr	a1, mtval
  csrr	a2, mcause
  csrr	a3, mhartid
  csrr	a4, mstatus
  # Move trap frame structure into a5 function argument register
  mv a5, t6
  # Call trap_handler rust function
  call trap_handler 

  # Check if a re-schedule is needed or not.
  call read_need_reschedule
  # If a0 != 0, goto 1f, else mret
  bnez a0, 1f
  # If there's no need to a reschedule, restore the task context
  la t0, TASK_HANDLER # Address in RAM
  lw t1, 0(t0) # Get the value behind the ref
  mv a0, t1
  call trap_restore_context
1:
  call scheduler

# Function used when the trap strack = 0, just infinite loop for debuging purpose 
trap_no_trapstack:
  j .
