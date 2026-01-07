#![cfg(feature = "test")]
use crate::{kprint, test_info_kprint, test_kprint, tests::TestCase};

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
        }
    }
    #[allow(static_mut_refs)]
    let platform_mode = unsafe { PLATFORM_INFO.flags };
    assert_eq!(platform_mode, 0o1);
}

/// Test getting device info from FDT.
pub fn test_platform_get_device_info_fdt() {
    // Test to get None from an invalid device in the FDT.
    let none = platform_get_device_info("ns16550", DeviceType::Serial);
    if none.is_none() {
    } else {
        panic!("should get None from invalid device asked.");
    }
    // Test to get Some from a valid device in the FDT.
    let some = platform_get_device_info("ns16550a", DeviceType::Serial);
    if some.is_some() {
    } else {
        panic!("should get Some from valid device asked.");
    }

    // Check the device get from FDT, if correct or not.
    let device = some.unwrap();
    if device.header.compatible != "ns16550a" {
        panic!(
            "Device get from platform should have the compatible property: 'ns16550a', got: {}",
            device.header.compatible
        );
    }
    if device.header.device_type != DeviceType::Serial {
        panic!("Device get from platform should have the device-type property: 'Serial'");
    }
    if device.header.device_addr.addr != 0x10000000 {
        panic!(
            "Device get from platform should have the MMIO address: '0x10000000', got: {}",
            device.header.device_addr.addr
        );
    }
    if device.header.device_addr.size != 0x100 {
        panic!(
            "Device get from platform should have the MMIO address size: '0x100', got: {:#x}",
            device.header.device_addr.size
        );
    }
}

/// Test getting device info from static
pub fn test_platform_get_device_info_static() {
    // Reset platform info to use static
    unsafe { PLATFORM_INFO.flags = 0 };
    // Test to get None from an invalid device in the FDT.
    let none = platform_get_device_info("ns1655", DeviceType::Serial);
    if none.is_none() {
    } else {
        panic!("should get None from invalid device asked.");
    }
    // Test to get Some from a valid device in the FDT.
    let some = platform_get_device_info("ns16550a", DeviceType::Serial);
    if some.is_some() {
    } else {
        panic!("should get Some from valid device asked.");
    }

    // Check the device get from FDT, if correct or not.
    let device = some.unwrap();
    if device.header.compatible != "ns16550a" {
        panic!(
            "Device get from platform should have the compatible property: 'ns16550a', got: {}",
            device.header.compatible
        );
    }
    if device.header.device_type != DeviceType::Serial {
        panic!("Device get from platform should have the device-type property: 'Serial'");
    }
    if device.header.device_addr.addr != 0x10000000 {
        panic!(
            "Device get from platform should have the MMIO address: '0x10000000', got: {}",
            device.header.device_addr.addr
        );
    }
    if device.header.device_addr.size != 0x100 {
        panic!(
            "Device get from platform should have the MMIO address size: '0x100', got: {:#x}",
            device.header.device_addr.size
        );
    }
}

pub static PLATFORM_TEST_SUITE: &[TestCase] = &[
    TestCase {
        name: "platform_device_info_fdt",
        func: test_platform_get_device_info_fdt,
    },
    TestCase {
        name: "platform_device_info_static",
        func: test_platform_get_device_info_static,
    },
];
