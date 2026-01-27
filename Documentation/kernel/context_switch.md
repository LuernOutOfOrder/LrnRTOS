# Kernel context switch

<!--toc:start-->
- [Kernel context switch](#kernel-context-switch)
  - [Description](#description)
  - [Purpose](#purpose)
  - [How does it works](#how-does-it-works)
<!--toc:end-->

## Description

Context switch allow the kernel to save and restore the entire CPU state. 
It allow the kernel to switch task, and not keep remaining data from previous task in the CPU registers.

## Purpose

To ensure that the kernel can run multiple task, on multiple core or not, the kernel need to make sure that all tasks have their own context.
Without that, a task can have a wrong state after a context switch, because without saving and restoring all the CPU state, there can be some remaining value in the CPU registers.
Making the task compute wrong things, or just have completely UB.

## How does it works

For now the kernel just have a cooperative scheduling, all task need to call explicitly the yield function to force a reschedule and pass the turn to the next task.
All task are currently saved in a task list, available from a public API. But it doesn't make the cooperative scheduling works. 
There's a static, used to store the current running task, it store the `pid` of the current running task, the kernel only working on a monocore CPU.
From now on, there's: a list containing all task, and a static storing the `pid` of the current running process. The cooperative scheduling can work on that, but it's DIY.
I've added a `FIFO queue`, from a `RingBuffer type`, it's used to store all `Ready` task, so when calling the yield, and trigger a reschedule, the kernel only have to look into this `FIFO queue` to find the next task to run.
Then, the reschedule will get the task from the static `handler(pid)`, update it to make it `Ready`, get the next task from the `FIFO queue` or ready task, update it to `Running` state, update the static `handler(pid)` and context switch on it.

## Invariants

- When a task is saved, its state is correctly saved in its own context.
- When a new task is restore, it's CPU state is correctly restored, its execution restart at `PC` and in its own stack.
- State of multiple task must not be crossed corrupted.
