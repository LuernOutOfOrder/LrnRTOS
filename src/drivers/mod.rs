use core::ptr;

use arrayvec::ArrayVec;
use cpu_intc::init_cpu_intc_subsystem;
use serials::init_serial_subsystem;
use timer::init_timer_subsystem;

use crate::{
    platform::fdt::{
        FdtNode,
        helpers::{fdt_get_node_prop, fdt_get_node_prop_in_hierarchy},
    },
    kprint, log,
    logs::LogLevel,
};

/// Module for serials devices
pub mod serials;

// Module for timer devices
pub mod timer;

// Module for cpu core interrupt-controller
pub mod cpu_intc;

// Module for cpu frequency
pub mod cpufreq;

/// Public structure used to define device region in memory.
/// addr: the address to use in drivers.
/// size: the size of the address.
#[derive(Copy, Clone, Debug)]
pub struct DriverRegion {
    pub addr: usize,
    pub size: usize,
}

impl DriverRegion {
    pub fn new(node: &FdtNode) -> Self {
        // Get address and size cells
        let address_cells = fdt_get_node_prop_in_hierarchy(node, "#address-cells")
            .expect("ERROR: node is missing '#address-cells' property from parent bus\n");
        let size_cells = fdt_get_node_prop_in_hierarchy(node, "#size-cells")
            .expect("ERROR: node is missing '#size-cells' property from parent bus\n");
        // Ptr read address and size cells value from off and cast it to u32 target's endianness
        let address_cells_val: u32 =
            u32::from_be(unsafe { ptr::read(address_cells.off_value as *const u32) });
        let size_cells_val: u32 =
            u32::from_be(unsafe { ptr::read(size_cells.off_value as *const u32) });
        // Get device memory region
        let reg = fdt_get_node_prop(node, "reg").expect("ERROR: node is missing 'reg' property");
        let mut reg_buff: ArrayVec<u32, 120> = ArrayVec::new();
        let mut reg_cursor = reg.off_value;
        // Divide reg.value_len by 4 because we read u32 and not u8
        for _ in 0..reg.value_len / 4 {
            let value = u32::from_be(unsafe { ptr::read(reg_cursor as *const u32) });
            reg_buff.push(value);
            reg_cursor += 4;
        }
        // Region size from #address-cells and #size-cells properties value
        let reg_size = address_cells_val + size_cells_val;
        // Init a new DriverRegion
        let mut device_addr: DriverRegion = DriverRegion { addr: 0, size: 0 };
        for addr in reg_buff.chunks(reg_size as usize) {
            // Build addr from chunk
            let mut device_addr_build: u64 = 0;
            for i in 0..address_cells_val {
                device_addr_build = (device_addr_build << 32) | (addr[i as usize] as u64);
            }
            // Build size from chunk
            let mut device_size: u64 = 0;
            for i in 0..size_cells_val {
                device_size =
                    (device_size << 32) | (addr[address_cells_val as usize + i as usize] as u64);
            }
            device_addr = DriverRegion {
                addr: device_addr_build as usize,
                size: device_size as usize,
            }
        }
        device_addr
    }
}

/// Init all device drivers sub-system, call all init function from device driver structure. The init function
/// will automatically save the new initialized device drivers it's own sub system static array.
pub fn init_devices_subsystems() {
    kprint!("Serial sub-system initializing...\n");
    init_serial_subsystem();
    log!(
        LogLevel::Debug,
        "Serial sub-system successfully initialized."
    );
    log!(
        LogLevel::Debug,
        "Cpu interrupt controller sub-system initializing..."
    );
    init_cpu_intc_subsystem();
    log!(
        LogLevel::Debug,
        "Cpu interrupt-controller sub-system successfully initialized."
    );
    log!(LogLevel::Debug, "Timer sub-system initializing...");
    init_timer_subsystem();
    log!(
        LogLevel::Debug,
        "Timer sub-system successfully initialized."
    );
}
