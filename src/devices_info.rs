// See documentation: `Documentation/kernel/platform.md`

use crate::{
    drivers::DriverRegion,
    platform::{
        CpuFreqDevice, CpuIntCDevice, DeviceInfo, DeviceType, Devices, DevicesHeader,
        InterruptExtended, SerialDevice, TimerDevice, mem::MemoryProvider,
    },
};

static mut SERIAL_DEVICE: SerialDevice = SerialDevice {};
static mut CLINT_DEVICE: TimerDevice = TimerDevice {
    interrupt_extended: [InterruptExtended {
        cpu_intc: 0,
        irq_len: 2,
        irq_ids: [3, 7, 0, 0],
    }; 4],
};
static mut CPU_INTC_DEVICE: CpuIntCDevice = CpuIntCDevice { core_id: 0 };
static mut CPU_FREQ_DEVICE: CpuFreqDevice = CpuFreqDevice { freq: 10000000 };

pub static MEM: MemoryProvider = MemoryProvider {
    reg: DriverRegion {
        addr: 0x80000000,
        size: 0x8000000,
    },
};

pub static DEVICES: &[Devices] = &[
    Devices {
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
    },
    Devices {
        header: DevicesHeader {
            device_type: DeviceType::Timer,
            compatible: "sifive,clint0",
            device_addr: DriverRegion {
                addr: 0x2000000,
                size: 0x10000,
            },
        },
        #[allow(static_mut_refs)]
        info: Some(unsafe { &CLINT_DEVICE as *const dyn DeviceInfo }),
    },
    Devices {
        header: DevicesHeader {
            device_type: DeviceType::CpuIntC,
            compatible: "riscv,cpu-intc",
            device_addr: DriverRegion { addr: 0, size: 0 },
        },
        #[allow(static_mut_refs)]
        info: Some(unsafe { &CPU_INTC_DEVICE as *const dyn DeviceInfo }),
    },
    Devices {
        header: DevicesHeader {
            device_type: DeviceType::CpuFreq,
            compatible: "cpu-freq",
            device_addr: DriverRegion { addr: 0, size: 0 },
        },
        #[allow(static_mut_refs)]
        info: Some(unsafe { &CPU_FREQ_DEVICE as *const dyn DeviceInfo }),
    },
];
