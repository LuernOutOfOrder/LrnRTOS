// Static for kernel tick, incremented at each timer interrupt
pub static mut GLOBAL_TICK: usize = 0;
// Static to track global idle time in kernel, increment each time...
pub static mut GLOBAL_IDLE_TIME_TICK: usize = 0;

pub fn increment_tick() {
    unsafe { GLOBAL_TICK += 1 }
}

pub fn get_tick() -> usize {
    unsafe { GLOBAL_TICK }
}

pub fn increment_idle_time_tick() {
    unsafe { GLOBAL_IDLE_TIME_TICK += 1 }
}

pub fn get_idle_time_tick() -> usize {
    unsafe { GLOBAL_IDLE_TIME_TICK }
}
