# Kernel task model

<!--toc:start-->
- [Kernel task model](#kernel-task-model)
  - [Description](#description)
  - [Purpose](#purpose)
  - [Structure](#structure)
  - [How task is store](#how-task-is-store)
  - [Idle task](#idle-task)
  - [Invariants](#invariants)
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

pub enum TaskBlockControl {
    // Store the awake tick for task awakening.
    AwakeTick(usize),
    // No reason for the task block
    None,
}

#[repr(C)]
struct Task {
    // Arch dependant context, don't handle this field in task, only use struct method when
    // interacting with it.
    context: TaskContext,
    // Task block control, define the reason the task is blocked.
    block_control: TaskBlockControl,
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
    pub ra: u32,
    pub mstatus: u32,
    pub flags: [u8; 3],
    pub instruction_register: u8,
}
```

## How task is store

Task are saved in a task list structure when creating a new task. This list is used to store all tasks and will be used in the scheduler.

Task list structure:

```rust
pub struct TaskList {
    // Static array with defined size in config file
    // Where all the tasks are stored
    list: [Option<Task>; TASK_LIST_MAX_SIZE],
    // Used to know which task has been add last and used when creating a new task to increment this, update it and use it as the new task pid
    last_pid: u16,
    // Size of the list, used to know how many task are stored
    size: u8,
}
```

## Idle task

The idle task is used to ensure that the kernel as always at least one task able to run.
This task is created at the lowest priority to ensure it does not use any CPU time if there are higher priority application tasks in the run queue.
It is not possible to update the idle task, it's a static defined task. 

## Invariants

- The task's function must never return.
- There can't be the same pid in different task.
- The state of a task must always be updated when its state change, the state must always reflect the current task's state.

## References

- Core Dumped process video: `https://www.youtube.com/watch?v=LDhoD4IVElk`.
- Linux kernel task model: `https://github.com/torvalds/linux/blob/master/include/linux/sched.h`.
