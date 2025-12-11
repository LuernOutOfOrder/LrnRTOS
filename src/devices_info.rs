use crate::drivers::DriverRegion;

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

impl DeviceInfo for SerialDevice {}

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
