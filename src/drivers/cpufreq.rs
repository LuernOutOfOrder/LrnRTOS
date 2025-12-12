use crate::{
    misc::RawTraitObject,
    platform::{CpuFreqDevice, DeviceType, devices_get_info},
};

// Struct to handle the CPU frequency
pub struct CpuFreq {
    pub frequency: u32,
}

impl CpuFreq {
    pub fn init() {
        let device = match devices_get_info("", DeviceType::CpuFreq) {
            Some(d) => d,
            None => panic!("ERROR: Failed to get CPU frequency"),
        };
        let device_info_trait = device.info.unwrap();
        let raw: RawTraitObject = unsafe { core::mem::transmute(device_info_trait) };
        let cpu_freq_device_ptr = raw.data as *const CpuFreqDevice;
        let cpu_freq_device_ref = unsafe { &*cpu_freq_device_ptr };
        let cpu_freq: CpuFreq = CpuFreq {
            frequency: cpu_freq_device_ref.freq,
        };
        unsafe { CPUFREQ = cpu_freq };
    }
}

pub static mut CPUFREQ: CpuFreq = CpuFreq { frequency: 0 };
