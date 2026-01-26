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

Logic for context switch in RISC-V 32 bit.

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

Next to each field there's a comment with `Offset` and a value, this is the offset of the field in the structure. 
It's used in the asm function to use that field.
We need to define the OFFSET using constant.

### Offsets

```asm
# General purpose registers offset
.set OFFSET_GP, 0
# Task address space hi and lo addr
.set OFFSET_ADDR_SPACE_HI, 128
.set OFFSET_ADDR_SPACE_LO, 132
# Program counter offset
.set OFFSET_PC, 136
# Stack pointer offset (will be set to OFFSET_ADDR_SPACE_HI at first if sp == 0)
.set OFFSET_SP, 140
# Optionnal flags, will be use later maybe
.set OFFSET_FLAGS, 144
.set OFFSET_INSTRUCTION_REG, 147
```


### Save context

When entering the asm function, we pass first in Rust, in first argument, a ptr to the task context structure:

```asm
mv t6, a0
```

So now we can use the offset define above using the t6 register.
Then we save the current pc:

```asm
mv t0, sp
```

Saving the current sp so we know where to restart in the task stack when we reswitch to that task.
Then, we save the sp, in the task context structure:

```asm
sw t0, OFFSET_SP(t6)
```

We save the sp exactly in the correct field of the structure using the structure ptr store in t6 and the offset define above.
The last specific value to save is the pc, on RISC-V, the pc is store in a CSR, see the documentation on CSR: `Documentation/arch/riscv/csr.md`.

```asm
csrr t0, mepc
sw t0, OFFSET_PC(t6)
```

Here, we read the `mepc` CSR and store it in t0, then we saved t0 in the task structure at the pc offset.
After that we save all general purpose registers. We don't want to save them all at once, so we use a gnu macro:

```asm
# GNU macros from `src/arch/riscv32/asm/gnu_macro.S`
  .set	i, 1
  .rept	31
    save_gp_context %i
    .set	i, i+1
  .endr
```

It's basically a loop that save all general purpose register starting from the OFFSET_GP.
It just avoid saving sp and t6.

After that the only thing to do is to update mstatus to make the kernel stay in M-mode, without disabling the interruptions:

```asm
# Read mstatus
  csrr  t0, mstatus
  # Clear mstatus.MPP (bits 12:11)
  li    t1, ~(0x1800)        # mask with MPP bits = 0
  and   t0, t0, t1
  # Set MPP = Machine (0b11 << 11 = 0x1800)
  li    t1, 0x1800
  or    t0, t0, t1
  # Update mstatus
  csrw  mstatus, t0
  # Using ret we will return and continue execution just after this function in the caller
  # We don't want to use mret to avoid returning in the task function entry point
  ret
```

This is how the kernel save a task context.

### Restore context


