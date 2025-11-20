.global trap_entry
.type trap_entry, @function
  trap_entry:
    # All registers are volatile here, we need to save them
    # before we do anything.
    csrrw	t6, mscratch, t6
    # We use t6 as the temporary register because it is the very
    # bottom register (x31)
    // .set 	i, 1
    // .rept	30
    //     save_gp	%i
    //     .set	i, i+1
    // .endr
    # Save the actual t6 register, which we swapped into
    # mscratch
    mv		t5, t6
    csrr	t6, mscratch
    save_gp 31, t5

    # Restore the kernel trap frame into mscratch
    csrw	mscratch, t5
    csrr	a0, mepc
    csrr	a1, mtval
    csrr	a2, mcause
    csrr	a3, mhartid
    csrr	a4, mstatus
    mv		a5, t5
    lw		sp, 520(a5)
    call trap_handler 
    mret

