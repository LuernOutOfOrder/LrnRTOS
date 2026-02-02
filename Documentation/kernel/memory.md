# Kernel memory management

<!--toc:start-->
- [Kernel memory management](#kernel-memory-management)
  - [Description](#description)
  - [Purpose](#purpose)
  - [How it works](#how-it-works)
  - [Kernel stack](#kernel-stack)
  - [Task stack](#task-stack)
  - [Invariants](#invariants)
<!--toc:end-->

## Description

Documentation of how the kernel handle the machine RAM.

## Purpose

The kernel need to use the machine RAM to be able to run correctly. The kernel get the RAM information from the platform layer.
First thing the kernel do with the RAM, is to switch from the temporary stack, define in the linker file, to the final stack, define from the static config file and RAM information from the platform layer.

## How it works

For now, the machine RAM is only used for the kernel stack, there's no heap allocation or something else.
The RAM is define using this structure:

```rust
pub struct Memory {
    pub mem_start: usize,
    pub mem_end: usize,
}
```

When allocating on the RAM for a task stack for exemple, we use the an address called `available`, this address, at the kernel boot, is the last address of the kernel stack, the bottom.
When allocating a task stack, the allocator will get the available address, calculate the lo address from available - size asked.
Available will be the hi address of the new task stack, and the bottom address will become the new available. The new available address is excluded from the task use. The task must never use this address.

## Kernel stack

The kernel stack start at the end of the RAM, and grow downward, so towards the start of the RAM.

```
High addresses
+---------------------+  <- end of RAM
|                     |
|   Kernel stack      |  <- SP initial
|   (grows downward)  |
|                     |
+---------------------+
|                     |
|   Free / heap       |
|                     |
+---------------------+
|                     |
|       Padding       |
|    (if there is)    |
|                     |
+---------------------+
|                     |
|   Kernel data       |
|   (.bss, .data)     |
|                     |
+---------------------+
|                     |
|   Kernel text       |
|   (.text, rodata)   |
|                     |
+---------------------+  <- start of RAM
Low addresses
```

## Task stack

When kernel create new task, it allocate the task stack on RAM, the task stack is allocate just under the kernel stack, and grows downward, like the kernel stack.
Each time the kernel create a new task, it allocate the new task stack under the previous one, etc.

```
High addresses
+---------------------+  <- end of RAM
|                     |
|   Kernel stack      |  <- SP initial
|   (grows downward)  |
|                     |
+---------------------+
|                     |
|     Task 1 stack    |
|                     |
+---------------------+
|                     |
|     Task 2 stack    |
|                     |
+---------------------+
|                     |
|   Free / heap       |
|                     |
+---------------------+
|                     |
|       Padding       |
|    (if there is)    |
|                     |
+---------------------+
|                     |
|   Kernel data       |
|   (.bss, .data)     |
|                     |
+---------------------+
|                     |
|   Kernel text       |
|   (.text, rodata)   |
|                     |
+---------------------+  <- start of RAM
Low addresses
```

## Invariants

- Once the kernel has finalized its boot process, the kernel image, at the bottom of the RAM, must never be accessed from any one else than the kernel.
- The lo address of a task stack must not be used by that task, it's consider excluded from the task stack. 
- The lo address of the kernel stack must not be used by the kernel, it's consider excluded from the stack and will be used as the available address for task.
