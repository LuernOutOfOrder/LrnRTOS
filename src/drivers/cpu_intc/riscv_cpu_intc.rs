use crate::{
    misc::RawTraitObject,
    platform::{DeviceType, PlatformCpuIntCDevice, platform_get_device_info},
};

use super::{CPU_INTC_SUBSYSTEM, CpuIntcDriver, CpuIntcHw};

pub struct RiscVCpuIntc {
    pub hart_id: u32,
}

impl RiscVCpuIntc {
    pub fn init() {
        let device_info = match platform_get_device_info("riscv,cpu-intc", DeviceType::CpuIntC) {
            Some(d) => d,
            None => return,
        };
        // Allow the use of expect, once we got the device asked, the trait should be working and
        // we should get the trait behind the Option<>
        #[allow(clippy:expect_used)]
        let device_info_trait = device_info
            .info
            .expect("Error: failed to get device trait behind option.");
        let raw: RawTraitObject = unsafe { core::mem::transmute(device_info_trait) };
        let cpu_intc_device_ptr = raw.data as *const PlatformCpuIntCDevice;
        let cpu_intc_device_ref = unsafe { &*cpu_intc_device_ptr };
        let cpu_intc_pool: RiscVCpuIntc = RiscVCpuIntc {
            hart_id: cpu_intc_device_ref.core_id,
        };
        let cpu_intc: CpuIntcHw = CpuIntcHw {
            driver: CpuIntcDriver::RiscVCpuIntc(cpu_intc_pool),
        };
        let cpu_core_id = &cpu_intc.get_cpu_intc_core_id();
        CPU_INTC_SUBSYSTEM.add_cpu_intc(cpu_intc, *cpu_core_id as usize);
    }
}
