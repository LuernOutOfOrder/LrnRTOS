use crate::{kprint, test_kprint};

use super::{
    DeviceType, PLATFORM_INFO,
    fdt::{fdt_present, parse_dtb_file},
    platform_get_device_info,
};

pub fn test_platform_init(dtb_addr: usize) {
    let test_mode_fdt: bool = true;
    // Disable warning on else if, this one is better than the collapsed one, I guess
    #[allow(clippy::collapsible_else_if)]
    if test_mode_fdt {
        if fdt_present(dtb_addr) {
            test_kprint!("FDT is present");
            parse_dtb_file(dtb_addr);
            #[allow(static_mut_refs)]
            unsafe {
                PLATFORM_INFO.set_mode_fdt()
            };
        } else {
            panic!("FDT should be present");
        }
    } else {
        if fdt_present(dtb_addr) {
            panic!("FDT is not supposed to be present");
        } else {
            test_kprint!("FDT is not present");
        }
    }
    // Condition with just kprint for debug purpose
    #[allow(static_mut_refs)]
    if unsafe { PLATFORM_INFO.read_mode() } {
        kprint!("Platform mode set to FDT.\n");
    } else {
        kprint!("Platform mode set to STATIC.\n");
    }
    #[allow(static_mut_refs)]
    let platform_mode = unsafe { PLATFORM_INFO.flags };
    assert_eq!(platform_mode, 0o1);
}

pub fn test_platform_get_device_info() {
    // Test to get None from an invalid device in the FDT.
    let none = platform_get_device_info("ns16550", DeviceType::Serial);
    if none.is_none() {
        test_kprint!("Correctly get None from invalid device asked.");
    } else {
        panic!("test failed, should get None from invalid device asked.");
    }
    // Test to get Some from a valid device in the FDT.
    let some = platform_get_device_info("ns16550a", DeviceType::Serial);
    if some.is_some() {
        test_kprint!("Correctly get Some from valid device asked.");
    } else {
        panic!("test failed, should get Some from valid device asked.");
    }
}
