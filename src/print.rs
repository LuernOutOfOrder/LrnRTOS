use crate::devices::serials::UART_DEVICES;

pub fn print(str: &str) {
    let devices = unsafe { &mut *UART_DEVICES.get() };
    for byte in str.bytes() {
            if let Some(uart) = devices[0] {
                uart.driver.putchar(byte);
            }
    }
}
