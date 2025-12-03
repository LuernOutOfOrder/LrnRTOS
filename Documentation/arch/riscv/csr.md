# RISCV CSR

## CSR roles

The RiscV CSR(Control and Status Register) are special purpose register. They are used to manage and observe hardware behavior. Unlike global registers, CSR are not used to store and compute ordinary values. They exposed machine state, like interrupt configurations, trap handling, privilege level, or other low-level controler over the CPU.

Accessing CSR lets software read or modify hardware behavior directly. Typical operations include enabling or disabling interrupts, configuring trap vectors, reading exception causes, saving return addresses after traps, or switching between privilege modes.

## Machine-Level CSRs

All CSR are not accessible, it depends on the machine privilege, in M-mode(machine mode), all CSRs are accessible. Here's is a list of all CSR used in the kernel, with description, how they are currently used, and what's inside. 

### mstatus - Machine Status

Description: Global machine state, control the CPU behavior in M-mode.
Important bit: 

- MIE - Machine Interrupt Enable (3 bit) (RW): Manage all interruptions.
- MPIE (7 bit) (RW): When an exception is triggered, MPIE will be set to MIE. When the mret instruction is executed, the value of MPIE will be stored to MIE.
- MPP - Machine Previous Priviledge (12:11 bits) (WARL (0x0, 0x3)): Returns the previous privilege mode. When an mret is executed, the privilege mode is change to the value of MPP.

### mie - Machine Interrupt Enable

Description: All in the name, CSR used to enable specific interruptions.
Important bit:

- MSIE - Machine Software Interrupt Enable (3 bit) (RW): Manage software interruption enabling.
- MTIE - Machine Timer Interrupt Enable (7 bit) (RW): Manage timer interruption enabling.

### mtvec - Machine Trap-Vector

Description: Manage trap handling behavior.
Important bit:

- MODE (1:0) (WARL (0x0, 0x1)): Change the trap handling mode, 0 = direct mode, 1 = vectored mode.
- trap entry address (31:1): Write the address of the trap entry function, written in all bits excepted the first bit.



## References

Official CSR documentation: `https://docs.openhwgroup.org/projects/cv32e40s-user-manual/en/latest/control_status_registers.html`
