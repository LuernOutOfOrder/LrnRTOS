use core::mem;

use crate::{devices_info::MEM, drivers::DriverRegion};

use super::{
    PLATFORM_INFO,
    fdt::{FdtNode, helpers::fdt_get_node_by_device_type},
};

pub struct MemoryProvider {
    pub reg: DriverRegion,
}

impl MemoryProvider {
    pub const fn init_default() -> Self {
        unsafe { mem::zeroed() }
    }

    pub fn init_fdt() -> Self {
        let mut mem: MemoryProvider = MemoryProvider {
            reg: DriverRegion { addr: 0, size: 0 },
        };
        {
            let node: &FdtNode = match fdt_get_node_by_device_type("memory") {
                Some(n) => n,
                None => {
                    panic!("Error while creating new MemoryProvider structure")
                }
            };
            let mem_reg = DriverRegion::new(node);
            mem.reg = mem_reg;
        }
        mem
    }
}

pub fn platform_init_mem() -> MemoryProvider {
    #[allow(static_mut_refs)]
    let mode = unsafe { PLATFORM_INFO.read_mode() };
    let mut mem: MemoryProvider = MemoryProvider::init_default();
    if mode {
        mem = MemoryProvider::init_fdt();
    } else {
        mem.reg.addr = MEM.reg.addr;
        mem.reg.size = MEM.reg.size;
    }
    mem
}
