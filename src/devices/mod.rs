use crate::fdt::{fdt_present, helpers::fdt_get_node_by_compatible, parse_dtb_file};

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

pub fn get_device_info(compatible: &str) {
    if unsafe { DEVICES_INFO } {
        fdt_get_node_by_compatible(compatible);
    }
}
