use crate::{
    devices_info::DEVICES, drivers::DriverRegion, fdt::{fdt_present, helpers::fdt_get_node_by_compatible, parse_dtb_file, FdtNode}
};

#[derive(Copy, Clone)]
pub enum DeviceType {
    Serial,
    Timer,
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
}

unsafe impl<'a> Sync for Devices<'a> {}

pub struct SerialDevice {}

pub struct TimerDevice {
    interrupt_extended: [TimerDeviceInterrupt; 4],
}

pub struct TimerDeviceInterrupt {
    // Ptr to CpuIntc struct
    cpu_intc: usize,
    // Field to follow the len of the irq_ids array to avoid crushing valid data
    irq_len: usize,
    // Array of all irq
    irq_ids: [u32; 4],
}

// Implement DeviceInfo trait to all Device type structure
impl DeviceInfo for SerialDevice {}
impl DeviceInfo for TimerDevice {}

// Boolean to define the type of info from devices to get.
// true == FDT
// false == static
static mut DEVICES_INFO: bool = true;

/// Initialize the FDT and the static devices. Choose the correct one to use.
pub fn devices_init(dtb_addr: usize) {
    if fdt_present(dtb_addr) {
        parse_dtb_file(dtb_addr);
        unsafe { DEVICES_INFO = false };
    }
}

pub fn devices_get_info(compatible: &'_ str) -> Option<Devices<'_>> {
    match unsafe { DEVICES_INFO } {
        true => {
            let node: &FdtNode = match fdt_get_node_by_compatible(compatible) {
                Some(n) => n,
                None => {
                    return None;
                }
            };
            let device_addr: DriverRegion = DriverRegion::new(node);
            let devices: Devices = Devices {
                header: DevicesHeader {
                    device_type: DeviceType::Serial,
                    compatible,
                    device_addr,
                },
                info: None,
            };
            Some(devices)
        }
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
