# Kernel scheduler

## Description

The kernel scheduler is in charge of managing which task to run, updating the run queue and the blocked queue and updating the task state.

### Queues

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
If it can be awake, it will update the `need_resched` flag,
