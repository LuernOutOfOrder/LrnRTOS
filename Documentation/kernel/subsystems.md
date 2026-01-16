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

They are the first system to be initialized after initializing the platform layer. First the system is initialized with empty devices, and after that, all drivers when initialized auto-registers themselves in the correct sub-systems.

## Purpose

There's a sub-system for each specific purpose, like: Timer, Serial, etc. The goal is that each sub-system provide an easy-to-use HAL that is lightweight, and fast.

## How it work

Because a sub-system is a hardware abstraction, we don't want a sub-system that only work for a specific devices, or CPU arch. So we use an enum to define all drivers for a sub-system.
Before we were using Trait, problem is, it needed the driver to auto-initialized themselves using static, structure were using `mut dyn Trait`, there was fat pointer everywhere, it was not the best design for an RTOS.
So we switch to tagged union, using enum. There's less dynamism on driver initializing and development, but there are fewer likely bugs, a more explicit design, and more robustness using tagged.

```rust
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

impl TimerDevice {
    fn timer_type(&self) -> TimerType {
        self.timer_type
    }

    pub fn read_time(&self) -> u64 {
        match &self.device {
            TimerDeviceDriver::Clint0(clint0) => clint0.read_mtime(),
        }
    }

    pub fn set_delay(&self, core: usize, delay: u64) {
        match &self.device {
            TimerDeviceDriver::Clint0(clint0) => clint0.set_delay(core, delay),
        }
    }
}

// The timer sub-system, use the TimerDevice structure.
pub struct TimerSubSystem {
    pub primary_timer: UnsafeCell<Option<TimerDevice>>,
    pub timer_pool: [UnsafeCell<Option<TimerDevice>>; TIMER_MAX_SIZE],
}

```

## How they work together

If some sub-system need to work together, they just call sub-system functions, like the timer sub-system need the cpu-intc sub-system, so it call the cpu-intc sub-system functions.

## Invariants

- Once all sub-systems are initialized; the kernel assumes that all sub-systems are correct and they will not change for the lifetime of the system.
- All sub-systems initialization assumed that there'll be at least one device per sub-system. If a sub-system is empty, the kernel won't continue the boot process.
- After initialization, all sub-system pools are assumed to have sufficient and fixed capacity; any exhaustion or overflow indicates a violation of kernel assumptions and results in a panic.

### Serial sub-system

- The first serial device registered will be considered as the default console.
