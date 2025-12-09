# Kernel Traps handling

## Description

On a machine, a trap is a control transfer that occurs when the processor must stop normal program execution and switch into privileged mode to handle an exceptional condition. 
A trap is triggered either by an exception (a synchronous event) or by an interrupt (an asynchronous event) by writting into specific register so that the hardware can know that a trap has been raised.

When a trap is triggered, the processor performs a well-defined sequence of actions:

1.	It switches to a higher privilege level (for example, from User mode to Machine or Supervisor mode).
2.	It saves the current execution context, usually by writing the program counter and state into architectural registers such as mepc, mcause, or mtval (on RISC-V).
3.	It redirects control flow to a trap handler entry point, whose address is stored in a dedicated register (mtvec on RISC-V).
4.	It hands control to the kernel, which is responsible for diagnosing the cause of the trap and taking the appropriate action.

The kernel’s trap handler must then:

– Inspect the trap cause, using architecture-specific registers.
– Dispatch to the correct handler: interrupt, system call, page fault, illegal instruction, timer interrupt, etc.
– Perform all required work in privileged mode, such as servicing hardware, signaling the scheduler, or handling memory faults.
– Restore the saved context so that execution can continue in the trapped program, unless the trap requires terminating or switching tasks.

## Traps gone wrong

Handling traps can be simple, but when there's error or UB, it's much harder to debug.
Because a trap can be composed by a trap_entry in asm that manage all context saving and restoring, and a trap_handler, that will handle exceptions or interrupts before restoring context.
It can be hard to fix things inside, here's a list of commun mistakes when handling traps:

- Incorrect Trap Vector Configuration:
If the trap vector (mtvec on RISC-V) points to an invalid address, uninitialized memory, or a handler that has not been fully set up, the processor will jump into undefined code paths. This usually results in an immediate reset, a recursive trap, or a silent hang depending on the hardware.

- Corrupted or Missing Context Saving:
A trap handler must preserve enough architectural state to resume execution. If registers such as the program counter, status register, scratch registers, or even just some general purpose registers are not saved and restored correctly, the kernel will attempt to return to inconsistent state. Symptoms include unexpected instruction re-execution, skipping instructions, or returning into random memory.

- Corrupted or Missing Trap stack:
When handling traps, we must used a dedicated stack to avoid using the kernel stack and corrupted it. If the trap configuration is missing a trap stack, or it's corrupted, or else, it can lead to UB.

- Recursive or Unbounded Trap Entry:
A trap handler that triggers another trap before returning—often due to accessing unmapped memory, executing privileged instructions at the wrong time, or enabling interrupts too early—can produce a recursive trap storm. Because the hardware treats each trap as higher priority than the current execution, the kernel can quickly exhaust stack memory or overwrite the saved context.

- Silent Failures:
Some traps do not produce visible failures immediately. An incorrectly restored mstatus, a stale satp value, or an accidentally cleared interrupt enable bit can leave the system in a subtly broken state where scheduling, system calls, or preemption behave inconsistently.
