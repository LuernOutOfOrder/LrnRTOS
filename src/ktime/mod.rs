/*
File info: Kernel timer abstraction layer. Use the timer sub-system to set delay or get the time from primary timer.

Test coverage: Only the set_ktime functions.

Tested:
- set_ktime_ms
- set_ktime_ns
- set_ktime_seconds
  
Not tested:
- Read ktime and set_mtimecmp_delta
  
Reasons:
- Hard to test those functions, even from a controlled environment like Qemu.

Tests files:
- 'src/tests/ktime/mod.rs'
*/

use crate::drivers::cpufreq::CPUFREQ;
use crate::drivers::timer::TIMER_SUBSYSTEM;
pub mod tick;
pub mod uptime;

// ———— Read ktime in specific time units ————

pub fn ktime_seconds() -> u64 {
    #[allow(static_mut_refs)]
    let cpu_freq = unsafe { CPUFREQ.frequency };
    #[allow(static_mut_refs)]
    let mtime = TIMER_SUBSYSTEM.get_primary_timer().read_time();
    mtime / cpu_freq as u64
}

pub fn ktime_ms() -> u64 {
    #[allow(static_mut_refs)]
    let cpu_freq = unsafe { CPUFREQ.frequency };
    #[allow(static_mut_refs)]
    let mtime = TIMER_SUBSYSTEM.get_primary_timer().read_time();
    (mtime * 1000) / cpu_freq as u64
}

pub fn ktime_ns() -> u64 {
    #[allow(static_mut_refs)]
    let cpu_freq = unsafe { CPUFREQ.frequency };
    #[allow(static_mut_refs)]
    let mtime = TIMER_SUBSYSTEM.get_primary_timer().read_time();
    (mtime * 1_000_000) / cpu_freq as u64
}

// Set ktime in specific time units, handle conversion from params duration to correct time unit
// and use set_mtime_cmp to write to timer sub-system

pub fn set_ktime_ms(duration_ms: u64) {
    #[allow(static_mut_refs)]
    let cpu_freq = unsafe { CPUFREQ.frequency };
    let delta_ticks = cpu_freq as u64 * duration_ms / 1000;
    set_mtimecmp_delta(delta_ticks);
}

pub fn set_ktime_ns(duration_ns: u64) {
    #[allow(static_mut_refs)]
    let cpu_freq = unsafe { CPUFREQ.frequency };
    let delta_ticks = cpu_freq as u64 * duration_ns / 1_000_000;
    set_mtimecmp_delta(delta_ticks);
}

pub fn set_ktime_seconds(duration: u64) {
    #[allow(static_mut_refs)]
    let cpu_freq = unsafe { CPUFREQ.frequency };
    let delta_ticks = cpu_freq as u64 * duration;
    set_mtimecmp_delta(delta_ticks);
}

pub fn set_mtimecmp_delta(delay: u64) {
    #[allow(static_mut_refs)]
    let mtime = TIMER_SUBSYSTEM.get_primary_timer().read_time();
    let delta_mtime = mtime + delay;
    #[allow(static_mut_refs)]
    TIMER_SUBSYSTEM
        .get_primary_timer()
        .set_delay(0, delta_mtime);
}
