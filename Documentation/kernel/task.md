# Kernel task model

<!--toc:start-->
- [Kernel task model](#kernel-task-model)
  - [Description](#description)
  - [Purpose](#purpose)
  - [Structure](#structure)
  - [References](#references)
<!--toc:end-->

## Description

A task in the kernel is the entity in charge of running given code. It has is own "address space" on the machine. Has it's own context, own stack, etc.

## Purpose

The kernel being a real-time kernel, it has to be able to run given task and schedule it. For now the task run in the same binary than the kernel.

## Structure

The task is structured like that:

```rust
enum TaskState {
    New,
    Running,
    Ready,
    Waiting,
    Terminated,
}

#[repr(C)]
struct Task {
    // Arch dependant context, don't handle this field in task, only use struct method when
    // interacting with it.
    context: TaskContext,
    // Fn ptr to task entry point, this must never return.
    // This will surely disappear
    func: fn() -> !,
    pid: u16,
    name: [u8; 16],
    // Task state, when creating a new task, use the new variant.
    state: TaskState,
    // Priority of a task, use an u8, u8 max size represent the higher level of priority.
    priority: u8,
}

// The context of a task being arch dependant, there's a structure per arch, example with the Risc-V 32 bits structure
pub struct TaskContext {
    pub gpr: [u32; 32],
    pub address_space: [u32; 2],
    pub pc: u32,
    pub sp: u32,
    pub flags: [u8; 3],
    pub instruction_register: u8,
}
```

## References

- Core Dumped process video: `https://www.youtube.com/watch?v=LDhoD4IVElk`.
- Linux kernel task model: `https://github.com/torvalds/linux/blob/master/include/linux/sched.h`.
