use crate::drivers::{
    cpufreq::CPUFREQ,
    timer::clint0::{CLINT_DEVICE, set_mtimecmp_delta},
};
pub mod tick;
pub mod uptime;

pub fn ktime_seconds() -> u64 {
    #[allow(static_mut_refs)]
    let cpu_freq = unsafe { CPUFREQ.frequency };
    #[allow(static_mut_refs)]
    let mtime = unsafe { CLINT_DEVICE.read_mtime() };
    mtime / cpu_freq as u64
}

pub fn ktime_ms() -> u64 {
    #[allow(static_mut_refs)]
    let cpu_freq = unsafe { CPUFREQ.frequency };
    #[allow(static_mut_refs)]
    let mtime = unsafe { CLINT_DEVICE.read_mtime() };
    (mtime * 1000) / cpu_freq as u64
}

pub fn ktime_ns() -> u64 {
    #[allow(static_mut_refs)]
    let cpu_freq = unsafe { CPUFREQ.frequency };
    #[allow(static_mut_refs)]
    let mtime = unsafe { CLINT_DEVICE.read_mtime() };
    (mtime * 1_000_000) / cpu_freq as u64
}

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
