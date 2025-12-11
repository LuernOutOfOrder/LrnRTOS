pub enum DeviceType {
    Serial,
    Timer,
}

pub trait DeviceInfo {}

/// Structure used to define a serial device.
/// Only used in static SERIAL_DEVICES
pub struct DevicesHeader {
    pub compatible: DeviceType,
    pub addr: usize,
    pub addr_size: usize,
}

pub struct Devices {
    header: DevicesHeader,
    info: *const dyn DeviceInfo,
}

unsafe impl Sync for Devices {}

pub struct SerialDevice {}

impl DeviceInfo for SerialDevice {}

static mut SERIAL_DEVICE: SerialDevice = SerialDevice {};

pub static DEVICES: &[Devices] = &[Devices {
    header: DevicesHeader {
        compatible: DeviceType::Serial,
        addr: 0x1000_0000,
        addr_size: 0x1000,
    },
    #[allow(static_mut_refs)]
    info: unsafe { &SERIAL_DEVICE as *const dyn DeviceInfo },
}];
