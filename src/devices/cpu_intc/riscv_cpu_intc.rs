use core::ptr;

use crate::dtb::{
    FdtNode,
    helpers::{get_fdt_node, get_node_prop},
};

#[derive(Clone, Copy, Debug)]
pub struct CpuIntc {
    #[allow(unused)]
    hart_id: u32,
}

pub static mut CPU_INTC_POOL: [CpuIntc; 4] = [CpuIntc { hart_id: 0 }; 4];

impl CpuIntc {
    pub fn init(node: &FdtNode) {
        let parent_node = get_fdt_node(
            node.parent_node_index
                .expect("ERROR: riscv,cpu-intc has no parent node in fdt"),
        );
        let reg = get_node_prop(&parent_node, "reg")
            .expect("ERROR: riscv,cpu-intc parent has no reg property in fdt");
        let reg_value = u32::from_be(unsafe { ptr::read(reg.off_value as *const u32) });
        let cpu_intc: CpuIntc = CpuIntc { hart_id: reg_value };
        // Update static array of initialized cpu_intc drivers
        #[allow(static_mut_refs)]
        unsafe {
            CPU_INTC_POOL[reg_value as usize] = cpu_intc;
        };
    }
}
