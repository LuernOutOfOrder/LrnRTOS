use crate::{devices::{DeviceInfo, DeviceType, Devices, DevicesHeader, SerialDevice}, drivers::DriverRegion};

static mut SERIAL_DEVICE: SerialDevice = SerialDevice {};

pub static DEVICES: &[Devices] = &[Devices {
    header: DevicesHeader {
        device_type: DeviceType::Serial,
        compatible: "ns16550a",
        device_addr: DriverRegion {
            addr: 0x1000_0000,
            size: 0x1000,
        },
    },
    #[allow(static_mut_refs)]
    info: Some(unsafe { &SERIAL_DEVICE as *const dyn DeviceInfo }),
}];
