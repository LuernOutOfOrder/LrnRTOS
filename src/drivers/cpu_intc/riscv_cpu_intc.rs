use crate::{
    misc::RawTraitObject,
    platform::{PlatformCpuIntCDevice, DeviceType, platform_get_device_info},
};

use super::{CpuIntc, CpuIntcDriver, CpuIntcHw};

#[derive(Clone, Copy)]
pub struct RiscVCpuIntc {
    pub hart_id: u32,
}

impl CpuIntc for RiscVCpuIntc {}

impl RiscVCpuIntc {
    pub fn init() -> Option<CpuIntcHw> {
        let device_info = platform_get_device_info("riscv,cpu-intc", DeviceType::CpuIntC)?;
        let device_info_trait = device_info.info.unwrap();
        let raw: RawTraitObject = unsafe { core::mem::transmute(device_info_trait) };
        let cpu_intc_device_ptr = raw.data as *const PlatformCpuIntCDevice;
        let cpu_intc_device_ref = unsafe { &*cpu_intc_device_ptr };
        let cpu_intc_pool: RiscVCpuIntc = RiscVCpuIntc {
            hart_id: cpu_intc_device_ref.core_id,
        };
        let cpu_intc: CpuIntcHw = CpuIntcHw {
            driver: CpuIntcDriver::RiscVCpuIntc(cpu_intc_pool),
        };
        Some(cpu_intc)
    }
}
