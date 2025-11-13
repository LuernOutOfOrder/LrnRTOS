use core::ptr;

use crate::{
    dtb::{
        FdtNode,
        helpers::{
            get_all_fdt_nodes, get_fdt_node, get_node_name, get_node_prop,
            get_node_prop_in_hierarchy,
        },
    },
    kprint,
};

pub struct CpuIntc {
    hart_id: u32,
    phandle: u32,
}

impl CpuIntc {
    pub fn init(node: &FdtNode) {
        let parent_node = get_fdt_node(
            node.parent_node_index
                .expect("ERROR: riscv,cpu-intc has no parent node in fdt"),
        );
        let reg = get_node_prop(&parent_node, "reg")
            .expect("ERROR: riscv,cpu-intc parent has no reg property in fdt");
        let reg_value = u32::from_be(unsafe { ptr::read(reg.off_value as *const u32) });
        let cpu_intc: CpuIntc = CpuIntc {
            hart_id: reg_value,
            phandle: ,
        };
    }
}
