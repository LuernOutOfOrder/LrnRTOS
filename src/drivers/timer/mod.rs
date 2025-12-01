use core::cell::UnsafeCell;

use clint0::Clint0;

use crate::config::TIMER_MAX_SIZE;

pub mod clint0;

pub trait Timer {
    fn read_time(&self) -> u64;
    fn set_delay(&self, core: usize, delay: u64);
}

pub struct TimerSubSystem {
    pub timer_pool: UnsafeCell<[Option<*mut dyn Timer>; TIMER_MAX_SIZE]>,
}

unsafe impl Sync for TimerSubSystem {}

impl TimerSubSystem {
    pub const fn init() -> Self {
        TimerSubSystem {
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
    let size = TIMER_SUBSYSTEM.get_timer_array_size();
    if size == 0 {
        panic!("Error while initializing timer sub-system, pool is empty.");
    }
}
