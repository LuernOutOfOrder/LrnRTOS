# Kernel boot flow

## Description

This document describes the boot flow of LrnRTOS. It is intended for developers working on the kernel or modifying early initialization routines.
The goal is to outline the phases of boot, the invariants established at each stage, and the restrictions on subsystem usage before these invariants are guaranteed.
This ensures that anyone extending or modifying the kernel understands the assumptions and guarantees at each point in the boot process.

## Boot phases

Initialize all different component of the kernel.

### Plaftorm

Initialize the platform layer [1], this is the first phase of early boot, it allows the kernel to know on what type of platform it runs basically, like embedded or not(with FDT).
It guarantees that the kernel can initialize properly all sub-systems and memory.
If the platform layer is not initialized first, the sub-systems cannot be initialized properly.
Without the platform layer, the early boot cannot continue, basically.

### Sub-systems 

Initialize all kernel sub-systems(serial,timer,etc) [2]. 
It guarantees that all kernel's sub-systems is initialized properly.
The sub-systems depends on the platform layer, it cannot be initialized and run properly without it.
Without the sub-systems the kernel cannot do anything, because all the kernel depends on different sub-systems.

### Trap-frame

Initialize the trap-frame, used to trap handling [3].
This will just correctly initialized the trap-frame structure used for trap handling.
It guarantees that all trap handling can work correctly using the trap-frame.
If the trap-frame is not initialized, trap handling cannot work, there's no obligation to initialize the trap-frame at this point, there's no dependency for the trap-frame init.
It just need to be initialized before enabling interruptions, and that's the next boot phase, so I find that correct to init trap-frame now.
Without the trap-frame, there's no trap handling, if there's no trap handling, there's no scheduling, timer interrupt, exception handling, nothing.

### Enabling interruptions



## References

[1] platform documentation: `Documentation/kernel/platform.md`.
[2] sub-systems documentation: `Documentation/kernel/subsystems.md`.
[3] traps documentation: `Documentation/kernel/traps.md`.
