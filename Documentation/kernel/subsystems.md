# LrnRTOS sub-systems

## Description

A sub-systems inside the kernel is a HAL(Hardware Abstraction Layer). 

## Sub-systems initializations

They are the first system to be initialized after parsing the fdt. First the system is initialized with empty devices, and after that, all drivers when initialized auto-registers themselves in the correct sub-systems.

## Purpose

There's a sub-system for each specific purpose, like: Timer, Serial, etc. The goal is that each sub-system provide a easy-to-use HAL that is lightweight, and fast.

## How it work

Because a sub-system is an hardware abstraction, we don't want a sub-system that only work for a specific devices, or CPU arch. So we use Rust's traits. 
For each sub-system we define a trait that will be implemented for all this sub-system drivers. Exemple:

```rust
// Trait to implement in each timer driver
pub trait Timer {
    fn read_time(&self) -> u64;
    fn set_delay(&self, core: usize, delay: u64);
    fn timer_type(&self) -> TimerType;
}

// The timer sub-system, use a *mut dyn Timer to point to a mutable struct implementing the trait.
pub struct TimerSubSystem {
    pub primary_timer: UnsafeCell<Option<*mut dyn Timer>>,
    pub timer_pool: UnsafeCell<[Option<*mut dyn Timer>; TIMER_MAX_SIZE]>,
}

```

By using trait, when we are writing new timer drivers, we just need to implement the trait, and add the auto-register driver at the end of the driver init function and that's it, we don't need to modify the sub-system each time we add new drivers.

## How they work together

If some sub-system need to work together, they just call sub-system functions, like the timer sub-system need the cpu-intc sub-system, so it call the cpu-intc sub-system functions.

## Invariants

Once all sub-systems are initialized; the kernel assumes that all sub-systems are correct and they will not change for the lifetime of the system.

After initialization, all sub-system pools are assumed to have sufficient and fixed capacity; any exhaustion or overflow indicates a violation of kernel assumptions and results in a panic.
