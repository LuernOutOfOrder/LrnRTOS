use crate::devices::serials::UART_DEVICES;

pub fn print(str: &str) {
    let devices = unsafe { &mut *UART_DEVICES.get() };
    for byte in str.bytes() {
        if let Some(uart) = devices[0] {
            uart.driver.putchar(byte);
        }
    }
}

#[macro_export]
macro_rules! print {
    ($s:expr) => {
        $crate::print::print($s);
    };
}

/// Only purpose to this function is to use it before the devices are init and before parsing the
/// dtb
pub fn debug_print(addr: usize, str: &str) {
    for byte in str.bytes() {
        unsafe { core::ptr::write_volatile(addr as *mut u8, byte) }
    }
}

pub fn print_hex_u32(addr: usize, mut val: u32) {
    // Buff for 8 digits, u32 have 8 hex digits
    let mut buf = [0u8; 8];
    for i in (0..8).rev() {
        let nibble = (val & 0xF) as u8;
        buf[i] = if nibble < 10 {
            b'0' + nibble
        } else {
            b'A' + (nibble - 10)
        };
        val >>= 4;
    }

    for &c in &buf {
        unsafe { core::ptr::write_volatile(addr as *mut u8, c) }
    }
}
