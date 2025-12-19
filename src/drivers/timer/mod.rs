use core::cell::UnsafeCell;

use clint0::Clint0;

use crate::config::TIMER_MAX_SIZE;

pub mod clint0;

pub trait Timer {
    fn read_time(&self) -> u64;
    fn set_delay(&self, core: usize, delay: u64);
    fn timer_type(&self) -> TimerType;
}

// Enum to define different type for Timer. This is used after the sub-system fill the timer_pool.
// All timers driver use this enum to tell what type the timer is, and the sub-system use it to
// select specific driver for specific task.
#[derive(Copy, Clone, PartialEq)]
pub enum TimerType {
    ArchitecturalTimer,
    SoCTimer,
}

pub struct TimerSubSystem {
    // Timer for scheduling and global work on the kernel
    pub primary_timer: UnsafeCell<Option<*mut dyn Timer>>,
    // Timer pool where all timer initialized is store, waiting to be assigned at another field
    pub timer_pool: UnsafeCell<[Option<*mut dyn Timer>; TIMER_MAX_SIZE]>,
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
    pub fn add_timer(&self, new_timer: &'static mut dyn Timer) {
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
                    (&mut *self.timer_pool.get())[i] = Some(new_timer as *mut dyn Timer);
                }
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

    pub fn get_timer(&self, index: usize) -> Option<*mut dyn Timer> {
        let timer = unsafe { (&*self.timer_pool.get())[index] };
        if let Some(ptr) = timer {
            // ptr was created from a &'static mut dyn Timer in add_timer,
            // converting the raw pointer back to a &'static mut is safe.
            unsafe { Some(&mut *ptr) }
        } else {
            None
        }
    }

    pub fn select_primary_timer(&self) {
        for i in 0..TIMER_MAX_SIZE {
            let timer_ptr = self.get_timer(i);
            if let Some(timer) = timer_ptr {
                let timer_ref: &mut dyn Timer = unsafe { &mut *timer };
                if timer_ref.timer_type() == TimerType::ArchitecturalTimer {
                    // Update the sub-system primary timer
                    unsafe {
                        *self.primary_timer.get() = timer_ptr;
                    }
                    // Remove timer in pool to avoid duplication
                    self.remove_timer(i);
                }
            } else {
                continue;
            }
        }
    }

    pub fn get_primary_timer(&self) -> &dyn Timer {
        let timer_ptr = unsafe { *self.primary_timer.get() };
        if let Some(timer) = timer_ptr {
            let timer_ref: &dyn Timer = unsafe { &mut *timer };
            timer_ref
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
    Clint0::init();
    TIMER_SUBSYSTEM.select_primary_timer();
}
