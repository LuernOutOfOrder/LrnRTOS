/*
File info: Timer sub-system.

Test coverage: All basic implementation and some edge case.

Tested:
- Basic methods from implementation
- Adding same devices
- Overflow in the sub-system pools.
- Selecting the primary timer.

Not tested:
- ...

Reasons:
- ...

Tests files:
- 'src/tests/drivers/timer/subsystem.rs'
*/

use core::cell::UnsafeCell;

use clint0::Clint0;

use crate::{config::TIMER_MAX_SIZE, log, logs::LogLevel};

pub mod clint0;

pub trait Timer {
    fn read_time(&self) -> u64;
    fn set_delay(&self, core: usize, delay: u64);
}

// Enum to define different type for Timer. This is used after the sub-system fill the timer_pool.
// All timers driver use this enum to tell what type the timer is, and the sub-system use it to
// select specific driver for specific task.
#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum TimerType {
    ArchitecturalTimer,
    SoCTimer,
}

#[derive(Copy, Clone, PartialEq)]
pub enum TimerDeviceDriver {
    Clint0(Clint0),
}

#[derive(Copy, Clone, PartialEq)]
pub struct TimerDevice {
    pub(crate) device: TimerDeviceDriver,
    pub timer_type: TimerType,
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

pub struct TimerSubSystem {
    // Timer pool where all timer initialized is store, waiting to be assigned at another field
    pub timer_pool: [UnsafeCell<Option<TimerDevice>>; TIMER_MAX_SIZE],
    // Timer for scheduling and global work on the kernel
    pub primary_timer: UnsafeCell<Option<TimerDevice>>,
}

unsafe impl Sync for TimerSubSystem {}

impl TimerSubSystem {
    pub const fn init() -> Self {
        TimerSubSystem {
            primary_timer: UnsafeCell::new(None),
            timer_pool: [const { UnsafeCell::new(None) }; TIMER_MAX_SIZE],
        }
    }

    /// Add a new driver for timer in the pool sub-system.
    ///
    /// Params:
    /// &self: the sub-system structure.
    /// new_timer: structure of a timer driver.
    pub fn add_timer(&self, new_timer: TimerDevice) {
        let size = self.get_timer_array_size();
        if size == TIMER_MAX_SIZE {
            log!(
                LogLevel::Warn,
                "Timer sub-system: subsystem is full, ignoring registration request"
            );
            return;
        }
        for i in 0..TIMER_MAX_SIZE {
            let device = unsafe { &*self.timer_pool[i].get() };
            if let Some(timer) = device {
                // Check duplication
                if *timer == new_timer {
                    log!(
                        LogLevel::Warn,
                        "Timer sub-system: duplicate device detected, ignoring registration request"
                    );
                    return;
                }
            } else {
                unsafe {
                    *self.timer_pool[i].get() = Some(new_timer);
                }
                break;
            }
        }
    }

    fn remove_timer(&self, index: usize) {
        unsafe { *self.timer_pool[index].get() = None }
    }

    pub fn get_timer_array_size(&self) -> usize {
        let mut size: usize = 0;
        for i in 0..TIMER_MAX_SIZE {
            let present = unsafe { &*self.timer_pool[i].get() };
            if present.is_some() {
                size += 1;
            }
        }
        size
    }

    fn get_timer(&self, index: usize) -> Option<&TimerDevice> {
        let timer = unsafe { &*self.timer_pool[index].get() };
        if let Some(t) = timer { Some(t) } else { None }
    }

    pub fn select_primary_timer(&self) {
        for i in 0..TIMER_MAX_SIZE {
            let get_timer = self.get_timer(i);
            if let Some(timer) = get_timer {
                if timer.timer_type() == TimerType::ArchitecturalTimer {
                    // Update the sub-system primary timer
                    unsafe {
                        *self.primary_timer.get() = Some(*timer);
                    }
                    // Remove timer in pool to avoid duplication
                    self.remove_timer(i);
                }
            } else {
                continue;
            }
        }
    }

    pub fn get_primary_timer(&self) -> TimerDevice {
        let primary_timer = unsafe { *self.primary_timer.get() };
        if let Some(timer) = primary_timer {
            timer
        } else {
            panic!("Error getting the primary timer in the timer sub-system");
        }
    }
}

// Init static timer sub-system
pub static TIMER_SUBSYSTEM: TimerSubSystem = TimerSubSystem::init();

pub fn init_timer_subsystem() {
    Clint0::init();
    TIMER_SUBSYSTEM.select_primary_timer();
}
