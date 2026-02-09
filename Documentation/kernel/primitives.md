# Kernel primitives types and functions

<!--toc:start-->
- [Kernel primitives types and functions](#kernel-primitives-types-and-functions)
  - [Description](#description)
    - [Primitive type](#primitive-type)
      - [Description](#description-1)
    - [Task primitive](#task-primitive)
      - [Description](#description-2)
      - [yield](#yield)
      - [sleep](#sleep)
      - [task_awake_blocked](#taskawakeblocked)
      - [Invariants](#invariants)
<!--toc:end-->

## Description

This document describes the kernel primitives.

In the context of this kernel, a *primitive* is defined as a low-level construct that
**directly affects kernel state or execution context**.
If using a type or function can change global kernel behavior, scheduling state,
or execution flow, it is considered a primitive.

Two categories of primitives are documented here:

- **Primitive types**: low-level types whose correct usage is required to preserve
  kernel invariants. These types are not mere data structures; they encode execution,
  synchronization, or memory-layout guarantees that the kernel relies on.
  Examples include synchronization objects such as mutexes or types enforcing
  strict alignment or execution constraints.

- **Task primitives**: execution control operations that may only be used from
  task context. These primitives modify the scheduling or blocking state of the
  current task and therefore have observable effects on global kernel execution.

Pure data structures that do not alter kernel state or execution context are
documented separately and are not considered primitives, even if they are used
internally by primitive implementations.
You can find data structure type here: `Documentation/kernel/data_structure.md`.

### Primitive type

#### Description

There are currently no primitive type implemented in the kernel.

### Task primitive

#### Description

To handle task correctly, the kernel need some primitives but only used in a task context.
These type can't and must not be used anywhere else than inside a task.

#### yield

Used in cooperative scheduling, when a task use `yield`, it will save it's context, and call a re-schedule.
It is used when you want a task to let another task take the control.

#### sleep

Put the task using the `sleep` primitive function to sleep for the given time.
It blocked the task until the kernel `GLOBAL_TICK` is equal or superior to the current tick + the given tick.
You can consider the given tick as `1ms`.

#### task_awake_blocked

Awake the oldest blocked task if it can.
This primitive is called from a timer interrupt, and only from a timer interrupt.
The timer interrupt will give the primitive `task_awake_blocked` the current `GLOBAL_TICK`, after updating it from the current interrupt.
The primitive will get the `oldest blocked task`, from the `BLOCKED_QUEUE`, then it'll check the reason why this task is blocked, and awake it if possible.

#### Invariants

- Task primitives must only be called from task context.
- The scheduler must be initialized before any task primitive is used.
- Time-based primitives rely on a functional timer subsystem.
- Principal data structure such as `RUN_QUEUE` and `BLOCKED_QUEUE` must be initialized before any task primitive is used.
