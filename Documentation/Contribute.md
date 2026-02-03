<!--toc:start-->
- [Writing Rust code](#writing-rust-code)
  - [unwrap](#unwrap)
  - [expect](#expect)
  - [panic](#panic)
  - [Implicit type](#implicit-type)
  - [No recursion](#no-recursion)
  - [No infinite loop](#no-infinite-loop)
  - [No heap allocation on the kernel logic](#no-heap-allocation-on-the-kernel-logic)
<!--toc:end-->

## Writing Rust code

I added warnings on some Rust usage. Here are some rules about what you shouldn't do, or if you do, how you write it clean:

### unwrap

You should never use unwrap directly, Option<> is a design choice, and when you need to interact with it, ask yourself those questions:

- Is None possible?
- Is None acceptable?
- Is None a bug?

Depending on the answer, you should use a different pattern, like match, or just change the type to not use Option<>.

### expect

Expect should also never be used, but only during the runtime; it's acceptable to use expect, especially during the boot process, if an invariant is violated.
But when you use an expect, you need to write a clear, explicit message, and always mark it with an allow(clippy::expected_used), and comment on why you use it.

### panic

Panic is the extreme case; when using panic, it's only when you consider that the kernel will be, or is already, in an unexpected state.
The kernel is corrupted, and it's better to fail-fast than continue the execution with an unstable kernel.

So panic should only be used when an invariant is violated, no matter in which state the kernel is.

### Implicit type

I don't like implicit type, so here's some rules about when a type shouldn't be implicit and where it can:

- The closer a value is to the material or an invariant, the more explicit its type should be.
- The more local, ephemeral, and conceptual a value is, the more acceptable the inference.

Best case is all variable have an explicit type, but if it's an integer for a loop, it's ok if it's implicit, we don't care.

### No recursion

No recursion, at all. Hard to analyze statically and to debug.

### No infinite loop

Only infinite loop authorize are the one on the function that shouldn't returned, or on some boot process, when you don't know how huge the loop should be, but apart from that, no infinite loop should be used.
When you need a loop, always use a max iteration. And if it's shouldn't reach the end, and it's specified that it's an invariant, you can use panic or else.

### No heap allocation on the kernel

The kernel should always use static array or static type but never use heap allocation for a vector for example. If a user want to heap allocate on his task, he can, but not on the kernel.
All allocation in the kernel core should be static allocation.
