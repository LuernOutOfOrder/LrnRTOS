# Kernel data structure

<!--toc:start-->
- [Kernel data structure](#kernel-data-structure)
  - [Description](#description)
    - [RingBuffer](#ringbuffer)
      - [Invariants](#invariants)
    - [AlignedStack16](#alignedstack16)
      - [Invariants](#invariants-1)
    - [IndexedLinkedList](#indexedlinkedlist)
      - [Invariants](#invariants-2)
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

### IndexedLinkedList

A linked list but store in an array, so each node is accessed from it's index in the array.
This is better to use this data structure as a side storage, like we use it to store blocked task.
Task are already store in a TaskList, so in the IndexedLinkedList used for blocked task we only store task id, and task awake tick for now.

#### Invariants

- All node should be accessible from the head node.
- The list is sorted naturally from the `value` field of a node.
- The `count` field should reflect the number of accessible node in the list.
- The list is empty when `count`, `head` and `tail` are equal to 0.
- If the `next_node` of a node is some, this `next_node` is valid.
- If the `next_node` of a node is none, then this node is the `tail`.
