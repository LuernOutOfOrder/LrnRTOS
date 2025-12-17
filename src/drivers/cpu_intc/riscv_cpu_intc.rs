use crate::{
    misc::RawTraitObject,
    platform::{CpuIntCDevice, DeviceType, platform_get_device_info},
};

use super::{CPU_INTC_SUBSYSTEM, CpuIntc};

#[derive(Clone, Copy)]
pub struct RiscVCpuIntc {
    #[allow(unused)]
    hart_id: u32,
}

impl CpuIntc for RiscVCpuIntc {}

static mut RISCV_CPU_INTC_INSTANCE: RiscVCpuIntc = RiscVCpuIntc { hart_id: 0 };

impl RiscVCpuIntc {
    pub fn init() {
        let device_info = match platform_get_device_info("riscv,cpu-intc", DeviceType::CpuIntC) {
            Some(d) => d,
            None => return,
        };
        let device_info_trait = device_info.info.unwrap();
        let raw: RawTraitObject = unsafe { core::mem::transmute(device_info_trait) };
        let cpu_intc_device_ptr = raw.data as *const CpuIntCDevice;
        let cpu_intc_device_ref = unsafe { &*cpu_intc_device_ptr };
        let cpu_intc_pool: RiscVCpuIntc = RiscVCpuIntc {
            hart_id: cpu_intc_device_ref.core_id,
        };
        unsafe { RISCV_CPU_INTC_INSTANCE = cpu_intc_pool };
        // Update cpu-intc sub-system pool with new driver
        #[allow(static_mut_refs)]
        CPU_INTC_SUBSYSTEM.add_cpu_intc(
            unsafe { &mut RISCV_CPU_INTC_INSTANCE },
            cpu_intc_device_ref.core_id as usize,
        );
    }
}
