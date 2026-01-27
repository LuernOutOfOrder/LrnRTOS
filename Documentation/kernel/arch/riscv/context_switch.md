# RISC-V context switch - saving and restoring CPU state.

<!--toc:start-->
- [RISC-V context switch - saving and restoring CPU state.](#risc-v-context-switch-saving-and-restoring-cpu-state)
  - [Description](#description)
  - [How does it works](#how-does-it-works)
    - [Save context](#save-context)
      - [Structure](#structure)
      - [Offsets](#offsets)
      - [Saving context](#saving-context)
    - [Restore context](#restore-context)
<!--toc:end-->

## Description

Documentation about how this kernel saves and restores CPU execution state during a context switch on RISC-V 32 bit systems running in Machine mode.
All registers that must be saved in the task structure when a context save happened, the only registers that are not saved during the context save are sp and t6.
For sp, it has it's own field in the TaskContext structure, and t6 because it's the register containing the task context structure when there's a context switch.

## How does it works

### Structure

To make the context switch work, we need to create a structure to store all context data needed. 
We don't create a general structure on the task structure because each architecture have their own type, registers, etc.
So we need a context structure per architecture.

Here's the context structure for the RISC-V 32 bit.

```rust
pub struct TaskContext {
    // Store all general purpose registers, except sp and t6.
    // Avoid storing sp because there's already a field in the structure for it, easier to get, and t6 because it's the only register use to load the structure in it.
    pub gpr: [u32; 32],           // Offset 0
    // Address space where the task stack will live.
    // See `Documentation/kernel/memory.md` for more information about the memory management of the task stack.
    pub address_space: [u32; 2],  // Offset 128 (first index 128; second index 132)
    // Program counter, where the task will start it's execution, a ptr to the task entry point function.
    pub pc: u32,                  // Offset 136
    // Stack pointer, where the task stack will start, the hi address of address_space field.
    pub sp: u32,                  // Offset 140
    // Flags, not used yet.
    pub flags: [u8; 3],           // Offset 144 (first index 144; second index 145, third index 146)
    // Instruction registers, not used yet.
    pub instruction_register: u8, // Offset 147
}
```

## Invariants

- The memory layout of TaskContext must remain strictly consistent with the assembly offsets. Any modification requires updating both Rust and assembly code.
- All GPRs except sp and t6 must be preserved.
- Context saving must return to kernel execution, not to task execution.
- Context restoration must transfer control back to task execution using mret.
- After restore, execution resumes exclusively via mret.
- No Rust code executes after a successful context restore except from the one from the task.
