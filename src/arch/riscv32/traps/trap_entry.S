.global trap_entry
.type trap_entry, @function
trap_entry:
  #csrr	t6, mscratch
  #mv t5,t6
  csrr a0, mcause
  csrr a1, mepc
  csrr a2, mhartid
  csrr a3, mtval
  csrr a4, mstatus
  #mv a5, t5
  #lw		sp, 388(a5) # Offset of sp in TrapFrame structure
  call _trap_handler 
  mret
