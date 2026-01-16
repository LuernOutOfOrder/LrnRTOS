# RISC-V CSR

<!--toc:start-->
- [RISC-V CSR](#risc-v-csr)
  - [CSR roles](#csr-roles)
  - [Machine-Level CSRs](#machine-level-csrs)
    - [mstatus - Machine Status](#mstatus-machine-status)
    - [mie - Machine Interrupt Enable](#mie-machine-interrupt-enable)
    - [mtvec - Machine Trap-Vector](#mtvec-machine-trap-vector)
    - [mscratch - Machine Scratch](#mscratch-machine-scratch)
    - [mcause - Machine Cause](#mcause-machine-cause)
    - [mepc - Machine Exception Program Counter](#mepc-machine-exception-program-counter)
    - [mtval - Machine Trap Value](#mtval-machine-trap-value)
    - [mip - Machine Interrupt Pending](#mip-machine-interrupt-pending)
  - [CSR Instructions](#csr-instructions)
  - [References](#references)
<!--toc:end-->

## CSR roles

The RISC-V CSR(Control and Status Register) are special purpose register. They are used to manage and observe hardware behavior. Unlike general purpose registers, CSR are not used to store and compute ordinary values. They exposed machine state, like interrupt configurations, trap handling, privilege level, or other low-level control over the CPU.

Accessing CSR lets software read or modify hardware behavior directly. Typical operations include enabling or disabling interrupts, configuring trap vectors, reading exception causes, saving return addresses after traps, or switching between privilege modes.

## Machine-Level CSRs

All CSR are not accessible, it depends on the machine privilege, in M-mode(machine mode), all CSRs are accessible. Here's is a list of all CSR used in the kernel, with description, how they are currently used, and what's inside. 

### mstatus - Machine Status

Description: Global machine state, control the CPU behavior in M-mode.

Important bit: 

- MIE - Machine Interrupt Enable (3 bit) (RW): Manage all interruptions.
- MPIE (7 bit) (RW): When an exception is triggered, MPIE will be set to MIE. When the mret instruction is executed, the value of MPIE will be stored to MIE.
- MPP - Machine Previous Privilege (12:11 bits) (WARL (0x0, 0x3)): Returns the previous privilege mode. When an mret is executed, the privilege mode is changed to the value of MPP.

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

### mscratch - Machine Scratch

Description: Used to save ptr to a trap frame structure, the structure is used when entering trap entry to save all global registers and use a dedicated stack for trap handling instead of the kernel stack.

Important bit:

- Scratch value (31:0) (RW): the mscratch csr is just a XLEN bits register used to store the address of the trap frame structure.

### mcause - Machine Cause

Description: Used when a trap happened.

Important bit:

- Interrupt (31) (RW): The bit is set(1) when an interrupt happened, else it's an exception. This bit is used to know what type of trap was triggered.
- Exccode (30:0) (WLRL): All remaining bits from 30:0 contains the interrupt or exception code. 

### mepc - Machine Exception Program Counter

Description: Program counter csr, used when mret, it reload the program at the address contains in mepc

Important bit:

- mepc (31:1) (WARL): When an exception occured, the current pc is saved in mepc. When mret after handling exception, the value from mepc replaces the current pc.

### mtval - Machine Trap Value

Description: Optional csr, used on specific exceptions or interrupts, for example, on load access fault, mtval will contains the faulty address, on instruction fault exception it will contains the illegal instruction. But on other exception it can be 0.

Important bit:

- mtval (31:0) (WARL): Contains optional trap value.

### mip - Machine Interrupt Pending

Description: Used when an interrupt is raised. When a timer interrupt happened, the hardware update the mip.MTIP to 1, and enter trap handler.

Important bit:

- MSIP (3) (R): Machine Software Interrupt pending enabled.
- MTIP (7) (R): Machine Timer Interrupt pending enabled.
- MEIP (11) (R): Machine External Interrupt pending enabled.

## CSR Instructions

Zicsr instructions set.

RISC-V CSR cannot be accessed with simple instruction like mv or else, it can only be accessed by using csr instructions. Here's a list of csr instructions used in the kernel, with a description, how they work, how we use them, and why.

- csrrs - Atomic Read and Set Bits in CSR: reads CSR value, zero-extends the value to XLEN bits, and write it to integer register rd. The value in rs1 is treated like a bit mask, it specified bit position to be set in the CSR. Any bit that is high in rs1 will cause the corresponding bit to be set in the CSR, if that CSR bit is writable. Other bits in the CSR are not explicitly written.
- csrrc - Atomic Read and Clear Bits in CSR: clears bits from rs1 in CSR value, zero-extends the value to XLEN bits, and write it to integer register rd. The value in rs1 is treated like a bit mask, it specified bit position to be cleared in the CSR. Any bit that is high in rs1 will cause the corresponding bit to be cleared in the CSR, if that CSR bit is writable. Other bits in the CSR are not explicitly written.

For csrrs and csrrc, if rs1 = x0, the instruction will not write at all in the CSR.

- csrw - Write in CSR: Write rs1 into CSR. Used for example to write the trap frame ptr to mscratch.
- csrr - Read from CSR: Read CSR into rd.

The thing is, csrw and csrr, are not real instructions, they are pseudoinstructions, for example:

When we use csrw csr, rs1, the assembler expands this to: csrrw x0, csr, rs1.

Using csrw and csrr is just like a shortcut.

csrrw is used to like this:

csrrw rd, csr, rs1

csrrw will write the value of rs1 into the csr, and return the previous value of the csr in rd.
If rd = x0, then the old value is discard.

So using csrw instead of csrrw is like using csrrw but with x0 as rd.

## References

Official CSR documentation: `https://docs.riscv.org/reference/isa/priv/priv-csrs.html?utm_source=chatgpt.com#csr-field-specifications`
OpenHW group CSR documentation: `https://docs.openhwgroup.org/projects/cv32e40s-user-manual/en/latest/control_status_registers.html`
Zicsr csr instruction documentation: `https://five-embeddev.com/riscv-user-isa-manual/Priv-v1.12/csr.html`
Machine-Level ISA: `https://five-embeddev.com/riscv-priv-isa-manual/Priv-v1.12/machine.html`
