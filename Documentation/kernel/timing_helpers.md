# Kernel timing helpers

<!--toc:start-->
- [Kernel timing helpers](#kernel-timing-helpers)
  - [Description](#description)
    - [delay](#delay)
    - [Invariants](#invariants)
<!--toc:end-->

## Description

The kernel sometimes need to use some helpers to handle or manage timing. Here's a list of some helpers to help with that.

### delay

Block the CPU for the given time, in `ms`.
This is not really recommended to use, it will not put the CPU to sleep, just waiting for the next timer interrupt.
If you need a task to wait or something else, prefer the use of `yield`.

### Invariants

- The scheduler must be initialized before any timing helpers is used.
