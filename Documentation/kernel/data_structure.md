# Kernel data structure

<!--toc:start-->
- [Kernel data structure](#kernel-data-structure)
  - [Description](#description)
    - [RingBuffer](#ringbuffer)
      - [Invariants](#invariants)
    - [AlignedStack16](#alignedstack16)
      - [Invariants](#invariants-1)
<!--toc:end-->

## Description

The kernel as multiple data structure implementation inside the codebase, they are useful to store and manipulate data inside the kernel.
These are all the data structure implemented inside the kernel.

### RingBuffer

A simple Ring buffer, used as an FIFO. If the RingBuffer is full and you try to push anyways, it will be abort.
It's like a close RingBuffer, you can't push if it's full and you don't pop before.

#### Invariants

- Length is always in [0, capacity].
- The length is len - 1; there's always an empty slot in the array.
- Head and tail always remain within the backing array bounds.
- Push is only valid when the buffer is not full; violating this is a logic error (abort).
- Pop is only valid when the buffer is not empty; violating this is a logic error (abort).

### AlignedStack16

This is just a structure wrapping a buffer of bytes, but the structure use the `#[repr(align(16))]`.
This type is used when you need a stack on a buffer, and the `sp` must be aligned on `16 bytes`.

#### Invariants

- The backing storage is always 16-byte aligned.
- Any stack pointer derived from this type must remain 16-byte aligned at all call boundaries.
- This type provides a memory-layout guarantee only; it does not validate stack usage correctness.
