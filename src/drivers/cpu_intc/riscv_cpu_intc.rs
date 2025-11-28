use core::ptr;

use crate::fdt::{
    FdtNode,
    helpers::{fdt_get_node, fdt_get_node_by_compatible, fdt_get_node_prop},
};

use super::{CPU_INTC_SUBSYSTEM, CpuIntc};

#[derive(Clone, Copy, Debug)]
pub struct RiscVCpuIntc {
    #[allow(unused)]
    hart_id: u32,
}

impl CpuIntc for RiscVCpuIntc {}

static mut RISCV_CPU_INTC_INSTANCE: RiscVCpuIntc = RiscVCpuIntc { hart_id: 0 };

impl RiscVCpuIntc {
    pub fn init() {
        let node: &FdtNode = match fdt_get_node_by_compatible("riscv,cpu-intc") {
            Some(n) => n,
            None => return,
        };
        let parent_node = fdt_get_node(
            node.parent_node_index
                .expect("ERROR: riscv,cpu-intc has no parent node in fdt"),
        );
        let reg = fdt_get_node_prop(&parent_node, "reg")
            .expect("ERROR: riscv,cpu-intc parent has no reg property in fdt");
        let reg_value = u32::from_be(unsafe { ptr::read(reg.off_value as *const u32) });
        let cpu_intc_pool: RiscVCpuIntc = RiscVCpuIntc { hart_id: reg_value };
        unsafe { RISCV_CPU_INTC_INSTANCE = cpu_intc_pool };
        // Update cpu-intc sub-system pool with new driver
        CPU_INTC_SUBSYSTEM.add_cpu_intc(unsafe { &mut RISCV_CPU_INTC_INSTANCE });
    }
}
