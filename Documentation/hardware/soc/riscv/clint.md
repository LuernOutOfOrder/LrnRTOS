# RiscV Core Local Interrupt (CLINT)

## Description 

The clint is a soc device only in riscv. It is responsible for maintaining memory mapped control and status registers which are associated with the software and timer interrupts. Basically, the clint is used to trigger timer or softwate interrupt or send interrupt to a specific hart. 

## Properties

### Reg

The clint used a region memory in MMIO like other devices. 

### Interrupt-extended

The clint has a property called interrupt-extended, it contains a list of hart id and irqs id.

It can be read like that: (&cpu_id, irq_id), where the &cpu_id is actually, in the device tree, a phandle to a cpu interrupt-controller to a specific hart, hence the &cpu_id.
The interrupt-extended is basically a list of (&cpu_id, irq_id).

## Registers

### mtime

#### Description:

mtime register is used to return current timer value. The hardware automatically increment mtime at each CPU cycle. 

#### Info:

Offset: 0xBFF8

Size: 64 bits

Reset: 0x0 hex

Access-type: RW

### mtimecmp

#### Description:

mtimecmp register hold the compare value for the timer. It can be used to trigger timer interrupt. The hardware automatically compare mtime with mtime cmp. If timer interrupt is enabled, the hardware will check mtime >= mtimecmp, if true, interrupt happened.

#### Info:

Offset: 0x4000

Size: 64 bits 

Reset: 0x0 hex

Access-type: RW

### msip

#### Description:

This register is used to send software interrupt to a specific hart. 

#### Info:

Offset: 0x0 + (hard_id * 4)

Size: 32 bits

Reset: 0x0 hex

Access-type: RW

## Driver API

The driver expose 3 functions used by the timer sub-system:

- read_mtime(): Read current mtime at the mtime register and return the value as u64.
- set_mtimecmp(hart_id: usize, delay: u64): Set mtimecmp at the mtime register on given hart_id with the delay params. For safety, when writting to mtimecmp, on Risc-V 32 bits, the register is still 64 bits, so we cannot write the delay params in one time. So we need to write two time, one high address and one low address. We use bitwise shifting to "split" the delay params in two parts. Then we start by the high address and set it to 0xFFFFFFFF, it avoid to trigger a timer interrupt when updating mtimecmp. Then we write at the lower address and rewrite at the high address.
- send_ipi(hart_id: usize): Write one bit to the msip register of the hart_id params.

## References

`https://chromitem-soc.readthedocs.io/en/latest/clint.html`
