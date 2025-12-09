# RISC-V Traps handling

## Description 

Specific logic for RISC-V traps handling, like CSRs used, structure and asm.
For more global documentation on traps, see: `Documentation/kernel/traps.md`.

## Entering Traps handling

On RISC-V, to trigger a trap, we need to activate interrupts, usually by using the 'mie' CSR. When interrupts is enabled, to trigger an interrupts, the hardware need to write in special CSR.
This CSR is 'mip', it handle all pending interrupts bit, see documentation: `Documentation/arch/riscv/csr.md`.

If the hardware check 'mip', and a register is enabled, the trap is triggered. When a trap is triggered on RISC-V, the hardware will use mtvec to see the trap mode, direct or vectored, and the trap_entry address.

We use 'mscratch' CSR to store the address of the static trap frame used to save context and use a trap stack.

The trap entry sequence is entirely driven by hardware. It saves the PC into mepc, populates mcause and mtval, and redirects execution to the trap vector without any software involvement.

Then it's the trap_entry that will first, handle the context save, we use a macro and a field in the trap_frame to save all General Purpose Register into the trap_frame. 
We save some CSRs in the trap_frame, load the trap stack from trap_frame into a register, check if the stack is valid, move the trap stack ptr to sp, mv CSRs into argument functions registers, and call trap_handler function. 

## Handling traps

In the trap_handler function, we use bitwise and bit masking to know the trap type(exception or interrupt), and the trap cause. 
We dispatch to the correct function to handle the trap by checking the trap type, and the cause. Most of the exception will just panic, because on a small kernel for real time we cannot handle as much exception as on general purpose, I guess.
If it can be handle, it will and the trap_handler function will return and runtime will continue in the caller function.

## Restore context

After returning from the callee, the trap_entry function will reload previous state, before the trap was triggered, it'll reload all save General Purpose Register from the trap_frame into correct registers, and use mret.

## References

Rust OS blog that helped: `https://osblog.stephenmarz.com/ch4.html`
