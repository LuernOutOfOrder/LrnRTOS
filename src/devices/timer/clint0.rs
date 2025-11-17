use core::ptr::{self, null_mut};

use arrayvec::ArrayVec;

use crate::{
    devices::{
        DriverRegion,
        cpu_intc::riscv_cpu_intc::{CPU_INTC_POOL, CpuIntc},
    },
    dtb::{
        FdtNode,
        helpers::{get_fdt_node, get_node_by_phandle, get_node_prop, get_node_prop_in_hierarchy},
    },
};

/// Structure for sifive clint device driver
#[derive(Copy, Clone)]
pub struct Clint0 {
    region: DriverRegion,
    #[allow(unused)]
    interrupt_extended: [Interrupt; 4],
}

#[derive(Copy, Clone)]
pub struct Interrupt {
    // Ptr to CpuIntc struct
    cpu_intc: *mut CpuIntc,
    // Field to follow the len of the irq_ids array to avoid crushing valid data
    irq_len: usize,
    // Array of all irq
    irq_ids: [u32; 4],
}

pub static mut CLINT_DEVICE: Clint0 = Clint0 {
    region: DriverRegion { addr: 0, size: 0 },
    interrupt_extended: [Interrupt {
        cpu_intc: null_mut(),
        irq_len: 0,
        irq_ids: [0u32; 4],
    }; 4],
};

impl Clint0 {
    pub fn init(node: &FdtNode) {
        // Get address and size cells
        let address_cells = get_node_prop_in_hierarchy(node, "#address-cells")
            .expect("ERROR: clint0 node is missing '#address-cells' property from parent bus\n");
        let size_cells = get_node_prop_in_hierarchy(node, "#size-cells")
            .expect("ERROR: clint0 node is missing '#size-cells' property from parent bus\n");
        // Ptr read address and size cells value from off and cast it to u32 target's endianness
        let address_cells_val: u32 =
            u32::from_be(unsafe { ptr::read(address_cells.off_value as *const u32) });
        let size_cells_val: u32 =
            u32::from_be(unsafe { ptr::read(size_cells.off_value as *const u32) });
        // Get device memory region
        let reg = get_node_prop(node, "reg").expect("ERROR: clint0 node is missing 'reg' property");
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
        let interrupt: Interrupt = Interrupt {
            cpu_intc: null_mut(),
            irq_len: 0,
            irq_ids: [0u32; 4],
        };
        let mut intc_extended_array: [Interrupt; 4] = [interrupt; 4];
        let interrupt_extended = get_node_prop(node, "interrupts-extended")
            .expect("ERROR: clint0 node is missing 'interrupts-extended' property\n");
        // First parsing through interrupts-extended to build complete array with values from
        // interrupts-extended property in fdt
        let mut interrupt_extended_cursor: usize;
        let mut interrupts_extended_vec: ArrayVec<u32, 16> = ArrayVec::new();
        {
            interrupt_extended_cursor = interrupt_extended.off_value;
            for _ in 0..interrupt_extended.value_len / 4 {
                let value =
                    u32::from_be(unsafe { ptr::read(interrupt_extended_cursor as *const u32) });
                interrupts_extended_vec.push(value);
                interrupt_extended_cursor += 4;
            }
        }
        interrupt_extended_cursor = interrupt_extended.off_value;
        let mut iter_safety: usize = 0;
        // Second parsing through interrupts-extended to associate correct irqs with hart id
        for mut i in 0..interrupts_extended_vec.len() {
            let value = u32::from_be(unsafe { ptr::read(interrupt_extended_cursor as *const u32) });
            // Get node from interrupt-extended value
            if iter_safety == interrupts_extended_vec.len() {
                break;
            }
            let node = get_node_by_phandle(value).expect(
                "ERROR: cannot find associate phandle node from clint0 interrupts-extended property",
            );
            let node_interrupt_cells = get_node_prop(&node, "#interrupt-cells")
                .expect("ERROR: clint0 phandle node is missing the property '#interrupt-cells'");
            // Read node interrupt-cells value to know how many clint interrupt-extended value to
            // read and assign to phandle
            let cpu_node = get_fdt_node(node.parent_node_index.unwrap());
            let cpu_reg = get_node_prop(&cpu_node, "reg")
                .expect("ERROR: failed to get core id from associated core from intc");
            let cpu_reg_value = u32::from_be(unsafe { ptr::read(cpu_reg.off_value as *const u32) });
            let node_interrupt_cells_value =
                u32::from_be(unsafe { ptr::read(node_interrupt_cells.off_value as *const u32) });
            let mut cpu_intc_driver = unsafe { CPU_INTC_POOL[cpu_reg_value as usize] };
            let mut parsed_interrupt: Interrupt = Interrupt {
                cpu_intc: &mut cpu_intc_driver,
                irq_len: 0,
                irq_ids: [0u32; 4],
            };
            // // Check if an interrupt for this phandle already exist
            #[allow(clippy::needless_range_loop)]
            for e in 0..intc_extended_array.len() {
                if intc_extended_array[e].cpu_intc != &mut cpu_intc_driver {
                    continue;
                } else {
                    // Update current parsed interrupt with existing one
                    parsed_interrupt = intc_extended_array[e];
                    // Update i iterator to be the same index as e to retrieve it in
                    // 'intc_extended_array'
                    i = e;
                }
            }
            // Push irqs inside 'irq_ids' array of current 'parsed_interrupt'
            for _ in 0..node_interrupt_cells_value {
                interrupt_extended_cursor += 4;
                iter_safety += 1;
                let irq_value =
                    u32::from_be(unsafe { ptr::read(interrupt_extended_cursor as *const u32) });
                parsed_interrupt.irq_ids[parsed_interrupt.irq_len] = irq_value;
                parsed_interrupt.irq_len += 1;
            }
            // Increment offset
            interrupt_extended_cursor += 4;
            // Increment iterator
            iter_safety += 1;
            // Update array with current interrupt
            intc_extended_array[i] = parsed_interrupt;
        }
        // Init Clint0 driver and update static for global access.
        let clint0: Clint0 = Clint0 {
            region: device_addr,
            interrupt_extended: intc_extended_array,
        };
        unsafe { CLINT_DEVICE = clint0 };
    }

