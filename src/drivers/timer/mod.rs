use core::cell::UnsafeCell;

use clint0::Clint0;

use crate::config::TIMER_MAX_SIZE;

pub mod clint0;

pub trait Timer {
    fn read_time(&self) -> u64;
    fn set_delay(&self, core: usize, delay: u64);
}

// Enum to define different type for Timer. This is used after the sub-system fill the timer_pool.
// All timers driver use this enum to tell what type the timer is, and the sub-system use it to
// select specific driver for specific task.
#[derive(Copy, Clone, PartialEq)]
pub enum TimerType {
    ArchitecturalTimer,
    SoCTimer,
}

#[derive(Copy, Clone)]
enum TimerDeviceDriver {
    Clint0(Clint0),
}

#[derive(Copy, Clone)]
pub struct TimerDevice {
    timer_type: TimerType,
    device: TimerDeviceDriver,
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
    // Timer for scheduling and global work on the kernel
    pub primary_timer: UnsafeCell<Option<TimerDevice>>,
    // Timer pool where all timer initialized is store, waiting to be assigned at another field
    pub timer_pool: UnsafeCell<[Option<TimerDevice>; TIMER_MAX_SIZE]>,
}

unsafe impl Sync for TimerSubSystem {}

impl TimerSubSystem {
    pub const fn init() -> Self {
        TimerSubSystem {
            primary_timer: UnsafeCell::new(None),
            timer_pool: UnsafeCell::new([const { None }; TIMER_MAX_SIZE]),
        }
    }

    /// Add a new driver for timer in the pool sub-system.
    ///
    /// Params:
    /// &self: the sub-system structure.
    /// new_timer: static structure of a driver implementing the Timer trait.
    pub fn add_timer(&self, new_timer: TimerDevice) {
        let size = self.get_timer_array_size();
        if size == TIMER_MAX_SIZE {
            panic!(
                "Timer sub-system pool possible overflow. Consider increase size in config file."
            )
        }
        for i in 0..TIMER_MAX_SIZE {
            let timer = unsafe { (&*self.timer_pool.get())[i].as_ref() };
            if timer.is_none() {
                unsafe {
                    (&mut *self.timer_pool.get())[i] = Some(new_timer);
                }
                break;
            }
        }
    }

    pub fn remove_timer(&self, index: usize) {
        unsafe { (&mut *self.timer_pool.get())[index] = None }
    }

    pub fn get_timer_array_size(&self) -> usize {
        let mut size: usize = 0;
        for i in 0..TIMER_MAX_SIZE {
            let present = unsafe { (&*self.timer_pool.get())[i].is_some() };
            if present {
                size += 1;
            }
        }
        size
    }

    pub fn get_timer(&self, index: usize) -> Option<&TimerDevice> {
        let timer = unsafe { &(*self.timer_pool.get())[index] };
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

/// Initialize the timer sub-system with all drivers available.
/// Call all timer driver init function, if the fn find a compatible node in the fdt, continue
/// the init and auto register itself in the sub-system. Else, if the init function doesn't find a
/// compatible node, it return to give the next driver init function the turn.
/// Panic if after all drivers init the sub-system pool is empty.
pub fn init_timer_subsystem() {
    let clint = Clint0::init();
    if let Some(c) = clint {
        TIMER_SUBSYSTEM.add_timer(c);
    }
    TIMER_SUBSYSTEM.select_primary_timer();
}
