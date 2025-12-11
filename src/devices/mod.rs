use crate::{
    devices_info::{DEVICES, DeviceType, Devices, DevicesHeader},
    drivers::DriverRegion,
    fdt::{FdtNode, fdt_present, helpers::fdt_get_node_by_compatible, parse_dtb_file},
};

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
