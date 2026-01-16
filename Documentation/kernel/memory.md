# Kernel memory management

## Description

Documentation of how the kernel handle the machine RAM.

## Purpose

The kernel need to use the machine RAM to be able to run correctly. The kernel get the RAM information from the platform layer.
First thing the kernel do with the RAM, is to switch from the temporary stack, define in the linker file, to the final stack, define from the static config file and RAM information from the platform layer.

## How it works

For now, the machine RAM is only used for the kernel stack, there's no heap allocation or something else.
The RAM is define using this structure:

```rust
pub struct Memory {
    pub mem_start: usize,
    pub mem_end: usize,
}
```

## Kernel stack

The kernel stack start at the end of the RAM, and grow downward, so towards the start of the RAM.

```
High addresses
+---------------------+  <- end of RAM
|                     |
|   Kernel stack      |  <- SP initial
|   (grows downward)  |
|                     |
+---------------------+
|                     |
|   Free / heap       |
|                     |
+---------------------+
|                     |
|       Padding       |
|    (if there is)    |
|                     |
+---------------------+
|                     |
|   Kernel data       |
|   (.bss, .data)     |
|                     |
+---------------------+
|                     |
|   Kernel text       |
|   (.text, rodata)   |
|                     |
+---------------------+  <- start of RAM
Low addresses
```
