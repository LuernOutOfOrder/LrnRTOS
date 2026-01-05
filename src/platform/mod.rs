pub mod fdt;
pub mod mem;
pub mod platform_info;

use core::ptr;

use arrayvec::ArrayVec;
use platform_info::PlatformInfo;

use crate::{devices_info::DEVICES, drivers::DriverRegion, kprint};
use fdt::{
    FdtNode, fdt_present,
    helpers::{
        fdt_get_node, fdt_get_node_by_compatible, fdt_get_node_by_phandle, fdt_get_node_prop,
        fdt_get_prop_by_node_name, fdt_get_prop_u32_value,
    },
    parse_dtb_file,
};

static mut PLATFORM_INFO: PlatformInfo = PlatformInfo::init();

/// Initialize the FDT and the static devices. Choose the correct one to use.
pub fn platform_init(dtb_addr: usize) {
    if fdt_present(dtb_addr) {
        parse_dtb_file(dtb_addr);
        #[allow(static_mut_refs)]
        unsafe {
            PLATFORM_INFO.set_mode_fdt()
        };
    }
    // Condition with just kprint for debug purpose
    #[allow(static_mut_refs)]
    if unsafe { PLATFORM_INFO.read_mode() } {
        kprint!("Platform mode set to FDT.\n");
    } else {
        kprint!("Platform mode set to STATIC.\n");
    }
}

#[derive(Copy, Clone)]
pub enum DeviceType {
    Serial,
    Timer,
    CpuIntC,
    CpuFreq,
}

pub trait DeviceInfo {}

/// Structure used to define a serial device.
/// Only used in static SERIAL_DEVICES
#[derive(Copy, Clone)]
pub struct DevicesHeader<'a> {
    pub device_type: DeviceType,
    pub compatible: &'a str,
    pub device_addr: DriverRegion,
}

#[derive(Copy, Clone)]
pub struct Devices<'a> {
    pub header: DevicesHeader<'a>,
    pub info: Option<*const dyn DeviceInfo>,
}

impl Devices<'_> {
    pub const fn init() -> Self {
        Devices {
            header: DevicesHeader {
                device_type: DeviceType::Serial,
                compatible: "",
                device_addr: DriverRegion { addr: 0, size: 0 },
            },
            info: None,
        }
    }

    pub fn init_fdt<'a>(compatible: &'a str, device_type: DeviceType) -> Devices<'a> {
        let node: &FdtNode = match fdt_get_node_by_compatible(compatible) {
            Some(n) => n,
            None => {
                panic!("Error while initialize generic device structure");
            }
        };
        let device_addr: DriverRegion = DriverRegion::new(node);
        Devices {
            header: DevicesHeader {
                device_type,
                compatible,
                device_addr,
            },
            info: None,
        }
    }
}

unsafe impl<'a> Sync for Devices<'a> {}

pub struct PlatformSerialDevice {}

impl PlatformSerialDevice {
    pub const fn init() -> Self {
        PlatformSerialDevice {}
    }
}

pub struct PlatformCpuIntCDevice {
    pub core_id: u32,
}

impl PlatformCpuIntCDevice {
    pub const fn init() -> Self {
        PlatformCpuIntCDevice { core_id: 0 }
    }

    pub fn init_fdt(compatible: &'_ str) -> Self {
        let node: &FdtNode = match fdt_get_node_by_compatible(compatible) {
            Some(n) => n,
            None => panic!("Error while creating new CPU interrupt-controller generic structure"),
        };
        let parent_node = fdt_get_node(
            node.parent_node_index
                .expect("ERROR: riscv,cpu-intc has no parent node in fdt"),
        );
        let reg = fdt_get_node_prop(&parent_node, "reg")
            .expect("ERROR: riscv,cpu-intc parent has no reg property in fdt");
        let reg_value = u32::from_be(unsafe { ptr::read(reg.off_value as *const u32) });
        PlatformCpuIntCDevice { core_id: reg_value }
    }
}

pub struct CpuFreqDevice {
    pub freq: u32,
}

impl CpuFreqDevice {
    pub const fn init() -> Self {
        CpuFreqDevice { freq: 0 }
    }

    pub fn init_fdt() -> Self {
        let cpus_freq = match fdt_get_prop_by_node_name("cpus", "timebase-frequency") {
            Some(n) => n,
            None => panic!("Error while creating new CPU freq generic structure"),
        };
        let freq_value = fdt_get_prop_u32_value(cpus_freq);
        CpuFreqDevice { freq: freq_value }
    }
}

pub struct TimerDevice {
    pub interrupt_extended: [InterruptExtended; 4],
}

#[derive(Copy, Clone)]
pub struct InterruptExtended {
    // CPU core id
    pub cpu_intc: u32,
    // Field to follow the len of the irq_ids array to avoid crushing valid data
    pub irq_len: usize,
    // Array of all irq
    pub irq_ids: [u32; 4],
}

