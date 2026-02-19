# Kernel scheduler

<!--toc:start-->
- [Kernel scheduler](#kernel-scheduler)
  - [Description](#description)
  - [Queues](#queues)
    - [Run queue](#run-queue)
    - [Blocked queue](#blocked-queue)
  - [Preemption](#preemption)
  - [Scheduling model](#scheduling-model)
  - [Invariants](#invariants)
<!--toc:end-->

## Description

The kernel scheduler is in charge of managing which task to run, updating the run queue and the blocked queue and updating the task state.

## Queues

There's 2 queues used in the scheduler. The `run queue` and the `blocked queue`. Each queue is specific for a `CPU core`. The `CPU core 1` has a different `run queue` than the `CPU core 2`.

### Run queue

The `run queue` contains all the task ready to be run. In the `ready` task state.

The `run queue` work using a `FIFO` queue, there's a queue for each `priority`, up to `32 queues`.
To find which queue to use to execute a task, instead of iterating over all queues, we use a `bitmap`.
There's only `32 priorities`, so we use a `u32 bitmap`, each bit representing a `run queue`, if the bit is set, there's at least 1 task to run.
Else, the queue is empty.

### Blocked queue

The `blocked queue` contains all the task currently blocked. With different block reasons.
But currently the `blocked queue` contains uniquely the task blocked using the `sleep` task primitive.

The `blocked queue` work using an `indexed linked list`, the list is sorted from the shortest awake tick to the largest awake tick.
The list is manage using `head` and `tail`, a bit like a `ring buffer` data structure.
So when we need to check the next task to awake, we just check the `head` of the `blocked queue`.
If it can be awake, it will update the `need_resched` flag, then it'll trigger a reschedule, the scheduler will be able to awake the task, and move it from the `blocked queue` to the `run queue`.

## Preemption

The scheduler is preemptive, meaning that if a task as a higher priority than the current task, it will run this higher priority task.
The current task having a lowest priority, will be saved, and re-execute when there's no higher priority task to run.

## Scheduling model

The current scheduling model is a `cooperative priority based`. Meaning that if there's `2 task` with the same `priority`, if they don't call `yield`, one task will `always run`.
Making the other task `starving`.
So if you want to switch from a `task` to another one, you need to use `cooperative` functions, like `yield` or `sleep`.

## Invariants

- The scheduler need at least one task in the `run queue`, if the `run queue` is empty, it will try to run the idle task. Make sure that there's always at least one task in the `run queue`, or enable the `idle task` feature.
- The scheduler assume that the `CpusState` is initialized to access the `CPU core scheduler state`.
- The scheduler can be called from the `trap epilogue`, if so, a `trap frame`, should be available and accessible for the scheduler to run on.
