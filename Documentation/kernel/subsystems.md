# LrnRTOS sub-systems

<!--toc:start-->
- [LrnRTOS sub-systems](#lrnrtos-sub-systems)
  - [Description](#description)
  - [Sub-systems initialization](#sub-systems-initialization)
  - [Purpose](#purpose)
  - [How it work](#how-it-work)
  - [How they work together](#how-they-work-together)
  - [Invariants](#invariants)
<!--toc:end-->

## Description

A sub-systems inside the kernel is an HAL(Hardware Abstraction Layer). 

## Sub-systems initialization

They are the first system to be initialized after parsing the FDT. First the system is initialized with empty devices, and after that, all drivers when initialized auto-registers themselves in the correct sub-systems.

## Purpose

There's a sub-system for each specific purpose, like: Timer, Serial, etc. The goal is that each sub-system provide an easy-to-use HAL that is lightweight, and fast.

## How it work

Because a sub-system is a hardware abstraction, we don't want a sub-system that only work for a specific devices, or CPU arch. So we use Rust's traits. 
For each sub-system we define a trait that will be implemented for all this sub-system drivers. Example:

```rust
// Trait to implement in each timer driver
pub trait Timer {
    fn read_time(&self) -> u64;
    fn set_delay(&self, core: usize, delay: u64);
    fn timer_type(&self) -> TimerType;
}

// Tagged union
enum TimerDeviceDriver {
    // define a variant Clint0 which use the Clint0 structure
    Clint0(Clint0)
}

// Define a timer device
// driver: enum of all drivers with the structure associate.
pub struct TimerDevice {
    driver: TimerDeviceDriver
}

// The timer sub-system, use the TimerDevice structure.
pub struct TimerSubSystem {
    pub primary_timer: UnsafeCell<Option<TimerDevice>>,
    pub timer_pool: UnsafeCell<[Option<TimerDevice>; TIMER_MAX_SIZE]>,
}

```

By using trait, when we are writing new timer drivers, we just need to implement the trait and add a new variant with corresponding structure in the tagged union. 
The driver will auto-register itself at the end of the driver init function and that's it, we don't need to modify the sub-system each time we add new drivers. Except for the TimerDevice implementation it's those functions that will make a match on the tagged union and redirect to correct driver.

## How they work together

If some sub-system need to work together, they just call sub-system functions, like the timer sub-system need the cpu-intc sub-system, so it call the cpu-intc sub-system functions.

## Invariants

Once all sub-systems are initialized; the kernel assumes that all sub-systems are correct and they will not change for the lifetime of the system.

All sub-systems initialization assumed that there'll be at least one device per sub-system. If a sub-system is empty, the kernel won't continue the boot process.

After initialization, all sub-system pools are assumed to have sufficient and fixed capacity; any exhaustion or overflow indicates a violation of kernel assumptions and results in a panic.