impl TimerDevice {
    #[allow(clippy::new_without_default)]
    pub const fn init() -> Self {
        TimerDevice {
            interrupt_extended: [InterruptExtended {
                cpu_intc: 0,
                irq_len: 0,
                irq_ids: [0u32; 4],
            }; 4],
        }
    }
    pub fn init_fdt(compatible: &str) -> Self {
        let node: &FdtNode = match fdt_get_node_by_compatible(compatible) {
            Some(n) => n,
            None => panic!("Error while creating new TimerDevice Generic structure"),
        };
        let interrupt: InterruptExtended = InterruptExtended {
            cpu_intc: u32::MAX,
            irq_len: 0,
            irq_ids: [0u32; 4],
        };
        let mut intc_extended_array: [InterruptExtended; 4] = [interrupt; 4];
        let interrupt_extended = fdt_get_node_prop(node, "interrupts-extended")
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
            let node = fdt_get_node_by_phandle(value).expect(
                "ERROR: cannot find associate phandle node from clint0 interrupts-extended property",
            );
            let node_interrupt_cells = fdt_get_node_prop(&node, "#interrupt-cells")
                .expect("ERROR: clint0 phandle node is missing the property '#interrupt-cells'");
            // Read node interrupt-cells value to know how many clint interrupt-extended value to
            // read and assign to phandle
            let cpu_node = fdt_get_node(node.parent_node_index.unwrap());
            let cpu_reg = fdt_get_node_prop(&cpu_node, "reg")
                .expect("ERROR: failed to get core id from associated core from intc");
            let cpu_reg_value = u32::from_be(unsafe { ptr::read(cpu_reg.off_value as *const u32) });
            let node_interrupt_cells_value =
                u32::from_be(unsafe { ptr::read(node_interrupt_cells.off_value as *const u32) });
            let mut parsed_interrupt: InterruptExtended = InterruptExtended {
                cpu_intc: cpu_reg_value,
                irq_len: 0,
                irq_ids: [0u32; 4],
            };
            // // Check if an interrupt for this phandle already exist
            #[allow(clippy::needless_range_loop)]
            for e in 0..intc_extended_array.len() {
                if intc_extended_array[e].cpu_intc != cpu_reg_value {
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
        TimerDevice {
            interrupt_extended: intc_extended_array,
        }
    }
}

// Implement DeviceInfo trait to all Device type structure
impl DeviceInfo for PlatformSerialDevice {}
impl DeviceInfo for TimerDevice {}
impl DeviceInfo for PlatformCpuIntCDevice {}
impl DeviceInfo for CpuFreqDevice {}

static mut TIMER_DEVICE_INSTANCE: TimerDevice = TimerDevice::init();
static mut SERIAL_DEVICE_INSTANCE: PlatformSerialDevice = PlatformSerialDevice::init();
static mut CPU_INTC_DEVICE_INSTANCE: PlatformCpuIntCDevice = PlatformCpuIntCDevice::init();
static mut CPU_FREQ_INSTANCE: CpuFreqDevice = CpuFreqDevice::init();

fn init_fdt_device(compatible: &'_ str, device_type: DeviceType) -> Option<Devices<'_>> {
    let mut default_device: Devices = Devices::init();
    match device_type {
        #[allow(static_mut_refs)]
        DeviceType::Serial => {
            let serial_device: PlatformSerialDevice = PlatformSerialDevice::init();
            unsafe { SERIAL_DEVICE_INSTANCE = serial_device };
            let mut device: Devices = Devices::init_fdt(compatible, device_type);
            device.info = Some(unsafe { &mut SERIAL_DEVICE_INSTANCE });
            default_device = device;
        }
        #[allow(static_mut_refs)]
        DeviceType::Timer => {
            let timer_device: TimerDevice = TimerDevice::init_fdt(compatible);
            unsafe { TIMER_DEVICE_INSTANCE = timer_device };
            let mut device: Devices = Devices::init_fdt(compatible, device_type);
            device.info = Some(unsafe { &mut TIMER_DEVICE_INSTANCE });
            default_device = device;
        }
        #[allow(static_mut_refs)]
        DeviceType::CpuIntC => {
            let cpu_intc_device: PlatformCpuIntCDevice = PlatformCpuIntCDevice::init_fdt(compatible);
            unsafe { CPU_INTC_DEVICE_INSTANCE = cpu_intc_device };
            default_device.info = Some(unsafe { &mut CPU_INTC_DEVICE_INSTANCE });
        }
        #[allow(static_mut_refs)]
        DeviceType::CpuFreq => {
            let cpu_freq_device: CpuFreqDevice = CpuFreqDevice::init_fdt();
            unsafe { CPU_FREQ_INSTANCE = cpu_freq_device };
            default_device.info = Some(unsafe { &mut CPU_FREQ_INSTANCE });
        }
    }
    Some(default_device)
}

pub fn platform_get_device_info(
    compatible: &'_ str,
    device_type: DeviceType,
) -> Option<Devices<'_>> {
    #[allow(static_mut_refs)]
    match unsafe { PLATFORM_INFO.read_mode() } {
        true => init_fdt_device(compatible, device_type),
        false => {
            let mut device: &Devices = &Devices::init();
            for each in DEVICES {
                if each.header.compatible == compatible {
                    device = each;
                }
            }
            Some(*device)
        }
    }
}
