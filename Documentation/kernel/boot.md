# Kernel boot flow

## Description

This document describes the boot flow of LrnRTOS. It is intended for developers working on the kernel or modifying early initialization routines.
The goal is to outline the phases of boot, the invariants established at each stage, and the restrictions on subsystem usage before these invariants are guaranteed.
This ensures that anyone extending or modifying the kernel understands the assumptions and guarantees at each point in the boot process.

## Prerequisites

For the kernel to be able to boot correctly, it will need:

- At least 8kb stack.
- CPU in M-mode.
- To know if the machine has a FDT, if not, need to define basic devices in static.
- The config file setup correctly.
- Disable the kprint module in Cargo.toml if no need or if you don't know a correct serial device address. 
- Only boot on mono-core machine ! No multi-core handled for now.

## Non-goals

This documentation does not describe exact instruction-level execution, only kernel-level invariants and phases.

## Boot phases

Initialize all different component of the kernel.
After all phases, the kernel guarantees that all core subsystems are initialized, traps are handled, memory is available, and drivers can be safely probed.

### Platform

Initialize the platform layer [1], this is the first phase of early boot, it allows the kernel to know on what type of platform it runs basically, like embedded or not(with FDT).
It guarantees that the kernel can initialize properly all sub-systems and memory.
If the platform layer is not initialized first, the sub-systems cannot be initialized properly.
Without the platform layer, the early boot cannot continue, basically.

### Sub-systems 

Initialize all kernel sub-systems(serial,timer,etc) [2]. 
It guarantees that all kernel's sub-systems is initialized properly.
The sub-systems use static from config file to set a static size for sub-systems pool. The kernel consider that all sub-systems pool have a correct size. If there's a panic during boot,
consider this invariant violated.
The sub-systems depends on the platform layer, it cannot be initialized and run properly without it.
Without the sub-systems the kernel cannot do anything, because all the kernel depends on different sub-systems.

### Trap-frame

Initialize the trap-frame, used to trap handling [3].
This will just correctly initialized the trap-frame structure used for trap handling.
It guarantees that all trap handling can work correctly using the trap-frame.
If the trap-frame is not initialized, trap handling cannot work, there's no obligation to initialize the trap-frame at this point, there's no dependency for the trap-frame init.
It just need to be initialized before enabling interruptions, and that's the next boot phase, so I find that correct to init trap-frame now.
Without the trap-frame, core services such as scheduling, interrupts, and exception handling cannot function.

### Enabling interruptions

Enable all interruptions and exceptions available on the machine.
Before enabling interruptions, we set a safety delay using the timer sub-systems, to avoid trigger a timer interruption before the kernel finalize booting properly.
This will guarantees that the trap handling can be trigger.
If the interruptions is not enabled, there's no trap handling, no trap handling, nothing working basically.
This need to be initialized after the sub-systems because the trap handler will use the sub-systems for trap handling, like timer interruption for exemple. 

### Memory 

Initialize machine memory, like ram.
It need the platform to be initialized, but it doesn't depends on sub-systems or trap handling, so it can be initialized just after the platform.
This guarantees the kernel to use the memory from the machine instead of the one define in the linker script at compilation.
It will be used for first: changing the temporary kernel stack with definitie stack, and used for all memory allocation on the heap(if there's one).
If the memory is not initialized, the kernel will not work properly, unless you modify the linker script for a specific machine but it's not recommended at all.
After the memory is initialized, the kernel will jump directly to the main functions, there cannot be return on functions modifying the stack pointer.
So if we want to initialize the memory just after plaftorm, we need to modify all the early boot functions, because there cannot be return after changing sp, it's easier to just jump to the main functions.
All of the allocation made in early boot was made on the temporary stack from the linker script. All sub-systems, drivers, anything else, is static. So after changing the stack, the kernel is like all clean up, ready to start is job.

## WARNINGS

This section cover a list of knowns and common errors on the early boot flow. 
It follow the following structure: Symptoms (what problem you encounter): Likely cause (what invariant is possibly broken).

### Drivers

- Driver failed to init: wrong definition of the driver in static or wrong compatible string given.

### Platform

- Use the wrong discovery mode: incorrect use of platform flags mode.

### Sub-systems

- Possible pool overflow: increase sub-system pool size in config file.

### Interruptions

- Failed to handle basic interruption: trap frame not properly initialized.
- Timer interrupt not handled properly: possible error in the timer sub-system. 

## Diagram

Show the current boot flow.

+----------------------+
|       Hardware       |
|   (FDT or STATIC)    |
+----------+-----------+
           v            
      +----------+      
      | Platform |      
      +----+-----+      
           |            
           |            
           v            
     +-----------+      
     |Sub-systems|      
     +-----+-----+      
           |            
           v            
     +------------+     
     |            |     
     | Trap-frame |     
     | (init only)|
     +-----+------+     
           |            
           |            
           v            
     +------------+     
     | Enable     |     
     | Interrupts |     
     +-----+------+     
           |            
           v            
        +-----------+        
        |  Memory   |        
        |(RAM init) |
        +--+--------+        
           |
           v            
 +---------------------+
 | Jump to kernel main |
 | avoid return because|
 | of temp stack       |
 +---------------------+

## Invariants

All the boot sequence assume that none of the invariant of any phases are violated.

## References

[1] platform documentation: `Documentation/kernel/platform.md`.
[2] sub-systems documentation: `Documentation/kernel/subsystems.md`.
[3] traps documentation: `Documentation/kernel/traps.md`.
