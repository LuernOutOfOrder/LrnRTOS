# Kernel primitives types and functions

## Description

The kernel has multiple primitive to improve the codebase, and qol(quality of life) for developers.
It goes from primitive type like circular buffer, to more specific primitive function like sleep for a task.

### Primitive type

#### Description

The lowest primitive type are the Rust types, but beside them there's the kernel primitive type.
Those can be used anywhere in the kernel. Here's a list of the primitive type currently available in the kernel:

#### RingBuffer

A simple Ring buffer, used as an FIFO. If the RingBuffer is full and you try to push anyways, it will be abort.
It's like a close RingBuffer, you can't push if it's full and you don't pop before.

#### AlignedStack16

This is just a structure wrapping a buffer of bytes, but the structure use the `#[repr(align(16))]`.
This type is used when you need a stack on a buffer, and the `sp` must be aligned on `16 bytes`.

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

#### delay

Block the CPU for the given time, in `ms`.
This is not really recommanded to use, it will not put the CPU to sleep, just waiting for the next timer interrupt.
If you need a task to wait or something else, prefer the use of `yield`. 
