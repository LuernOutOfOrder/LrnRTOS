use crate::fdt::helpers::{fdt_get_prop_by_node_name, fdt_get_prop_u32_value};

// Struct to handle the cpu frequency
pub struct CpuFreq {
    pub frequency: u32,
}

impl CpuFreq {
    pub fn init() {
        let cpus_freq = fdt_get_prop_by_node_name("cpus", "timebase-frequency");
        if let Some(freq) = cpus_freq {
            let freq_value = fdt_get_prop_u32_value(freq);
            let cpu_freq: CpuFreq = CpuFreq {
                frequency: freq_value,
            };
            unsafe { CPUFREQ = cpu_freq };
        } else {
            panic!("ERROR: Failed to create cpu frequency structure");
        }
    }
}

pub static mut CPUFREQ: CpuFreq = CpuFreq { frequency: 0 };
