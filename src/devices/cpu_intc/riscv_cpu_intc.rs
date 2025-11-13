use core::ptr;

use crate::{
    dtb::{
        FdtNode,
        helpers::{get_fdt_node, get_node_prop},
    },
    kprint,
};

#[derive(Clone, Copy)]
pub struct EarlyBootCpuIntc {
    phandle: u32,
    cpu_intc_ptr: *mut CpuIntc,
}

#[derive(Clone, Copy)]
pub struct CpuIntc {
    hart_id: u32,
}

pub static mut CPU_INTC_ARRAY: [EarlyBootCpuIntc; 4] = [EarlyBootCpuIntc {
    phandle: 0,
    cpu_intc_ptr: ptr::null_mut(),
}; 4];

pub static mut CPU_INTC_POOL: [CpuIntc; 4] = [CpuIntc { hart_id: 0 }; 4];

impl CpuIntc {
    pub fn init(node: &FdtNode) {
        let phandle = get_node_prop(node, "phandle")
            .expect("ERROR: riscv,cpu_intc has no property 'phandle' in fdt");
        let phandle_value = u32::from_be(unsafe { ptr::read(phandle.off_value as *const u32) });
        let parent_node = get_fdt_node(
            node.parent_node_index
                .expect("ERROR: riscv,cpu-intc has no parent node in fdt"),
        );
        let reg = get_node_prop(&parent_node, "reg")
            .expect("ERROR: riscv,cpu-intc parent has no reg property in fdt");
        let reg_value = u32::from_be(unsafe { ptr::read(reg.off_value as *const u32) });
        let cpu_intc: *mut CpuIntc = &mut CpuIntc { hart_id: reg_value };
        let earlyboot_cpu_intc: EarlyBootCpuIntc = EarlyBootCpuIntc {
            phandle: phandle_value,
            cpu_intc_ptr: cpu_intc,
        };
        // Update static array for cpu_intc driver association with clint
        #[allow(static_mut_refs)]
        for i in 0..unsafe { CPU_INTC_ARRAY.len() } {
            if unsafe { CPU_INTC_ARRAY }[i].cpu_intc_ptr.is_null() {
                unsafe { CPU_INTC_ARRAY[i] = earlyboot_cpu_intc }
            } else {
                continue;
            }
        }
        // Update static array of initialized cpu_intc drivers
        #[allow(static_mut_refs)]
        for i in 0..unsafe { CPU_INTC_POOL.len() } {
            if unsafe { CPU_INTC_POOL }[i].hart_id != (unsafe { *cpu_intc }).hart_id {
                unsafe { CPU_INTC_POOL[i] = *cpu_intc }
            }
        }
    }
}