    /// Read mtime from clint0 addr + offset from `https://chromitem-soc.readthedocs.io/en/latest/clint.html`
    /// Check 2 time value from high addr to avoid miscompute mtime and giving wrong tick, and led
    /// to UB.
    pub fn read_mtime(&self) -> u64 {
        // Offset from doc
        // TODO: check if I can make this better than just hardcoded offset ????
        let off = 0xBFF8;
        // Define mtime value
        let mut mtime_low: u32 = 0;
        let mut mtime_high: u32 = 0;
        // Define mtime_high checking value to make the while loop work
        let mut mtime_high_check: u32 = 1;

        // While the two mtime is different continue to read to avoid miss compute mtime and lead
        // to UB.
        while mtime_high != mtime_high_check {
            let mtime_low_addr = self.region.addr + off;
            let mtime_high_addr = self.region.addr + off + 4;
            mtime_high = unsafe { ptr::read(mtime_high_addr as *const u32) };
            mtime_low = unsafe { ptr::read(mtime_low_addr as *const u32) };
            mtime_high_check = unsafe { ptr::read(mtime_high_addr as *const u32) };
        }
        // Bitwise to compute mtime from value. Cannot read u64 directly on riscv 32 bits.
        let output: u64 = ((mtime_high as u64) << 32) | (mtime_low as u64);
        output
    }

    /// Set a timer
    /// hart_id: id of the target hart to send timer interrupt.
    /// next_tick: value to set the timer to, current-time + next_tick
    pub fn set_mtimecmp(&self, hart_id: usize, next_tick: u64) {
        let off = 0x4000 + (hart_id * 8);
        // Value to deactivate temporaly interrupt
        let deactivate_int: u32 = 0xFFFF_FFFF;
        let mtimecmp_low_addr = self.region.addr + off;
        let mtimecmp_high_addr = self.region.addr + off + 4;
        let next_tick_high: u32 = (next_tick >> 32) as u32;
        let next_tick_low: u32 = (next_tick & 0xFFFF_FFFF) as u32;
        unsafe {
            // First write deactivate_int value to temporaly deactivate interrupt from hardware
            ptr::write_volatile(mtimecmp_high_addr as *mut u32, deactivate_int);
            // Second write next_tick_low value to low addr of the set_mtimecmp addr
            ptr::write_volatile(mtimecmp_low_addr as *mut u32, next_tick_low);
            // And finally write the next_tick_high value to high addr of the set_mtimecmp addr
            ptr::write_volatile(mtimecmp_high_addr as *mut u32, next_tick_high);
        }
    }

    /// Send an interrupt to given hart by writting 1 to the clint0 addr + hart_id * 4.
    /// hart_id: id of the target hart to send interrupt.
    pub fn send_ipi(&self, hart_id: usize) {
        let addr = self.region.addr + (hart_id * 4);
        unsafe { ptr::write_volatile(addr as *mut u32, 1) };
    }
}
