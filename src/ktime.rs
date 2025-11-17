use crate::devices::{cpufreq::CPUFREQ, timer::clint0::CLINT_DEVICE};

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

pub fn set_ktime_ms(delay: u64) {
    #[allow(static_mut_refs)]
    let mtime = unsafe { CLINT_DEVICE.read_mtime() };
    let delta_mtime = mtime + delay;
    #[allow(static_mut_refs)]
    unsafe {
        CLINT_DEVICE.set_mtimecmp(0, delta_mtime)
    };
}
