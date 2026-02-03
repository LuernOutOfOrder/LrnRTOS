use crate::{
    misc::RawTraitObject,
    platform::{DeviceType, PlatformCpuFreqDevice, platform_get_device_info},
};

// Struct to handle the CPU frequency
pub struct CpuFreq {
    pub frequency: u32,
}

impl CpuFreq {
    pub fn init() {
        let device = match platform_get_device_info("cpu-freq", DeviceType::CpuFreq) {
            Some(d) => d,
            None => panic!("ERROR: Failed to get CPU frequency"),
        };
        // Allow the use of expect, once we got the device asked, the trait should be working and
        // we should get the trait behind the Option<>
        #[allow(clippy::expect_used)]
        let device_info_trait = device
            .info
            .expect("Error: failed to get device trait behind option.");
        let raw: RawTraitObject = unsafe { core::mem::transmute(device_info_trait) };
        let cpu_freq_device_ptr = raw.data as *const PlatformCpuFreqDevice;
        let cpu_freq_device_ref = unsafe { &*cpu_freq_device_ptr };
        let cpu_freq: CpuFreq = CpuFreq {
            frequency: cpu_freq_device_ref.freq,
        };
        unsafe { CPUFREQ = cpu_freq };
    }
}

pub static mut CPUFREQ: CpuFreq = CpuFreq { frequency: 0 };
