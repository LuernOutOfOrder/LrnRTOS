# RISC-V Traps handling

## Description 

Specific logic for RISC-V traps handling, like CSRs used, structure and asm.
For more global documentation on traps, see: `Documentation/kernel/traps.md`.

## Entering Traps handling

On RISC-V, to trigger a trap, we need to activate interrupts, usually by using the 'mie' CSR. When interrupts is enabled, to trigger an interrupts, the hardware need to write in special CSR.
This CSR is 'mip', it handle all pending interrupts bit, see documentation: `Documentation/arch/riscv/csr.md`.

If the hardware check 'mip', and a register is enabled, the trap is triggered. When a trap is triggered on RISC-V, the hardware will use mtvec to see the trap mode, direct or vectored, and the trap_entry address.


