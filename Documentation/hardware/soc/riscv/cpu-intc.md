# RISC-V Cpu Interrupt-Controller

## Description

The CPU Interrupt-Controller, or cpu-intc for shorter, is not like a real device. It's a local part of a CPU core, it can raise three type of interruptions:

- Timer interrupt.
- Software interrupt.
- External interrupt bit.

It's the cpu-intc that make possible the traps to work. The cpu-intc will check CSRs register to know if interruptions is enabled or not. See `Documentation/arch/riscv/csr.md` for more informations about the CSR used for interruptions.


## Properties

### Hart id

The id of the CPU core. Used to know at which core send interrupt.

## What does the CPU Interrupt-Controller does ?

That's the part of the hart that handle local interruptions.
It's not a separate part of the hardware, it's part of the CPU core.

Also including:

- Priorisation logic.
- Use CSRs(mie, mip, sie, sip) to check and know if a trap has to be handled or not.
- switch to trap mode with mtvec.
- save some CSRs, like mcause, mepc, mtval.
- handle all traps, exception or interrupts.

## Flow

           +------------------------------+
           |            Core              |
           |                              |
           |   +-----------------------+  |
           |   |   CPU Local INTC      |  |
           |   |  (per-hart logic)     |  |
           |   |                       |  |
 event --->|-->| - illegal instr       |  |
 external -|-->| - MTIP (mtimecmp)     |  |
 interrupt |   | - MSIP                |  |
 from PLIC |   | - MEIP (signaled)     |  |
           |   +----------|------------+  |
           |              |               |
           |              v               |
           |           triggers           |
           |        trap sequence         |
           +------------------------------+

## Driver Structure

```rust
pub struct RiscVCpuIntc {
    hart_id: u32,
}
```

## References

`https://www.kernel.org/doc/Documentation/devicetree/bindings/interrupt-controller/riscv%2Ccpu-intc.txt`
